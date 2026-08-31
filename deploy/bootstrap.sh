#!/usr/bin/env bash
#
# ax-genealogy bootstrap — fresh Ubuntu LTS machine to a running site, one
# command. The package, the binary, the unit and the system user are all
# called axgf-cms; ax-genealogy is what the site calls itself.
#
#   curl -fsSL https://raw.githubusercontent.com/plkarin/axgf-cms/main/deploy/bootstrap.sh | sudo bash
#
# Options:
#   --with-sample     seed a new bundle with the built-in demonstration family
#   --from-source     build with cargo instead of downloading a release binary
#   --version <TAG>   install a specific release tag (default: latest stable).
#                     /releases/latest resolves stable releases only, so a
#                     release candidate is reachable only by tag:
#                     --version v0.1.0-rc1
#   --bind <ADDR>     address to bind (default: 127.0.0.1:8080)
#   --admin-user <U>  username for the first administrator (default: admin)
#   --dry-run         print what would happen and change nothing
#
# IDEMPOTENT. Running it twice must not destroy an existing bundle and must
# not regenerate the admin token. Both are checked before anything is written.

set -euo pipefail

REPO="plkarin/axgf-cms"
BIN_NAME="axgf-cms"

# Where release assets and release metadata are fetched from. Both are
# overridable so the download path can be exercised against a local mirror
# instead of being run for the first time by an operator.
RELEASE_BASE="${AXGF_CMS_RELEASE_BASE:-https://github.com/${REPO}}"
API_BASE="${AXGF_CMS_API_BASE:-https://api.github.com/repos/${REPO}}"

# Every system path is prefixed by AXGF_CMS_PREFIX, normally empty. Setting it
# installs the whole layout under a directory instead, which is what the
# idempotency test uses to run this script twice without root. Two further
# hooks let that test supply a locally built binary and skip the parts that
# need privileges.
PREFIX="${AXGF_CMS_PREFIX:-}"
LOCAL_BINARY="${AXGF_CMS_LOCAL_BINARY:-}"
SKIP_PRIVILEGED="${AXGF_CMS_SKIP_PRIVILEGED:-0}"

INSTALL_PATH="${PREFIX}/usr/local/bin/${BIN_NAME}"
SERVICE_USER="axgf-cms"
DATA_DIR="${PREFIX}/var/lib/axgf-cms"
CONF_DIR="${PREFIX}/etc/axgf-cms"
ENV_FILE="${CONF_DIR}/env"
BUNDLE="${DATA_DIR}/family.axgf"
UNIT_PATH="${PREFIX}/etc/systemd/system/axgf-cms.service"

WITH_SAMPLE=0
FROM_SOURCE=0
VERSION=""
BIND="127.0.0.1:8080"
DRY_RUN=0
# Username for the first administrator account, created on a fresh install.
ADMIN_USER="admin"

say()  { printf '  %s\n' "$*"; }
step() { printf '\n\033[1m==> %s\033[0m\n' "$*"; }
warn() { printf '  \033[33m!\033[0m %s\n' "$*"; }
die()  { printf '\n\033[31merror:\033[0m %s\n' "$*" >&2; exit 1; }

# Options that take a value must be given one. Without this, a trailing
# `--version` set the tag to the empty string and then ran off the end of the
# argument list, which under `set -e` ends the script with no message at all.
need_value() {
  [ -n "${2:-}" ] || die "$1 needs a value (for example: $1 $3)"
}

while [ $# -gt 0 ]; do
  case "$1" in
    --with-sample) WITH_SAMPLE=1 ;;
    --from-source) FROM_SOURCE=1 ;;
    --version) need_value "$1" "${2:-}" "v0.1.0-rc1"; VERSION="$2"; shift ;;
    --bind) need_value "$1" "${2:-}" "127.0.0.1:8080"; BIND="$2"; shift ;;
    --admin-user) need_value "$1" "${2:-}" "admin"; ADMIN_USER="$2"; shift ;;
    --dry-run) DRY_RUN=1 ;;
    # The header comment down to the first blank line, which is the whole of
    # it. This was a line range, and a line range goes stale the moment the
    # header grows — adding two lines to the options list silently truncated
    # --help below --dry-run.
    -h|--help) sed -n '2,/^$/p' "$0" | sed 's/^# \{0,1\}//'; exit 0 ;;
    *) die "unknown option: $1 (try --help)" ;;
  esac
  shift
