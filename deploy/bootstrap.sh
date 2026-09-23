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
#   --backup-dir <D>  where daily archives go (default: /var/lib/axgf-cms/backups)
#   --backup-at <T>   when the daily backup runs, as a systemd OnCalendar
#                     expression (default: 03:30, with a random delay)
#   --upgrade         install a newer binary over a working installation:
#                     backup, replace, restart, check /health, and put the old
#                     binary back automatically if the check fails
#   --uninstall       stop and remove the service, the timers and the binary.
#                     Never the data: the bundle, the accounts, the journal and
#                     every backup are left exactly where they are, and the
#                     script says where that is
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
BACKUP_SERVICE_PATH="${PREFIX}/etc/systemd/system/axgf-cms-backup.service"
BACKUP_TIMER_PATH="${PREFIX}/etc/systemd/system/axgf-cms-backup.timer"
VERIFY_SERVICE_PATH="${PREFIX}/etc/systemd/system/axgf-cms-verify.service"
VERIFY_TIMER_PATH="${PREFIX}/etc/systemd/system/axgf-cms-verify.timer"
# Where the daily archives go. On the same disk as the bundle by default,
# because that is the only location this script can know exists — and that is
# exactly why the closing notes insist on copying them off the machine.
BACKUP_DIR="${AXGF_CMS_BACKUP_DIR:-${DATA_DIR}/backups}"

WITH_SAMPLE=0
FROM_SOURCE=0
VERSION=""
BIND="127.0.0.1:8080"
DRY_RUN=0
UPGRADE=0
UNINSTALL=0
BACKUP_AT="03:30"
# Sunday, after the night's backup has landed and long before anybody is
# reading the site.
VERIFY_AT="Sun 04:30"
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
    --backup-dir) need_value "$1" "${2:-}" "/srv/backups"; BACKUP_DIR="$2"; shift ;;
    --backup-at) need_value "$1" "${2:-}" "03:30"; BACKUP_AT="$2"; shift ;;
    --upgrade) UPGRADE=1 ;;
    --uninstall) UNINSTALL=1 ;;
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
# Installing the binary
# --------------------------------------------------------------------------
# A function rather than a straight-line block, because `--upgrade` needs
# exactly the same three ways of getting a binary and must not drift from them.
install_binary() {
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
}

# Wait for the service to answer /health with a bundle it can actually read.
#
# The bundle check is what decides, not the overall status. A brand-new
# installation on a disk that is 92 % full answers 503 for a reason that has
# nothing to do with the binary that was just installed, and rolling back over
# it would be a misdiagnosis with a rollback attached. "Can this binary serve
# this family's data" is `checks.bundle`, and nothing else.
wait_for_health() {
  local url="http://${BIND}/health" body i
  for i in $(seq 1 30); do
    if body="$(curl -fsS --max-time 3 "$url" 2>/dev/null)" \
       || body="$(curl -sS --max-time 3 "$url" 2>/dev/null)"; then
      case "$body" in
        *'"bundle"'*'"status":"ok"'*) printf '%s' "$body"; return 0 ;;
        *'"bundle":{"status":"ok"'*)  printf '%s' "$body"; return 0 ;;
      esac
    fi
    sleep 1
  done
  printf '%s' "${body:-<no response from ${url}>}"
  return 1
}

# --------------------------------------------------------------------------
# Topping up the environment file
# --------------------------------------------------------------------------
# An installation from before the settings moved into this file has only the
# token in it. What is missing is appended; what is there is left exactly as it
# is, so an operator's own edits and comments survive.
#
# The upgrade path needs this as much as the install path does, and for a
# sharper reason: an upgrade writes the new unit, whose ExecStart carries no
# arguments at all. Against an env file that names no bundle, that unit starts
# a binary with nothing to open — which is precisely what happened the first
# time this was run against the published release.
ensure_env_keys() {
  [ -f "$ENV_FILE" ] || return 0
  local line key
  for line in \
    "AXGF_CMS_BUNDLE=${BUNDLE}" \
    "AXGF_CMS_BIND=${BIND}" \
    "AXGF_CMS_BACKUP_DIR=${BACKUP_DIR}" \
    "AXGF_CMS_CACHE_DIR=${DATA_DIR}/cache" \
    "RUST_LOG=axgf_cms=info,tower_http=warn"
  do
    key="${line%%=*}"
    if ! grep -q "^${key}=" "$ENV_FILE" 2>/dev/null; then
      if [ "$DRY_RUN" = "1" ]; then
        say "would add ${key} to $ENV_FILE"
      else
        printf '%s\n' "$line" >> "$ENV_FILE"
        say "added ${key} to $ENV_FILE"
      fi
    fi
  done
}

