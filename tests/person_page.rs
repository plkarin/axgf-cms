//! The identity view, driven through the real router.

mod common;

use axum::http::StatusCode;
use common::*;
use serde_json::json;

/// A bundle exercising every showcase feature at once.
fn showcase_bundle(tag: &str) -> std::path::PathBuf {
    let dir = scratch(tag);
    let path = dir.join("showcase.axgf");
    let flat = json!({
        "manifest": {"axgf": "1.0"},
        "persons": {
            "11111111-1111-4111-8111-111111111111": {
                "id": "11111111-1111-4111-8111-111111111111", "type": "person",
                "axgf_version": "1.0",
                "identity": {
                    "name": {"display": "Jules Meunier", "components": []},
                    "gender": {"value": "M"}, "is_living": false,
                    "names": [{"type": "birth", "display": "Jules Meunier",
                               "components": [], "confidence": 0.9}]},
                "birth": {"date": {"value": "1500", "circa": true, "precision": "year"},
                          "confidence": 0.35},
                "death": {"date": {"precision": "unknown",
                                   "note": "Michaelmas term, 3 Edw. III"},
                          "confidence": 0.2}
            },
            "22222222-2222-4222-8222-222222222222": {
                "id": "22222222-2222-4222-8222-222222222222", "type": "person",
                "axgf_version": "1.0",
                "identity": {"name": {"display": "Jean Boucher", "components": []}}
            }
        },
        "families": {},
        "links": {
            "33333333-3333-4333-8333-333333333333": {
                "id": "33333333-3333-4333-8333-333333333333", "type": "link",
                "axgf_version": "1.0",
                "from": {"entity_type": "person",
                         "entity_id": "22222222-2222-4222-8222-222222222222"},
                "to": {"entity_type": "person",
                       "entity_id": "11111111-1111-4111-8111-111111111111"},
                "label": "godfather", "label_reverse": "godson",
                "category": "spiritual",
                "valid_from": {"date": {"value": "1950", "precision": "year"}},
                "confidence": 0.85,
                "source_id": "44444444-4444-4444-8444-444444444444",
                "note": "per a family letter"}
        },
        "occupations": {
            "55555555-5555-4555-8555-555555555555": {
                "id": "55555555-5555-4555-8555-555555555555", "type": "occupation",
                "axgf_version": "1.0",
                "person_id": "11111111-1111-4111-8111-111111111111",
                "title": "Schoolteacher",
                "employer": {"name": "École Normale"},
                "valid_from": {"date": {"value": "1948", "precision": "year"}},
                "valid_until": {"date": {"value": "1978", "precision": "year"}},
                "confidence": 0.9}
        },
        "sources": {
            "44444444-4444-4444-8444-444444444444": {
                "id": "44444444-4444-4444-8444-444444444444", "type": "source",
                "axgf_version": "1.0", "title": "Family letter, 1954",
                "source_type": "letter", "reliability": "oral", "confidence": 0.6}
        },
        "places": {}, "events": {}, "documents": {}
    });
    let bytes = axgf_cms::state::export_to_bytes(&flat.to_string()).expect("export");
    std::fs::write(&path, bytes).expect("write");
    path
}

const JULES: &str = "11111111-1111-4111-8111-111111111111";

