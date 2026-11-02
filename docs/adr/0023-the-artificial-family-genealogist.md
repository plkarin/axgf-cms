# ADR 0023 — The Artificial family genealogist: where its computing comes from

**Status:** accepted, v1.1.0 (2026-11-02); amended by
[ADR 0025](0025-the-models-of-the-moment.md), which suggests the models of the
moment and gives every kind an effort · **Builds on:** the administration
console (docs/adr/0011), the shared export (`state::export_copy`), the
sign-in services' secrets (`src/site.rs`) · **Applies to:** `src/afg.rs`,
`src/routes/afg.rs`, `src/routes/console.rs`, `src/routes/mod.rs`,
`src/site.rs`, `src/state.rs`, `templates/admin_afg.html`,
`templates/admin_afg_run.html`, `templates/admin_dashboard.html`,
`templates/_admin_nav.html`, `templates/_icons.html`, `static/afg.js`,
`locales/*.ftl`, `tests/afg.rs`

## Context

On 2 November 2026 the operator asked for "l'Artificial family genealogist,
l'AFG": an agent that reads the family's archive and answers about it. They
split the work in two. The second part will bring it to the pages, one use
at a time. This first part sets up where its computing comes from, in the
administration, by one of five ways:

- an address (Ollama);
- an address, an API key and a model (OpenAI, Gemini, Claude, vLLM,
  Mistral);
- Claude Code on this machine, with a model and an effort;
- Codex on this machine, with a model and an effort;
- OpenCode on this machine, with a model and an effort.

A local agent is taken to be signed in already, and each way must respect
its agent's own way of being called. A **test** button sends the AFG the
archive as it is, in AXGF, every JSON file of it and no photograph, and asks:
"Donne-moi le top 3 des familles disposant du plus grand nombre d'individus
déclarés". If the archive is empty, the AFG answers "Bundle vide".

## Decision

### 1. A sixth room

`/admin/afg` joins the administration strip (`ADMIN_NAV` rises to six in
`tests/interactive_walk.rs`). The overview gains a matching tile. The room
is for **administrators only**: it holds an API key, it can start a program
on this machine, and its test sends the whole archive away.

### 2. Ten kinds in three groups

| Kind | Spoken to as | Uses |
|---|---|---|
| Ollama | `POST {address}/api/chat`, `stream: false`, `options.num_ctx` sized to the archive (8,192 to 131,072) | address, model (optional: the first one `GET /api/tags` lists) |
| OpenAI, Mistral, vLLM | `POST {address}/chat/completions`, `Authorization: Bearer` | address, key (optional for vLLM), model |
| Claude (Anthropic API) | `POST {address}/v1/messages`, `x-api-key`, `anthropic-version: 2023-06-01`, `max_tokens` 8,192 | address, key, model |
| Gemini | `POST {address}/v1beta/models/{model}:generateContent`, `x-goog-api-key` | address, key, model |
| Claude Code | `claude -p --output-format json`, archive on standard input | model, effort (`low` to `max`), program |
| Codex | `codex exec … -`, archive on standard input, answer from `--output-last-message` | model, effort (`model_reasoning_effort`, `minimal` to `xhigh`), program |
| OpenCode | `opencode run <message> --file <archive>` | model (`provider/model`), effort (`--variant`), program |

Each address defaults to the service's usual one (Ollama
`http://127.0.0.1:11434`, vLLM `http://127.0.0.1:8000/v1`, and the public
APIs). The page shows the address it will call. Each answer, with its usage
when the service reports it, is read in the service's own shape. A
refusal is reported in the service's own words.

### 3. What is sent

- **The archive**, as an export writes it (`export_to_bytes`), read back out
  of the ZIP. Every JSON file goes, compacted, manifest first, each headed
  `=== path ===`. No photograph or scan is sent.
- **The sensitive classes** (health, biometrics, genomics, legal records,
  behaviour) are left out, as a shared export leaves them out, unless the
  administrator ticks the box.
- **The size is said before sending**: files, kilobytes, and tokens at about
  three bytes each. A family of a thousand people is over a megabyte of
  JSON, some 400,000 tokens. That is more than many models read at once,
  and the service's own refusal says so.
