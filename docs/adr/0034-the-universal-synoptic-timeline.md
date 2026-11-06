# ADR 0034 — Impact, the Universal Synoptic Timeline

**Status:** accepted, v1.9.0 (2026-11-06) · **Builds on:** the lens (docs/adr/0001),
the record's places and their states (`country_history`), the words diet
(docs/adr/0032) · **Applies to:** `src/impact.rs`, `src/impact_events.txt`,
`src/routes/impact.rs`, `templates/impact.html`,
`templates/impact_person.html`, `templates/_impact.html`,
`templates/base.html`, `templates/person.html`, `static/impact.js`,
`static/app.css`, `locales/*.ftl`, `tests/impact.rs`

## Context

On 6 November 2026 the operator specified a "mega feature": a vertical,
interactive timeline for the family's lives against the history of the
world. Its specification asks for:

- time running from top to bottom, ancient civilisations at the top and
  the present at the bottom;
- the world's landmarks along it: civilisations, migrations, epidemics, and
  wars with their length. The operator gave a repertoire from -10000 to now
  and asked for more;
- each ancestor as a window from birth to death, or as a pin where only an
  approximate moment is known, laid in columns so that they do not overlap;
- the crossing: across one band of time, the eye sees which wars and
  epidemics a life went through;
- a focus on the countries the persons lived in. Later, a reading of
  movements and periods between states and within civilisations;
- two views: the whole family (the continents as micro, the world as
  macro), and one person (their country's events explained as micro, their
  continent as macro);
- a page called **Impact**, between Family and Import in the masthead, and
  a button on the record of every person it concerns;
- models without Canva for now; the design will come later.

## Decision

### 1. The catalogue

The landmarks are a text file kept with the code (`src/impact_events.txt`),
one line each:

```
kind|from|to|scopes|name in English|name in French|note in English|note in French
```

- **Kinds:** era, war, epidemic, migration, event, economy, disaster. Each
  kind takes one of the theme's hues; no colour is named.
- **Dates:**
  - a year (`-3500`), or a year, month and day (`1914-07-28`);
  - `to` empty for a moment, `now` for what goes on;
  - with a day, `to` is the day after the last one, so a length reads as the
    history books give it: "6 years 1 day" for the Second World War, "6
    days", "100 days".
- **Scopes:** `world` (the operator's repertoire, on every timeline), a
  continent, or a present-day country's ISO code.
- **Content:**
  - the operator's repertoire in full, with the lengths computed from the
    dates;
  - about forty more world landmarks (the Black Death's successors, the
    Great Depression, the Holocaust, cholera, Tambora…);
  - each continent's own;
  - a first set of countries, each landmark with a one-line explanation:
    France, Poland, Belgium, the two Congos, Haiti, Germany, Italy, Spain,
    Portugal, the United Kingdom, Ireland, Russia, Ukraine, the United
    States, Algeria.
- **Languages:** names and notes in English and French; the other twelve
  languages read them in English until they are translated. The page's own
  words are in all fourteen.
- A unit test reads every line: kind, dates, scopes, a name in both
  languages, a key used once.

### 2. The lives

A person's life comes from their record, through the reader's lens:

- **a window** from birth to death when both are known. A living person's
  runs to today. A date given as approximate ("circa", "before", "between")
  fades its end;
- **a pin** where one moment only is known: the birth (°), the death (†),
  or else the earliest event the person took part in (~);
- **not placed** when nothing is dated. They are counted ("Without a
  date: n").

A person the reader may not read is counted ("Not shown: n"), never named or
placed. Their own view answers 403, as their record does.

**Their countries** come from their places (birth, death, burial, baptism,
and every event they took part in), the commonest first:

1. the country a place states, as an ISO code or a name;
2. else the last part of its name ("Lyon, Rhone, France");
3. else a well-known city in it ("Warszawa", "Kinshasa").

"Congo" alone names no country, since it could be either one: better none
than the wrong one. The country is the present-day one; the states that
held the place at the time are shown from its `country_history`, when the
record has one (§4).

### 3. The axis and the placing

- **No canvas, and no Canva.** Each block of text is placed at its height by
  a style computed in Rust. The page is HTML and CSS; the Content Security
  Policy already allows inline styles.
- **The family's view: the whole of history.** The scale is piecewise
  linear, to keep the family's centuries readable:
  - prehistory folded to 210 pixels;
  - antiquity at 0.12 pixels a year, the Middle Ages at 0.35 to 0.7;
  - the early modern period at 2;
  - the last two centuries at 7 pixels a year.
- **The family's view: its own centuries.** The other choice runs straight
  from thirty years before the earliest life to today.
- **A person's view** runs from twenty years before their birth to ten after
  their death. Its scale grows, to 24 pixels a year at most, until each
  label sits near its year.
- **Periods** are bars at their true years, packed in narrow lanes.
- **Labels:**
  - each label sits at its year, in the first lane free there;
  - where every lane is taken, it is pushed below and a dotted thread joins
    it to its year;
  - nothing is drawn over anything else (unit test).
- **Lives** are packed the same way, a narrow lane each, in a column that
  scrolls with the rest. The axis and the headings stay pinned while the
  timeline scrolls in its own frame.

### 4. The two views

**The family's** (`/impact`), from left to right:

- the axis;
- the world (every `world` landmark);
- the continents the family lived on, their own landmarks tagged with the
  continent;
- the family's lives.

Pointing at a life, or reaching it with the keyboard, lights the band of its
years across the page (`impact.js`). The landmarks it crossed stand out, the
others step back, and the name and years show beside the pointer. Following
the life opens its view.

**A person's** (`/impact/:id`), from left to right:

- the axis;
- the macro side: what touched the whole world and what touched their
  continent. The Qing are not shown to a Lyonnais; the repertoire's other
  continents stay on the family's view;
- their life, with its milestones: birth, marriages, the children's births
  (each child a link, when the reader may read them), death;
- the micro side: their countries' landmarks, each with what it meant.

Every landmark says the person's age when it began ("aged 24"), or "under
way at their birth". Those outside their life step back. Above the timeline:

- the person's years and countries;
- the states that held their places while they lived, when the places
  record them (`country_history`: "Kingdom of France (–1792) → French First
  Republic (1792–1804) …").

That is the first reading between states that the specification asks for.

### 5. Where it is reached

- **Impact** is the masthead's fifth section, between Family and Import. The
  masthead's cap grows from ten controls to eleven (`tests/interactive_walk.rs`).
- A record whose person can be placed offers **See the impact** beside the
  way to the river.

## Consequences

- The specification's later part, a reading of movements and periods across
  states and civilisations, starts here (the states that held a place) and
  grows with the catalogue: more countries, more continents, and
  translations of the landmarks into the twelve other languages.
- A place known by a name alone, in a village no list knows, gives no
  country; filling the place's country in its editor fixes it, and the
  timeline follows.
- The design (pictures, a drawn axis) is to come, as the operator said; the
  page's structure will not need to change for it.