done

run() {
  if [ "$DRY_RUN" = "1" ]; then
    printf '  [dry-run] %s\n' "$*"
  else
    "$@"
  fi
}

# Run a command as the service user, so anything it creates is owned by the
# account that has to read it afterwards. The .acl is written at mode 600, and
# a file root owns at 600 is a file the service cannot open.
run_as_service() {
  if [ "$DRY_RUN" = "1" ]; then
    printf '  [dry-run] %s\n' "$*"
    return
  fi
  if [ "$SKIP_PRIVILEGED" = "1" ] || [ "$(id -u)" != "0" ]; then
    "$@"
  else
    runuser -u "$SERVICE_USER" -- "$@"
  fi
}

# Write stdin to $1 with mode $2, honouring --dry-run.
write_file() {
  local path="$1" mode="$2" content
  content="$(cat)"
  if [ "$DRY_RUN" = "1" ]; then
    printf '  [dry-run] write %s (mode %s, %d bytes)\n' "$path" "$mode" "${#content}"
    return
  fi
  printf '%s\n' "$content" > "$path"
  chmod "$mode" "$path"
}

# Tags of every published release, newest first, one per line. Fails (rather
# than printing nothing) when the API cannot be reached, so "no releases" and
# "no network" stay distinguishable.
published_tags() {
  local json
  json="$(curl -fsSL "${API_BASE}/releases")" || return 1
  # A repository with no releases answers `[]`, and grep finding nothing there
  # is the answer, not a failure — but `set -o pipefail` would make it one.
  printf '%s' "$json" \
    | grep -o '"tag_name"[[:space:]]*:[[:space:]]*"[^"]*"' \
    | sed 's/.*"\([^"]*\)"$/\1/' || true
}

# True when a latest *stable* release exists. /releases/latest ignores
# prereleases, so a repository can have releases and still have no latest.
has_latest_stable() {
  curl -fsSL -o /dev/null "${API_BASE}/releases/latest" 2>/dev/null
}

# Why the download failed, in terms the operator can act on. This used to be a
# single line — "No release published yet? Use --from-source." — offered for
# every failure, including the one where a release had been published and the
# only problem was that it was not the latest stable one. Sending somebody to a
# ten-minute source build when `--version` would have worked is a misdiagnosis,
# so each case now gets its own answer.
explain_download_failure() {
  local tags newest
  if ! tags="$(published_tags)"; then
    die "download failed: ${URL}

  Could not reach ${API_BASE} to find out why. Check the network and the URL
  above; if this host has no route to GitHub, build from source instead with
  --from-source."
  fi

  if [ -n "$VERSION" ]; then
    if printf '%s\n' "$tags" | grep -qxF -- "$VERSION"; then
      die "release ${VERSION} exists, but publishes no asset for this machine.

  Expected:  ${ASSET}
  Machine:   ${ARCH} (${TARGET})

  Build from source instead: --from-source"
    fi
    die "no release is tagged ${VERSION}.

  Published releases:
$(printf '%s\n' "$tags" | sed 's/^/    /')

  Install one of those with --version, or build from source with --from-source."
  fi

  if [ -z "$tags" ]; then
    die "no release has been published for ${REPO} yet, so there is nothing to
  download.

  Build from source instead: --from-source"
  fi

  newest="$(printf '%s\n' "$tags" | head -1)"
  if has_latest_stable; then
    die "the latest release publishes no asset for this machine.

  Expected:  ${ASSET}
  Machine:   ${ARCH} (${TARGET})

  Build from source instead: --from-source"
  fi

  die "releases have been published, but none of them is a stable release, and
  /releases/latest resolves stable releases only — which is why the default
  install finds nothing. Every published release is a prerelease.

  Install one by tag:

    --version ${newest}

  ...or build from source with --from-source."
}

