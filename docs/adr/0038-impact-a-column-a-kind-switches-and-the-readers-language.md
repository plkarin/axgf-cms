# ADR 0038 — Impact: a column a kind, kinds to switch, the great inventions and discoveries, the reader's language

**Status:** accepted, v1.11.0 (2026-11-09) · **Builds on:** docs/adr/0034, 0035,
0036 · **Applies to:** `src/impact.rs`, `src/impact_events.txt`,
`src/impact_details.txt`, `src/impact_i18n/`, `templates/_impact.html`,
`templates/impact_person.html`, `static/impact.js`, `static/app.css`,
`locales/*.ftl`

## Context

On 8 and 9 November 2026 the operator asked four things of Impact:

1. **More columns.** A person's view laid the world's and their
   continent's landmarks in one column, every kind mixed. The operator
   wanted the kinds side by side, "even if the columns are narrower".
2. **New kinds:** the great inventions (the rabies vaccine…) and the
   great discoveries (the Americas, the first steps on the Moon…).
3. **Switches:** to choose which kinds the timeline shows.
4. **The reader's language.** A Polish reader read the landmarks and their
   windows in English. Only English and French were written. The operator
   asked for every language the site speaks.

## Decision

### 1. A column a kind on a person's view

- **The columns.** The world's and their continent's landmarks are laid a
  column a kind (`KindColumn`), in the order of `Kind::ALL`. A kind with
  nothing in the person's years has none.
- **Their sizes:**
  - a kind's column is 104 pixels wide, plus a few for the bars of its
    long events. A name takes two lines before it is cut;
  - their life's column is 11rem, their country's 236 pixels.
- **The result:** on the operator's 1,470-pixel screen, a life with eight
  kinds in its years fits without scrolling sideways (measured on a
  fictitious record). With more, the page scrolls sideways, until a kind
  is switched off.

### 2. The great inventions and discoveries

- **Two kinds**, `invention` and `discovery`, each with a colour of its
  own (`--hue-5`, `--hue-8`) and a name in the fourteen languages.
- **Forty landmarks** with their windows:
  - inventions: the wheel, paper, movable type, the telescope, the steam
    engine, the smallpox and rabies vaccines, the battery, photography,
    the telegraph, the telephone, the bulb, the car, cinema, the
    aeroplane, the computer, the transistor, the polio vaccine, the Web,
    the smartphone, mRNA vaccines;
  - discoveries: Vasco da Gama, Magellan, Copernicus, Galileo, Newton,
    Cook, Darwin, Mendeleev, X-rays, Marie Curie, Einstein, the South
    Pole, penicillin, DNA, Everest, Sputnik, Gagarin, the genome, the
    Higgs boson.
- **Three landmarks change kind:** Gutenberg's press becomes an invention,
  Columbus and the first steps on the Moon become discoveries. Their keys,
  their year and English name, do not change.

### 3. Kinds to switch

- **The switches.** Once `impact.js` runs, the legend's kinds become
  switches (`button`, `aria-pressed`). Without scripts the legend stays a
  legend: the page holds no control that does nothing.
- **A kind switched off is hidden on both views:**
  - its landmarks and their bars (`.ip-hide-<kind>` on the grid);
  - on a person's view, its column too. The grid closes up over it:
    each column says its width in `data-cols`.
- **Where the choice lives.** The reader's browser keeps it
  (`localStorage`, `axgf-impact-hidden`). It is a reader's own
  convenience, so it is never shared.

### 4. The landmarks in the reader's language

- **Where the texts live.** English and French stay in the catalogue. Each
  other language has a file of its own, `src/impact_i18n/<lang>.txt`.
  - Under `## <key>`, it gives `name|`, `note|`, `context|`, `stakes|`
    and `victims|`.
  - A landmark or a field it leaves out reads in English.
- **All twelve languages**, for all 353 landmarks: Polish, German,
  Spanish, Italian, Portuguese, Russian, Chinese, Japanese, Arabic,
  Hebrew, Swahili and Lingala. They are a machine's translations
  (Claude), not yet proofread by native speakers, as each file says.
  Swahili and Lingala are the least sure.
- **The tests:**
  - every language the site speaks, French and English aside, has its
    file;
  - every key of a file names a landmark;
  - every file names and explains every landmark, with its note and its
    victims where English has them;
  - a reader reads their own language, and a language the site does not
    speak reads English.

## Consequences

- A person's view says what kind of history each landmark is at a glance,
  and a reader can leave out what does not interest them.
- Each language is about 10,900 English words translated. A landmark
  added later reads in English in every file that leaves it out, until
  its lines are written there.
- A language added to the site needs its file: the test fails without
  it.
