//! Public, read-only pages.

use axum::extract::{Path, Query, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use axum::Json;
use minijinja::context;
use serde_json::{json, Value};

use crate::routes::Shared;
use crate::state::COLLECTIONS;
use crate::{auth, render, view};

/// The family page's notes: what was searched, and the letter chosen.
#[derive(serde::Deserialize, Default)]
pub struct HomeQuery {
    #[serde(default)]
    q: String,
    #[serde(default)]
    letter: String,
}

/// `GET /about` — the family page: what this tree holds, the family's notes
/// and contributions for its members, and the ways in. Also what `/` shows
/// for an archive with nobody in it, which has no river to draw.
pub async fn home(
    State(state): State<Shared>,
    headers: HeaderMap,
    Query(hq): Query<HomeQuery>,
) -> Response {
    let viewer = auth::viewer(&state, &headers);
    let chrome = render::Chrome::resolve(&viewer, &headers, "/about");
    let counts = state.counts();
    let total: usize = counts.iter().map(|(_, n)| n).sum();

    // The largest words on the page. An archive that has not been named yet
    // says so in the reader's own language rather than naming the file.
    let family_name = state
        .read(|flat| {
            flat.get("manifest")
                .and_then(|m| m.get("family"))
                .and_then(|f| f.get("name"))
                .and_then(Value::as_str)
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_string)
        })
        .unwrap_or_else(|| crate::i18n::translate(chrome.lang, "home-unnamed-family", None));

    let showcase = state.read_as(viewer.ceiling(), |flat, lens| {
        showcase_highlights(flat, lens, chrome.lang)
    });

    // How sure the archive is of who is whose child (v0.5.0): the
    // confidence of every filiation this reader may see, in the record
    // page's four bands, and those nobody has rated yet.
    let trust = state.read_as(viewer.ceiling(), filiation_trust);

    render::page_with(
        &chrome,
        "home.html",
        context! {
            nav => "about",
            family_name,
            total,
            // The tile's label is a translated plural, not the collection's
            // own key: "persons" is how the bundle spells it, not how a
            // reader says it in Japanese.
            counts => counts
                .iter()
                .zip(crate::admin::KINDS.iter())
                .map(|((_, n), singular)| json!({"singular": singular, "n": n}))
                .collect::<Vec<_>>(),
            showcase,
            trust,
            // Notes and contributions (docs/adr/0014), for the family's
            // members, under the head of the page.
            notes => super::notes::family_context(&state, &viewer, chrome.lang, &hq.q, &hq.letter),
        },
    )
}

/// The confidence of every parent–child link the reader may see, in the
/// record page's bands (`view::Confidence`: certain ≥ 0.90, high ≥ 0.75,
/// medium ≥ 0.50, low below), plus the links nobody has rated — which are
/// not 0.5 or any other number: an absent value has no position (ADR 0003).
///
/// A link counts only when the reader may see the family and the child, so
/// a hidden child is not even a number here. Each band carries its share of
/// the whole, rounded, and a band with nothing in it is not drawn.
fn filiation_trust(flat: &Value, lens: &crate::access::Lens) -> Vec<Value> {
    let mut n = [0usize; 5];
    for fam in flat
        .get("families")
        .and_then(Value::as_object)
        .into_iter()
        .flat_map(|m| m.values())
        .filter(|f| lens.sees_entity(f))
    {
        for child in fam
            .get("children")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            let seen = child
                .get("person_id")
                .and_then(Value::as_str)
                .is_some_and(|id| lens.sees_person(id));
            if !seen {
                continue;
            }
            let band = match child.get("confidence").and_then(Value::as_f64) {
                Some(x) if x >= 0.90 => 0,
                Some(x) if x >= 0.75 => 1,
                Some(x) if x >= 0.50 => 2,
                Some(_) => 3,
                None => 4,
            };
            n[band] += 1;
        }
    }
    let total: usize = n.iter().sum();
    if total == 0 {
        return Vec::new();
    }
    ["certain", "high", "medium", "low", "unrated"]
        .iter()
        .zip(n)
        .filter(|(_, k)| *k > 0)
        .map(|(band, k)| json!({"band": band, "n": k, "pct": (k * 100 + total / 2) / total}))
        .collect()
}

