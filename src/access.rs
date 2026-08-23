//! Who may see what, and who may change what.
//!
//! Two separate questions, deliberately kept apart:
//!
//! * **Visibility** decides what a request may *read*. It comes from the
//!   entity — `person.identity.visibility`, `link.visibility` — measured
//!   against the ceiling the requester's role grants. It is the same ladder
//!   the AXGF specification defines, not a parallel invention.
//! * **Family scope** decides what an account may *write*. It comes from the
//!   account, not the entity, and it never widens or narrows what that account
//!   can see. A contributor restricted to one branch still reads the whole
//!   tree at their visibility ceiling; they simply cannot edit outside it.
//!
//! # Enforcement happens here, not in a template
//!
//! Every read path resolves a [`Visible`] set *before* building anything, and
//! passes it into the builders — [`crate::tree`], [`crate::person`] — which
//! skip what is not in it. Nothing hidden is rendered and then styled away,
//! because markup that reaches the browser has already left the building. The
//! JSON endpoints take the same set.
//!
//! # The default when a record says nothing
//!
//! AXGF makes `visibility` optional, so a converted GEDCOM carries none at
//! all. Guessing `public` would publish living people the moment a bundle was
//! imported; guessing `members` would blank every converted bundle for
//! visitors and look broken. The rule is therefore stated on one axis the
//! format *does* carry:
//!
//! > An explicit `visibility` always wins. Where there is none, a person
//! > marked `is_living` is `members`, and everyone else is `public`.
//!
//! A living person's record is the one with an actual privacy interest;
//! publishing the dead is what genealogy is for.

use std::collections::BTreeSet;
use std::sync::Arc;

use serde_json::Value;

use crate::acl::{Role, User, Visibility};

/// Which persons a request may read.
///
/// `All` is not an optimisation detail — it is the common case (an admin, or
/// a bundle whose every person is public), and it keeps those requests on
/// exactly the code path they were on before filtering existed, at exactly
/// the same cost.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Visible {
    /// Every person in the bundle.
    All,
    /// Only these person ids.
    ///
    /// Behind an [`Arc`] so that [`AppState`](crate::state::AppState) can hand
    /// the same resolved set to every request at that ceiling. Cloning a
    /// `Visible` is then a refcount bump rather than 866 string allocations,
    /// which is what makes memoising it worth doing at all.
    Only(Arc<BTreeSet<String>>),
}

impl Visible {
    /// Whether `id` may be read.
    pub fn allows(&self, id: &str) -> bool {
        match self {
            Visible::All => true,
            Visible::Only(set) => set.contains(id),
        }
    }

    /// The restricted set, for the layout's `only` parameter.
    pub fn set(&self) -> Option<&BTreeSet<String>> {
        match self {
            Visible::All => None,
            Visible::Only(set) => Some(set),
        }
    }

    /// How many of `total` persons this permits, given the bundle's count.
    pub fn count(&self, total: usize) -> usize {
        match self {
            Visible::All => total,
            Visible::Only(set) => set.len(),
        }
    }

    /// Whether anything at all is hidden.
    pub fn is_all(&self) -> bool {
        matches!(self, Visible::All)
    }
}

/// The visibility a person's record actually carries.
///
/// See the module documentation for why an absent value depends on
/// `is_living` rather than defaulting to one constant.
pub fn person_visibility(person: &Value) -> Visibility {
    let identity = person.get("identity");
    if let Some(v) = identity
        .and_then(|i| i.get("visibility"))
        .and_then(Value::as_str)
        .and_then(Visibility::parse)
    {
        return v;
    }
    let living = identity
        .and_then(|i| i.get("is_living"))
        .and_then(Value::as_bool)
        .unwrap_or(false);
    if living {
        Visibility::Members
    } else {
        Visibility::Public
    }
}

/// The visibility a link carries. Links have no `is_living` to lean on, so an
/// absent value is `public` — the link says nothing the endpoints do not.
pub fn link_visibility(link: &Value) -> Visibility {
    link.get("visibility")
        .and_then(Value::as_str)
        .and_then(Visibility::parse)
        .unwrap_or(Visibility::Public)
}

