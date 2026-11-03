# axgf-cms reference

The detail behind the [README](../README.md): every flag, every route, how
attachments and accounts work, and how the application is put together. An
operator installing with the one-line script needs none of it; start with the
README and [OPERATOR.md](OPERATOR.md).

---

## Running it

```
axgf-cms --bundle /var/lib/axgf-cms/family.axgf \
         --bind 127.0.0.1:8080 \
         --admin-token <token>
```

| Flag | Default | Meaning |
|---|---|---|
| `--bundle <PATH>` | *required* | The `.axgf` file to serve. Created empty if absent. |
| `--bind <ADDR>` | `127.0.0.1:8080` | Address to listen on. |
| `--admin-token <TOKEN>` | `$AXGF_CMS_ADMIN_TOKEN` | Emergency recovery token, granting an administrator session. Not an account. If neither is set, a random one is generated and printed once to stderr. |
| `--create-admin <USERNAME>` | — | Create an administrator, print a generated password once to stderr, and exit without serving. How an installation gets its first account. Refuses an existing username rather than resetting it, so it is safe to re-run. |
| `--seed-sample` | off | When creating a *new* bundle, seed it with the built-in demonstration family. Ignored if the bundle already exists. |
| `--size-warn-mb <MB>` | `200` | Bundle size past which the admin panel warns that the archive is getting heavy. Not a limit. |

### Routes

Public, read-only:

| Route | What it shows |
|---|---|
| `GET /` | Why AXGF, what is in this bundle, entry points |
| `GET /tree` | A classic family chart around one person, one column per generation: the ancestors to the left, the oldest first, the descendants to the right, read down the page (docs/adr/0019). `?root=<id>` centres it, `?depth=<n>` sets how many generations each way (default 3), `?sel=<id>` shows a record in the side panel, `?all=1` draws the whole bundle by generation |
| `GET /person/:id` | The whole record for one person, in sections: identity and every recorded name, a chronological life timeline, family, non-family relationships, occupations, places, sources and documents, notes, and the entity's raw JSON. A section with no content is omitted. `?tab=kinship` lists every relative and, with `&with=<id>`, says how that person is related and shows the way step by step (docs/adr/0020) |
| `GET /convert` | GEDCOM → AXGF conversion |
| `POST /convert/gedcom` | Convert an upload, report what it carried against what AXGF holds, and offer the result |
| `GET /document/:id/raw` | The stored bytes of an attached file, with `X-Content-Type-Options: nosniff`. Raster images are served inline; everything else downloads |
| `GET /document/:id/thumb` | A downscaled PNG of an image, `404` for anything else |
| `GET /health` | `200` with entity counts. `persons` counts what *this* requester may read |

Admin (requires a signed-in account; `Accounts` and the operations marked
*admin* require the `admin` role, everything else `contributor`):

