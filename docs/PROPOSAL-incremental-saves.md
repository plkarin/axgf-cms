# axgf-rs 0.6 — merging two records, and saves that do not rewrite the media

**Status:** design for review. No code is written, in either crate, until
this has been read. · **After:** v0.4.0 · **Touches:** axgf-rs (new
functions, no format change); in axgf-cms `src/state.rs` (`persist`), the
bulk bar, the journal, `src/backup.rs`, the weekly verifier

Two library features, designed together because they meet: a merge rewrites
references across many records, and every one of those records is a save.

- **Part 1, merging two records** — someone always enters the same
  grandmother twice. This is what a family hits first, so it comes first.
- **Part 2, incremental saves** — a one-line edit rewrites 435 MB today.

Both are additions to axgf-rs. Neither changes AXGF: 1.1.0 is frozen, and
everything below is written with structures 1.1 already has.

---

# Part 1 — Merging two records

## What exists today

The library merges people only inside `deduplicate`, which runs over the
whole bundle and merges only exact matches (same normalised name, same
birth and death years, no ancestor or sibling relation between them).
Nobody can say "these two records are the same person". Reading
`logic/dedup.rs` for this design turned up two things the new function must
not inherit:

1. **It chooses silently.** `merge_person_records` keeps the survivor's value
   for every field both records have and drops the other without a word.
   Deduplication only groups records whose *years* agree, so the full dates,
   the places and the sources can still differ — and the absorbed record's
   are lost, unreported.
2. **References are found by name**: every key ending in `_id` (except `id`)
   whose value is the absorbed id. Checked against the 1.1 schema, this is
   right: all 53 reference fields are single `*_id` strings, none is a list
   of ids, and `entity_id` (links, event participants, document links) may
   point at any kind. One `_id` field is not a reference — `source.dna.kit_id`,
   an external kit number — and is safe because only a value equal to the
   absorbed record's UUID is ever rewritten. The new function keeps the rule.

## The API: plan, then merge

Stateless like the rest of the library, so in two calls — the first changes
nothing:

```text
plan_merge(flat_json, kind, keep_id, absorb_id)              -> Envelope
merge_entities(flat_json, kind, keep_id, absorb_id, resolutions_json) -> Envelope
```

`plan_merge` returns, without touching the bundle:

- `agreements` — fields the two records hold alike, or that only one holds;
- `conflicts` — each field both hold differently, with both values, their
  sources and confidences, the choices allowed, and a suggestion;
- `effects` — how many references will be rewritten, per kind; links that
  would point from a person to themselves; families that will describe the
  same couple afterwards;
- `refusals` — reasons the merge cannot be done at all (below);
- `fingerprint` — a hash of the two records and of every record that refers
  to the absorbed one.

`merge_entities` applies it. It refuses, changing nothing, when a conflict
has no resolution (`MERGE_UNRESOLVED`), when any refusal holds, or when the
fingerprint no longer matches because someone edited one of those records
in between (`STALE_PLAN` — axgf-cms shows that the way it shows a concurrent
edit today).

**Kinds in 0.6:** person, family, place, source — the grandmother, the
couple recorded twice, the village spelt two ways, the register cited
twice. Events, occupations, links and documents are not merged: in practice
they are not entered twice, and a duplicate one is deleted.

## Nothing is chosen silently, and nothing is lost

The rule for every field: a value only one record holds is taken; values
both hold alike are kept once; values that differ are a **conflict**, which
a person resolves. Where AXGF has room for both values, the one not chosen
is kept, in a structure 1.1 already defines.

| field | when both hold it, differently |
|---|---|
| names (`identity.names`) | not a conflict: the lists are joined, duplicates dropped; the absorbed display name joins as an `alias` with its source |
| display name | conflict: keep / absorb |
| gender, living | conflict: keep / absorb |
| visibility | conflict: keep / absorb, the more restrictive suggested |
| birth, death | conflict: keep / absorb. The value not chosen becomes an **event** (category `birth` or `death`) with the kept person as participant, its own date, place, source and confidence. When both values cite a source, the chosen source gains a **`source.conflicts`** entry — `{source_id, field: "birth.date", this_value, conflict_value, resolution: "this_preferred"}`: the spec's own record of two sources disagreeing |
| a profile attribute recorded as a series (1.1 claims) | not a conflict: the claims are joined, identical ones dropped |
| a profile attribute holding one claim | conflict: keep / absorb |
| notes, biography (free text) | conflict: keep / absorb / both, *both* suggested (joined, each under a line naming its record) |
| a field this library does not know (forward compatibility) | conflict: keep / absorb, shown as raw JSON |

