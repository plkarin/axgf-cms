#!/usr/bin/env bash
# Cold-start and warm timings for the river, against a real bundle.
#
#   scripts/river-bench.sh BUNDLE [RUNS] [PORT]
#
# Builds nothing: run `cargo build --release` first. Each run starts a fresh
# server on a copy of BUNDLE, times the first river request and the one after
# it, and stops the server.
#
# This script exists because ad-hoc versions of it left servers running three
# times, one of them for thirteen days. So:
#
#  * the preamble stops every axgf-cms that runs a binary from *this
#    checkout's* target/ directory, and nothing else — an installed service
#    (/usr/local/bin/axgf-cms, under systemd) is somebody's live archive and is
#    never matched;
#  * the port must be free before a run starts, or the run would time a stale
#    server, which is how one batch of cold-start numbers was once wrong;
#  * the server is launched directly, not inside a `cd … &&` list, so `$!` is
#    the server and not a subshell, and an EXIT trap stops it however the
#    script ends;
#  * it binds to 127.0.0.1 explicitly;
#  * the bundle is copied into $SCRATCH (default ~/scratch/river-bench), never
#    /tmp, and the original is only read.
set -euo pipefail

BUNDLE=${1:?usage: river-bench.sh BUNDLE [RUNS] [PORT]}
RUNS=${2:-3}
PORT=${3:-18731}
REPO=$(cd "$(dirname "$0")/.." && pwd)
BIN="$REPO/target/release/axgf-cms"
SCRATCH=${SCRATCH:-$HOME/scratch/river-bench}

[ "$(id -u)" -ne 0 ] || { echo "refusing to run as root" >&2; exit 1; }
[ -x "$BIN" ] || { echo "no $BIN; run cargo build --release" >&2; exit 1; }

# Stop development servers from this checkout, by the binary they run.
for pid in $(pgrep -x axgf-cms || true); do
    exe=$(readlink "/proc/$pid/exe" 2>/dev/null || true)
    case "$exe" in
        "$REPO"/target/*)
            echo "stopping development server $pid ($exe)"
            kill "$pid" 2>/dev/null || true
            ;;
    esac
done
sleep 1

port_busy() { ss -ltn "( sport = :$PORT )" | grep -q LISTEN; }
if port_busy; then
    echo "port $PORT is still in use:" >&2
    ss -ltnp "( sport = :$PORT )" >&2
    exit 1
fi

mkdir -p "$SCRATCH"
cp "$BUNDLE" "$SCRATCH/bundle.axgf"
export TMPDIR="$SCRATCH"

server=""
stop() {
    if [ -n "$server" ] && kill -0 "$server" 2>/dev/null; then
        kill "$server"
        wait "$server" 2>/dev/null || true
    fi
    server=""
}
trap stop EXIT

ms() { awk -v s="$1" 'BEGIN { printf "%.1f", s * 1000 }'; }

for run in $(seq 1 "$RUNS"); do
    port_busy && { echo "port $PORT busy before run $run" >&2; exit 1; }
    "$BIN" --bundle "$SCRATCH/bundle.axgf" --bind "127.0.0.1:$PORT" \
        --admin-token "river-bench-$RANDOM$RANDOM" >"$SCRATCH/server.log" 2>&1 &
    server=$!
    start=$(date +%s.%N)
    until curl -s -o /dev/null "http://127.0.0.1:$PORT/static/app.css"; do
        kill -0 "$server" 2>/dev/null || { echo "server exited:" >&2; tail -5 "$SCRATCH/server.log" >&2; exit 1; }
        sleep 0.05
    done
    ready=$(date +%s.%N)
    first=$(curl -s -o /dev/null -w '%{time_total}' "http://127.0.0.1:$PORT/")
    second=$(curl -s -o /dev/null -w '%{time_total}' "http://127.0.0.1:$PORT/")
    other=$(curl -s -o /dev/null -w '%{time_total}' -H 'Cookie: axgf_lang=ja' "http://127.0.0.1:$PORT/?n=5")
    echo "run $run: ready $(awk -v a="$start" -v b="$ready" 'BEGIN { printf "%.2f", b - a }') s · first / $(ms "$first") ms · second $(ms "$second") ms · first in Japanese, ±5 $(ms "$other") ms"
    stop
done
