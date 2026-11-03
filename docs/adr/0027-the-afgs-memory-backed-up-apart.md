# ADR 0027 — The AFG's memory, backed up apart

**Status:** proposed (2026-11-03) · **Builds on:** docs/adr/0024 (the AFG's
brains), docs/adr/0026 (My personal assistant) · **Amends:** docs/adr/0024
and docs/adr/0026, which left the AFG's memory out of every backup ·
**Applies to:** `src/memory_backup.rs`, `src/backup.rs`, `src/main.rs`,
`src/config.rs`, `src/host.rs`, `src/lib.rs`, `tests/backup_restore.rs`

## Context

The AFG keeps two folders beside the bundle:

- `family.brains/`: each account's brain (docs/adr/0024);
- `family.assistant/`: each account's personal assistant, with its projects,
  conversations, files and second brains (docs/adr/0026).

Both ADRs left them out of the nightly backup and left the choice to the
operator. On 3 November 2026 the operator decided: *the brains must be backed
up, as a backup of ax-genealogy that is not the backup of the AXGF bundle.*

## Decision

### 1. An archive of its own

The nightly run, `axgf-cms backup`, writes the bundle's archive as before:
the bundle, the accounts and the journal, byte for byte what it was. Then it
writes a second archive, **`axgf-memory-<stamp>.zip`**, in the same
directory:

- **What it holds:** the two folders, every file under them, under their own
  names (`family.brains/<account>/memory.md`,
  `family.assistant/<account>/projects/…`).
- **What it leaves out:** a file still being written (`*.tmp`), and any
  symbolic link (the application writes none).
- **When there is nothing:** neither folder exists yet, so no archive is
  written.
- **Its manifest, `memory.json`,** gives:
  - the folders it holds, and how many accounts each holds;
  - the SHA-256 and the size of every file;
  - when it was taken, by which version, and beside which bundle.
- **Telling the two apart:** each archive's manifest says which kind it is
  (`memory.json` against `backup.json`), so a renamed archive is still known
  for what it is.
- **No pre-upgrade copy:** an archive taken before an upgrade stays the
  bundle's alone.

### 2. The same discipline as the bundle's

Every step mirrors the bundle's archive:

- **Writing:** the archive is written as a `.part` claimed with `flock`, mode
  600 from the moment it exists.
- **Reading back:** it is read back before it is given its name, and one that
  fails is deleted. A file passes when:
  - every member's CRC holds;
  - every file's SHA-256 and size are the ones the manifest recorded;
  - every path stays inside a folder named `*.brains` or `*.assistant`;
  - nothing in the archive is missing from the manifest.
- **Interruptions:** a `.part` left by an interrupted run is reclaimed by the
  next.
- **Retention:** the same selection keeps it (7 daily, 4 weekly, 12 monthly),
  as a set of its own. Neither set ever counts against the other.
- **The weekly check** reads the newest archive of the memory back too, and
  fails its unit if it cannot.
- **Plain ZIP:** `unzip` reads it without this program.

### 3. Taken while the service runs

The service writes these folders while the backup reads them, and needs no
lock for it:

- **Rewritten files:** a brain's memory, a project's card or a picture is
  written to a temporary file and renamed into place. The archive therefore
  holds the old version or the new one, never half of each.
- **Appended files:** a conversation's lines are added at the end. One being
  written at that very second may be cut, and the application already skips
  a line it cannot read.

### 4. Restoring it

`axgf-cms restore <archive>` recognises an archive of the memory by its
manifest and puts the memory back instead of the bundle:

1. it refuses while the service is running against the bundle;
2. it verifies the archive in full;
3. it extracts the archive beside the bundle, checking each file again;
4. it moves the folders that are there now aside, into
   `axgf-memory-replaced-<stamp>/`, which it never deletes;
5. it renames the restored folders into place.

- **What it replaces:** only the folders the archive holds. An archive taken
  before any assistant existed leaves today's assistants as they are.
- **What it refuses:** a folder in the manifest that is not a `*.brains` or
  `*.assistant` folder. An altered archive can never write over the bundle or
  the accounts.

## Consequences

- **The memory survives what the bundle's archive survives.** It is on the
  same disk, in the same directory, so whatever copies the backups off the
  machine (docs/OPERATOR.md) copies both.
- **Forgetting reaches the backups as they age out.** A fact an account
  forgets, or a project it deletes, stays in the archives that held it, for
  up to twelve months with the default retention. That is what a backup is;
  the operator can delete archives by hand when someone asks for it sooner.
- **It costs disk.** The assistants' files (up to 20 MB each) are kept
  again in each archive the retention keeps. The run says the archive's size,
  and `/health` warns when the disk runs low.
- **The archive is personal data** about living people. It is mode 600, in a
  directory that is mode 700, as the bundle's is.
- **The console's "newest archive"** stays the bundle's newest. Its count and
  total size include both sets.
