//! The models of the moment, and the efforts each takes: what the AFG's room
//! suggests for each way of computing (the operator's request of 2 November
//! 2026, docs/adr/0025). Suggestions only: the room's fields take any other
//! name or effort written plainly ([`crate::afg::plain`]) and send it as it
//! is.
//!
//! Read from each maker's own documentation on 2 November 2026, and from
//! Codex's own catalogue of models; the ADR names each source. Models come
//! and go, so the list is read again for each release.
//!
//! An effort is written in the words of the setting it goes into:
//!
//! * Ollama: `think`, `true` or `false`, or a level the model names
//!   (`low`, `medium`, `high` for gpt-oss);
//! * OpenAI, Mistral and vLLM: `reasoning_effort`;
//! * Claude (Anthropic API): `output_config.effort`;
//! * Gemini: `generationConfig.thinkingConfig.thinkingLevel`;
//! * Claude Code: `--effort`; Codex: `model_reasoning_effort`; OpenCode:
//!   `--variant`, which its provider defines.

use serde::Serialize;

use crate::afg::Kind;

/// A model suggested for a way of computing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Suggestion {
    /// Its name, as the service or the agent writes it.
    pub model: &'static str,
    /// Who makes it.
    pub maker: &'static str,
    /// The efforts it takes, the lightest first.
    pub efforts: &'static [&'static str],
    /// The effort it takes when none is given, where its maker says.
    pub default: Option<&'static str>,
}

const fn s(
    model: &'static str,
    maker: &'static str,
    efforts: &'static [&'static str],
    default: Option<&'static str>,
) -> Suggestion {
    Suggestion {
        model,
        maker,
        efforts,
        default,
    }
}

/// Every effort a suggestion names, the lightest first: the order the
/// room lists them in.
pub const ORDER: &[&str] = &[
    "false", "true", "none", "minimal", "low", "medium", "high", "xhigh", "max", "ultra",
];

const ON_OFF: &[&str] = &["false", "true"];
const THREE: &[&str] = &["low", "medium", "high"];
const FIVE: &[&str] = &["low", "medium", "high", "xhigh", "max"];
const FIVE_AND_NONE: &[&str] = &["none", "low", "medium", "high", "xhigh", "max"];
const FIVE_AND_ULTRA: &[&str] = &["low", "medium", "high", "xhigh", "max", "ultra"];
const MINIMAL_TO_HIGH: &[&str] = &["minimal", "low", "medium", "high"];
const NONE_OR_HIGH: &[&str] = &["none", "high"];
const NONE: &[&str] = &[];

/// Ollama, on this machine or the network: what its library offers that
/// thinks (`think`), the most pulled first.
const OLLAMA: &[Suggestion] = &[
    s("qwen3.8", "Qwen", ON_OFF, Some("true")),
    s("gemma4", "Google", ON_OFF, None),
    s("gpt-oss:20b", "OpenAI", THREE, Some("medium")),
    s("gpt-oss:120b", "OpenAI", THREE, Some("medium")),
];

/// OpenAI's API: GPT-6.1 Sol is its recommended balance, Astra its most
/// capable, Terra and Luna the lighter ones.
const OPENAI: &[Suggestion] = &[
    s("gpt-6.1-sol", "OpenAI", FIVE, Some("medium")),
    s("gpt-6-astra", "OpenAI", FIVE, None),
    s("gpt-6.1-terra", "OpenAI", FIVE_AND_NONE, None),
    s("gpt-6-luna", "OpenAI", FIVE_AND_NONE, Some("medium")),
];

/// Mistral's API, with the third-party model it serves that reasons.
const MISTRAL: &[Suggestion] = &[
    s("mistral-large-4-0", "Mistral AI", THREE, None),
    s("mistral-small-2603", "Mistral AI", NONE_OR_HIGH, None),
    s("zai-glm-5-3", "Z.ai", &["low", "high", "max"], None),
];

