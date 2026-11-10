# ADR 0045 — The AFG takes commands, and an MCP for the family's archive

**Status:** accepted, v1.14.0 (2026-11-10) · **Builds on:** docs/adr/0023–0027 (the AFG,
its brains, the personal assistant), docs/adr/0039 (a person created with
what surrounds them), docs/adr/0012 (roles and the rules of entry),
docs/adr/0001 (the lens) · **Applies to:** `src/ops.rs`, `src/tokens.rs`,
`src/acl.rs`, `src/site.rs`, `src/routes/mcp.rs`, `src/assistant.rs`,
`src/routes/assistant.rs`, `src/routes/account.rs`, `src/routes/afg.rs`,
`src/guide.rs`, `templates/_afg_ops.html`, `templates/_assistant_message.html`,
`templates/assistant.html`, `templates/account.html`,
`templates/admin_afg.html`, `locales/guide/*.md`, `locales/*.ftl`,
`mcp/ax-genealogy/`

## Context

On 9 November 2026 the operator asked for the "mega feature" that brings the
CMS closer to its final aim — a family archive kept simply by talking to the
AFG:

- the AFG given commands: correct, add people, manage the AXGF bundle
  directly — the whole of create, read, update and delete — from the AFG
  section of each member's account;
- "a sort of MCP for the CMS", so that the AFG and the CMS speak;
- a community connector that ChatGPT, Claude or any other AI can download,
  then point at its own site (its address, a token), so that a web AI can
  work on a genealogy site;
- two flows, one inside (the AFG) and one outside (the MCP), using the same
  functions for every object, the same rules;
- the MCP connecting to the CMS with a token;
- on each member's AFG page, a guideline — a runbook in chapters, with
  example prompts for every kind of scenario.

## Decision

### 1. One set of operations

`src/ops.rs` is the one place an operation on the archive is defined, as
JSON (`{"op": "create_person", …}`):

- reads: `search`, `get_person`, `get_family`;
- writes: `create_person` (with parents, siblings and unions, through the
  new-person page's own plan, docs/adr/0039), `update_person` (names,
  nicknames, gender, living, visibility, biography), `set_birth`,
  `set_death`, `add_event`, `update_event`, `delete_event`, `add_union`
  (with its marriage as an event of its own), `add_child`, `remove_child`,
  `delete_union`, and `delete_person` (administrators only: the person
  leaves every family and event first).

