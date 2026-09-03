# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]


### Added

**A masthead on the person page.** A photograph when the bundle carries one
whose bytes are present, and otherwise the person's own initials on the theme's
accent — the same size and shape, so the heading beside it does not move
depending on whether a photograph exists. On a converted bundle most people
have none, which makes the placeholder the common case rather than the failure
case.

Beside it: the full name, alternative names, the span rendered at the precision
the record supports, and the age — at death, or now for someone living, and
omitted whenever either end is missing or the arithmetic would be a guess. Then
a line of facts placing them: born where, died where, how many children, how
many generations of descendants the bundle records below them. Each part
appears only when the record states it. Living status is shown to everyone; the
visibility level only to the people who can change it.

The generation count is a breadth-first walk of the family graph with a seen
set, which is not paranoia: the operator's bundle records two people as both a
couple and as parent and child, so the descent graph has a cycle in it.

The avatar prefers a document that declares itself a portrait and falls back to
the first present image. On a converted bundle neither of the first two ever
matches — the converter stamps `photo` on every scan and records one role,
`subject` — so the avatar is whichever image came first, which on the
operator's file is as often a death notice as a face.



**A soft wash behind the page.** Low-contrast CSS gradients, one set per theme,
mixed from that theme's own tokens toward its own hues — nothing in them is a
colour the theme did not already contain. Each of the three tints lands two to
three L* from its page background, which is a contrast of about 1.05:1: felt
rather than seen. Sepia is warm (amber, terracotta, olive); dark is cool and
nearly imperceptible, at 0.7 grey levels of mean difference across the page.

Gradients rather than photographs, deliberately. A gradient is a known colour
at every point and can be measured against the text in front of it, where
contrast over a photograph varies pixel by pixel; and it adds no bytes to the
binary. The rule is enforced by a test rather than left as an intention: no
`url()` may appear in the page background.

**The wash varies by route, deterministically.** The same three tints in a
different arrangement on the tree, the import page and the admin area, so
moving between them feels like moving rather than reloading. The same page
always draws the same wash — nothing is random and nothing is per-visit.

**Nothing moves.** There is no animation and no transition on it, and not only
under `prefers-reduced-motion`: a gradient that drifts is a distraction on a
page somebody reads for an hour, which is a decision for every reader rather
than one the motion query makes for a few. A test says so.

**Turning it off.** A preference stored the way the theme and the language are
— cookie first, and on the account as well when there is one — so a reader who
turns it off has turned it off on every machine they sign in from. The theme
can also refuse it: `Theme::wash` is false for high-contrast, whose purpose is
maximum luminance separation, and the server resolves the theme's flag and the
reader's preference together into one `data-wash` attribute. The stylesheet has
no exception for high-contrast, because it needs none, and the next theme built
for legibility gets the same treatment by setting one boolean.

Content surfaces — the tree and record split, cards, tables, forms — keep their
solid backgrounds, so no text sits on the gradient. Measured rather than
asserted: with the wash on and off, a rendered page differs by at most 14 of
255 in any channel, and by not one pixel of type.

### Changed

**One scroll on the tree page, not three.** The page scrolled, the tree column
scrolled inside its own 82vh box, and the record panel scrolled inside its own;
which one the wheel drove depended on where the pointer sat, and the two inner
frames cut one surface into two windows. Both inner boxes now grow to their
content and the page scroll is the only vertical scroll on the page — verified
in a browser at 1920, 1440, 1280 and 390px, on a three-generation view and on
`?all=1`: one vertical scrollbar, none nested.

The tree keeps its horizontal scroll, because a 2,078px focused tree and a
24,124px full view cannot be shown any other way, and a sideways overflow in
its own column does not compete with a vertical wheel gesture. It is declared
on both axes (`overflow-x: auto; overflow-y: hidden`) because CSS will not let
them disagree: a lone `overflow-x` computes the other one straight back to
`auto`.

**The tree is pinned, not the record.** Once the panel grows with the page, a
long record — the operator's own is 2,295px against a 978px tree — pushes the
tree above the fold and the reader has to scroll past the whole record to reach
it again. Pinning the *panel* does nothing about that: it is the taller of the
two columns, and the tallest item in a grid row has no room to move inside it.
So the tree column is the one that sticks. It stays on screen for the full
length of the record, which is also where the clicking happens. A tree taller
than the viewport pins by its bottom edge instead of its top — the offset is
derived from the canvas height the layout already computes — so a whole screen
of tree stays in view either way, and the horizontal scrollbar, the last thing
in the box, stays reachable.

**The frames are gone.** The border, background and shadow that used to be
drawn around each column were the edge of a scroll region, and there is no
scroll region left to draw. The surface belongs to the split now, once, so the
tree and the record read as one page rather than two windows side by side.


**The tree fits the screen it is on.** A generation wider than the column it is
drawn in now folds onto as many rows as it needs, inside its own band, instead
of running off the side. On the operator's bundle the focused view was 2,078px
wide at every screen size; it is 1,246px at 1920, 954 at 1440, 808 at 1280 and
320 on a phone — the tree column's width in each case, so the horizontal
scrollbar is gone from the default view at every size measured.

