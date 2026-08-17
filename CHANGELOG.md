# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] — unreleased

First release. A single binary serving a browsable, editable website for one
AXGF bundle, built to demonstrate what the format records that GEDCOM
discards.

### Added

**Showcase rendering.** Every page surfaces at least one thing GEDCOM cannot
express.

- Confidence is rendered visually — a filled bar, an inline opacity-and-underline
  treatment, or a dot — driven by four bands from a 0.0–1.0 score, so a fact at
  0.98 and one at 0.35 never look alike. Dash patterns carry the same signal for
  readers who cannot rely on colour or opacity.
- Dates are rendered at whatever precision the source actually supports: exact,
  month, year, decade, quarter-century, century, `circa`, and ranges as
  *between*, *before* and *after*. Text no converter could parse is shown
  verbatim instead of dropped, and a date that says nothing reads as
  "Date unknown" rather than a blank. A value shorter than its stated precision
  is never padded into a day the source did not assert.
- Non-family links — godparent, employer, witness, mentor — have their own
  prominent section, each with its own dates, source and confidence, and read
  from the viewed person's side using the link's reverse label.
- Occupations render as bars on one shared axis, so two posts can be compared
  by eye, with open-ended bars where a bound is missing.
- Sources are badged and ordered by reliability, so a claim resting on a birth
  certificate is visibly not the same as one resting on recollection.
- Places carry their border history, so a town that changed hands says so.

**Completeness readout.** The conversion result page and the admin dashboard
both count what the bundle in front of you records against what AXGF can hold:
confidences that were judged individually rather than stamped by a bulk import,
parentage confidence, non-family links, occupation spans, source reliability
grades, and every date broken down by the shape it actually has. Each row names
the AXGF field and links its specification section. The framing is plain — a
GEDCOM import shows five empty rows because GEDCOM has nowhere to put any of
it, not because the conversion lost anything — and a bundle that already
carries rich data is told so instead.

**Pages.** Home with a "Why AXGF" panel and a list of the GEDCOM-impossible
features the loaded bundle actually contains; `/tree`; `/person/:id`;
`/convert`; `/health`; and a plain server-rendered admin panel with per-kind
forms, paginated listings, validate, deduplicate and export.

**The identity page is the whole record.** `/person/:id` is divided into
sections, each present only when it has content, so the shape of the page is
itself a readout of what the bundle carries: **Identity** — every recorded
name with its type, the period it was used and the source behind it, with the
native script and its Latin transliteration side by side where they differ,
plus gender, living status and visibility; **Life events** — birth, death and
every event this person participated in, in date order, each with their role,
so a marriage they merely witnessed appears alongside their own, and an
undated fact sorts last rather than pretending to come first; **Family** —
parents and siblings, then each union with its type, dates, place, how it
ended and its children in birth order; **Other relationships** — every link
with this person at either end, read from their side, so the same record shows
as "godfather of" from one end and "godchild of" from the other; **Occupations**
as spans; **Places**, each listing what happened there and carrying its border
history; **Sources and documents**, where every source names the facts on the
page that rest on it; **Notes**, including text a converter could not parse and
kept verbatim; and **Raw**, a collapsed block holding the entity's own JSON,
because a format worth arguing for should be readable without a tool.

**Tree view.** Oldest generation at the bottom, youngest at the top. The
default is a focused subtree — ancestors and descendants of one person to a
configurable depth (default 3 each way) plus their partners — because the whole
bundle in one page is over 23,000px wide on a real file and nobody scrolls that
far. Every card re-centres the view on whoever it names. `?all=1` still draws
everything, with a warning stating the canvas width. Anyone in no family gets a
labelled band rather than being dropped. Connector opacity is the
relationship's confidence.

**Generations respect marriage, not just descent.** Two spouses share a
generation however much of each side's ancestry happens to be written down.
Deriving depth from parent-child edges alone put the operator in generation 14
and his wife in generation 1 — each number correct on its own, the pair wrong —
and split 236 of the bundle's 287 couples across different rows. Each union is
now contracted to a single node before the parent-child constraint is solved,
so spouses cannot come out on different rows, and a married-in line with no
further recorded ancestry is slid down to sit directly above the descendant it
attaches to instead of stranding a parent fourteen rows from their child. A
bundle that contradicts itself — someone recorded as their own ancestor, or two
people on one line of descent recorded as a couple — has the offending
relationship left out of the numbering and says so on the page. All 287 couples
now share a row. On the operator's 767-person, 295-family bundle: focused 16ms,
full 17ms, down from 143ms and 133ms.

**Attached documents and photographs.** An AXGF bundle already carries binary
attachments — files under `documents/files/**` inside the ZIP — and
`import_bundle` and `export_bundle` already round-trip them, so uploading is
wiring rather than a format change; `axgf-rs` is untouched.
`POST /admin/person/:id/document` takes a multipart upload, hashes it, and
writes the Document entity and the bytes in one atomic rewrite, so the picture
travels inside the `.axgf` rather than beside it. `GET /document/:id/raw`
serves the stored bytes and `GET /document/:id/thumb` a downscaled PNG, cached
in memory by document id and content hash under a byte budget so a gallery
neither re-decodes on every request nor grows without limit. Images render as a
gallery on the identity page, everything else as a list with a download link,
and the upload form appears only for an admin.

