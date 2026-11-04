# ADR 0030 — Canva and the family's own servers make the AFG's pictures

**Status:** accepted, v1.5.0 (2026-11-04) · **Builds on:**
[ADR 0029](0029-the-afg-on-a-persons-record.md), whose picture service had
two choices, OpenAI and Gemini · **Applies to:** `src/afg.rs`,
`src/canva.rs`, `src/comfyui.rs`, `src/afg_record.rs`, `src/routes/afg.rs`,
`src/routes/afg_record.rs`, `src/routes/mod.rs`,
`templates/admin_afg.html`, `static/afg.js`, `static/app.css`,
`locales/*.ftl`, `tests/afg_images.rs`, `tests/rendered_language.rs`

## Context

On 4 November 2026 the operator asked for two more ways to make the
pictures of ADR 0029:

- **Canva**, through its MCP server: the operator has a Canva Pro account,
  and its AI retouches pictures. The CMS itself talks to the MCP server,
  with no language model between them.
- **Ollama and other models of the family's own**, served on its own
  machines.

Six things were found before writing it, against Canva's real server with a
fictional photograph:

1. **The server.** `https://mcp.canva.com/mcp` speaks MCP over plain JSON
   (protocol 2025-06-18, no session). Its OAuth 2.1 authorization server
   registers a public client on request (RFC 7591), with PKCE S256 and a
   return address on `localhost` over plain HTTP.
2. **A retouch.** `generate-image` restores a photograph given as a
   reference: it is uploaded first, through `create-upload-url`.
3. **Only a thumbnail comes back.** `generate-image` returns a thumbnail
   (157 × 200 for a picture of 720 × 912). Its signed address refuses any
   other size. The picture at its full size comes only out of a design's
   export.
4. **Credits.** `generate-image` and `create-design` spend the account's AI
   credits. Canva holds them back for a while when they come too fast:
   `quota_cooldown`, said in the tool's answer.
5. **Canva's usage policy** for its MCP (§6.2, §6.3) says a shared agent must
   not use another person's Canva token, and expects each person to connect
   their own account. The operator decided otherwise: *the AFG is the single
   agent that edits images, so it answers the request as that single agent.*
   One connection serves the family, and the operator takes that on.
6. **Ollama** generates pictures, experimentally, since January 2026. Its
   documentation (OpenAI compatibility, November 2026) offers no editing of
   a picture it is given.

## Decision

### 1. Seven picture services

In the AFG room's Images section, each service asks only for the fields it
uses. afg.js shows those of the service chosen; without scripting, every
field is shown.

| Service | What it asks for |
| --- | --- |
| None | Nothing |
| OpenAI | An address (empty is the usual one), a key, a model |
| Google Gemini | An address, a key, a model |
| **Canva** | A connection (§2) |
| **Ollama** (experimental) | An address (`http://127.0.0.1:11434/v1` by default), a model (`x/flux2-klein` suggested) |
| **A server of your own** (OpenAI-compatible) | Its address (required), a key if it wants one, a model (`Qwen/Qwen-Image-Edit` or `black-forest-labs/FLUX.1-Kontext-dev` suggested) |
| **ComfyUI** | An address (`http://127.0.0.1:8188` by default), a workflow (§4) |

A key or a Canva account belongs to the service it was given for. Choosing
another service forgets it, unless a new key comes with the change. "None"
forgets everything.

### 2. Canva

**Connecting.** "Connect Canva" works as follows:

1. This site registers itself with Canva the first time, as the client
   `ax-genealogy`, returning to
   `http://localhost:<port>/admin/afg/canva/callback`.
2. The room shows the way to Canva: the authorization address with its PKCE
   challenge and a state.
3. Once the administrator accepts, Canva sends them back to that address:
   - when their browser reaches this site at `localhost` (on the machine
     itself, or through a tunnel), the connection ends by itself;
   - otherwise the page does not load, and they paste its address into the
     room.
4. The code is exchanged for tokens, and the account's name is asked for
   the room.

