# axgf-cms

A single Rust binary that serves a browsable, editable website for one
[AXGF](https://github.com/plkarin/axgf-spec) genealogy bundle. The `.axgf`
file *is* the database — no SQL, no cache server, no external service.

```
[browser] → [axgf-cms binary] → [axgf-rs crate] → [family.axgf on disk]
```

This is the reference showcase for the format. It exists to answer one
question for someone who has never heard of AXGF: **load a GEDCOM, and see
immediately what AXGF expresses that GEDCOM cannot.**

---

## Why AXGF

GEDCOM records what a genealogist concluded. It has nowhere to record how sure
they were, or why. A date is either there or it is not; a parent is either
stated or absent. Everything in between — the census that narrowed a birth to
a five-year window, the family letter that named a godfather, the entry no one
could read — is either discarded on export or flattened into a note nobody
parses again. AXGF is built for the in-between, because in genealogy the
in-between is most of the evidence.

Every fact in an AXGF bundle carries a confidence from 0.0 to 1.0, and this
site renders that visually rather than as a number in fine print. A birth date
recorded at 98% and a speculative parentage at 35% do not look alike anywhere:
not on the identity page, not in a list, and not on the tree, where a faint
connector means the record is not sure of that relationship. Dates keep their
shape — `circa 1500`, `before 1430` and `between 1920 and 1925` survive as
distinct statements instead of collapsing into one blank field, and text no
converter could parse is shown verbatim rather than dropped.

Relationships beyond blood are first-class. "Jean was Jules' godfather from
1950, per a family letter, 85% confident" is a single entity with its own
dates, source and confidence — a sentence GEDCOM cannot express at all, and
one this site gives its own section. Occupations are spans with a duration
rather than events with a date, so "schoolteacher, 1948–1978" renders as a bar
you can compare against another. Sources are graded — primary, secondary,
oral, DNA — so a claim resting on a birth certificate is visibly not the same
as one resting on a relative's recollection sixty years later.

Read the specification at
[github.com/plkarin/axgf-spec](https://github.com/plkarin/axgf-spec).

---

## Install

On a fresh Ubuntu LTS machine:

```sh
curl -fsSL https://raw.githubusercontent.com/plkarin/axgf-cms/main/deploy/bootstrap.sh \
  | sudo bash -s -- --with-sample
```

That installs the binary, creates a `axgf-cms` system user with no shell, sets
up `/var/lib/axgf-cms`, generates an admin token into `/etc/axgf-cms/env`
(mode 0600), installs and starts a systemd unit bound to `127.0.0.1:8080`, and
prints the token and URL once at the end.

`--with-sample` seeds a small demonstration family so a fresh install has
something to look at. Drop it for an empty bundle.

The script is idempotent: running it again will not overwrite your bundle and
will not regenerate your token. Add `--dry-run` to see exactly what it would
do first, or `--from-source` to build with cargo instead of downloading a
release.

Manual installation, systemd reference, reverse-proxy configuration and backup
advice are in [docs/DEPLOY.md](docs/DEPLOY.md).

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
| `--admin-token <TOKEN>` | `$AXGF_CMS_ADMIN_TOKEN` | Shared admin token. If neither is set, a random one is generated and printed once to stderr. |
| `--seed-sample` | off | When creating a *new* bundle, seed it with the built-in demonstration family. Ignored if the bundle already exists. |
| `--size-warn-mb <MB>` | `200` | Bundle size past which the admin panel warns that the archive is getting heavy. Not a limit. |

### Routes

Public, read-only:

| Route | What it shows |
|---|---|
| `GET /` | Why AXGF, what is in this bundle, entry points |
| `GET /tree` | A focused subtree around one person, oldest generation at the bottom. `?root=<id>` centres it, `?depth=<n>` sets how many generations each way (default 3), `?all=1` draws the whole bundle |
| `GET /person/:id` | The whole record for one person, in sections: identity and every recorded name, a chronological life timeline, family, non-family relationships, occupations, places, sources and documents, notes, and the entity's raw JSON. A section with no content is omitted |
| `GET /convert` | GEDCOM → AXGF conversion |
| `POST /convert/gedcom` | Convert an upload, report what it carried against what AXGF holds, and offer the result |
| `GET /document/:id/raw` | The stored bytes of an attached file, with `X-Content-Type-Options: nosniff`. Raster images are served inline; everything else downloads |
| `GET /document/:id/thumb` | A downscaled PNG of an image, `404` for anything else |
| `GET /health` | `200` with entity counts |

Admin (requires the token cookie):

| Route | What it does |
|---|---|
| `GET /admin` | Counts, bundle completeness, validation report, operations |
| `GET/POST /admin/login`, `POST /admin/logout` | Session |
| `GET /admin/:kind` | Paginated, filterable listing |
| `GET /admin/:kind/new`, `POST /admin/:kind` | Create |
| `GET /admin/:kind/:id/edit`, `POST /admin/:kind/:id` | Update |
| `POST /admin/:kind/:id/delete` | Delete, with a referential-integrity policy |
| `POST /admin/person/:id/document` | Attach a file to a person — multipart upload, stored inside the bundle |
| `POST /admin/validate`, `POST /admin/dedup` | Run the library's checks |
| `GET /admin/export` | Download the live bundle |

`:kind` is one of `person`, `family`, `event`, `link`, `occupation`, `source`,
`place`, `document`.

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

**Textual data is memory-resident; binary payloads are not.** At startup, after
the bundle is imported, every attachment is written out to a disk cache and
dropped from the in-memory bundle. What stays in RAM is the textual data —
persons, families, document *metadata*, the manifest — bounded by the size of
the tree, not by its media; on the operator's 420 MiB archive that is under a
megabyte. Payloads are streamed from the cache on `/document/:id/raw` and
`/thumb`, and folded back in only for the moment it takes to export a complete
`.axgf`. Single uploads are still capped at 10 MB, and the admin panel warns
once the *textual* bundle passes `--size-warn-mb` (default 200 MB).

The cache lives at `<bundle_dir>/.axgf-cms-cache/<bundle-sha>/` by default, or
wherever `--cache-dir` points; it is keyed by a hash of the bundle so a
different bundle never reads another's payloads. A restart on an unchanged
bundle verifies the cache by sha256 and skips extraction, so it is fast rather
than a full rewrite. The cache is **derived data** — the `.axgf` is the
authoritative copy — so it does not need backing up, and can be deleted at any
time; the next start rebuilds it. A document whose bytes live elsewhere is still
recorded with `status: "referenced"`, which the identity page renders without
offering a download.

---

## Security: V1 has no user accounts

**Read this before exposing the site.**

Authentication is a single shared token in a cookie. That is the whole system.
There are no user accounts, no roles, no per-entity visibility and no audit
trail. Anyone who can reach the port and holds the token can edit or delete
anything in the bundle.

This is why `--bind` defaults to `127.0.0.1`. Binding to `0.0.0.0` without a
reverse proxy in front puts an unauthenticated-by-modern-standards admin
surface on the public internet, and the login form itself would be sent in
clear text. If you need this reachable from elsewhere:

1. keep it bound to localhost;
2. put nginx or Caddy in front, terminating TLS
   (snippets in [docs/DEPLOY.md](docs/DEPLOY.md));
3. treat the admin token like a root password.

Per-user accounts and per-entity visibility are V1.2. Until then the honest
position is: this is a viewer and a personal editing tool, not a multi-user
application.

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
returns the diagnostics with memory and file both untouched. On success the
bundle is exported, written to `family.axgf.tmp`, fsynced, and renamed over the
live file. The live file is never truncated, so a crash mid-write leaves the
previous bundle intact.

There is no build step. No npm, no bundler, no framework, no CDN. Templates and
the stylesheet are ordinary files in the repository, embedded into the binary
at compile time, so the deliverable is exactly one executable.

### Development

```sh
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

Rust edition 2021, MSRV 1.88 (inherited from axgf-rs).

To regenerate the demonstration bundle after editing `deploy/sample.ged`:

```sh
AXGF_CMS_REGENERATE_SAMPLE=1 cargo test --test sample_bundle
```

---

## Export back to GEDCOM

Not implemented, and not planned. GEDCOM has nowhere to put confidence,
non-family relationships, occupation spans, graded sources or preserved
uncertainty — the round trip is lossy by nature, and the loss is precisely the
subject of this project. Use `GET /admin/export` to get your `.axgf` bundle,
which is a ZIP of plain JSON you can read with any tool.

---

## Licence

Apache-2.0. See [LICENSE](LICENSE).

- Format specification: [plkarin/axgf-spec](https://github.com/plkarin/axgf-spec)
- Reference library: [plkarin/axgf-lib](https://github.com/plkarin/axgf-lib)
