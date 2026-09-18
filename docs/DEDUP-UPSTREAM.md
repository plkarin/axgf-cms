# `deduplicate()` refuses a duplicate couple whose type is unrecorded

A report for `axgf-rs`, written from `axgf-cms` against
`axgf-lib@2815e299c0aaf7a38a0daeee98fb664cf42091de` (0.4.0) and the operator's
866-person bundle. Nothing here is worked around in the CMS: all genealogy
logic lives in the library, and merging two families is genealogy.

## What happens

`validate()` reports three `DUPLICATE_UNIQUE_REF` diagnostics on this bundle —
three pairs of Family entities sharing a spouse set. `deduplicate()` merges
one of the three and refuses the other two with `MANUAL_REVIEW_REQUIRED`.

| pair | `union.type` | `union.start` | outcome |
|------|--------------|---------------|---------|
| `aec2bbe4…` / `cce10ed4…` | `unknown` / `unknown` | – / – | **merged** |
| `43ad8281…` / `73d5c839…` | `unknown` / `marriage` | – / 1911 | **refused** |
| `e823f9e2…` / `f6c1c9ae…` | `unknown` / `marriage` | – / 1991-08-24 | **refused** |

The third is the couple on the operator's own family: Janusz Kasprzyk and
Saturnina Majcher, both records naming the same child.

## Why

`logic/dedup.rs::is_ambiguous_family_group` collects the group's union types
into a `BTreeSet<String>` and refuses the group when the set holds more than
one value:

```rust
if let Some(t) = f.get("union").and_then(|u| u.get("type")).and_then(Value::as_str) {
    union_types.insert(t.to_string());
}
…
if union_types.len() > 1 {
    return true;   // ambiguous: not merged
}
```

`"unknown"` goes into that set as an ordinary value, so `{"marriage",
"unknown"}` has two members and the pair is refused. But `unknown` is not a
different kind of union — it is the absence of a recorded kind, and an absence
does not disagree with anything.

## The absence the code already tolerates is one a valid bundle cannot express

The function is already tolerant of a *missing* `union.type`: the `if let`
simply does not insert, the set stays at one member, and the group merges.
Measured, by mutating the operator's bundle and re-running `deduplicate()`:

| `union.type` on the unrecorded side | result for the pair |
|---|---|
| `"unknown"` (as the bundle actually is) | **refused** |
| key removed entirely | **merged** |
| `null` | **merged** |
| `"marriage"` (control: both agree) | **merged** |

So the tolerance exists — and is unreachable. The AXGF schema (`schema/*.json`
in the bundle) makes `union.type` **required** and lists `unknown` in its
enum:

```json
"union": {
  "required": ["type", "persons"],
  "properties": {
    "type": {"type": "string",
             "enum": ["marriage", "civil_union", "cohabitation",
                      "religious_only", "polygamous", "unknown"]}
```

A schema-valid bundle therefore *cannot* say "the kind of union is not
recorded" by omitting the key. It must write `"unknown"` — which is precisely
the value the merge rule treats as a disagreement. 186 of this bundle's 331
families carry it.

## What the library would need to change

Treat the `unknown` sentinel as the absence it is, in the same place that
already treats a missing key as an absence:

```rust
if let Some(t) = f.get("union").and_then(|u| u.get("type")).and_then(Value::as_str) {
    // `unknown` is the schema's way of writing "not recorded" — `union.type`
    // is required, so a conforming bundle has no other way to say it. An
    // absence does not disagree with a recorded type.
    if t != "unknown" && !t.is_empty() {
        union_types.insert(t.to_string());
    }
}
```

That takes this bundle from one merged family and two manual-review
diagnostics to three merged and none.

## But that change alone would lose data, and must not ship on its own

`merge_family_records` iterates the victim's **top-level** keys and fills the
keeper only where the keeper has `None` or `Null`:

```rust
for (key, val) in v_obj {
    …
    match k_obj.get(key) {
        None | Some(Value::Null) => { k_obj.insert(key.clone(), val.clone()); }
        _ => {}
    }
}
```

`union` is one such top-level key. The keeper already has a `union`, so the
victim's `union` is skipped **entirely** — not merged field by field. And the
keeper is the lowest-UUID family, which for this pair is `e823f9e2…`: the thin
record.

Measured, by applying the ambiguity fix above and letting the pair merge:

```
A survives=true  B survives=false
KEEPER e823f9e2-8817-4d44-aabb-8b3999dcffe6
  union = { "confidence": 0.8, "persons": [ …Janusz…, …Saturnina… ] }
  children = [ {"person_id": "5eb76a82…", "confidence": 0.8} ]
```

The surviving union has no `type`, no `status`, and **no `start`** — the
24 August 1991 date, its `place_id` and its `event_id` are gone. The pair's
only real information was in the record that got discarded.

So the ambiguity fix on its own converts a visible, reported duplicate into
silent data loss, which is worse than the refusal it replaces. Both changes are
needed together:

1. **Treat the `unknown` sentinel as absent** in `is_ambiguous_family_group`,
   so a recorded type and an unrecorded one stop counting as a disagreement.
2. **Merge `union` field by field**, as `children` and `documents` already get
   their own handling, so the keeper gains `type`, `status` and `start` from
   the other record wherever it lacks them — and, applying the same rule as
   (1), wherever it holds only `unknown`.

With both, this pair merges to a family that keeps the marriage, its date, its
place and its event, and loses nothing.

## Reproducing

`deduplicate()` over the bundle's flat JSON; the pair is
`e823f9e2-8817-4d44-aabb-8b3999dcffe6` and
`f6c1c9ae-82b2-44b6-92fc-390c85566f65`. Both survive, and the diagnostic reads:

```
families [...] share spouse set [...] but disagree on union.type or
start.date year (> 1 year apart) — manual review required, not merged
```

The start dates do not disagree here: only one of the two records has one, and
the rule compares the min and max of the years it found, so a single year
cannot span more than zero. `union.type` is the whole of it.
