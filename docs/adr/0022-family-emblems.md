# ADR 0022 — A family's emblem, and the archive's

**Status:** accepted (2026-11-01) · **Builds on:** the person's avatar
(`src/avatar.rs`), AXGF 1.0 §3 (manifest), §4.2 (family documents), §5.5
(documents) · **Proposes:** axgf-spec proposals/0004 · **Applies to:**
`src/emblem.rs`, `src/access.rs`, `src/render.rs`, `src/person.rs`,
`src/chart.rs`, `src/tree.rs`, `src/routes/admin.rs`, `src/routes/mod.rs`,
`src/routes/person_editor.rs`, `templates/admin_emblem.html`,
`templates/base.html`, `templates/home.html`,
`templates/_person_detail.html`, `templates/_tree_canvas.html`,
`templates/admin_bundle.html`, `templates/admin_person.html`,
`static/app.css`, `locales/*.ftl`

## Context

On 1 November 2026 the operator asked for "un avatar de famille, afin de
disposer d'un blason ou avatar de famille dans le CMS". Asked at which
level, they answered: both. One emblem for the whole archive, at the head of
the site, and one for each family the archive records.

A person already has a picture that stands for them, chosen rather than
guessed (`axgf-cms:avatar/v1`). Neither a family nor the archive had one.

## Decision

1. **A family's emblem** is chosen as a person's picture is. It is stored as
   `extensions["axgf-cms:avatar/v1"]` on the Family, in the same shape: one
   of its documents with a focal point, or "none".
   - When nobody has chosen, it is the first image the family links in the
     role `emblem`.
   - A family's documents are those whose `linked_to` names it, and those
     its own `documents[]` lists.
2. **The archive's emblem** stands for the family the whole archive is
   about. It is stored as `manifest.family.extensions["axgf-cms:emblem/v1"]`,
   in the same shape. It lives in `manifest.family` because that is the part
   of the manifest an administrator edits, checked against concurrent edits,
   journalled and exported.
3. **Where they show.**
   - The archive's emblem appears at the head of every page, beside the
     product's name, and above the family's name on the overview.
   - A family's emblem appears beside its union on each record, and on its
     couple's knot in the tree. A polygamous household's marriages share
     the one emblem.
   - Emblems are shown whole (`object-fit: contain`), because a coat of
     arms is a whole shape.
4. **Where they are chosen.**
   - A family's emblem is chosen at `/admin/family/:id/avatar`, reached from
     its union block in the record editor.
   - The archive's is chosen at `/admin/bundle/emblem`, reached from the
     Archive room of the administration. That page is for administrators
     only.
   - Both pages offer the pictures already there, a new upload that is used
     at once, and "No emblem".
   - An upload must be a raster image. It becomes a Document whose
     `linked_to` names the family in the role `emblem`. The archive's has no
     subject.
5. **Who may see them** (`access::may_read_document`).
   - A family's documents follow the family's people, as a person's follow
     the person: anyone who may read one of its partners or children may read
     them. A family nobody of which a reader may see keeps its emblem.
   - The archive's emblem is every visitor's, as the family's name is.

## Consequences

- **The page chrome reads the archive.** `Chrome::with_archive` reads the
  emblem once per page. Every place a page resolves its chrome calls it.
- **No new kind of entity.** An emblem is a Document like any other. Leaving
  the role or the extension unread costs another program nothing: it sees a
  picture of the family.
- **The format proposal.** AXGF 1.0 has no `coat_of_arms` document type and
  no emblem in the manifest. Proposal 0004 asks 1.2 for both, and for the
  role `emblem`.