The type of an upload is read from its own leading bytes, never from the
filename or the `Content-Type` the client sent, and the check is an allowlist:
images, PDF, plain text, audio and video are recognised, and anything else —
every executable format among them — is refused because nothing matched it.
**SVG is refused outright**, not sanitised: it is a document a browser will run
scripts from, sanitising it means owning an XML element allowlist, and it has
no magic number to identify it by in the first place. Any SVG that arrives
inside a bundle authored elsewhere is served as a download. Every stored file
is served with `X-Content-Type-Options: nosniff`, and only the raster formats a
browser draws as pixels are served inline; everything else gets
`Content-Disposition: attachment`. Uploads are capped at 10 MB each, and the
admin panel reports the bundle size and warns past `--size-warn-mb`
(default 200 MB) — the *textual* bundle is what is held in memory, so that is
what the threshold measures.

**Conversion.** GEDCOM 5.5.1 to AXGF, with entity counts and every diagnostic
shown before the download link. `GEDCOM_UNRECOGNIZED_TAG` warnings are
presented as evidence that nothing was silently dropped. Conversion never
touches the served bundle.

**Storage.** The bundle file is the database. Mutations take a write lock, call
`axgf-rs`, and on refusal return the diagnostics with memory and file both
untouched. Writes are atomic: stream the new archive into `.tmp`, fsync,
rename. The live file is never truncated and is not touched until the rename.

**Binary payloads never enter memory.** Loading uses
`axgf-rs`'s `import_bundle_streaming`, which hands over one payload at a time
as a live reader, so each attachment is copied from the archive into a disk
cache through a fixed 64 KiB buffer and the resident bundle carries metadata
and an `external_payloads` declaration rather than bytes. Saving uses
`export_bundle_streaming`, which asks for one payload at a time and writes it
straight into the open ZIP entry. Peak memory for a load or a save is the copy
buffer, not the bundle and not its largest file. A cache entry deleted behind
the application's back makes the library refuse the save with
`PAYLOAD_SOURCE_FAILED` rather than write a bundle with the file missing; the
application reports it, rebuilds that entry from the `.axgf` on disk — which is
the authoritative copy — and retries once. `GET /admin/export` streams from a
temp file for the same reason, so downloading a backup costs a file handle.

**Deployment.** `deploy/bootstrap.sh` takes a fresh Ubuntu LTS host to a
running site in one command, and is idempotent — re-running preserves both the
bundle and the admin token. `--with-sample` seeds a demonstration family that
exercises every showcase feature. A hardened systemd unit binds to localhost.
Release workflow builds `x86_64-unknown-linux-musl` and
`aarch64-unknown-linux-gnu` behind a test, clippy and fmt gate.

### Security

- Authentication is a single shared token in an `HttpOnly`, `SameSite=Lax`
  cookie, compared in constant time. A blank token is rejected in two places so
  an empty `AXGF_CMS_ADMIN_TOKEN` can never become "no password required".
- The server binds to `127.0.0.1` by default, because V1's admin surface must
  not reach the network by accident.

### Notes

- All genealogy logic lives in [axgf-rs](https://github.com/plkarin/axgf-lib).
  This application contains none of its own; it reads the bundle, calls a
  library function, writes it back and renders HTML.
- No build step. No npm, bundler, framework or CDN. Templates and the
  stylesheet are embedded at compile time, so the deliverable is one binary.

---

## Not in this release

These are deliberate omissions, not oversights.

- **User accounts.** V1 authentication is one shared token, and that is the
  whole system: no accounts, no roles, no audit trail. Anyone with the token
  can edit or delete anything. This is why the default bind address is
  localhost. Planned for V1.2.
- **Per-entity visibility.** AXGF entities carry a `visibility` field. This
  release reads it but does not enforce it — with no user accounts there is no
  one to enforce it against. It becomes meaningful alongside accounts in V1.2.
- **AXGF → GEDCOM export.** Not implemented and not planned. GEDCOM has nowhere
  to put confidence, non-family relationships, occupation spans, graded sources
  or preserved uncertainty, so the round trip is lossy by nature — and that loss
  is the subject of this project. Use `GET /admin/export` for the `.axgf`
  bundle, which is a ZIP of plain JSON readable with any tool.
- **Editing list-valued fields through typed form controls.** A family's
  partners and children, a place's border history and similar collections are
  edited through the raw JSON textarea that every admin form carries. Nothing is
  uneditable; some things are just edited as JSON.
- **Document upload.** Document *metadata* is editable, and binary payloads
  already in a bundle are preserved across every write, but there is no upload
  form.
- **Search beyond name substring.** The tree filter is a case-insensitive
  substring match over display names, client-side. No fuzzy matching, no
  full-text index.

[0.1.0]: https://github.com/plkarin/axgf-cms/releases/tag/v0.1.0