- **The prompt.** A system prompt tells the AFG what it is reading. It must
  answer from the archive only, in the question's language, and name the
  records the answer rests on. If the archive holds no person and no family,
  it must answer exactly the empty-archive words.
- **The question and those words** are in the administrator's language:
  "Donne-moi le top 3 des familles disposant du plus grand nombre
  d'individus déclarés" and "Bundle vide" in French.

It is the AFG that answers "Bundle vide", as the operator asked. The site
does not answer in its place, because the test is a test of the AFG.

### 4. An agent is run as its own command line, and given nothing to act with

The agent is started directly, with its arguments and never through a
shell. The archive goes on its standard input, or into a file in a private
working directory: `.afg-<uuid>` beside the bundle, mode 700, removed
afterwards. A leftover from a process stopped mid-test is removed by the next
test. The archive never goes on a command line, which is too short for it.

The archive is text that every contributor can write, and a note can say
anything. So the agent is given nothing it could read with or reach out
with:

- **Claude Code:**
  - `--tools ""` (no tool at all);
  - `--safe-mode` (no CLAUDE.md, skill, hook or plugin of the account);
  - `--strict-mcp-config` (no MCP server);
  - `--no-session-persistence`.
- **Codex:**
  - `-c features.shell_tool=false` (no shell);
  - `-c web_search="disabled"`;
  - `--ignore-user-config` (not the account's `config.toml` or its MCP
    servers; its sign-in still comes from `CODEX_HOME`);
  - `--sandbox read-only`, `--ephemeral`.
- **OpenCode:**
  - `OPENCODE_PERMISSION={"*":"deny","read":"allow"}`, which denies every
    tool but reading. Its attachment is read that way, and only inside its
    working directory, because `external_directory` is denied with the rest;
  - `OPENCODE_PURE` and the variables that turn off project files, Claude
    skills, external skills, updates and LSP downloads.

What every version knows is always passed: no tool for Claude Code, no
shell for Codex. A version that does not know these says so in the test's
failure. Some options only came with later versions: `--safe-mode` and
`--permission-prompts` for Claude Code (2.1.131 knows neither), and
`--ignore-user-config` and `--ephemeral` for Codex. They are passed when the
program's own help names them (`claude --help`, `codex exec --help`), read
before each test.

The agent inherits the service's environment, so it can be configured
through it, but not this site's own settings: every `AXGF_*` variable is
removed. The emergency token among them.

The program is found on the search path and wherever its installer puts it,
or given by its full path. A configured path must end in the agent's own
name (`claude`, `codex`, `opencode`): this field never runs anything else.

### 5. The key is kept as the sign-in services' secrets are

- **Where it is kept:** in `family.acl`, mode 600, under the site settings.
  It is never in the bundle, and always in a backup.
- **On the page:** it is never shown, only its last four characters. A
  blank field keeps it, and "Forget the key" clears it.
- **In the journal:** who changed where the archive may be sent, the kind
  and the model are recorded. The key never is.

### 6. A test runs on its own

Pressing the button opens the test's page, which asks again every three
seconds until the answer is there. (Since docs/adr/0024 the result opens in
a window over the room instead, success or error, and the test goes through
the asker's brain.) The question and what was sent are on
that page, then the answer and its usage, or the failure with the end of
what the program said on its error output. One test runs at a time. The
last ten are kept for the life of the process. The time allowed is 600
seconds unless set otherwise (30 to 3,600).

## Consequences

- **The installed service's sandbox allows only a model on this machine.**
  The unit (`deploy/bootstrap.sh`) reaches localhost only
  (`IPAddressDeny=any`, `IPAddressAllow=localhost`). Its memory is never
  both written and run (`MemoryDenyWriteExecute=yes`), and that stops
  Claude Code and OpenCode, which are built with Bun. It cannot see `/home`.
  - An Ollama or a vLLM on the same machine works as installed.
  - A service on the internet, or an agent, needs the drop-in that
    docs/OPERATOR.md gives. It is the operator's decision, deliberately not
    the installer's.
- **Living people's records leave the machine** with a service on the
  internet, or with an agent that calls one. The room says so above the
  button. The sensitive classes stay out unless chosen.
- **Not in this part:** the AFG on the pages (the second part), answers
  streamed as they come, and a cost for anything but Claude Code, which is
  the only one to report its own.