A band is still one generation, and that is what the folding had to protect.
Three things say so at once: the rows of one generation sit 26px apart where
two generations sit 78px apart; the whole band has a single tinted zone behind
it, `rows` high; and the label spans the zone rather than sitting on the first
row. A band that had to fold also carries a bracket down its leading edge.
Vertical position still carries generation — everything inside one zone belongs
to one.

The fold keeps the barycentre ordering rather than undoing it. Rows are filled
left to right in the order the ordering pass produced, so reading a band is
reading its rows top to bottom, left to right — the same sequence the single
wide row had, and a test asserts the two orders are identical. A couple is
never split across a break unless the two of them will not fit a row at all,
which on a phone is any two cards.

**Connectors go around a folded band, not through it.** An edge leaving a card
on an inner row has to get past its own generation's other rows to reach the
one above. It climbs a lane: a vertical strip with no card in it, found among
the gaps between cards — which line up down the band, because every row of a
band shares a left edge — and falling back to two corridors kept clear at the
canvas edges, which always exist. Measured on the rendered paths rather than
asserted: on the full view the number of connector segments that run across a
card fell from 22 to 9.

**A person the reader may not read is a marker, not a card.** They keep their
place in the row — the shape of the family is not what is being withheld, and
omitting them would make a converted bundle look as though the line died out —
but they no longer keep a card's width. On the operator's bundle a signed-out
visitor may read nobody at all, so this is not a handful of cards but all 866.
Measured on its own, with folding disabled, it takes the anonymous full view
from 24,168px to 7,998 and the anonymous focused view from 2,122px to 750.

**The row width is negotiated, not guessed.** The layout is computed in Rust
and shipped as absolute coordinates — the connectors have to meet the cards
without a layout pass in the browser — so the width has to be chosen before the
reader's is knowable. There is a default for the no-JavaScript case, a `?w=`
parameter, and an `axgf_tw` cookie that `tree.js` fills in from the column it
actually measured, in the same order the theme and the language are negotiated.
Deliberately not a reload: re-rooting, changing the depth and following a card
are all full navigations already, so the measured width is in use within one
click of arriving.


### Fixed

**An error page carried a button labelled `None`.** `Chrome` has a `back`
field — where a preference form returns the reader to, which is the page they
are on — and the error context reused the name for the link out. Merging the
two contexts let the chrome's value win, so every error page grew a second
button pointing back at the page that had just refused the reader, labelled
with a rendered `None`.

**The record column was 24px too wide, and the tree paid for it.** The grid has
three tracks and therefore two gaps, but the panel's width subtracted one. The
tree is the only track allowed to shrink, so it absorbed the difference: a tree
laid out to exactly the column width it had been told about still overflowed by
a gap. Found by folding the tree to the measured column and watching a 24px
scrollbar survive it.


**Control borders were below WCAG 1.4.11, and had been.** The sweep that found
it was run over four pages rather than two, and against the darkest ground a
control can sit on rather than against `--surface`. `--border-strong` was
chosen to clear 3:1 on `--surface`, but form controls on these pages sit on
`--bg`, which is darker — 2.81:1 in sepia, 2.95 in tritanopia, 2.97 in the two
red-green themes, all of them under the 3.0 the standard requires and all of
them there before this release. Darkened in four themes, measured against each
theme's deepest gradient stop, which is now the worst case:

| Theme | Worst required pair | Before | After | Needs |
|---|---|---|---|---|
| light | input border | 3.00 | 3.07 | 3.0 |
| dark | compact button border | 3.13 | 3.13 | 3.0 |
| high-contrast | diagnostic code text | 7.46 | 7.46 | 4.5 |
| sepia | input border | **2.81** | 3.07 | 3.0 |
| deuteranopia | diagnostic code text | **4.33** | **4.33** | 4.5 |
| protanopia | diagnostic code text | **4.33** | **4.33** | 4.5 |
| tritanopia | input border | **2.95** | 3.05 | 3.0 |

Confidence still reads without colour, re-measured in a browser rather than
assumed: under the three colour-blind themes the four legend dots fall on a
strictly increasing greyscale ramp (deuteranopia and protanopia 41/70/114/162,
tritanopia 18/40/87/161), and in every one of the seven themes the dot's filled
*area* still descends 87/75/61/51% against a 97/82/62/30 target — an encoding
no form of colour blindness can take away. The wash reaches none of it: the
dots sit on solid surfaces.

### Removed

**The interface no longer explains itself.** Eighteen strings whose subject was
the software rather than the family are gone, ten more are trimmed to the fact
they opened with, and what genuinely needed saying moved behind the small `?`
the record sections already used.

Off the tree page: the sentence teaching the reader what a faint connector
means (the confidence legend is on the same page, three inches away, and a
legend is a key rather than a lesson), and the one telling them that clicking a
card opens the panel — a gesture they have already made by the time they can
read about it. The record summary above them stays: how many ancestors,
descendants and partners are drawn is a fact about the family.

Off the person page: the ten section descriptions. They were printed under the
heading on the standalone page and folded behind a `?` only in the narrow
panel; they are behind the `?` on both surfaces now. The words are unchanged
and one click away.

Off the home page: the five cards headed "What this does for a family". That is
the product's argument, the README makes it already, and a family opening its
own front page does not need to be sold the software it is looking at. The
family's name, what it has recorded, and where the tree says more than names
and dates are what is left.

