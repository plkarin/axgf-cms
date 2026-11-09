# ADR 0040 — The AFG reads a record's PDF documents

**Status:** accepted, v1.11.0 (2026-11-09) · **Builds on:** docs/adr/0031 (the AFG
on a record), docs/adr/0033 (its photographs) · **Applies to:**
`src/pdf.rs`, `src/person_chat.rs`, `src/routes/person_chat.rs`,
`src/state.rs`, `templates/_person_afg.html`, `locales/*.ftl`

## Context

Since 1.8.0 the AFG reads the photographs of a person's record with each
question asked on its tab (docs/adr/0033). Asked whether it should read the
record's PDF documents too — deeds, registers, letters, certificates — the
operator answered yes.

A PDF is not a picture: a model reads each of its pages as an image and as
its text, about 1,500 to 3,000 tokens a page, and a request carrying files
may weigh 32 MB at most with the services that read them. A record can
hold a parish book of several hundred pages.

## Decision

1. **Which**: the PDF documents of the person's record, on the photographs'
   terms — the record's own that the member may open, none an AI made,
   none that sensitive data the AFG room does not let go refers to
   (docs/adr/0023). A relative's are not given.
2. **How many**: in the record's order, each whole, while the pages given
   stay within **60** (`PDF_PAGES`) and their bytes within **20 MB**
   (`PDF_BYTES`): sent as base64, a third larger, they leave room under
   32 MB for the photographs and the dossier. A PDF that does not fit is
   left out, the next one tried; one whose pages cannot be counted is left
   out as unreadable.
3. **Counting pages without a PDF library**: `pdf::pages` reads the page
   tree's `/Count` (or counts the pages), inflating the compressed object
   streams with `flate2`, and bounding what it inflates. A PDF's count is
   kept by its SHA-256 (`AppState::pdf_pages`), so it is read once, not at
   every question.
4. **In the dossier**: a section lists them, `P1. "acte.pdf" (Acte,
   2 pages) — caption — note`, and says how many were left out. Its size,
   shown on the tab before the first question, counts **2,000 tokens a
   page**; the tab says how many PDF documents go with each message.
5. **With the question**: the PDFs are read off the page's way and joined
   to the message being answered, named as the dossier lists them, after
   the photographs. A way of computing that reads no PDF (Ollama, an
   OpenAI-compatible server other than OpenAI's) is told one was sent and
   could not be read; a service that refuses the request is asked once
   more without the files, and the AFG told they could not be given.
6. **Its instructions** ask it to say which document and which page it
   read, and how sure it is of a hand.

## Consequences

- A question about a person whose record holds a scanned deed can be
  answered from the deed itself, with the page named.
- A long PDF (a whole register) is not read with every question: past 60
  pages it is left out, and the dossier says so. Reading one document at
  length stays the record's "read with the AFG" (docs/adr/0029).
- Tests: `pdf::pages` (structures, object streams, bounds), the budget in
  `person_chat` (sixty pages, twenty megabytes, an unreadable one), and
  `tests/person_chat.rs` (the tab's count, the dossier's line, a relative's
  PDF not given, the PDF joined to the question).
