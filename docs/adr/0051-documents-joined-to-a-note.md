# ADR 0051 — Documents joined to a note

**Status:** accepted, v1.19.0 (2026-11-26) · **Builds on:** docs/adr/0014 (notes and
contributions), docs/adr/0041 (what a file may weigh and be) · **Applies
to:** `src/notes.rs`, `src/routes/notes.rs`, `src/routes/public.rs`
(`document_read`), `src/routes/mod.rs`, `templates/admin_note.html`,
`templates/_md_editor.html`, `templates/_note_frame.html`,
`templates/_notes.html`, `static/notes.js`, `static/app.css`,
`tests/notes.rs`

## Context

On 26 November 2026 the operator asked, looking at the notes' editor
(`/admin/notes/new`), for a note of the family's library to hold more than
its text: documents joined to it — a film, a photograph, a PDF document —
several if need be, read in the library with the note, the text written
beside them belonging to the same note. A button *Add a document* was to
sit beside *Add a picture* in the editor's toolbar.

A note's pictures already live in the archive as Documents of their own,
marked with the note they belong to (docs/adr/0014), so that any reader of
the format finds them.

## Decision

1. **A joined document is a Document of its own**, its bytes in the archive
   at `notes/<note>/files/<id>.<ext>`, tagged `note-file` and marked
   `extensions["axgf-cms:note-file/v1"] = {"note": <id>}`. The note lists
   them, in the order they were joined, under `files` in its own extension.
   Like the pictures, they are never among a record's files, and they are
   read by whoever may read the note, under its rule, and by nobody else.
2. **They come with the note's form**, which is multipart, and are saved in
   the same commit as the note: all of it or nothing. No script is needed:
   the field is a plain file input under the editor, and the toolbar's
   button opens it. Each file's type comes from its bytes, as a record's
   does (`documents::sniff`); a kind the administration refuses, and
   anything that is a program, are refused with the page. A file weighs at
   most what a record's file may (`document` under Uploads), one save
   carries four times that (the request a record's file may weigh, held in
   memory while it is read), and a note holds 20: more documents than one
   save carries are joined over several.
3. **Each is shown as what it is:** a picture drawn, a film or a recording
   played in the page, a text's first 20,000 characters shown, a PDF
   document read in the browser's own viewer through `/document/:id/read`,
   anything else downloaded. The site never frames itself (`frame-ancestors
   'none'`), so a PDF document opens in a tab of its own rather than inside
   the note; its response says `inline` and carries a policy that lets the
   viewer in and nothing else. It is not sandboxed: a browser refuses to
   show a PDF document in a sandbox.
4. **Changing a note keeps its documents**; each can be let go when it is
   saved. **Deleting a note** takes its documents with it, as its pictures.

## Consequences

- A note's documents weigh in the archive, its backups and its exports like
  any record's file.
- A document refused, or past the size, makes the form come back with the
  text as written; the documents have to be chosen again, as a browser
  never fills a file input itself.
- A PDF document's own scripts run, if at all, in the browser's viewer,
  which keeps them away from the page and its session. The documents are
  given by administrators and contributors, as a record's are.