/// Resolve which persons a ceiling permits.
///
/// Scans the bundle once. When nothing is hidden the result is [`Visible::All`]
/// rather than a set holding every id, so a public bundle costs a scan and no
/// allocation, and every downstream builder stays on its original path.
pub fn visible_persons(flat: &Value, ceiling: Visibility) -> Visible {
    // An admin reads everything by definition; there is nothing to compute.
    if ceiling >= Visibility::Private {
        return Visible::All;
    }
    let Some(persons) = flat.get("persons").and_then(Value::as_object) else {
        return Visible::All;
    };
    let mut allowed: BTreeSet<String> = BTreeSet::new();
    let mut hidden = 0usize;
    for (id, p) in persons {
        if person_visibility(p) <= ceiling {
            allowed.insert(id.clone());
        } else {
            hidden += 1;
        }
    }
    if hidden == 0 {
        Visible::All
    } else {
        Visible::Only(Arc::new(allowed))
    }
}

// ---------------------------------------------------------------------------
// the request-scoped lens
// ---------------------------------------------------------------------------

/// What one request may read, resolved once and threaded through every builder.
///
/// # Existence is disclosed; data is not
///
/// A person the requester may not read is *redacted*, not omitted: their card
/// still occupies its place in the tree and they still count as a parent, but
/// they carry no name, no dates, no gender and no link. Two reasons, and the
/// first is the one that decides it:
///
/// 1. Omission is a false statement. A record that lists one parent where the
///    bundle holds two asserts something about the genealogy that is not true.
///    For an application whose whole argument is that a format should preserve
///    what it actually knows, silently dropping a relationship is the one
///    behaviour it cannot have. Redaction says "there is something here you may
///    not see", which is true.
/// 2. With the `is_living` default, omission would make every converted bundle
///    look as though the family died out two generations ago — every living
///    person is `members`, so a signed-out visitor would see a tree that simply
///    stops. That reads as a broken import, not as a privacy control.
///
/// The consequence is deliberate and worth stating plainly: an anonymous
/// visitor can learn that a hidden person *exists*, that they sit in a given
/// generation, and how they connect. They learn nothing else — not a name, not
/// a date, not a gender, and no way to reach the record.
#[derive(Debug, Clone)]
pub struct Lens {
    persons: Visible,
    ceiling: Visibility,
}

impl Lens {
    /// Resolve the lens for `ceiling` over `flat`. One scan, at the top of the
    /// request, and never again inside it.
    pub fn resolve(flat: &Value, ceiling: Visibility) -> Self {
        Self {
            persons: visible_persons(flat, ceiling),
            ceiling,
        }
    }

    /// Build a lens from an already-resolved set. Used by
    /// [`AppState::read_as`](crate::state::AppState::read_as), which memoises
    /// the set per bundle version.
    pub fn from_parts(persons: Visible, ceiling: Visibility) -> Self {
        Self { persons, ceiling }
    }

    /// A lens that hides nothing — an admin, or a context with no reader to
    /// filter for, such as the conversion preview of an uploaded GEDCOM.
    pub fn unrestricted() -> Self {
        Self {
            persons: Visible::All,
            ceiling: Visibility::Private,
        }
    }

    /// Whether this request may read `id`'s data.
    pub fn sees_person(&self, id: &str) -> bool {
        self.persons.allows(id)
    }

    /// Whether an entity carrying its own `visibility` field may be read.
    ///
    /// Used for the collections that are not persons — links above all, where
    /// the *relationship* can be more sensitive than either endpoint.
    pub fn sees_entity(&self, entity: &Value) -> bool {
        link_visibility(entity) <= self.ceiling
    }

    /// The permitted set, or `None` when nothing is hidden.
    pub fn set(&self) -> Option<&BTreeSet<String>> {
        self.persons.set()
    }

