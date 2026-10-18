# ADR 0005 — An installation sets itself up: three doors into a family

Status: accepted, v0.5.0 (2026-10-18).

## Context

The operator asked for a CMS that manages its own beginning. A family arrives
in one of three ways, and each has to work from the first start of a new
installation:

- with an existing **`.axgf`** (another installation's, or a backup's);
- with **nothing**, to be written person by person;
- with a **GEDCOM** of whatever version its old software wrote. This is the
  catch-up path for a family coming from another program, uploaded during
  the first set-up.

Before this decision:

- **Nothing** worked, silently. With no bundle at startup, `AppState::load`
  creates an empty one, and an administrator can add the first person. No
  screen said so, and the overview told *every* visitor to "import a family
  file", which a visitor has no right to do.
- **An `.axgf`** could only be set by pointing `AXGF_CMS_BUNDLE` at it before
  startup, or with `axgf-cms restore` of a backup archive, both on the server.
- **A GEDCOM** went through `/convert`, which produces an `.axgf` *to
  download* and never touches the served bundle. Nothing set an installation
  up from one.

The converter, `axgf-rs` 0.5.0, reads GEDCOM 5.5.1. One small family was
written seven ways (`tests/fixtures/gedcom/`, synthetic people) and
converted:

| file | before |
|---|---|
| 5.5.1, UTF-8 | whole |
| 5.5, ANSEL | every accent wrong: "Józef Gęsiorek" became "Jâozef Gñesiorek" |
| 5.5.5, UTF-16 without a byte-order mark | refused |
| 5.5.1, old Macintosh line endings (CR) | converted with nobody in it |
| 5.5.1, Windows-1252 | its typographic apostrophe became a control character |
| 7.0, UTF-8 | its `@VOID@` placeholder counted as a child; its shared note (`SNOTE`) lost |
| 7.0, `.gdz` | refused |

## Decision

### 1. An installation knows whether it has a family

It is **unset** while its bundle holds no entity in any collection
(`AppState::is_unset`). That is what a first start without a bundle
produces, and the only state in which the set-up is offered.

### 2. Three doors, to administrators, at `/admin/setup`

1. **An AXGF archive.** It is streamed to a file beside the bundle, never
   held in memory, up to 4 GB. It is then taken in exactly as the server
   takes its own bundle at startup: each photograph goes into the disk
   cache through a fixed buffer, the bundle is written atomically, and the
   result is swapped in under the write lock (`AppState::adopt_archive`).
   The space it needs (three times its size) is checked before a byte is
   written.
2. **Nothing.** The empty archive stays, and the door is the ordinary
   editor's first person.
3. **A GEDCOM** of any version (§3). It is read, converted, and shown first:
   - how it was read: version, encoding, container, and every repair;
   - what came across, and what the quality screen would flag;
   - what the converter did not recognise.

   Only a second press keeps it (`AppState::adopt`).

The page closes as soon as a family is there. The overview and the dashboard
show an administrator of an unset installation the way in. A visitor is told
the archive is being set up, and is no longer invited to import a file.

### 3. Any GEDCOM, read into the dialect the converter reads

`src/gedcom_in.rs` sits in front of the converter, for this door and for
`/convert` alike:

- **container**: a `.ged`, or GEDCOM 7's `.gdz`, a zip whose dataset is
  `gedcom.ged` (or its only `.ged`), unpacked within the upload limit;
- **encoding**:
  - a byte-order mark when there is one;
  - UTF-16 without one, recognised by where the zero bytes fall;
  - otherwise bytes that are valid UTF-8 are read as UTF-8, whatever the
    header says;
  - the rest as the header's `CHAR` declares: ANSEL (decoded here, its
    accents moved after their letter and composed onto it), Macintosh, or
    Windows-1252 for anything else;
- **line endings**: CR, CRLF and LF all become LF;
- **GEDCOM 7**: a pointer to `@VOID@` is dropped with its substructure, and
  `SNOTE` is read as `NOTE`.

What it did is reported on the page. The normaliser lives in the CMS
because the library is a published crate; it belongs upstream, in
`axgf-rs`, and should move there.

### 4. Nothing is ever written over a family

Every door checks for a family again under the write lock, at the moment of
writing. With one there, it is refused (409) and nothing is written.
Replacing a family that exists stays `axgf-cms restore` from a backup, on
the command line, deliberately: the act that destroys a family's archive is
not one a page offers.

### 5. Provenance

The journal records a `setup` entry: who did it and how ("GEDCOM ·
tree.axgf", "AXGF · family.axgf"). The accounts (`.acl`) are rebound to the
new family, so the next start does not warn that one family's accounts are
being applied to another's tree.

## Consequences and limits

- A `.gdz`'s media are counted and reported, not carried: they do not
  become documents yet.
- Some GEDCOM 7 structures are dropped with the converter's usual
  "unrecognised tag" warnings, which the page lists:
  - besides `@VOID@` and `SNOTE`: `EXID`, `PHRASE`, `SDATE`, `TRAN`, `NO`;
  - the extension tags `SCHMA` declares.
- The GEDCOM upload limit is the conversion page's: 10 MB. A 767-person
  file is about 320 KB.
- A GEDCOM is never *merged* into a family that exists. Deduplicating two
  trees is a separate task, and not a door.

## How this was verified

- `src/gedcom_in.rs`, 10 unit tests:
  - byte-order marks, and UTF-16 LE and BE with and without one;
  - ANSEL accents composed, and an accent at a line's end left behind;
  - Windows-1252 and Mac Roman, and CR line endings;
  - GEDCOM 7's `@VOID@` and `SNOTE`, and `HEAD.GEDC.VERS` (not the
    software's `VERS`);
  - `.gdz` with media and size limits, and refusals.
- `tests/setup.rs`:
  - the doors are offered only to an administrator, and only while unset;
  - every one of the seven files sets a family up with its names whole,
    and the 7.0 files with one child, not two, and the shared note on its
    person;
  - an archive sets a family up with its photograph served byte for byte;
  - a file that is not an archive changes nothing;
  - no door writes over a family.
- By hand, on fresh servers started without a bundle:
  - the ANSEL file through the third door, read back on the river with
    every accent;
  - an archive with a photograph through the first: thumbnail generated,
    door closed after;
  - a restart: the adopted bundle reloads clean, with no account warning,
    and the stale cache generation is swept.
