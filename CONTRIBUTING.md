# Contributing

## Translations

This is where help is most useful, and where the honest position needs stating
plainly: **eleven interface languages are complete. Two of them have been read
by someone who speaks them. The other nine have not.**

Complete is not the same as reviewed, and the language selector says both — a
finished machine translation is still a machine translation, and a bare "100%"
would read as a quality score rather than as a count. English and French are
complete and read. The other nine were translated without a native speaker.

The risk is not that a button says the wrong thing; a wrong button is obvious.
The risk is genealogical vocabulary, where a plausible-looking wrong word is
worse than an English one because nobody notices it.

### Where each language stands

| Tag | Language | | Messages | Coverage | Reviewed |
|---|---|---|---|---|---|
| `en` | English | English | 553 / 553 | 100% | yes |
| `fr` | French | Français | 553 / 553 | 100% | yes |
| `pl` | Polish | Polski | 553 / 553 | 100% | **no — machine quality** |
| `ru` | Russian | Русский | 553 / 553 | 100% | **no — machine quality** |
| `de` | German | Deutsch | 553 / 553 | 100% | **no — machine quality** |
| `it` | Italian | Italiano | 553 / 553 | 100% | **no — machine quality** |
| `es` | Spanish | Español | 553 / 553 | 100% | **no — machine quality** |
| `pt` | Portuguese | Português | 553 / 553 | 100% | **no — machine quality** |
| `zh-Hans` | Chinese (Simplified) | 简体中文 | 553 / 553 | 100% | **no — machine quality** |
| `ja` | Japanese | 日本語 | 553 / 553 | 100% | **no — machine quality** |
| `ar` | Arabic | العربية | 553 / 553 | 100% | **no — machine quality** |

Coverage is the share of English's messages that the locale defines. It is
computed from the files, shown in the language selector, and asserted by a
test — nobody has to take the number on trust. A message a locale does not
define falls back to English rather than rendering as an error, so a partial
translation is a usable one.

Russian is here for the domain rather than for the count: the civil and parish
registers of the former Russian Empire were kept in Russian, so a researcher
working on Polish, Lithuanian, Ukrainian or Belarusian records is reading
Russian documents.

### Start here: the words most likely to be wrong

If you read one of these languages and have ten minutes, this list is where
the ten minutes are best spent. Each term has an established equivalent that
differs by national record-keeping tradition, and every catalogue states its
choices in a comment at the top of the file so you have something specific to
disagree with.

| English | What to check |
|---|---|
| union, family, spouse, partner | Does your language distinguish a marriage from a recorded partnership? AXGF's *union* covers both. |
| affiliation, link, godparent, witness | *Godparent* especially: Russian uses `восприемник` in the registers and `крёстный` in speech. Which belongs in a genealogy interface? |
| confidence, certainty, speculative | *Confidence* is how sure the researcher is. It must not collide with the word you pick for a source's reliability. |
| source, primary source, reliability, evidence | Archive traditions differ on what counts as *primary*. Use the term your national archives use. |
| occupation | A period with a start and an end, not a job title. Some languages have a separate word for each. |
| record, entry, archive | *Record* is one person's page. *Archive* is the whole file the family owns. |
| visibility, members, contributors, private | These are permission levels. Prefer the plainest word over a technical one. |
| circa, before, after, between | Date qualifiers. They must stay four distinct statements. |
| generation, ancestor, descendant, sibling | Usually settled, but check the plural forms. |

Prefer the plainer word over a false-friend cognate. If you are unsure, say so
in the pull request rather than guessing — an English fallback is honest, and a
confident wrong word is not.

### Correcting a language

1. Open `locales/<tag>.ftl`. It is [Fluent][fluent] — plain text, one message
   per entry, readable without a tool. The header comment records the
   vocabulary choices that were made and invites you to overrule them.
2. Fix what is wrong. English (`locales/en.ftl`) is the source of truth for
   which messages exist and what each one means; French is complete and
   reviewed, so it is often the better guide to what an English message
   actually intends.
3. Run `cargo test`. It will tell you if a message is malformed, if a key
   exists that English does not have, if a plural category is missing, or if
   the coverage number has moved.
4. Open a pull request. Say which language you speak and how much of the file
   you read — "I checked the vocabulary table and the dates" is a useful
   review; it does not have to be the whole file to be worth merging.
