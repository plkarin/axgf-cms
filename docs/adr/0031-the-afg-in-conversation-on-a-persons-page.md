# ADR 0031 — The AFG in conversation on a person's page

**Status:** accepted (unreleased) · **Builds on:**
[ADR 0024](0024-the-afgs-brains.md) (who may talk to the AFG),
[ADR 0026](0026-my-personal-assistant.md) (conversations, the answer as it
is written), [ADR 0029](0029-the-afg-on-a-persons-record.md) (the record as
the AFG reads it) · **Applies to:** `src/person_chat.rs`, `src/assistant.rs`,
`src/person.rs`, `src/render.rs`, `src/routes/person_chat.rs`,
`src/routes/assistant.rs`, `src/routes/public.rs`, `src/routes/mod.rs`,
`templates/person.html`, `templates/_person_afg.html`,
`templates/_assistant_message.html`, `templates/_assistant_composer.html`,
`static/app.css`, `locales/*.ftl`, `tests/person_chat.rs`,
`tests/interactive_walk.rs`, `tests/rendered_language.rs`

## Context

On 4 November 2026 the operator asked for the AFG on each person's page, as
a chatbot that answers questions about that person:

- its context is the person's AXGF records and, more broadly, the family's,
  so that it can look into a question about the person or about others. The
  operator's example: *in which regiment was he at the battle his notes
  mention?*;
- each member's conversations are kept on the person's page, so that they
  can be taken up again over time, or new ones begun;
- the members manage their conversations there: they rename and delete them.

## Decision

### 1. Where it is, and for whom

The record page has an **AFG** tab, last of its tabs, for a signed-in
account the AFG is open to (`User.afg`, ADR 0024 §5). Its count is the
number of that account's conversations about the person. Nobody else has
the tab, and the routes refuse them:

- a reader who is not signed in is sent to sign in;
- an account the AFG is not open to is told so (403);
- a person the account may not read answers 404, as on every page.

The tab is a link, `/person/<id>?tab=afg`, and a conversation on it is
`?tab=afg&c=<conversation>`. Without scripting, the tab works with forms
and asks again every three seconds while the AFG writes. With scripting,
the assistant's own script (`static/assistant.js`) sends the question while
the page stays and shows the answer as it is written. The tab is not drawn
beside the river: it is the record page's.

### 2. What the AFG reads: the dossier

With every message, the AFG is given a dossier made through the **account's
own lens**, so that it reads nothing of the family that the member could not
read on the site (`src/person_chat.rs`):

| Part | What | Ceiling |
|---|---|---|
| The record | The person's page as the summary of ADR 0029 reads it: identity, dates, events, family, occupations, places, sources, the profile's groups (the sensitive ones only when the AFG room allows them), without files | none |
| The notes | Every note about the person the member may read, their transcriptions and their summary included | 40,000 characters |
| The close relatives | Parents, spouses, children and siblings the member may read, each with its record (without the profile) and the notes about it | 40,000 characters of records, 30,000 of notes |
| The family | Everyone the member may read, one line each: name, lifespan, id | 40,000 characters |

A part that runs over its ceiling is cut, and the dossier says what was left
out. The tab says, before the first question, how many notes, relatives and
persons go, and about how many tokens.

The instructions tell the AFG:

- whom it is talking to, in which language, and about whom;
- to say what the dossier says and where it says it: a note's title, an
  event, a source;
- to say plainly when it goes beyond the dossier (history, places, military
  units, registers), and how sure it is;
- never to invent a fact about the family, and to say where it might be
  found when the dossier does not say;
- to link the family's persons it names, `[name](/person/<id>)`. The page
  draws such a link, as it draws any link to this site (`notes::to_html`).

The member's AFG brain (ADR 0024) is neither read nor written here, and the
AFG keeps no second brain for the person: a conversation carries itself, its
last 40 messages within 80,000 characters going back with each question
(ADR 0026 §4).

### 3. Where the conversations are kept

In the account's assistant space (ADR 0026 §2), in a project of the person's
own:

```text
family.assistant/<username>/projects/<id>/
  project.json            "person": the person's id; its name as the page shows it
  conversations/<c>.json  the conversation's title (its first question), dates
  conversations/<c>.jsonl its messages
```

- **The project's id comes from the person's** (the first 16 hexadecimal
  digits of `SHA-256("person:" + id)`). The page finds it without a list,
  and it is made at the first question, never by looking at the tab.
- **The assistant does not list it** among its projects.
- **It is backed up with the assistants**, apart from the bundle (ADR 0027).
- **Words only.** The person's page takes no file with a question: the
  dossier is what the AFG reads.

### 4. Managing them

On the tab, the account's conversations about the person, the most recent
first. Each is a bubble that opens it: a raised surface with its border, a
speech bubble's tail, and an arrow. The personal assistant's projects show
theirs the same way (the operator's request of 5 November 2026). Each opens
on its thread. From there it is carried on, renamed, or
deleted once confirmed. All of them can be deleted at once, once confirmed:
the person's project goes, whole. An answer that failed, or a question left
without one because the site stopped, is asked again, as in the assistant.

## Consequences

- **What leaves the machine.** Every question sends the dossier, as the
  member may read it, and the conversation so far to where the AFG's
  computing runs. The administrators chose it, and the AFG room's warning
  holds here too (ADR 0023). A member who may read the living sends what
  they may read of them.
- **Cost.** The dossier is sent again with every message: at most about
  50,000 tokens, usually far fewer. A model with a small window, run at
  home, may not take a large family's whole index.
- **No new setting.** The tab follows the AFG's computing and each account's
  access, as the assistant does.
- **Tests:** `tests/person_chat.rs`, against a fake Ollama that keeps what
  it was sent: the dossier holds the record, the notes, the close relatives
  and the family, and never a person the member may not read; nobody reads
  another account's conversations; renaming and deleting; nothing is sent
  without the AFG's computing.
