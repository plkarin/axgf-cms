# GEDCOM import under stress

The campaign of 22 October 2026: every kind of GEDCOM file a family holds,
through the CMS's reading (`src/gedcom_in.rs`), the conversion of axgf-rs
0.5.0 (`axgf_rs::convert_gedcom`), the validator, the export, the import back,
and every page that serves the family it makes. What it found, what was
mended here, and what is the library's to mend.

Every file in it is invented: the corpus is generated
(`scripts/gedcom-corpus.py`), and its families, places and notes are fiction.

## How

- **A corpus** of 72 files, each aimed at one thing a family's file can be
  (`scripts/gedcom-corpus.py OUT`; `--large` adds five more of 20,000 to
  40,000 people and a line of 20,000 generations): GEDCOM 4.0 to 7.0, bare and
  zipped; ASCII, ANSEL, Windows-1252, Mac Roman, IBM PC, UTF-8 and UTF-16 with
  and without a mark; every line ending; indented, tab-separated, double-
  spaced, lower-case; 79 forms of date; names in Latin, Greek, Cyrillic,
  Arabic, Hebrew and Chinese script, with emoji and joiners; every event
  and attribute; lineage, cycles, a child of itself, 3,000 children, 500
  partners; dangling pointers and pointers to the wrong kind of record;
  markup and hostile file paths; a zip bomb; lines nested 50,000 deep.
- **A probe** (`tests/gedcom_probe.rs`, ignored by the suite) that takes each
  file, in its own process so that a crash or a hang is a finding, through
  `/convert/gedcom`, the reading, the conversion, the validator, the export,
  the import back, the data-quality and completeness analyses, and then
  serves the family and opens its pages — the river, the tree, the search,
  the quality screen, the console, and each person's record, panel and
  editor.
- **The tests** that hold what it found, in the suite:
  `tests/gedcom_reading.rs` (23: versions, encodings, line endings, layout,
  depth, refusals), `tests/gedcom_conversion.rs` (9 and one in release:
  the conversion page, markup, the findings list, the server's answers
  during a conversion, every odd family served, documents never served, the
  largest file the upload takes), and `tests/gedcom_known_defects.rs` (31,
  ignored: the converter's defects below, each stated as what a faithful
  conversion does).

## What held

No file crashed the conversion page once the depth floor below was in
place; none hung it. Every encoding the CMS read before was read the same.
Markup in a name, a note, a place, a source's title or the upload's own file
name was shown as text everywhere it appeared. A document path that leaves
the server's directory (`../../../../etc/passwd`, `/etc/hostname`,
`file://…`) is a reference the site never serves: asked for, it says there
is no such file. A `.gdz` that would unpack past the limit was refused in
44 ms. A cycle, a person who is their own child, a family of 3,000 children
and a line of 1,000 generations were all served without an error.

## Mended in the CMS

Each of these was in the layer the CMS owns, the reading of a file into the
dialect the converter reads, or in the conversion page.

| What happened | Now |
|---|---|
| **A file of 300 KB whose lines each go one level deeper crashed the server.** The converter's cost grows with the square of the depth and its records drop recursively: at 50,000 levels the stack overflowed and the process was aborted — through `/convert`, which needs no account. | Lines deeper than 32 levels (GEDCOM's deepest structure is seven) are set aside with what they hold, and the page says how many. The file converts in 50 ms. |
| **Files with two spaces after the level, tabs between the fields, or lower-case tags came out with nobody in them**; indented files kept their people and lost their families, events and places. | Indentation, tabs and runs of spaces between the level, the cross-reference and the tag are mended, and a standard tag in lower case set upright. Each comes out as the same family as the reference. |
| **A UTF-8 file with one stray byte** (a note pasted from a Windows program) was read entirely as Windows-1252: every accent in the file became "Ã©". | The file stays UTF-8; that byte alone is read as Windows-1252, and the page says how many there were. |
| **IBM PC (DOS) files** were read as Windows-1252: "Fran‡ois", "H‚lŠne", and a control character in "M\x81ller". | Read as code page 850, whose letters are code page 437's and more. |
| **A note split after a space** (`… coupée ` / `CONC au milieu`) lost the space: "coupéeau milieu". | The space is moved to the head of the continuation, where the converter keeps it. |
| A line of prose before the header ("Exported by …") made the file "not GEDCOM"; UTF-16 without a mark was missed after a blank line. | Passed over, and found by where its zero bytes fall. |
| `@@` stayed `@@`; control characters (NUL, bell, escape) reached names and pages; `SEX m`, `Male`, `FEMALE` became unknown; a date marked `@#DGREGORIAN@` was not read. | `@@` is `@`, controls are taken out, the sex is its letter, the Gregorian mark is dropped. |
| **A messy file's result page listed every data-quality finding**: 32,000 of them for 20,000 people, a 4.7 MB page. | The count is whole; the first 200 are listed, and the data-quality page lists every one once the file is served. |
| **A conversion held a thread that answers requests**: 7.5 s for 40,000 people, so two uploads at once held both threads of a two-core server, and nobody was answered for as long. | The reading, conversion, analyses and export run on the blocking pool; the server answers throughout. |