Off the import page: the paragraph on what the import adds, and the note that
there is no way back to `.ged` — the README carries the latter under its own
heading. The confidence slider's four-sentence rationale is behind a `?`. So
are the raw-JSON edit form's merge rules and the fact that a contributor's
branch limits writing but not reading: both are traps rather than tutorials, so
they were kept rather than dropped.

What stays: the confidence legend, the record summary, the self-contradiction
banner, every diagnostic and error, and the import report — including which
entries were left behind, though no longer the sentence congratulating the
importer for listing them.

### Known

**The full view trades width for crossings, and `?all=1` still scrolls
sideways.** 866 people across 16 generations cannot be made legible by folding.
The widest generation holds 165, and folding it stops paying for itself long
before it fits: a generation over 48 people folds onto at most six rows and
then the canvas is allowed to be wider than the target. `?all=1` is 4,312px for
an administrator at every screen size, down from 24,124 — a scrollbar of three
screens instead of sixteen — and keeps its horizontal scrollbar, as it should.

The cost is crossings. Counted geometrically from the rendered paths, the full
view goes from 169 to 586 for an administrator and to 1,513 for a signed-out
visitor, whose narrower canvas folds harder. That is inherent rather than a
routing defect: folding two adjacent wide generations maps a grid onto a grid,
and the connectors between them must interleave. The default focused view pays
almost nothing for the same change — 0 crossings before, 0 to 3 after,
depending on the screen.

The built-in crossing counter cannot see any of this. It counts inversions
between adjacent layers, which is a property of the ordering, and the ordering
is untouched: 7,290 before the sweeps and 1,342 after, identical to the figure
before this change. The geometric count above was measured separately, from the
path data of a rendered page.


**Diagnostic codes are below AA in the two red-green themes, and this release
does not fix it.** `.diag-code` colours a severity with the confidence ramp,
and that ramp is deliberately a *lightness* ramp — under deuteranopia and
protanopia it runs L* 19 → 33 → 51 → 67, because "darker means surer" is what
survives colour blindness. Its light end therefore cannot carry 0.8rem text:
`--conf-medium` is 4.33:1 against white where AA asks 4.5, and `--conf-low`
would be 2.53:1 on a page that had an error-severity diagnostic to show. The
defect is that severity and confidence are different scales sharing a palette,
not that the palette is wrong, and correcting the ramp to suit the text would
break the encoding the product rests on. Diagnostics need severity colours of
their own; that is a change to the confidence system and does not belong in a
commit about backgrounds.

## [0.1.0] — 2026-08-XX

The first release. One binary serves a browsable, editable website for a
single family's AXGF archive: the tree, every person's record, the documents
and photographs, user accounts with roles and per-person privacy, safe
concurrent editing, eleven complete interface languages and seven themes.

This file was written up to here in one pass over the history, because it was
not kept honestly along the way — several commits changed the product and
added nothing (`eaadfe0` merged the record into the tree as a side panel,
`17f5b1c` clamped long names, `afaadeb` rewrote payload streaming), and later
entries caught up piecemeal. Everything below is the state of the code as it
stands, not a diary. There is no earlier version to differ from: the entries
are grouped by what they are rather than by when they landed.

### Added

**Performance is where it was.** `/tree` on the operator's 866-person bundle
is 16.83 ms against a 16.80 ms baseline, `?all=1` 21.00 against 20.97, and RSS
46.1 MB against 46.0 — measured with both builds serving at once and requests
alternating between them, so the machine's own load falls on both arms. The
work in this release added three Fluent lookups per timeline entry and a
resolved name pair on a page that almost never has one; none of it is
measurable.

**Release documentation made honest.** The changelog carried two sections both
marked *unreleased*, which cannot both be true when nothing has shipped; they
are one `[0.1.0]` now, with the early work kept as a subsection and the
handful of claims the reframing reversed corrected in place rather than left
to contradict the entries above them. The omissions list gained the three
things most likely to be mistaken for oversights — no database, no
multi-tenancy, no graph queries — each with the reason.

The README opened by calling itself "the reference showcase for the format".
It now opens with what the product does for a family, the one-line install
above the fold, who can reach it, and the translation position stated plainly:
two of eleven languages reviewed, nine complete but unread.

`docs/DEPLOY.md` had two gaps that would have bitten a real operator. The
manual installation never created an administrator, so anyone following it
instead of the script ended up with only the emergency token — it now includes
the same seed-and-create-admin step the script runs, verified to produce a
bundle with the ten sample people and a `.acl`. And the backup advice named
only the `.axgf`: restoring from it would have given the tree back with every
account gone. Both files are named now, with a table saying which one is safe
to share.

**bootstrap.sh was run end to end for the first time, and it was broken in
two ways.** It had never been run against a real release because there has
never been a release; both defects were on the happy path.

*A fresh `--with-sample` install served a signed-out visitor "0 of 0 people".*
The script installed the unit, started the service — which began seeding the
demonstration family — then stopped it a fraction of a second later so the
`.acl` could be written, and ran `--create-admin` against the same path. That
invocation found no bundle yet, because the seed had not finished writing, and
created an empty one. The service then came back up on the empty file. All the
file creation now happens before anything is running: the bundle and the first
administrator are made in one invocation that carries the seed flag, and the
unit's `ExecStart` no longer carries `--seed-sample` at all, so there is
nothing left to race with. A test asserts the seeded bundle actually contains
the family and that every person in it is public, rather than asserting the
ordering — a future reshuffle has to keep the result, not the shape.

