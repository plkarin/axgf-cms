# ADR 0037 — A file of up to fifty megabytes, and one over it refused before it is sent

**Status:** accepted, v1.10.1 (2026-11-08) · **Applies to:** `src/documents.rs`,
`src/routes/uploads.rs`, `src/routes/person_editor.rs`,
`src/routes/admin.rs`, `src/routes/ai_portraits.rs`, `src/routes/mod.rs`,
`static/ui.js`, the upload forms' templates, `deploy/proxy/`

## Context

On 8 November 2026 the operator attached a 20 MB PDF to a person's record,
from the editor's Media tab, over her local network. The browser ended on a
blank page.

- **The limit was 10 MB** (`documents::MAX_UPLOAD`), and the form said so in
  small print. A scanned archive in PDF is often larger, and the operator
  wants the AFG to read a record's PDFs.
- **The refusal did not arrive.** The server stopped reading at its body
  limit, answered "That upload was not stored", and closed the connection
  while the browser was still sending. A browser in that state can lose the
  answer it was sent; hers showed the start of a page and nothing more.
  - Locally, curl and Chromium both received the page, even at 2 MB/s. The
    loss depends on the browser and the network.
  - Nothing was stored, and the server did not stop or restart.
- **Another form stopped saving silently.** A confidence of 0.88 in a
  number field whose step was 0.05 made the editor's form invalid. In a
  hidden tab, Chromium then refuses to submit and says nothing. The
  operator's 3,040 confidences are all multiples of 0.05; the sample's are
  not.

## Decision

1. **Fifty megabytes a file** (`documents::MAX_UPLOAD`). Every form's hint
   follows the constant.
2. **Refused in the page, before it is sent.** Every upload field carries
   the limit (`data-max-bytes`). `ui.js` checks the chosen files:
   - a file over the limit empties the field, so the rest of the form still
     saves;
   - a line beside the field says why, in the reader's language
     (`upload-file-too-large`).
3. **Read to its end when it comes anyway**, from a browser without scripts
   or another client:
   - `routes::uploads::read_capped` keeps a file up to the limit, then reads
     the rest and throws it away;
   - the routes that take a file accept a request of up to
     `documents::MAX_REQUEST` (four files' worth, 200 MB), so that what is
     over can be read to its end;
   - the page that says "over the limit" goes only once the browser has
     finished sending.

   Over 200 MB, the request is cut as before.
4. **The proxies we ship** allow 210 MB, so that the product, not nginx or
   Caddy, answers an oversized file.
5. **A confidence is a hundredth**: the editor's number fields step by 0.01,
   so a recorded 0.88 is valid.

## Consequences

- The operator's 20 MB PDF is stored like any other file. An upload of
  20 MB took under 7 seconds locally, with the save that rebuilds the
  archive.
- A larger archive saves more slowly. Each save rewrites the bundle, and
  `--size-warn-mb` still warns at 200 MB of text.
- A request over 50 MB from an authenticated writer is read in full before
  it is refused, up to 200 MB. Nobody else gets that far: the session is
  checked first.