if [ "$DRY_RUN" = "0" ] && [ "$SKIP_PRIVILEGED" = "0" ] && [ "$(id -u)" != "0" ]; then
  die "must run as root (try: sudo bash bootstrap.sh)"
fi

step "Checking the host"
ARCH="$(uname -m)"
case "$ARCH" in
  x86_64|amd64)  TARGET="x86_64-unknown-linux-musl" ;;
  aarch64|arm64) TARGET="aarch64-unknown-linux-gnu" ;;
  *) die "unsupported architecture: $ARCH (this project builds x86_64 and aarch64 Linux only)" ;;
esac
say "architecture $ARCH -> $TARGET"
[ "$(uname -s)" = "Linux" ] || die "this installer targets Linux servers only"
command -v systemctl >/dev/null 2>&1 || warn "systemd not found; the unit will be installed but not started"

# --------------------------------------------------------------------------
step "Installing the binary"
# --------------------------------------------------------------------------
# Every branch below installs to $INSTALL_PATH, so the directory is made once
# here rather than in one of the three. /usr/local/bin exists on any normal
# machine, which is why this went unnoticed: it only fails under a prefix, or
# on an image minimal enough not to have it.
run mkdir -p "$(dirname "$INSTALL_PATH")"

if [ -n "$LOCAL_BINARY" ]; then
  say "installing the locally supplied binary $LOCAL_BINARY"
  run install -m 0755 "$LOCAL_BINARY" "$INSTALL_PATH"
elif [ "$FROM_SOURCE" = "1" ]; then
  command -v cargo >/dev/null 2>&1 || die "--from-source needs cargo on PATH"
  say "building from source (this takes a few minutes)"
  SRC="$(mktemp -d)"
  run git clone --depth 1 "https://github.com/${REPO}.git" "$SRC/src"
  run env -C "$SRC/src" cargo build --release --locked
  run install -m 0755 "$SRC/src/target/release/${BIN_NAME}" "$INSTALL_PATH"
else
  ASSET="${BIN_NAME}-${TARGET}.tar.gz"
  if [ -n "$VERSION" ]; then
    # A tagged asset carries its tag in the filename, and this is the only way
    # to reach a release that is not the latest stable one — a release
    # candidate, or an older version being pinned.
    ASSET="${BIN_NAME}-${VERSION}-${TARGET}.tar.gz"
    URL="${RELEASE_BASE}/releases/download/${VERSION}/${ASSET}"
  else
    URL="${RELEASE_BASE}/releases/latest/download/${ASSET}"
  fi
  say "downloading $URL"
  TMP="$(mktemp -d)"
  if [ "$DRY_RUN" = "0" ]; then
    curl -fsSL "$URL" -o "$TMP/pkg.tar.gz" || explain_download_failure
    # Verify the checksum when the release publishes one.
    if curl -fsSL "${URL}.sha256" -o "$TMP/pkg.sha256" 2>/dev/null; then
      ( cd "$TMP" && sed "s#\([a-f0-9]\{64\}\).*#\1  pkg.tar.gz#" pkg.sha256 | sha256sum -c - ) \
        || die "checksum mismatch — refusing to install"
      say "checksum verified"
    else
      warn "no .sha256 published alongside the release; skipping verification"
    fi
    tar -xzf "$TMP/pkg.tar.gz" -C "$TMP"
    install -m 0755 "$(find "$TMP" -type f -name "$BIN_NAME" | head -1)" "$INSTALL_PATH"
    rm -rf "$TMP"
  else
    printf '  [dry-run] download, verify sha256, install to %s\n' "$INSTALL_PATH"
  fi
fi
say "installed $INSTALL_PATH"

# --------------------------------------------------------------------------
step "Creating the service user"
# --------------------------------------------------------------------------
if [ "$SKIP_PRIVILEGED" = "1" ]; then
  say "skipping user creation (AXGF_CMS_SKIP_PRIVILEGED)"
elif id -u "$SERVICE_USER" >/dev/null 2>&1; then
  say "user $SERVICE_USER already exists — leaving it alone"
