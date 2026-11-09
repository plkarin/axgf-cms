# ADR 0039 — A person created with what surrounds them

**Status:** accepted, v1.11.0 (2026-11-09) · **Builds on:** docs/adr/0014 (notes),
docs/adr/0022 (avatars) · **Applies to:** `src/person_new.rs`,
`src/routes/person_new.rs`, `templates/admin_person_new.html`,
`templates/_md_editor.html`, `static/person_new.js`, `static/notes.js`,
`static/ui.js`, `src/routes/public.rs` (search), `src/forms.rs`,
`src/person.rs`, `locales/*.ftl`

## Context

The operator asked, over the first days of November 2026 and again on
9 November:

1. **A search that finds nobody offers to create the person**: "Créer
   « Prénom Nom »", in the masthead's suggestions and on the search page.
2. **A page for the new person** with the birth and death places, created
   on the fly or reused, a portrait, the name in its parts ("Prénom,
   surnom 1 surnom 2…, Nom de famille"), and a biography or a note written
   with the notes' editor, formatted freely.
3. **On that page, the people around them**: the father, the mother, the
   brothers and sisters, the unions (a spouse and children, as many as
   needed), each picked from the archive or created on the fly, reusing
   what exists.
4. **A wrong message**: a place the archive did not hold was refused with
   "No person of that name is in this archive".

Until now a person was created by the generic form (one field per AXGF
path, a place by its id), then completed record by record: the father
created, the family created, the child added, each in its own save.

## Decision

### 1. The offer to create

`/search` and `/search.json` carry `create` — `{label, href, query}` — for
an account that may write, when no person found bears the searched name
letter for letter (case and spaces aside). The masthead lists it last, as
an option: chosen by the arrows or the pointer, never by a bare Enter, which
still opens the first person or, with nobody found, the search page. The
search page shows it as a button. It opens `/admin/person/new?name=…`,
which splits the name: its last word the family name, the rest the first
name.

### 2. One page, four tabs, one save

`/admin/person/new` replaces the generic form for a person (`POST
/admin/person` stays, for scripts). It is the record editor's shape —
tabs over one form, editor.js switching them, the strip marking a tab with
changes, a refused save coming back to the tab of its field — because the
screen budget (thirteen controls of its own, tests/interactive_walk.rs)
allows no more on one tab:

- **Nom**: first name, nicknames (rows that grow), family name, the name
  as it will read (`Prénom « surnom » Nom`, the quotation marks of the
  reader's language, `person-new-nickname-quoted`), gender, living, the
  portrait.
- **Vie**: birth and death, each a date with its *circa* and a place.
- **Biographie et note**: two Markdown editors (§4).
- **Famille**: father, mother, brothers and sisters, the unions — a
  partner, the kind of union, the children — added at need.

The name is stored as `identity.name.display` and its `components`
(`given_name`, `nickname`, `family_name`, in order).

### 3. A name: the person it names, a new one, or a question

Every person and every place typed on the page is resolved by one rule,
the same on the server (`person_new::who`, `person_new::place`) and in the
page (person_new.js, which says under each field what the save will do):

- a **label from the list** (`Nom · années · #xxxxxxxx`) or an id names
  that record;
- any other text names the **one record whose name it is**, letter for
  letter, case and spaces aside — no prefix, no guessing;
- **no record**: a new one, created in the same save; the same new name
  typed twice is one new record;
- **several records**: refused, the field named, until one is picked from
  the list.

A new person is created with what the page knows of them: the father a man,
the mother a woman, anyone else unknown, living. A new place has its name
as its one primary name.

### 4. The families

- **Parents**: the family whose union holds exactly the father and the
  mother (or the one parent) is reused and takes the person as a child —
  a version-checked update; otherwise a family is created, its union of
  unknown kind.
- **Brothers and sisters** join the parents' family. Named without
  parents, they need one already in a family: that family takes the
  person; none, or two different ones, is refused.
- **Unions**: each makes a family with the person and the partner as
  spouses, the kind chosen, the children in it.
- The same father and mother is refused.

Everything — places, people, the person, the portrait's document, the
note and its pasted pictures, the families created and the families
updated — is one `state.commit`: written together or not at all, every
record checked against the writer's scope and against names that point
nowhere. A refusal shows the page again as it was sent, the field marked.

### 5. A biography written like a note

The biography and the note use the notes' editor (`_md_editor.html`, one
toolbar macro; notes.js binds every `[data-md-editor]`). The note is a
real note (docs/adr/0014) about the person, its pictures pasted in, its
title the person's name when left empty. The biography is the person's
`bio`, in Markdown, without pictures, its preview keeping every line
break. Such a biography is marked `extensions["axgf-cms:bio/v1"]` =
`{"format": "markdown"}` and only a marked biography is read as Markdown
on the record (`notes::to_html_keeping_lines`): a biography from a GEDCOM
file is plain text, where "* 1890" is a birth and not a list.

### 6. Each picker says what it picks

`PickError::key_for(group)`: a place, a source or a linked record not
found or ambiguous has its own message (`pick-error-place-*`,
`pick-error-source-*`, `pick-error-thing-*`), in every language.

## Consequences

- A person and their family circle are created in one save instead of a
  dozen, and a name the archive holds is not created twice by accident.
- A name typed on the page that several people bear stops the save; the
  writer picks one from the list.
- 59 new messages in each of the fourteen languages, machine-translated
  for twelve of them and not yet proofread.
- Tests: `src/person_new.rs` (the plan), `tests/person_new.rs` (the
  search's offer, one save with parents reused, a new brother, a union, a
  new place and one found whatever its case; the ambiguous name refused
  with nothing written; a brother alone; the portrait; the place picker's
  message; the biography's preview), and the screen budget and label tests
  over the page.
