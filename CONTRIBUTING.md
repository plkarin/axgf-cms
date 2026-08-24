# Contributing

## Translations

This is where help is most useful, and where the honest position needs stating
plainly: **two of the ten interface languages have been reviewed by someone who
speaks them. The other eight have not.**

English and French are complete and read. Everything else was translated
without a native speaker, and the risk is not that a button says the wrong
thing — a wrong button is obvious. The risk is genealogical vocabulary, where
*union*, *affiliation*, *confidence*, *parentage* and *reliability* all have
established equivalents that differ by national record-keeping tradition, and a
plausible-looking wrong word is worse than an English one because nobody
notices it. The Polish date rendering already demonstrates the problem: it puts
a nominative month where the genitive belongs, so a date reads as
"12 kwiecień 1923" where a Polish speaker writes "12 kwietnia 1923".

### Where each language stands

| Tag | Language | | Messages | Coverage | Reviewed |
|---|---|---|---|---|---|
| `en` | English | English | 505 / 505 | 100% | yes |
| `fr` | French | Français | 505 / 505 | 100% | yes |
| `pl` | Polish | Polski | 216 / 505 | 43% | **no — machine quality** |
| `de` | German | Deutsch | 216 / 505 | 43% | **no — machine quality** |
| `it` | Italian | Italiano | 216 / 505 | 43% | **no — machine quality** |
| `es` | Spanish | Español | 216 / 505 | 43% | **no — machine quality** |
| `pt` | Portuguese | Português | 216 / 505 | 43% | **no — machine quality** |
| `zh-Hans` | Chinese (Simplified) | 简体中文 | 216 / 505 | 43% | **no — machine quality** |
| `ja` | Japanese | 日本語 | 216 / 505 | 43% | **no — machine quality** |
| `ar` | Arabic | العربية | 308 / 505 | 61% | **no — machine quality** |
Coverage is the share of English's messages that the locale defines. It is
computed from the files, shown in the language selector next to each
machine-quality language, and asserted by a test — nobody has to take the
number on trust. A message a locale does not define falls back to English
rather than rendering as an error, so a partial translation is a usable one.

### Correcting a language

1. Open `locales/<tag>.ftl`. It is [Fluent][fluent] — plain text, one message
   per entry, readable without a tool.
2. Fix what is wrong and add what is missing. English (`locales/en.ftl`) is the
   source of truth for which messages exist and what each one means.
3. Run `cargo test`. The test suite will tell you if a message is malformed, if
   a key exists that English does not have, or if the coverage number has moved.
4. When a language has been read end to end by somebody who speaks it, set
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

Polish has `one`/`few`/`many`; Arabic has `zero`/`one`/`two`/`few`/`many`/`other`;
Chinese and Japanese have only `other`, and that is correct rather than lazy.
The `*` marks the fallback branch and is required.

### The rule that matters most

**Translate the interface. Never translate the data.**

An English speaker browsing a Polish family wants English buttons and Polish
place names. Names, places, notes, occupations and source titles come from the
`.axgf` bundle and are rendered in their own language and script whatever the
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