else
  run useradd --system --home-dir "$DATA_DIR" --shell /usr/sbin/nologin "$SERVICE_USER"
  say "created system user $SERVICE_USER (no shell)"
fi

# --------------------------------------------------------------------------
step "Preparing directories"
# --------------------------------------------------------------------------
if [ "$SKIP_PRIVILEGED" = "1" ]; then
  run mkdir -p "$DATA_DIR" "$CONF_DIR"
else
  run install -d -o "$SERVICE_USER" -g "$SERVICE_USER" -m 0750 "$DATA_DIR"
  run install -d -o root -g "$SERVICE_USER" -m 0750 "$CONF_DIR"
fi
say "$DATA_DIR and $CONF_DIR ready"

# --------------------------------------------------------------------------
step "Admin token"
# --------------------------------------------------------------------------
# The token is generated once and never regenerated: rewriting it on a second
# run would silently lock the operator out of a working install.
if [ -f "$ENV_FILE" ]; then
  say "$ENV_FILE exists — keeping the existing token"
  TOKEN="$(sed -n 's/^AXGF_CMS_ADMIN_TOKEN=//p' "$ENV_FILE" | head -1)"
else
  if [ "$DRY_RUN" = "1" ]; then
    TOKEN="<generated-on-first-real-run>"
  else
    TOKEN="$(head -c 32 /dev/urandom | od -An -tx1 | tr -d ' \n')"
  fi
  write_file "$ENV_FILE" 0600 <<EOF
# axgf-cms configuration. This file holds the admin token: it is written
# 0600, then chowned root:axgf-cms and set 0640 so the service can read it
# and nobody else can.
AXGF_CMS_ADMIN_TOKEN=${TOKEN}
EOF
  if [ "$SKIP_PRIVILEGED" = "0" ]; then
    run chown root:"$SERVICE_USER" "$ENV_FILE"
    run chmod 0640 "$ENV_FILE"
  fi
  say "generated a new admin token into $ENV_FILE"
fi

# --------------------------------------------------------------------------
step "Bundle"
# --------------------------------------------------------------------------
# An existing bundle is the entire database. It is never overwritten.
SEED_FLAG=""
if [ -f "$BUNDLE" ]; then
  say "$BUNDLE already exists — leaving it untouched"
  if [ "$WITH_SAMPLE" = "1" ]; then
    warn "--with-sample ignored: a bundle is already present"
  fi
else
  if [ "$WITH_SAMPLE" = "1" ]; then
    SEED_FLAG="--seed-sample"
    say "a new bundle will be seeded with the demonstration family"
  else
    say "a new empty bundle will be created on first start"
  fi
fi

# --------------------------------------------------------------------------
step "First administrator"
# --------------------------------------------------------------------------
# The accounts live in a companion .acl file beside the bundle, never inside
# it: a .axgf is copied, mailed and published, and password hashes in it would
# make every copy of the family tree a copy of the credential store.
#
# This runs BEFORE the unit is installed and started, and that ordering is
# load-bearing. It used to run after: the service was started, began seeding
# the sample, and was stopped again a fraction of a second later so the ACL
# could be written. The create-admin invocation then found no bundle yet —
# because the seed had not finished writing — and created an empty one, so a
# fresh --with-sample install served a signed-out visitor "0 of 0 people".
# Doing all the file creation before anything is running removes the race
# rather than widening the window.
#
# There is deliberately no web setup page. The window between deploying and the
# first login is exactly when an installation is unprotected, so the first
# account is created here, from the shell, by somebody who already has the
# host. Re-running is safe: an existing username is refused, not reset, so this
# never silently rotates a working account's password.
ACL_FILE="${BUNDLE%.axgf}.acl"
ADMIN_CREATED=0
if [ -f "$ACL_FILE" ]; then
  say "$ACL_FILE exists — leaving the existing accounts alone"
elif [ "$DRY_RUN" = "1" ]; then
  say "an administrator account would be created as '${ADMIN_USER}'"