| Route | What it does |
|---|---|
| `GET /admin` | Counts, bundle completeness, validation report, operations |
| `GET/POST /admin/login`, `POST /admin/logout` | Session |
| `GET /account`, `POST /account/person` | The signed-in account's own page: who it is, which person in the tree it says it is, and its AFG brain (docs/adr/0024 §6) — *any account* |
| `POST /account/brain/forget` | Forget one fact (`fact=<mark>`) or one exchange (`exchange=<mark>`) of the AFG's brain of the signed-in account, or all of it (`all=1&confirm=on`). A mark is the first 16 hex digits of the line's SHA-256. Always the session's own brain; answers `303` to `/account?done=forgotten|forgotten-all|already#account-brain` — *any account* |
| `GET /account/assistant`, `POST /account/assistant` | My personal assistant (docs/adr/0026): the account's projects, and a new one (`name`, `avatar` = a pack avatar's name, or empty for the one its id picks). Every route under it is the session's own space: another account's project, conversation or file answers 404 — *an account the AFG is open to* |
| `GET/POST /account/assistant/:project` | A project: its conversations, its code of conduct, its memory. A POST saves its settings (`part=settings`: `name`, `avatar`, `archive=on` to send the family's archive with its messages) or its code of conduct (`part=conduct`: `conduct`, 8,000 characters at most) — *its account* |
| `POST /account/assistant/:project/delete` | The project and all of it, with `confirm=on` — *its account* |
| `POST /account/assistant/:project/memory/forget` | Forget one fact of the project's second brain (`fact=<mark>`, the first 16 hex digits of the line's SHA-256), or all of it (`all=1&confirm=on`) — *its account* |
| `GET/POST /account/assistant/:project/avatar` | Its picture of its own: a multipart `picture` of 8 MB at most, kept as a 256 px JPEG — *its account* |
| `POST /account/assistant/:project/c` | A new conversation, on its first message: a multipart `text` (32,000 characters) and up to six `files` (20 MB each, 40 MB in all). Answers a redirect to the conversation, or `{"redirect"}` with `Accept: application/json` — *its account* |
| `GET/POST /account/assistant/:project/c/:conversation` | The conversation; the next message, as above. With `Accept: application/json`: `{"message": {"id", "html"}, "count", "answering"}`; `409` while the AFG still writes in it, `503` while the site writes four answers already — *its account* |
| `GET /account/assistant/:project/c/:conversation/state?after=N` | What the page does not show yet: `{"count", "messages": [{"id", "role", "html"}], "answering", "live"}`, `live` being the answer being written, drawn — *its account* |
| `POST /account/assistant/:project/c/:conversation/retry`, `…/edit`, `…/delete`, `GET/POST …/avatar` | Ask again after a failed answer; its `title` and `avatar`; its deletion with `confirm=on`; its picture of its own — *its account* |
| `GET /account/assistant/:project/f/:file` | A file a message carried: a picture inline, anything else as a download, `nosniff` and a sandboxing CSP — *its account* |
| `GET /static/avatars/:name.webp` | An avatar of the pack (docs/adr/0026 §8) — *anyone* |
| `GET /admin/:kind` | Paginated, filterable listing |
| `GET /admin/:kind/new`, `POST /admin/:kind` | Create |
| `GET /admin/:kind/:id/edit`, `POST /admin/:kind/:id` | Update |
| `POST /admin/:kind/:id/delete` | Delete, with a referential-integrity policy — *admin* |
| `POST /admin/person/:id/document` | Attach a file to a person — multipart upload, stored inside the bundle |
| `GET/POST /admin/family/:id/avatar`, `POST /admin/family/:id/document` | A family's emblem (docs/adr/0022): choose among the family's images, `none` or `auto`, written as `extensions["axgf-cms:avatar/v1"]` on the Family; or upload a raster image, stored as a Document whose `linked_to` names the family in the role `emblem`, and used at once. `?back=<person id>` returns to that record |
| `GET/POST /admin/bundle/emblem`, `POST /admin/bundle/emblem/upload` | The archive's emblem, at the head of every page (docs/adr/0022): `manifest.family.extensions["axgf-cms:emblem/v1"]`, checked against the manifest's token like the family form — *admin* |
| `GET/POST /admin/person/:id/links`, `POST /admin/person/:id/links/:lid` | The person's links, the relations that are not family: list, create, change. The form's `rite` field (`<rite>:<role>`, the role `sponsor`, `sponsored` or `mutual`; the 27 rites are in `src/rites.rs`) records a tie of rite: the nearest AXGF 1.1 `relation` and its `category`, the rite's words in `label` and `label_reverse` when they are left empty, `bidirectional` for a mutual tie, and `extensions["axgf-cms:kinship/v1"]` = `{"rite": <key>, "sponsor": "from" or "to"}`, without `sponsor` for a mutual tie (docs/adr/0020) |
| `POST /admin/validate`, `POST /admin/dedup` | Run the library's checks — *admin* |
| `GET /admin/export` | Download the live bundle — *admin* |
| `GET /admin/users`, `POST /admin/users`, `POST /admin/users/:id` | Accounts: create, change a role, a branch, a status or a password, and whether the account may talk to the AFG (`afg=on`, a checkbox: absent is no; docs/adr/0024) — *admin* |
| `GET/POST /admin/afg` | The Artificial family genealogist (docs/adr/0023): where its computing comes from (`kind` = `off`, `ollama`, `openai`, `mistral`, `vllm`, `anthropic`, `gemini`, `claude-code`, `codex` or `opencode`; `url`, `api_key`, `clear_key`, `model`, `effort`, `program`, `sensitive`, `timeout`), kept in `family.acl` under the site settings. An empty `api_key` keeps the stored one, which is never rendered. `model` and `effort` take any plain word (letters, digits and `. _ : / @ [ ] + -`, no leading `-`); the page suggests the models of the moment and their efforts (docs/adr/0025). The effort goes to every kind: Ollama's `think`, `reasoning_effort`, Anthropic's `output_config.effort`, Gemini's `thinkingConfig.thinkingLevel`, or the agent's own option — *admin* |
| `POST /admin/afg/test` | Send the archive's JSON files, the asker's brain and the test question to the AFG (docs/adr/0024), then open the room on `?run=<id>`, the result in a window: waiting, success with the answer's Markdown drawn, or error. One test at a time; the last ten are kept in memory — *admin* |
| `GET /admin/afg/run/:id` | The window's script asks it while the answer is awaited: `{"id", "state": "running"/"done"/"failed", "html"}`, the window drawn; `404 {"state": "gone"}` for a test no longer kept — *admin* |