/// Work out which of the richer kinds of detail this tree actually holds, so
/// the home page points at real examples instead of advertising what the data
/// does not exercise.
fn showcase_highlights(flat: &Value, lens: &crate::access::Lens, lang: &str) -> Vec<Value> {
    use fluent::FluentValue as F;
    let mut out = Vec::new();
    let obj = |key: &str| flat.get(key).and_then(Value::as_object);
    let t = |key: &str, args: &[(&str, F<'_>)]| -> String {
        let mut a = fluent::FluentArgs::new();
        for (k, v) in args {
            a.set(*k, v.clone());
        }
        crate::i18n::translate(lang, key, Some(&a))
    };

    // Every count here is a count of what *this* reader can reach, and every
    // "example_id" is a link, so it has to lead somewhere they may open. A
    // showcase advertising 40 relationships and linking to a wall reading
    // "Private" would be worse than not advertising them.
    if let Some(links) = obj("links") {
        let readable: Vec<&Value> = links.values().filter(|l| lens.sees_entity(l)).collect();
        if !readable.is_empty() {
            let example = readable.iter().find_map(|l| {
                l.get("from")
                    .filter(|f| f.get("entity_type").and_then(Value::as_str) == Some("person"))
                    .and_then(|f| f.get("entity_id"))
                    .and_then(Value::as_str)
                    .filter(|id| lens.sees_person(id))
                    .map(str::to_string)
            });
            out.push(json!({
                "kind": "links",
                "title": t("showcase-links-title", &[("n", (readable.len() as i64).into())]),
                "detail": t("showcase-links-detail", &[]),
                "example_id": example,
            }));
        }
    }

    if let Some(occs) = obj("occupations") {
        let readable: Vec<&Value> = occs
            .values()
            .filter(|o| lens.sees_entity(o))
            .filter(|o| {
                o.get("person_id")
                    .and_then(Value::as_str)
                    .is_some_and(|id| lens.sees_person(id))
            })
            .collect();
        if !readable.is_empty() {
            let example = readable
                .iter()
                .find_map(|o| o.get("person_id"))
                .and_then(Value::as_str)
                .map(str::to_string);
            out.push(json!({
                "kind": "occupations",
                "title": t("showcase-occupations-title", &[("n", (readable.len() as i64).into())]),
                "detail": t("showcase-occupations-detail", &[]),
                "example_id": example,
            }));
        }
    }

    // Persons whose birth or death date is anything other than a pinned
    // calendar day — the ones another program would show as a blank or as a
    // precision nobody actually claimed.
    if let Some(persons) = obj("persons") {
        let mut uncertain = 0usize;
        let mut preserved = 0usize;
        let mut example: Option<String> = None;
        for (id, p) in persons.iter() {
            if !lens.sees_person(id) {
                continue;
            }
            for key in ["birth", "death"] {
                let Some(ev) = p.get(key) else { continue };
                // English deliberately: this is classifying the date's shape,
                // not showing it, and the classification must not change with
                // the interface language.
                let d = view::render_date_field(ev, "date");
                match d.kind {
                    "range" | "approximate" => {
                        uncertain += 1;
                        example.get_or_insert_with(|| id.clone());
                    }
                    "preserved" => {
                        preserved += 1;
                        example = Some(id.clone());
                    }
                    _ => {}
                }
            }
        }
        if uncertain > 0 {
            out.push(json!({
                "kind": "uncertain",
                "title": t("showcase-uncertain-title", &[("n", (uncertain as i64).into())]),
                "detail": t("showcase-uncertain-detail", &[]),
                "example_id": example,
            }));
        }
        if preserved > 0 {
            out.push(json!({
                "kind": "preserved",
                "title": t("showcase-preserved-title", &[("n", (preserved as i64).into())]),
                "detail": t("showcase-preserved-detail", &[]),
                "example_id": Value::Null,
            }));
        }
    }

    if let Some(sources) = obj("sources") {
        if !sources.is_empty() {
            let primary = sources
                .values()
                .filter(|s| s.get("reliability").and_then(Value::as_str) == Some("primary"))
                .count();
            out.push(json!({
                "kind": "sources",
                "title": t("showcase-sources-title", &[("n", (sources.len() as i64).into())]),
                "detail": t("showcase-sources-detail", &[("primary", (primary as i64).into())]),
                "example_id": Value::Null,
            }));
        }
    }

    if let Some(places) = obj("places") {
        let with_history = places
            .values()
            .filter(|p| {
                p.get("country_history")
                    .and_then(Value::as_array)
                    .is_some_and(|a| !a.is_empty())
            })
            .count();
        if with_history > 0 {
            out.push(json!({
                "kind": "places",
                "title": t("showcase-places-title", &[("n", (with_history as i64).into())]),
                "detail": t("showcase-places-detail", &[]),
                "example_id": Value::Null,
            }));
        }
    }

    out
}

/// Query parameters for `/tree`.
#[derive(serde::Deserialize)]
pub struct TreeQuery {
    /// Centre the view on this person. Defaults to whoever's surroundings
    /// make the fullest first screen at the requested depth.
    #[serde(default)]
    root: Option<String>,
    /// Generations shown in each direction.
    #[serde(default)]
    depth: Option<usize>,
    /// Draw every person in the bundle instead of a focused subtree.
    #[serde(default)]
    all: Option<String>,
    /// Person whose record is shown in the side panel. Defaults to the root.
    /// Kept distinct from `root` so opening a record in the panel does not move
    /// the tree — re-centring is an explicit action.
    #[serde(default)]
    sel: Option<String>,
    /// How wide a row of cards may be, in CSS pixels.
    ///
    /// The tree is laid out in Rust and shipped as absolute coordinates, so
    /// the width has to be chosen before the reader's own is knowable. This is
    /// the explicit answer; the `axgf_tw` cookie is the one `tree.js` sets from
    /// the column it actually measured, and a default covers both being absent.
    #[serde(default)]
    w: Option<f64>,
}

/// Name of the cookie `tree.js` stores the measured column width in.
pub const WIDTH_COOKIE: &str = "axgf_tw";

/// The river's motion choice, "cut" or "travel", written by river.js when the
/// reader picks one. A reader who chose "cut" — often because animation is
/// hard on them — must not be handed animation back by the next page, the
/// next reload or a stray click; the server renders the choice back, so it
/// holds before any script runs.
pub const RIVER_MOTION_COOKIE: &str = "axgf_river_motion";

/// The row width to lay this request's tree out to.
///
/// The query string wins over the cookie, because it is the more explicit of
/// the two and is what a link can carry. Anything unparseable or out of range
/// falls back rather than failing: a bad width is a worse-looking tree, not an
/// error page.
fn wrap_width(q: &TreeQuery, headers: &HeaderMap) -> f64 {
    q.w.or_else(|| {
        crate::session::named_cookie(headers, WIDTH_COOKIE)
            .and_then(|v| v.trim().parse::<f64>().ok())
    })
    .filter(|v| v.is_finite() && *v > 0.0)
    .map(|v| v.clamp(crate::tree::MIN_WRAP_W, crate::tree::MAX_WRAP_W))
    .unwrap_or(crate::tree::DEFAULT_WRAP_W)
}

/// Depth shown above and below the root when none is requested.
const DEFAULT_DEPTH: usize = 3;
/// Upper bound on depth. Past this a "focused" view is the whole bundle again.
const MAX_DEPTH: usize = 8;

/// `GET /tree` — a focused subtree by default, the whole bundle with `?all=1`.
///
/// The full view is laid out correctly but is not usable on a real file: the
/// operator's bundle puts 161 people in its widest generation, which is a
/// canvas over 23,000px wide. Nobody scrolls that far to find an ancestor, so
/// the default is a few dozen people around one person, and every card
/// re-roots the view.
pub async fn tree(
    State(state): State<Shared>,
    headers: HeaderMap,
    // The whole URI, query string included, so that changing the language
    // returns the reader to the person they were looking at rather than to a
    // bare /tree. `Chrome::resolve` rejects anything that is not a same-site
    // path, so this is not a way to smuggle a redirect in.
    axum::extract::OriginalUri(uri): axum::extract::OriginalUri,
    Query(q): Query<TreeQuery>,
) -> Response {
    let viewer = auth::viewer(&state, &headers);
    let chrome = render::Chrome::resolve(&viewer, &headers, &uri.to_string());
    // The page's one select offers the depths and, last, the whole family as
    // depth 0 (the minimalism pass); `all=1` is the older spelling of it.
    let show_all =
        q.all.as_deref().is_some_and(|v| v != "0" && !v.is_empty()) || q.depth == Some(0);
    let depth = q
        .depth
        .filter(|d| *d > 0)
        .unwrap_or(DEFAULT_DEPTH)
        .min(MAX_DEPTH);
    let row_w = wrap_width(&q, &headers);

    let started = std::time::Instant::now();
    let (layout, focus, roster, panel, selected, hidden) =
        state.read_as(viewer.ceiling(), |flat, lens| {
            // The lens arrives resolved and memoised per bundle version, under the
            // same read lock as the bundle — so it can never describe a different
            // version than the one being read.
            // The root picker lists only people this reader can actually open;
            // an entry that leads to "Private" is not a destination.
            let roster = person_roster(flat, lens);
            let hidden = flat
                .get("persons")
                .and_then(Value::as_object)
                .map(|p| p.len().saturating_sub(lens.count(p.len())))
                .unwrap_or(0);

            let opts = crate::tree::LayoutOpts::default()
                .with_width(row_w)
                .seeing(lens.set());

            if show_all {
                let mut l = crate::tree::layout_with(flat, opts);
                crate::tree::localise(&mut l, chrome.lang);
                crate::tree::redact_in(&mut l, lens.set(), chrome.lang);
                return (l, None, roster, None, None, hidden);
            }

            // Choosing a root evaluates every candidate's whole subtree, so it
            // is the most expensive thing on this path and must happen exactly
            // once. When the reader may read nobody — every signed-out visitor
            // to a bundle that marks its family `members`, which is not a rare
            // case — searching readable candidates first would scan all 866
            // people to find none and then scan them all again unrestricted.
            // Ask only the question that can be answered.
            let candidates = lens.set();
            let restricted_search = candidates.is_some_and(|set| !set.is_empty());
            let root = q
                .root
                .clone()
                .filter(|id| flat.get("persons").and_then(|p| p.get(id)).is_some())
                .or_else(|| {
                    if restricted_search {
                        crate::tree::best_root_among(flat, depth, candidates)
                    } else {
                        // Nobody readable: the tree still draws its shape, all
                        // of it redacted, centred where an admin would land.
                        crate::tree::best_root(flat, depth)
                    }
                });

            match root {
                Some(root) => {
                    let sub = crate::tree::select_subtree(flat, &root, depth, depth);
                    let mut l = crate::tree::layout_focused_with(flat, &sub, opts);
                    crate::tree::localise(&mut l, chrome.lang);
                    crate::tree::redact_in(&mut l, lens.set(), chrome.lang);
                    let name = if lens.sees_person(&root) {
                        flat.get("persons")
                            .and_then(|p| p.get(&root))
                            .map(view::person_display_name)
                            // `record-unknown-person`, not the English of it:
                            // this is a word the interface says, and it was
                            // reaching a Polish and an Arabic page as
                            // "[Unknown]".
                            .unwrap_or_else(|| {
                                crate::i18n::translate(chrome.lang, "record-unknown-person", None)
                            })
                    } else {
                        crate::i18n::translate(chrome.lang, crate::person::RESTRICTED_KEY, None)
                    };
                    let focus = json!({
                        "root": root,
                        "root_name": name,
                        "ancestors": sub.ancestor_count,
                        "descendants": sub.descendant_count,
                        "spouses": sub.spouse_count,
                    });
                    // The panel opens on the selection, or the root if none was
                    // asked for. Selecting a person never moves the tree.
                    let sel = q
                        .sel
                        .clone()
                        .filter(|id| flat.get("persons").and_then(|p| p.get(id)).is_some())
                        .unwrap_or_else(|| root.clone());
                    // A selection the reader may not read opens no panel at all,
                    // rather than a panel of blanks.
                    let panel = if lens.sees_person(&sel) {
                        crate::person::build_in(flat, &sel, lens, chrome.lang)
                    } else {
                        None
                    };
                    (l, Some(focus), roster, panel, Some(sel), hidden)
                }
                // An empty bundle has nobody to focus on.
                None => {
                    let mut l = crate::tree::layout_with(flat, opts);
                    crate::tree::localise(&mut l, chrome.lang);
                    crate::tree::redact_in(&mut l, lens.set(), chrome.lang);
                    (l, None, roster, None, None, hidden)
                }
            }
        });
    let elapsed = started.elapsed();

    tracing::debug!(
        drawn = layout.person_count,
        total = layout.total_person_count,
        edges = layout.edges.len(),
        ms = elapsed.as_secs_f64() * 1000.0,
        all = show_all,
        "tree laid out"
    );

    // The warning that states the canvas width sits above the full view only,
    // so the focused path must not pay for a second whole-bundle layout to
    // compute a number it never shows.
    let full_width = layout.width;

    // The self-contradiction banner, resolved to the people it is about.
    //
    // Two decisions here. It is named rather than merely announced: the old
    // copy said "run the validator from the admin dashboard", which a
    // signed-out visitor cannot do and which costs an administrator several
    // steps to act on. And it is shown only to readers who could act on it.
    //
    // Hiding it from a visitor is not tidying an inconvenient fact away. The
    // banner exists to prompt a correction, and a visitor can neither locate
    // the error, edit either record, nor necessarily even see the two people
    // — they may be `members`-visible. Against that, the cost is real: on a
    // public page, in the product's own voice, "this tree contradicts itself"
    // reads as the software confessing a fault, and invites a reader to
    // distrust an entire family's record over one bad edge in one union. The
    // tree already says where the *data* is uncertain, through confidence;
    // somebody's data-entry slip is a different kind of thing and does not
    // belong in the same channel. Contributors and admins see all of it.
    // `truncated` rather than a non-empty pair list: a pure parentage loop
    // sets the flag without naming an edge, and that still needs saying.
    let contradictions = if viewer.may_write() && layout.truncated {
        state.read_as(viewer.ceiling(), |flat, lens| {
            let name = |id: &str| -> Value {
                if lens.sees_person(id) {
                    json!({
                        "id": id,
                        "name": flat
                            .get("persons")
                            .and_then(|p| p.get(id))
                            .map(view::person_display_name)
                            .unwrap_or_else(|| {
                                crate::i18n::translate(
                                    chrome.lang,
                                    "record-unknown-person",
                                    None,
                                )
                            }),
                    })
                } else {
                    // Named to a reader who may edit but may not read them
                    // would be a way to enumerate people they cannot see.
                    json!({
                        "name": crate::i18n::translate(
                            chrome.lang, crate::person::RESTRICTED_KEY, None),
                    })
                }
            };
            let shown: Vec<Value> = layout
                .contradictions
                .iter()
                .take(3)
                .map(|(parent, child)| json!({"parent": name(parent), "child": name(child)}))
                .collect();
            json!({
                "pairs": shown,
                "more": layout.contradictions.len().saturating_sub(3),
            })
        })
    } else {
        Value::Null
    };

    render::page_with(
        &chrome,
        "tree.html",
        context! {
            nav => "tree",
            // This page is a diagram six thousand pixels tall, and following a
            // line while panning it is the task. The ambient drift is held off
            // it for that reason — see the note beside the rule in app.css,
            // which carries the frame timings that decided it.
            still => true,
            contradictions,
            layout,
            focus,
            roster,
            depth,
            show_all,
            max_depth => MAX_DEPTH,
            full_width => full_width.round() as i64,
            p => panel,
            selected,
            // How many people this reader is not being shown. Stated rather
            // than hidden: a tree with silent gaps looks like a broken import.
            hidden,
            signed_in => viewer.signed_in(),
            // The record sections inside the panel lay themselves out for a
            // clamped column rather than a page.
            compact => true,
            max_upload_mb => crate::documents::MAX_UPLOAD / (1024 * 1024),
                document_types => crate::documents::document_type_options(chrome.lang),
        },
    )
}

/// `GET /tree/panel/:id` — the side-panel fragment for one person.
///
/// Returns just the panel markup, not a whole page, so a card click can swap it
/// in without reloading the tree. It renders the same `_panel.html` (and thus
/// the same record sections) the initial server-rendered panel uses.
pub async fn tree_panel(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(id): Path<String>,
    Query(q): Query<PanelQuery>,
) -> Response {
    let viewer = auth::viewer(&state, &headers);
    let chrome = render::Chrome::resolve(&viewer, &headers, &format!("/tree?sel={id}"));
    // The panel fetch is a read path like any other, and it is the one most
    // easily forgotten: it returns a fragment rather than a page, so a
    // template-level check would never have covered it. It resolves its own
    // lens and refuses on its own.
    let outcome = state.read_as(viewer.ceiling(), |flat, lens| {
        if flat.get("persons").and_then(|p| p.get(&id)).is_none() {
            return Reading::Absent;
        }
        if !lens.sees_person(&id) {
            return Reading::Restricted;
        }
        match crate::person::build_in(flat, &id, lens, chrome.lang) {
            Some(p) => Reading::Ok(Box::new(p)),
            None => Reading::Absent,
        }
    });
    match outcome {
        Reading::Ok(p) => {
            // The river asks for its tabs; the grid does not, and gets the
            // panel exactly as before: every section.
            let tab_ctx = if q.river.is_some() {
                let n =
                    q.n.filter(|n| crate::river::RANGES.contains(n))
                        .unwrap_or(crate::river::DEFAULT_RANGE);
                river_tab_context(&state, &viewer, chrome.lang, &p, n, q.tab.as_deref())
            } else {
                context! {}
            };
            render::page_with(
                &chrome,
                "_panel.html",
                context! {
                    p,
                    compact => true,
                    max_upload_mb => crate::documents::MAX_UPLOAD / (1024 * 1024),
                    document_types => crate::documents::document_type_options(chrome.lang),
                    ..tab_ctx
                },
            )
        }
        Reading::Restricted => restricted_page(&chrome, viewer.signed_in()),
        Reading::Absent => render::error_page_in(
            &chrome,
            StatusCode::NOT_FOUND,
            "error-no-such-person-title",
            "error-no-such-person-detail",
        ),
    }
}

/// A person who is not there: merged into someone, when the journal says so
/// and the reader may see that someone (B7); otherwise simply not found. The
/// survivor is named only through the reader's lens, so an old address never
/// tells a visitor about a record they may not read.
fn merged_or_absent(
    state: &Shared,
    chrome: &render::Chrome,
    viewer: &crate::access::Viewer,
    id: &str,
) -> Response {
    let merged = state.journal().merged_into("person", id).and_then(|e| {
        let into = e.merged_into.clone()?;
        let name = state.read_as(viewer.ceiling(), |flat, lens| {
            (lens.sees_person(&into))
                .then(|| {
                    flat.pointer(&format!("/persons/{into}"))
                        .map(crate::view::person_display_name)
                })
                .flatten()
        })?;
        Some((into, name, e.at))
    });
    let Some((into, name, at)) = merged else {
        return render::error_page_in(
            chrome,
            StatusCode::NOT_FOUND,
            "error-no-such-person-title",
            "error-no-such-person-detail",
        );
    };
    let date = day_month(chrome, &at);
    let detail = chrome.t_args(
        "error-merged-detail",
        &[("name", name.into()), ("date", date.into())],
    );
    let mut resp = render::page_with(
        chrome,
        "merged.html",
        context! {
            title => chrome.t("error-merged-title"),
            detail,
            into,
            nav => "",
        },
    );
    *resp.status_mut() = StatusCode::NOT_FOUND;
    resp
}

/// "3 août 2026", from an RFC 3339 timestamp, in the reader's language: the
/// record pages' own full-date pattern, which spells the month as the
/// language needs it there.
fn day_month(chrome: &render::Chrome, at: &str) -> String {
    let num =
        |r: std::ops::Range<usize>| at.get(r).and_then(|s| s.parse::<i64>().ok()).unwrap_or(1);
    chrome.t_args(
        "date-day-month-year",
        &[
            ("day", num(8..10).into()),
            ("month", num(5..7).into()),
            ("year", num(0..4).into()),
        ],
    )
}

/// The three answers a read of one entity can have.
///
/// "Absent" and "restricted" are kept apart deliberately. Collapsing them into
/// a 404 is the reflex, but it buys nothing here: the tree already shows that a
/// hidden person exists, so a 404 would be a lie that protects nothing while
/// telling a legitimate reader — a family member who is simply signed out —
/// that the record they were sent is missing rather than closed to them.
enum Reading {
    Ok(Box<crate::person::PersonView>),
    Restricted,
    Absent,
}

/// The page shown for a record this reader may not read.
fn restricted_page(chrome: &render::Chrome, signed_in: bool) -> Response {
    // Two different sentences, because two different readers: one can do
    // something about it by signing in, the other needs an administrator.
    let detail = if signed_in {
        "access-restricted-signed-in"
    } else {
        "access-restricted-anonymous"
    };
    render::error_page_in(
        chrome,
        StatusCode::FORBIDDEN,
        "access-restricted-title",
        detail,
    )
}

/// Every readable person as `{id, name}`, sorted by name, for the root picker.
///
/// Hidden people are left out entirely rather than redacted, which is the one
/// place this application omits rather than redacts — and for a reason that
/// does not apply anywhere else. The roster is a *destination list*: every
/// entry is somewhere the reader can go. A row reading "Private" leads
/// nowhere, and a searchable list of them would also be the one surface where
/// existence turns into an enumerable index.
fn person_roster(flat: &Value, lens: &crate::access::Lens) -> Vec<Value> {
    let Some(persons) = flat.get("persons").and_then(Value::as_object) else {
        return Vec::new();
    };
    let mut out: Vec<(String, String)> = persons
        .iter()
        .filter(|(id, _)| lens.sees_person(id))
        .map(|(id, p)| (view::person_display_name(p), id.clone()))
        .collect();
    out.sort();
    out.into_iter()
        .map(|(name, id)| json!({"id": id, "name": name}))
        .collect()
}

/// `?tab=` on the person page.
#[derive(Debug, serde::Deserialize)]
pub struct PersonQuery {
    #[serde(default)]
    tab: Option<String>,
}

/// `GET /person/:id` — everything known about one person.
pub async fn person(
    State(state): State<Shared>,
    Path(id): Path<String>,
    headers: HeaderMap,
    Query(q): Query<PersonQuery>,
) -> Response {
    let viewer = auth::viewer(&state, &headers);
    // The tab travels in the path Chrome remembers, so switching language on
    // the media tab comes back to the media tab.
    let asked = crate::person::RecordTab::from_query(q.tab.as_deref());
    let here = match asked {
        crate::person::RecordTab::Fiche => format!("/person/{id}"),
        other => format!("/person/{id}?tab={}", other.slug()),
    };
    let chrome = render::Chrome::resolve(&viewer, &headers, &here);
    let outcome = state.read_as(viewer.ceiling(), |flat, lens| {
        if flat.get("persons").and_then(|p| p.get(&id)).is_none() {
            return Reading::Absent;
        }
        if !lens.sees_person(&id) {
            return Reading::Restricted;
        }
        match crate::person::build_in(flat, &id, lens, chrome.lang) {
            Some(p) => Reading::Ok(Box::new(p)),
            None => Reading::Absent,
        }
    });

    match outcome {
        Reading::Ok(p) => {
            let profile_groups = state.read_as(viewer.ceiling(), |flat, lens| {
                crate::person::profile_groups_filled(flat, &id, lens, chrome.lang)
            });
            // The record page's Profil tab draws the filled groups and the
            // radars that hold three axes or more; with neither, it is not a
            // tab.
            let charts = p.radars.iter().any(|c| c.present >= 3);
            let tabs = crate::person::tabs_for(&p, !profile_groups.is_empty() || charts);
            let tab = crate::person::current_tab(q.tab.as_deref(), &tabs);
            // The places this person's record actually locates. Usually
            // none: on a converted bundle one place in 123 carries a position,
            // so the map appears when there is something to put on it and is
            // absent otherwise rather than drawing an empty world.
            let place_points: Vec<Value> = p
                .places
                .iter()
                .filter_map(|pu| {
                    let (lat, lon) = (pu.place.lat?, pu.place.lon?);
                    Some(json!({
                        "name": pu.place.name,
                        "lat": lat,
                        "lon": lon,
                        "uses": pu.uses,
                        "href": pu.place.known.then(|| format!("/admin/place/{}/edit", pu.place.id)),
                    }))
                })
                .collect();

            render::page_with(
                &chrome,
                "person.html",
                context! {
                    nav => "tree",
                    p,
                    // Read mode: every filled section and every filled
                    // profile group, under the tabs that hold them, and no
                    // edit history — that is an editing concern.
                    tabs,
                    tab => tab.slug(),
                    // Modifier opens the editor on the tab being read.
                    edit_tab => super::person_editor::fragment_for(tab.slug()),
                    profile_groups,
                    full_record => true,
                    // The standalone page has the width for the explanatory
                    // prose and the comparison tables; the panel does not.
                    compact => false,
                    max_upload_mb => crate::documents::MAX_UPLOAD / (1024 * 1024),
                    document_types => crate::documents::document_type_options(chrome.lang),
                    map => state.map().cloned(),
                    // Serialised here rather than by a template filter: the
                    // attribute holds one JSON document, and building it in
                    // Rust is what makes that a fact rather than a hope.
                    place_points_json => serde_json::to_string(&place_points)
                        .unwrap_or_else(|_| "[]".to_string()),
                    place_points,
                    // Notes and contributions about this person, for the
                    // family's members (docs/adr/0014), under Identity.
                    person_notes => person_notes(&state, &viewer, chrome.lang, &id),
                    signed_in => viewer.signed_in(),
                    may_write => viewer.may_write(),
                },
            )
        }
        Reading::Restricted => restricted_page(&chrome, viewer.signed_in()),
        Reading::Absent => merged_or_absent(&state, &chrome, &viewer, &id),
    }
}

/// The notes about one person this reader may read, newest first.
fn person_notes(
    state: &Shared,
    viewer: &crate::access::Viewer,
    lang: &str,
    id: &str,
) -> Vec<crate::notes::NoteView> {
    if !viewer.signed_in() {
        return Vec::new();
    }
    state.read_as(viewer.ceiling(), |flat, lens| {
        crate::notes::notes_about(flat, lens, true, id)
            .into_iter()
            .map(|(nid, d)| crate::notes::view_of(flat, nid, d, lang))
            .collect()
    })
}

/// A document's metadata and the ZIP path its payload is cached under.
struct StoredDocument {
    mime: String,
    filename: String,
    sha256: String,
    /// The attachment key; the bytes live in the disk cache under it.
    path: String,
}

/// Resolve a document id to its metadata and the key its payload is stored at.
///
/// `None` covers both "no such document" and "the document exists but this
/// bundle does not carry the file" — a `referenced` document names something
/// held elsewhere, and there is nothing here to serve either way.
fn stored_document(
    state: &Shared,
    viewer: &crate::access::Viewer,
    id: &str,
) -> Option<StoredDocument> {
    let doc = state.read_as(viewer.ceiling(), |flat, lens| {
        // A document is reached through the person who attaches it, so that is
        // what governs its bytes. Checked here rather than in each of the three
        // handlers, because /raw, /view and /thumb are three doors into the
        // same file and a check on two of them is a check on none.
        if !crate::access::may_read_document(flat, lens, viewer.signed_in(), id) {
            return None;
        }
        let d = flat.get("documents")?.get(id)?;
        // A portrait an AI reworked is the family's members' alone
        // (docs/adr/0009): its bytes are refused to a reader who is not
        // signed in, whatever the person's own visibility.
        if !viewer.signed_in() && crate::ai_portrait::is_ai_portrait(d) {
            return None;
        }
        // A note and its pictures are the family's members' (docs/adr/0014):
        // read by one who may read the note, under its own rule.
        if crate::notes::is_note_part(d)
            && !crate::notes::part_readable(flat, lens, viewer.signed_in(), d)
        {
            return None;
        }
        let path = d.get("file")?.get("path")?.as_str()?.to_string();
        Some(StoredDocument {
            mime: d
                .get("mime_type")
                .and_then(Value::as_str)
                .unwrap_or("application/octet-stream")
                .to_string(),
            filename: d
                .get("filename")
                .and_then(Value::as_str)
                .unwrap_or("document")
                .to_string(),
            sha256: d
                .get("file")
                .and_then(|f| f.get("sha256"))
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string(),
            path,
        })
    })?;
    // Only serve when the payload is actually present in the cache.
    state.payloads().path_of(&doc.path)?;
    Some(doc)
}

/// `GET /document/:id/raw` — the stored bytes.
///
/// Served with `X-Content-Type-Options: nosniff` always, and as an attachment
/// for everything except the raster image formats a browser draws as pixels.
/// The exception matters: an SVG or an HTML file rendered inline from this
/// origin would run its own script against the viewer's admin session, so
/// only formats that cannot carry script are shown in the page.
pub async fn document_raw(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Response {
    let viewer = auth::viewer(&state, &headers);
    let chrome = render::Chrome::resolve(&viewer, &headers, "/");
    let Some(doc) = stored_document(&state, &viewer, &id) else {
        return render::error_page_in(
            &chrome,
            StatusCode::NOT_FOUND,
            "error-no-such-file-title",
            "error-no-such-file-detail",
        );
    };
    let Some(path) = state.payloads().path_of(&doc.path) else {
        return render::error_page_in(
            &chrome,
            StatusCode::NOT_FOUND,
            "error-payload-missing-title",
            "error-payload-missing-detail",
        );
    };

    let disposition = if crate::documents::serve_inline(&doc.mime) {
        format!("inline; filename=\"{}\"", sanitize_header(&doc.filename))
    } else {
        format!(
            "attachment; filename=\"{}\"",
            sanitize_header(&doc.filename)
        )
    };

    // Stream the bytes from disk rather than reading them into memory. The
    // download is byte-identical to what is in the bundle: EXIF orientation is
    // corrected only for display (see `document_view`), never for the original.
    let Ok(file) = tokio::fs::File::open(&path).await else {
        return render::error_page_in(
            &chrome,
            StatusCode::NOT_FOUND,
            "error-payload-missing-title",
            "error-payload-unopenable-detail",
        );
    };
    let len = file.metadata().await.map(|m| m.len()).ok();
    let stream = tokio_util::io::ReaderStream::new(file);
    let body = axum::body::Body::from_stream(stream);

    let mut headers = header::HeaderMap::new();
    set_header(&mut headers, header::CONTENT_TYPE, &doc.mime);
    set_header(&mut headers, header::CONTENT_DISPOSITION, &disposition);
    set_header(
        &mut headers,
        header::HeaderName::from_static("x-content-type-options"),
        "nosniff",
    );
    // The payload is immutable: a different file means a different document id,
    // because uploads never overwrite in place.
    set_header(&mut headers, header::CACHE_CONTROL, "private, max-age=3600");
    if let Some(len) = len {
        set_header(&mut headers, header::CONTENT_LENGTH, &len.to_string());
    }
    (headers, body).into_response()
}

/// Insert a header, silently skipping a value that cannot be a header value
/// (e.g. a filename with bytes that are not valid in a header). The content
/// type falls back to a safe default rather than being dropped.
fn set_header(headers: &mut header::HeaderMap, name: header::HeaderName, value: &str) {
    match header::HeaderValue::from_str(value) {
        Ok(v) => {
            headers.insert(name, v);
        }
        Err(_) if name == header::CONTENT_TYPE => {
            headers.insert(
                name,
                header::HeaderValue::from_static("application/octet-stream"),
            );
        }
        Err(_) => {}
    }
}

/// `GET /document/:id/view` — a full-size image, EXIF-orientation corrected,
/// for display in the page. Non-images and images already upright stream their
/// stored bytes unchanged; a rotated image is re-encoded so it shows upright.
///
/// This is deliberately distinct from `/raw`: `/raw` is the byte-identical
/// original a reader downloads, while `/view` is what the gallery opens, where
/// a sideways phone photo must appear the right way up.
pub async fn document_view(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Response {
    let viewer = auth::viewer(&state, &headers);
    let chrome = render::Chrome::resolve(&viewer, &headers, "/");
    let Some(doc) = stored_document(&state, &viewer, &id) else {
        return render::error_page_in(
            &chrome,
            StatusCode::NOT_FOUND,
            "error-no-such-file-title",
            "error-no-such-document-detail",
        );
    };
    let Some(bytes) = state.attachment(&doc.path) else {
        return render::error_page_in(
            &chrome,
            StatusCode::NOT_FOUND,
            "error-payload-missing-title",
            "error-payload-missing-detail",
        );
    };

    // Only raster images are corrected; anything else is served as its stored
    // bytes inline where safe, or as an attachment otherwise.
    if crate::documents::serve_inline(&doc.mime) {
        if let Some(png) = crate::documents::oriented_image(&bytes) {
            return (
                [
                    (header::CONTENT_TYPE, "image/png".to_string()),
                    (
                        header::HeaderName::from_static("x-content-type-options"),
                        "nosniff".to_string(),
                    ),
                    (header::CACHE_CONTROL, "private, max-age=3600".to_string()),
                ],
                png,
            )
                .into_response();
        }
    }
    // Fallback: hand off to the byte-identical path.
    document_raw(State(state), headers, Path(id)).await
}

/// `GET /document/:id/thumb` — a downscaled PNG, or 404 for a non-image.
pub async fn document_thumb(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Response {
    let viewer = auth::viewer(&state, &headers);
    let chrome = render::Chrome::resolve(&viewer, &headers, "/");
    let Some(doc) = stored_document(&state, &viewer, &id) else {
        return render::error_page_in(
            &chrome,
            StatusCode::NOT_FOUND,
            "error-no-such-file-title",
            "error-no-such-document-detail",
        );
    };

    let png = state.thumbs().get_or_insert(&id, &doc.sha256, || {
        state
            .attachment(&doc.path)
            .and_then(|bytes| crate::documents::thumbnail(&bytes))
    });

    let Some(png) = png else {
        return render::error_page_in(
            &chrome,
            StatusCode::NOT_FOUND,
            "error-not-an-image-title",
            "error-not-an-image-detail",
        );
    };

    (
        [
            (header::CONTENT_TYPE, "image/png".to_string()),
            (
                header::HeaderName::from_static("x-content-type-options"),
                "nosniff".to_string(),
            ),
            (header::CACHE_CONTROL, "private, max-age=3600".to_string()),
        ],
        png,
    )
        .into_response()
}

/// Strip what would let a filename break out of the `Content-Disposition`
/// quoting or inject a second header.
fn sanitize_header(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            '"' | '\\' | '\r' | '\n' | '/' => '_',
            c if c.is_control() => '_',
            c => c,
        })
        .collect()
}

/// `GET /static/tree.js`
pub async fn tree_js() -> Response {
    (
        [
            (header::CONTENT_TYPE, "text/javascript; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=3600"),
        ],
        render::TREE_JS,
    )
        .into_response()
}

/// `GET /static/map.js` — the place-editor map, enhancement only.
pub async fn map_js() -> Response {
    (
        [
            (header::CONTENT_TYPE, "text/javascript; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=3600"),
        ],
        render::MAP_JS,
    )
        .into_response()
}

/// `GET /static/profile.js` — the profile editor's "add another entry".
pub async fn profile_js() -> Response {
    (
        [
            (header::CONTENT_TYPE, "text/javascript; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=3600"),
        ],
        render::PROFILE_JS,
    )
        .into_response()
}

/// `GET /static/editor.js` — the record's editor's tabs and change marks.
pub async fn editor_js() -> Response {
    (
        [
            (header::CONTENT_TYPE, "text/javascript; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=3600"),
        ],
        render::EDITOR_JS,
    )
        .into_response()
}

/// `GET /static/notes.js` — notes and contributions, read and written
/// (docs/adr/0014).
pub async fn notes_js() -> Response {
    (
        [
            (header::CONTENT_TYPE, "text/javascript; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=3600"),
        ],
        render::NOTES_JS,
    )
        .into_response()
}

/// `GET /static/ui.js` — the delegated handlers every page carries.
pub async fn ui_js() -> Response {
    (
        [
            (header::CONTENT_TYPE, "text/javascript; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=3600"),
        ],
        render::UI_JS,
    )
        .into_response()
}

/// `GET /static/avatar.js` — the picker's focal-point control.
pub async fn avatar_js() -> Response {
    (
        [
            (header::CONTENT_TYPE, "text/javascript; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=3600"),
        ],
        render::AVATAR_JS,
    )
        .into_response()
}

/// `GET /static/vendor/leaflet.js` — the library itself, served by this
/// binary rather than by a CDN, so a reader fetches the page's code from the
/// same place they fetched the page.
pub async fn leaflet_js() -> Response {
    (
        [
            (header::CONTENT_TYPE, "text/javascript; charset=utf-8"),
            // Vendored at a pinned version, so it is immutable for as long as
            // this binary is the one answering.
            (header::CACHE_CONTROL, "public, max-age=604800, immutable"),
        ],
        render::LEAFLET_JS,
    )
        .into_response()
}

/// `GET /static/vendor/leaflet.css`
pub async fn leaflet_css() -> Response {
    (
        [
            (header::CONTENT_TYPE, "text/css; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=604800, immutable"),
        ],
        render::LEAFLET_CSS,
    )
        .into_response()
}

/// `GET /health` — liveness, plus the entity counts this requester may see.
///
/// The only JSON endpoint the application serves, and therefore the one a
/// visibility rule is easiest to forget on: nothing here renders through a
/// template, so nothing here would have been covered by a template-level
/// check.
///
/// `persons` is the count this requester may *read*, not the count the bundle
/// holds. It stays unauthenticated: the endpoint's job is liveness, monitors
/// are not signed in, and the number it now reports is one the tree page
/// already states in words — "17 people are shown without their details" — so
/// withholding it here would protect nothing and break the monitor.
pub async fn health(State(state): State<Shared>, headers: HeaderMap) -> Response {
    let viewer = auth::viewer(&state, &headers);
    let counts = state.counts();
    let persons_visible = state.read_as(viewer.ceiling(), |flat, lens| {
        flat.get("persons")
            .and_then(Value::as_object)
            .map(|p| lens.count(p.len()))
    });
    let mut entities = serde_json::Map::new();
    let mut total = 0usize;
    for (k, n) in &counts {
        // Persons are the one collection with a per-entity ceiling, so they
        // are the one collection whose count depends on who is asking.
        let n = if *k == "persons" {
            persons_visible.unwrap_or(*n)
        } else {
            *n
        };
        total += n;
        entities.insert((*k).to_string(), json!(n));
    }

    // The operational half. `status` is the worst of the checks rather than a
    // constant "ok", and the HTTP status follows it, so a monitor that
    // understands nothing about this application still catches a full disk and
    // a backup timer that stopped firing.
    let report = state.health();
    let mut body = report.to_json();
    if let Some(obj) = body.as_object_mut() {
        obj.insert("total_entities".into(), json!(total));
        obj.insert("entities".into(), Value::Object(entities));
        obj.insert("collections".into(), json!(COLLECTIONS));
    }

    // A monitor polls this every minute for years; a cached 200 from when the
    // disk was fine is the one answer that must never be given.
    (
        report.http_status(),
        [(header::CACHE_CONTROL, "no-store, no-cache, must-revalidate")],
        Json(body),
    )
        .into_response()
}

/// `GET /static/css/tokens.css`
pub async fn tokens_css() -> Response {
    (
        [
            (header::CONTENT_TYPE, "text/css; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=3600"),
        ],
        render::TOKENS_CSS,
    )
        .into_response()
}

/// `GET /static/app.css`
pub async fn css() -> Response {
    (
        [
            (header::CONTENT_TYPE, "text/css; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=3600"),
        ],
        render::APP_CSS,
    )
        .into_response()
}

/// A picture the binary carries (v0.5.0, docs/adr/0004). A day of cache: a
/// picture changes with a release, and a release is announced.
fn picture(kind: &'static str, body: &'static [u8]) -> Response {
    (
        [
            (header::CONTENT_TYPE, kind),
            (header::CACHE_CONTROL, "public, max-age=86400"),
        ],
        body,
    )
        .into_response()
}

/// `GET /static/img/canopee-hero.jpg` — behind the overview page's hero, under
/// Canopée only.
pub async fn canopee_hero_jpg() -> Response {
    picture("image/jpeg", render::CANOPEE_HERO_JPG)
}

/// `GET /static/img/canopee-cta.jpg` — beside the overview page's last band,
/// under Canopée only.
pub async fn canopee_cta_jpg() -> Response {
    picture("image/jpeg", render::CANOPEE_CTA_JPG)
}

/// `GET /static/img/fusion-hero.jpg` and the three others: Fusion's and
/// Galaxy's photographs, under their theme only.
pub async fn fusion_hero_jpg() -> Response {
    picture("image/jpeg", render::FUSION_HERO_JPG)
}
pub async fn fusion_cta_jpg() -> Response {
    picture("image/jpeg", render::FUSION_CTA_JPG)
}
pub async fn galaxy_hero_jpg() -> Response {
    picture("image/jpeg", render::GALAXY_HERO_JPG)
}
pub async fn galaxy_cta_jpg() -> Response {
    picture("image/jpeg", render::GALAXY_CTA_JPG)
}
pub async fn summit_hero_jpg() -> Response {
    picture("image/jpeg", render::SUMMIT_HERO_JPG)
}
pub async fn summit_cta_jpg() -> Response {
    picture("image/jpeg", render::SUMMIT_CTA_JPG)
}
pub async fn clarity_hero_jpg() -> Response {
    picture("image/jpeg", render::CLARITY_HERO_JPG)
}
pub async fn clarity_cta_jpg() -> Response {
    picture("image/jpeg", render::CLARITY_CTA_JPG)
}
pub async fn genealogy_hero_jpg() -> Response {
    picture("image/jpeg", render::GENEALOGY_HERO_JPG)
}
pub async fn genealogy_cta_jpg() -> Response {
    picture("image/jpeg", render::GENEALOGY_CTA_JPG)
}
/// `GET /static/img/genealogy-vine-l.jpg` and its mirror: the vine along
/// each edge of Genealogy's pages.
pub async fn genealogy_vine_l_jpg() -> Response {
    picture("image/jpeg", render::GENEALOGY_VINE_L_JPG)
}
pub async fn genealogy_vine_r_jpg() -> Response {
    picture("image/jpeg", render::GENEALOGY_VINE_R_JPG)
}
/// `GET /static/img/genealogy-tree.jpg` and its depth map: Genealogy's tree,
/// the same picture behind every person.
pub async fn genealogy_tree_jpg() -> Response {
    picture("image/jpeg", render::GENEALOGY_TREE_JPG)
}
pub async fn genealogy_tree_depth_jpg() -> Response {
    picture("image/jpeg", render::GENEALOGY_TREE_DEPTH_JPG)
}
/// `GET /static/img/summit-panorama.jpg` — the mountain under Summit's
/// ascent. The same picture behind every person: nothing in it depends on the
/// archive.
pub async fn summit_panorama_jpg() -> Response {
    picture("image/jpeg", render::SUMMIT_PANORAMA_JPG)
}

/// `GET /static/img/notes-canopee.jpg` and the five others: behind the
/// family's notes, each under its own theme.
pub async fn notes_canopee_jpg() -> Response {
    picture("image/jpeg", render::NOTES_CANOPEE_JPG)
}
pub async fn notes_fusion_jpg() -> Response {
    picture("image/jpeg", render::NOTES_FUSION_JPG)
}
pub async fn notes_galaxy_jpg() -> Response {
    picture("image/jpeg", render::NOTES_GALAXY_JPG)
}
pub async fn notes_summit_jpg() -> Response {
    picture("image/jpeg", render::NOTES_SUMMIT_JPG)
}
pub async fn notes_clarity_jpg() -> Response {
    picture("image/jpeg", render::NOTES_CLARITY_JPG)
}
pub async fn notes_genealogy_jpg() -> Response {
    picture("image/jpeg", render::NOTES_GENEALOGY_JPG)
}

/// `GET /static/img/fusion-lab.jpg` — the room the tree's glass stands in,
/// under Fusion.
pub async fn fusion_lab_jpg() -> Response {
    picture("image/jpeg", render::FUSION_LAB_JPG)
}

/// `GET /static/img/river-ground.jpg` — the river's ground. The same picture
/// behind every person: nothing in it depends on the archive.
pub async fn river_ground_jpg() -> Response {
    picture("image/jpeg", render::RIVER_GROUND_JPG)
}
/// `GET /static/img/river-ground-maps.jpg` — its depth and its water.
pub async fn river_ground_maps_jpg() -> Response {
    picture("image/jpeg", render::RIVER_GROUND_MAPS_JPG)
}

/// `GET /static/img/river-contours.svg` — the contour lines over the river's
/// ground, a mask coloured by the stylesheet.
pub async fn river_contours_svg() -> Response {
    picture("image/svg+xml", render::RIVER_CONTOURS_SVG.as_bytes())
}

/// `GET /static/img/logo-mark.svg` — the logo whole, in its own colours.
pub async fn logo_svg() -> Response {
    picture("image/svg+xml", render::LOGO_SVG.as_bytes())
}

/// `GET /favicon.svg` — the tab's icon, wherever SVG icons are read.
pub async fn favicon_svg() -> Response {
    picture("image/svg+xml", render::FAVICON_SVG.as_bytes())
}

/// `GET /favicon.ico` — 16, 32 and 48 pixels, for everything else, and for
/// the clients that ask for it at the root without reading the page.
pub async fn favicon_ico() -> Response {
    picture("image/x-icon", render::FAVICON_ICO)
}

/// `GET /apple-touch-icon.png` — 180 pixels, where iOS looks for it even when
/// the page does not say.
pub async fn apple_touch_icon_png() -> Response {
    picture("image/png", render::APPLE_TOUCH_ICON_PNG)
}

/// `GET /static/img/icon-192.png` — the manifest's home-screen icon.
pub async fn icon_192_png() -> Response {
    picture("image/png", render::ICON_192_PNG)
}

/// `GET /static/img/icon-512.png` — the manifest's large icon.
pub async fn icon_512_png() -> Response {
    picture("image/png", render::ICON_512_PNG)
}

/// `GET /site.webmanifest` — the name and the icons a phone puts on its home
/// screen.
pub async fn web_manifest() -> Response {
    picture("application/manifest+json", render::WEB_MANIFEST.as_bytes())
}

/// Fallback for unmatched paths.
pub async fn not_found(State(state): State<Shared>, headers: HeaderMap) -> Response {
    // Even a 404 is rendered in the reader's language and theme. It is a page
    // like any other, and it is one of the more likely ones to be seen.
    let viewer = auth::viewer(&state, &headers);
    let chrome = render::Chrome::resolve(&viewer, &headers, "/");
    render::error_page_in(
        &chrome,
        StatusCode::NOT_FOUND,
        "error-not-found-title",
        "error-not-found-detail",
    )
}

// ---------------------------------------------------------------------------
// The river
// ---------------------------------------------------------------------------

/// Query parameters for `/tree/panel/:id`. All optional: the grid sends
/// none, the river sends `river=1` and its tab.
#[derive(serde::Deserialize)]
pub struct PanelQuery {
    #[serde(default)]
    river: Option<String>,
    #[serde(default)]
    n: Option<usize>,
    #[serde(default)]
    tab: Option<String>,
}

/// Query parameters for `/` and `/river/data`.
#[derive(serde::Deserialize)]
pub struct RiverQuery {
    /// The person at the fixed point. Defaults to the fullest first screen.
    #[serde(default)]
    p: Option<String>,
    /// Generations each way: 2, 3 or 5.
    #[serde(default)]
    n: Option<usize>,
    /// The record tab in the panel; kept as the reader travels.
    #[serde(default)]
    tab: Option<String>,
}

/// What `_panel.html` needs to know beside the river: that it is beside the
/// river (no grid re-root button), the range to link back with, and the
/// record's tabs — only those that hold something — with the one on screen.
/// The panel's Profil tab draws the filled groups; the radars stay on the
/// record page, where there is the width for them.
fn river_tab_context(
    state: &Shared,
    viewer: &crate::access::Viewer,
    lang: &str,
    p: &crate::person::PersonView,
    n: usize,
    requested: Option<&str>,
) -> minijinja::Value {
    let profile_groups = state.read_as(viewer.ceiling(), |flat, lens| {
        crate::person::profile_groups_filled(flat, &p.id, lens, lang)
    });
    let tabs = crate::person::tabs_for(p, !profile_groups.is_empty());
    let tab = crate::person::current_tab(requested, &tabs);
    context! {
        river_tabs => true,
        river_n => n,
        tabs,
        tab => tab.slug(),
        edit_tab => super::person_editor::fragment_for(tab.slug()),
        profile_groups,
    }
}

/// One river, resolved for one reader.
struct RiverView {
    svg: String,
    payload: Value,
    centre: String,
    name: String,
    n: usize,
    panel: Option<crate::person::PersonView>,
    hidden: usize,
}

/// The words the river draws, in `lang`.
fn river_words(lang: &str) -> crate::river::Words {
    crate::river::Words {
        living_band: crate::i18n::translate(lang, "river-band-living", None),
        circa: crate::i18n::translate(lang, "river-circa", None),
    }
}

/// JSON safe to put inside a `<script type="application/json">`: nothing in
/// it can close the element or open a comment.
fn script_json(v: &impl serde::Serialize) -> String {
    serde_json::to_string(v)
        .unwrap_or_else(|_| "null".into())
        .replace('<', "\\u003c")
        .replace('>', "\\u003e")
        .replace('&', "\\u0026")
}

/// Lay the river out for this reader. `None` for an archive with nobody in it.
fn river_view(
    state: &Shared,
    viewer: &crate::access::Viewer,
    lang: &str,
    q: &RiverQuery,
    with_panel: bool,
) -> Option<RiverView> {
    let n =
        q.n.filter(|n| crate::river::RANGES.contains(n))
            .unwrap_or(crate::river::DEFAULT_RANGE);
    state.read_as(viewer.ceiling(), |flat, lens| {
        let g = crate::river::Graph::build(flat);
        if g.is_empty() {
            return None;
        }
        let centre =
            q.p.clone()
                .filter(|id| g.index_of(id).is_some())
                .or_else(|| g.default_centre(lens))?;
        let river = crate::river::build(
            flat,
            lens,
            &g,
            &centre,
            n,
            crate::river::Shape::default(),
            river_words(lang),
            viewer.signed_in(),
        )?;
        let name = if lens.sees_person(&centre) {
            river
                .shown
                .iter()
                .find(|s| s.id == centre)
                .map(|s| s.names[2].clone())
                .unwrap_or_default()
        } else {
            crate::i18n::translate(lang, crate::person::RESTRICTED_KEY, None)
        };
        let aria = {
            let args =
                fluent::FluentArgs::from_iter([("name", fluent::FluentValue::from(name.clone()))]);
            crate::i18n::translate(lang, "river-aria", Some(&args))
        };
        let panel = (with_panel && lens.sees_person(&centre))
            .then(|| crate::person::build_in(flat, &centre, lens, lang))
            .flatten();
        let hidden = g.len().saturating_sub(lens.count(g.len()));
        Some(RiverView {
            svg: river.svg(&aria),
            payload: json!({"river": river.payload(), "aria": aria}),
            centre,
            name,
            n,
            panel,
            hidden,
        })
    })
}

/// `GET /` — the river, or the overview for an archive with nobody in it.
///
/// Server-rendered and complete without JavaScript: the SVG is the first
/// paint, the range segment is three links, and every person is a link to
/// the same page centred on them. `static/river.js` turns a click into
/// travel when it can.
pub async fn river(
    State(state): State<Shared>,
    headers: HeaderMap,
    axum::extract::OriginalUri(uri): axum::extract::OriginalUri,
    Query(q): Query<RiverQuery>,
) -> Response {
    let viewer = auth::viewer(&state, &headers);
    let chrome = render::Chrome::resolve(&viewer, &headers, &uri.to_string());
    let started = std::time::Instant::now();
    let Some(v) = river_view(&state, &viewer, chrome.lang, &q, true) else {
        return home(State(state), headers, Query(HomeQuery::default())).await;
    };
    tracing::debug!(
        centre = %v.centre,
        n = v.n,
        bytes = v.svg.len(),
        ms = started.elapsed().as_secs_f64() * 1000.0,
        "river laid out"
    );
    let tab_ctx = v
        .panel
        .as_ref()
        .map(|p| river_tab_context(&state, &viewer, chrome.lang, p, v.n, q.tab.as_deref()))
        .unwrap_or_else(|| context! {});
    // The reader's choice between travel and cut, remembered across pages
    // and reloads ([`RIVER_MOTION_COOKIE`]); prefers-reduced-motion still
    // forces a cut in river.js whatever this says.
    let motion_cut =
        crate::session::named_cookie(&headers, RIVER_MOTION_COOKIE).as_deref() == Some("cut");
    render::page_with(
        &chrome,
        "river.html",
        context! {
            nav => "river",
            still => true,
            motion_cut,
            svg => v.svg,
            payload => script_json(&v.payload),
            centre_q => crate::river::url_component(&v.centre),
            centre => v.centre,
            centre_name => v.name,
            n => v.n,
            ranges => crate::river::RANGES,
            p => v.panel,
            hidden => v.hidden,
            signed_in => viewer.signed_in(),
            compact => true,
            max_upload_mb => crate::documents::MAX_UPLOAD / (1024 * 1024),
            document_types => crate::documents::document_type_options(chrome.lang),
            ..tab_ctx
        },
    )
}

/// `GET /river/data` — one layout as JSON, for travel.
///
/// The client tweens between the layout it has and this one, then puts the
/// server's own SVG in place, so a still frame is always the server's.
pub async fn river_data(
    State(state): State<Shared>,
    headers: HeaderMap,
    Query(q): Query<RiverQuery>,
) -> Response {
    let viewer = auth::viewer(&state, &headers);
    let chrome = render::Chrome::resolve(&viewer, &headers, "/");
    match river_view(&state, &viewer, chrome.lang, &q, false) {
        Some(v) => {
            let mut body = v.payload;
            body["svg"] = Value::String(v.svg);
            body["name"] = Value::String(v.name);
            body["centre"] = Value::String(v.centre);
            body["n"] = json!(v.n);
            ([(header::CACHE_CONTROL, "no-store")], Json(body)).into_response()
        }
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

/// `?group=` on the data-quality screen.
#[derive(Debug, serde::Deserialize)]
pub struct QualityQuery {
    #[serde(default)]
    group: Option<String>,
    /// `dismissed` to see what somebody marked as normal; anything else, the
    /// findings still open.
    #[serde(default)]
    show: Option<String>,
}

/// The findings this reader may be shown ([`crate::quality::for_reader`]),
/// or `None` for a signed-out visitor: the screen is a working list for the
/// family, not a public page.
fn quality_findings(
    state: &Shared,
    viewer: &crate::access::Viewer,
) -> Option<Vec<crate::quality::Finding>> {
    if !viewer.signed_in() {
        return None;
    }
    Some(state.read_as(viewer.ceiling(), |flat, lens| {
        crate::quality::for_reader(flat, lens)
    }))
}

/// `GET /quality` — things worth a look in the archive, grouped, each with
/// one sentence (what is wrong, then what to do) and a link to the record's
/// own editor. Nothing is changed from here.
pub async fn quality(
    State(state): State<Shared>,
    headers: HeaderMap,
    axum::extract::OriginalUri(uri): axum::extract::OriginalUri,
    Query(q): Query<QualityQuery>,
) -> Response {
    let viewer = auth::viewer(&state, &headers);
    let chrome = render::Chrome::resolve(&viewer, &headers, &uri.to_string());
    let Some(all) = quality_findings(&state, &viewer) else {
        return Redirect::to("/admin/login").into_response();
    };
    let may_write = viewer.may_write();
    let show_dismissed = q.show.as_deref() == Some("dismissed");
    let dismissed_total = all.iter().filter(|f| f.dismissed.is_some()).count();
    let all: Vec<_> = all
        .into_iter()
        .filter(|f| f.dismissed.is_some() == show_dismissed)
        .collect();
    let total = all.len();
    let groups: Vec<Value> = crate::quality::GROUPS
        .iter()
        .map(|g| json!({"slug": g, "n": all.iter().filter(|f| f.rule.group() == *g).count()}))
        .collect();
    let chosen = q
        .group
        .filter(|g| crate::quality::GROUPS.contains(&g.as_str()));
    let back = match (&chosen, show_dismissed) {
        (Some(g), true) => format!("/quality?show=dismissed&group={g}"),
        (Some(g), false) => format!("/quality?group={g}"),
        (None, true) => "/quality?show=dismissed".into(),
        (None, false) => "/quality".into(),
    };
    let mut rules: Vec<Value> = Vec::new();
    for rule in crate::quality::Rule::ALL {
        if chosen.as_deref().is_some_and(|g| g != rule.group()) {
            continue;
        }
        let items: Vec<Value> = all
            .iter()
            .filter(|f| f.rule == rule)
            .map(|f| {
                json!({
                    "message": crate::quality::message(f, chrome.lang),
                    "link": crate::quality::link(f, may_write),
                    "kind": f.kind,
                    "id": f.id,
                    "fingerprint": f.fingerprint,
                    "version": f.version,
                    "dismissed": f.dismissed,
                })
            })
            .collect();
        if !items.is_empty() {
            rules.push(json!({"slug": rule.slug(), "group": rule.group(), "n": items.len(), "items": items}));
        }
    }
    render::page_with(
        &chrome,
        "quality.html",
        context! {
            nav => "quality",
            total,
            dismissed_total,
            show_dismissed,
            back,
            may_write,
            groups,
            group => chosen,
            rules,
        },
    )
}

/// `GET /quality.csv` — the same list, to work through offline.
pub async fn quality_csv(State(state): State<Shared>, headers: HeaderMap) -> Response {
    let viewer = auth::viewer(&state, &headers);
    let chrome = render::Chrome::resolve(&viewer, &headers, "/quality");
    let Some(all) = quality_findings(&state, &viewer) else {
        return Redirect::to("/admin/login").into_response();
    };
    (
        [
            (header::CONTENT_TYPE, "text/csv; charset=utf-8"),
            (
                header::CONTENT_DISPOSITION,
                "attachment; filename=\"quality.csv\"",
            ),
            (header::CACHE_CONTROL, "no-store"),
        ],
        crate::quality::csv(&all, chrome.lang, viewer.may_write()),
    )
        .into_response()
}

/// `GET /quality.json` — the same list, with each finding's arguments.
pub async fn quality_json(State(state): State<Shared>, headers: HeaderMap) -> Response {
    let viewer = auth::viewer(&state, &headers);
    let chrome = render::Chrome::resolve(&viewer, &headers, "/quality");
    let Some(all) = quality_findings(&state, &viewer) else {
        return Redirect::to("/admin/login").into_response();
    };
    let may_write = viewer.may_write();
    let body: Vec<Value> = all
        .iter()
        .map(|f| {
            json!({
                "group": f.rule.group(),
                "rule": f.rule.slug(),
                "kind": f.kind,
                "id": f.id,
                "link": crate::quality::link(f, may_write),
                "message": crate::quality::message(f, chrome.lang),
                "args": f.args,
                "fingerprint": f.fingerprint,
                "dismissed": f.dismissed,
                "version": f.version,
            })
        })
        .collect();
    (
        [
            (
                header::CONTENT_DISPOSITION,
                "attachment; filename=\"quality.json\"",
            ),
            (header::CACHE_CONTROL, "no-store"),
        ],
        Json(body),
    )
        .into_response()
}

/// `?q=` on the search page and its JSON twin.
#[derive(Debug, serde::Deserialize)]
pub struct SearchQuery {
    #[serde(default)]
    q: String,
}

/// How many people the masthead's suggestions list; the page shows more.
const SEARCH_SUGGESTIONS: usize = 8;
const SEARCH_PAGE: usize = 60;

/// `GET /search?q=` — people by name, through the reader's lens
/// ([`crate::search::people`]). The masthead form lands here with scripting
/// off; with it on, `ui.js` lists suggestions from `/search.json` instead and
/// Enter travels without leaving the page.
pub async fn search(
    State(state): State<Shared>,
    headers: HeaderMap,
    axum::extract::OriginalUri(uri): axum::extract::OriginalUri,
    Query(q): Query<SearchQuery>,
) -> Response {
    let viewer = auth::viewer(&state, &headers);
    let chrome = render::Chrome::resolve(&viewer, &headers, &uri.to_string());
    let hits = state.read_as(viewer.ceiling(), |flat, lens| {
        crate::search::people(flat, lens, &q.q, SEARCH_PAGE)
    });
    render::page_with(
        &chrome,
        "search.html",
        context! {
            nav => "search",
            q => q.q.trim(),
            hits,
        },
    )
}

/// `GET /search.json?q=` — the masthead's suggestions: `{hits, total,
/// summary}`, each hit with its discharge class.
pub async fn search_json(
    State(state): State<Shared>,
    headers: HeaderMap,
    Query(q): Query<SearchQuery>,
) -> Response {
    let viewer = auth::viewer(&state, &headers);
    let (hits, total) = state.read_as(viewer.ceiling(), |flat, lens| {
        crate::search::people_counted(flat, lens, &q.q, SEARCH_SUGGESTIONS)
    });
    // The suggestions' footer (B3): how many matched, and the two keys.
    let chrome = render::Chrome::resolve(&viewer, &headers, "/");
    let summary = chrome.t_args("search-summary", &[("n", (total as i64).into())]);
    (
        [(header::CACHE_CONTROL, "no-store")],
        Json(json!({"hits": hits, "total": total, "summary": summary})),
    )
        .into_response()
}

/// `GET /static/river.js`.
pub async fn river_js() -> Response {
    (
        [
            (header::CONTENT_TYPE, "text/javascript; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=3600"),
        ],
        render::RIVER_JS,
    )
        .into_response()
}

/// `GET /static/river-scene.js`.
pub async fn river_scene_js() -> Response {
    (
        [
            (header::CONTENT_TYPE, "text/javascript; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=3600"),
        ],
        render::RIVER_SCENE_JS,
    )
        .into_response()
}

/// `GET /static/fusion.js` — Fusion's living tree (docs/adr/0007).
pub async fn fusion_js() -> Response {
    (
        [
            (header::CONTENT_TYPE, "text/javascript; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=3600"),
        ],
        render::FUSION_JS,
    )
        .into_response()
}

/// `GET /static/galaxy.js` — Galaxy's solar system (docs/adr/0007).
pub async fn galaxy_js() -> Response {
    (
        [
            (header::CONTENT_TYPE, "text/javascript; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=3600"),
        ],
        render::GALAXY_JS,
    )
        .into_response()
}

/// `GET /static/summit.js` — Summit's ascent (docs/adr/0010).
pub async fn summit_js() -> Response {
    (
        [
            (header::CONTENT_TYPE, "text/javascript; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=3600"),
        ],
        render::SUMMIT_JS,
    )
        .into_response()
}

/// `GET /static/clarity.js` — Clarity's funnel (docs/adr/0013).
pub async fn clarity_js() -> Response {
    (
        [
            (header::CONTENT_TYPE, "text/javascript; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=3600"),
        ],
        render::CLARITY_JS,
    )
        .into_response()
}

/// `GET /static/genealogy.js` — Genealogy's scroll (docs/adr/0016).
pub async fn genealogy_js() -> Response {
    (
        [
            (header::CONTENT_TYPE, "text/javascript; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=3600"),
        ],
        render::GENEALOGY_JS,
    )
        .into_response()
}