5. When a language has been read end to end by somebody who speaks it, set
   `reviewed: true` for it in `src/i18n.rs`. There is a test that pins the list
   of reviewed languages, so it will fail and you will have to change it
   deliberately — which is the point.

[fluent]: https://projectfluent.org/

### Plural forms

Fluent carries the CLDR plural rules, so write out the forms your language
actually has rather than the two English has:

    tree-hidden-notice = { $n ->
            [one] Jedna osoba jest pokazana bez szczegółów
            [few] { $n } osoby są pokazane bez szczegółów
            [many] { $n } osób jest pokazanych bez szczegółów
           *[other] { $n } osób jest pokazanych bez szczegółów
        }

Polish and Russian have `one`/`few`/`many`/`other`; Arabic has all six —
`zero`/`one`/`two`/`few`/`many`/`other`; Chinese and Japanese have only
`other`, and that is correct rather than lazy. The `*` marks the fallback
branch and is required.

A missing category does not fail loudly: Fluent falls through to `*[other]`
and renders a sentence that is quietly ungrammatical. `no_catalogue_is_missing
_a_plural_category` exists for exactly that, and it asks the running Fluent
stack which categories your language needs rather than consulting a table.

### Dates

Month names live inside each locale's own `date-day-month-year` and
`date-month-year` patterns rather than in a shared table, because a shared
table cannot be right for every language at once:

- Polish and Russian inflect the month inside a full date. `12 kwietnia 1923`
  and `12 апреля 1923` are genitive; the same months standing alone are
  `kwiecień 1923` and `апрель 1923`. Both forms are written out per locale.
- Spanish and Portuguese carry their prepositions: `12 de abril de 1923`.
- German takes a point after the day: `12. April 1923`.
- Chinese and Japanese do not name months at all. `1923年4月12日` is a numeric
  structure, so those patterns use the month number directly.

The application hands the pattern a day, a month **number** and a year. What
your language does with them is yours to decide.

### The rule that matters most

**Translate the interface. Never translate the data.**

An English speaker browsing a Polish family wants English buttons and Polish
place names. Names, places, notes, occupations and source titles come from the
archive and are rendered in their own language and script whatever the
interface is set to. A locale file only ever contains labels, headings, help
text, messages, and the specification's own enum values said out loud.

A date is the subtle case. Its *words* are translated — `circa 1500` becomes
`vers 1500` — but its value and its precision are data and are never touched.
A date the source left at year precision stays at year precision; a partial
date stays partial. Never "improve" one.

### What the tests enforce

- No template may contain a literal English string, pass one into a macro, or
  put one in a `title`/`aria-label`/`placeholder` attribute.
- Every key a template asks for exists in English.
- No locale defines a key English does not.
- No catalogue is missing a plural category its language requires.
- No catalogue renders a count without a selector where English uses one.
- The coverage figure the selector shows is recomputed from the files.
- A language marked `reviewed` is complete.

## Right-to-left

Arabic is served with `dir="rtl"` on the document, rendered server-side so the
page arrives mirrored rather than flipping after a script runs. The stylesheet
uses logical properties (`margin-inline-start`, `inset-inline-start`) rather
than `left`/`right`, so the layout mirrors without a second stylesheet.

Two things do not follow from that automatically and are handled explicitly:

- **The tree canvas.** Cards carry `inset-inline-start` and mirror on their
  own; the SVG that connects them is drawn in absolute coordinates and is
  reflected with `transform: scaleX(-1)`.
- **Data inside translated prose.** A rendered date mixes translated words,
  digits and neutral marks — `>1930`, `1826–1830` — and the bidirectional
  algorithm reorders neutrals, turning `>1930` into `1930<`, which is a
  different claim. Dates and names are wrapped in `<bdi>`. A message that fell
  back to English inside an RTL page is wrapped in Unicode isolates for the
  same reason.

If you add a component, check it in Arabic before calling it done. Rendering
the page found three bugs that reading the CSS did not.

## Running the tests

    cargo test
    cargo clippy --all-targets -- -D warnings
    cargo fmt --check

## Everything else

All genealogy logic lives in [axgf-rs](https://github.com/plkarin/axgf-lib).
This application reads a bundle, calls a library function, writes it back and
renders HTML. If a change needs new genealogy behaviour, it probably belongs
there rather than here.
