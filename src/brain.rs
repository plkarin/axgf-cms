//! The AFG's brains (the operator's request of 2 November 2026,
//! docs/adr/0024): a folder on disk for each account that talks to the
//! Artificial family genealogist, which is the AFG's memory of that person.
//! The archive is what the AFG knows of the family; a brain is what it
//! remembers of whoever asks.
//!
//! ```text
//! family.brains/            beside family.axgf, as family.acl is
//!   <username>/             one per account, under its name, mode 700
//!     brain.json            whose brain this is: the account's id and name
//!     memory.md             what the AFG chose to keep, one fact a line
//!     conversations.jsonl   every exchange, one JSON object a line
//! ```
//!
//! A brain is its account's, not its name's. A folder whose `brain.json`
//! names another account (one deleted, and its name given again) is set
//! aside under `<name>@<its account>`, never read to the newcomer.
//!
//! The AFG writes to its memory the one way every kind of computing can: it
//! ends an answer with a `<memory>` block, one fact a line
//! ([`take_memory`]), which is taken out of the answer and kept here.

use std::fs;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// How much of `memory.md` goes back to the AFG with each question: the
/// most recent part when it is longer.
const RECALL_MEMORY: usize = 16 * 1024;
/// How many of the last exchanges go back with it.
const RECALL_EXCHANGES: usize = 5;
/// How much of each of those answers.
const RECALL_ANSWER: usize = 1_500;
/// How much of the end of `conversations.jsonl` is read to find them.
const RECALL_TAIL: u64 = 2 * 1024 * 1024;
/// The most a memory keeps: past it, its oldest facts go first.
const MEMORY_MAX: usize = 64 * 1024;
/// What one answer may add to it.
const FACTS_PER_ANSWER: usize = 20;
const FACT_CHARS: usize = 400;

const CARD: &str = "brain.json";
const MEMORY: &str = "memory.md";
const CONVERSATIONS: &str = "conversations.jsonl";

/// The brains of one archive.
#[derive(Debug, Clone)]
pub struct Brains {
    root: PathBuf,
}

/// Whose brain: an account, by its id and its name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Owner {
    pub id: String,
    pub name: String,
    /// The person in the tree the account says it is.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub person_id: Option<String>,
}

impl Owner {
    pub fn of(user: &crate::acl::User) -> Self {
        Owner {
            id: user.id.clone(),
            name: user.username.clone(),
            person_id: user.person_id.clone(),
        }
    }
}

/// `brain.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Card {
    account_id: String,
    username: String,
    created_at: String,
}

/// One question and its answer, as `conversations.jsonl` keeps it.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Exchange {
    pub at: String,
    /// Where it was asked: `test`, in the administration's AFG room. The
    /// pages the AFG will come to name themselves.
    pub context: String,
    /// The way of computing that answered, and its model.
    pub kind: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub model: String,
    pub question: String,
    /// The answer as shown, its memory block taken out.
    pub answer: String,
    /// What the AFG kept from it in the memory.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub remembered: Vec<String>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub run: String,
}

/// What goes back to the AFG with a question.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Recall {
    /// `memory.md`'s facts, the most recent part of them.
    pub memory: String,
    /// The last exchanges, oldest first, each answer cut short.
    pub recent: Vec<Exchange>,
}

/// A brain at a glance, for the room.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
pub struct Stats {
    pub exchanges: usize,
    pub facts: usize,
}

/// One account's brain, its folder made.
#[derive(Debug, Clone)]
pub struct Brain {
    dir: PathBuf,
    pub owner: Owner,
}

/// The folder an account's brain takes: its name, which a username's
/// alphabet keeps safe for a path, but never a dot first (`..` is a valid
/// username, and must not be its parent).
pub fn folder_name(username: &str) -> String {
    let safe: String = username
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_') {
                c
            } else {
                '_'
            }
        })
        .collect();
    if safe.is_empty() || safe.starts_with('.') {
        format!("_{safe}")
    } else {
        safe
    }
}

fn make_dir(dir: &Path) -> io::Result<()> {
    fs::DirBuilder::new()
        .recursive(true)
        .mode(0o700)
        .create(dir)
}

fn read_card(dir: &Path) -> Option<Card> {
    let text = fs::read_to_string(dir.join(CARD)).ok()?;
    serde_json::from_str(&text).ok()
}

/// Write a whole file readable by its owner only.
fn write_private(path: &Path, bytes: &[u8]) -> io::Result<()> {
    let tmp = path.with_extension("tmp");
    {
        let mut f = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o600)
            .open(&tmp)?;
        f.write_all(bytes)?;
        f.sync_all()?;
    }
    fs::rename(&tmp, path)
}