/// vLLM serves open weights under their Hugging Face names. Only some
/// models read `reasoning_effort`; the others think as their template
/// says.
const VLLM: &[Suggestion] = &[
    s("openai/gpt-oss-120b", "OpenAI", THREE, Some("medium")),
    s("openai/gpt-oss-20b", "OpenAI", THREE, Some("medium")),
    s("Qwen/Qwen3.8-27B", "Qwen", NONE, None),
    s(
        "mistralai/Mistral-Small-4-119B-2603",
        "Mistral AI",
        NONE_OR_HIGH,
        None,
    ),
    s("google/gemma-4-31B-it", "Google", NONE, None),
    s("deepseek-ai/DeepSeek-V4.1-Flash", "DeepSeek", NONE, None),
];

/// Anthropic's API. Opus 5.5 is its recommended start.
const ANTHROPIC: &[Suggestion] = &[
    s("claude-opus-5-5", "Anthropic", FIVE, Some("medium")),
    s("claude-fable-5-1", "Anthropic", FIVE, Some("high")),
    s("claude-sonnet-5-5", "Anthropic", FIVE, Some("high")),
    s("claude-haiku-5-5", "Anthropic", FIVE, Some("medium")),
];

/// Google's API, its stable models that think.
const GEMINI: &[Suggestion] = &[
    s("gemini-4-argon", "Google", THREE, Some("medium")),
    s("gemini-3.8-flash", "Google", THREE, Some("medium")),
    s(
        "gemini-3.6-flash",
        "Google",
        MINIMAL_TO_HIGH,
        Some("medium"),
    ),
    s(
        "gemini-3.5-flash-lite",
        "Google",
        MINIMAL_TO_HIGH,
        Some("minimal"),
    ),
];

/// Claude Code takes a model's full name or an alias, which names the
/// latest model of its line for the version installed.
const CLAUDE_CODE: &[Suggestion] = &[
    s("claude-opus-5-5", "Anthropic", FIVE, Some("medium")),
    s("claude-fable-5-1", "Anthropic", FIVE, Some("high")),
    s("claude-sonnet-5-5", "Anthropic", FIVE, Some("medium")),
    s("claude-haiku-5-5", "Anthropic", FIVE, Some("medium")),
    s("opus", "Anthropic", FIVE, None),
    s("fable", "Anthropic", FIVE, None),
    s("sonnet", "Anthropic", FIVE, None),
    s("haiku", "Anthropic", FIVE, None),
];

/// Codex, as its own catalogue lists them: GPT-6.1 Sol first.
const CODEX: &[Suggestion] = &[
    s("gpt-6.1-sol", "OpenAI", FIVE_AND_ULTRA, Some("low")),
    s("gpt-6-astra", "OpenAI", FIVE_AND_ULTRA, Some("low")),
    s("gpt-6.1-terra", "OpenAI", FIVE_AND_ULTRA, Some("low")),
    s("gpt-6-luna", "OpenAI", FIVE, Some("medium")),
];

/// OpenCode names a model `provider/model`, and each provider's variants:
/// Anthropic's `high` and `max`, Google's `low` and `high`, and OpenAI's as
/// far as the model goes.
const OPENCODE: &[Suggestion] = &[
    s(
        "anthropic/claude-opus-5-5",
        "Anthropic",
        &["high", "max"],
        Some("high"),
    ),
    s(
        "anthropic/claude-sonnet-5-5",
        "Anthropic",
        &["high", "max"],
        Some("high"),
    ),
    s(
        "openai/gpt-6.1-sol",
        "OpenAI",
        &["low", "medium", "high", "xhigh"],
        None,
    ),
    s("google/gemini-3.8-flash", "Google", &["low", "high"], None),
];

/// The models suggested for a way of computing, the first the one to start
/// with.
pub fn of(kind: Kind) -> &'static [Suggestion] {
    match kind {
        Kind::Off => &[],
        Kind::Ollama => OLLAMA,
        Kind::Openai => OPENAI,
        Kind::Mistral => MISTRAL,
        Kind::Vllm => VLLM,
        Kind::Anthropic => ANTHROPIC,
        Kind::Gemini => GEMINI,
        Kind::ClaudeCode => CLAUDE_CODE,
        Kind::Codex => CODEX,
        Kind::Opencode => OPENCODE,
    }
}