And whatever is chosen, the merge's report carries **the absorbed record
verbatim**. axgf-cms writes it into the journal entry that already records
`merged_into`, so even a value the structures above cannot hold survives,
readable, beside the archive.

## References, and merges that cannot be

Every `*_id` equal to the absorbed id becomes the kept id, in all eight
kinds. Then the duplicates this creates are tidied, and reported:

- a family listing the person twice — as two spouses, or as two children —
  keeps one entry, the one with the higher confidence;
- a link from the person to themselves is removed;
- an event naming the person twice as participant keeps one.

**Refused** — the result would be impossible, not merely doubtful:

- one is the other's ancestor (a father and son of one name — the commonest
  homonym; the merged person would be their own parent);
- in one family one is a spouse and the other a child;
- the two are the spouses of one union (a self-marriage).

**Allowed, with a warning in the plan:** two children of the same parents.
`deduplicate` never merges these, because same-name siblings are real
(a first child who died, and the next one given the name). But the same
child entered twice in one family is also real — it is what a GEDCOM import
does — and only a person can tell the two apart. The plan says "these are
recorded as siblings", and the person decides.

**Families that now describe the same couple** are listed in the plan and
not merged on the side: one merge is one decision. axgf-cms offers the
family merge next, through the same two calls.

## Places and sources

The same plan and merge, with their own fields: a place's names are joined
by language, its coordinates are a conflict, its history (the jurisdictions
over time) is joined; a source's citations and `conflicts` are joined, its
title and type are conflicts.

## The report

`merge_entities` returns the new bundle and a report: kept and absorbed ids,
the absorbed record verbatim, each resolution applied, references rewritten
per kind, events created, `source.conflicts` entries added, links removed,
duplicate entries tidied, families now describing one couple. It replaces
the guesswork the dedup route in axgf-cms does today to find the survivor.

## In axgf-cms, once the library has it

- **The bulk bar gains "Merge"** — with exactly two people (or two places,
  two sources) selected; it is not drawn otherwise. The bar goes from two
  controls to three; the list screen is at 12 of its 20.
- **A review screen** before anything is written: the two records side by
  side; what they agree on, as a count; each conflict as a choice between
  the two values, the suggestion preselected; one primary button. Refusals
  are said in a sentence, with no button.
- The journal entry keeps the absorbed record; the old address already says
  where the person went (B7).
- A quality detector, "possibly the same person" (one name, overlapping
  dates), links to the review. Detectors never merge.

## Library tests

The sample's duplicates merged by plan and resolution; a birth conflict kept
as an event and a `source.conflicts` entry, and the result valid against the
1.1 schema; each refusal; a stale plan refused with the bundle unchanged;
every reference to the absorbed id gone, in all eight kinds; the report's
absorbed record equal to the input.

---

# Part 2 — Saves that do not rewrite the media

## The problem, measured

Every save rewrites the whole archive. On the operator's archive (release
build, 2026-10-11):

| what | time |
|---|---|
| one dismissal on `/quality` (one person record) | 12–17 s |
| marking all 97 places of one rule, one save | 20 s |
| the same 97, one at a time | about 23 min |

What the archive holds (`family.axgf`, 435 MB, 1,967 entries):

| part | entries | raw | stored |
|---|---:|---:|---:|
| `documents/` (scans, photographs, PDFs) | 408 | 440.8 MB | 434.6 MB |
| persons, families, events, places, … , manifest, schema | 1,559 | about 1.5 MB | about 0.6 MB |

So 99.9 % of each save is copying media that did not change. Most of it is
already-compressed images, so the copy is I/O-bound rather than CPU-bound.
`payloads.rs` already keeps it out of memory: export streams each file from
the cache into the ZIP through a 64 KiB buffer. The cost is the bytes, not
the RAM.

## First: is an appended archive still AXGF? Yes — tested

AXGF 1.1.0 is frozen. If an archive with appended segments could not be read
unchanged, appending would be a format change, not an implementation
detail, and it would stop here. So it was tested first, on a copy of the
operator's archive (2026-10-12).

**How.** The copy was left byte for byte as it was, and three things were
appended to it:

- one person's JSON entry, with its display name changed (`… APPENDED`);
- a new central directory: 1,967 records, 1,966 of them pointing at the
  original entries where they already were, and one pointing at the new
  entry;
- a new end-of-central-directory record.

The old entry, the old central directory and the old end record were left
in the middle of the file. 190,624 bytes were appended and 190,611 became
dead. No zip64 was involved: the archive is under 4 GiB.

**Read by, unchanged:**

| reader | result |
|---|---|
| Info-ZIP `unzip -t` | "No errors detected in compressed data" |
| `zipinfo -t` | 1,967 files, as before |
| `unzip -p` of the changed entry | the new name |
| Python `zipfile` | every CRC checks; the new name; the other 1,966 entries identical |
| axgf-rs 0.5.0 `import_bundle` | `Ok`, no diagnostics, the new name |
| axgf-rs 0.5.0 `import_bundle_streaming` (what axgf-cms uses) | `Ok`, all 408 payloads, 440,560,338 bytes, the same as the original; the new name |
| axgf-rs 0.5.0 `validate` | `Ok`, no errors |
| axgf-cms 0.3.1, the production release | "loaded and valid, 1963 entities", as the original; search finds the new name |

**Why it works.** A ZIP's central directory is authoritative, and a reader
finds it from the end-of-central-directory record, which it searches for
from the end of the file. axgf-rs reads through `zip` 0.6.6's
`ZipArchive`, which does exactly that, and so do the standard tools. The
AXGF spec (1.0 §2, unchanged in 1.1) says a bundle "is a ZIP archive" with
the listed members, and nothing about the order of entries or bytes that
nothing refers to.

**So: not a format change.** An appended archive is a valid ZIP and a valid
AXGF 1.1 bundle, read unchanged by axgf-rs 0.5.0 and by standard tools.