#[tokio::test]
async fn person_page_surfaces_every_gedcom_gap() {
    let src = showcase_bundle("person-src");
    let (app, _p) = app_with_bundle("person", &src);
    // The gaps this test is about are spread across the record and the life
    // tab, and the question — does the page state what the source could not
    // pin down — is about the page rather than about any one tab. So it reads
    // both and asserts against the pair.
    let record = expect_status(
        get(&app, &format!("/person/{JULES}")).await,
        StatusCode::OK,
        "person page",
    )
    .await;
    let life = expect_status(
        get(&app, &format!("/person/{JULES}?tab=life")).await,
        StatusCode::OK,
        "life tab",
    )
    .await;
    let body = format!("{record}{life}");

    // Date honesty: circa and preserved-unparseable both stated as such.
    assert!(body.contains("circa 1500"), "circa must be stated");
    assert!(
        body.contains("Michaelmas term, 3 Edw. III"),
        "unparseable text must survive, not be dropped"
    );
    assert!(
        body.contains(r#"data-kind="preserved""#),
        "preserved text needs its own styling hook"
    );
    assert!(
        body.contains(r#"data-kind="approximate""#),
        "an approximate date must be marked approximate"
    );

    // Confidence is rendered visually, with bands.
    assert!(body.contains("conf-bar"), "confidence needs a visual form");
    assert!(
        body.contains(r#"data-band="low""#),
        "a 0.35 fact must land in the low band"
    );

    // First-class links, in their own section.
    assert!(body.contains("links-section"));
    assert!(
        body.contains("godson"),
        "the link reads from this person's side"
    );
    assert!(body.contains("per a family letter"));

    // Occupations as spans.
    assert!(body.contains("Schoolteacher"));
    assert!(
        body.contains("tl-bar"),
        "an occupation is a bar, not a bullet"
    );
    assert!(body.contains("1948–1978"));

    // Sources graded by reliability.
    assert!(body.contains("src-chip"));
    assert!(body.contains("oral"));
}

#[tokio::test]
async fn a_missing_person_is_a_404_page_not_a_panic() {
    let (app, _p) = app_with_empty_bundle("person-404");
    let body = expect_status(
        get(&app, "/person/99999999-9999-4999-8999-999999999999").await,
        StatusCode::NOT_FOUND,
        "missing person",
    )
    .await;
    assert!(body.contains("No such person"));
}

#[tokio::test]
async fn the_edit_link_appears_only_for_an_admin() {
    let src = showcase_bundle("person-edit-src");
    let (app, _p) = app_with_bundle("person-edit", &src);

    let anon = body_string(get(&app, &format!("/person/{JULES}")).await).await;
    assert!(
        !anon.contains("/admin/person/"),
        "an anonymous visitor must not see an edit link"
    );

    let admin = body_string(get_admin(&app, &format!("/person/{JULES}")).await).await;
    assert!(
        admin.contains(&format!("/admin/person/{JULES}/edit")),
        "an admin should see the edit link"
    );
}

#[tokio::test]
async fn a_referenced_but_absent_person_renders_unlinked() {
    let dir = scratch("ghost-src");
    let path = dir.join("ghost.axgf");
    let flat = json!({
        "manifest": {"axgf": "1.0"},
        "persons": {
            "aaaaaaaa-1111-4111-8111-111111111111": {
                "id": "aaaaaaaa-1111-4111-8111-111111111111", "type": "person",
                "axgf_version": "1.0",
                "identity": {"name": {"display": "Present Person", "components": []}}}
        },
        "families": {
            "bbbbbbbb-1111-4111-8111-111111111111": {
                "id": "bbbbbbbb-1111-4111-8111-111111111111", "type": "family",
                "axgf_version": "1.0",
                "union": {"type": "marriage", "persons": [
                    {"person_id": "aaaaaaaa-1111-4111-8111-111111111111", "role": "spouse"}]},
                "children": [
                    {"person_id": "cccccccc-9999-4999-8999-999999999999", "confidence": 0.5}]}
        },
        "links": {}, "occupations": {}, "sources": {}, "places": {},
        "events": {}, "documents": {}
    });
    let bytes = axgf_cms::state::export_to_bytes(&flat.to_string()).expect("export");
    std::fs::write(&path, bytes).expect("write");

    let (app, _p) = app_with_bundle("ghost", &path);
    let body = expect_status(
        get(&app, "/person/aaaaaaaa-1111-4111-8111-111111111111").await,
        StatusCode::OK,
        "person with a ghost child",
    )
    .await;

    assert!(
        body.contains("[Unknown]"),
        "an absent person is named as unknown"
    );
    assert!(
        !body.contains("/person/cccccccc-9999-4999-8999-999999999999"),
        "an absent person must not be linked into a 404"
    );
}

#[tokio::test]
async fn a_section_with_no_data_is_omitted_rather_than_shown_empty() {
    // The shape of the page is meant to be a readout of what the bundle holds,
    // so an empty section is not rendered at all — no heading, no "none
    // recorded" placeholder taking up a screen.
    let dir = scratch("bare-src");
    let path = dir.join("bare.axgf");
    let flat = json!({
        "manifest": {"axgf": "1.0"},
        "persons": {
            "dddddddd-1111-4111-8111-111111111111": {
                "id": "dddddddd-1111-4111-8111-111111111111", "type": "person",
                "axgf_version": "1.0",
                "identity": {"name": {"display": "Nothing Recorded", "components": []},
                             "gender": {"value": "U"}, "is_living": false}}
        },
        "families": {}, "links": {}, "occupations": {}, "sources": {},
        "places": {}, "events": {}, "documents": {}
    });
    let bytes = axgf_cms::state::export_to_bytes(&flat.to_string()).expect("export");
    std::fs::write(&path, bytes).expect("write");

    let (app, _p) = app_with_bundle("bare", &path);
    let body = expect_status(
        get(&app, "/person/dddddddd-1111-4111-8111-111111111111").await,
        StatusCode::OK,
        "person with nothing recorded",
    )
    .await;

    for absent in [
        r#"id="life""#,
        r#"id="family""#,
        r#"id="relationships""#,
        r#"id="occupations""#,
        r#"id="places""#,
        r#"id="evidence""#,
        r#"id="notes""#,
    ] {
        assert!(!body.contains(absent), "{absent} should not be on the page");
    }
    // The raw block always has something to say, so it is always there.
    assert!(body.contains(r#"id="raw""#));
    assert!(body.contains("Nothing Recorded"));
}

#[tokio::test]
async fn the_page_carries_the_full_record_section_by_section() {
    let dir = scratch("full-src");
    let path = dir.join("full.axgf");
    let jules = "11111111-1111-4111-8111-111111111111";
    let adele = "22222222-2222-4222-8222-222222222222";
    let kid = "66666666-6666-4666-8666-666666666666";
    let flat = json!({
        "manifest": {"axgf": "1.0"},
        "persons": {
            jules: {
                "id": jules, "type": "person", "axgf_version": "1.0",
                "identity": {
                    "name": {"display": "Bronisław Klicki", "components": []},
                    "gender": {"value": "M"}, "is_living": false,
                    "visibility": "public",
                    "names": [{"type": "transliteration", "display": "Бронислав Клицкий",
                               "display_latin": "Bronislav Klitskiy", "culture": "ru",
                               "components": [], "valid_from": "1919"}]},
                "birth": {"date": {"value": "1890-03-02", "precision": "exact"},
                          "place_id": "77777777-7777-4777-8777-777777777777",
                          "confidence": 0.9,
                          "source_id": "44444444-4444-4444-8444-444444444444"},
                "death": {"date": {"value": "1955", "precision": "year"}, "confidence": 0.8},
                "notes": "Emigrated in 1921."
            },
            adele: {"id": adele, "type": "person", "axgf_version": "1.0",
                    "identity": {"name": {"display": "Karolina Boddin", "components": []},
                                 "is_living": false},
                    "birth": {"date": {"value": "1893", "precision": "year"}},
                    "death": {"date": {"value": "1970", "precision": "year"}}},
            kid: {"id": kid, "type": "person", "axgf_version": "1.0",
                  "identity": {"name": {"display": "Michał Klicki", "components": []}}}
        },
        "families": {
            "88888888-8888-4888-8888-888888888888": {
                "id": "88888888-8888-4888-8888-888888888888", "type": "family",
                "axgf_version": "1.0",
                "union": {"type": "marriage", "confidence": 0.95,
                          "persons": [{"person_id": jules, "role": "spouse"},
                                      {"person_id": adele, "role": "spouse"}],
                          "start": {"date": {"value": "1919-06-01", "precision": "exact"},
                                    "place_id": "77777777-7777-4777-8777-777777777777"},
                          "end": {"date": {"value": "1955", "precision": "year"},
                                  "reason": "death_of_spouse"}},
                "children": [{"person_id": kid, "birth_order": 1, "confidence": 0.97}]}
        },
        "events": {
            "99999999-9999-4999-8999-999999999999": {
                "id": "99999999-9999-4999-8999-999999999999", "type": "event",
                "axgf_version": "1.0", "category": "baptism",
                "date": {"value": "1890-03-09", "precision": "exact"},
                "participants": [{"entity_type": "person", "entity_id": jules,
                                  "role": "subject"}],
                "confidence": 0.85}
        },
        "links": {}, "occupations": {},
        "sources": {
            "44444444-4444-4444-8444-444444444444": {
                "id": "44444444-4444-4444-8444-444444444444", "type": "source",
                "axgf_version": "1.0", "title": "Lwów parish register",
                "source_type": "birth_certificate", "reliability": "primary",
                "confidence": 0.95}
        },
        "places": {
            "77777777-7777-4777-8777-777777777777": {
                "id": "77777777-7777-4777-8777-777777777777", "type": "place",
                "axgf_version": "1.0",
                "names": [{"lang": "pl", "value": "Lwów", "is_primary": true}],
                "country_current": "Ukraine",
                "country_history": [{"country": "Austria-Hungary", "until": "1918"},
                                    {"country": "Poland", "from": "1918", "until": "1945"},
                                    {"country": "Ukraine", "from": "1991"}]}
        },
        "documents": {}
    });
    let bytes = axgf_cms::state::export_to_bytes(&flat.to_string()).expect("export");
    std::fs::write(&path, bytes).expect("write");

    let (app, _p) = app_with_bundle("full", &path);
    // "The full record, section by section" is now a claim about the page as a
    // whole rather than about one response: the sections are dealt across four
    // tabs. Both halves are read and the assertions run against the pair, with
    // the life tab kept separately for the ordering check below.
    let record = expect_status(
        get(&app, &format!("/person/{jules}")).await,
        StatusCode::OK,
        "the full record",
    )
    .await;
    let life_body = expect_status(
        get(&app, &format!("/person/{jules}?tab=life")).await,
        StatusCode::OK,
        "the life tab",
    )
    .await;
    let media_body = expect_status(
        get(&app, &format!("/person/{jules}?tab=media")).await,
        StatusCode::OK,
        "the media tab",
    )
    .await;
    let body = format!("{record}{life_body}{media_body}");

    // Identity: both scripts, the period the name was used, visibility.
    assert!(body.contains("Бронислав Клицкий"), "native script");
    assert!(
        body.contains("Bronislav Klitskiy"),
        "transliteration beside it"
    );
    assert!(body.contains("from 1919"), "the period the name was used");
    // Visibility is an editing concern — which readers a record is exposed to
    // — so the masthead states it to the people who can change it and to
    // nobody else. It is still in the Identity section as a labelled chip.
    assert!(
        !body.contains("visibility: public"),
        "a reader who cannot edit is not shown the visibility level"
    );

    // Life events: birth, the baptism a week later, then death, in that order.
    let life = life_body
        .split(r#"id="life""#)
        .nth(1)
        .expect("a life section");
    let birth_at = life.find("Born").expect("birth on the timeline");
    let baptism_at = life.find("Baptism").expect("baptism on the timeline");
    let death_at = life.find("Died").expect("death on the timeline");
    assert!(
        birth_at < baptism_at && baptism_at < death_at,
        "the timeline must be chronological"
    );
    assert!(life.contains("as subject"), "the role in each event");

    // Family: the union's type, how it ended, and the child's birth order.
    assert!(body.contains("ended by the death of a spouse"));
    assert!(body.contains("Karolina Boddin"));
    assert!(body.contains(&format!(r#"href="/person/{kid}""#)));
    assert!(body.contains("birth order"));

    // Places, with the border history that makes the town meaningful.
    assert!(body.contains(r#"id="places""#));
    assert!(body.contains("Austria-Hungary"));

    // Evidence names what rests on it.
    assert!(body.contains("Lwów parish register"));
    assert!(body.contains("Supports"));

    // Notes, and the raw entity.
    assert!(body.contains("Emigrated in 1921."));
    // The raw block is escaped on the way out — it is data being displayed,
    // not markup — so it is matched on the key rather than on the quoting.
    let raw = body
        .split(r#"<pre class="raw-json">"#)
        .nth(1)
        .expect("the raw JSON block");
    assert!(raw.contains("axgf_version"));
    assert!(raw.contains("Bronis"), "the entity itself, not a summary");

    // Sections with nothing in them stay off the page.
    assert!(!body.contains(r#"id="relationships""#));
    assert!(!body.contains(r#"id="occupations""#));
}

/// The masthead states who this person was, from the record.
#[tokio::test]
async fn the_masthead_carries_a_face_a_span_and_a_placing() {
    let src = showcase_bundle("masthead-src");
    let (app, _p) = app_with_bundle("masthead", &src);
    let body = body_string(get_admin(&app, &format!("/person/{JULES}")).await).await;

    // Nobody in the fixture has a photograph, so the initials placeholder is
    // what a reader sees — the common case on a converted bundle.
    assert!(
        body.contains("person-avatar is-initials"),
        "no photograph means initials, never a broken image"
    );
    assert!(
        !body.contains("/thumb\""),
        "and no request for a thumbnail that does not exist"
    );
    assert!(body.contains("person-vitals"), "the span is stated");
}

/// Sections are dealt to tabs, and no section is lost in the dealing.
///
/// The page used to be one scroll of eight sections behind a list of anchor
/// links. Splitting it into four views is only defensible if the union of the
/// views is still the whole record — a tab system that quietly drops a section
/// is a record that quietly drops a fact.
#[tokio::test]
async fn every_section_lands_on_exactly_one_tab() {
    let src = showcase_bundle("tabs-src");
    let (app, _p) = app_with_bundle("tabs", &src);

    // What the whole record holds, taken from the one render that names no
    // tab: the tree side panel. Deriving the expectation rather than writing
    // it down means this test keeps working when a section is added, and
    // still fails when one is dropped.
    let whole =
        sections_in(&body_string(get_admin(&app, &format!("/tree/panel/{JULES}")).await).await);
    assert!(
        whole.len() >= 4,
        "the fixture is too thin to test with: {whole:?}"
    );

    let mut seen: Vec<String> = Vec::new();
    for tab in ["", "?tab=life", "?tab=media"] {
        let body = body_string(get_admin(&app, &format!("/person/{JULES}{tab}")).await).await;
        for id in sections_in(&body) {
            assert!(
                !seen.contains(&id),
                "{id} appears on more than one tab: {seen:?}"
            );
            seen.push(id);
        }
    }

    for id in &whole {
        assert!(
            seen.contains(id),
            "{id} is on no tab at all — it was dropped, not moved: {seen:?}"
        );
    }
}

/// The `id` of every `<section>` in a rendered record, in order.
fn sections_in(html: &str) -> Vec<String> {
    let mut out = Vec::new();
    for part in html.split("<section id=\"").skip(1) {
        if let Some(end) = part.find('"') {
            let id = &part[..end];
            // `raw` is the JSON dump and `history` the journal; neither is a
            // section of the record in the sense this is counting.
            if id != "raw" && id != "history" {
                out.push(id.to_string());
            }
        }
    }
    out
}

/// A tab is a link, so it survives having no scripting and being sent to
/// somebody else.
#[tokio::test]
async fn a_tab_is_a_url_and_an_unknown_one_is_the_record() {
    let src = showcase_bundle("tab-url-src");
    let (app, _p) = app_with_bundle("tab-url", &src);

    let body = body_string(get_admin(&app, &format!("/person/{JULES}")).await).await;
    assert!(
        body.contains(&format!("href=\"/person/{JULES}?tab=life\"")),
        "the tabs are real links: {body}"
    );
    assert!(
        body.matches("aria-current=\"page\"").count() >= 1,
        "the open tab says so to a screen reader"
    );

    // A stale or mistyped tab shows the person, not an error about a query
    // parameter: the thing the reader wanted is right there behind it.
    let resp = get_admin(&app, &format!("/person/{JULES}?tab=nonsense")).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_string(resp).await;
    assert!(
        body.contains("<section id=\"identity\""),
        "an unknown tab falls back to the record"
    );
}

/// The tree tab is rooted on this person, and is not laid out for the others.
#[tokio::test]
async fn the_tree_tab_is_rooted_on_this_person() {
    let src = showcase_bundle("tab-tree-src");
    let (app, _p) = app_with_bundle("tab-tree", &src);

    let body = body_string(get_admin(&app, &format!("/person/{JULES}?tab=tree")).await).await;
    assert!(
        body.contains("tree-canvas"),
        "the tree tab draws the canvas: {body}"
    );
    assert!(
        body.contains(&format!("data-id=\"{JULES}\"")),
        "and this person is on it"
    );
    assert!(
        !body.contains("<section id=\"identity\""),
        "the record sections are not also drawn underneath"
    );

    // The other tabs pay nothing for a tree they do not draw.
    let record = body_string(get_admin(&app, &format!("/person/{JULES}")).await).await;
    assert!(
        !record.contains("tree-canvas"),
        "no tree is laid out for the record tab"
    );
}

/// The side panel is not a tabbed page and still shows everything.
///
/// `_person_detail.html` renders every section when no tab is named. That is
/// what the panel asks for: it is one glance at a whole person inside the
/// tree, not somewhere to navigate.
#[tokio::test]
async fn the_tree_panel_still_shows_the_whole_record() {
    let src = showcase_bundle("panel-src");
    let (app, _p) = app_with_bundle("panel", &src);

    let panel =
        sections_in(&body_string(get_admin(&app, &format!("/tree/panel/{JULES}")).await).await);
    let record =
        sections_in(&body_string(get_admin(&app, &format!("/person/{JULES}")).await).await);
    assert!(
        panel.len() > record.len(),
        "the panel names no tab and so shows more than any single tab does: \
         panel {panel:?} vs record tab {record:?}"
    );
    for id in ["identity", "life"] {
        assert!(
            panel.iter().any(|s| s == id),
            "{id} is missing from the panel, which names no tab: {panel:?}"
        );
    }
}

/// A tab the reader clicked is a question, and a blank page is not an answer.
///
/// Everywhere else a section with no content is omitted, and that is right —
/// it makes the page a readout of what the bundle holds. A tab is different.
/// Before this, a signed-out reader opening Media on a person with no
/// documents got an entirely empty page: the section rendered only for an
/// administrator, because only they had the upload form to put in it. Most
/// people on a converted bundle have no documents, so that was the common
/// case, not the edge one.
#[tokio::test]
async fn an_empty_media_tab_explains_itself_instead_of_being_blank() {
    // A person with nothing attached, which is what most of a converted
    // bundle looks like — the showcase fixture is deliberately the opposite.
    let dir = scratch("media-empty-src");
    let path = dir.join("bare.axgf");
    let flat = json!({
        "manifest": {"axgf": "1.0"},
        "persons": {
            JULES: {
                "id": JULES, "type": "person", "axgf_version": "1.0",
                "identity": {"name": {"display": "Anna Bare", "components": []}},
                "visibility": "public"
            }
        },
        "families": {}, "events": {}, "links": {}, "occupations": {},
        "sources": {}, "places": {}, "documents": {}
    });
    std::fs::write(
        &path,
        axgf_cms::state::export_to_bytes(&flat.to_string()).expect("export"),
    )
    .expect("write");
    let (app, _p) = app_with_bundle("media-empty", &path);

    // Signed out: no upload form, and previously nothing else either.
    let body = body_string(get(&app, &format!("/person/{JULES}?tab=media")).await).await;
    assert!(
        body.contains("<section id=\"evidence\""),
        "the tab renders its section even with nothing in it: {body}"
    );
    assert!(
        body.contains("Nothing is attached to this record"),
        "and says so in words"
    );
    assert!(
        !body.contains("/document"),
        "without offering an upload to somebody who may not upload"
    );

    // Signed in: the same sentence, plus the form.
    let admin = body_string(get_admin(&app, &format!("/person/{JULES}?tab=media")).await).await;
    assert!(admin.contains("Nothing is attached to this record"));
    assert!(
        admin.contains(&format!("/admin/person/{JULES}/document")),
        "an administrator is offered the upload"
    );
}

/// The tabs say how much is behind them, and say nothing when it is nothing.
#[tokio::test]
async fn a_tab_counts_what_is_behind_it_and_omits_a_zero() {
    let src = showcase_bundle("tab-count-src");
    let (app, _p) = app_with_bundle("tab-count", &src);

    let body = body_string(get_admin(&app, &format!("/person/{JULES}")).await).await;
    let nav = body
        .split(r#"<nav class="person-tabs""#)
        .nth(1)
        .and_then(|s| s.split("</nav>").next())
        .expect("the tab bar");

    assert!(
        nav.contains("tab-count"),
        "a tab with content says how much: {nav}"
    );
    // "Media 0" is a worse thing to read than "Media", and the tab opens and
    // explains itself either way.
    assert!(
        !nav.contains(">0</span>"),
        "a zero is omitted rather than printed: {nav}"
    );
}
