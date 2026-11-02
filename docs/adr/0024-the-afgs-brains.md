# ADR 0024 — The AFG's brains, its window, and who may talk to it

**Status:** accepted, v1.1.0 (2026-11-02) · **Builds on:** docs/adr/0023 (where the
AFG's computing comes from, and its test) · **Applies to:** `src/brain.rs`,
`src/afg.rs`, `src/acl.rs`, `src/render.rs`, `src/routes/afg.rs`,
`src/routes/admin.rs`, `src/routes/account.rs`, `templates/account.html`,
`templates/admin_afg.html`,
`templates/_afg_result.html`, `templates/admin_users.html`,
`templates/_icons.html`, `static/afg.js`, `static/app.css`,
`locales/*.ftl`, `tests/afg.rs`

## Context

The operator, the same day as ADR 0023, asked for four things:

- **The test's result in a window.** It should open as a popup, success or
  error, with what the model wrote, drawn rather than raw.
- **A brain for each person.** "Une logique de memory brain per user": a
  memory on disk for each person who asks. Every account that starts a
  conversation with the model gets a brain folder under its name, and that
  folder is the model's memory of them.
- **Two kinds of knowledge.** The bundle is what the model knows of the
  family. The brain is what it remembers of the person in front of it.
- **Who may talk to the AFG** is a simple yes or no for each account, set on
  the accounts page.

Later the same day the operator asked that every account see this
"secondary brain" of its AFG on its own account page, and be able to remove
some subjects from it, or not (§6).

Where the AFG appears, and what it helps with there, is the next step.

## Decision

### 1. Two kinds of knowledge

| | What it is | Where it lives | Who reads it |
|---|---|---|---|
| **The archive** | What the AFG knows of the family: every JSON file of the AXGF bundle (docs/adr/0023 §3) | `family.axgf` | Every exchange, whoever asks |
| **A brain** | What the AFG remembers of one person: what it chose to note about them, and its last exchanges with them | `family.brains/<username>/` | That person's exchanges only |

The system prompt says so. The bundle comes first. The asker's brain follows
it, then the question:

```text
=== brain: marie ===
You are talking to marie, who says they are … (persons/<id>.json) in the archive.
What you noted about them:
- [2026-11-02] Researches the Exemple branch
Your last exchanges with them, oldest first:
--- 2026-11-02T10:00:00Z, test
Question: …
Answer: …
```

The person comes from the account's link to the tree (`person_id`, the
account's own word, 2026-10-23), when there is one.

### 2. A brain is a folder, its account's

```text
family.brains/            beside family.axgf, as family.acl is
  <username>/             one per account, under its name, mode 700
    brain.json            whose brain this is: the account's id and name
    memory.md             what the AFG chose to keep, one fact a line
    conversations.jsonl   every exchange, one JSON object a line
```

- **When it is made.** At the account's first exchange: today, the first
  test it runs.
- **Its name.** The folder takes the username. A username's alphabet is safe
  for a path, but `..` is a valid username, so a name that starts with a dot
  gets a `_` in front.
- **Its owner.** A brain belongs to its account, not to its name. Suppose a
  folder's `brain.json` names another account, because an account was
  deleted and its name given again. That folder is moved aside whole, under
  `<name>@<its account>` (a name no username can take), and is never read to
  the newcomer.
- **Its files** are mode 600, written whole and renamed into place, or added
  to.

### 3. How the AFG remembers

Every kind of computing can write text. Not all of them can write files, and
none is given a tool (docs/adr/0023 §4). So the AFG keeps something the one
way all ten can.

- **How it asks.** It ends its answer with a `<memory>` block, one new fact
  per line.
- **What the site does with the block** (`brain::take_memory`):
  - takes it out of the answer, with the code fence a model may put it in;
  - takes the bullets and numbering off its lines;
  - keeps at most 20 facts, of 400 characters each.
- **What it keeps.** Facts are dated and kept once: one already held is not
  added again. Past 64 KiB, the oldest facts go first.
- **What goes back with the next question.** The last 16 KiB of `memory.md`,
  and the last five exchanges, each answer cut at 1,500 characters.
- **Every exchange is logged**, with the context it was asked in:
  - the question;
  - the answer as shown;
  - the model;
  - what was kept.

  The context is `test` for the room's test, and the pages to come will name
  themselves.
- **No account, no brain.** A session opened with the emergency token has no
  account. It is answered without a brain, and the window says so.

### 4. The result in a window

- **The flow.** The test button posts as before. The room then opens on
  `/admin/afg?run=<id>`, with the result in a `<dialog>`:
  - waiting;
  - then **success**, with the answer's Markdown drawn as a note's is
    (`notes::to_html`: raw HTML shown as text, links only to the web);
  - or **error**, with what the service or the agent said.
- **What success shows.** The model, the tokens and, for Claude Code, the
  cost. Then what was kept in the asker's brain, or that nothing new was.
- **With scripting** the window is modal. While the answer is awaited,
  `afg.js` asks `GET /admin/afg/run/<id>`, which returns
  `{"state", "html"}`, the window drawn by the same template. Closing the
  window draws the room afresh, its last test and its brain with it.
- **Without scripting** the window is open in place, and the room asks again
  every three seconds while the answer is awaited (`<noscript>` around the
  refresh, so that scripted pages are not reloaded).
- **The "Last test" tile** opens the window again.

### 5. Who may talk to the AFG

- **The flag.** `afg: bool` on the account (`family.acl`), off until an
  administrator ticks it on the account's row of the accounts page. The page
  says how many accounts may.
- **What it does today.** Changing it closes the account's sessions, as any
  change to an account does. Nothing reads it yet: it is for the pages the
  AFG will come to.
- **The administrator's test does not ask for it.** It is a check of the
  configuration, not a conversation.

### 6. Each account sees its brain, and forgets what it chooses

*My account* (`/account`) has a section, **Your AFG's brain**. It shows when
the account may talk to the AFG, or when the AFG remembers something of it.

- **What it shows.**
  - The **facts** kept, oldest first, each with its date.
  - The **exchanges**, most recent first (the last 200, with how many older
    ones are kept). Each shows its date, where it was asked, the question,
    and the answer drawn, folded.
- **Forgetting one thing.** Each line has its **Forget** button, posted to
  `POST /account/brain/forget` with the line's mark: the first 16 hex digits
  of the line's SHA-256, never the text itself.
  - A fact forgotten is no longer told to the AFG.
  - An exchange forgotten leaves `conversations.jsonl`, and so no longer
    goes back with the last exchanges.
  - Forgetting an exchange does not forget the facts it gave. They are
    their own lines.
- **Forgetting everything** asks for a box to be ticked, then removes the
  brain's folder. The next exchange starts a new one.
- **Whose brain.** It is always the session's account's, never a field of
  the form. Another account's mark forgets nothing, and says it was
  already forgotten. Administrators see no one's brain but their own here.
  The emergency token has none.
- **Keeping and forgetting together.** One writer at a time rewrites a
  brain, keeping or forgetting, so that an answer kept while a fact is
  forgotten loses neither. The journal records that an account forgot, and
  what kind of thing, never what.

## Consequences

- **Brains are kept beside the archive and are not in its backups.** The
  nightly backup holds the bundle, the accounts and the journal. Brains are
  for the operator to add to it, or to copy themselves.
- **A brain is personal data about a living person.** It is readable only by
  the service's user. Its owner forgets from their account page (§6). The
  operator can still delete a folder by hand.
- **The memory is the model's own wording.** What it chooses to keep can be
  wrong. The window shows what was kept after every answer.
- **The room's own run page is gone.** Its address now answers the window's
  script, in JSON.
