# ADR 0026 — My personal assistant

**Status:** proposed (2026-11-03) · **Builds on:** docs/adr/0023 (where the
AFG's computing comes from), docs/adr/0024 (brains, and who may talk to the
AFG), docs/adr/0025 (models and efforts) · **Applies to:**
`src/assistant.rs`, `src/afg_chat.rs`, `src/avatar_pack.rs`, `src/afg.rs`,
`src/brain.rs`, `src/render.rs`, `src/state.rs`, `src/routes/assistant.rs`,
`src/routes/mod.rs`, `src/routes/public.rs`, `templates/assistant.html`,
`templates/_assistant_*.html`, `templates/account.html`,
`templates/base.html`, `templates/_icons.html`, `static/assistant.js`,
`static/app.css`, `static/avatars/`, `locales/*.ftl`, `tests/assistant.rs`,
`tests/interactive_walk.rs`, `tests/rendered_language.rs`,
`tests/design_tokens.rs`, `tests/common/mod.rs`

## Context

On 3 November 2026 the operator asked that every account, from its own
account page, have a chat with the AFG on a page called **My personal
assistant**:

- **Projects** the member creates and names.
- **Conversations** in each project, each opening on a first question. In
  one, the member reads the exchanges in a dialogue, as on Claude's web
  interface, and carries on from there. A message carries documents,
  photographs and media as well as words, but no microphone.
- **A code of conduct** for each project, like the instructions of a
  project on Claude's web interface.
- **A second brain** for each project, made from its messages, and the
  memory of each conversation, so that a conversation can be taken up again.
- **An interface in Claude's codes, very minimal**: smooth, pleasant, and as
  beautiful as Canva's pictures and effects can make it.
- **Avatars** on projects and conversations: a picture of the member's own,
  or one of a pack of eighty made for the site, covering domains such as
  medicine, work and genealogy.

## Decision

### 1. Whose it is

The page belongs to a signed-in account the AFG is open to (`User.afg`,
docs/adr/0024 §5). Others see why it is not theirs:

- the emergency session has no account, so it has no assistant;
- an account the AFG is not open to is told that an administrator decides
  it.

It is reached from the account page, where a card at the top opens it, and
from the account menu in the masthead.

**Everything in it is the account's own.** The space is always the
session's. A form or an address never names whose space it is. A project, a
conversation or a file that is not the account's answers 404, exactly as one
that does not exist. Administrators see only their own space.

### 2. Where it is kept

```text
family.assistant/                beside family.axgf, as family.brains is
  <username>/                    mode 700
    owner.json                   the account's id and name
    projects/<project>/
      project.json               name, avatar, code of conduct, archive or not
      memory.md                  the second brain, one fact a line, dated
      avatar.jpg                 a picture of the account's own, 256 px
      conversations/<c>.json     title (its first question), avatar, dates
      conversations/<c>.jsonl    its messages, one JSON object a line
      conversations/<c>.jpg      its own picture, when given one
      files/<f>, files/<f>.json  what a message carried, and what it is
```

- **Files are mode 600.** Ids are 16 hexadecimal digits and are checked
  before they become a path.
- **A space is its account's, not its name's.** A folder whose `owner.json`
  names another account is set aside, as a brain is.
- **It is not the account's AFG brain.** Forgetting that brain on the
  account page leaves the projects as they are.
- **It is not in the nightly backup either, as brains are not.** Whether
  both should be is the operator's decision.

### 3. A project

- **Name and avatar.**
- **Code of conduct.** Up to 8,000 characters. It is put in the AFG's
  instructions inside a `<code-of-conduct>` block. The AFG follows it unless
  it asks the AFG to deceive the member or to cause harm.
- **Second brain.** The AFG writes to it the way it writes to a brain: it
  ends an answer with a `<memory>` block, which is taken out of the answer
  and kept, dated. Every message of the project takes the most recent 16 KiB
  of it back to the AFG. The member reads it, forgets a fact, or forgets all
  of it.
- **The family's archive.** It goes with the project's messages only when
  the member says so. **The default is off.** The archive is every record,
  the living included, sent again with every message: often hundreds of
  thousands of tokens. The project's settings say how large it is.
- **Deletion.** A project goes whole, once the deletion is confirmed.

### 4. A conversation

- **Its title is its first question**, cut to 120 characters. It can be
  renamed and given its own avatar.