# --------------------------------------------------------------------------
# Writing the units
# --------------------------------------------------------------------------
# A function, because an upgrade installs them too.
#
# The units are generated by this script, so a binary that gains something the
# unit has to know about — the readiness protocol, a new sandbox directive —
# only gets it if the upgrade path writes them as well. It did not: `--upgrade`
# replaced the binary and left whatever unit the older release had written.
write_units() {
# --------------------------------------------------------------------------
step "systemd unit"
# --------------------------------------------------------------------------
run mkdir -p "$(dirname "$UNIT_PATH")"
write_file "$UNIT_PATH" 0644 <<EOF
[Unit]
Description=axgf-cms — AXGF genealogy showcase
Documentation=https://github.com/${REPO}
After=network.target
# A restart that keeps failing must not spin. See RestartSec and its steps
# below: the delay grows, and no start limit ever stops it trying, because a
# family's site giving up permanently after a transient failure is worse than
# one that keeps knocking quietly.
StartLimitIntervalSec=0

[Service]
# notify, not exec: "started" then means the bundle is loaded, validated and
# listening, rather than "the process was spawned". On a 435 MB archive that
# is several seconds of difference, and it is the difference between
# \`systemctl start\` returning into a working site and returning into a 502
# from the proxy in front of it.
Type=notify
NotifyAccess=main
User=${SERVICE_USER}
Group=${SERVICE_USER}

# Everything the service is configured by. ExecStart carries no arguments on
# purpose: one place to change a setting, and no flag that has to agree with a
# line in this file.
EnvironmentFile=${ENV_FILE}
ExecStart=${INSTALL_PATH}

Restart=on-failure
RestartSec=2s
# Back off rather than hammer. The failure this exists for is a full disk: a
# service that crash-loops every two seconds on a full disk writes a journal
# entry every two seconds onto the disk that is full.
RestartSteps=5
RestartMaxDelaySec=2min

# SIGTERM, and time to finish. The process stops accepting, lets the requests
# in flight return, and then takes the write lock — which is how it waits out
# a save or a backup that another process started. 90 seconds is longer than
# any of that takes on a 435 MB bundle and short enough that a wedged process
# is still killed.
KillSignal=SIGTERM
TimeoutStopSec=90s

# ---------------------------------------------------------------------------
# Sandbox
#
# This is a web-facing process holding a family's private records. It needs
# one directory it can write and nothing else at all.
# ---------------------------------------------------------------------------
NoNewPrivileges=yes
PrivateTmp=yes
PrivateDevices=yes
PrivateUsers=yes
ProtectSystem=strict
ProtectHome=yes
ProtectKernelTunables=yes
ProtectKernelModules=yes
ProtectKernelLogs=yes
ProtectControlGroups=yes
ProtectClock=yes
ProtectHostname=yes
ProtectProc=invisible
ProcSubset=pid
RestrictNamespaces=yes
RestrictRealtime=yes
RestrictSUIDSGID=yes
RemoveIPC=yes
LockPersonality=yes
MemoryDenyWriteExecute=yes
SystemCallArchitectures=native
SystemCallFilter=@system-service
# @system-service still allows chown, ioprio_set and the rest of @privileged
# and @resources. This process changes no ownership and no scheduling
# priority, so both come back out. Tested by exercising the product under
# them: a page, a sign-in, a save, an upload, a backup.
SystemCallFilter=~@privileged
SystemCallFilter=~@resources
SystemCallErrorNumber=EPERM
# It binds a port above 1024 and reads its own files. No capability in the
# set at all, which also means none to inherit.
CapabilityBoundingSet=
AmbientCapabilities=
# It answers a reverse proxy on the same machine and calls nothing out.
#
# If you turn the place-name geocoder on (--geocoder-contact), this process
# starts making outbound HTTPS calls to Nominatim and they will be refused
# here. Widen it deliberately rather than by deleting the pair:
#
#   sudo systemctl edit axgf-cms
#   [Service]
#   IPAddressAllow=any
IPAddressDeny=any
IPAddressAllow=localhost
# AF_UNIX is not optional: it is how this process reaches journald's socket
# and how it answers systemd's own readiness protocol. Removing it costs the
# logs and hangs the start. AF_INET and AF_INET6 are the listening socket.
RestrictAddressFamilies=AF_UNIX AF_INET AF_INET6
# Files it creates — the bundle, the accounts, an archive — are readable by
# nobody but the service user.
UMask=0077

# Exactly the two directories it writes, named rather than implied.
ReadWritePaths=${DATA_DIR} ${BACKUP_DIR}

[Install]
WantedBy=multi-user.target
EOF
say "wrote $UNIT_PATH"

# --------------------------------------------------------------------------
step "Daily backup"
# --------------------------------------------------------------------------
# A timer rather than a cron line: it survives a machine that was switched off
# at half past three (Persistent=true catches up on the next boot), it is
# visible in `systemctl list-timers` beside everything else, and its output
# goes to the journal where the rest of this service's output already is.
write_file "$BACKUP_SERVICE_PATH" 0644 <<EOF
[Unit]
Description=axgf-cms — verified backup of the bundle, accounts and journal
Documentation=https://github.com/${REPO}
# Not After=axgf-cms.service, and not bound to it either: the backup is
# deliberately safe to run against a live instance, and it must still run on a
# machine where the web service is stopped or failed. It takes the same write
# lock a save takes, so the three files in the archive agree with each other.

[Service]
Type=oneshot
User=${SERVICE_USER}
Group=${SERVICE_USER}
EnvironmentFile=${ENV_FILE}
ExecStart=${INSTALL_PATH} backup

# The same sandbox as the service, minus the listening socket: this one talks
# to nothing over the network at all.
NoNewPrivileges=yes
PrivateTmp=yes
PrivateDevices=yes
PrivateNetwork=yes
PrivateUsers=yes
ProtectSystem=strict
ProtectHome=yes
ProtectKernelTunables=yes
ProtectKernelModules=yes
ProtectKernelLogs=yes
ProtectControlGroups=yes
ProtectClock=yes
ProtectHostname=yes
ProtectProc=invisible
ProcSubset=pid
RestrictNamespaces=yes
RestrictRealtime=yes
RestrictSUIDSGID=yes
RemoveIPC=yes
LockPersonality=yes
MemoryDenyWriteExecute=yes
SystemCallArchitectures=native
SystemCallFilter=@system-service
SystemCallErrorNumber=EPERM
CapabilityBoundingSet=
AmbientCapabilities=
# AF_UNIX alone: journald's socket, and nothing else.
RestrictAddressFamilies=AF_UNIX
UMask=0077
ReadWritePaths=${DATA_DIR} ${BACKUP_DIR}
EOF
say "wrote $BACKUP_SERVICE_PATH"

write_file "$BACKUP_TIMER_PATH" 0644 <<EOF
[Unit]
Description=axgf-cms — daily backup
Documentation=https://github.com/${REPO}

[Timer]
OnCalendar=${BACKUP_AT}
# Up to an hour of jitter, so a fleet of these does not all wake at once and
# so the archive's timestamp is not a reliable statement about when somebody
# is asleep.
RandomizedDelaySec=1h
# Catch up after a machine that was switched off at the appointed hour. This
# is a family's computer, not a datacentre.
Persistent=true

[Install]
WantedBy=timers.target
EOF
say "wrote $BACKUP_TIMER_PATH"

# --------------------------------------------------------------------------
step "Weekly verification"
# --------------------------------------------------------------------------
# A backup nobody has read is a claim. Once a week this loads the bundle and
# validates it, measures the payload cache against what the bundle declares,
# and reads the newest archive back in full — every CRC, every member's
# SHA-256 against the manifest, the bundle inside it re-imported and
# validated. It fails the unit when any of that is wrong, so
# `systemctl is-failed axgf-cms-verify` is an answer and not a guess.
#
# Its own unit, because reading 435 MB back once a week must never be able to
# take the web service down with it, and because a failure here deserves its
# own status and its own log rather than a line in somebody else's.
write_file "$VERIFY_SERVICE_PATH" 0644 <<EOF
[Unit]
Description=axgf-cms — read the bundle and the newest archive back
Documentation=https://github.com/${REPO}

[Service]
Type=oneshot
User=${SERVICE_USER}
Group=${SERVICE_USER}
EnvironmentFile=${ENV_FILE}
ExecStart=${INSTALL_PATH} verify

# Reading a 415 MB archive back means extracting it somewhere, and this unit
# deliberately cannot write to the data directory. systemd hands it one
# directory of its own — /var/cache/axgf-cms-verify, on real disk rather than
# the RAM that /run would cost — and cleans it when the unit stops.
CacheDirectory=axgf-cms-verify
CacheDirectoryMode=0700

# Beyond that: reads everything, writes nothing. No ReadWritePaths at all.
NoNewPrivileges=yes
PrivateTmp=yes
PrivateDevices=yes
PrivateNetwork=yes
PrivateUsers=yes
ProtectSystem=strict
ProtectHome=yes
ProtectKernelTunables=yes
ProtectKernelModules=yes
ProtectKernelLogs=yes
ProtectControlGroups=yes
ProtectClock=yes
ProtectHostname=yes
ProtectProc=invisible
ProcSubset=pid
RestrictNamespaces=yes
RestrictRealtime=yes
RestrictSUIDSGID=yes
RemoveIPC=yes
LockPersonality=yes
MemoryDenyWriteExecute=yes
SystemCallArchitectures=native
SystemCallFilter=@system-service
SystemCallErrorNumber=EPERM
CapabilityBoundingSet=
AmbientCapabilities=
RestrictAddressFamilies=AF_UNIX
UMask=0077
EOF
say "wrote $VERIFY_SERVICE_PATH"

write_file "$VERIFY_TIMER_PATH" 0644 <<EOF
[Unit]
Description=axgf-cms — weekly verification
Documentation=https://github.com/${REPO}

[Timer]
OnCalendar=${VERIFY_AT}
RandomizedDelaySec=1h
Persistent=true

[Install]
WantedBy=timers.target
EOF
say "wrote $VERIFY_TIMER_PATH"
}