*`--from-source` could not install the binary.* It ran `install` into
`/usr/local/bin` without creating the directory; only the local-binary branch
did that. On a normal machine the directory already exists, which is why it
went unnoticed, and it fails under a prefix or on a minimal image. The `mkdir`
is hoisted out of the one branch that had it.

Verified on this machine: `--from-source` clones and builds; a second run
leaves the bundle, the `.acl` and the token byte-identical and refuses
`--with-sample`; the privileged run creates the system user, writes the token
file `0640 root:axgf-cms`, installs and enables the unit, and the service comes
up listening on `127.0.0.1:8080` and serving 9 of 10 sample people to a visitor
who is not signed in. Torn down completely afterwards.

`deploy/sample.axgf` declares `identity.visibility: public` on all ten people
explicitly, so it needed no change. The comparison worth recording: the
operator's converted `wt-full.axgf` marks all 866 `members`, which is why that
bundle shows a signed-out visitor an entirely redacted tree.

**The first release test found that there was no release.** The `v0.1.0-rc1`
tag was pushed and the workflow ran and went red: `cargo test` failed on CI at
`converting_a_real_gedcom_reports_counts_diagnostics_and_a_download`, so the
build and publish jobs were skipped and no release was ever created. The 404
from `/releases/latest` was therefore not a prerelease being skipped over —
there was nothing published at all.

The test read its GEDCOM from an absolute path into a sibling checkout,
`/home/cbrain/axgf-lib/tests/fixtures`, and silently fell back to a
one-person inline GEDCOM when that path did not resolve. On this machine the
path resolved and the realistic fixture was used; on CI it never did, so the
fallback ran and the assertion that counts are shown by kind — plural — failed
there and only there. A fixture a test depends on belongs beside the test, so
`small.ged` is committed to this repository and read from
`CARGO_MANIFEST_DIR`, and the fallback is gone: a missing fixture is now a
panic naming the path, not a quietly different test.

Removing the fallback exposed two more tests reading the same way, both of
which had been passing on CI without testing anything. They took the
operator's own 866-person `tree.ged` — real genealogy about living relatives,
which is not going into a public repository — and one of them,
`unrecognized_tags_are_presented_as_a_feature_not_hidden`, wrapped its
assertions in `if body.contains("GEDCOM_UNRECOGNIZED_TAG")`, so on CI, where
the file was absent and the fallback had nothing unimportable in it, the test
asserted nothing at all. What it is actually about is the shape of a real
export rather than its size: stray `FAM` stubs left by deletions, empty tags,
records private to the exporter. `tests/fixtures/stray-records.ged` carries one
of each, and the `if` is gone — the diagnostic must appear now, or the test
fails and says why. The other moved to `small.ged`, which exercises the
completeness report it asserts on.

**Release candidates are prereleases, and `--version` is how you install one.**
The workflow published every tag the same way, so `v0.1.0-rc1` would have
become the latest release and the advertised one-line install would have handed
every new operator a release candidate. Tags carrying a semver prerelease
suffix are published with `--prerelease` now; stable tags are published with
`--latest` explicitly rather than by default.

That makes `/releases/latest` resolve stable releases only, which is what the
`--version <TAG>` flag is for. The flag has been in the script since it was
written and had never been used, because there was no release to point it at.
It works, and there are now tests that say so: the default asks for
`releases/latest/download/axgf-cms-<target>.tar.gz` and a tag asks for
`releases/download/<TAG>/axgf-cms-<TAG>-<target>.tar.gz`. A trailing
`--version` with no tag used to set the tag to the empty string and then run
off the end of the argument list, which under `set -e` ended the script with no
message at all; `--version`, `--bind` and `--admin-user` all require a value
now and say so. `--help` printed a fixed line range of the header comment, so
documenting the flag properly pushed `--dry-run` and the idempotency note off
the bottom of it; it prints the header down to its first blank line now, and a
test fails if an option the parser accepts is not listed.

*The download failure said the wrong thing.* Every failure got the same line —
"download failed. No release published yet? Use `--from-source`." — including
the case where a release had been published and the only problem was that it
was not the latest stable one. Sending somebody to a ten-minute source build
when `--version` would have worked is a misdiagnosis, so the script now asks
the API what is actually published and answers the case it finds: nothing
published at all (build from source); releases published but every one of them
a prerelease, so `/releases/latest` resolves none of them (install by tag, with
the newest tag named in the message); a latest release that publishes no asset
for this architecture; an unknown tag, listing the tags that do exist; and the
API being unreachable, which is none of the above and no longer claims to be.

*The download path had never been executed.* Every bootstrap test supplied a
binary through `AXGF_CMS_LOCAL_BINARY`, so the branch that fetches a release
had only ever been read. Two more test hooks alongside the existing ones,
`AXGF_CMS_RELEASE_BASE` and `AXGF_CMS_API_BASE`, point the script at a staged
mirror, and six tests now run it: the two URLs, the refused empty tag, a real
download whose checksum verifies and installs, a corrupted archive that is
refused and installs nothing, the two failure diagnoses told apart, and the
unknown tag.

