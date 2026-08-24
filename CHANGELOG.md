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

**The demonstration bundle declares itself public.** `deploy/sample.axgf` now
carries an explicit `visibility: "public"` on every person, link, event and
occupation. It was public before only by way of this application's rule for an
absent value — every person in it happens to be dead, and the dead default to
public — which is three coincidences deep and each of them can move. Adding one
living person to `sample.ged`, or a converter that stamps `members` the way the
specification's own `tools/gedcom2axgf.py` does, would have turned a fresh
`--with-sample` install into a site showing a signed-out visitor an entirely
redacted tree. That reads as a broken install, not as a privacy control. Two
tests pin it: one on what the file declares, one on what a visitor with no
account is actually served.

Filed as [axgf-spec#1](https://github.com/plkarin/axgf-spec/issues/1): the
specification says every entity carries a `visibility` field, the schema does
not require it, no default is defined for its absence, and the three converters
in circulation each answer differently — `tools/gedcom2axgf.py` writes
`members` unconditionally, `tools/webtrees2axgf.py` and `axgf-rs` write nothing.

**Seven themes**, chosen by the reader and stored on the account, falling back
to a cookie and then to `prefers-color-scheme`: light, dark, high-contrast,
sepia, deuteranopia, protanopia and tritanopia. Every colour is a custom
property, and a theme is one block redefining them under a `data-theme`
attribute rendered *server-side* — a script that set it after load would flash
the wrong theme on every page, worst for exactly the reader who chose
high-contrast. A test fails if any rule outside the theme blocks hardcodes a
colour, because a literal is one no theme can override.

**Confidence survives without colour, in every theme.** Colour is precisely
what disappears under colour blindness and confidence is this product's
argument, so it is carried four ways over:

- a bar's **length** is the value, and length survives everything;
- the dot is a **pie filled to the value** rather than a disc tinted by band.
  It used to distinguish `certain`, `high` and `medium` by hue alone — and the
  dot is what the tree cards and the dense lists use, so that was the one place
  the whole argument leaked out;
- the **numeric percentage** is printed beside the bar and is in the accessible
  name of both, so a screen reader hears "0.35" rather than "orange";
- the three colour-blind themes drop hue from the ramp altogether and use a
  **lightness ramp** of one safe hue, because lightness is the one channel no
  form of colour blindness takes away.

Verified by rendering, not by reasoning: a strip of indicators at 98/82/62/45/12%
was rendered under each theme, then again through a dichromacy matrix, then
again fully desaturated. At every step the shorter bar and the emptier pie
still read as less certain.

**Residual edge-crossing hues are paired with dash patterns.** The marker is
now an index the stylesheet turns into both a hue and a rhythm, so two lines
that converge in hue under a colour-blind theme are still one dashed and one
dotted. A test asserts all eight patterns differ.

**WCAG AA, measured rather than asserted.** Every pair was read from
`getComputedStyle` in a real browser across two pages and seven themes. The
worst *required* pair per theme:

| Theme | Worst required pair | Ratio | Needs |
|---|---|---|---|
| light | tree card border | 3.09 | 3.0 |
| dark | select border | 3.13 | 3.0 |
| high-contrast | small muted text | 17.4 | 4.5 |
| sepia | select border | 3.04 | 3.0 |
| deuteranopia | tree card border | 3.05 | 3.0 |
| protanopia | tree card border | 3.05 | 3.0 |
| tritanopia | tree card border | 3.07 | 3.0 |

The sweep found four real defects that reading the stylesheet had not:
form controls inherited the user agent's black text, giving **1.26:1 in dark
mode** — an unreadable input; `--border-strong` was 1.7–2.1:1 where a control
boundary needs 3:1; the tree card outlines were 2.1–2.5:1; and the confidence
bar's track was `--border`, which under high-contrast is pure black, so a 12%
bar rendered as a mostly-dark bar and looked *more* certain than a 98% one —
inverting the signal in the theme that needs it most. The track has its own
variable now.

Muted secondary text is where dark palettes usually fail, so it was measured
first: it clears 4.5:1 in every theme (5.2–5.8, and 17.4 in high-contrast).
Purely decorative separators are reported alongside but not failed — WCAG
1.4.11 exempts them, and the lowest is 1.0–1.4:1.

**`prefers-reduced-motion` is respected.** A medical setting, not a taste: the
tree's hover transitions fire hundreds of times as a pointer crosses a canvas
of cards. Durations go to 0.01ms rather than 0, because a zero-length
transition never fires `transitionend` and anything awaiting it would hang.

**Ten interface languages, translating the interface and never the data.**
An English speaker browsing a Polish family wants English buttons and Polish
place names. AXGF carries that distinction itself — `place.names[].lang`,
`name.components[].value_latin` — and rendering *Kraków* as *Cracow* would be
a worse product and a false record. Labels, headings, help text and messages
are translated; names, places, notes, occupations, source titles and every
date's *meaning* come from the bundle in their own language and script.

- **Fluent (CLDR plural rules)**, because hand-rolled `if n == 1` logic is
  simply wrong in most of these languages and wrong in a way an
  English-speaking author cannot see. Verified per locale: Arabic selects its
  dual at n=2 and distinct `few`/`many` at 3 and 11; Polish splits `few` (2–4)
  from `many` (5+); Chinese and Japanese have one form, which is the CLDR rule
  and not an omission.
- **Dates are localised in their words only.** `circa 1500` becomes `vers
  1500` in French — still 1500, still circa, still a year and not a day. The
  date helper takes the locale as a parameter; the value and the precision are
  data and are never touched.
- Selection order: the signed-in account's stored preference, then the cookie
  the selector sets, then `Accept-Language` negotiated against what is
  available, then English. A regional variant finds its base language, so
  `pt-BR` resolves to `pt` and `zh-CN` to `zh-Hans`.
- **Right-to-left is a layout, not a translation detail.** Arabic is served
  with `dir="rtl"` rendered server-side, and the stylesheet is written in
  logical properties, so the page mirrors without a second stylesheet: the
  panel moves to the left, the tree to the right, sibling order reverses.
  Generations still stack vertically, because mirroring is horizontal. Two
  things needed explicit handling — the SVG that connects the tree's cards is
  drawn in absolute coordinates and is reflected with `transform: scaleX(-1)`,
  and dates are wrapped in `<bdi>` because the bidirectional algorithm
  reorders neutral characters and was turning `>1930` into `1930<`, which is a
  different claim. A message that falls back to English inside an RTL page is
  wrapped in Unicode isolates for the same reason.
- **Verified by rendering, not by reasoning.** The layout was measured in a
  real browser at 1920, 1440, 1280 and 900px, in both directions: no
  horizontal page overflow anywhere, the panel and tree swap sides, and every
  connector's endpoint lands on the centre of the card it names in both
  directions. Rendering the Arabic page found three bugs that reading the CSS
  had not — the reordered dates, untranslated band labels coming from Rust
  rather than a template, and an English fallback scrambling inside an RTL
  paragraph.
- **Honest translation quality.** English and French are reviewed and complete
  at 505 of 505 messages. The other eight are machine-quality, labelled as
  such, and rather than a bare flag the selector shows each one's real coverage
  — Arabic 61%, the rest 43% — with everything else falling back to English
  rather than being guessed at. `CONTRIBUTING.md` states where each language
  stands and how to correct one. Genealogical vocabulary is exactly where
  machine translation fails, and the Polish date rendering demonstrates it:
  it puts a nominative month where the genitive belongs, writing
  "12 kwiecień 1923" for what a Polish speaker writes "12 kwietnia 1923".
- **A linter keeps it translated.** Four tests fail on a literal English string
  in a template, one passed into a macro, one in a `title`/`aria-label`
  attribute, or a key a template asks for and no locale defines. Translating an
  application once is easy; the next person writes `<h2>Sources</h2>` without
  thinking, and nothing complains until a reader of Polish finds an English
  heading in their page.

**Two people can edit the same record without one of them losing their work.**
Until now the second save silently overwrote the first — no error, no warning,
one person's work simply gone. That is data loss, and it is what made the CMS
unusable by a family.

Optimistic locking on the `version_num` AXGF entities already carry. The edit
form embeds the version it was rendered from; the save compares it against the
bundle's current value **inside the write lock**, because comparing it in the
handler and writing afterwards would leave a window for exactly the race this
closes. Unchanged, it applies and increments. Changed, it refuses.

- The library stores `version_num` and does not increment it, so the increment
  is the application's; it happens under the same lock as the comparison.
- A save that declares no version at all — an old form, a script posting by
  hand — is refused rather than allowed. Falling back to the stored version
  would make the check pass by default, which is the one thing it must never
  do.
- **Nothing is ever merged automatically.** A merge produces a record no human
  chose, and in a genealogy two editors disagreeing about a date usually means
  they are reading different sources — a question for a person, not for a
  program.

**The conflict screen shows all three versions.** Who changed it, when, and a
field-by-field diff between the version the editor started from, the version
that is now current, and their own edit. Fields both editors touched are
called out separately from fields only one of them touched, because those are
the only ones where re-applying actually destroys something.

The bundle holds only the current version, so the one the losing editor opened
is gone — it is *reconstructed* by replaying the journal's `from`/`to` pairs
backwards from the current document. Where the journal cannot account for
every version in between (an edit made before journalling, or by another tool)
the page falls back to a two-way comparison rather than presenting a confident
reconstruction that might be wrong. The editor's own text is carried forward
against the new version, so accepting the conflict is one click and not a
retype.

**An edit journal**, appended on every successful mutation: timestamp, user,
entity kind and id, and the field-by-field diff. JSON Lines, mode 600, beside
the bundle rather than inside it — for the same reason the accounts are: a
`.axgf` is copied, mailed and published, and "grandmother's birth year was
wrong for six months and Anna fixed it" is a fact about the family's *editors*,
not about the family. Appending is one write on an `O_APPEND` handle, so a
crash can lose the last line but cannot corrupt the ones before it, and a
torn line is skipped on read rather than making the history page unreachable —
which is exactly when somebody needs it.

It is surfaced on the admin dashboard, on each entity's edit form, and as a
**History** section on the record page — the last of those only for readers who
are signed in, since publishing it would put the editors' names back in front
of exactly the audience the separate file keeps them from.

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

**The shared token is replaced by accounts.** `POST /admin/login` takes a
username and a password. `--admin-token` survives as the *emergency recovery*
path and nothing else: it opens an administrator session for getting back in
when the `.acl` has been lost or every administrator is locked out, it owns no
preferences, the edit journal records it as `emergency-token` rather than as a
person, and its use is logged as a warning. It is now behind a disclosure on
the sign-in page rather than being the form.

- The first account is created by `--create-admin <username>`, which prints a
  generated password once to stderr and exits without serving. There is no web
  setup page on purpose: the window between deploying and the first login is
  exactly when an installation is unprotected, and a setup page is a door
  standing open for the length of it. Creating it from the shell requires
  access to the host, which whoever is in that window does not have.
  `deploy/bootstrap.sh` calls it on a fresh install and prints the credentials
  in its summary. Re-running refuses an existing username rather than resetting
  it, so a bootstrap script can run on every deploy without silently rotating a
  working account's password.
- Everyone else is created from **Admin → Accounts**, by an administrator. No
  self-registration, no invitations — see *Not in this release*.
- A failed login answers identically whether the username exists, the password
  was wrong, or the account is disabled, and an unknown username is verified
  against a real Argon2id hash so that it costs the same wall-clock time as a
  known one. At these parameters the difference would otherwise be tens of
  milliseconds — not a subtle timing signal needing statistics to extract, but
  a plainly visible one that turns the form into a list of which accounts
  exist.
- The account is re-read from the ACL on every request rather than cached in
  the session, so disabling an account, lowering its role or changing its
  password takes effect on the next request. Each of those also closes that
  account's open sessions. A demotion that waited for a cookie to expire would
  be advisory.
- The last active administrator cannot demote or disable themselves. Recovering
  an installation with no administrator means editing the `.acl` by hand or
  using the emergency token, and neither should be the result of a stray click.

**Family scope limits writing, never reading.** A contributor restricted to a
branch may edit those people, their descendants and their spouses. The
accessible set is computed once per request — it is a walk of the family graph
— and applied to every write in it.

- **Every** person a record names must be inside the branch, not merely one. A
  family with one partner from outside would otherwise be a way to rewrite that
  person's parentage from inside the branch.
- Both the submitted entity and the stored one are checked. Checking only the
  submission would let a scoped contributor retarget a record they may edit at
  people they may not; checking only the stored one would let them edit a
  record into their branch that never belonged to it.
- A record naming nobody — a source, a place — is refused to a scoped account.
  There is no branch to measure it against, so allowing it would be a hole in
  the scope rather than an exception to it.

**The panel is no longer admin-only.** A contributor reaches the forms, the
listings and document upload; delete, dedup, validate, export and account
management stay with `admin`. A refusal states which of the two stopped it —
a contributor landing on the account list is told their role is the reason,
not shown a login form for the account they are already using.

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