- **What the AFG is given** with each message:
  - its instructions (the project's code of conduct and memory);
  - the archive, when the project takes it;
  - the conversation's last 40 messages, within 80,000 characters.
- **Files in the history.** The files of the member's last three messages
  go whole. Older ones are named, not sent again.
- **Errors stay in the thread.** An answer that failed shows there, with
  what the service or the program said and a way to ask again. Errors are
  never sent to the AFG.
- **A message left without an answer**, because the site stopped while the
  AFG wrote, is offered again the same way. Only such a message, or one
  whose answer failed, is asked again.

### 5. Files

At most 6 files a message, 20 MB each, 40 MB in all. What a file is comes
from its bytes, never from its name, except a text file's flavour:

| Kind | What the AFG reads |
|---|---|
| Picture (PNG, JPEG, GIF, WebP) | Every service, sent at no more than 1,568 px on its longest side. Claude Code (a content block) and Codex (`--image`) read it too. Ollama and vLLM read it only with a vision model. |
| PDF | Claude (a `document` block), Gemini, OpenAI (a `file` part), Claude Code, OpenCode |
| Text (`.txt`, `.md`, `.csv`, `.json`, `.ged`…) | Every one, in the message's words, up to 100,000 characters |
| Sound, film | Gemini only |
| Anything else | Kept and offered back to the member; the AFG is told only that it was sent |

When the chosen computing cannot read a file, the AFG is told in words that
the file was sent and cannot be read here, so that it says so rather than
guess.

A file is served only to its account: pictures inline, everything else as
a download, always with `X-Content-Type-Options: nosniff` and a sandboxing
`Content-Security-Policy`.

### 6. The answer, as it is written

Every way of computing is asked to stream (`src/afg_chat.rs`):

- Ollama, one JSON object a line;
- OpenAI, Mistral and vLLM, server-sent events;
- Claude, `content_block_delta` events;
- Gemini, `streamGenerateContent?alt=sse`;
- Claude Code, `--output-format stream-json --include-partial-messages` when
  the installed version knows it, with the pictures and documents as content
  blocks on its input (`--input-format stream-json`).

Codex and OpenCode give their answer whole, at the end.

- **Showing the answer as it comes.** Each piece goes into the answer being
  written, held in memory. The page asks for what is new every 0.65 seconds
  and shows the pieces as they come, with a blinking caret.
- **The memory block is never shown,** even while it is being written.
- **Without scripting,** the page asks again every three seconds.
- **Limits.** A conversation is answered one message at a time. The site
  answers at most four at once.
- **The archive is kept off the command line.** It never goes on an agent's
  command line, which could not carry it: Claude Code reads it on its input,
  OpenCode in a file.
- **Concurrent runs.** Each agent runs in its own working directory. Only
  directories more than six hours old are cleared, since several
  conversations may be answered at once.

### 7. The page

The page takes the codes of Claude's web interface, minimal:

- **Layout.** A column of projects. A project's page: its composer to start
  a conversation, its conversations, and three closed panels (code of
  conduct, memory, settings). A conversation's page: the thread, and a
  composer that stays at the foot of the window.
- **Composer.**
  - Enter sends and Shift+Enter starts a new line (on a touch screen, Enter
    is a new line).
  - Files are dropped, pasted or chosen, and shown as chips that can be
    removed before sending.
  - The message is sent while the page stays.
- **Effects.**
  - A message appears by sliding up into place.
  - The AFG's mark turns while it writes, and three dots wait for its first
    words.
  - Avatars float on the empty page.
  - All of it stops for a reader who asked for less movement.
- **The site's rules hold.** Every colour is the theme's, so the page works
  under all six themes. There is one primary button a page. Every field has
  a visible label and a focus ring.
- **Budget.** The assistant counts as an edit screen
  (`tests/interactive_walk.rs`): its member writes there.

### 8. Avatars

- **The pack.** Eighty pictures were made for this site with Canva
  (`generate-image`), four for each of twenty domains: genealogy, health,
  work, money, law, learning, home, cooking, travel, nature, well-being,
  family life, arts, sport, technology, science, history, paperwork, animals,
  celebrations and rites.
- **The style.** One symbol on a gradient of its domain's colour, in one
  flat style.
- **The files.** Each is a 192 px WebP, embedded in the binary
  (`src/avatar_pack.rs`, `/static/avatars/<name>.webp`).
- **The default.** A project or a conversation with no avatar chosen takes
  the one its id picks, the same every time (FNV-1a).
- **The member's own picture** is cut square from its middle and kept as a
  256 px JPEG.

## Consequences

- **What leaves the machine.** Every message sends the conversation's recent
  history, its files and the project's memory to wherever the AFG's
  computing runs, and the family's archive too when the project takes it.
  The member chooses the archive; the operator chose the computing. The
  privacy warning of the AFG room (docs/adr/0023) applies here too.
- **Cost grows with the conversation.** The history is sent again with each
  message, within its limits. The archive is the heaviest part, and is off
  by default.
- **Not in the backups.** The projects, their files and their memories are
  in no nightly backup until the operator decides they should be.
- **Not in this part:** sharing a project with another account, a search
  across conversations, and voice. There is no microphone, as asked.
