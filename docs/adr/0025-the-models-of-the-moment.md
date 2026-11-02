# ADR 0025 — The AFG's models of the moment, and their efforts

**Status:** proposed (2026-11-02) · **Builds on:** docs/adr/0023 (where the
AFG's computing comes from) · **Applies to:** `src/afg_models.rs`,
`src/afg.rs`, `src/routes/afg.rs`, `templates/admin_afg.html`,
`static/afg.js`, `static/app.css`, `locales/*.ftl`, `tests/afg.rs`

## Context

After the first tests, the operator asked for lists of suggestions in the
AFG room: for each way of computing, the models of the moment of each
maker, and for each model the efforts it takes. A name or an effort outside
the lists must always stay possible.

The same evening a test of Claude Code had been given the model `opus-5`,
which is not a model's name. Claude Code passes a name it does not know to
the API as it is, so nothing in the room had said so before the test.

Until then the effort meant something to the three agents only. It was
checked against a fixed list for each one, and the lists had aged: Codex
now takes `max` and `ultra`, and no longer `minimal` for its current
models.

## Decision

### 1. A catalogue in the code, read again for each release

`src/afg_models.rs` holds, for each way of computing, the models suggested
(the one to start with first), with for each one:

- its name, as the service or the agent writes it;
- its maker;
- the efforts it takes, the lightest first;
- the effort it takes when none is given, when its maker says.

It is data, not a lookup: the room never calls a maker to learn its models,
and a model a maker adds tomorrow is written in the field by hand until the
next release lists it. The list as of 2 November 2026:

| Way | Models suggested | Efforts |
|---|---|---|
| Ollama | `qwen3.8`, `gemma4`, `gpt-oss:20b`, `gpt-oss:120b` | `think`: `false`, `true`; gpt-oss `low`, `medium`, `high` |
| OpenAI | `gpt-6.1-sol`, `gpt-6-astra`, `gpt-6.1-terra`, `gpt-6-luna` | `low` to `max`; Terra and Luna `none` too |
| Mistral | `mistral-large-4-0`, `mistral-small-2603`, `zai-glm-5-3` | `low`, `medium`, `high`; `none`, `high`; `low`, `high`, `max` |
| vLLM | `openai/gpt-oss-120b`, `openai/gpt-oss-20b`, `Qwen/Qwen3.8-27B`, `mistralai/Mistral-Small-4-119B-2603`, `google/gemma-4-31B-it`, `deepseek-ai/DeepSeek-V4.1-Flash` | gpt-oss `low`, `medium`, `high`; Mistral Small 4 `none`, `high`; the others as their template says |
| Claude (Anthropic API) | `claude-opus-5-5`, `claude-fable-5-1`, `claude-sonnet-5-5`, `claude-haiku-5-5` | `low`, `medium`, `high`, `xhigh`, `max` |
| Gemini | `gemini-4-argon`, `gemini-3.8-flash`, `gemini-3.6-flash`, `gemini-3.5-flash-lite` | `low`, `medium`, `high`; the last two `minimal` too |
| Claude Code | the four Claude models, then the aliases `opus`, `fable`, `sonnet`, `haiku` | `low` to `max` |
| Codex | `gpt-6.1-sol`, `gpt-6-astra`, `gpt-6.1-terra`, `gpt-6-luna` | `low` to `max`, and `ultra` but for Luna |
| OpenCode | `anthropic/claude-opus-5-5`, `anthropic/claude-sonnet-5-5`, `openai/gpt-6.1-sol`, `google/gemini-3.8-flash` | its providers' variants: `high`, `max`; `low` to `xhigh`; `low`, `high` |

Where it was read:

- Anthropic: [models overview](https://platform.claude.com/docs/en/about-claude/models/overview),
  [effort](https://platform.claude.com/docs/en/build-with-claude/effort);
  Claude Code: [model configuration](https://code.claude.com/docs/en/model-config).
- OpenAI: [models](https://developers.openai.com/api/docs/models),
  [GPT-6.1 Sol](https://developers.openai.com/api/docs/models/gpt-6.1-sol),
  [reasoning](https://developers.openai.com/api/docs/guides/reasoning);
  Codex: its own catalogue, `codex-rs/models-manager/models.json` in
  [openai/codex](https://github.com/openai/codex), and its
  [models page](https://learn.chatgpt.com/docs/models).
- Google: [models](https://aistudio.google.com/docs/models),
  [thinking](https://aistudio.google.com/docs/thinking).
- Mistral: [models](https://docs.mistral.ai/getting-started/models/models_overview/),
  [reasoning](https://docs.mistral.ai/capabilities/reasoning/),
  [Mistral Large 4](https://docs.mistral.ai/models/mistral-large-4-0.md).
- OpenCode: [models and variants](https://opencode.ai/docs/models/),
  [`opencode run`](https://opencode.ai/docs/cli/).
- Ollama: [thinking](https://docs.ollama.com/capabilities/thinking),
  [its library](https://ollama.com/search?c=thinking); vLLM's names are
  Hugging Face's ([its API](https://huggingface.co/api/models)).

Left out on purpose: models their maker has deprecated (Mistral Medium 3.5,
Gemini 2.5), previews when a stable model does the same (Gemini 3.1 Pro),
models served only in Ollama's cloud (the archive would leave the machine
under a local way's name), and Codex's `persistent`, which is a setting of
Codex's own and not an effort.

### 2. Suggested, never imposed

The model field and the effort field each have a list (`<datalist>`): the
browser offers it, and the field still takes anything.

- **The model list follows the way of computing.** The page holds one list
  for each way; the field points at the chosen way's.
- **The effort list follows the model.** When the model is one of the
  suggestions, it lists that model's efforts, its usual one marked
  *default*. Otherwise it lists every effort the way's suggestions take.
  An Ollama name is its model whatever size or build follows its colon:
  `qwen3.8:27b` is `qwen3.8`, and `gpt-oss` is `gpt-oss:20b`.
- **A name outside the list is kept, and said to be outside it**, under the
  field: "Not one of the suggestions: sent as written." The same goes for
  an effort the model does not list. `opus-5` is saved, and the room says
  it is not a suggestion.
- **Changing the way of computing** empties the model and the effort only
  when they came from the last way's lists and are not in the new one's. A
  name written by hand stays.

The server checks a model and an effort only for how they are written
(`plain`: letters, digits and `. _ : / @ [ ] + -`, never a leading `-`).
That keeps a value from being read as a program's option, or from leaving
the TOML string Codex is given it in. The old per-agent lists of efforts
are gone.

### 3. Every service takes an effort

The effort field is shown for every way of computing, and is sent in the
setting each protocol names:

| Protocol | Setting |
|---|---|
| Ollama | `think`: `true` and `false` as booleans, any other word as a level |
| OpenAI, Mistral, vLLM | `reasoning_effort` |
| Claude (Anthropic API) | `output_config.effort` |
| Gemini | `generationConfig.thinkingConfig.thinkingLevel` |
| Claude Code, Codex, OpenCode | `--effort`, `model_reasoning_effort`, `--variant`, as before |

Left empty, nothing is sent, and the model's own default holds.

Claude's `max_tokens` rises from 8,192 to 32,000. Every current Claude model
thinks before it answers, and its thinking counts within `max_tokens`. At
`high` and above, 8,192 could be spent before the answer began.

## Consequences

- **The catalogue ages.** It is read again from the makers' pages for each
  release, and the release notes say so. Between two releases the field
  takes any name, and the list is a starting point, not a register.
- **An effort saved for an agent now reaches a service.** Before, an effort
  left in the hidden field was ignored by a service. Now the field is
  shown, and its value is sent. A value a service does not take returns
  the service's own refusal in the test's window.
- **Claude Code's aliases depend on the version installed.** `opus` is
  Opus 5.5 for 2.1.300 and Opus 4.7 for 2.1.131. The full names come first
  in the list for that reason.
- **Not in this part:** asking a service which models it has (Ollama's
  `/api/tags`, a provider's model list), which would make the room call the
  network to draw itself.