if [ "$UNINSTALL" = "1" ]; then
  # ------------------------------------------------------------------------
  step "Uninstalling"
  # ------------------------------------------------------------------------
  # What goes: the units, the timers, the binary. What stays: every byte of
  # the family's data, and the configuration that says where it is.
  #
  # There is no --purge and there will not be one. An uninstall that can
  # delete a genealogy is a rm -rf with a friendly name, and the person most
  # likely to run it is somebody who thinks they are tidying up. Deleting the
  # data is three commands the operator types themselves, with the paths in
  # front of them, printed below.
  if command -v systemctl >/dev/null 2>&1 && [ "$SKIP_PRIVILEGED" = "0" ]; then
    for unit in axgf-cms-verify.timer axgf-cms-backup.timer axgf-cms.service; do
      if systemctl list-unit-files "$unit" >/dev/null 2>&1; then
        run systemctl disable --now "$unit" || true
      fi
    done
    say "stopped and disabled the service and both timers"
  fi

  for f in "$UNIT_PATH" "$BACKUP_SERVICE_PATH" "$BACKUP_TIMER_PATH" \
           "$VERIFY_SERVICE_PATH" "$VERIFY_TIMER_PATH"; do
    [ -e "$f" ] && run rm -f "$f"
    # The copies an upgrade keeps for its rollback. Leaving them behind made an
    # uninstalled machine still list five axgf-cms files in
    # /etc/systemd/system, which is not what "removed" should look like.
    [ -e "${f}.previous" ] && run rm -f "${f}.previous"
  done
  say "removed the unit files"

  if command -v systemctl >/dev/null 2>&1 && [ "$SKIP_PRIVILEGED" = "0" ]; then
    run systemctl daemon-reload
    run systemctl reset-failed || true
  fi

  [ -e "$INSTALL_PATH" ] && run rm -f "$INSTALL_PATH"
  [ -e "${INSTALL_PATH}.previous" ] && run rm -f "${INSTALL_PATH}.previous"
  say "removed the binary"

  BUNDLE_SIZE="$( [ -f "$BUNDLE" ] && du -h "$BUNDLE" | cut -f1 || echo '-' )"
  ARCHIVES="$( ls -1 "$BACKUP_DIR"/axgf-backup-*.zip 2>/dev/null | wc -l )"

  step "Done"
  cat <<EOF

  The service, both timers and the binary are gone. Nothing else is.

    Bundle       ${BUNDLE}  (${BUNDLE_SIZE})
    Accounts     ${BUNDLE%.axgf}.acl
    Journal      ${BUNDLE}.journal*
    Backups      ${BACKUP_DIR}  (${ARCHIVES} archive(s))
    Config       ${ENV_FILE}
    Service user ${SERVICE_USER}  (still owns those files)

  Reinstalling over this picks all of it up again, including the emergency
  token, and starts serving the same tree.

  To delete the data as well, with the paths in front of you:

    sudo rm -rf ${DATA_DIR}
    sudo rm -rf ${CONF_DIR}
    sudo userdel ${SERVICE_USER}

  This script will not do that for you.

