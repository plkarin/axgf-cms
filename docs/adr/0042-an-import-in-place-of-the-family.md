# ADR 0042 — An import in place of the family, behind a word and a verified backup

**Status:** accepted (unreleased) · **Amends:** docs/adr/0005 §1, §2 (the
page that closes) and §4 (nothing written over a family) · **Applies to:**
`src/routes/setup.rs`, `src/state.rs` (`take_in`, `take_in_archive`,
`Over`, `generation`), `src/backup.rs` (`Kind::PreImport`),
`templates/admin_setup.html`, `templates/convert_result.html`,
`templates/admin_setup_result.html`, `templates/admin_bundle.html`,
`locales/*.ftl`

## Context

ADR 0005 gave an installation three doors into a family — an archive,
nothing, a GEDCOM — open only while it has none. Replacing a family that
exists was `axgf-cms restore`, on the command line, "deliberately: the act
that destroys a family's archive is not one a page offers".

The operator needs it from a page: their family is to be imported again,
whole, from the GEDCOM their old software exports, into an installation
that already holds it. They asked for it **protected**: if a family is
there, say that it will be overwritten and what will be lost, take an
automatic backup first and make sure it is good, and require a typed word
before anything happens — the word proposed being « remplacer l'archive ».

## Decision

1. **The doors stay open over a family**, to administrators only, in a
   *replace* mode. The archive and GEDCOM doors replace the family; the
   empty start is not offered (emptying an archive is not what was asked).
   The archive's room opens it from a closed disclosure, *Replace the family
   with an import*.
2. **The page says what will be lost**: everything not in the imported
   file — records, notes, pictures, links — with the family's counts, and
   where the backup will be written.
3. **A typed word**, in the page's language (`setup-replace-keyword`:
   « remplacer l'archive », "replace the archive", « zastąp archiwum »…),
   compared case, spaces and the apostrophe's shape aside. The archive's
   door asks for it beside the file; the GEDCOM's door reads and shows the
   file as before, and its *keep* button asks for it. Without it, or with
   other words, nothing is backed up and nothing is written (`400`).
4. **A verified backup first**: the same archive as `axgf-cms backup` —
   the bundle, the accounts and the journal, re-opened and checked as soon
   as it is written, deleted rather than kept if it does not read back —
   in a set of its own (`axgf-pre-import-…zip`, the three newest kept),
   in the operator's backup directory or `backups` beside the bundle. A
   backup that fails stops the replacement, and says why.
5. **No change lost between the backup and the replacement**: the bundle's
   generation is read before the backup, and the replacement, under the
   write lock, is refused if it moved (`Adoption::Changed`, `409`): a save
   made meanwhile is in the family still there, not lost.
6. **The replacement** is the first door's own taking-in (the archive
   streamed, its photographs into the cache, the bundle written atomically
   and swapped in), or the third door's (the converted bundle applied);
   the accounts are bound to the new family as at a set-up. The journal's
   `setup` entry names the backup (`GEDCOM · tree.ged → axgf-pre-import-…
   .zip`), and the page that ends it shows where it is.

## Consequences

- The operator can re-import a family from a page; a mistaken click cannot
  do it, and a mistaken word costs nothing.
- The family replaced is one `axgf-cms restore` away, from an archive
  proven to read back.
- An account bound to a person (`person_id`) keeps the id: if the new
  family has no such person, the account is bound to nobody until it is
  bound again.
- What ADR 0005 §4 said remains true of the set-up proper: an installation
  with no family is never written over by a race — two set-ups at once,
  the second finds a family (`Occupied`).
- Tests: `tests/setup.rs` (the page in replace mode; no words, other words:
  nothing written, no backup; the GEDCOM's keep button asking for them;
  the words, case and spaces aside: replaced, the backup named, listed in
  its set and verified), and `backup` (its own set).