Verified end to end against `v0.1.0-rc1` on this machine, from the release
artefacts packaged exactly as the workflow packages them and served over HTTP
with `/releases/latest` returning 404 the way a prerelease-only repository
does. With no `--version` the script names the tag to use instead of blaming a
missing release. With `--version v0.1.0-rc1` the fresh privileged install
downloads the tarball, verifies its SHA-256, installs a binary byte-identical
to the built one, creates the system user, writes the token `0640
root:axgf-cms`, seeds the sample and creates the administrator, and the service
comes up on `127.0.0.1:8080` serving 9 of 10 sample people to a visitor who is
not signed in. A second run keeps the bundle, the `.acl` and the token
byte-identical, refuses `--with-sample`, and prints no password. Torn down
completely afterwards.

**The product is called ax-genealogy.** The masthead read `axgf-cms`, which is
the crate, the binary, the systemd unit and the system user — none of which a
genealogist has any reason to know. Those all keep their names. What a reader
sees does not.

The display name is a single Fluent message, `app-name`, which every page
title already used; the two places that did not were the masthead, which was
raw HTML reading `axgf<span>-cms</span>` so that the halves could be tinted
separately, and the admin result page's title. Both now take the constant, and
the brand is one string in one colour — a name split across two elements to
style it is exactly the sort of place a rename never reaches.

Two tests keep it that way: one asserts every catalogue resolves `app-name` to
the same string, because the failure worth guarding is not the eleven edits a
rename costs but a rename that reaches ten of them; the other fails if a
template presents `axgf-cms` to a reader as anything other than a command line.
The one surviving occurrence is `axgf-cms --bundle family.axgf --create-admin`
on the empty-installation screen, which is the binary's real name and an
operator has to type it exactly.

There are no OpenGraph or Twitter card tags in this application, so there was
nothing to miss there.

**The self-contradiction banner names its people, and only editors see it.**
It used to say "run the validator from the admin dashboard to find it" — an
instruction a signed-out visitor cannot follow and an administrator has to
work at. The layout already knows exactly which parent-child edge it could not
honour, because that is where it stops; it now keeps the pair instead of
counting it. On the operator's own data the banner reads:

> Recorded both as a couple and as parent and child:
> **Jakub Tomasz Klicki** · **Bronisław Stanisław Klicki**

with a link to each record. Up to three pairs are named and the rest counted.
A reader who may edit but may not read one of the two sees the restricted
label rather than the name, since naming them would be a way to enumerate
people they cannot see.

A signed-out visitor is now shown none of it. That is not tidying an
inconvenient fact away: the banner exists to prompt a correction, and a
visitor can neither locate the error, edit either record, nor necessarily even
see the two people involved. Against that, the cost is real — on a public
page, in the product's own voice, "this tree contradicts itself" reads as the
software confessing a fault, and invites a reader to distrust an entire
family's record over one bad edge in one union. The tree already says where
the *data* is uncertain, through confidence; somebody's data-entry slip is a
different kind of thing and does not belong in the same channel. Contributors
and administrators see all of it, including the case where a parentage loop
sets the flag without any single pair being to blame.

**Three vocabularies that were never translated, found by looking.** A CJK
font was installed and Chinese and Japanese were rendered at 1280, 768 and
390px — the inspection the previous release could not do, because the headless
browser had no CJK font and both languages came out as tofu.

The typography was fine: full-width punctuation throughout, no English
punctuation carried across, correct line breaking with no line beginning on a
prohibited character, and dates in the numeric form — `1848年6月15日`, beside
`15. Juni 1848`, `15 июня 1848`, `15 czerwca 1848`, `15 de junio de 1848` and
`15 يونيو 1848`.

What the inspection did find was three places where a controlled vocabulary
never reached Fluent at all, so it rendered as its raw English enum value in
all ten other languages. Geometry could not have caught any of them, and they
were on the most-visited pages:

- **The count tiles** on the home page, the import report and the admin
  dashboard printed the collection's own key — `persons`, `families`,
  `events` — which is how the bundle spells it, not how a reader says it.
  They now go through `kind-*-plural`, which already existed.
- **The timeline label and participant role** were built with a
  `capitalise()` over the raw value, so a Japanese record read
  `Marriage · spouse`. Both now go through `vocab`, and `role-*` is a new
  vocabulary in all eleven catalogues.
- **Source reliability** was a hardcoded English `match` in Rust. Worse, the
  catalogues' `reliability-*` keys used words the specification does not —
  `tertiary`, `recollection` — so `derivative`, `authored` and `oral` had no
  message at all and appeared verbatim. All three are added everywhere.

`every_controlled_value_the_real_data_uses_is_translated` pins the values the
operator's own bundles actually contain, in every locale, and checks that an
unrecognised value still opens its underscores rather than rendering a message
id.

**Eleven complete catalogues, two of them reviewed.** Nine languages sat at
36% — Arabic at 51% — so two thirds of the interface fell back to English and
a reader in Warsaw got a mixture that reads as unfinished rather than as
multilingual. All eleven are now complete at 553 messages each.

