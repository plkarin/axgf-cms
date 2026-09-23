# Deploying axgf-cms

The whole application is one binary and one file. If you remember nothing else
from this document, remember that **`family.axgf` is the entire database** —
copy it, together with the `.acl` beside it, and you have a complete backup.

- [One-line install](#one-line-install)
- [Manual installation](#manual-installation)
- [systemd reference](#systemd-reference)
- [Reverse proxy and TLS](#reverse-proxy-and-tls)
- [Backups](#backups)
- [Upgrading](#upgrading)
- [Troubleshooting](#troubleshooting)

---

## One-line install

```sh
curl -fsSL https://raw.githubusercontent.com/plkarin/axgf-cms/main/deploy/bootstrap.sh \
  | sudo bash -s -- --with-sample
```

| Option | Effect |
|---|---|
| `--with-sample` | Seed a *new* bundle with the demonstration family. Ignored if a bundle already exists. |
| `--from-source` | Build with cargo instead of downloading a release binary. |
| `--version <TAG>` | Install a specific release tag, e.g. `--version v0.1.0-rc1`. The default resolves `/releases/latest`, which is the latest *stable* release — a release candidate is published as a prerelease and is reachable only by tag. |
| `--bind <ADDR>` | Address for the unit to bind (default `127.0.0.1:8080`). |
| `--admin-user <NAME>` | Username for the administrator created on a fresh install (default `admin`). |
| `--dry-run` | Print every action and change nothing. |

The script is safe to re-run. It will not overwrite an existing bundle, will
not regenerate an existing emergency token, and will not touch an existing
`.acl` — so a re-run never silently rotates a working account's password. It
prints which of those it is preserving. If you want to see exactly what it will do first:

```sh
sudo bash bootstrap.sh --dry-run --with-sample
```

What it creates:

| Path | Purpose |
|---|---|
| `/usr/local/bin/axgf-cms` | The binary. Nothing else is installed. |
| `/var/lib/axgf-cms/family.axgf` | **The database.** The genealogy — shareable. |
| `/var/lib/axgf-cms/family.acl` | **The accounts.** Mode 0600, owned by the service user. Never share this. |
| `/etc/axgf-cms/env` | Emergency recovery token, mode 0640, root-owned. |
| `/etc/systemd/system/axgf-cms.service` | The unit. |
| user `axgf-cms` | System user, no shell, no login. |

---

## Manual installation

If you would rather not run a script:

```sh
# 1. Binary
ARCH=$(uname -m)
case "$ARCH" in
  x86_64)  TARGET=x86_64-unknown-linux-musl ;;
  aarch64) TARGET=aarch64-unknown-linux-gnu ;;
esac
curl -fsSLO "https://github.com/plkarin/axgf-cms/releases/latest/download/axgf-cms-${TARGET}.tar.gz"
curl -fsSLO "https://github.com/plkarin/axgf-cms/releases/latest/download/axgf-cms-${TARGET}.tar.gz.sha256"
sha256sum -c "axgf-cms-${TARGET}.tar.gz.sha256"
tar -xzf "axgf-cms-${TARGET}.tar.gz"
sudo install -m 0755 axgf-cms-*/axgf-cms /usr/local/bin/axgf-cms

# 2. User and directories
sudo useradd --system --home-dir /var/lib/axgf-cms --shell /usr/sbin/nologin axgf-cms
sudo install -d -o axgf-cms -g axgf-cms -m 0750 /var/lib/axgf-cms
sudo install -d -o root -g axgf-cms -m 0750 /etc/axgf-cms

# 3. Admin token
printf 'AXGF_CMS_ADMIN_TOKEN=%s\n' "$(head -c 32 /dev/urandom | od -An -tx1 | tr -d ' \n')" \
  | sudo tee /etc/axgf-cms/env >/dev/null
sudo chown root:axgf-cms /etc/axgf-cms/env
sudo chmod 0640 /etc/axgf-cms/env

# 4. The bundle and the first administrator, in one step and BEFORE the
#    service exists. --seed-sample only acts when the bundle is absent, so
#    this creates it; drop the flag to start from an empty tree. Doing this
#    first is what keeps the service from racing the bundle into existence.
sudo -u axgf-cms /usr/local/bin/axgf-cms \
  --bundle /var/lib/axgf-cms/family.axgf --seed-sample \
  --create-admin yourname
```

The password is printed once and stored only as an Argon2id hash. Write it
down. Then install the unit below and `sudo systemctl enable --now axgf-cms`.

Or build from source:

```sh
git clone https://github.com/plkarin/axgf-cms.git
cd axgf-cms
cargo build --release --locked
sudo install -m 0755 target/release/axgf-cms /usr/local/bin/axgf-cms
```

---

## systemd reference

`bootstrap.sh` writes five files, and they are the whole of the installation:

| Unit | What it is |
| --- | --- |
| `axgf-cms.service` | The website. |
| `axgf-cms-backup.service` + `.timer` | One verified archive a day. |
| `axgf-cms-verify.service` + `.timer` | Reads the bundle and the newest archive back, weekly. |

Rather than reproducing them here, where a copy goes stale the day the script
changes, read the ones on the machine:

```sh
systemctl cat axgf-cms
systemctl cat axgf-cms-backup.timer
systemctl cat axgf-cms-verify.timer
```

Three things about the main unit are worth knowing before you read it.

**`ExecStart` carries no arguments.** Every setting is in
`/etc/axgf-cms/env` — the bundle, the bind address, the cache directory, the
backup destination, the log level and the emergency token. Change one there
and `systemctl restart axgf-cms`. There is nothing to edit in the unit, and no
flag that can disagree with the file.

**`Type=notify`.** "Started" means the bundle is loaded, validated and
listening rather than "the process was spawned" — several seconds apart on a
435 MB archive, and the difference between `systemctl start` returning into a
working site and into a 502. It is also where the line under
`systemctl status` comes from:

```
Status: "/var/lib/axgf-cms/family.axgf · 866 people · newest archive
         axgf-backup-20260923T200223Z.zip is under an hour old"
```

**The restart backs off.** `RestartSec=2s` with `RestartSteps=5` and
`RestartMaxDelaySec=2min`, and no start limit that would ever give up.
Measured over five consecutive `kill -9`s: 2.1, 4.6, 10.4, 23.4, 52.9 seconds.
The failure it is for is a full disk, where a service restarting every two
seconds writes a journal entry every two seconds onto the disk that is full.

The sandbox scores **0.9 SAFE** under `systemd-analyze security axgf-cms`
(the backup and verify units score 0.6). The one family that could not be
dropped is `AF_UNIX`: it is journald's socket and systemd's readiness protocol
both, and without it the service logs nothing and never finishes starting.

Day-to-day operation — including what to do when the disk fills and how to
plug `/health` into a monitor — is in [OPERATOR.md](OPERATOR.md).


## Reverse proxy and TLS

**The service binds 127.0.0.1 and must stay there.** It speaks plain HTTP.
Bound wider than localhost with nothing in front of it, every password typed
into the sign-in form crosses the network in clear text, the session cookie is
issued without `Secure`, and a family's private records are readable by anyone
who can reach the port. Do not change `AXGF_CMS_BIND` to `0.0.0.0`. Put a
proxy in front and let it terminate TLS.

Two complete configurations are in the repository, and both have been run in
front of a live instance rather than written from memory:

```sh
# nginx
sudo cp deploy/proxy/nginx-axgf-cms.conf /etc/nginx/sites-available/axgf-cms
sudo ln -s /etc/nginx/sites-available/axgf-cms /etc/nginx/sites-enabled/
sudo certbot --nginx -d tree.example.org
sudo nginx -t && sudo systemctl reload nginx
```

```sh
# Caddy — gets and renews the certificate itself
sudo cp deploy/proxy/Caddyfile /etc/caddy/Caddyfile
sudo systemctl reload caddy
```

Replace `tree.example.org` in whichever you use. What was tested, on nginx
1.24 and Caddy 2.6, against the 435 MB bundle:

| | Result |
| --- | --- |
| `http://` → `https://` | 301 |
| the site over TLS | 200, HTTP/2 |
| session cookie | `Secure` through the proxy; without it, no `Secure` |
| security headers | exactly one of each, plus HSTS from the proxy |
| a 9 MB photograph | accepted, 303 |
| an 11 MB one | 413 **from the application**, with a page that explains itself |
| a 13-second save | 200 |
| the same save with `proxy_read_timeout 5s` | 504 at exactly 5.0 s |

If you write your own instead, four things matter:

* `X-Forwarded-Proto` — without it the session cookie is issued without
  `Secure`, over TLS, and nothing says so.
* `X-Forwarded-For`, **set** from the peer rather than appended to. The login
  throttle counts per client, and a client that supplies its own value picks
  its own bucket.
* a body limit just above the 10 MB upload limit, so an oversized upload is
  refused by the product's own page rather than by a bare 413.
* a read timeout well past 60 seconds. A save rebuilds the whole archive.

And one thing not to do: **do not set `Content-Security-Policy`,
`X-Frame-Options`, `Referrer-Policy`, `X-Content-Type-Options` or
`Permissions-Policy` at the proxy.** The application sets all five on every
response; adding them again sends two of each, and two CSP headers are
enforced as the intersection of the two, which breaks the page in a way
nothing reports. HSTS is the exception — only the thing terminating TLS can
promise that, so the proxy adds it.


## Backups

**Two files are the whole state**, and they want different handling:

| File | What it is | Share it? |
|---|---|---|
| `family.axgf` | The genealogy — every person, relationship, document and photograph. | Freely. It is what the family owns. |
| `family.acl` | The accounts: usernames, Argon2id hashes, roles, branch scope. | With nobody. |

There is no database to dump and no schema to migrate. Back up both: restoring
only the `.axgf` gives you the tree back with every account gone, and nobody
able to sign in but the emergency token.

The payload cache under `<bundle_dir>/.axgf-cms-cache/` (or wherever
`--cache-dir` points) is **derived data** — the `.axgf` holds the authoritative
copy of every attached file — so it does not need backing up. Exclude it; the
next start rebuilds it from the bundle.

```sh
sudo systemctl stop axgf-cms
sudo cp /var/lib/axgf-cms/family.axgf /backups/family-$(date +%F).axgf
sudo cp /var/lib/axgf-cms/family.acl  /backups/family-$(date +%F).acl
sudo systemctl start axgf-cms
```

Stopping is not strictly required — writes are atomic, so a copy taken while
the service runs is always either the previous or the next complete bundle,
never a half-written one — but stopping removes the question entirely.

Without stopping, and keeping a month of daily copies:

```sh
install -d -m 0700 /backups/axgf-cms
cp /var/lib/axgf-cms/family.axgf "/backups/axgf-cms/family-$(date +%F).axgf"
cp /var/lib/axgf-cms/family.acl  "/backups/axgf-cms/family-$(date +%F).acl"
find /backups/axgf-cms -name 'family-*.axgf' -mtime +31 -delete
find /backups/axgf-cms -name 'family-*.acl'  -mtime +31 -delete
```

You can also pull a backup over HTTP from the admin panel
(`GET /admin/export`), which rebuilds the bundle from the state the process is
serving rather than reading the file. It streams: the archive is written to a
temp file one payload at a time and sent from there, so exporting a large
bundle does not cost the process a copy of it.

A `.axgf` bundle is a ZIP of plain JSON. To inspect a backup without this
application:

```sh
unzip -o family-2026-08-07.axgf -d /tmp/inspect
cat /tmp/inspect/manifest.json
```

That is the point of the format: your data stays readable with `unzip` and a
text editor.

### Restoring

Stop the service, copy the backup over `family.axgf`, make sure the service
user owns it, and start again:

```sh
sudo systemctl stop axgf-cms
sudo cp /backups/axgf-cms/family-2026-08-07.axgf /var/lib/axgf-cms/family.axgf
sudo cp /backups/axgf-cms/family-2026-08-07.acl  /var/lib/axgf-cms/family.acl
sudo chown axgf-cms:axgf-cms /var/lib/axgf-cms/family.axgf
sudo systemctl start axgf-cms
sudo journalctl -u axgf-cms -n 20
```

---

## Upgrading

Upgrades replace one file. The bundle format is versioned independently and
the binary refuses to load a spec version it does not understand, so a bad
upgrade fails loudly at startup rather than silently rewriting your data.

```sh
# Back up first — always.
sudo cp /var/lib/axgf-cms/family.axgf /backups/pre-upgrade.axgf

# Re-running bootstrap installs the new binary and leaves the bundle,
# the token and the unit's settings alone.
curl -fsSL https://raw.githubusercontent.com/plkarin/axgf-cms/main/deploy/bootstrap.sh \
  | sudo bash

sudo systemctl restart axgf-cms
curl -s localhost:8080/health
```

Or by hand: download the new archive, verify its checksum, `install` it over
`/usr/local/bin/axgf-cms`, and restart.

To roll back, install the previous binary with `--version` and, if the
bundle was modified in the meantime, restore the backup.

---

## Troubleshooting

**I am locked out of every account.** The emergency token is in
`/etc/axgf-cms/env`:

```sh
sudo sed -n 's/^AXGF_CMS_ADMIN_TOKEN=//p' /etc/axgf-cms/env
```

Enter it under "Emergency access" on the sign-in page. It opens an
administrator session so you can reset a password from **Admin → Accounts**.
Its use is logged as a warning. To rotate it, write a new value into that file
and restart the service.

**I forgot one user's password.** Sign in as another administrator and set a
new one from **Admin → Accounts**. Saving it signs that account out everywhere.

**The service refuses to start, complaining about the `.acl` file's mode.**
That is deliberate: a credential store any local user can read is not a
credential store. The message names the mode it found and the fix:

```sh
sudo chown axgf-cms:axgf-cms /var/lib/axgf-cms/family.acl
sudo chmod 600 /var/lib/axgf-cms/family.acl
```

**I want to start the accounts over.** Stop the service, delete
`/var/lib/axgf-cms/family.acl`, and run `--create-admin` again:

```sh
sudo systemctl stop axgf-cms
sudo rm /var/lib/axgf-cms/family.acl
sudo runuser -u axgf-cms -- /usr/local/bin/axgf-cms \
     --bundle /var/lib/axgf-cms/family.axgf --create-admin yourname
sudo systemctl start axgf-cms
```

The genealogy is untouched by any of this: the accounts live in a separate
file precisely so that losing one does not mean losing the other.

**The service will not start.** `journalctl -u axgf-cms -n 50`. The usual
causes are the bundle not being readable by the `axgf-cms` user, or the port
already being in use.

```sh
sudo chown axgf-cms:axgf-cms /var/lib/axgf-cms/family.axgf
sudo ss -lptn 'sport = :8080'
```

**`UNSUPPORTED_SPEC_VERSION` at startup.** The bundle was written by a newer
AXGF than this binary understands. Upgrade axgf-cms rather than editing the
manifest.

**A `family.axgf.tmp` is left behind.** That is a write that failed before its
rename. Your `family.axgf` is the last good state; the temp file can be
deleted. Check the logs for what failed — usually a full disk.

**Validation reports warnings.** Warnings never block, by design: a bundle
that records genuine uncertainty is not a broken bundle. Run
`POST /admin/validate` from the dashboard to see them, and treat them as a
worklist rather than as errors.