/// Add to the end of a file readable by its owner only.
fn append_private(path: &Path, bytes: &[u8]) -> io::Result<()> {
    let mut f = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .mode(0o600)
        .open(path)?;
    f.write_all(bytes)?;
    f.sync_all()
}

/// The first `n` characters of a text, said to be cut when they are.
fn cut(s: &str, n: usize) -> String {
    if s.chars().count() <= n {
        return s.to_string();
    }
    let mut out: String = s.chars().take(n).collect();
    out.push('…');
    out
}

impl Brains {
    /// The brains beside an archive: `family.brains` for `family.axgf`.
    pub fn beside(bundle: &Path) -> Self {
        Brains {
            root: bundle.with_extension("brains"),
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    /// The account's brain, made the first time it is asked for.
    pub fn of(&self, owner: &Owner) -> io::Result<Brain> {
        make_dir(&self.root)?;
        let dir = self.root.join(folder_name(&owner.name));
        if let Some(card) = read_card(&dir) {
            if card.account_id != owner.id {
                // Another account's memory under this name: set it aside
                // whole, under a name no username can take.
                let short: String = card.account_id.chars().take(8).collect();
                let mut aside = self
                    .root
                    .join(format!("{}@{short}", folder_name(&card.username)));
                let mut n = 2;
                while aside.exists() {
                    aside = self
                        .root
                        .join(format!("{}@{short}-{n}", folder_name(&card.username)));
                    n += 1;
                }
                fs::rename(&dir, &aside)?;
                tracing::warn!(
                    folder = %dir.display(),
                    moved_to = %aside.display(),
                    "a brain under this name was another account's; set aside"
                );
            }
        }
        make_dir(&dir)?;
        if read_card(&dir).is_none() {
            let card = Card {
                account_id: owner.id.clone(),
                username: owner.name.clone(),
                created_at: crate::view::now_iso8601(),
            };
            let body = serde_json::to_vec_pretty(&card).map_err(io::Error::other)?;
            write_private(&dir.join(CARD), &body)?;
        }
        Ok(Brain {
            dir,
            owner: owner.clone(),
        })
    }

    /// The account's brain at a glance, without making it.
    pub fn peek(&self, owner: &Owner) -> Option<Stats> {
        let dir = self.root.join(folder_name(&owner.name));
        let card = read_card(&dir)?;
        (card.account_id == owner.id).then(|| {
            Brain {
                dir,
                owner: owner.clone(),
            }
            .stats()
        })
    }

    /// How many accounts have a brain.
    pub fn count(&self) -> usize {
        fs::read_dir(&self.root)
            .map(|entries| {
                entries
                    .flatten()
                    .filter(|e| e.path().join(CARD).is_file())
                    .filter(|e| !e.file_name().to_string_lossy().contains('@'))
                    .count()
            })
            .unwrap_or(0)
    }
}

impl Brain {
    pub fn dir(&self) -> &Path {
        &self.dir
    }

    /// The facts of `memory.md`, without its heading.
    fn facts(&self) -> Vec<String> {
        fs::read_to_string(self.dir.join(MEMORY))
            .unwrap_or_default()
            .lines()
            .filter(|l| l.starts_with("- "))
            .map(str::to_string)
            .collect()
    }

    /// The last lines of `conversations.jsonl`, read from its end.
    fn last_exchanges(&self, n: usize) -> Vec<Exchange> {
        let Ok(mut f) = fs::File::open(self.dir.join(CONVERSATIONS)) else {
            return Vec::new();
        };
        let len = f.metadata().map(|m| m.len()).unwrap_or(0);
        let from = len.saturating_sub(RECALL_TAIL);
        if f.seek(SeekFrom::Start(from)).is_err() {
            return Vec::new();
        }
        let mut bytes = Vec::new();
        if f.read_to_end(&mut bytes).is_err() {
            return Vec::new();
        }
        let text = String::from_utf8_lossy(&bytes);
        let mut lines: Vec<&str> = text.lines().collect();
        if from > 0 && !lines.is_empty() {
            // The first line read may be the end of a longer one.
            lines.remove(0);
        }
        let mut out: Vec<Exchange> = lines
            .iter()
            .rev()
            .filter_map(|l| serde_json::from_str(l).ok())
            .take(n)
            .collect();
        out.reverse();
        out
    }

    /// What goes back to the AFG with the next question.
    pub fn recall(&self) -> Recall {
        let mut memory = self.facts().join("\n");
        if memory.len() > RECALL_MEMORY {
            let mut at = memory.len() - RECALL_MEMORY;
            while !memory.is_char_boundary(at) {
                at += 1;
            }
            let start = memory[at..].find('\n').map_or(at, |i| at + i + 1);
            memory = memory[start..].to_string();
        }
        let recent = self
            .last_exchanges(RECALL_EXCHANGES)
            .into_iter()
            .map(|mut e| {
                e.answer = cut(&e.answer, RECALL_ANSWER);
                e
            })
            .collect();
        Recall { memory, recent }
    }

    /// Keep an exchange, and the facts the AFG asked to remember from it.
    /// Returns the facts kept: those it did not already hold.
    pub fn remember(&self, exchange: &Exchange, facts: &[String]) -> io::Result<Vec<String>> {
        let mut known = self.facts();
        let held: Vec<String> = known.iter().map(|l| fact_text(l).to_lowercase()).collect();
        let date: String = exchange.at.chars().take(10).collect();
        let mut kept = Vec::new();
        for f in facts {
            let f = f.trim();
            if f.is_empty()
                || held.contains(&f.to_lowercase())
                || kept.iter().any(|k: &String| k.eq_ignore_ascii_case(f))
            {
                continue;
            }
            kept.push(f.to_string());
        }
        if !kept.is_empty() {
            known.extend(kept.iter().map(|f| format!("- [{date}] {f}")));
            // Past the most a memory keeps, its oldest facts go first.
            let mut size: usize = known.iter().map(|l| l.len() + 1).sum();
            while size > MEMORY_MAX && known.len() > 1 {
                size -= known[0].len() + 1;
                known.remove(0);
            }
            let body = format!(
                "# What the AFG remembers of {}\n\n{}\n",
                self.owner.name,
                known.join("\n")
            );
            write_private(&self.dir.join(MEMORY), body.as_bytes())?;
        }
        let mut entry = exchange.clone();
        entry.remembered = kept.clone();
        let mut line = serde_json::to_string(&entry).map_err(io::Error::other)?;
        line.push('\n');
        append_private(&self.dir.join(CONVERSATIONS), line.as_bytes())?;
        Ok(kept)
    }

    pub fn stats(&self) -> Stats {
        let exchanges = fs::read(self.dir.join(CONVERSATIONS))
            .map(|b| b.iter().filter(|&&c| c == b'\n').count())
            .unwrap_or(0);
        Stats {
            exchanges,
            facts: self.facts().len(),
        }
    }
}

/// A memory line's fact, without its bullet and date.
fn fact_text(line: &str) -> &str {
    let l = line.trim_start_matches("- ");
    match l.strip_prefix('[').and_then(|r| r.split_once("] ")) {
        Some((_, rest)) => rest,
        None => l,
    }
}

/// Split an answer into what is shown and what the AFG asked to remember:
/// its last `<memory>` block (a code fence around it included), one fact a
/// line, bullets and numbering taken off. A block with no end runs to the
/// end of the answer.
pub fn take_memory(answer: &str) -> (String, Vec<String>) {
    let lower = answer.to_ascii_lowercase();
    let Some(open) = lower.rfind("<memory>") else {
        return (answer.trim().to_string(), Vec::new());
    };
    let inner_start = open + "<memory>".len();
    let (inner_end, mut close) = match lower[inner_start..].find("</memory>") {
        Some(i) => (inner_start + i, inner_start + i + "</memory>".len()),
        None => (answer.len(), answer.len()),
    };
    let mut start = open;
    // A fence the model put the block in goes with it.
    let before = answer[..open].trim_end();
    if let Some(fence) = before.rfind("```") {
        let after_fence = &before[fence + 3..];
        let after = answer[close..].trim_start();
        if !after_fence.contains('\n') && after.starts_with("```") {
            start = fence;
            close = answer.len() - after.len() + 3;
        }
    }
    let facts = answer[inner_start..inner_end]
        .lines()
        .map(|l| {
            let l = l.trim();
            let l = l.trim_start_matches(['-', '*', '•']).trim_start();
            let digits = l.chars().take_while(char::is_ascii_digit).count();
            let l = match l[digits..].strip_prefix(['.', ')']) {
                Some(rest) if digits > 0 => rest.trim_start(),
                _ => l,
            };
            cut(l, FACT_CHARS)
        })
        .filter(|l| !l.is_empty())
        .take(FACTS_PER_ANSWER)
        .collect();
    let shown = format!("{}{}", &answer[..start], &answer[close..]);
    (shown.trim().to_string(), facts)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(tag: &str) -> crate::scratch::Dir {
        crate::scratch::Dir::new(tag)
    }

    fn owner(id: &str, name: &str) -> Owner {
        Owner {
            id: id.into(),
            name: name.into(),
            person_id: None,
        }
    }

    #[test]
    fn a_brain_is_a_private_folder_under_its_accounts_name() {
        use std::os::unix::fs::PermissionsExt;
        let dir = scratch("folder");
        let brains = Brains::beside(&dir.join("family.axgf"));
        assert_eq!(brains.root(), dir.join("family.brains"));
        assert_eq!(brains.count(), 0);
        assert!(
            brains.peek(&owner("a1", "marie")).is_none(),
            "peeking makes nothing"
        );
        let b = brains.of(&owner("a1", "marie")).unwrap();
        assert_eq!(b.dir(), dir.join("family.brains/marie"));
        let mode = fs::metadata(b.dir()).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o700);
        assert_eq!(brains.count(), 1);
        assert_eq!(folder_name(".."), "_..");
        assert_eq!(folder_name(".hidden"), "_.hidden");
        assert_eq!(folder_name("jean-luc_2"), "jean-luc_2");
    }

    #[test]
    fn a_name_given_again_never_reads_the_old_accounts_memory() {
        let dir = scratch("again");
        let brains = Brains::beside(&dir.join("family.axgf"));
        let first = brains.of(&owner("old-account-1", "lea")).unwrap();
        let ex = Exchange {
            at: "2026-11-02T10:00:00Z".into(),
            context: "test".into(),
            question: "Q".into(),
            answer: "A".into(),
            ..Exchange::default()
        };
        first
            .remember(&ex, &["Researches her mother's side".into()])
            .unwrap();
        let second = brains.of(&owner("new-account-2", "lea")).unwrap();
        assert_eq!(second.recall(), Recall::default(), "a fresh brain");
        assert!(
            dir.join("family.brains/lea@old-acco")
                .join(MEMORY)
                .is_file(),
            "set aside, kept"
        );
        assert_eq!(brains.count(), 1, "the set-aside one is not counted");
    }

    #[test]
    fn what_is_remembered_comes_back_with_the_next_question() {
        use std::os::unix::fs::PermissionsExt;
        let dir = scratch("recall");
        let brains = Brains::beside(&dir.join("family.axgf"));
        let b = brains.of(&owner("a1", "marie")).unwrap();
        for i in 0..7 {
            let ex = Exchange {
                at: format!("2026-11-0{}T10:00:00Z", i + 1),
                context: "test".into(),
                kind: "ollama".into(),
                question: format!("Question {i}"),
                answer: "x".repeat(3_000),
                ..Exchange::default()
            };
            let facts = if i == 0 {
                vec![
                    "Prefers short answers".into(),
                    "prefers short answers".into(),
                ]
            } else {
                vec!["Prefers short answers".into()]
            };
            let kept = b.remember(&ex, &facts).unwrap();
            assert_eq!(kept.len(), usize::from(i == 0), "a fact is kept once");
        }
        let r = b.recall();
        assert_eq!(r.memory, "- [2026-11-01] Prefers short answers");
        assert_eq!(r.recent.len(), RECALL_EXCHANGES);
        assert_eq!(r.recent[0].question, "Question 2", "oldest first");
        assert!(r.recent[4].answer.chars().count() <= RECALL_ANSWER + 1);
        assert_eq!(
            b.stats(),
            Stats {
                exchanges: 7,
                facts: 1
            }
        );
        let memory = fs::read_to_string(b.dir().join(MEMORY)).unwrap();
        assert!(memory.starts_with("# What the AFG remembers of marie\n"));
        for f in [MEMORY, CONVERSATIONS, CARD] {
            let mode = fs::metadata(b.dir().join(f)).unwrap().permissions().mode() & 0o777;
            assert_eq!(mode, 0o600, "{f}");
        }
    }

    #[test]
    fn the_memory_block_is_taken_out_of_the_answer() {
        let (shown, facts) = take_memory(
            "1. The Exemple family, 12 people.\n\n<memory>\n- Asked for the largest families\n* Writes in French\n3. Likes tables\n</memory>",
        );
        assert_eq!(shown, "1. The Exemple family, 12 people.");
        assert_eq!(
            facts,
            [
                "Asked for the largest families",
                "Writes in French",
                "Likes tables"
            ]
        );
        let (shown, facts) =
            take_memory("Bundle vide\n```\n<MEMORY>\nThe archive was empty\n</MEMORY>\n```\n");
        assert_eq!(shown, "Bundle vide", "the fence goes with the block");
        assert_eq!(facts, ["The archive was empty"]);
        let (shown, facts) = take_memory("Answer.\n<memory>\nno end");
        assert_eq!((shown.as_str(), facts.len()), ("Answer.", 1));
        let (shown, facts) = take_memory("No block here.");
        assert_eq!((shown.as_str(), facts.is_empty()), ("No block here.", true));
    }
}