Russian is new, and it is here for the domain rather than for the count: the
civil and parish registers of the former Russian Empire were kept in Russian,
so a researcher working on Polish, Lithuanian, Ukrainian or Belarusian records
is reading Russian documents. Its vocabulary follows those registers —
`восприемник` rather than the modern `крёстный` for a godparent, `первичный
источник`, `род занятий` for an occupation as a period rather than a post.

Complete is not reviewed, and the selector now says both. A finished machine
translation is still a machine translation; a bare "100%" would read as a
quality score rather than as a count, so the badge reads **complete, not yet
reviewed**. CONTRIBUTING.md carries what that labelling promises: where the
files are, what review means, how to submit a correction, and a table of the
terms most likely to be wrong — union, godparent, confidence against
reliability, primary source, occupation-as-a-period — as the place to start.
Every catalogue's header states the choices it made so a native speaker has
something specific to disagree with.

**Month names moved into each locale's date pattern.** One shared table cannot
be right for every language at once. Polish and Russian inflect the month
inside a full date, so a single table rendered `12 kwiecień 1923` where a
Polish speaker writes `12 kwietnia 1923` — the nominative where the genitive
belongs. Spanish and Portuguese need their prepositions (`12 de abril de
1923`), German its point after the day (`12. April 1923`), and Chinese and
Japanese need no month name at all, because `1923年4月12日` is a numeric
structure rather than a translated word.

The application now hands the pattern a day, a month **number** and a year,
and each locale spells it, inflects it, or ignores it. That retires the
`month-N` family and the `month-in-date-N` family that briefly replaced it: 24
keys per catalogue, which the two CJK locales would have carried without ever
rendering one.

**A test that plural categories cannot go missing.** This is the guard for all
of the above. Drop `[few]` from a Polish message and Fluent falls through to
`*[other]` in silence: no error, no failing test that reads English, and the
only person who can see "3 osób" where "3 osoby" belongs does not work on this
project. `no_catalogue_is_missing_a_plural_category` asks the running Fluent
stack which categories a language needs — this build's CLDR answer rather than
a table that drifts from it — and checks every plural-bearing message against
it. A companion test catches the opposite failure: a locale that renders a
count as a flat sentence where English has a selector.

It found a real one on its first run: `ar/tree-counts` was missing `[zero]`.
It also taught us something. Arabic's six categories made three family labels
look short, but they open with a literal `[0]` variant, and Arabic's `zero`
holds n = 0 and nothing else — so the literal already says everything the
category could. The test now asks whether a category holds any number the
literal does not, and only then demands it; Polish's `[2]` earns no such
exemption, because `few` also holds 3, 4 and 22.

**Memory, stated rather than buried.** Eleven complete catalogues cost
**3.7 MB of resident memory**: 42.2 MB before, 45.9 MB after, measured with
both builds serving `/tmp/wt-full.axgf` at once and requests alternating
between them. The catalogue sources grew from 182 KB to 560 KB, and parsed
Fluent resources expand that roughly tenfold — every locale is parsed at
startup and held for the life of the process.

Render time did not move: `/tree` went 16.64 ms → 16.26 ms and `?all=1`
20.96 ms → 20.32 ms, both inside the noise of the same window. Translation
happens per message lookup against an already-parsed bundle, and there are no
more lookups than before.

3.7 MB on a 42 MB baseline is not a problem today, and it is worth naming what
would fix it if it becomes one: catalogues are parsed eagerly because there
are eleven of them and a family server serves a handful of readers. Parsing a
locale on first use instead would trade a few milliseconds on the first
request in each language for most of that memory back. Nothing in the design
prevents it; it simply is not worth the complexity yet.

**Two layout defects the eleven-language render pass found.** Every page was
rendered in all eleven languages at 1280, 768 and 390px and measured for a
body that scrolls sideways and for elements whose text spills past their box.
Nothing overflowed — the prose reflows, and German and Russian ledes simply
take three lines where English takes two, which is what they should do.

The selector was another matter. At 390px the masthead wraps, so the control
the panel hangs from is no longer near the right edge, and a 22rem panel
anchored to it started at x = -223: entirely off the left of the page, in
every language including English. Below 30rem the masthead is now the
containing block and the panel is a full-width sheet beneath the header.

The second was caused by this release: the badge went from "machine, 36%" to
"complete, not yet reviewed", which pushed the longest row — 简体中文 ·
Chinese (Simplified) · badge — onto two lines while its neighbours stayed on
one. The panel is 2rem wider, which fits every row at desktop widths, and
below 30rem every badge takes its own line so all eleven rows are the same
shape rather than some wrapping and some not.

**The site speaks to a family, not about a file format.** Every public page
argued for AXGF: the home page opened with a "Why AXGF" panel comparing the
format against GEDCOM, the completeness readout explained each empty row as
something "GEDCOM cannot express" and linked the specification section that
defines the field, and the conversion page sold the format rather than
reporting the import. A family paying for somewhere to keep their tree was
being handed a specification argument.

What the product does for them is now what the home page says: the tree, the
documents and the photographs in one place; several relatives contributing
under their own roles, with every change attributed; privacy decided per
person rather than per tree; eleven interface languages, with names kept in their
own script; and the whole archive exportable as a single file the family owns.

Removed: the "Why AXGF" panel and its GEDCOM comparison, the seven
specification links and the `AXGF field` column from the completeness table,
`gedcom_cannot` from every metric, and the three-repository link row in the
footer.