else
  # $SEED_FLAG matters here: this runs before the service exists, so it is
  # what creates the bundle. Without it the bundle would be created empty and
  # the sample never seeded — which is exactly what used to happen, from the
  # other direction.
  # shellcheck disable=SC2086
  ADMIN_OUT="$(run_as_service "$INSTALL_PATH" --bundle "$BUNDLE" $SEED_FLAG \
                 --create-admin "$ADMIN_USER" 2>&1 || true)"
  ADMIN_PASSWORD="$(printf '%s\n' "$ADMIN_OUT" | sed -n 's/^ *password: *//p' | head -1)"
  if [ -n "$ADMIN_PASSWORD" ]; then
    ADMIN_CREATED=1
    say "created administrator '${ADMIN_USER}'"
  else
    warn "could not create the first administrator:"
    printf '%s\n' "$ADMIN_OUT" | sed 's/^/    /'
  fi
fi

# --------------------------------------------------------------------------
step "systemd unit"
# --------------------------------------------------------------------------
run mkdir -p "$(dirname "$UNIT_PATH")"
write_file "$UNIT_PATH" 0644 <<EOF
[Unit]
Description=axgf-cms — AXGF genealogy showcase
Documentation=https://github.com/${REPO}
After=network.target

[Service]
Type=exec
User=${SERVICE_USER}
Group=${SERVICE_USER}
EnvironmentFile=${ENV_FILE}
ExecStart=${INSTALL_PATH} --bundle ${BUNDLE} --bind ${BIND}
Restart=on-failure
RestartSec=2s

# The process needs exactly one writable directory and nothing else.
NoNewPrivileges=yes
PrivateTmp=yes
PrivateDevices=yes
ProtectSystem=strict
ProtectHome=yes
ProtectKernelTunables=yes
ProtectKernelModules=yes
ProtectControlGroups=yes
ReadWritePaths=${DATA_DIR}
RestrictAddressFamilies=AF_INET AF_INET6
RestrictNamespaces=yes
LockPersonality=yes
MemoryDenyWriteExecute=yes

[Install]
WantedBy=multi-user.target
EOF
say "wrote $UNIT_PATH"

if [ "$SKIP_PRIVILEGED" = "1" ]; then
  say "skipping systemctl (AXGF_CMS_SKIP_PRIVILEGED)"
elif command -v systemctl >/dev/null 2>&1; then
  run systemctl daemon-reload
  run systemctl enable axgf-cms
  run systemctl restart axgf-cms
  say "service enabled and started"
fi

# --------------------------------------------------------------------------
step "Done"
# --------------------------------------------------------------------------
cat <<EOF

  ax-genealogy is running.

    URL          http://${BIND}/
    Sign in      http://${BIND}/admin/login

    Bundle       ${BUNDLE}       <- the genealogy; back it up, share it freely
    Accounts     ${ACL_FILE}       <- mode 600; back it up, share it with nobody
    Config       ${ENV_FILE}
    Logs         journalctl -u axgf-cms -f
EOF

if [ "$ADMIN_CREATED" = "1" ]; then
cat <<EOF

    username     ${ADMIN_USER}
    password     ${ADMIN_PASSWORD}

  This password is shown once and is not recoverable: it is stored only as an
  Argon2id hash. Write it down now. Sign in and create accounts for everyone
  else from the Accounts page — there is no self-registration.
EOF
fi

cat <<EOF

  Emergency token  ${TOKEN}

  Stored in ${ENV_FILE} (root-readable only). It is not an account: it grants
  an administrator session for getting back in when ${ACL_FILE} has been lost
  or every administrator is locked out. Its use is logged as a warning.

  SECURITY: this binds to localhost by design. To publish it, put a TLS
  reverse proxy in front — see docs/DEPLOY.md — and do not move the bind
  address to 0.0.0.0 without one. Records marked \`private\` or \`members\` are
  withheld from signed-out visitors, but a bundle converted from GEDCOM
  carries no visibility at all: there, everyone recorded as living is treated
  as \`members\` and everyone else is public. Check that this is what you want
  before publishing.

EOF
