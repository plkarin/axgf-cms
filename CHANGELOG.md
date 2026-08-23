# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] — unreleased

Panel content laid out for the column it has, user accounts with roles and
per-entity visibility, safe concurrent editing, ten interface languages and
seven themes.

### Added

**The record reads in a 460px column.** `0.1.0` merged the identity view into
`/tree` as a side panel and clamped the panel's grid track, but the content
inside it was still written for a page column two to three times wider. At the
clamp's 320px floor — what a 1231px viewport gives — the identity block spent
half the panel on a four-column table stating a single fact.

- A single name is a heading with its type as a small label beneath it. The
  four-column table survives only where several names exist and comparing them
  is the point, and even then not in the panel, where several names stack as
  blocks instead. Nothing is dropped on either surface: every name keeps its
  type, period, script, transliteration and evidence.
- A name's components are label/value pairs rather than one joined string, so
  `given name: Laura` is a single unbreakable box and the line can only break
  between components. The separator sits inside the component it follows, so a
  line can never begin with one.
- Gender, living and visibility became labelled chips — label above value in a
  bordered box — replacing small-caps labels interleaved with values that read
  as one run-on string: `GENDER Female LIVING yes VISIBILITY members`.
- Every other section was audited at panel width and re-declared as one column
  there: **life events** (a 9rem date column left under 270px for the event),
  **occupations** (a 160px label column left ~150px of timeline track, which is
  the one thing the chart is for), **sources and documents** (five-column
  tables now stack into labelled lines driven by a `data-label` carrying the
  `th` text), **other relationships** and **unions** (box padding was a fifth
  of the line), and chips, which stop being `nowrap` where nowrap would push
  the panel sideways. **Family** no longer emits an empty two-column grid when
  only unions are recorded. **Notes** and **places** needed no change.
- The long section descriptions stay on `/person/:id` and move behind a `?`
  control in the panel. Dropping them there was the alternative and would make
  the panel a lesser record than the page it shares a partial with — the wrong
  trade now that the panel is where most reading happens. Collapsed they cost
  one 18px control per section instead of four lines.

**Cards fit the names in them.** The two-line clamp on `.tname` was in place
but the card was 58px tall, enough for one line plus the dates, so the second
line and its ellipsis were cut off by the card's own overflow and a long name
read as a truncated fragment rather than a clamped one. Polish given-name
chains make this the common case: `Alfons Władysław Antoni Wierzbięta` lost
everything after the first word. Cards are 66px; row pitch and the SVG
connector geometry derive from `CARD_H` and followed automatically.

### Security

**Accounts live beside the bundle, never inside it.** A `.axgf` is copied,
mailed, published and archived; password hashes in it would make every copy of
the family tree a copy of the credential store. `family.axgf` is the genealogy
and stays shareable; `family.acl` beside it holds the accounts at mode 600.
Encryption at rest is left to GPG and is out of scope here.

- Passwords are Argon2id at the OWASP 2024 parameters (m=19456, t=2, p=1),
  stored in PHC form so the parameters travel with each hash. A test asserts
  the stored parameters rather than trusting the constant, and another checks
  that a bare SHA-256 digest never verifies — SHA-256 is a fast hash for
  integrity, and a GPU tries billions of them per second.
- The file is created mode 600 through the temp file it is renamed from, so
  the hashes are never briefly world-readable. Loading refuses anything looser
  and names both the mode it found and the `chmod` that fixes it.
- Three roles reusing the specification's own `visibility` vocabulary, so the
  two systems share one language: **viewer** reads `public` and `members`;
  **contributor** adds create, update, document upload and `contributors`
  entities; **admin** adds user management, delete, dedup, validate, export
  and `private` entities.
- A bundle binding of family name, manifest `created_at` and the SHA-256 at
  creation, so one family's accounts applied to another family's tree are
  detected. The SHA moves on the first edit, so identity falls back to the two
  manifest fields. A genuine mismatch is reported rather than enforced: a
  restored backup is legitimate, and refusing to start would be worse.
- `family_scope` resolves a contributor's root person ids to a person set —
  the roots, their descendants, and the spouses of everyone reached, spouses as
  a leaf since following their ancestry would widen a branch scope back to the
  whole tree.

