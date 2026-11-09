# ADR 0035 — Impact: the years, the windows and the flags

**Status:** accepted, v1.10.0 (2026-11-08); extended by docs/adr/0038, the
windows in the reader's language · **Builds on:** Impact, the Universal
Synoptic Timeline (docs/adr/0034) · **Applies to:** `src/impact.rs`,
`src/impact_details.txt`, `src/impact_places.txt`, `static/flags/`,
`src/routes/impact.rs`, `src/person.rs`, `templates/_impact.html`,
`templates/impact*.html`, `templates/person.html`,
`templates/_person_detail.html`, `templates/_macros.html`,
`static/impact.js`, `static/app.css`, `locales/*.ftl`, `tests/impact.rs`

## Context

On 6 November 2026, looking at the timeline in production, the operator
asked for four things:

1. pointing at a landmark should light its place on the timeline, to give a
   sense of its time. A label pushed below its year, as the Chinese Civil
   War's was, showed nothing of when it was;
2. one graduation, by the year, for the whole timeline;
3. a click on a landmark should open a window explaining briefly its
   context, what was at stake and its victims;
4. a country's flag beside it, on the timeline and on the persons'
   records.

Her screenshot also showed a person of the family with "no country found in
their places". Of the archive's 123 places, 101 named no country the
timeline could read: 85 of them a bare village name ("Słomniki"). The 123
places' language tags were all "pl", the French ones' too, so a tag says
nothing of the country. 218 of the 423 persons with a place had a country.

## Decision

### 1. A landmark pointed at

Each label carries its true years in pixels (`data-top`, `data-height`). The
script uses them when the label is pointed at or reached by keyboard:

- a band runs across every column at those years;
- a segment on the axis shows them, with the dates written on it;
- the landmark's bar is lit.

It works in both views, on a label pushed below its year as well.

### 2. A graduation by the year

- **The marks.** `Scale::grid` gives a mark for every year wherever a year
  has four pixels: on a person's view, on the family's centuries, and on the
  whole of history from 1800 on. Where a year has less, it gives a mark for
  every five, ten, twenty-five… years.
- **What is drawn.** Each mark is a line across every column and a tick on
  the axis; every tenth one is stronger.
- **The written years** keep their own spacing, so that they never crowd.

### 3. A landmark's window

- **The content.** Every landmark of the catalogue, 313 of them, has its
  details in `src/impact_details.txt`, in English and French:
  - the context, in one or two sentences;
  - what was at stake;
  - the victims, where there were any to count.

  The figures are the ranges historians give ("70 to 85 million dead, most
  of them civilians"), said as ranges.
- **The tests.** A unit test checks that every landmark has its details in
  both languages, and that no detail names a landmark that is not there.
- **Choosing a label.** A click, Enter or Space opens one `<dialog>` for the
  page. It shows the label's own content, which the page drew hidden inside
  the label: the name, the dates, the length, the person's age and the
  country note, then the three parts. Escape, the button or a click outside
  closes it.

### 4. The flags

- **Images, not emoji.** Windows shows no flag emoji: it writes the two
  letters. So the flags are SVG images: flag-icons 7.2.3 (MIT, licence in
  `static/flags/LICENSE`), only for the 47 countries the resolver can name.
- **Serving them.** The binary carries them, served at
  `/static/flags/<code>.svg` and nowhere else.
- **On the timeline**, a flag goes beside the person's countries, their
  country's column heading and each landmark tagged with a country.
- **On a record**, a flag goes beside every place: the "born in, died in"
  line, the life events and the places section.
- **Present-day flags only.** The flag is the place's present-day country's.
  A historical state's flag ("Congress Poland") would need a set of its own;
  the record's `country_history` already names those states in words.

### 5. A village's country

The resolver (`country_of_place`) gains two steps, after the place's stated
country, the last part of its name and the well-known cities:

- **the place's region**, when it names a country, as a GEDCOM conversion
  sometimes left it ("Pologne");
- **a gazetteer:** every populated place of Poland, France, Belgium,
  Ukraine, the DR Congo and Haiti, from GeoNames (CC BY 4.0).
  - It holds 163,000 folded names, 1.8 MB, read once.
  - A name found in two of those countries is left out, so a village is
    never given to the wrong one.
  - A word that names a country or a continent ("Congo") is never looked
    up as a village.

On the operator's archive this finds a country for 67 more places, and
brings the persons with a country from 218 to 379 of 423.

## Consequences

- A record page shows flags, and the timeline's person view finds its
  countries for most of the family.
- The gazetteer and the flags are data of others, credited in the README and
  beside the files. Adding a country means rebuilding the gazetteer
  (`scripts/impact-gazetteer.py`) and adding its flag.
- The windows' texts are in English and French, like the landmarks' names;
  the twelve other languages read them in English for now.