`:kind` is one of `person`, `family`, `event`, `link`, `occupation`, `source`,
`place`, `document`.

A union of type `polygamous` (docs/adr/0021) is read as one marriage per
spouse. The record editor writes three fields beyond AXGF 1.1, proposed for
1.2 (axgf-spec proposals/0003), and keeps them through every editor:

- `union.primary_person_id`: the person married to each of the others, from
  the field `fam.<id>.primary`;
- `union.unions[]`: each spouse's own marriage (`spouse_id`, `start`, `end`,
  `status`), kept and read;
- `children[].parent_ids`: the child's parents within the union, from
  `fam.<id>.child.<n>.parent`.

---

## Attached documents and photographs

An AXGF bundle carries its own binary attachments: files under
`documents/files/**` inside the ZIP, with a Document entity describing each.
Uploading a photograph through `/admin/person/:id/document` writes both in one
atomic rewrite of the bundle, so the picture travels with the data — copy the
`.axgf` to another machine and the album comes with it. Images appear as a
gallery on the identity page, everything else as a list with a download link.

**The file type is read from the file, never from its name.** A client controls
both the filename and the `Content-Type` header, so neither is evidence. The
leading bytes are matched against an allowlist — PNG, JPEG, GIF, WebP, BMP,
TIFF, PDF, plain text, and common audio and video containers — and anything
unrecognised is refused. An executable renamed to `portrait.jpg` does not get
in, because nothing in the allowlist matches an ELF header.

**SVG is refused.** Not sanitised, not stripped: refused. An SVG is a document
that can carry `<script>`, and serving one from the same origin as the admin
session would hand an uploader script execution against that session.
Sanitising it properly means parsing XML and maintaining an element and
attribute allowlist — a security surface with no business in a genealogy
viewer. It is also plain XML with no magic number, so it cannot be identified
by the rule every other upload follows. Bitmap formats cover what a family
archive holds. A bundle authored elsewhere may still contain an SVG; it is
served as a download, never rendered inline.

Every stored file is served with `X-Content-Type-Options: nosniff`. Only the
raster formats a browser draws as pixels are served inline; everything else
gets `Content-Disposition: attachment`.

