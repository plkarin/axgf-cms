# ADR 0014 — Notes and contributions: what the family writes down

**Status:** accepted, v0.5.0 (2026-10-22) · **Builds on:**
[ADR 0009](0009-ai-reworked-portraits-are-never-evidence.md), whose marked
documents are the precedent for documents a record does not list ·
**Applies to:** `src/notes.rs`, `src/note_pdf.rs`, `src/routes/notes.rs`,
`src/routes/public.rs`, `src/routes/admin.rs`, `src/routes/mod.rs`,
`src/state.rs`, `src/person.rs`, `src/render.rs`, `templates/_notes.html`,
`templates/_note_frame.html`, `templates/note.html`,
`templates/admin_notes.html`, `templates/admin_note.html`,
`templates/home.html`, `templates/_person_detail.html`,
`templates/admin_dashboard.html`, `static/notes.js`, `static/app.css`,
`static/fonts/`, `static/img/notes-*.jpg`, `locales/*.ftl`,
`tests/notes.rs`

## Context

On 22 October 2026 the operator asked for a section under a record's
Identity called "Notes and contributions":

- written by administrators and contributors from the administration, each
  with a title, a category, a text into which pictures are pasted or
  uploaded, a date, and the people it concerns;
- shown on the record of each person it concerns; a note that concerns no
  one belongs to the family;
- all of them readable from the masthead's "Overview", renamed "Family",
  in a block right under the page's head, for every member: searched, read
  by letter or all at once;
- each opened in a window that shows it and offers it as a PDF;
- in the theme's mood, over soft, neutral pictures that follow the theme.

## Decision

### 1. A note is an AXGF document, and the format does not change

A note is a `Document` of the bundle:

- `mime_type: text/markdown`, its text a payload at
  `notes/<id>-<version>.md` (a new path per version, so an edit never
  overwrites a payload another version names);
- `caption` the title, `date` an AXGF date (year, month or day),
  `created_by` the account that wrote it;
- `linked_to` each person it concerns, role `note_subject`;
- `tags: ["note"]`, and `extensions["axgf-cms:note/v1"]` holding the
  category, a summary for the cards, and its pictures' ids.

Another reader of the bundle sees a Markdown document linked to people,
which is what it is.

### 2. Its pictures are documents too, named by the text

Each picture is a `Document` of its own (`photo`, tag `note-image`,
`extensions["axgf-cms:note-image/v1"].note` naming its note, payload under
`notes/<note>/`). The text names it as `axgf:document/<id>`.

A picture pasted, dropped or chosen while writing waits in memory, for the
account that pasted it only (six hours, 200 MB in all), named
`axgf:pending/<token>` in the text. It is stored when the note is, in the
same commit, before the note that names it. A save that fails — a conflict,
a refusal — keeps the text as written and the pictures waiting; they leave
the waiting room only once the commit is applied. A picture the text no
longer names is dropped with the edit that removed it; deleting a note
deletes its pictures.

A note and its pictures are never a record's files, gallery, face or
editable documents, as ADR 0009's portraits are not.

### 3. Who reads

- **Members only**: a reader who is signed in. A visitor is told, on the
  Family page, that the family keeps notes for its members and how to sign
  in; a note's address, its window and its PDF answer *not found* to them.
- **Through the lens**: a note is read only by a reader who sees every
  person it concerns. A note about a private person is as hidden as the
  person. Its pictures follow their note; a picture that lost its note is
  refused.
- Every answer that carries a note is `private, no-store`.

### 4. Who writes

Administrators write and change any note. Contributors write notes and
change or delete only their own (`created_by` is their account). Viewers
write none. Every change carries the version it was made from; a change to
a version that is no longer the latest is refused, and the form says so with
what was written still in it.

### 5. The text: Markdown, read safely

pulldown-cmark, with:

- raw HTML shown as text, never as markup;
- links only `http`, `https`, `mailto` and this site's own paths;
- headings moved down two levels, under the note's own title;
- pictures only the note's own (`axgf:document/…`; a pending one in the
  writer's preview only), served through the documents' one gate.

### 6. Where it is read

- **The Family page** (`/about`; "Overview" in the masthead is now
  "Family"): a block right under the page's head. A search over the title,
  category, people, author and the whole text, without case or accents; a
  row of letters, A to Z, then any other letter with a case a title begins
  with (Cyrillic, Greek, Ł), then `#` for the rest (digits, scripts without
  case); "All". Without script these are a form and links
  (`?q=`, `?letter=`); with it they filter in place.
- **A record**: a section under Identity, the cards of the notes about this
  person; a writer sees it even empty, with the way to write the first.
- **The window**: a `<dialog>` filled from `/notes/<id>?frame=1`; without
  script, each card is a link to `/notes/<id>`.
- **The administration**: `/admin/notes`, reached from the dashboard (not a
  sixth room of the console's strip, which stays at five).

### 7. The PDF is drawn by the server

`/notes/<id>/pdf`: A4, the title and its lines (category, date, the people,
the author), the text with its headings, lists, quotations and pictures, and
a footer naming the family, the application and the page. Drawn with krilla
and measured with rustybuzz, in DejaVu Sans embedded with its licence
(`static/fonts/`); italics are the regular face leaned. No browser or
HTML-to-PDF engine runs on the server.

Known limits: a right-to-left line is drawn as one run, without the
bidirectional algorithm; Chinese and Japanese have no glyphs in this face.
Such a note reads whole in the window and on its page.

### 8. Over the theme's own picture

One soft, nearly plain picture per theme, made in the operator's Canva
(`static/img/notes-<theme>.jpg`, 1600 px, 26–65 KB), under a veil of the
theme's surface so that text over it reads as anywhere else: a forest's
canopy at dusk (Canopée), a dark ground with an ember glow (Fusion), a
violet nebula (Galaxy), ridges at dawn (Summit), pale glass ribbons on
white (Clarity).

## Consequences

- The bundle stays AXGF 1.1; a note travels with the archive and survives an
  export and an import.
- The Family page and every record read the notes on each visit; the
  search reads the text payloads of the notes the cards do not already
  match.
- A forgotten pasted picture costs memory for six hours at most, within a
  fixed budget; the oldest goes first.
