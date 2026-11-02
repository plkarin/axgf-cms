# ADR 0021 — Polygamous unions: one marriage per spouse, never a "full" union

**Status:** accepted, v1.1.0 (2026-11-01) · **Builds on:** AXGF 1.0 §4.2.1–§4.2.2,
ADR 0020 (kinship) · **Proposes:** axgf-spec proposals/0003 · **Applies
to:** `src/unions.rs`, `src/kinship.rs`, `src/kinship_text.rs`,
`src/person.rs`, `src/tree.rs`, `src/chart.rs`, `src/river.rs`,
`src/access.rs`, `src/acl.rs`, `src/routes/person_editor.rs`,
`src/routes/editors.rs`, `templates/_person_detail.html`,
`templates/_tree_canvas.html`, `templates/tree.html`, `static/app.css`,
`locales/*.ftl`

## Context

On 1 November 2026 the operator asked that a polygamous union be told
apart from a "full" union: "la polygamie est renseignée dans la spec, il le
faut ; pour les unions polygames dans le CMS il faut les distinguer d'une
union pleine".

AXGF names the union type `polygamous` (1.0 §4.2.1). Its example (§4.2.2)
gives the union a `primary_person_id` and a `unions[]` list, each entry
being one spouse's marriage to the primary person with its own dates. The
schema defines neither and requires `union.persons`, and the reference
library keeps both as extra fields.

This application read every union as a couple. A union of three was
everyone married to everyone, its children full brothers and sisters, and a
family written as the specification shows it showed only the husband, since
the wives were only in `unions[]`.

## Decision

1. **One reading, everywhere** (`src/unions.rs`). Every view reads a family
   through `unions::couples`: the record, the tree, the river, kinship, and
   the access and branch-scope checks (`unions::members`).
   - A union that is not polygamous is one couple, as before.
   - A polygamous union with more than one spouse is **one couple per
     spouse**: the primary person and that spouse.
     - Each couple carries the spouse's own marriage, its `unions[]` entry:
       start, end and status.
     - Its children are the ones whose `parent_ids` name that spouse.
     - The children whose other parent is not recorded stay with the
       primary person alone, in an *open* couple.
   - A polygamous couple of two, one family per marriage as GEDCOM records
     it, is one couple, marked polygamous.
2. **The primary person** is the one `primary_person_id` names. When none is
   named, it is the member the record makes plain:
   - the only member `unions[]` does not name as a spouse;
   - else the only one whose recorded role, then whose sex, differs from all
     the others';
   - else the first listed, where every converter and the specification's
     own example put them.
3. **Kinship** (ADR 0020).
   - The wives of one husband are each other's **co-wives** (co-husbands,
     co-spouses): two partners of one person in polygamous unions, wherever
     they are recorded. They are never each other's wife.
   - Children of two wives are **half**-siblings.
   - A child in an open couple is a brother or a sister of the others, never
     said full or half: nobody knows whether they share a mother.
4. **Drawn apart from a couple.**
   - On the tree, each marriage's bracket has a long-and-short pattern and
     its knot three rings, and the legend says so.
   - On the record, each marriage is its own union block, framed with a
     double rule. A wife's block names the husband's other wives under
     "Co-wives".
5. **The editor.** On a polygamous union, the record editor's Family tab
   offers two fields:
   - "At the centre of the union", which writes `primary_person_id`;
   - "Other parent" for each child, which writes `parent_ids`.

   Taking a spouse out of the union forgets them everywhere they were named:
   `unions[]`, `primary_person_id` and every child's `parent_ids`. Every
   spouse `unions[]` names is also listed in `union.persons`, so that a
   reader that knows only `persons` still finds everyone. The older
   relationships page keeps every union and child field it does not draw;
   it used to rebuild the union and dropped them.

## Consequences

- **Nothing about the file changes for a couple.** A polygamous family
  written by this application is valid AXGF 1.1: the fields it adds are
  extra fields, kept by the reference library. Proposal 0003 asks AXGF 1.2
  to define them.
- **One family per couple stays readable.** A GEDCOM import keeps one family
  per marriage. Typed `polygamous`, its marriages read the same way, the
  co-wives included.
- **A union typed otherwise is not guessed at.** Three partners in a
  `marriage` are still read as the record says: as a union of three.
- **Words.** "Co-wife" and its kin are new in the fourteen catalogues. The
  twelve beyond English and French are unreviewed, like the rest.