## The library's to mend: axgf-rs 0.5.0

Not worked around here: each is a reading of what GEDCOM means, and the
conversion is the library's (the same rule as `DEDUP-UPSTREAM.md`). Each has
a test in `tests/gedcom_known_defects.rs`, ignored with its reason, to be
unignored when a release mends it; `cargo test --test gedcom_known_defects
-- --ignored` shows where the library stands. All 31 fail today.

**Data dropped without a word.** The converter carries BIRT, DEAT, OCCU, the
first MARR, NAME, SEX, NOTE, TITL, FACT and OBJE, and warns only about a
record type it does not know, an empty occupation and an empty family. Everything else under a person or a family is
dropped silently, while the conversion page presents the warnings it does
list as what could not be brought across:

- christenings, baptisms, burials, cremations, residences, censuses,
  emigrations, immigrations, naturalisations, education, religion,
  nationality, descriptions, generic EVEN with their TYPE — and a second
  BIRT or MARR;
- citations under an event (`SOUR @S1@`, `PAGE`, `QUAY`), and a source's
  AUTH, PUBL, TEXT and REPO;
- a death's cause and age; a divorce, an engagement, a family's own events;
- a place's coordinates (`MAP`, `LATI`, `LONG`);
- names written as their pieces (`GIVN`, `SURN`) — the person becomes
  "[Unknown]" — and a name's TYPE, NICK, NPFX, NSFX, SPFX;
- adoption and fostering where GEDCOM 5.5.1 records them, on the child's
  `FAMC` (`PEDI`); a child named only in its own `FAMC`;
- a person with no cross-reference, and the first of two records sharing
  one.

**Dates.** `ABT 12 MAR 1850` is given the precision of a year; `CAL`, `EST`,
`FROM … TO …`, `INT`, dual years (`1750/51`) and every calendar escape
(Julian, French Republican, Hebrew) are kept only as a note, though AXGF has
calendars for all three; `31 FEB 1900`, `0 JAN` and `32 JAN` are written as
exact dates; a year past 9999 or below zero is written as a value the
schema refuses; an occupation's `FROM 1868 TO 1880` has no end.

**Shapes the schema refuses or the reader would not expect.** A name with a
suffix (`Jean /Dupont/ Jr.`) gets a `suffix` component, which the schema
does not have (`SCHEMA_VALIDATION_FAILED`); an unclosed or doubled `/surname/`
leaves slashes inside components; `SEX X` (GEDCOM 7) becomes unknown; a
child listed twice is a child twice; a photograph referenced twice is
attached twice; a Windows path keeps its backslashes in the file's name; an
empty `PLAC` makes a place with an empty name; one town written three ways
(`Paris, France`, `Paris,France`, `paris, france`) is three places; a note
whose text is `@home@` is taken for a pointer and lost; a family whose HUSB
is a family and whose CHIL is a source links them as people, and the
validator accepts it.

## At scale

On the operator's two-core server, in release:

| family | file | conversion page | loading the archive | pages |
|---|---|---|---|---|
| 5,000 people, as a real family reads | 1.3 MB | 1.3 s | 0.5 s | every page under 1.6 s |
| 40,000 people, as a real family reads | 9.8 MB (the limit is 10) | 7.5 s: reading 0.4, conversion 2.2, export 4.8 | 3.3 s | home and river 9–13 s, search 13–14 s, console 11 s, the whole tree 54–95 s |
| 20,000 generations in one line | 3.0 MB | 3.4 s | — | the river answers; the whole tree did not answer within the probe's four minutes |

The conversion keeps up with the upload limit. Serving a family past ten
thousand people is slow, page by page; no family here is near it (the
operator's has under a thousand people), and it is a matter for the pages,
not the import.

## Running it again

```sh
python3 scripts/gedcom-corpus.py CORPUS            # add --large for the slow ones
PROBE_DIR=CORPUS PROBE_OUT=REPORTS \
  cargo test --release --test gedcom_probe -- --ignored --exact probe_all --nocapture
cargo test --release --test gedcom_conversion -- --ignored   # the upload limit
cargo test --test gedcom_known_defects -- --ignored          # the library's state
```
