# ADR 0018 — A merge is planned, then decided by a person

**Status:** proposed (2026-10-26) · **Builds on:** axgf-rs 0.6.0's
`plan_merge` / `merge_entities`, and the journal's `merged_into` (B7) ·
**Applies to:** `src/routes/merge.rs`, `src/routes/admin.rs`,
`src/routes/mod.rs`, `src/render.rs`, `templates/admin_merge.html`,
`templates/admin_merge_done.html`, `templates/admin_list.html`,
`static/ui.js`, `static/app.css`, `locales/*.ftl`, `tests/merge_screen.rs`,
`tests/i18n.rs`

## Context

Someone always enters the same grandmother twice. Until now the CMS could
merge only what `deduplicate` merges on its own: exact matches, by name and
both years. The two records a person *knows* are one (a baptism and a
burial, a village spelt two ways, a register cited twice) stayed two.

axgf-rs 0.6.0 gives the means. `plan_merge` says what merging two persons,
families, places or sources would do, and changes nothing: what both hold
alike or only one holds, each field they disagree on with the choices it
allows and a suggestion, what else would change, and what is refused.
`merge_entities` does it, with every disagreement decided by the caller,
and returns the absorbed record verbatim.

## Decision

1. **The listing's bulk bar offers "Merge"** to an administrator, for the
   four kinds the library merges, enabled when exactly two rows are ticked
   (`ui.js`); without scripts the server refuses anything but two, with the
   reason. The first ticked is offered as the record kept; the plan page
   swaps them in one click.
2. **The plan page shows the library's plan and decides nothing itself.**
   Each disagreement is a set of radio buttons — the kept record's value,
   the other's, both where the library allows it — pre-set to the library's
   suggestion, with why (higher confidence, sourced, the more restrictive
   visibility, living…) and what the value not chosen becomes (another
   name, an event). The agreements and the other effects (references
   rewritten per kind, links to itself removed, lists that would name a
   record twice, families of the same couple) are listed. A refusal is
   shown with its reason; the one a person may lift (two siblings of one
   name) asks for the name typed back, as the library does; the others
   leave only "Cancel".
3. **The merge is the library's.** The page sends the plan's fingerprint,
   the choices and the typed name; a plan another edit has made stale is
   refused (`STALE_PLAN`) and the reader is sent back to plan again.
   Nothing about AXGF changes: the result is what `merge_entities` writes.
4. **Nothing is lost.** The absorbed record goes to the journal as a
   `merge` entry, field by field against nothing, with `merged_into` naming
   the kept record — so an old address of a merged person says where it
   went (B7), as after a deduplication. The kept record's change is an
   `update` entry with its diff. The result page shows the absorbed record
   as it was.
5. **Administrators only**, as the bulk delete and the deduplication are,
   and within a scope when the account has one: both records are checked.

## Consequences

- Persons and places need their own `/admin/person/merge` and
  `/admin/place/merge` routes: `/admin/person/:id` and `/admin/place/:id`
  would take "merge" for an id. The places listing's bulk bar had the same
  problem with `/admin/place/bulk`, which answered 404 for delete and export
  until this change gave it a route of its own.
- The plan's words come from the catalogue by building the key
  (`merge-refusal-*`, `merge-because-*`, `merge-becomes-*`,
  `merge-from-*`, `merge-choice-*`), enumerated in `tests/i18n.rs`; a value
  a newer library adds shows as itself until it has words. The fields are
  named as the record page names them; one without a name there shows its
  path.
- Events, links, occupations and documents are not merged: the library
  does not merge them, and their bulk bar has no button.
