# ADR 0003 — An absent value has no default position

**Status:** accepted · **Applies to:** every form that records a fact
(`templates/admin_person.html`, `templates/admin_form.html`, the structured
editors in `src/routes/`, `src/routes/person_editor.rs`)

**Status of the code (2026-10-15).** The person editor complies since v0.4.2.
The generic form (`templates/admin_form.html`, used for families, events,
sources, places, documents and for creating a person) still draws a
confidence as a slider. That use is open, and this rule is what it is held
to.

---

## The defect

From the first administration panel (2026-08-07, `c9b98c1`) until v0.4.2,
the generic form drew every confidence (`FieldKind::Confidence`) as

```html
<input type="range" min="0" max="1" step="0.05" value="">
```

with a live readout that said "not set" when the record held none.

A range input cannot hold no value. HTML's value sanitisation gives a range
whose value is missing or not a number its default value: the midpoint,
`min + (max − min) / 2`, here **0.5**. The thumb sits in the middle and the
form submits `0.5`. `apply_form` then stores it, because a number arrived. So
saving a record whose confidence nobody had recorded wrote one: 0.5, "as
likely as not", attributed to whoever pressed Save. The readout beside the
slider said "not set" while the control posted the opposite.

The user who saved never touched that control. Nothing on screen told them
it was about to write. Nothing in the journal marks the value as anything
other than a judgement somebody made.

**What it reached.** The production journal records no write of 0.5 to any
confidence. The archive holds two 0.5 confidences (both `death.confidence`),
and both were already there in the 18 September copy, from the import, before
any form edited it. Production has seen few form saves, which is the only
reason this is a finding and not a repair.

## Why it matters more here than anywhere

Confidence is what separates AXGF from GEDCOM. GEDCOM carries a date. AXGF
carries a date and how sure the record is of it, and the difference between
"0.5" and "nobody said" is the whole of that distinction:

- **0.5 is a claim.** Somebody weighed the evidence and found it balanced.
  The river draws it as a dashed line, the radars read it, and a later
  researcher defers to it.
- **No confidence is an absence.** The record states the fact and does not
  grade it. AXGF keeps the two apart, and the rest of this application
  draws them apart: an absent radar axis is a dotted spoke, never 50.

A control that turns the absence into the claim on every save erases that
distinction one record at a time. It does so silently, under the name of
whoever happened to edit something else on the page.

## Why a slider cannot represent an absent value

A slider is a position on a track, and every position is a value. It has no
position that means "not on the track":

- The DOM gives a range input no empty state. Its `value` is always a
  number, and the form always submits one.
- The thumb has to be drawn somewhere. Wherever that is (the middle, the
  minimum, the last value used), it reads as that value.
- "Not set" written beside it is a label the control contradicts. Moving the
  thumb back to "nothing" is impossible once it has been moved at all.

The same holds for any control with a default position: a radio group with
one option pre-checked, a select whose first option is a real value, a
checkbox standing for a three-valued fact.

## The rule

**A field whose absence is meaningful never gets a control with a default
position.**

- Its control must be able to show and to submit "nothing recorded" as
  itself: an empty text or number input, a select whose first option is
  "— not set —" with the value `""`, a checkbox only where unchecked
  genuinely means false.
- Submitting the form untouched must post, for that field, exactly what the
  record holds, and nothing for what it does not.
- Clearing the field must be possible after it has been filled, with the
  same control.

A confidence is drawn as `<input type="number" min="0" max="1" step="0.05">`,
empty when the record holds none (`src/routes/person_editor.rs`,
`generic_fields`).

**Where a default position is right.** A setting that always has a value is
not a recorded fact, and its absence means nothing. The GEDCOM import's
"confidence to assign" (`templates/convert.html`, 0.8) is such a parameter
and may stay a slider.

**How it is held.** The editor tests (`tests/person_editor.rs`,
`tests/profile_editor.rs`) submit forms the way a browser does and assert
that an untouched block writes nothing. A future test should render every
form, submit it untouched, and assert that no record changes. That is the
general form of this rule, and it would have caught the slider on the day it
was written.
