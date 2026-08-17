# Deploying axgf-cms

The whole application is one binary and one file. If you remember nothing else
from this document, remember that **`family.axgf` is the entire database** —
copy it and you have a complete backup; lose it and there is nothing else.

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
| `--version <TAG>` | Install a specific release tag rather than the latest. |
| `--bind <ADDR>` | Address for the unit to bind (default `127.0.0.1:8080`). |
| `--dry-run` | Print every action and change nothing. |

The script is safe to re-run. It will not overwrite an existing bundle and will
not regenerate an existing admin token; it prints which of those it is
preserving. If you want to see exactly what it will do first:

```sh
sudo bash bootstrap.sh --dry-run --with-sample
```

What it creates:

| Path | Purpose |
|---|---|
| `/usr/local/bin/axgf-cms` | The binary. Nothing else is installed. |
| `/var/lib/axgf-cms/family.axgf` | **The database.** |
| `/etc/axgf-cms/env` | Admin token, mode 0640, root-owned. |
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
```

Then install the unit below and `sudo systemctl enable --now axgf-cms`.

Or build from source:

```sh
git clone https://github.com/plkarin/axgf-cms.git
cd axgf-cms
cargo build --release --locked
sudo install -m 0755 target/release/axgf-cms /usr/local/bin/axgf-cms
```

---

## systemd reference

`/etc/systemd/system/axgf-cms.service`:

```ini
[Unit]
Description=axgf-cms — AXGF genealogy showcase
Documentation=https://github.com/plkarin/axgf-cms
After=network.target

[Service]
Type=exec
User=axgf-cms
Group=axgf-cms
EnvironmentFile=/etc/axgf-cms/env
ExecStart=/usr/local/bin/axgf-cms --bundle /var/lib/axgf-cms/family.axgf --bind 127.0.0.1:8080
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
ReadWritePaths=/var/lib/axgf-cms
RestrictAddressFamilies=AF_INET AF_INET6
RestrictNamespaces=yes
LockPersonality=yes
MemoryDenyWriteExecute=yes

[Install]
WantedBy=multi-user.target
```

`ProtectSystem=strict` with a single `ReadWritePaths` is the important part:
the service can write to its bundle directory and nowhere else on the
filesystem. The admin token arrives through `EnvironmentFile` so it never
appears in the process arguments, where any local user could read it from
`ps`.

Useful commands:

```sh
sudo systemctl status axgf-cms
sudo journalctl -u axgf-cms -f          # logs, including the generated token on first boot
sudo systemctl restart axgf-cms
```

---

## Reverse proxy and TLS

**The service binds to localhost on purpose.** V1 has a single shared admin
token and no user accounts, so anything that reaches the port and holds the
token has full edit rights. Do not move `--bind` to `0.0.0.0`. Put a proxy in
front and let it terminate TLS.

### Caddy

Caddy obtains and renews a certificate automatically.

```caddy
genealogy.example.org {
    reverse_proxy 127.0.0.1:8080

    # V1 has no accounts. If the site is reachable from the internet, add a
    # second factor in front of the admin surface.
    @admin path /admin*
    basic_auth @admin {
        # caddy hash-password --plaintext 'your-password'
        curator $2a$14$replace_this_with_a_real_bcrypt_hash
    }
}
```

### nginx

```nginx
server {
    listen 443 ssl http2;
    server_name genealogy.example.org;

    ssl_certificate     /etc/letsencrypt/live/genealogy.example.org/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/genealogy.example.org/privkey.pem;

    # A 767-person GEDCOM is about 320 KB; the app's own ceiling is 10 MB.
    client_max_body_size 12m;

    location / {
        proxy_pass         http://127.0.0.1:8080;
        proxy_set_header   Host              $host;
        proxy_set_header   X-Real-IP         $remote_addr;
        proxy_set_header   X-Forwarded-For   $proxy_add_x_forwarded_for;
        proxy_set_header   X-Forwarded-Proto $scheme;
    }

    # V1 has no accounts. Gate the admin surface separately.
    location /admin {
        auth_basic           "axgf-cms admin";
        auth_basic_user_file /etc/nginx/axgf-cms.htpasswd;
        proxy_pass           http://127.0.0.1:8080;
        proxy_set_header     Host $host;
        proxy_set_header     X-Forwarded-Proto $scheme;
    }
}

server {
    listen 80;
    server_name genealogy.example.org;
    return 301 https://$host$request_uri;
}
```

The application does not set `Secure` on its session cookie, because its
documented default is plain HTTP on localhost, where a `Secure` cookie would
never be stored. Behind TLS, have the proxy add it:

```nginx
proxy_cookie_flags axgf_admin secure samesite=lax;
```

---

## Backups

The bundle is the whole state. There is no database to dump and no schema to
migrate. The payload cache under `<bundle_dir>/.axgf-cms-cache/` (or wherever
`--cache-dir` points) is **derived data** — the `.axgf` holds the authoritative
copy of every file — so it does not need backing up. Exclude it from backups;
the next start rebuilds it from the bundle.

```sh
sudo systemctl stop axgf-cms
sudo cp /var/lib/axgf-cms/family.axgf /backups/family-$(date +%F).axgf
sudo systemctl start axgf-cms
```

Stopping is not strictly required — writes are atomic, so a copy taken while
the service runs is always either the previous or the next complete bundle,
never a half-written one — but stopping removes the question entirely.

Without stopping, and keeping a month of daily copies:

```sh
install -d -m 0700 /backups/axgf-cms
cp /var/lib/axgf-cms/family.axgf "/backups/axgf-cms/family-$(date +%F).axgf"
find /backups/axgf-cms -name 'family-*.axgf' -mtime +31 -delete
```

You can also pull a backup over HTTP from the admin panel
(`GET /admin/export`), which exports the in-memory bundle rather than reading
the file.

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

**I lost the admin token.** It is in `/etc/axgf-cms/env`:

```sh
sudo sed -n 's/^AXGF_CMS_ADMIN_TOKEN=//p' /etc/axgf-cms/env
```

To rotate it, write a new value into that file and restart the service. Any
existing session cookie stops working immediately.

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
