# ADR 0033 — The AFG reads a person's photographs

**Status:** accepted (unreleased) · **Builds on:** the AFG in conversation on
a person's page (docs/adr/0031), the personal assistant's files
(docs/adr/0026) · **Applies to:** `src/person_chat.rs`,
`src/routes/person_chat.rs`, `src/routes/assistant.rs`, `src/assistant.rs`,
`src/state.rs`, `templates/_person_afg.html`, `locales/*.ftl`,
`tests/person_chat.rs`

## Context

On 6 November 2026 the operator asked that the AFG, on a person's AFG tab,
also read the record's media: the photographs, yes; the films, no.

Until then the dossier was words only: the record, the notes, the close
relatives and the index of the family. A photograph often says what no note
says, such as a uniform, an insignia, a place, a face or a handwritten line.

## Decision

### 1. What is given

The pictures of the person's record, as the record's Media tab lists them
for the member:

- only files the member may open;
- in the raster formats this build decodes: JPEG, PNG, GIF, WebP, BMP.

These are left out:

- **films and sound**, as asked;
- **PDF documents**. When the AFG read one, its transcription is already a
  note in the dossier;
- **a picture an AI made**, a restored copy or a reworked portrait: it is
  the AI's work, not the family's record;
- **a picture that sensitive data refers to** (health, biometrics, genome,
  legal, behaviour), unless the AFG room lets those classes go, as for the
  rest of the dossier (docs/adr/0023);
- a TIFF or an SVG, which no model is given here.

A relative's photographs are not given: the dossier is about the person.

### 2. How

- **Twelve at most** with each message, in the record's order. The dossier
  says how many were left out.
- **Sized as the assistant sizes a picture**: turned upright as its EXIF
  says, at most 1,568 pixels on its longest side, as a JPEG
  (`assistant::model_photo`).
- **Sized once per file**, then kept in a bounded cache of 48 MB, keyed by
  the document and its checksum (`AppState::model_photos`).
- **Read in the background**, on the way to the AFG, not while the page
  waits (`run_answer`'s `Prepare`).
- **They go with the message being answered**, the last of the
  conversation's turns. Each request carries them once; the earlier
  messages are not given them again.
- **The dossier lists them** by number, file name, kind of document,
  caption and note, in the order the pictures follow.
- **The instructions ask the AFG to look at them**: say which photograph,
  and how sure it is of what it sees.

### 3. What the tab says

The line under the tab's title counts them: "Read with each message: the
record, photos (2), notes (1)…". Its estimate adds about 1,600 tokens for
each photograph, about what a model reads of a picture at that size.

### 4. A model that takes no picture

The services that take pictures take them in the way each reads them
(`afg_chat`). A model that takes none refuses the request (HTTP 4xx); this
can happen with a local model under Ollama, Mistral or vLLM, or an OpenAI
model without vision. The question is then asked once more without the
photographs. The AFG is told that they could not be given, so that it says
so, in the member's language, when the question needs them.

## Consequences

- A message about a person with photographs costs more: about 1,600 tokens
  each, up to about 19,000 for twelve.
- The photographs go to the AFG's computing as the room sets it, like the
  rest of the dossier. With Claude Code they count against the account's
  usage.
- The log says how many photographs went with a question, never what they
  show.