    /// Whether this lens hides nothing at all.
    pub fn is_all(&self) -> bool {
        self.persons.is_all()
    }

    /// The visibility ceiling behind this lens.
    pub fn ceiling(&self) -> Visibility {
        self.ceiling
    }

    /// How many of `total` persons are readable.
    pub fn count(&self, total: usize) -> usize {
        self.persons.count(total)
    }

    /// The underlying set, for the layout's `only` parameter.
    pub fn visible(&self) -> &Visible {
        &self.persons
    }
}

impl Default for Lens {
    fn default() -> Self {
        Self::unrestricted()
    }
}

/// Which documents a request may read.
///
/// A document is reached through the person it is attached to, so that is what
/// governs it: the bytes are served when some person the requester may read
/// attaches it. A document no person attaches has no owner to inherit from and
/// is served to signed-in accounts only — it is either an orphan of an edit or
/// something a tool put there, and neither is public by default.
pub fn may_read_document(flat: &Value, vis: &Visible, signed_in: bool, document_id: &str) -> bool {
    if vis.is_all() {
        return true;
    }
    let Some(persons) = flat.get("persons").and_then(Value::as_object) else {
        return signed_in;
    };
    let documents = flat.get("documents").and_then(Value::as_object);

    let mut referenced = false;
    let mut check = |person_id: &str| -> bool {
        referenced = true;
        vis.allows(person_id)
    };

    // Direction one: the person lists the document.
    for (id, p) in persons {
        let attaches = p
            .get("documents")
            .and_then(Value::as_array)
            .is_some_and(|arr| {
                arr.iter()
                    .any(|d| d.get("document_id").and_then(Value::as_str) == Some(document_id))
            });
        if attaches && check(id) {
            return true;
        }
    }

    // Direction two: the document lists the person. Both directions exist in
    // AXGF and `person::documents_for` reads both, so a check that read only
    // one would disagree with the page it is meant to be protecting — in the
    // safe direction for an upload (which writes this one), and in the unsafe
    // direction for any bundle that carries the other.
    if let Some(links) = documents
        .and_then(|d| d.get(document_id))
        .and_then(|d| d.get("linked_to"))
        .and_then(Value::as_array)
    {
        for l in links {
            if l.get("entity_type").and_then(Value::as_str) != Some("person") {
                continue;
            }
            let Some(pid) = l.get("entity_id").and_then(Value::as_str) else {
                continue;
            };
            if !persons.contains_key(pid) {
                continue;
            }
            if check(pid) {
                return true;
            }
        }
    }

    if referenced {
        false
    } else {
        signed_in
    }
}

// ---------------------------------------------------------------------------
// the requester
// ---------------------------------------------------------------------------

/// Who is asking, resolved once per request.
///
/// Built by [`crate::auth::viewer`] from the session cookie. Anonymous is a
/// first-class case, not an error: most of this site is meant to be readable
/// without an account, at the `public` ceiling.
#[derive(Debug, Clone, Default)]
pub struct Viewer {
    /// The signed-in account, when there is one.
    pub user: Option<User>,
    /// True when this session came from `--admin-token` rather than an
    /// account. It grants admin for the session and owns no preferences.
    pub emergency: bool,
}

impl Viewer {
    /// An anonymous requester.
    pub fn anonymous() -> Self {
        Self::default()
    }

    /// The emergency-recovery admin.
    pub fn emergency_admin() -> Self {
        Self {
            user: None,
            emergency: true,
        }
    }

    /// The role this requester acts with, if any.
    pub fn role(&self) -> Option<Role> {
        if self.emergency {
            return Some(Role::Admin);
        }
        self.user.as_ref().map(|u| u.role)
    }

    /// Whether anyone is signed in at all.
    pub fn signed_in(&self) -> bool {
        self.emergency || self.user.is_some()
    }

    /// The highest visibility this requester may read.
    pub fn ceiling(&self) -> Visibility {
        match self.role() {
            Some(r) => r.ceiling(),
            None => Visibility::Public,
        }
    }

