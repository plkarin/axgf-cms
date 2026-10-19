# ADR 0009 — Portraits reworked by an AI are shown, and are never evidence

**Status:** accepted, v0.5.0 (2026-10-19) · **Applies to:**
`src/ai_portrait.rs`, `src/person.rs`, `src/routes/ai_portraits.rs`,
`src/routes/public.rs`, `src/routes/person_editor.rs`,
`src/routes/editors.rs`, `src/routes/mod.rs`, `templates/person.html`,
`templates/admin_ai_portraits.html`, `static/app.css`, `locales/*.ftl`,
`tests/ai_portraits.rs`, `tests/read_mode.rs`

## Context

On 19 October 2026 the operator asked for "a game-changing feature". In
the record's masthead, in the empty space between who the person was and
the edit button, there should be:

- a button for uploading pictures reworked by an AI;
- beside the face, up to three of them, which "let a signed-in member see
  what the person might have looked like after an AI retouch — a friendly,
  futuristic section that gives more feeling for the person".

A genealogy archive's pictures are evidence. A photograph says that
somebody stood there, looking like that. A picture an AI coloured,
sharpened, aged or imagined says nothing of the kind. If it can be taken
for a photograph, it will be:

- by a cousin who saves it;
- by the next tool that imports the archive;
- by the record itself, whose automatic avatar is the first image linked
  to it.

## Decision

### 1. An ordinary Document, marked three ways

A reworked portrait is an AXGF Document, so it travels with the archive.
It carries three marks, any one of which is enough
(`ai_portrait::is_ai_portrait`):

- it is linked to its person with the role `ai_portrait`, not `subject`.
  1.0 §5.5 leaves the role a free string;
- it is tagged `ai-generated`. Every entity has `tags`;
- it has `extensions["axgf-cms:ai-portrait/v1"]`, which holds 1.1's two
  artefact fields (1.1 §3.5.1): the `generator` and the Document it was
  `derived_from_id`, both optional.

`document_type` is `other`. 1.0's vocabulary has no word for a picture
nobody took, and `photo` is the one claim this record must not make. A tool
that knows none of this still reads a Document whose role, tag and type
make no claim.

### 2. Never one of the record's pictures

The person's view (`person::build_in`) sets them apart from everything
else. So a reworked portrait is never:

- among the record's files;
- in its gallery or on its Media tab;
- the person's picture, chosen or automatic;
- offered on the avatar picker;
- offered as a file to attach.

They are drawn only in their own band, in the masthead, in arrival order,
three at most. Each one is marked with the badge "AI" ("IA", "KI"…) and a
turning ring, and the band says once that none of them is a picture of the
time.

### 3. For the family's members

The band is drawn only for a reader who is signed in.
`public::stored_document` is the one door every file goes through
(`/raw`, `/view`, `/thumb`). It refuses a portrait's bytes to a reader who
is not signed in, whatever the person's own visibility. The person's own
rules still apply on top, through the same lens as every other file.

### 4. One page, one save

`/admin/person/:id/ai-portraits` shows the portraits there are, each with a
box that removes it, and as many file slots as are left. The save does
everything in one commit, so a replacement is one act and the archive is
rewritten once:

- it removes the ticked portraits;
- it adds the new ones, several at once.

It refuses:

- a fourth portrait;
- anything that is not a picture, judged from the bytes and not from the
  file name;
- a removal of a portrait that belongs to someone else.

A removed portrait is deleted, not detached: outside its band it has no
use.

### 5. Read mode

The band follows the read page's rule (`tests/read_mode.rs`): it draws what
is there. An editor whose record has no portrait yet gets only the button,
with no heading over an empty band.

## Consequences

- An AI portrait in an exported archive is plainly one, to any reader of
  the format.
- Generating the pictures is the uploader's business. This application
  stores, labels and shows them; it calls no model.
- `tests/ai_portraits.rs` covers:
  - the three marks and the claim not made;
  - not the face, not in the gallery, not on the Media tab, not on the
    picker;
  - three at most, pictures only, one save that removes and adds;
  - a visitor shown nothing and refused the bytes;
  - a reader shown the band and offered no control.

## Alternatives considered

**Ordinary photos with a caption saying "AI".** Rejected. A caption is
prose, so the gallery, the automatic avatar and every other tool would
still take the file for a photograph.

**Keeping the portraits outside the archive.** Rejected. They would be lost
by an export and a restore, which is exactly how a family archive moves.
