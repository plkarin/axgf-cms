# ADR 0032 — A Status tab, and fewer words

**Status:** accepted (unreleased) · **Builds on:** the data-quality screen
(`src/quality.rs`), the record's tabs (v0.4.1) · **Applies to:**
`src/person.rs`, `src/quality.rs`, `src/state.rs`, `src/routes/public.rs`,
`templates/_person_detail.html`, `static/app.css`, `locales/*.ftl`,
`tests/quality.rs`, `tests/read_mode.rs`, `tests/person_chat.rs`

## Context

On 5 November 2026 the operator found the pages too wordy: help and
information in sentences everywhere, where a line would do. Two examples:

- the tree's notice that it contradicts itself;
- the strip at the top of a record, "Notes on this record", with a sentence
  for each point.

The operator asked for those words to be cut, and for a **Status** tab on the
record. It should gather what is worth checking about the person: the
coherence of the AXGF record, in the sense of a sanity check.

## Decision

### 1. The Status tab

The record has a **Status** tab, after Profile. It is drawn when it has
something to say, as every tab is, and it holds:

- **To check:** the data-quality findings about the person
  (`quality::about_person`): a death before a birth, a couple who are also
  parent and child, a parent too young, a claim certain without a source, and
  so on. Each finding is one line: the rule's name, then what it found. It
  links to the record's editor for someone who may write. The tab's count is
  the number of these findings.
- How many were marked "normal for us", leading to the Quality page.
- **Good to know:** what used to be the strip on the fiche, said as labels
  ("Dated occupations: 2", "Birth date imprecise in the source").

The rules are the Quality page's own:

- a reader who is not signed in is shown no finding;
- a finding is shown only when every person it names is readable.

The strip at the top of the fiche is gone. The tree's panel, which has no
tabs, keeps a single line, "To check: n", that leads to the tab.

The findings are worked out once for each version of the bundle
(`AppState::quality_findings`, keyed on the generation that already tells the
lens cache it is stale), so a record page costs no detection.

### 2. Fewer words

The sentences a reader sees when a page has loaded were measured, on every
member's page and every administration page: the release binary renders
the sample, and every text of 90 characters or more drawn at rest is listed
with its catalogue key. Then:

- **the members' pages:** the AFG tab, the assistant, the account page, the
  Quality and import pages, the kinship tab, the tree's contradiction notice,
  and the data-quality findings themselves;
- **the administration:** the AFG room, the accounts, the connections, the
  infrastructure, the archive's page, the completeness metrics and the notes'
  editor.

91 texts became one short line each, in the fourteen catalogues, with the
same variables as before. A line keeps what to do, and drops why. Counted
labels take the form `label: n` ("Accounts that may talk to the AFG: 2"),
which needs no plural form in any language.

## Consequences

- The fiche is shorter by a strip, and the panel by a box.
- A finding fixed or marked as normal leaves the tab at the next request:
  every save bumps the generation, so the cache is worked out again.
- The reasons a line no longer gives are in the operator's guide and the
  ADRs, where they always were.
