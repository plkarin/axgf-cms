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
- [The AFG: where its computing runs](#the-afg-where-its-computing-runs)
- [When to worry](#when-to-worry)
- [Uninstalling](#uninstalling)

---

## Install

```sh
curl -fsSLO https://github.com/plkarin/axgf-cms/releases/latest/download/bootstrap.sh
curl -fsSLO https://github.com/plkarin/axgf-cms/releases/latest/download/bootstrap.sh.sha256
sha256sum -c bootstrap.sh.sha256 && sudo bash bootstrap.sh --with-sample
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

### Sessions

A restart signs nobody out: the accounts' sessions are kept beside the
bundle, in `/var/lib/axgf-cms/family.sessions`, mode 600, with the secret
that signs their cookies (docs/adr/0028).

- **How long:** a session lasts 30 days without use, and each use moves that
  on.
- **Not backed up, and disposable:** if the file is lost or unreadable,
  everyone signs in again, nothing else.
- **To sign everyone out:**

  ```sh
  sudo systemctl stop axgf-cms
  sudo rm /var/lib/axgf-cms/family.sessions
  sudo systemctl start axgf-cms
  ```

- **The emergency token's session** is never written there, and still ends
  with the process.

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

**The AFG's memory has an archive of its own** (docs/adr/0027). The same run
then writes `axgf-memory-<stamp>.zip` beside it:

- **What it holds:** the brains (`family.brains/`) and the personal
  assistants (`family.assistant/`), every file with its SHA-256 in the
  archive's manifest.
- **How it is checked:** it is read back the same way before it is kept.
- **Retention:** the same, as a set of its own.
- **The bundle's archive stays exactly what it was.**

An installation where nobody has talked to the AFG yet has no such archive.
A run whose memory archive fails says so and fails, even though the bundle's
archive was written.

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
archive back in full, the newest archive of the AFG's memory too. It fails the
unit if any of that is wrong:

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

The same command puts the AFG's memory back from an `axgf-memory-…` archive,
and leaves the bundle as it is. The brains and the assistants that are there
now go aside, into `axgf-memory-replaced-<stamp>/`:

```sh
sudo systemctl stop axgf-cms
sudo -u axgf-cms axgf-cms restore \
  --bundle /var/lib/axgf-cms/family.axgf \
  /var/lib/axgf-cms/backups/axgf-memory-20261104T033012Z.zip
sudo systemctl start axgf-cms
```

## Upgrading

```sh
curl -fsSLO https://github.com/plkarin/axgf-cms/releases/latest/download/bootstrap.sh
curl -fsSLO https://github.com/plkarin/axgf-cms/releases/latest/download/bootstrap.sh.sha256
sha256sum -c bootstrap.sh.sha256 && sudo bash bootstrap.sh --upgrade
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

### From the Infrastructure page (0.5.0)

`bootstrap.sh` from 0.5.0 on also installs a small root helper beside the
service (docs/adr/0011 §5): the page's **Change version** button writes a
request, and the helper runs exactly the upgrade above for that release tag,
then reports back on the page. It also fetches the list of published releases
once a day and when the page asks. A private repository needs a GitHub token,
which the helper reads from a file only root can read:

```sh
sudo install -m 600 -o root -g root /dev/null /etc/axgf-cms/github-token
sudoedit /etc/axgf-cms/github-token    # paste the token: never on a command line
```

`bootstrap.sh` itself uses that file, or `AXGF_CMS_GITHUB_TOKEN`, to download a
private repository's releases, and never prints the token. Without the helper
the page shows the command to run instead. `systemctl status
axgf-cms-upgrade.path axgf-cms-releases.timer` says whether it is watching.

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
- a body limit just above the 200 MB a request with a file may be. A file is
  at most 50 MB; a larger one is read to its end and refused by a page that
  explains itself rather than by a bare 413.
- the import of a whole family as the exception: `/admin/setup` and the
  addresses under it, up to just above the archive limit in Uploads (4 GB
  unless you changed it; the files say 4200 MiB). Raise the proxy's with the
  application's. In nginx the prefix has no trailing slash — with one, nginx
  answers `/admin/setup` with a 301 to `/admin/setup/`, which is "Not found";
  in Caddy it has a `handle` of its own, since two `request_body` limits on
  one request both apply and the smaller wins.
- a read timeout well past 60 seconds. A save rebuilds the whole archive; at
  five seconds the same save reaches the reader as a 504 while the write
  carries on regardless. An import reads a whole archive after backing up the
  family it replaces: the files give it fifteen minutes.
- **no** `Content-Security-Policy`, `X-Frame-Options`, `Referrer-Policy`,
  `X-Content-Type-Options` or `Permissions-Policy`. The application sets all
  five on every response. Adding them at the proxy sends two of each, and two
  CSP headers are enforced as the intersection of the two — which breaks the
  page in a way nothing reports. HSTS is the exception: only the thing
  terminating TLS can promise that, so the proxy adds it.

## The AFG: where its computing runs

The administration's AFG room, the Artificial family genealogist
(docs/adr/0023), sends the archive to a model and shows its answer. Where
that model runs is your choice. The service's sandbox, as the installer
writes it, allows only the first of these:

| Where | What it needs from this machine |
| --- | --- |
| **Ollama or vLLM on this machine** (`http://127.0.0.1:…`) | Nothing: the service may reach localhost. |
| **A service on the internet**: OpenAI, Mistral, Claude, Gemini, or an Ollama or vLLM elsewhere | Outbound network. The unit allows localhost only. |
| **A coding agent on this machine**: Claude Code, Codex, OpenCode | The agent installed where the service can see it, and signed in as the service's user. Outbound network. For Claude Code and OpenCode, which are built with Bun, memory that is written and then run. |

**What leaves the machine.** A test sends every record of the archive, the
living included, to the model you chose. The health, biometric, genomic,
legal and behavioural classes stay out unless the administrator ticks the
box for them. A service on the internet reads the records on its owner's
servers, and so does an agent that calls one. Decide this with the family
before you open the way.

### Opening the sandbox for it

Widen the unit deliberately, with a drop-in, rather than by editing it:

```sh
sudo systemctl edit axgf-cms
```

```ini
[Service]
# The AFG's service, or the one its agent calls, is on the internet.
IPAddressAllow=any
# Claude Code and OpenCode only: their JavaScript engine writes code and then
# runs it.
MemoryDenyWriteExecute=no
```

```sh
sudo systemctl restart axgf-cms
```

Everything else in the sandbox stays as it is. If a test still fails, its
page shows what the service or the agent said: the end of what the program
printed on its error output.

An agent whose network is closed does not always fail at once. Claude Code
retries a refused connection until the time allowed runs out, and the test
then says only "no answer after 600 s". Check the sandbox first.

### An agent on this machine

The service runs as the system user `axgf-cms`. Its home is the data
directory, `/var/lib/axgf-cms`, and it cannot see `/home`. So:

1. **Install the agent where the service can see it.** That is
   `/usr/local/bin`, or the service user's own `~/.local/bin`
   (`/var/lib/axgf-cms/.local/bin`). The room says whether it found the
   program, and where. Otherwise give its full path in the **Program** field,
   which accepts only the agent's own program: `claude`, `codex` or
   `opencode`.
2. **Sign it in as the service user**, once. Its credentials are then kept in
   that home, under the data directory:

   ```sh
   sudo -u axgf-cms -H claude auth login          # check: claude auth status
   sudo -u axgf-cms -H codex login --device-auth  # check: codex login status
   sudo -u axgf-cms -H opencode auth login        # check: opencode auth list
   ```

   An agent that reads its key from the environment can be given it in the
   same drop-in, with `Environment=`: `ANTHROPIC_API_KEY`,
   `CLAUDE_CODE_OAUTH_TOKEN` (from `claude setup-token`) or
   `OPENAI_API_KEY`. The agent inherits the service's environment, except
   this site's own settings. Every `AXGF_*` variable is removed, and the
   emergency token is one of them.

What each agent may do while it reads the archive:

- **Claude Code** has no tool and no MCP server, and keeps no session. A
  version that knows `--safe-mode` also reads none of the account's
  CLAUDE.md, skills, hooks or plugins. An older one, such as 2.1.131, still
  runs the account's hooks and reads its CLAUDE.md.
- **Codex** has no shell and no web search. It reads no `config.toml`, and
  so none of its MCP servers. It runs in a read-only sandbox and keeps
  nothing.
- **OpenCode** has every tool denied except reading its own working
  directory, through which its attachment is read. It loads no plugins and
  no project files.

**Models and efforts.** The room suggests the models of the moment for each
way of computing, and each model's efforts (docs/adr/0025). Any other name
is kept and sent as written, and the room says it is not a suggestion. A
name a model does not have, such as `opus-5`, then fails at the test.
Claude Code's aliases (`opus`, `sonnet`, `haiku`, `fable`) name the latest
model the installed version knows: `opus` is Opus 4.7 for 2.1.131 and
Opus 5.5 for 2.1.300. A full name, such as `claude-opus-5-5`, is the same
for every version.

**Brains.** The AFG keeps a brain for each account that talks to it
(docs/adr/0024). That is a folder `family.brains/<username>/` in the data
directory, mode 700, holding:

- `memory.md`: what the AFG chose to remember of the person, one fact a
  line;
- `conversations.jsonl`: every exchange.

It is personal data about a living person. Four things to know:

- **Backups.** The nightly run backs them up apart from the family's
  archive, in `axgf-memory-<stamp>.zip` (see Backups above). A fact
  forgotten stays in the archives that held it until they age out.
- **Forgetting.** Each person forgets from *My account*: a fact, an
  exchange, or everything. You can also delete a folder by hand.
- **A name given again.** The new account starts from nothing. The old
  folder is moved aside under `<name>@<its account>`, and you may delete it.
- **Who may talk to the AFG** is set for each account on the Accounts page.

**Personal assistants.** Each account the AFG is open to has a personal
assistant (docs/adr/0026): projects and conversations with the AFG. They
are kept in `family.assistant/<username>/` in the data directory, mode 700,
with the files the member sent (6 a message at most, 20 MB each). Like the
brains, they are personal, and backed up with the brains, apart from the
family's archive, in `axgf-memory-<stamp>.zip` (see Backups above). Every
message sends the conversation's recent history, its files and the
project's memory to the AFG's computing. The family's archive goes too only
when the project's member chose it. The site writes at most four answers at
once.

**Size.** A family of a thousand people is over a megabyte of JSON, some
400,000 tokens. That is more than many models read at once. The room says
the size before you send. Ollama is asked for a context large enough for the
archive, up to 131,072 tokens. Beyond what a model reads, the test shows its
refusal. A test is given 600 seconds unless the room says otherwise.

### The AFG on a person's record

A member the AFG is open to, who may write, finds its buttons on a person's
record (docs/adr/0029):

- **Under the reworked portraits**, "Rework the image with AI": the face,
  restored, joins those portraits.
- **On the Media tab**, under each photograph:
  - "AI retouch": a restored copy, `<name>_IA`, beside it;
  - "Read the document": its text, transcribed in its own language, as a
    note. A PDF is read from its row.
- **Above Identity**, "Write it with the AI": the person's summary, in the
  language chosen, from the record and the notes about them.

Reading and summaries go to the AFG's computing as it is set. Pictures do
not: Claude and the coding agents write text only. A picture needs a
**picture service**, chosen in the room's **Images** section
(docs/adr/0029, 0030):

- **OpenAI** (GPT Image) or **Google Gemini** (Nano Banana), each with its
  own API key. Each picture is billed to that key's account by the service.
- **Canva**, through its AI and your Canva account (Pro or above):
  1. Choose Canva and save.
  2. Press **Connect Canva**, open Canva from the link and accept.
  3. Canva sends you back to an address on `localhost`. If the page does not
     load, which is expected unless you reach the site at `localhost`, paste
     its address into the room.
  4. **Finish connecting** takes up to a minute: it also makes, in Canva,
     the page every picture is laid on.

  Each picture spends one of the account's AI credits, and each connection
  one more, for that page. When the credits come too fast, Canva holds them
  back for some minutes (`quota_cooldown`), and the record says so in the
  reader's language: try again a few minutes later. Canva's own words are in
  the journal, in the `F_ERROR` field (`journalctl -u axgf-cms -o
  verbose`). What Canva makes is filed in the account's folder
  `ax-genealogy`. The photographs sent stay in its uploads; clear them from
  Canva when you like. One connection serves the whole family, while Canva's
  usage policy expects one per person: connecting yours for the AFG, you
  take that on.
- **A machine of the family's** with a graphics card, which keeps the
  photograph on your network:
  - **Ollama** (experimental): its address, by default
    `http://127.0.0.1:11434/v1`. Ollama does not yet edit a picture it is
    given; the choice is ready for when it does.
  - **A server of your own**, compatible with OpenAI's `/images/edits` (for
    example a server running Qwen-Image-Edit or FLUX.1 Kontext): its address
    ending in `/v1`, a model, and a key only if it asks for one.
  - **ComfyUI**: its address, by default `http://127.0.0.1:8188`, and your
    restoration workflow, exported with "Save (API Format)". The photograph
    goes into every `LoadImage` node, and `{{restore}}` in a text becomes the
    restoration instruction.
- Leave an address empty for the service's usual one, and the model empty
  for the first suggested.
- A key, or a Canva account, is never shown again. Changing service forgets
  it unless you give a new key at the same time. "None" forgets everything.

Until a service is set, the picture buttons are greyed out.

**What leaves the machine.** Each request sends its photograph to the
picture service, or its document, or the person's record and notes, to the
AFG's computing. The health, biometric, genomic, legal and behavioural
classes reach a summary only if the room's box allows them. A picture service
on the internet needs the outbound network the AFG's does (see "Opening the
sandbox for it" above). The log says who sent what to which service, never
what it held.

### The AFG in conversation on a person's page

Every account the AFG is open to, whether it may write or not, has an
**AFG** tab on each record it may read (docs/adr/0031). There it asks
questions about the person, and finds its own conversations about them
again, to carry them on, rename them or delete them. Nobody else reads them,
administrators included.

With every question the AFG reads a dossier, made from what that account may
read and nothing more:

- the person's record and the notes about them;
- the photographs of the record, twelve at most, as pictures sized for a
  model (docs/adr/0033). Films, sound and PDF documents are not given, nor a
  picture an AI made, nor one that sensitive data refers to unless the room
  lets those classes go;
- their close relatives, with the notes about them;
- one line for each person of the family.

The tab says how large it is before the first question: at most about
50,000 tokens of words, and about 1,600 for each photograph. It goes to the
AFG's computing as the room sets it. A local model with a small window may
not take a large family's whole index. A model that takes no picture is
asked again without the photographs, and says so when a question needs them.

The conversations are kept in the account's assistant space, so the nightly
backup of the AFG's memory takes them (docs/adr/0027). The log says who asked
about whom, and how much the dossier held, never what was asked.

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
