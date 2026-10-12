# Proposal — a text edit should not rewrite the media

**Status:** proposed, not started · **After:** v0.4.0 · **Touches:** `src/state.rs`
(`persist`, `write_streaming`), `src/payloads.rs`, `src/backup.rs`, axgf-rs

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

## Options

### A. Append to the archive, compact now and then — recommended

A ZIP's index (the central directory) sits at the end of the file, and
readers find it from the end. A save would:

- **append** the changed JSON entries;
- then append a **new central directory** that points at the new entries
  and at the untouched media where it already is;
- then sync.

The old entries become dead bytes in the middle of the file.

- **Cost:** about 0.6 MB of JSON at most (usually a few KB) plus a central
  directory of about 150 KB. That is milliseconds plus one fsync, against 12–17 s.
- **Crash safety:** until the new end-of-directory record is written and
  synced, the last complete one in the file is the old one, and its offsets
  are still valid. A crash mid-append leaves the previous archive readable.
  This replaces "temp file, then rename" with "append, sync, then write the
  record that makes it live". The durability tests have to prove it the way
  they prove the rename today: kill mid-write, reopen, compare.
- **Growth:** every save leaves its old JSON behind. **Compaction** is
  today's full rewrite, run when the dead bytes exceed a threshold (say 25 %
  of the live JSON, or 50 MB). It also runs before every daily backup, so
  backups stay compact and portable. A compaction costs what a save costs
  now, but only occasionally, and never while someone waits on a form.
- **Still one file**, still a valid ZIP, and any AXGF reader sees only the
  live entries.
- **Needs from axgf-rs** (0.6): `append_entries(path, changed) -> Envelope`.
  It writes only the changed members and a new directory, validates what it
  writes as `export_bundle_streaming` does, and refuses an archive it did not
  produce. Writing ZIP structures by hand in this crate would break rule 3.

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

## Recommendation

**A**, in four steps:

1. **axgf-rs 0.6:** `append_entries` and a `dead_bytes(path)` report, with
   the library's own tests: an appended archive reads back identical to a
   rewritten one, and a truncated append leaves the previous archive
   readable.
2. **axgf-cms:** `persist` appends when the change touches no payload and
   rewrites when it does (an upload, a document delete, a dedup that removes
   media). It compacts past the threshold and always before the daily
   backup.
3. **Proof:** the durability suite gains kill-mid-append tests. The
   dismissal measurement is repeated on a copy of the operator's archive in
   a release build, with the target under one second for a single
   dismissal, and the archive is shown to open in the `axgf` CLI after 1,000
   appended saves and after a compaction.
4. **Release** through the runbook, rollback rehearsal included. An older
   binary reading an appended archive must also be tested: it reads the last
   directory, so it should work, and that test is what says so.

## Not part of this

Media uploads still rewrite. They are rare, and the reader expects an upload
to take a moment. Moving media saves to appends too would make dead space
grow by megabytes per save, and compaction would then dominate.