**Textual data is memory-resident; binary payloads are never in memory at
all.** The bundle is read with `axgf-rs`'s streaming boundary, which hands over
one payload at a time as a live reader: each attachment goes from the archive
straight into a disk cache through a fixed 64 KiB buffer, and the flat JSON
that comes back carries document *metadata* and a per-file `external_payloads`
entry rather than the bytes. Saving reverses it — the new archive is streamed
into a temp file and each payload copied in from the cache — so neither
direction ever holds a photograph, let alone all of them. What stays in RAM is
the textual data: persons, families, document metadata, the manifest, bounded
by the size of the tree rather than its media; on the operator's 420 MiB
archive that is under a megabyte, and the peak while loading or saving is
bounded by the copy buffer rather than by the largest file. Single uploads are
still capped at 10 MB, and the admin panel warns once the *textual* bundle
passes `--size-warn-mb` (default 200 MB).

The cache lives at `<bundle_dir>/.axgf-cms-cache/<bundle-sha>/` by default, or
wherever `--cache-dir` points; it is keyed by a hash of the bundle so a
different bundle never reads another's payloads. A restart on an unchanged
bundle recomputes each cached file's CRC-32 and compares it against the one the
archive's central directory records — a direct proof that the cache still holds
this bundle's bytes — and skips extraction where it matches, so nothing is
decompressed at all. The sha256 the document metadata records is checked
separately, and a disagreement between a file and its record is reported rather
than served silently. The cache is **derived data** — the `.axgf` is the
authoritative copy — so it does not need backing up, and can be deleted at any
time; the next start rebuilds it, and a save that finds an entry missing
rebuilds that entry rather than writing a bundle with the file absent. A
document whose bytes live elsewhere is still recorded with
`status: "referenced"`, which the identity page renders without offering a
download.

---

## Accounts, roles and visibility

**Read this before exposing the site.**

### Two files, and only one of them is shareable

    family.axgf   the genealogy — copy it, mail it, publish it, archive it
    family.acl    the accounts  — mode 600, shared with nobody

Accounts are **not** inside the bundle. A `.axgf` is meant to travel; password
hashes in it would make every copy of the family tree a copy of the credential
store. Passwords are Argon2id at the OWASP 2024 parameters (m=19456, t=2, p=1),
never a fast hash. The server refuses to start if the `.acl` is readable by
anyone but its owner, and says which `chmod` fixes it. Encryption at rest is
GPG's job and out of scope here.

Beside them, once anyone has talked to the AFG, a third (docs/adr/0024):

    family.brains/   the AFG's memory of each account — mode 700, shared with nobody
      <username>/brain.json            whose brain: the account's id and name
      <username>/memory.md             what the AFG chose to keep, one fact a line
      <username>/conversations.jsonl   every exchange, one JSON object a line

A brain belongs to its account's id, not to its name. The nightly backup
keeps the brains and the assistants apart from the family's archive, in an
archive of their own, `axgf-memory-<stamp>.zip` (docs/adr/0027).

### The first account

There is no web setup page, deliberately: the window between deploying and the
first login is exactly when an installation is unprotected, and a setup page is
a door standing open for the length of it. `deploy/bootstrap.sh` creates the
first administrator and prints its generated password once. By hand:

    axgf-cms --bundle family.axgf --create-admin yourname

Everyone else is created from **Admin → Accounts**. There is no
self-registration and no invitation flow. For a family archive an administrator
who knows everyone is sufficient, and it removes an abuse surface — open
registration, invitation tokens, email delivery, and the account-enumeration
oracle each of those carries — rather than defending one.

### Three roles

They reuse the vocabulary the AXGF specification already defines for
`visibility`, so the two systems share one language rather than two.

| Role | Reads | Also may |
|---|---|---|
| `viewer` | `public`, `members` | — |
| `contributor` | plus `contributors` | create, update, upload documents |
| `admin` | plus `private` | manage accounts, delete, dedup, validate, export |