    /// May create, update and upload — subject to family scope.
    pub fn may_write(&self) -> bool {
        self.role().is_some_and(Role::may_write)
    }

    /// May manage accounts, delete, dedup, validate and export.
    pub fn is_admin(&self) -> bool {
        self.role().is_some_and(Role::is_admin)
    }

    /// The username to show, and to record in the edit journal.
    pub fn name(&self) -> &str {
        match (&self.user, self.emergency) {
            (Some(u), _) => &u.username,
            (None, true) => "emergency-token",
            (None, false) => "",
        }
    }

    /// The account's stored interface language, if it has one.
    pub fn language(&self) -> Option<&str> {
        self.user.as_ref().and_then(|u| u.language.as_deref())
    }

    /// The account's stored theme, if it has one.
    pub fn theme(&self) -> Option<&str> {
        self.user.as_ref().and_then(|u| u.theme.as_deref())
    }

    /// The persons this requester may read in `flat`.
    pub fn visible(&self, flat: &Value) -> Visible {
        visible_persons(flat, self.ceiling())
    }

    /// The read lens for this requester, resolved once per request.
    pub fn lens(&self, flat: &Value) -> Lens {
        Lens::resolve(flat, self.ceiling())
    }

    /// The persons this requester may *edit*, or `None` for the whole tree.
    ///
    /// Computed once per request by the caller and applied to every write in
    /// it; recomputing per entity would walk the families repeatedly for an
    /// answer that cannot change inside one request.
    pub fn scope(&self, flat: &Value) -> Option<BTreeSet<String>> {
        let roots = self.user.as_ref().map(|u| u.family_scope.as_slice())?;
        crate::acl::scope_set(flat, roots)
    }
}

/// Why a write was refused, so the page can say which rule stopped it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Denied {
    /// Not signed in, or signed in as a viewer.
    Role,
    /// Signed in with the right role, but outside the account's branch.
    Scope,
}

/// Every person an entity is *about*, which is what a family scope is checked
/// against.
///
/// A scope limits a contributor to one branch, so the question for any write
/// is "which people does this record concern?" — and for everything except a
/// person that means reading the entity's own references. A source or a place
/// concerns nobody in particular and returns empty, which [`check_write`]
/// treats as out of scope for a scoped account: there is no branch to measure
/// it against, so permitting it would be a hole in the scope rather than an
/// exception to it.
pub fn subjects_of(kind: axgf_rs::EntityKind, entity: &Value) -> Vec<String> {
    use axgf_rs::EntityKind as K;
    let str_at = |v: &Value, k: &str| v.get(k).and_then(Value::as_str).map(str::to_string);
    let mut out = Vec::new();

    match kind {
        K::Person => out.extend(str_at(entity, "id")),
        K::Family => {
            if let Some(ps) = entity
                .get("union")
                .and_then(|u| u.get("persons"))
                .and_then(Value::as_array)
            {
                out.extend(ps.iter().filter_map(|p| str_at(p, "person_id")));
            }
            if let Some(cs) = entity.get("children").and_then(Value::as_array) {
                out.extend(cs.iter().filter_map(|c| str_at(c, "person_id")));
            }
        }
        K::Event => {
            if let Some(ps) = entity.get("participants").and_then(Value::as_array) {
                out.extend(ps.iter().filter_map(|p| {
                    (p.get("entity_type").and_then(Value::as_str) == Some("person"))
                        .then(|| str_at(p, "entity_id"))
                        .flatten()
                }));
            }
        }
        K::Link => {
            for end in ["from", "to"] {
                if let Some(e) = entity.get(end) {
                    if e.get("entity_type").and_then(Value::as_str) == Some("person") {
                        out.extend(str_at(e, "entity_id"));
                    }
                }
            }
        }
        K::Occupation => out.extend(str_at(entity, "person_id")),
        K::Document => {
            if let Some(ls) = entity.get("linked_to").and_then(Value::as_array) {
                out.extend(ls.iter().filter_map(|l| {
                    (l.get("entity_type").and_then(Value::as_str) == Some("person"))
                        .then(|| str_at(l, "entity_id"))
                        .flatten()
                }));
            }
        }
        // A source and a place are about evidence and geography, not people.
        _ => {}
    }
    out.sort();
    out.dedup();
    out
}

