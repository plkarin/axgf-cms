# ADR 0001 — The river's invariants

**Status:** accepted · **Applies to:** `src/river.rs`, `src/access.rs` (the
river's two projections), `static/river.js`

The river (`/`) draws one person at a fixed point, ancestors below and
descendants above. Four of its rules look like mistakes. Each was chosen on
purpose, each has been "fixed" once already in the course of building it, and
each is now held by a test. Change one only by changing its test first, and
read why it is there before you do.

---

## 1. Width means recorded descendants, and it does not sum at a confluence

**Decision.** A line's width is a discharge class looked up from a person's
recorded descendants + 1 (a family's: its distinct descendants): 1 · 2–5 ·
6–20 · 21–80 · 81+, drawn 1.6 · 3 · 5 · 8 · 12. A lookup, never `sqrt`. Width
means nothing else.

**What looks wrong.** Where a father's line and a mother's line meet at a
union, the line below is not their sum. Both parents' lines are drawn at the
full width of the family, and so is the child's.

**Why.** Water is conserved and people are not. Everyone has two parents, so
going upstream a current does not gather tributaries — it forks, and the same
descendants flow down the father's line and the mother's. Summing at the
confluence would count every descendant twice per generation. Any other
meaning for width ("ancestors", "people in the file") stops being meaningful
the moment two lines meet, which is why the answer to a second meaning is no.
Quantised classes are testable, and stay comparable between two screenshots a
year apart in a way a continuous scale does not.

**Enforced by** `river::tests::width_is_descendants_and_does_not_sum_at_a_confluence`
and `river::tests::classes_come_from_the_lookup_not_a_square_root`
(`src/river.rs`).

---

## 2. Sibling order uses birth dates only when the reader may see every sibling

**Decision.** The children of one family are drawn in birth-date order only
when the reader may see every one of them; undated children keep their
recorded places. If any sibling is hidden from the reader, the whole sibship
is drawn in the order the family records it (`birth_order`, then the order of
its `children` list), and no date is read. `Graph::order_for` does this, per
reader.

**What looks wrong.** The same family can be drawn in birth order for an
administrator and in some other order for a visitor; and for a visitor a
family can appear out of birth order although every date it shows is
correct.

**Why.** A hidden person's dates must not change anything a visitor receives.
Sorting a sibship that contains a hidden child by date moves the visible
siblings according to that child's birth date, and puts two hidden siblings
in the order of their ages — a statement about people the reader may not see.
Recorded order reads nothing hidden. Recorded order is not birth order often
enough to matter — on the operator's 866-person bundle 57 of the 120 families
with two or more children disagree (the worst, 14 children, 37 inversions; no
family uses `birth_order`) — which is why the date is used everywhere it
safely can be. Measured on the same bundle, ordering by date where visible
also gives fewer long horizontal runs than either all-by-year or
all-recorded.

**Enforced by** `hidden_peoples_dates_never_change_what_a_visitor_receives` —
two bundles that differ only in hidden people's dates render byte-identically,
SVG and payload, from every centre, at every range, signed in or out — and
`siblings_follow_birth_dates_only_where_the_reader_sees_every_sibling`
(`tests/river.rs`).

---

## 3. The year scale needs an anchor the reader may see

**Decision.** The half-century bands and the colour ramp are placed by a year
scale anchored on a birth year the reader may see: the centre's, if visible
and recorded, otherwise the nearest visible dated person's, shifted 29 years
a generation. Everyone the reader may not see, or who has no recorded birth
year, is coloured by that anchor plus their generation. With no anchor the
river has no era: no bands, one neutral colour, and no year of any kind in
the page. Bands are drawn only for a signed-in reader.

**What looks wrong.** An administrator looking at an isolated, undated person
gets no bands at all; a signed-out visitor never does; and hidden people are
coloured by an estimate when their real birth year is right there in the
bundle.

**Why.** Every band, every colour along the water and every year in the
browser's payload is a statement about when someone was born. Three times a
version of this code leaked a hidden person's era — through tail and stub
colours, through band positions anchored on a hidden centre, and through a
"century-rounded" fallback computed from that same hidden year — and once it
drew a wrong era outright, around a hard-coded 1900. Rounding is not hiding,
and an estimate is not a fallback when it is wrong. A river that cannot place
itself in time says so by drawing no time.

**Enforced by** `with_no_visible_year_the_river_has_no_era_at_all`,
`a_signed_out_visitor_gets_no_bands`,
`the_signed_out_page_has_no_bands_and_the_signed_in_page_does`, and the
invariant in §2, which covers every route by which a hidden date could reach
the page (`tests/river.rs`).

---

## 4. The terminator is the same for the living and the dead

**Decision.** Every line the record does not continue — a childless leaf, a
missing parent, the end of a recorded pedigree — ends in the same faded line
and open ring, whether or not the person is living. The legend calls it
"record ends". A line that only leaves the frame is different: solid, with an
arrowhead and a count.

**What looks wrong.** A living twenty-year-old with no children ends in the
same ring as a line that died out in 1850, and "record ends" sounds final for
somebody who may yet have children.

**Why.** The reference drew the ring only for the dead. That made the ring
above a redacted, childless person say whether they were alive — the one fact
about a living person the redaction beside it exists to withhold. The ring
states what the record holds, not what will happen: there are no recorded
descendants. Living status is therefore not an input to the layout at all
(`access::RiverShape` does not carry it). Do not reintroduce a second
terminator for the living; it is the same leak.

**Enforced by** `a_lost_line_ends_the_same_way_whether_or_not_the_person_is_living`
(byte-identical SVG and payload for a hidden childless person, living or not)
and the legend assertion in `the_river_is_drawn_on_the_server`
(`tests/river.rs`).

---

## How these were verified

Each test above was confirmed red with only its fix reverted, before the fix
was kept.