EOF
  exit 0
fi

if [ "$UPGRADE" = "1" ]; then
  # ------------------------------------------------------------------------
  step "Upgrading"
  # ------------------------------------------------------------------------
  [ -x "$INSTALL_PATH" ] || die "nothing is installed at ${INSTALL_PATH}, so there is \
nothing to upgrade. Run this script without --upgrade to install."
  [ -f "$BUNDLE" ] || die "no bundle at ${BUNDLE}. This does not look like an \
installation of axgf-cms."

  OLD_VERSION="$("$INSTALL_PATH" --version 2>/dev/null || echo unknown)"
  say "currently installed: ${OLD_VERSION}"

  # 1. A backup FIRST, taken by the binary that is known to work. If the
  #    upgrade goes wrong in a way the rollback cannot undo — a new binary
  #    that wrote the bundle before failing — this archive is what is left.
  step "Backing up before anything is replaced"
  # Created here for an installation from a release that had no backups at all,
  # and owned by the service user rather than by root — a root-owned directory
  # is one the backup unit cannot write into, which would leave the timer
  # failing every night on a machine that had just been upgraded to have one.
  if [ ! -d "$BACKUP_DIR" ]; then
    if [ "$SKIP_PRIVILEGED" = "0" ]; then
      run install -d -o "$SERVICE_USER" -g "$SERVICE_USER" -m 0700 "$BACKUP_DIR"
    else
      run mkdir -p "$BACKUP_DIR"
    fi
  elif [ "$SKIP_PRIVILEGED" = "0" ]; then
    run chown "$SERVICE_USER":"$SERVICE_USER" "$BACKUP_DIR"
  fi
  if [ "$DRY_RUN" = "1" ]; then
    printf '  [dry-run] axgf-cms backup --bundle %s --dest %s\n' "$BUNDLE" "$BACKUP_DIR"
  elif "$INSTALL_PATH" backup --help >/dev/null 2>&1; then
    run_as_service "$INSTALL_PATH" backup --bundle "$BUNDLE" --dest "$BACKUP_DIR" \
      || die "the pre-upgrade backup failed, so the upgrade stopped before it \
started. Nothing was changed. Fix the backup first: an upgrade without one is \
the single most expensive thing that can go wrong here."
  else
    # The installed version predates `axgf-cms backup` — which is exactly the
    # version most installations are upgrading *from*, since backups did not
    # exist in the first published release. Refusing to upgrade for want of a
    # command the old binary never had would strand precisely those machines,
    # so the three files are copied instead.
    #
    # The service is stopped first, and that ordering is the whole point: a
    # copy taken from under a running server can catch a save in progress and
    # produce a bundle and an .acl that disagree. Stopped, they cannot.
    warn "the installed version has no backup command; copying the files instead"
    STAMP="$(date -u +%Y%m%dT%H%M%SZ)"
    PRE="${BACKUP_DIR}/pre-upgrade-${STAMP}"
    if [ "$SKIP_PRIVILEGED" = "0" ] && command -v systemctl >/dev/null 2>&1; then
      run systemctl stop axgf-cms || true
    fi
    run mkdir -p "$PRE"
    for f in "$BUNDLE" "${BUNDLE%.axgf}.acl" "${BUNDLE}".journal*; do
      [ -e "$f" ] && run cp -p "$f" "$PRE/"
    done
    if [ "$SKIP_PRIVILEGED" = "0" ]; then
      run chown -R "$SERVICE_USER":"$SERVICE_USER" "$PRE" || true
    fi
    say "copied the bundle, the accounts and the journal into ${PRE}"
    say "the new binary will write a real, verified archive after this"
  fi

  # 2. Keep the working binary where the rollback can find it.
  PREVIOUS="${INSTALL_PATH}.previous"
  run cp -p "$INSTALL_PATH" "$PREVIOUS"
  say "kept the working binary as ${PREVIOUS}"

  # 3. Replace it.
  step "Installing the new binary"
  install_binary
  NEW_VERSION="$("$INSTALL_PATH" --version 2>/dev/null || echo unknown)"
  say "now installed: ${NEW_VERSION}"

  # 4. The units, too. An upgrade that replaces only the binary leaves this
  #    installation running whatever unit its original release wrote — which
  #    is how a binary that speaks systemd's readiness protocol ends up under
  #    a `Type=exec` unit that never waits for it, and how a sandbox
  #    directive added in this release never reaches the machine that most
  #    needs it. Both are written from this script, so both are upgraded.
  step "Refreshing the configuration and the units"
  # The new unit's ExecStart carries no arguments, so the env file has to name
  # the bundle before that unit is asked to start anything.
  ensure_env_keys
  # Keep the units that work, beside the binary that works. A rollback that
  # restores the binary and leaves the new unit in place puts an old binary
  # under a unit written for a new one — which is how the first rollback here
  # failed as comprehensively as the upgrade it was undoing.
  for u in "$UNIT_PATH" "$BACKUP_SERVICE_PATH" "$BACKUP_TIMER_PATH" \
           "$VERIFY_SERVICE_PATH" "$VERIFY_TIMER_PATH"; do
    [ -f "$u" ] && run cp -p "$u" "${u}.previous"
  done
  write_units
  if [ "$SKIP_PRIVILEGED" = "0" ] && [ "$DRY_RUN" = "0" ] && command -v systemctl >/dev/null 2>&1; then
    run systemctl daemon-reload
  fi

  # 5. Restart and ask it whether it can read the family's data.
  step "Restarting and checking /health"
  if [ "$SKIP_PRIVILEGED" = "1" ] || [ "$DRY_RUN" = "1" ]; then
    say "skipping restart and health check"
  elif systemctl restart axgf-cms && HEALTH="$(wait_for_health)"; then
    say "healthy: ${HEALTH}"

    # An installation upgraded from a release that had no backups has no
    # archive at all, only the file copy taken above — and a copy is not a
    # verified archive. The new binary can write one, so it does, now rather
    # than at half past three.
    case "$HEALTH" in
      *'"backup"'*'"status":"fail"'*|*'"backup":{"status":"fail"'*)
        step "First verified archive"
        if systemctl start axgf-cms-backup.service; then
          say "wrote one with the new binary"
        else
          warn "could not: check systemctl status axgf-cms-backup"
        fi
        ;;
    esac

    step "Done"
    cat <<EOF

  Upgraded ${OLD_VERSION} -> ${NEW_VERSION}.

  The previous binary is kept at ${PREVIOUS}. The pre-upgrade backup is the
  newest archive in ${BACKUP_DIR}.