**One kind of reader it does not serve.** A pure stream reader walks the
local headers from the start of the file and never reads the central
directory (Java's `ZipInputStream`, `unzip` reading from a pipe). It would
see the superseded entry first and stop at the old directory in the middle,
so it would read the *previous* state, silently. Nothing in this project
reads that way, but a family member's tool might. That is why compaction
runs before every daily backup and every export or download (Durability, below):
**no archive that leaves the server is ever appended.** Only the working
copy on the server is.

## What must not change

These rules are established elsewhere, and a fix that breaks one is not a
fix:

1. **One file the family owns.** The about page promises it, and an export
   or a backup is that file, readable by any AXGF tool.
2. **A save is all or nothing.** It is written to a temporary file, synced
   and renamed today; after a crash the archive is either the old one or the
   new one.
3. **Everything goes through axgf-rs.** This crate does not write AXGF
   itself.
4. **Backups, the weekly verification and the rollback rehearsal keep
   working unchanged**, or change with their tests in the same step.

## The design, corrected

The sketch in this document's first version was wrong in three places,
found while designing it against the library and the `zip` crate:

1. **The library takes no paths.** Its contract (crate docs, rule 3) is
   that the caller passes bytes and the library never touches the
   filesystem. So not `append_entries(path, …)`: the library is handed an
   open archive, `Read + Write + Seek`, as `export_bundle_streaming` is
   handed a writer.
2. **`zip`'s own append mode cannot be used.** `ZipWriter::new_append`
   (zip 0.6.6, `write.rs`) seeks to the start of the existing central
   directory and *overwrites it* — a crash during the write leaves no valid
   directory anywhere — and it keeps the superseded entries, so the new
   directory would list two members of one name. The library writes the
   appended part itself: the changed members' local entries, a new central
   directory, a new end record. The directory records of the untouched
   members, payloads included, are copied **byte for byte** from the old
   directory: their entries do not move, so their records stay true.
3. **A torn append is not "the previous archive, still readable".** Readers
   find the end record by scanning back over at most the last 65,557 bytes
   (the longest possible ZIP comment). The new central directory alone is
   about 150 KB, so after a crash between it and the new end record, the old
   end record lies outside every reader's search: the file opens nowhere.
   The previous archive is still in the file, byte for byte, at its old
   length — so the rollback is to **truncate to that length**, and that has
   to be explicit (below).

## Library API

```text
append_changes(archive: Read + Write + Seek, flat_json) -> Envelope
inspect_archive(archive: Read + Seek)                    -> Envelope
```

`append_changes` reads the central directory, reads and inflates the JSON
members (about 1.5 MB on the operator's archive), serialises the flat bundle
exactly as the export does, and compares member by member. It appends only
what differs, writes a directory without the members that are gone, and
returns `{appended, removed, start_len, end_len, dead_bytes}`. It validates
first, as the export does, and it does not sync — the caller owns the file.

It works out the changes itself rather than being told: a caller cannot
forget a record, and a save that changed nothing writes nothing.

It refuses with `NEEDS_REWRITE`, writing nothing, when:

- the payloads changed — a document added, removed or replaced: media
  saves keep rewriting, as before;
- the archive is not one it can append to — zip64, a comment, encryption,
  more than one disk, data descriptors, or a member outside the AXGF layout;
- the result would need zip64 (over 4 GiB, or over 65,535 members).

`inspect_archive` reports live and dead bytes and whether `append_changes`
would accept the archive; axgf-cms uses it to decide when to compact.

## Durability, in axgf-cms

Under the write lock that saves and backups already share:

1. write the archive's current length to a small sidecar
   (`family.axgf.append`), and sync it;
2. `append_changes` on the archive, and sync it;
3. write the new length to the sidecar, and sync it.

At startup, a length recorded in step 1 and never confirmed in step 3 means
an append was interrupted: the archive is truncated to that length and
synced, which gives back the previous archive byte for byte. An archive
longer than the confirmed length is truncated to it.

**Compaction** is today's full rewrite (temp file, sync, rename), run when
dead bytes pass a threshold (25 % of the live JSON, or 50 MB), and always
before the daily backup and before any export or download — so no appended
archive ever leaves the server, and the pure stream readers described above
never meet one. The backup takes the write lock already. The weekly
verifier opens the archive read-only without it, and must not catch an
append half-written: either it takes the lock shared, or it reads only up to
the confirmed length (question 6).

## Tests

- **Library:** an appended archive reads back identical to a rewritten one;
  an append cut at every byte offset, then truncated to its start length,
  reads back identical to the archive before it; 1,000 appends then a
  compaction; `unzip -t` and Python's `zipfile` in the e2e suite.
- **axgf-cms:** the durability suite's kill-and-reopen tests, killing
  mid-append; the dismissal measured again on a copy of the operator's
  archive in a release build, with a target under one second; an older
  binary reading an appended archive.

## Order, and what I recommend

1. **axgf-rs 0.6.0: `plan_merge` and `merge_entities`.** No format change,
   no durability change: a merge is saved like any other edit. Then the
   bulk bar's Merge and the review screen in axgf-cms.
2. **axgf-rs 0.6.1: `append_changes` and `inspect_archive`.** It changes how
   the family's file is written, so it ships alone, with its crash tests and
   a rollback rehearsal of its own.
3. The rest of Part 2 in axgf-cms, released through the runbook.

Options B and C below were weighed against Part 2 and rejected; they are kept
for the record.

## Questions for you

1. **Kinds:** person, family, place and source in 0.6 — enough?
2. **Visibility conflicts:** preselect the more restrictive, or no
   preselection at all, so the choice is always made by hand?
3. **Notes and biographies:** suggest *both*, or require a choice?
4. **Two siblings:** allowed with a warning (as proposed), or refused like
   `deduplicate` does?
5. **Order:** merging first and appends a release later, as proposed, or
   both in one release?
6. **The weekly verifier:** a shared lock, or reading up to the confirmed
   length?

## Rejected alternatives to Part 2

### B. Keep the media beside the archive

`family.axgf` would hold only the text, and media would live in a
content-addressed directory (`family.media/<sha256>`). A save rewrites
about 0.6 MB.

- Fastest in steady state, and the simplest save path.
- **Breaks rule 1** unless every export, download and backup reassembles a
  single file. That puts the full cost back into every backup and makes
  "the file" a derived thing rather than the thing itself.
- Backup, restore, verification and the rollback rehearsal would all have to
  learn two locations. The pre-upgrade backup alone would need a format
  change.

### C. External payload references in the spec

AXGF 1.2 would let a document point at a payload outside the archive. This
is the most general option and the slowest to land: it is a spec change, a
library change and a migration, and every other reader has to follow. It
does not help the operator this year.

## Not part of this

Media uploads still rewrite. They are rare, and the reader expects an upload
to take a moment. Moving media saves to appends too would make dead space
grow by megabytes per save, and compaction would then dominate.
