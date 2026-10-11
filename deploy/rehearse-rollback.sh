#!/usr/bin/env bash
#
# Rehearse an upgrade's rollback on this installation, before the upgrade.
#
#   sudo bash rehearse-rollback.sh --version v0.3.1
#
# A REQUIRED step of every deploy (docs/DEPLOY.md, "Deploying a release"). It
# has caught a serious defect each time it has been run: the health check
# asking the wrong port (it would have rolled back a healthy upgrade), and a
# pre-upgrade backup that evicted the day's daily archive.
#
# It fetches the installer the target release ships, checks it against its
# checksum, and runs its --upgrade with a stand-in binary that refuses to
# start. The installer must take its backup, install the stand-in, see it
# fail, and put the running version back on its own. Then this checks that
# it did:
#
#   * the binary, every unit file, the bundle and the accounts are
#     byte-identical to before;
#   * no daily archive was removed;
#   * the service is active and /health says the bundle is ok;
#   * how long the site was unreachable.
#
# It prints PASS or FAIL and exits 0 or 1. On FAIL, do not deploy.
set -euo pipefail

REPO="plkarin/axgf-cms"
RELEASE_BASE="${AXGF_CMS_RELEASE_BASE:-https://github.com/${REPO}}"
CONF="/etc/axgf-cms/env"
BIN="/usr/local/bin/axgf-cms"
VERSION=""
while [ $# -gt 0 ]; do
  case "$1" in
    --version) VERSION="${2:?--version needs a tag, e.g. --version v0.3.1}"; shift ;;
    -h|--help) sed -n '2,/^set -euo/p' "$0" | sed 's/^# \{0,1\}//' | grep -v '^set -euo'; exit 0 ;;
    *) echo "unknown option: $1" >&2; exit 2 ;;
  esac
  shift
done
[ -n "$VERSION" ] || { echo "--version is required: rehearse with the release you are about to deploy" >&2; exit 2; }
[ "$(id -u)" = "0" ] || { echo "run as root (sudo)" >&2; exit 2; }
[ -x "$BIN" ] && [ -f "$CONF" ] || { echo "no installation here ($BIN, $CONF)" >&2; exit 2; }

env_value() { sed -n "s/^$1=//p" "$CONF" | tail -1; }
BIND="$(env_value AXGF_CMS_BIND)"; BIND="${BIND:-127.0.0.1:8080}"
BUNDLE="$(env_value AXGF_CMS_BUNDLE)"
BACKUPS="$(env_value AXGF_CMS_BACKUP_DIR)"
ACL="${BUNDLE%.axgf}.acl"

WORK="$(mktemp -d)"
POLLER=""
cleanup() { [ -n "$POLLER" ] && kill "$POLLER" 2>/dev/null; rm -rf "$WORK"; }
trap cleanup EXIT

echo "==> Fetching the installer ${VERSION} ships"
curl -fsSL "${RELEASE_BASE}/releases/download/${VERSION}/bootstrap.sh" -o "$WORK/bootstrap.sh"
curl -fsSL "${RELEASE_BASE}/releases/download/${VERSION}/bootstrap.sh.sha256" -o "$WORK/bootstrap.sh.sha256"
( cd "$WORK" && sed "s#\([a-f0-9]\{64\}\).*#\1  bootstrap.sh#" bootstrap.sh.sha256 | sha256sum -c - >/dev/null ) \
  || { echo "FAIL: the installer does not match its checksum" >&2; exit 1; }
grep -qx "SCRIPT_VERSION=\"${VERSION}\"" "$WORK/bootstrap.sh" \
  || { echo "FAIL: that installer is not stamped ${VERSION}" >&2; exit 1; }
echo "  checksum verified, stamped ${VERSION}"

echo "==> Recording the installation as it is"
units=(/etc/systemd/system/axgf-cms*.service /etc/systemd/system/axgf-cms*.timer)
sha256sum "$BIN" "${units[@]}" "$BUNDLE" "$ACL" > "$WORK/before.sums"
ls -1 "$BACKUPS" 2>/dev/null | grep '^axgf-backup-.*\.zip$' > "$WORK/before.dailies" || true
echo "  $(wc -l < "$WORK/before.sums") files, $(wc -l < "$WORK/before.dailies") daily archives, running $("$BIN" --version)"
df -h "$(dirname "$BUNDLE")" | tail -1 | awk '{print "  free: " $4 " (" $5 " used)"}'

printf '#!/bin/sh\ncase "$1" in --version) echo "axgf-cms 0.0.0-rollback-rehearsal"; exit 0 ;; esac\necho "rollback rehearsal: refusing to start" >&2\nexit 1\n' > "$WORK/probe"
chmod 755 "$WORK/probe"

( while :; do printf '%s %s\n' "$(date +%s.%N)" "$(curl -s -m 2 -o /dev/null -w '%{http_code}' "http://${BIND}/health")"; sleep 0.5; done ) > "$WORK/poll" 2>&1 &
POLLER=$!

echo "==> Upgrading to a binary that will not start"
set +e
AXGF_CMS_LOCAL_BINARY="$WORK/probe" bash "$WORK/bootstrap.sh" --upgrade > "$WORK/upgrade.log" 2>&1
rc=$?
set -e
sleep 3
kill "$POLLER" 2>/dev/null; POLLER=""

verdict=PASS
fail() { echo "  FAIL: $*"; verdict=FAIL; }
log="$(sed 's/\x1b\[[0-9;]*m//g' "$WORK/upgrade.log")"
[ "$rc" != "0" ] || fail "the installer reported success with a binary that cannot start"
printf '%s\n' "$log" | grep -q "was rolled back" || fail "the installer did not report a rollback"
printf '%s\n' "$log" | grep -q "is running again and the site is up" || fail "the installer did not report the site up again"
sha256sum "$BIN" "${units[@]}" "$BUNDLE" "$ACL" > "$WORK/after.sums"
diff -q "$WORK/before.sums" "$WORK/after.sums" >/dev/null || { fail "these changed:"; diff "$WORK/before.sums" "$WORK/after.sums" | sed 's/^/    /'; }
ls -1 "$BACKUPS" 2>/dev/null | grep '^axgf-backup-.*\.zip$' > "$WORK/after.dailies" || true
lost="$(comm -23 "$WORK/before.dailies" "$WORK/after.dailies")"
[ -z "$lost" ] || fail "daily archives were removed: $lost"
systemctl is-active --quiet axgf-cms || fail "the service is not active"
curl -s -m 3 "http://${BIND}/health" | grep -q '"bundle":{"status":"ok"' || fail "/health does not report the bundle ok"
down="$(awk '$2!="200"{if(!s)s=$1; e=$1} END{if(s) printf "%.1f", e-s+0.5; else print "0"}' "$WORK/poll")"

echo "==> Result"
printf '%s\n' "$log" | grep -E "kept as|retention|rolled back|running again" | sed 's/^ */  installer: /'
echo "  unreachable for about ${down} s"
echo "$verdict"
[ "$verdict" = PASS ]