/// Check a write against role and scope.
///
/// `subjects` are the people the write is about — see [`subjects_of`]. **All**
/// of them must be inside the scope, not merely one: a family with a partner
/// from outside the branch would otherwise be a way to rewrite that person's
/// parentage from inside it. An empty list means the write is about no person
/// in particular, which a scoped account may not do at all.
pub fn check_write(
    viewer: &Viewer,
    scope: Option<&BTreeSet<String>>,
    subjects: &[String],
) -> Result<(), Denied> {
    if !viewer.may_write() {
        return Err(Denied::Role);
    }
    let Some(scope) = scope else {
        return Ok(()); // unscoped: the whole tree
    };
    if subjects.is_empty() || !subjects.iter().all(|s| scope.contains(s)) {
        return Err(Denied::Scope);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn bundle() -> Value {
        json!({
            "persons": {
                "pub":  {"identity": {"visibility": "public", "is_living": false}},
                "mem":  {"identity": {"visibility": "members", "is_living": false}},
                "con":  {"identity": {"visibility": "contributors", "is_living": false}},
                "priv": {"identity": {"visibility": "private", "is_living": false}},
                "living_unmarked": {"identity": {"is_living": true}},
                "dead_unmarked":   {"identity": {"is_living": false}}
            }
        })
    }

    #[test]
    fn an_explicit_visibility_is_taken_literally() {
        let b = bundle();
        let p = |k: &str| b["persons"][k].clone();
        assert_eq!(person_visibility(&p("pub")), Visibility::Public);
        assert_eq!(person_visibility(&p("mem")), Visibility::Members);
        assert_eq!(person_visibility(&p("con")), Visibility::Contributors);
        assert_eq!(person_visibility(&p("priv")), Visibility::Private);
    }

    #[test]
    fn an_absent_visibility_follows_is_living() {
        let b = bundle();
        assert_eq!(
            person_visibility(&b["persons"]["living_unmarked"]),
            Visibility::Members,
            "a living person with no stated visibility is not published"
        );
        assert_eq!(
            person_visibility(&b["persons"]["dead_unmarked"]),
            Visibility::Public,
            "publishing the dead is what genealogy is for"
        );
    }

    #[test]
    fn each_ceiling_admits_exactly_its_own_rungs() {
        let b = bundle();
        let ids = |v: &Visible| match v {
            Visible::All => vec!["ALL".to_string()],
            Visible::Only(s) => s.iter().cloned().collect(),
        };

        let anon = ids(&visible_persons(&b, Visibility::Public));
        assert_eq!(anon, vec!["dead_unmarked", "pub"]);

        let viewer = ids(&visible_persons(&b, Visibility::Members));
        assert_eq!(
            viewer,
            vec!["dead_unmarked", "living_unmarked", "mem", "pub"]
        );

        let contributor = ids(&visible_persons(&b, Visibility::Contributors));
        assert!(contributor.contains(&"con".to_string()));
        assert!(!contributor.contains(&"priv".to_string()));

        assert_eq!(
            visible_persons(&b, Visibility::Private),
            Visible::All,
            "an admin needs no set built"
        );
    }

    #[test]
    fn a_bundle_with_nothing_hidden_costs_no_set() {
        // The public-bundle case stays on exactly the path it was on before
        // filtering existed — which is what protects the render budget.
        let b = json!({"persons": {
            "a": {"identity": {"visibility": "public"}},
            "b": {"identity": {"is_living": false}}
        }});
        assert_eq!(visible_persons(&b, Visibility::Public), Visible::All);
    }

    #[test]
    fn a_document_is_readable_through_a_person_who_is() {
        let flat = json!({
            "persons": {
                "seen":   {"identity": {"visibility": "public"},
                           "documents": [{"document_id": "shared"}, {"document_id": "open"}]},
                "hidden": {"identity": {"visibility": "private"},
                           "documents": [{"document_id": "shared"}, {"document_id": "secret"}]}
            }
        });
        let vis = visible_persons(&flat, Visibility::Public);
        assert!(may_read_document(&flat, &vis, false, "open"));
        assert!(
            may_read_document(&flat, &vis, false, "shared"),
            "a document a visible person attaches is readable through them"
        );
        assert!(
            !may_read_document(&flat, &vis, false, "secret"),
            "a document only a hidden person attaches must not be served"
        );
        assert!(
            !may_read_document(&flat, &vis, false, "orphan"),
            "an unattached document is not public"
        );
        assert!(
            may_read_document(&flat, &vis, true, "orphan"),
            "…but a signed-in account may fetch it"
        );
    }

    #[test]
    fn an_anonymous_viewer_reads_public_only() {
        let v = Viewer::anonymous();
        assert_eq!(v.ceiling(), Visibility::Public);
        assert!(!v.signed_in());
        assert!(!v.may_write());
        assert!(!v.is_admin());
        assert_eq!(v.role(), None);
    }

    #[test]
    fn the_emergency_token_is_an_admin_without_an_account() {
        let v = Viewer::emergency_admin();
        assert!(v.is_admin());
        assert!(v.may_write());
        assert_eq!(v.ceiling(), Visibility::Private);
        assert!(
            v.user.is_none(),
            "it owns no preferences and no journal name"
        );
        assert_eq!(v.name(), "emergency-token");
    }

    fn s(v: &str) -> String {
        v.to_string()
    }

    fn viewer_with(role: Role, scope: &[&str]) -> Viewer {
        let mut u = crate::acl::new_user("u", "correct horse battery", role).unwrap();
        u.family_scope = scope.iter().map(|s| s.to_string()).collect();
        Viewer {
            user: Some(u),
            emergency: false,
        }
    }

    #[test]
    fn a_viewer_may_not_write_at_all() {
        let v = viewer_with(Role::Viewer, &[]);
        assert_eq!(check_write(&v, None, &[s("anyone")]), Err(Denied::Role));
    }

    #[test]
    fn a_scoped_contributor_writes_inside_the_branch_and_nowhere_else() {
        let v = viewer_with(Role::Contributor, &["root"]);
        let scope: BTreeSet<String> = ["root", "child"].iter().map(|s| s.to_string()).collect();
        assert_eq!(check_write(&v, Some(&scope), &[s("child")]), Ok(()));
        assert_eq!(
            check_write(&v, Some(&scope), &[s("stranger")]),
            Err(Denied::Scope)
        );
        assert_eq!(
            check_write(&v, Some(&scope), &[]),
            Err(Denied::Scope),
            "an edit with no person to check is a hole in the scope, not an \
             exception to it"
        );
        assert_eq!(
            check_write(&v, Some(&scope), &[s("child"), s("stranger")]),
            Err(Denied::Scope),
            "one foot outside the branch is outside the branch: a family with \
             an out-of-scope partner would otherwise rewrite their parentage"
        );
    }

    #[test]
    fn an_unscoped_contributor_writes_anywhere() {
        let v = viewer_with(Role::Contributor, &[]);
        assert_eq!(check_write(&v, None, &[s("stranger")]), Ok(()));
        assert_eq!(check_write(&v, None, &[]), Ok(()));
    }

    #[test]
    fn scope_limits_writing_but_never_reading() {
        // The rule that keeps the two systems apart: a branch-scoped account
        // still reads the whole tree at its ceiling.
        let v = viewer_with(Role::Contributor, &["root"]);
        assert_eq!(v.ceiling(), Visibility::Contributors);
        let b = bundle();
        let vis = v.visible(&b);
        assert!(vis.allows("con"), "reading is governed by visibility alone");
        assert!(!vis.allows("priv"));
    }
}