EOF
    exit 0
  else
    # 6. Roll back. Automatically, without asking, because the alternative is
    #    a family's site being down while somebody reads documentation.
    warn "the new binary did not come up healthy:"
    printf '%s\n' "${HEALTH:-<no response>}" | sed 's/^/    /'
    step "Rolling back"
    run install -m 0755 "$PREVIOUS" "$INSTALL_PATH"
    # Everything the upgrade replaced, not merely the binary.
    for u in "$UNIT_PATH" "$BACKUP_SERVICE_PATH" "$BACKUP_TIMER_PATH" \
             "$VERIFY_SERVICE_PATH" "$VERIFY_TIMER_PATH"; do
      [ -f "${u}.previous" ] && run cp -p "${u}.previous" "$u"
    done
    if command -v systemctl >/dev/null 2>&1; then
      run systemctl daemon-reload
    fi
    run systemctl restart axgf-cms
    if HEALTH="$(wait_for_health)"; then
      die "the upgrade to ${NEW_VERSION} failed its health check and was rolled back.
  ${OLD_VERSION} is running again and the site is up.

  Nothing was lost: the pre-upgrade backup is in ${BACKUP_DIR}, and the bundle
  was never touched by the new binary.

  Report the version that failed before trying again."
    fi
    die "the upgrade to ${NEW_VERSION} failed, AND the rollback to ${OLD_VERSION}
  did not come up either. The site is down.

  Last response from /health:
$(printf '%s\n' "${HEALTH:-<none>}" | sed 's/^/    /')

  Restore from the pre-upgrade backup:

    sudo systemctl stop axgf-cms
    sudo -u ${SERVICE_USER} ${INSTALL_PATH} restore --bundle ${BUNDLE} \\
      ${BACKUP_DIR}/<newest archive>
    sudo systemctl start axgf-cms"
  fi
fi

step "Installing the binary"
install_binary

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
  run mkdir -p "$DATA_DIR" "$CONF_DIR" "$BACKUP_DIR"
else
  run install -d -o "$SERVICE_USER" -g "$SERVICE_USER" -m 0750 "$DATA_DIR"
  run install -d -o root -g "$SERVICE_USER" -m 0750 "$CONF_DIR"
  # The archives hold the accounts file, so this directory is no more public
  # than the ACL is.
  run install -d -o "$SERVICE_USER" -g "$SERVICE_USER" -m 0700 "$BACKUP_DIR"
fi
say "$DATA_DIR, $CONF_DIR and $BACKUP_DIR ready"

# --------------------------------------------------------------------------
step "Configuration"
# --------------------------------------------------------------------------
# Every setting the service has lives in this one file, and the unit's
# ExecStart is the binary and nothing else. Changing where the bundle lives,
# what address it binds, where backups go or how much it logs is an edit here
# and `systemctl restart axgf-cms` — never a unit to edit, never a flag in two
# places that have to agree.
#
# The token is generated once and never regenerated: rewriting it on a second
# run would silently lock the operator out of a working install.
if [ -f "$ENV_FILE" ]; then
  say "$ENV_FILE exists — keeping the existing token and settings"
  TOKEN="$(sed -n 's/^AXGF_CMS_ADMIN_TOKEN=//p' "$ENV_FILE" | head -1)"
  ensure_env_keys