A batch — at most sixty operations — is compiled against a working copy of
the archive, one operation after another, so that a later one sees what an
earlier one made: by its `ref` (`"new:1"`), or by its name. Every write is
checked as the record's editor checks its own: `access::check_write` on the
record's people before and after (the role, then the account's branch), then
no reference to a record that is nowhere (`refuse_new_dangling`'s rule). The
writes are folded, one per record, each update carrying the version the
archive held, and committed by `AppState::commit`: version-checked, all or
nothing, validated by the AXGF library, one journal entry a record. The
journal names the account, then how the write came: `karin · AFG`,
`karin · MCP · ChatGPT`. A person is named by id, by a list label, or by a
name exactly one person bears; a name two people bear is refused, never
guessed. Reads and writes alike go through the reader's lens: the working
copy holds only the people they may see, so a person they may not see is
nobody to them — not a name to match, not an id to name — and a family or an
event of nobody they may see is not found.

What a batch does is said in lines of the reader's language, an event's kind
and a union's through the vocabularies the pages already translate; why it
cannot be applied is said by a refusal naming the operation.

### 2. The AFG proposes, the member applies

The AFG keeps the ten ways its computing can come from (docs/adr/0023); none
needs tools. When the project takes the archive and the account may write,
its system prompt carries the operations and says how to propose: a sentence
or two, then one fenced block ```` ```axgf-ops ````, JSON. A viewer's AFG is
told the account reads and does not write. The thread draws each block as a
card — "Proposed changes" — checked now against the archive as the member
would write it (cached while the archive's generation does not change), in
plain words, with **Apply** and **Set aside**; a block that cannot be applied
says why and offers only to set it aside; the JSON stays one click away.
While the AFG writes a block, the live answer shows that it is preparing
changes, not the JSON.

Apply (`POST /account/assistant/:p/c/:c/ops`) compiles the block again
against the archive as it is now and commits it as the account. What became
of it — applied with its lines and the ids its `ref`s took, or set aside —
is a message of its own in the thread (`role: done`), told to the AFG on the
next question as the member's words. A block is applied or set aside once.
Nothing is written without the member's click, and nothing the member could
not write on a page.

### 3. Access tokens

An account makes tokens on its own page, while the MCP is open: a name, and
whether it may write (only an account that may). A token is
`axgf_<8 hex>_<43 characters>` — its id finds it, its 32 random bytes prove
it — shown once, with the MCP's address and the address with the token in
it. The accounts file (mode 600, docs/adr/0011) keeps only its SHA-256, its
name, whether it writes, its creation and the day it was last used (written
once a day at most); ten at most an account; one click revokes it. A token
acts as its account — its role, its branch, its lens — and stops when the
account is disabled. The administrator opens the MCP on the AFG page of the
administration (`Site.mcp`, closed by default); closed, every token is kept
and opens nothing.

### 4. The MCP

`/mcp` speaks the Model Context Protocol over Streamable HTTP: JSON-RPC 2.0
posted, one JSON answer each (a notification is answered 202), protocol
versions 2024-11-05 to 2025-11-25; no stream (`GET` is 405), no session. The
methods are `initialize` (with the operations as the server's instructions),
`ping`, `tools/list`, `tools/call`, and empty resource and prompt lists. A
tool for each operation of §1, with its JSON Schema and its hints (read
only, destructive), plus `check_operations` (a batch, without writing),
`apply_operations` (a batch, all or nothing) and `guide`. A reading token, or
a viewer's, is offered the reads alone; `delete_person` is offered to
administrators. The token comes in `Authorization: Bearer`, or in the
address (`/mcp/t/<token>`) for the assistants that take an address and
nothing else (ChatGPT's and claude.ai's connectors); the session cookie is
never read there, and a post from another site's page is refused as every
other post is. Writes run off the request's thread. Messages are in
English: an assistant reads them and says them in its user's words.

### 5. The community connector

`mcp/ax-genealogy/`: `server.mjs`, a stdio bridge with no dependency
(Node 18), which posts each JSON-RPC line to `AXGF_URL/mcp` with
`AXGF_TOKEN`; `manifest.json`, an MCP bundle for Claude Desktop that asks
for the site's address and the token (marked sensitive); and its README:
Claude Desktop, Claude Code, claude.ai, ChatGPT, Cursor and the others. The
site serves it packed, at this release's version, at
`/mcp/ax-genealogy.mcpb`, linked from the account's tokens; a test keeps its
list of tools the MCP's.

### 6. The guide

On each member's personal assistant, "Keeping the family's genealogy through
the AFG": a lead and twelve chapters — before you start; find and check;
add people; correct a record; births and deaths; unions and children; life
events; work from a document; large work step by step; delete and repair;
from ChatGPT, Claude or another AI; good practice and limits — each with
requests to make, folded, in the reader's language. Each language's guide is
a Markdown file, `locales/guide/<lang>.md`, embedded like the catalogues; a
test keeps every language's chapters the English ones', each with a request.

## Consequences

- The archive can be kept by conversation, inside (the AFG, applied by a
  click) and outside (an AI through the MCP, with a token), by one set of
  functions and one set of rules.
- A token is a password for a machine: shown once, revocable, scoped, never
  stored. Whoever holds one is the account, within what it may.
- What an account may read leaves for the company whose assistant reads it,
  the living included; the administration says so where the MCP is opened.
- What the AFG does not do yet: attach photographs or documents, merge two
  records, write notes. Those stay on the administration's pages.
- The guide's translations other than French and English are machine-made
  and not yet read by a speaker.