The tokens are kept with the AFG's settings in `family.acl`. No page shows
them. A token within a minute of its end is renewed before use. A connection
started more than 20 minutes ago, or not started by this server, is refused.
"Disconnect Canva" forgets the tokens and keeps the registration.

**A retouch** takes these steps, one at a time for the whole site:

0. **The base design**, a page that every picture is laid on in turn, is made
   once with `create-design` and filed. It is made before a picture is asked
   for, so that a cooldown there costs no picture's credit.
1. **Upload.** `create-upload-url`, then the photograph is posted to it.
2. **Restoration.** `generate-image`, with the restoration instruction of
   ADR 0029 and the photograph as its reference, then
   `get-generate-image-job` until it is done.
3. **The page.** `get-assets` gives the picture's size. One editing
   transaction then empties the base design's page (its texts and laid-on
   images; its own background stays) and lays the picture in its top-left
   corner, as large as the page allows.
4. **The file.** `export-design` exports the page as PNG, at the scale that
   gives the picture its own size. The picture is cut out of it here and
   kept as a JPEG.
5. **Filing.** The picture is moved to the account's folder `ax-genealogy`,
   which is made when there is none.

A refusal is said in Canva's own words: `quota_cooldown`, a failed job, a
refused token. After every retouch, whether it succeeded or not, the
connection is kept as it now stands: a renewed token may have made the one
before it worthless.

**The single connection** follows the operator's decision of 4 November
2026. The room says it beside the buttons: one connection serves the whole
family, through this account's credits and folder, and Canva's policy
expects one connection per person.

### 3. Ollama, and a server of the family's own

Both use the code that calls OpenAI, at `<address>/images/edits`, with three
differences:

- the photograph goes as `image`, as OpenAI's own libraries send one
  picture;
- a key goes only when one is set;
- the picture is asked for in base64 (`response_format`), and taken from an
  address when the server gives one instead.

OpenAI itself is still sent `image[]`, as its documentation shows.

Ollama is marked experimental. The choice is ready for the day Ollama edits
a picture it is given. Until then, its refusal is what the page shows.

### 4. ComfyUI

The workflow is pasted in the room in ComfyUI's API format. It must be an
object of nodes, each with its `class_type`, among them at least one
`LoadImage`, and it may weigh at most 1 MB. For each retouch:

1. The photograph is uploaded (`/upload/image`) under a name of its own.
2. Every `LoadImage` node is given the photograph.
3. Every text input holding `{{restore}}` is given the restoration
   instruction, for a workflow whose model takes one.
4. The workflow is queued (`/prompt`) and followed (`/history/<id>`).
5. The first picture it saved is fetched (`/view`): an `output` picture
   first, a temporary one otherwise.

### 5. What the picture says it is

A restored copy's `generator`, and a reworked portrait's, name the service:
`Canva`, `ComfyUI`, `Ollama <model>` or `OpenAI-compatible <model>`.

## Consequences

- **Canva spends the account's credits:** one picture a retouch, and one
  design the first time. A cooldown refuses a retouch for some minutes.
- **What stays in Canva:**
  - the restored pictures and the base design, in the folder `ax-genealogy`;
  - the photographs sent. The MCP server can neither move nor delete them,
    so they stay in the account's uploads.

  The family clears them from Canva when it likes.
- **What leaves the machine.** With Canva, the photograph goes into the
  operator's Canva account. With Ollama, a server of the family's or
  ComfyUI, it stays on the network: the room says which.
- **A machine with a graphics card.** The family's own image models need
  one. The server that runs the site (3.8 GB of memory) cannot be it, so the
  service's address is another machine's.
- **Tests:**
  - every service against fakes (`tests/afg_images.rs`);
  - Canva once against its real server, on 4 November 2026, with a
    fictional photograph and a test connection since forgotten: the picture
    came back at its own size (720 × 912) in 27 seconds, its token renewed
    on the way;
  - Ollama, OpenAI-compatible servers and ComfyUI only against fakes.
