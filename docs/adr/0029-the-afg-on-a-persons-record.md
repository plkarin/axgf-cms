# ADR 0029 — The AFG on a person's record

**Status:** accepted, v1.4.0 (2026-11-04) · **Amended by:**
[ADR 0030](0030-canva-and-the-familys-own-servers-for-pictures.md): the
picture service may also be Canva or a machine of the family's own ·
**Builds on:** docs/adr/0009
(portraits an AI reworked), docs/adr/0014 (notes), docs/adr/0023 (the AFG),
docs/adr/0026 (My personal assistant) · **Amends:** docs/adr/0009, which kept
every picture an AI made out of the Media tab; docs/adr/0014, which wrote
notes from the Fiche · **Applies to:** `src/afg.rs`, `src/afg_record.rs`,
`src/routes/afg_record.rs`, `src/routes/afg.rs`, `src/routes/notes.rs`,
`src/routes/public.rs`, `src/routes/admin.rs`, `src/person.rs`,
`src/view.rs`, `templates/person.html`, `templates/_person_detail.html`,
`templates/_notes.html`, `templates/admin_note.html`,
`templates/admin_afg.html`, `static/app.css`, `locales/*.ftl`,
`tests/afg_record.rs`

## Context

On 3 November 2026 the operator asked for the AFG on a person's record, for
the signed-in members it is open to:

- **The face.** Under the button that uploads portraits an AI reworked, a
  button that reworks the image with the AI: colours put back, sharpness
  improved. The picture then goes where an uploaded one goes.
- **The Media tab.** For each file:
  - an AI retouch, which puts the new photograph in the Media tab under the
    same name with `_IA` added;
  - a button that reads the document: a letter photographed is transcribed
    by the AI into a note, in the document's own language.

  The notes become editable and deletable on the Media tab, and are only read
  on the Fiche.
- **The Fiche.** A short summary of the person, first, just above Identity:
  - written by hand, in the notes' editor;
  - or written by the AI from everything the record holds but its files: the
    notes and the record's fields, in the language chosen.
- **The languages.** The site's own fourteen.

Claude and the coding agents write text only, and production computes with
Claude Code. A picture therefore needs a service that makes pictures.

## Decision

### 1. Who

Every button is drawn, and every door opened, only for a member who:

- may write (contributor or administrator);
- has the AFG open to their account (docs/adr/0024);
- can read the person and may change their record (the account's branch).

A writer without the AFG sees none of them, and is answered 403. The picture
service is chosen by an administrator.

### 2. Where pictures are made

The AFG room has a section of its own, **Images** (`#images`,
`POST /admin/afg/images`), kept in `family.acl` beside the AFG's settings:

- **The service:**
  - none;
  - OpenAI, `POST /images/edits`, a multipart form with `model`,
    `prompt` and `image[]`, the picture back as `data[0].b64_json`;
  - Google Gemini, `POST models/<model>:generateContent`, the photograph
    inline with the instruction and `responseModalities: ["IMAGE"]`, the
    picture back as an `inlineData` part.
- **Its address:** empty is the service's usual one.
- **Its key:**
  - never rendered, only its last four characters;
  - forgotten when the service changes, unless a new one comes with the
    change, so a key is never sent to a service it was not given for;
  - forgotten, with everything else, when the service is set to none.
- **Its model:** empty is the first suggested. The suggestions, as read from
  the makers' pages on 3 November 2026:
  - OpenAI: `gpt-image-2.5-sunburst`, then `gpt-image-2.5-flare`;
  - Google: `gemini-3.1-flash-image` first, the one `generateContent` is
    known to answer with a picture; then `gemini-nano-banana-2.1`, which
    Google's page shows only through its newer Interactions API; then
    `gemini-3-pro-image`.

Every picture is asked for with one fixed instruction (`afg_record::RESTORE`):
give colours to a black-and-white, sepia or faded photograph, sharpen it,
remove scratches, dust and noise, and keep the people, their faces, their
clothes and the framing exactly as they are. The photograph is sent at 2,048
pixels at most. A picture larger than the 10 MB of an upload is kept as a
JPEG.

### 3. The face

**Rework the image with AI** sends the person's face, the picture the
masthead shows, to the picture service. What comes back takes the path of a
portrait uploaded by hand (docs/adr/0009):

- the role `ai_portrait`, the tag `ai-generated`;
- `extensions["axgf-cms:ai-portrait/v1"]` naming the service and its model
  as `generator`, and the face as `derived_from_id`;
- named after the photograph, `<name>_IA.<ext>`;
- three at most: past them the button is greyed and the door answers 409.

### 4. A photograph restored

**AI retouch**, under each photograph of the Media tab, keeps what comes back
beside the photograph, as a Document of the person's:

- named `<name>_IA.<ext>`;
- tagged `ai-retouched`, with `extensions["axgf-cms:ai-retouch/v1"]` naming
  the `generator` and the photograph it was `derived_from_id`;
- `document_type: other`, for the reason docs/adr/0009 gives: nobody took
  it;
- with the photograph's caption and date;
- linked to the person, `subject`.

It stays in the Media tab, marked **AI**. It is never the person's face,
chosen or guessed: a choice that points at one falls back to the heuristic,
and the picker neither offers nor accepts one. It is not retouched or read
again. Its tag is not `ai-generated`, which would take it out of the Media
tab (docs/adr/0009).

### 5. A document read

**Read the document**, under each photograph and on each PDF's row, sends the
document alone, without the archive or a brain, to the AFG's computing:

- **The instruction:** transcribe every word, in the document's own language
  and script, without translating, correcting or commenting. Mark a word
  that cannot be read `[illegible]`, and a doubtful one `[?]`.
- **A document without text:** the AFG answers `NO_TEXT`, the page says so,
  and nothing is written.
- **A PDF:** sent only to computing that reads one (OpenAI, Claude, Gemini,
  Claude Code and OpenCode). The others would be given its name alone, so the
  door refuses it.

The transcription is a note (docs/adr/0014):

- titled "Transcription — <caption or name>", in the category Transcription;
- dated as the document is;
- about the person;
- tagged `transcription`, with `extensions["axgf-cms:transcription/v1"]`
  naming the `document` and the `generator`;
- its text ending with a link to the document read.

### 6. The summary

The summary is a note too, tagged `person-summary`, with
`extensions["axgf-cms:summary/v1"]`:

- `person`: the person it sums up;
- `language` and `generator`, when the AFG wrote it.

It is shown first on the Fiche, above Identity, to the family's members, as
every note is. It is not listed among the notes, on either tab.

**By hand.** "Write the summary" opens the notes' editor (`kind=summary`),
titled for the person, and comes back to the record.

**By the AFG.** "Write it with the AI", with a list of the site's fourteen
languages, the page's chosen. The AFG is sent:

- the person's page, its labels in the language asked, without its files,
  its pictures or its page furniture;
- the profile's groups;
- the text of the notes about the person, 120,000 characters at most.

The health, biometric, genomic, legal and behavioural classes go only when
the AFG room's box allows them, as with the archive (docs/adr/0023).

**Replacing it.** A summary already there is rewritten in place: the same
note, its next version. Only its author or an administrator may do that, as
for any note. A summary rewritten by hand loses its `generator` and keeps its
language. When two summaries exist, the latest is the one shown.

### 7. The notes, kept up on the Media tab

- **The Media tab** lists the notes about the person, the transcriptions
  marked **AI**. Each note has Edit and Delete for whoever may change it, and
  "Write a note about this person" is at the top. Each of these comes back to
  the tab.
- **The Fiche** reads them, with a link to the Media tab for a writer. While
  there are none, the button that writes the first stays on the Fiche.
- **The Media tab is drawn** for notes alone, for a reader who is signed in.

The return is a form field, `back`. Only a path to one of the site's records
(`/person/…`) is followed. Any other value is ignored.

## Consequences

- **Each picture is paid for.** OpenAI or Google bill the account whose key
  is given, per picture. Reading and summaries go to the AFG's computing as
  it is set: with Claude Code, the subscription it is signed in with.
- **What leaves the machine:**
  - the face or the photograph, to the picture service;
  - the document read, to the AFG's computing;
  - the record and the notes summarised, to the AFG's computing, the
    sensitive classes as the box says.

  The log says who sent what to which service, never what it held.
- **The sandbox.** A picture service on the internet needs the outbound
  network the AFG's own does (docs/OPERATOR.md, "The AFG").
- **A button waits for its answer.** The page says it is working and refuses
  a second press. A long answer keeps it waiting, up to the AFG's time
  allowed.
- **Tested with fakes only.** The OpenAI and Gemini calls follow the makers'
  documentation as read on 3 November 2026; no real picture has been asked
  for.