else
  if [ "$DRY_RUN" = "1" ]; then
    TOKEN="<generated-on-first-real-run>"
  else
    TOKEN="$(head -c 32 /dev/urandom | od -An -tx1 | tr -d ' \n')"
  fi
  write_file "$ENV_FILE" 0600 <<EOF
# axgf-cms — every setting this service has.
#
# Edit a value and \`systemctl restart axgf-cms\`. Nothing else reads these:
# the unit runs the binary with no arguments at all, so what is here is what
# the service is.
#
# This file holds the emergency admin token, so it is root:axgf-cms 0640 —
# the service can read it and nobody else can.

# The genealogy itself. Everything else is derived from it or beside it.
AXGF_CMS_BUNDLE=${BUNDLE}

# Localhost, and it should stay there: this process speaks plain HTTP. To
# publish the site, put a TLS reverse proxy in front of it — docs/DEPLOY.md
# has tested nginx and Caddy configurations. Binding anything wider than
# localhost without one exposes a family's records to anyone who reaches the
# port, in clear text, passwords included.
AXGF_CMS_BIND=${BIND}

# Where the daily archives go, and what /health measures the backup age
# against. An installation with this unset cannot tell a working timer from
# one that stopped firing.
AXGF_CMS_BACKUP_DIR=${BACKUP_DIR}