/// The suggestion of that name, if it is one. Ollama names a size or a
/// build after a colon (`qwen3.8:27b`, `gpt-oss`): the same model, which
/// thinks the same way.
pub fn find(kind: Kind, model: &str) -> Option<&'static Suggestion> {
    let model = model.trim();
    let list = of(kind);
    let base = |m: &'static str| m.split(':').next().unwrap_or(m);
    list.iter().find(|s| s.model == model).or_else(|| {
        let wanted = model.split(':').next().unwrap_or(model);
        (kind == Kind::Ollama && !wanted.is_empty())
            .then(|| list.iter().find(|s| base(s.model) == wanted))
            .flatten()
    })
}

/// Every effort the way of computing's suggestions take, the lightest
/// first: what is offered while the model is not one of them.
pub fn efforts_of(kind: Kind) -> Vec<&'static str> {
    ORDER
        .iter()
        .copied()
        .filter(|e| of(kind).iter().any(|s| s.efforts.contains(e)))
        .collect()
}

/// The efforts offered for a model, and the one it takes when none is
/// given: its own when it is a suggestion, every one of the kind's
/// otherwise.
pub fn efforts_for(kind: Kind, model: &str) -> (Vec<&'static str>, Option<&'static str>) {
    match find(kind, model) {
        Some(s) => (s.efforts.to_vec(), s.default),
        None => (efforts_of(kind), None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::afg::plain;

    #[test]
    fn every_way_of_computing_suggests_models_written_plainly() {
        for kind in Kind::ALL {
            let list = of(kind);
            assert_eq!(list.is_empty(), kind == Kind::Off, "{kind:?}");
            for (i, s) in list.iter().enumerate() {
                assert!(plain(s.model), "{kind:?}: {}", s.model);
                assert!(!s.maker.is_empty());
                assert!(
                    !list[..i].iter().any(|t| t.model == s.model),
                    "{kind:?}: {} twice",
                    s.model
                );
                for e in s.efforts {
                    assert!(plain(e) && ORDER.contains(e), "{kind:?}: {} {e}", s.model);
                }
                let mut sorted = s.efforts.to_vec();
                sorted.sort_by_key(|e| ORDER.iter().position(|o| o == e));
                assert_eq!(sorted, s.efforts, "{kind:?}: {} lightest first", s.model);
                if let Some(d) = s.default {
                    assert!(s.efforts.contains(&d), "{kind:?}: {} default {d}", s.model);
                }
            }
        }
    }

    #[test]
    fn the_efforts_offered_follow_the_model() {
        assert_eq!(
            efforts_for(Kind::Anthropic, " claude-opus-5-5 "),
            (
                vec!["low", "medium", "high", "xhigh", "max"],
                Some("medium")
            )
        );
        assert_eq!(
            efforts_for(Kind::Mistral, "mistral-small-2603"),
            (vec!["none", "high"], None)
        );
        // A model that is not suggested is offered every effort its kind's
        // suggestions take, and none as its default.
        assert_eq!(
            efforts_for(Kind::Ollama, "llama3.1:8b"),
            (vec!["false", "true", "low", "medium", "high"], None)
        );
        assert_eq!(
            efforts_of(Kind::Codex),
            ["low", "medium", "high", "xhigh", "max", "ultra"]
        );
        assert!(efforts_of(Kind::Off).is_empty());
        assert!(find(Kind::ClaudeCode, "opus-5").is_none());
        // An Ollama model is itself whatever its size.
        assert_eq!(find(Kind::Ollama, "qwen3.8:27b").unwrap().model, "qwen3.8");
        assert_eq!(
            efforts_for(Kind::Ollama, "gpt-oss"),
            (vec!["low", "medium", "high"], Some("medium"))
        );
        assert!(find(Kind::Ollama, ":latest").is_none());
        assert!(
            find(Kind::Vllm, "openai/gpt-oss-120b:x").is_none(),
            "only Ollama's names have sizes"
        );
        assert_eq!(of(Kind::ClaudeCode)[0].model, "claude-opus-5-5");
    }
}
