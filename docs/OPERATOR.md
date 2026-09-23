# Running axgf-cms

For whoever looks after the machine. Every command here has been run, in this
order, on Ubuntu 24.04 LTS.

If you remember one thing: **`family.axgf` is the whole genealogy**, and the
`.acl` beside it is everyone's accounts. Copy those two off this machine
regularly and nothing else here can hurt you very much.

- [Install](#install)
- [The five commands](#the-five-commands)
- [What systemctl status tells you](#what-systemctl-status-tells-you)
- [Backups](#backups)
- [Restore](#restore)
- [Upgrading](#upgrading)
- [Health, and plugging it into a monitor](#health-and-plugging-it-into-a-monitor)
- [When the disk fills](#when-the-disk-fills)
- [Publishing it: TLS and a reverse proxy](#publishing-it-tls-and-a-reverse-proxy)
- [When to worry](#when-to-worry)
- [Uninstalling](#uninstalling)

---

## Install

```sh
curl -fsSL https://raw.githubusercontent.com/plkarin/axgf-cms/main/deploy/bootstrap.sh \
  | sudo bash -s -- --with-sample
```

It creates a system user with no shell, a data directory, a configuration
file, three units and two timers; installs the binary; makes the first
administrator account and prints its password once; and takes a backup
immediately rather than waiting for the first nightly one.

Drop `--with-sample` if you are going to import your own GEDCOM. Other options
worth knowing:

| Option | What it does |
| --- | --- |
| `--bind 127.0.0.1:8080` | Where it listens. Keep it on localhost — see [Publishing it](#publishing-it-tls-and-a-reverse-proxy). |
| `--backup-dir /srv/backups` | Where archives go. Put this on a different disk if you have one. |
| `--backup-at 03:30` | When the nightly backup runs, as a systemd `OnCalendar` expression. |
| `--admin-user gran` | Username for the first account. |
| `--from-source` | Build with cargo instead of downloading a release. |
| `--dry-run` | Print what it would do and change nothing. |

Write the password down when it is printed. It is stored only as an Argon2id
hash and cannot be recovered — but you are not locked out if you lose it: see
[the emergency token](#the-emergency-token).

## The five commands

An operator needs `systemctl` and `journalctl`, and nothing else.

```sh
systemctl status axgf-cms          # is it up, what is it serving, when was the last backup
systemctl restart axgf-cms         # after changing /etc/axgf-cms/env
systemctl stop axgf-cms            # finishes the save in flight first
systemctl start axgf-cms
journalctl -u axgf-cms -f          # what it is doing
```

The scheduled work is separate, so a backup that hangs cannot take the website
with it:

```sh
systemctl list-timers 'axgf-cms*'          # when each next runs
systemctl start axgf-cms-backup.service    # back up now, without waiting for tonight
systemctl start axgf-cms-verify.service    # read everything back now
journalctl -u axgf-cms-backup --since today
journalctl -u axgf-cms-verify  --since '1 week ago'
```

Everything the service is configured by is in one file:

```sh
sudoedit /etc/axgf-cms/env
sudo systemctl restart axgf-cms
```

```ini
AXGF_CMS_BUNDLE=/var/lib/axgf-cms/family.axgf
AXGF_CMS_BIND=127.0.0.1:8080
AXGF_CMS_BACKUP_DIR=/var/lib/axgf-cms/backups
AXGF_CMS_CACHE_DIR=/var/lib/axgf-cms/cache
RUST_LOG=axgf_cms=info,tower_http=warn
AXGF_CMS_ADMIN_TOKEN=…
```

There is nothing to change in the unit file. `ExecStart` runs the binary with
no arguments at all, on purpose: one place to change a setting, and no flag
that can disagree with this file.

### The emergency token

`AXGF_CMS_ADMIN_TOKEN` is the way back in when the accounts file is lost or
every administrator is locked out. It is not an account. Paste it into the
**token** field on the sign-in page; it opens an ordinary administrator
session, it is rate-limited like any other sign-in, and every use — and every
wrong guess — is a warning in the journal:

```sh
journalctl -u axgf-cms | grep -i 'emergency admin token'
```

Once somebody can sign in normally, delete that line and restart. Until you
do, the admin dashboard says so on every visit.

## What systemctl status tells you

```
● axgf-cms.service - axgf-cms — AXGF genealogy showcase
     Active: active (running) since Wed 2026-09-23 20:02:44 UTC
     Status: "/var/lib/axgf-cms/family.axgf · 866 people · newest archive
              axgf-backup-20260923T200223Z.zip is under an hour old"
```

The `Status:` line is the three things you opened `status` to find out: which
bundle is loaded, how many people are in it, and whether the backups are still
happening. It refreshes every minute.

`Active: active (running)` means the bundle is loaded, validated and listening
— not merely that a process was started. On a 435 MB archive those are several
seconds apart, and `systemctl start` waits for the second one.

## Backups

One verified archive a day, at 03:30 with up to an hour of jitter, catching up
if the machine was off:

```sh
systemctl list-timers axgf-cms-backup.timer
ls -lh /var/lib/axgf-cms/backups/
```

Each archive is an ordinary ZIP holding the bundle, the accounts and every
journal segment. It is read back before it is given its final name — the CRCs,
each member's SHA-256 against the manifest, the bundle re-imported and
validated — and one that fails is deleted rather than left looking like a
backup. Retention keeps 7 daily, 4 weekly and 12 monthly.

You can open one with `unzip`. That is deliberate: the day a backup is needed
is the worst possible day to discover that reading it needs this program.

```sh
axgf-cms verify /var/lib/axgf-cms/backups/axgf-backup-20260923T200223Z.zip
```

### Off this machine

**An archive on the same disk as the bundle does not survive that disk.** The
nightly backup protects you from a mistake, not from a failure. One of these,
as a cron job or a second systemd timer:

```sh
rsync -a --delete /var/lib/axgf-cms/backups/ backup@othermachine:/srv/axgf/
```

```sh
rclone sync /var/lib/axgf-cms/backups/ remote:axgf-backups/
```

Both read the archives as the service user wrote them (mode 0600), so run them
as `root` or as `axgf-cms`. Until one of these runs somewhere, you have a
backup of a mistake.

### The weekly read-back

Once a week, `axgf-cms-verify.service` loads the bundle and validates it,
checks the payload cache against what the bundle declares, and reads the newest
archive back in full. It fails the unit if any of that is wrong:

```sh
systemctl is-failed axgf-cms-verify     # "failed" is the answer you act on
journalctl -u axgf-cms-verify -n 20
```

## Restore

Restoring refuses to run while the service is up, keeps the current files
aside before touching anything, and verifies the archive completely first.

```sh
sudo systemctl stop axgf-cms
sudo -u axgf-cms axgf-cms restore \
  --bundle /var/lib/axgf-cms/family.axgf \
  /var/lib/axgf-cms/backups/axgf-backup-20260923T200223Z.zip
sudo systemctl start axgf-cms
```

What it replaced is moved into a timestamped directory beside the bundle, not
deleted: restoring the wrong archive at three in the morning has to be
reversible.

## Upgrading

```sh
curl -fsSL https://raw.githubusercontent.com/plkarin/axgf-cms/main/deploy/bootstrap.sh \
  | sudo bash -s -- --upgrade
```

In order: a backup with the binary that is known to work; the working binary
and the current units kept aside; the new binary installed; the configuration
topped up and the units refreshed; a restart; and `/health` asked whether it
can read the family's data. If it cannot, the previous binary **and** the
previous units go back automatically and the site comes up again on the
version that worked.

Upgrading from a release old enough to have no `backup` command is handled: the
three files are copied aside with the service stopped first, and the new binary
writes a proper verified archive as soon as it is up.

## Health, and plugging it into a monitor

```sh
curl -s http://127.0.0.1:8080/health | jq
```

Four checks — the bundle loads and validates, free disk against a threshold,
the age of the newest backup, and whether the payload cache still holds every
file the bundle declares. The HTTP status is the worst of them:

| Status | Meaning | What a monitor should do |
| --- | --- | --- |
| `200` + `"status":"ok"` | Everything is fine. | Nothing. |
| `200` + `"status":"warn"` | Somebody should look this week: backup 2+ days old, disk under 10%. | Notice it. Do not page anyone. |
| `503` | Broken or about to be: the bundle does not validate, or the disk is too full for the next save. | Alert. |

A warning answers 200 on purpose: nobody should be woken at midnight over a
stale backup, and a monitor that pages on warnings is muted within a week.

Any uptime monitor that understands "non-200 is down" therefore works with no
configuration at all. Point it at `https://your-name/health` through the proxy,
every 60 seconds:

- **Uptime Kuma** — Monitor type HTTP(s), URL `https://tree.example.org/health`,
  accepted status codes `200-299`, interval 60.
- **systemd, on the same machine** — no extra software:

  ```ini
  # /etc/systemd/system/axgf-cms-watch.service
  [Service]
  Type=oneshot
  ExecStart=/usr/bin/curl -fsS --max-time 10 http://127.0.0.1:8080/health
  ```

  with a timer every five minutes and `OnFailure=` pointing at whatever sends
  you mail. `curl -f` fails on 503, which is exactly the signal.
- **Anything else** — a 60-second HTTP check on `/health`, alert on non-2xx.

The same four checks appear at the top of the admin dashboard, in the reader's
own language, for a household with no monitor at all.

## When the disk fills

The application refuses to start a write it cannot finish. A save on a nearly
full disk returns a page saying so, and nothing is written:

```
Could not save
not enough free space to write the bundle, so nothing was written and the
existing data is untouched.
  needs      16.0 MB
  free        5.2 MB on the filesystem holding /var/lib/axgf-cms
```

`/health` answers 503 before that point, and `systemctl status` shows it. To
recover:

```sh
df -h /var/lib/axgf-cms                       # confirm it is this filesystem
sudo du -sh /var/lib/axgf-cms/* | sort -h     # where it went
sudo ls -lh /var/lib/axgf-cms/backups/        # usually the archives
```

Move the archives off the machine (see [Off this
machine](#off-this-machine)) and delete the oldest. Two things that look
alarming and are not:

- `/var/lib/axgf-cms/cache` is extracted photographs. It is derived data, safe
  to delete, and the next save rebuilds it. Stop the service first.
- a `family.axgf.tmp` or a `.zip.part` is wreckage from a crash. The service
  removes them itself — at startup for the bundle's, at the next backup for an
  archive's — and the size they were taking comes back.

Why a save needs twice the bundle's size: the archive is rebuilt beside itself
and renamed into place, so both exist for a moment. That is what makes a save
atomic, and it is why the free-space check asks for the whole bundle rather
than for the size of your edit.

## Publishing it: TLS and a reverse proxy

**The service binds 127.0.0.1 and must stay there.** It speaks plain HTTP.
Bound wider than localhost with nothing in front of it, every password typed
into the sign-in form crosses the network in clear text, the session cookie is
issued without `Secure`, and a family's private records are readable by anyone
who can reach the port.

Put a TLS reverse proxy in front instead. Both of these are in the repository
and both have been tested in front of a live instance:

```sh
sudo cp deploy/proxy/nginx-axgf-cms.conf /etc/nginx/sites-available/axgf-cms
sudo ln -s /etc/nginx/sites-available/axgf-cms /etc/nginx/sites-enabled/
sudo certbot --nginx -d tree.example.org
sudo nginx -t && sudo systemctl reload nginx
```

```sh
sudo cp deploy/proxy/Caddyfile /etc/caddy/Caddyfile
sudo systemctl reload caddy      # Caddy gets and renews the certificate itself
```

Replace `tree.example.org` in either. What those files get right, and what to
keep if you write your own:

- `X-Forwarded-Proto` — without it the session cookie is issued without
  `Secure`, over TLS, and nothing says so.
- `X-Forwarded-For`, **set** from the peer rather than appended to — the login
  rate limit buckets by client, and a client that supplies its own value picks
  its own bucket.
- a body limit just above the product's 10 MB upload limit, so an oversized
  upload is refused by a page that explains itself rather than by a bare 413.
- a read timeout well past 60 seconds. A save rebuilds the whole archive; at
  five seconds the same save reaches the reader as a 504 while the write
  carries on regardless.
- **no** `Content-Security-Policy`, `X-Frame-Options`, `Referrer-Policy`,
  `X-Content-Type-Options` or `Permissions-Policy`. The application sets all
  five on every response. Adding them at the proxy sends two of each, and two
  CSP headers are enforced as the intersection of the two — which breaks the
  page in a way nothing reports. HSTS is the exception: only the thing
  terminating TLS can promise that, so the proxy adds it.

## When to worry

| What you see | How bad | What to do |
| --- | --- | --- |
| `systemctl status` says `active (running)` | Fine | Nothing. |
| Status line: backup "under an hour old" | Fine | Nothing. |
| Dashboard banner, amber: backup 2+ days old | This week | `systemctl list-timers`, then `journalctl -u axgf-cms-backup`. |
| Dashboard banner, amber: disk under 10% | This week | [When the disk fills](#when-the-disk-fills), before it does. |
| `/health` returns 503 | Today | Read the JSON: it says which check failed and what to do. |
| `systemctl is-failed axgf-cms-verify` says `failed` | Today | An archive did not read back. `journalctl -u axgf-cms-verify`. Do not delete older archives until you know why. |
| Repeated "Scheduled restart job" in the journal | Today | Something is crashing. `journalctl -u axgf-cms -p err`. The restart delay grows to two minutes, so the machine is not being hammered while you look. |
| The site is up but a photograph 404s | This week | The cache lost a file. It comes back on the next save; `systemctl start axgf-cms-verify` reports how many are missing. |
| "an emergency administrator token is still set" | This month | Remove `AXGF_CMS_ADMIN_TOKEN` from `/etc/axgf-cms/env` and restart. |

## Uninstalling

```sh
sudo bash deploy/bootstrap.sh --uninstall
```

Stops and removes the service, both timers and the binary. It does not remove
one byte of data, and it prints where everything was left: the bundle, the
accounts, the journal, the archives and the configuration. Reinstalling over it
picks all of it up again, including the token.

Deleting the genealogy is three commands the script prints and will not run for
you.