**Visibility and scope are separated at the type level.** Visibility decides
what a request may read and comes from the entity; family scope decides what an
account may write and comes from the account. A branch-scoped contributor still
reads the whole tree at their ceiling. Where a record states no `visibility` at
all — every converted GEDCOM — the default is stated on the one axis the format
does carry: an explicit value always wins, and failing that a person marked
`is_living` is `members` and everyone else is `public`. Guessing `public`
would publish living people the moment a bundle was imported; guessing
`members` would blank every converted bundle for visitors and look broken.

**Visibility is enforced server-side, on every read path.** Not in a template:
markup that reaches the browser has already left the building, and a name
hidden with CSS has been published with a note asking politely. Each request
resolves one lens at the top of the handler and passes it down; nothing below
that point reads a header or re-derives the answer.

The paths, all of them: the tree page, the panel *fragment* (`/tree/panel/:id`
returns no page, so no page-level check would ever have covered it), the
standalone record, the document bytes on all three of `/raw`, `/view` and
`/thumb`, the `/health` JSON, the root picker and the home page's showcase
counts and example links. `tests/visibility.rs` searches raw response bodies
for names that appear nowhere else in the fixture, so a leak is proof and not
a coincidence; removing the enforcement fails nine of its eleven tests.

**Hidden people are redacted, not omitted.** A person the reader may not read
keeps their card in the tree and their place among the parents, and carries no
name, no dates, no gender, no link and nothing for the client-side filter to
match on. Two reasons, the first decisive:

- Omission is a false statement. A record listing one parent where the bundle
  holds two asserts something untrue about the genealogy, and for an
  application whose argument is that a format should preserve what it knows,
  silently dropping a relationship is the one behaviour it cannot have.
- With the `is_living` default, omission would make every converted bundle look
  as though the family died out two generations ago. That reads as a broken
  import, not as a privacy control.

The consequence is deliberate: a signed-out visitor can learn that a hidden
person exists, sits in a given generation, and how they connect. Nothing else.
For the same reason a withheld record answers `403`, not `404` — the tree
already discloses that it exists, so a `404` would protect nothing while
telling a family member who is merely signed out that their record had been
deleted. The tree states the count in words rather than leaving silent gaps.
The one place that omits rather than redacts is the root picker, because every
entry there is a *destination* and a row reading "Private" leads nowhere.

**A link can be private when both its endpoints are not.** The one case that
does not reduce to person visibility — an acknowledged natural parentage, a
witness nobody wants named — so `link.visibility` is honoured on the link
itself rather than inferred from its ends.

**Sessions.** A signed cookie — 244 bits of session id and an HMAC-SHA256
signature under a secret generated at startup — `HttpOnly`, `SameSite=Strict`,
and `Secure` only when the request actually arrived over TLS, since setting it
unconditionally makes the cookie undeliverable on the documented
`http://localhost` deployment. Sessions are held in memory with a 12-hour
expiry, so a restart signs everyone out; at one family's scale that is a mild
inconvenience against a second persistence layer to back up and migrate.
Failed logins are throttled per username and per client address. Disabling an
account, lowering its role or changing its password closes every session it
holds.

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

- **Self-registration and invitations.** Accounts are created by an admin and
  by nobody else. For a family CMS that is sufficient, and it removes an abuse
  surface — open registration, invitation tokens, email delivery and the
  account-enumeration oracle each of those carries — entirely rather than
  defending it. There is no web setup page for the same reason: the window
  between deployment and first login is exactly when an installation is
  unprotected, so the first admin is created by the bootstrap script, which
  prints a generated password once to stderr.
- **AXGF → GEDCOM export.** Not implemented and not planned. GEDCOM has nowhere
  to put confidence, non-family relationships, occupation spans, graded sources
  or preserved uncertainty, so the round trip is lossy by nature — and that loss
  is the subject of this project. Use `GET /admin/export` for the `.axgf`
  bundle, which is a ZIP of plain JSON readable with any tool.
- **Editing list-valued fields through typed form controls.** A family's
  partners and children, a place's border history and similar collections are
  edited through the raw JSON textarea that every admin form carries. Nothing is
  uneditable; some things are just edited as JSON.
- **Search beyond name substring.** The tree filter is a case-insensitive
  substring match over display names, client-side. No fuzzy matching, no
  full-text index.

[0.1.0]: https://github.com/plkarin/axgf-cms/releases/tag/v0.1.0
