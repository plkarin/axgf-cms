# ADR 0041 — Uploads the administration sets, and films that play

**Status:** accepted, v1.11.0 (2026-11-09) · **Builds on:** docs/adr/0037 (a file
over the limit refused before it is sent) · **Applies to:** `src/limits.rs`,
`src/site.rs`, `src/state.rs`, `src/routes/limits.rs`,
`src/routes/mod.rs`, every route that takes a file, `src/documents.rs`,
`src/person.rs`, `src/routes/public.rs` (`document_raw`),
`templates/admin_uploads.html`, `templates/admin_bundle.html`,
`templates/_person_detail.html`, `locales/*.ftl`

## Context

The operator asked, after a 20 MB PDF refused on 8 November 2026
(docs/adr/0037), for **every upload limit and the accepted kinds of file to
be set in the administration**, and for **a player for the films sent**:
a film attached to a record downloaded, and could not be watched on the
page.

The limits were constants (`documents::MAX_UPLOAD` 50 MB, `notes::MAX_IMAGE`
10 MB, the assistant's 20 MB a file and 40 MB a message, a GEDCOM's 10 MB,
an archive's 4 GB), and so were the request limits axum enforces, fixed
when the router is built. The kinds were whatever `documents::sniff`
recognises.

## Decision

### 1. The limits and the kinds are settings

`site.uploads` (`limits::Limits`), kept with the site's other settings and
omitted while it holds the defaults:

| Limit | Default | Bounds |
|---|---|---|
| A file attached to a record (document, portrait, emblem, reworked portrait) | 50 MB | 1–2048 |
| A picture pasted into a note | 10 MB | 1–200 |
| A file sent to the personal assistant | 20 MB | 1–500 |
| All the files of one message | 40 MB | 1–2000 |
| A GEDCOM file (conversion, set-up) | 10 MB | 1–500 |
| An archive imported whole (set-up) | 4096 MB | 16–65536 |

The defaults are the old constants. A value outside its bounds is refused
by the form, and one edited into the settings file is brought back within
them when read. The assistant's own picture (8 MB, resized to 256 px) and
a note's text keep their fixed limits: they are not files kept.

**Kinds**: the sixteen the archive recognises from a file's bytes, by
family — pictures (JPEG, PNG, GIF, WebP, BMP, TIFF), documents (PDF, TXT),
sounds (MP3, Ogg, WAV, M4A, FLAC), films (MP4, WebM, AVI). The settings keep
the kinds **refused**, so a kind a later version learns to recognise is
accepted until refused. A refused kind is refused when sent to a record, an
emblem, a portrait or a note (`415`, `error-upload-kind-refused`, naming
it); the files already in the archive stay. The assistant's files, which
the archive does not keep, are not filtered.

### 2. Read at each request

`AppState::limits()` reads the settings each time. The routes that take
files no longer carry axum's fixed `DefaultBodyLimit`: it is disabled, and
a middleware (`weighed`) holds the body to the weight allowed **now**
(`http_body_util::Limited`) — four times the file for a record's routes,
so that a file somewhat over the limit is read to its end and refused with
a page instead of a closed connection (docs/adr/0037). The handlers check
the file against the same settings, and the pages say the same numbers,
including the browser's own check before sending. A change in the
administration holds at the next request, without a restart.

### 3. The page

`/admin/uploads`, a page of the archive's room (which says the record
limit and how many kinds are accepted, and links to it): six numbers, and
the kinds in a disclosure — sixteen boxes would be sixteen controls on a
page whose business is six numbers (the screen budget).

### 4. Films and sounds play

- `documents::plays_as(mime)`: `video` for MP4 and WebM, `audio` for MP3,
  Ogg, WAV, M4A and FLAC. An AVI is a film no browser plays: it downloads.
- The record's Media tab plays them (`<video controls preload="metadata">`,
  `<audio controls>`), above the documents table, named by their caption.
- `GET /document/:id/raw` serves them inline and answers **byte ranges**:
  `Accept-Ranges: bytes`, one `Range: bytes=a-b` (or `a-`, `-n`) gives `206`
  with that piece read from the file at its offset, `416` past the end;
  several ranges are served whole. A long film starts at once and its
  slider goes anywhere in it, without the whole file being sent first.

## Consequences

- An operator whose family films weigh a gigabyte raises one number; one
  who wants no films refuses three boxes.
- A limit lowered below a file already kept does not touch it.
- The record's films are watched where they are kept.
- Tests: `limits` (defaults equal to the old constants, bounds, kinds),
  `tests/media.rs` (the player and its ranges; the page's bounds, a limit
  and a refused kind holding at once), and the suites over every route
  that takes a file.
