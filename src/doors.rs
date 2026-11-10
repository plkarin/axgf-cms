//! Who may open each part of the site (docs/adr/0046): which part an address
//! belongs to, and what the masthead offers a reader once the doors are
//! known. The rule itself is applied in one place, before any page is drawn
//! (`routes::doors`); the masthead only stops offering what would be
//! refused.

use serde::Serialize;

use crate::site::{Doors, Level, Part};

/// The part of the site an address belongs to, or none: the sign-in, the
/// administration, an account's own pages, the preferences, the static files,
/// `/health` and the MCP keep their own rules.
pub fn part_of(path: &str) -> Option<Part> {
    match path {
        "/" => return Some(Part::River),
        "/about" => return Some(Part::Family),
        "/tree" => return Some(Part::Tree),
        "/impact" => return Some(Part::Impact),
        "/convert" => return Some(Part::Import),
        "/search" | "/search.json" | "/quality" | "/quality.csv" | "/quality.json" => {
            return Some(Part::Records)
        }
        _ => {}
    }
    let prefixed = [
        ("/river/", Part::River),
        ("/tree/", Part::Tree),
        ("/impact/", Part::Impact),
        ("/convert/", Part::Import),
        ("/person/", Part::Records),
        ("/notes/", Part::Records),
        ("/document/", Part::Records),
        ("/quality/", Part::Records),
    ];
    prefixed
        .into_iter()
        .find(|(prefix, _)| path.starts_with(prefix))
        .map(|(_, part)| part)
}

/// Whether a request is a script's or a file's rather than a page's: what a
/// refusal answers with a status, not with a page to go to.
pub fn is_data(path: &str, accept: Option<&str>) -> bool {
    accept.is_some_and(|a| a.contains("application/json"))
        || path.ends_with(".json")
        || path.ends_with(".csv")
        || path.ends_with("/pdf")
        || path.ends_with("/state")
        || path == "/river/data"
        || path.starts_with("/tree/panel/")
        || path.starts_with("/document/")
}

/// The document an address under `/document/` names.
pub fn document_of(path: &str) -> Option<&str> {
    path.strip_prefix("/document/")?.split('/').next()
}

/// What the masthead offers a reader, once the doors are known: a part's
/// link when it is open to them, the sections' row when it would hold more
/// than one, the person search with the records, and the reading
/// preferences to whoever may read more than the family's page.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Open {
    pub family: bool,
    pub river: bool,
    pub tree: bool,
    pub records: bool,
    pub impact: bool,
    pub import: bool,
    /// The row of sections: two at least.
    pub nav: bool,
    pub search: bool,
    pub prefs: bool,
}

impl Open {
    /// Everything, as before the doors: what a page drawn without the
    /// archive's settings shows.
    pub fn everything() -> Self {
        Open::of(&Doors::everyone(), false, false, false)
    }

    pub fn of(doors: &Doors, signed_in: bool, may_write: bool, is_admin: bool) -> Self {
        let lets = |level: Level| level.admits(signed_in, may_write, is_admin);
        let (family, river, tree, records, impact, import) = (
            lets(doors.family),
            lets(doors.river),
            lets(doors.tree),
            lets(doors.records),
            lets(doors.impact),
            lets(doors.import),
        );
        let sections = [river, tree, family, impact, import]
            .into_iter()
            .filter(|open| *open)
            .count();
        Open {
            family,
            river,
            tree,
            records,
            impact,
            import,
            nav: sections >= 2,
            search: records,
            prefs: signed_in || river || tree || records || impact || import,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_address_of_a_part_is_of_that_part() {
        assert_eq!(part_of("/"), Some(Part::River));
        assert_eq!(part_of("/river/data"), Some(Part::River));
        assert_eq!(part_of("/about"), Some(Part::Family));
        assert_eq!(part_of("/tree/panel/x"), Some(Part::Tree));
        assert_eq!(part_of("/person/x/afg/y/state"), Some(Part::Records));
        assert_eq!(part_of("/search.json"), Some(Part::Records));
        assert_eq!(part_of("/document/x/thumb"), Some(Part::Records));
        assert_eq!(part_of("/impact/x"), Some(Part::Impact));
        assert_eq!(part_of("/convert/download/x"), Some(Part::Import));
        for own in [
            "/admin/login",
            "/account",
            "/static/app.css",
            "/health",
            "/mcp",
            "/settings",
            "/register",
        ] {
            assert_eq!(part_of(own), None, "{own}");
        }
        // A prefix is a prefix of a path, not of a word.
        assert_eq!(part_of("/trees"), None);
        assert_eq!(document_of("/document/abc/thumb"), Some("abc"));
    }

    #[test]
    fn a_visitor_of_a_closed_site_is_offered_nothing_but_the_sign_in() {
        let closed = Open::of(&Doors::default(), false, false, false);
        assert!(closed.family && !closed.river && !closed.records);
        assert!(!closed.nav && !closed.search && !closed.prefs);
        let member = Open::of(&Doors::default(), true, false, false);
        assert!(member.nav && member.search && member.prefs);
        let doors = Doors {
            import: Level::Contributors,
            ..Doors::default()
        };
        assert!(!Open::of(&doors, true, false, false).import);
        assert!(Open::of(&doors, true, true, false).import);
        assert!(Open::everything().nav && Open::everything().prefs);
    }
}