**Visibility is enforced on the server, on every read path** — the tree, the
panel fetch, the standalone record, the document bytes and the JSON endpoint.
Nothing hidden is rendered and then styled away.

A person you may not read is **redacted, not omitted**: their card keeps its
place in the tree and they still count among a child's parents, carrying no
name, no dates, no gender and no link. Omitting them would be a false statement
about the genealogy, and would also make every converted bundle look as though
the family died out two generations ago. The trade is deliberate: a signed-out
visitor can learn that a hidden person exists and how they connect, and nothing
else.

**A converted GEDCOM carries no `visibility` at all.** Where a record states
none, anyone marked `is_living` is treated as `members` and everyone else as
`public`. Check that this is what you want before publishing a converted
bundle.

### Family scope

A contributor can be restricted to a branch — a list of root person ids,
covering those people, their descendants and their spouses. It limits what they
may **change**, never what they may read; reading is governed by visibility
alone, and the two are kept apart on purpose.

### Sessions

A signed, `HttpOnly`, `SameSite=Strict` cookie, `Secure` when the request
arrived over TLS. Its name carries the installation's own mark
(`axgf_session_<8 hex digits>`), so two installations on one machine do not
sign each other's visitors out. A session lasts 30 days without use, and each
use moves that on. Sessions are kept beside the bundle in `family.sessions`
(mode 600), so a restart signs nobody out (docs/adr/0028); the emergency
token's session still ends with the process. Failed logins are throttled per
username and per client address. Disabling an account, lowering its role or
changing its password closes its open sessions immediately, and that holds
after a restart.

### The emergency token

`--admin-token` still opens an administrator session, and that is now its only
job: getting back in when the `.acl` has been lost or every administrator is
locked out. It is not an account — it owns no preferences, and the edit journal
records it as `emergency-token`. Its use is logged as a warning. Treat it like
a root password.

### Exposing it

`--bind` defaults to `127.0.0.1`. Binding to `0.0.0.0` without a reverse proxy
sends the login form in clear text. If you need it reachable:

1. keep it bound to localhost;
2. put nginx or Caddy in front, terminating TLS
   (snippets in [docs/DEPLOY.md](DEPLOY.md)), and make sure it sets
   `X-Forwarded-Proto`, which is what makes the session cookie `Secure`;
3. treat the emergency token like a root password.

---

## Architecture

**All genealogy logic lives in [axgf-rs](https://github.com/plkarin/axgf-lib).**
This application contains none of its own. It does not parse dates, merge
entities, validate structure or convert GEDCOM — it reads the bundle, calls a
library function, writes the bundle back, and renders HTML. The one thing it
decides for itself is *presentation*: how a date the library already parsed
should read in prose, and how a confidence should look on screen.

The server holds the bundle in memory behind a read-write lock. Every mutation
takes the write lock, calls the library, and — if the library refuses —
returns the diagnostics with memory and file both untouched. On success the new
archive is streamed into `family.axgf.tmp`, fsynced, and renamed over the live
file. The live file is never truncated and is not touched until the rename, so
a crash at any point during the export leaves the previous bundle intact.

There is no build step. No npm, no bundler, no framework, no CDN. Templates and
the stylesheet are ordinary files in the repository, embedded into the binary
at compile time, so the deliverable is exactly one executable.

### Development

```sh
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

Rust edition 2021, MSRV 1.92: krilla, which draws a note's PDF, asks for it (axgf-rs asks only 1.88). CI checks it.

To regenerate the demonstration bundle after editing `deploy/sample.ged`:

```sh
AXGF_CMS_REGENERATE_SAMPLE=1 cargo test --test sample_bundle
```

---

## Export back to GEDCOM

Not offered, and not planned. That format has nowhere to put how sure a fact
is, a relationship outside the family, the length of a job, or a date nobody
could pin down — the return trip would quietly drop all of it. Your archive
exports whole instead: `GET /admin/export` gives you the `.axgf`, which is a
ZIP of plain JSON you can read with any tool.