# Derived data — extracted photographs and documents. Safe to delete; the
# next save rebuilds it. It is not backed up, on purpose.
AXGF_CMS_CACHE_DIR=${DATA_DIR}/cache

# How much reaches the journal. \`info\` is a handful of lines a day;
# \`axgf_cms=debug\` is several per request and will bury the week you need.
RUST_LOG=axgf_cms=info,tower_http=warn

# The way back in when the accounts file is lost or every administrator is
# locked out. It is not an account: it opens an administrator session through
# the login form, throttled like any other sign-in, and every use is logged as
# a warning. The dashboard says, to anyone signed in, that it is still here —
# remove this line and restart once you have a working account.
AXGF_CMS_ADMIN_TOKEN=${TOKEN}
EOF
  if [ "$SKIP_PRIVILEGED" = "0" ]; then
    run chown root:"$SERVICE_USER" "$ENV_FILE"
    run chmod 0640 "$ENV_FILE"
  fi
  say "wrote $ENV_FILE with a new admin token"
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


write_units


if [ "$SKIP_PRIVILEGED" = "1" ]; then
  say "skipping systemctl (AXGF_CMS_SKIP_PRIVILEGED)"
elif command -v systemctl >/dev/null 2>&1; then
  run systemctl daemon-reload
  run systemctl enable axgf-cms
  run systemctl restart axgf-cms
  run systemctl enable --now axgf-cms-backup.timer
  run systemctl enable --now axgf-cms-verify.timer
  say "service enabled and started; daily backup and weekly verify timers enabled"

  # And one backup now, rather than at half past three tomorrow morning.
  #
  # Two reasons. An installation whose first night has not happened yet has no
  # archive at all, which /health reports as a failure — correctly, since the
  # data is one disk away from gone — so a fresh install would answer 503 to a
  # monitor for its first day. And the first hour after an install is when
  # somebody is importing a GEDCOM and finding out what the import did to it;
  # having a backup from before that is worth more than one from after.
  if [ "$DRY_RUN" = "0" ]; then
    if systemctl start axgf-cms-backup.service; then
      say "took the first backup now, rather than at ${BACKUP_AT} tomorrow"
    else
      warn "the first backup did not run; check systemctl status axgf-cms-backup"
    fi
  fi
fi

# --------------------------------------------------------------------------
step "Done"
# --------------------------------------------------------------------------
cat <<EOF

  ax-genealogy is running.

    URL          http://${BIND}/
    Sign in      http://${BIND}/admin/login

    Bundle       ${BUNDLE}       <- the genealogy; share it freely
    Accounts     ${ACL_FILE}       <- mode 600; share it with nobody
    Backups      ${BACKUP_DIR}       <- one verified archive a day, at ${BACKUP_AT}
    Config       ${ENV_FILE}
    Logs         journalctl -u axgf-cms -f
    Health       curl -s http://${BIND}/health
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

  BACKUPS: one verified archive a day lands in

    ${BACKUP_DIR}

  That is the same disk as the bundle, so it does not survive that disk.
  Copy the archives somewhere else — a nightly

    rsync -a --delete ${BACKUP_DIR}/ user@another-machine:/srv/axgf-backups/

  is enough, and docs/OPERATOR.md has the rclone version for a cloud bucket.
  Until you do this, you have a backup of a mistake, not of a disk failure.

  SECURITY: this binds to localhost by design. To publish it, put a TLS
  reverse proxy in front — see docs/DEPLOY.md — and do not move the bind
  address to 0.0.0.0 without one. Records marked \`private\` or \`members\` are
  withheld from signed-out visitors, but a bundle converted from GEDCOM
  carries no visibility at all: there, everyone recorded as living is treated
  as \`members\` and everyone else is public. Check that this is what you want
  before publishing.

EOF
