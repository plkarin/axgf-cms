# ADR 0036 — Impact: a life never longer than a hundred years, and the family's column within the page

**Status:** accepted (unreleased) · **Amends:** docs/adr/0034 §2 and §3 ·
**Applies to:** `src/impact.rs`, `templates/impact.html`,
`templates/impact_person.html`, `static/impact.js`, `static/app.css`

## Context

On 8 November 2026, looking at the timeline in production, the operator
asked for two things.

**A life must stay plausible.** One person's view said "aged 322" beside
the invasion of Ukraine. Their record said they were living, and that they
were born about 1700, so their window ran to today. Her rule:

- a life that starts at a date but has no end is not drawn to the present.
  It gets a hundred years at most, from the start;
- a life known only "about" a date gets a window from fifty years before it
  to fifty years after.

The archive is a GEDCOM conversion. The converter marks living every person
without a death (axgf-rs: `is_living = !died`), so 570 of the archive's
records say living. 150 of them have a birth more than a hundred years ago.
The record pages already presume those people dead after 120 years
(`--presume-deceased-after`); the timeline did not. Its longest window was
477 years, and 150 windows were longer than a hundred.

**The family's column was too wide.** It took 247 lanes of 14 pixels, about
3,460 pixels. The world's columns sat beside it, and the page scrolled
sideways on her 1,470-pixel screen.

Counted on her archive, the hundred-year rule does not make the column
narrower. Most of the over-long windows end sooner, but the 142 pins
become hundred-year windows. Around 1926, 298 lives overlap, and lives that
overlap cannot share a lane. Only the lanes' width can change.

## Decision

### 1. Every life is a window

| What the record gives | The window | Its ends |
|---|---|---|
| a birth and a death | from the one to the other, as recorded | an approximate date fades a little |
| a birth, living, born no more than 100 years ago | from the birth to today | living |
| a birth only, or "living" but born more than 100 years ago | from the birth, a hundred years | the end fades over its last 40 % |
| a death only | the hundred years before it | the start fades over its first 40 % |
| only events (a residence, a baptism as a witness…) | fifty years either side of their middle, and at least from the first to the last | both ends fade |

- **Never past today.** A window that would end after today ends today.
- **A life recorded with both dates stays as recorded**, even when it is
  longer than a hundred years. The record is evidence; an estimate is not.
- **No more pins.** A pin said "one moment"; a window says when the person
  could have lived, and lights the landmarks they could have seen.
- **An age needs a birth.** A window estimated from a death or from events
  says when they lived, not how old they were: no age is written beside a
  landmark in their view.
- **A record said living with a death date**, when it was born more than a
  hundred years ago, takes its death date.

The constant is `impact::LONGEST_LIFE`, a hundred years. Unit tests hold
each row of the table.

### 2. The family's column fits the page

- **Its width.** The column takes the room the page leaves. It is at least
  16 rem, or its fourteen-pixel width when that is less, and at most
  fourteen pixels a lane. The page scrolls sideways only below the minimum.
- **Its lanes.** A lane is `min(14px, column ÷ lanes)` wide; a life is 72 %
  of its lane, never under one pixel. On the operator's screen, 238 lanes of
  a fictitious crowd fit in 366 pixels, without sideways scrolling.
- **The life pointed at is the one nearest the pointer.** A lane can be one
  pixel wide, too narrow to aim at, so `impact.js`:
  - takes the lane under the pointer at its height, or the next ones, out to
    about six pixels either side;
  - lights that life, which widens to four pixels above the others, its band
    and its name;
  - follows the nearest life on a click between two lanes.
- **Unchanged:** a life is still a link to the person's view, reached by Tab,
  and named by its title for a reader with a screen reader.

## Consequences

- No view says an age over a hundred for a life missing a date. On the
  operator's archive, the 150 "living" records born long ago end a hundred
  years after their birth.
- The family's column reads as the shape of the family through time: thin
  where few lived at once, dense where many did.
- The converter's `is_living = !died` is axgf-rs's to change, for every
  reader of the format. The timeline does not rely on it any more than the
  record pages do.