Rewritten rather than removed:

- The strip on a person's record was headed *What GEDCOM would lose here* —
  a claim about a competitor stapled to somebody's grandmother. It is now
  **Notes on this record**, and each chip states a fact about the record:
  *a job recorded with a start and an end*, *a birth date the source could not
  pin down, shown as recorded*.
- The completeness panel keeps every count and both framings — *Where this
  tree could say more* on the dashboard, *What the import brought over* after
  an import — and says a blank row is "somewhere the record could grow, not
  something that has gone wrong" instead of blaming a file format for it.
- The conversion page is an **import report**: what came across, what could
  not be read and why it is listed rather than swallowed. It still refuses to
  write GEDCOM back out and still says why, because that is a genealogist's
  question about their own data, not an argument about formats.
- `bundle` left the interface. It is the word for the file on disk, not for
  what a family calls their record: *This bundle contains nobody to draw*
  became *There is nobody to draw yet*, and the home page's fallback heading —
  the largest words on the page — went from **This bundle** to a translated
  *This family tree*.

The attribution stays where the licence puts it. `LICENSE` and `README.md` are
untouched, and one discreet line sits in the footer of every page: *Your
family's archive is one file you keep, written in an open format so it will
still open long after this site is gone.* It carries the single link to the
specification that remains anywhere in the interface.

Every string went through Fluent. English and French are complete at 563 keys.
The eight machine-quality locales lost the entries whose *claim* changed —
a Polish reader is better served by reviewed English than by a fluent
translation of a promise the product no longer makes — so they fall from 40%
to 36% (Arabic 56% to 51%), which the language selector reports honestly.

**A visual pass over the controls.** A refinement of what was there, not a
redesign: no framework, no build step, one binary and a static directory as
before.

- **Buttons and form controls** share one height, one radius and one
  transition, so a row of them lines up on its centres instead of drifting by
  a padding value. Hover fills rather than fades — dropping opacity would take
  the label's contrast down with it, and several of these pairs have no margin
  to give away. Pressing moves the control one pixel: enough to feel, not
  enough to shift what is around it. The primary action reads as primary
  through weight and fill rather than size, so it does not break the rhythm of
  the row it sits in.
- **The language selector** had no rules at all and rendered as the browser's
  default disclosure triangle beside an emoji — `▸ 🌐 English`. It is now a
  control with a chevron that turns, opening a positioned panel so the
  masthead does not change height and shift the page under the pointer. Each
  language is listed in its own script with its English name and its real
  coverage. The globe is an inline SVG in `currentColor` rather than an emoji,
  which is a font dependency that renders as a replacement box wherever that
  font is missing and cannot take the theme's ink.
- **Tables** got taller rows — 0.5rem put the lines close enough that the eye
  lost its place scanning a column — a sticky header, a hover state, and
  tabular figures so a column of numbers compares digit by digit.
- **Cards, notices and the panel** carry a soft elevation composed from two
  per-theme tokens rather than a literal shadow each, with the high-contrast
  theme setting its strength to zero: that theme wants crisp edges, not soft
  ones.
- **Motion** runs at one duration and one easing across the interface, so it
  has a single tempo rather than one per component. The panel animates when a
  fragment fetch replaces its content — the swap was instantaneous and it was
  not obvious anything had happened. Every transition and animation collapses
  to 0.01ms under `prefers-reduced-motion`, verified by rendering the page
  under both settings.

Both constraints from `0.2.0` survive, re-measured rather than assumed:

| Theme | Worst required pair | Before | After |
|---|---|---|---|
| light | tree card border | 3.09 | 3.09 |
| dark | select border | 3.13 | 3.13 |
| high-contrast | small muted text → primary button label | 17.4 | 11.22 |
| sepia | select border | 3.04 | 3.04 |
| deuteranopia | tree card border | 3.05 | 3.05 |
| protanopia | tree card border | 3.05 | 3.05 |
| tritanopia | tree card border | 3.07 | 3.07 |

The restyle introduced new colour pairs, so those were measured too rather
than assumed to be safe: the hovered button (accent on accent-soft) is
5.82–9.30:1, the selector badges 6.44–21:1, the hovered row and selector
10.25–17.42:1. High-contrast's worst pair moved only because a new one — the
primary button's label at 11.22:1 — is now the lowest of a set that no longer
includes anything marginal.

Confidence still reads without colour: the strip at 98/82/62/45/12% was
re-rendered under each theme, through the dichromacy matrices and fully
desaturated. Bar length and pie fill are untouched by the restyle.

**The split follows the tree.** The panel's track was clamped but the split
was still a fixed ratio, so a three-generation view — about 620px of cards —
was given 1412px of a 1920px page and left an 809px band of nothing between
the drawing and the panel. On the widest screen, where there was most to give
away.

The canvas width is something the server already computes, so it now reaches
the stylesheet as `--tree-width`: the same figure that sets the canvas's own
inline width. The tree's column caps there, and the panel takes what is left
between its floor and a 560px ceiling — past which a line of the record's
prose stops being comfortable.

The panel is *computed from* the tree's width rather than left to the grid's
free-space sharing, and that is the part that matters. Grid distributes free
space equally until each track reaches its growth limit, so a panel merely
allowed to reach 560px got there before a 2078px-wide tree had taken anything;
at 1280px that left the tree 656px where it could have had 883. Writing the
panel as `clamp(min, 100% − tree − gap, max)` states the priority instead of
hoping for it.

