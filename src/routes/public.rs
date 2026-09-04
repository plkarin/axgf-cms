//! Public, read-only pages.

use axum::extract::{Path, Query, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use minijinja::context;
use serde_json::{json, Value};

use crate::routes::Shared;
use crate::state::COLLECTIONS;
use crate::{auth, render, view};

/// `GET /` — what the site does for a family, what this tree holds, entry points.
pub async fn home(State(state): State<Shared>, headers: HeaderMap) -> Response {
    let viewer = auth::viewer(&state, &headers);
    let chrome = render::Chrome::resolve(&viewer, &headers, "/");
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

    render::page_with(
        &chrome,
        "home.html",
        context! {
            nav => "home",
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
        },
    )
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
                "title": t("showcase-uncertain-title", &[("n", (uncertain as i64).into())]),
                "detail": t("showcase-uncertain-detail", &[]),
                "example_id": example,
            }));
        }
        if preserved > 0 {
            out.push(json!({
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

/// The row width to lay this request's tree out to.
///
/// The query string wins over the cookie, because it is the more explicit of
/// the two and is what a link can carry. Anything unparseable or out of range
/// falls back rather than failing: a bad width is a worse-looking tree, not an
/// error page.
/// The remembered row width, for a page with no tree query of its own.
fn wrap_width_from(headers: &HeaderMap) -> f64 {
    crate::session::named_cookie(headers, WIDTH_COOKIE)
        .and_then(|v| v.trim().parse::<f64>().ok())
        .filter(|v| v.is_finite() && *v > 0.0)
        .map(|v| v.clamp(crate::tree::MIN_WRAP_W, crate::tree::MAX_WRAP_W))
        .unwrap_or(crate::tree::DEFAULT_WRAP_W)
}

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
    let show_all = q.all.as_deref().is_some_and(|v| v != "0" && !v.is_empty());
    let depth = q.depth.unwrap_or(DEFAULT_DEPTH).min(MAX_DEPTH);
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
                            .unwrap_or_else(|| "[Unknown]".into())
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
                            .unwrap_or_else(|| "[Unknown]".into()),
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
            history => selected
                .as_deref()
                .filter(|_| viewer.signed_in())
                .map(|id| entity_history(&state, id)),
            // How many people this reader is not being shown. Stated rather
            // than hidden: a tree with silent gaps looks like a broken import.
            hidden,
            signed_in => viewer.signed_in(),
            // The record sections inside the panel lay themselves out for a
            // clamped column rather than a page.
            compact => true,
            max_upload_mb => crate::documents::MAX_UPLOAD / (1024 * 1024),
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
        Reading::Ok(p) => render::page_with(
            &chrome,
            "_panel.html",
            context! {
                p,
                history => viewer.signed_in().then(|| entity_history(&state, &id)),
                compact => true,
                max_upload_mb => crate::documents::MAX_UPLOAD / (1024 * 1024),
            },
        ),
        Reading::Restricted => restricted_page(&chrome, viewer.signed_in()),
        Reading::Absent => render::error_page_in(
            &chrome,
            StatusCode::NOT_FOUND,
            "error-no-such-person-title",
            "error-no-such-person-detail",
        ),
    }
}

/// One person's edit history, newest first, as the record page renders it.
fn entity_history(state: &Shared, id: &str) -> Vec<Value> {
    state
        .journal()
        .for_entity("person", id)
        .into_iter()
        .map(|e| {
            json!({
                "at": e.at,
                "who": e.who,
                "action": e.action,
                "version_num": e.version_num,
                "summary": e.summary(),
                "changes": e.changes,
            })
        })
        .collect()
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

/// How deep the person page's tree tab reaches in each direction.
///
/// Three is grandparents to grandchildren, which is the span a reader means by
/// "where does this person sit". Deeper turns the tab into the tree page,
/// which already exists and is one link away.
const PERSON_TREE_DEPTH: usize = 3;

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
    let tab = crate::person::Tab::from_query(q.tab.as_deref());
    // The tab travels in the path Chrome remembers, so switching language on
    // the media tab comes back to the media tab.
    let here = match tab {
        crate::person::Tab::Record => format!("/person/{id}"),
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
            // The tree is laid out only for the tab that draws it. It is by a
            // wide margin the most expensive thing this page can do, and most
            // visits to a person never ask for it.
            let layout = (tab == crate::person::Tab::Tree).then(|| {
                state.read_as(viewer.ceiling(), |flat, lens| {
                    let subtree = crate::tree::select_subtree(
                        flat,
                        &id,
                        PERSON_TREE_DEPTH,
                        PERSON_TREE_DEPTH,
                    );
                    let opts = crate::tree::LayoutOpts::default()
                        .with_width(wrap_width_from(&headers))
                        .seeing(lens.set());
                    let mut l = crate::tree::layout_focused_with(flat, &subtree, opts);
                    crate::tree::localise(&mut l, chrome.lang);
                    // A person this reader may not see keeps their place and
                    // loses everything else, exactly as on the tree page. The
                    // shape of a family is not a secret; the names in it can
                    // be.
                    crate::tree::redact_in(&mut l, lens.set(), chrome.lang);
                    l
                })
            });
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
                    // The journal names editors, so it is shown to people who
                    // are signed in and to nobody else.
                    history => viewer.signed_in().then(|| entity_history(&state, &id)),
                    // The standalone page has the width for the explanatory
                    // prose and the comparison tables; the panel does not.
                    compact => false,
                    max_upload_mb => crate::documents::MAX_UPLOAD / (1024 * 1024),
                    tab => tab.slug(),
                    tabs => crate::person::TABS
                        .iter()
                        .map(|t| json!({"slug": t.slug(), "key": t.key()}))
                        .collect::<Vec<_>>(),
                    layout,
                    // The canvas partial marks the selected card; on this page
                    // that is always the person whose record it is.
                    selected => id,
                    tree_depth => PERSON_TREE_DEPTH,
                    map => state.map().cloned(),
                    // Serialised here rather than by a template filter: the
                    // attribute holds one JSON document, and building it in
                    // Rust is what makes that a fact rather than a hope.
                    place_points_json => serde_json::to_string(&place_points)
                        .unwrap_or_else(|_| "[]".to_string()),
                    place_points,
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
        if !crate::access::may_read_document(flat, lens.visible(), viewer.signed_in(), id) {
            return None;
        }
        let d = flat.get("documents")?.get(id)?;
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
    Json(json!({
        "status": "ok",
        "total_entities": total,
        "entities": Value::Object(entities),
        "collections": COLLECTIONS,
    }))
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