Measured in a browser, band being the space between the drawn tree and the
panel:

| Tree | Viewport | Band before | Band after |
|---|---|---|---|
| 618px (3 generations) | 1920 | 809px | 15px |
| 618px | 1440 | 415px | 15px |
| 618px | 1280 | 296px | 15px |

15px is the grid gap itself — the deliberate separation, with nothing left
over. Where the tree is wider than its column it still scrolls inside it and
the panel falls back to its minimum: on the operator's bundle a 2078px tree
gives the panel 460/374/333px at those three widths and takes 1396/1002/883px
for itself. `?all=1` at 24,124px is unchanged, and no width in either text
direction scrolls the page body.

**Admin listings name what they list.** `/admin/family` showed 331 rows
reading `(unnamed family, 1 children)` beside a UUID — a count, not a list. A
family entity from a conversion carries no `name` of its own, so its label is
now derived from the people in it:

| | Label |
|---|---|
| both partners | `Leonard Kasprzyk & Janina Kasprzyk — 2 children` |
| one partner | `Eugenia Oktawia Zboiński & [Unknown] — one child` |
| children only | `Florian Klicki and 3 siblings — parents not recorded` |
| nothing recorded | `Family with nobody recorded` |

An explicit `name` still wins where one exists. The children-only case names
the **eldest child** rather than saying "children of [unknown]", because every
such family would otherwise carry identical words — which is the failure being
fixed, not a smaller version of it — and a child's name is the one thing an
administrator can search for.

`1 children` is gone: counts go through Fluent's plural rules like everything
else, so the same label is correct in Polish's three forms and Arabic's six.

The same treatment reached the other listings whose summary named no people:

- **event** — was `marriage — 24 August 1991` for all 141 rows; now
  `Marriage — Adam Filip Pendzych & Katarzyna Elżbieta Grzybowski, 1820`.
- **occupation** — was the title alone, and titles repeat across a family;
  now `Adam Bochenek — lekarz, anatom, histolog, antropolog`.
- **link** — was `godfather of → godchild of`, naming the relationship twice
  and neither of the people; now `godfather of: Leonard → Janina`.
- **source** — dropped the `(unknown)` that followed every title, and the
  reliability now reads as words rather than an enum.
- **document** — `document_type` reads as words, and a file with no filename
  falls back to its caption before its id.

**person** and **place** already carried a real name and were left alone.

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

**Eleven interface languages, translating the interface and never the data.**
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

### The foundations

What the first weeks built, before accounts, translation or the reframing
above. Where the two disagree, above wins: the completeness readout no longer
links a specification and the home page no longer argues for a file format.

#### Added

**Record rendering.** Every page surfaces at least one thing most family trees
flatten away.

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
what the record could say. The framing is plain — an imported tree shows five
empty rows because the file it came from had nowhere to put any of it, not
because the import lost anything — and a tree that already carries rich detail
is told so instead. (The specification links this originally carried were
removed; see the reframing above.)

**Pages.** Home; `/tree`; `/person/:id`; `/convert`; `/health`; and a plain
server-rendered admin panel with per-kind forms, paginated listings, validate,
deduplicate and export. (Home originally opened with a "Why AXGF" panel; see
the reframing above for what replaced it.)

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
so the archive can be read without this site if it ever has to be.

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

#### Security

- Authentication began as a single shared token in an `HttpOnly`,
  `SameSite=Lax` cookie, compared in constant time, with a blank token
  rejected in two places so an empty `AXGF_CMS_ADMIN_TOKEN` could never become
  "no password required". Named accounts in the `.acl` replaced it; the token
  survives only as the emergency door described above.
- The server binds to `127.0.0.1` by default, because an admin surface must
  not reach the network by accident.

#### Notes

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
- **A database.** There is no PostgreSQL backend and no SQL of any kind. The
  archive is one `.axgf` file read into memory at startup and written back
  atomically; the accounts live beside it in a `.acl`, and the payload cache is
  derived from the archive and can be deleted at any time. That suits one
  family's tree — hundreds to low thousands of people — and it is why the whole
  state is two files an operator can copy. It is also the ceiling: an archive
  past roughly a gigabyte starts costing real memory, and the admin panel says
  so on the dashboard rather than waiting for someone to find out.
- **Multi-tenancy.** One process serves one family's archive. Two families mean
  two processes, two bundles and two ports — which is a deployment decision, not
  a feature to build. Nothing in the state model is keyed by a tenant, and
  retrofitting one would touch every read path, so it is better done as a
  different product than bolted on here.
- **Graph queries.** No relationship solver, no "how is X related to Y", no
  common-ancestor or path search, no cousin-degree calculation. The tree view
  walks ancestors and descendants to a fixed depth and stops there. This is the
  omission most likely to be mistaken for an oversight: it is genuinely useful,
  it is genuinely absent, and it wants a considered answer to what a
  relationship *is* when parentage carries confidence — a path through three
  links at 0.4 is not a fact about a family, and reporting it as one would be
  worse than not answering.

[0.1.0]: https://github.com/plkarin/axgf-cms/releases/tag/v0.1.0
