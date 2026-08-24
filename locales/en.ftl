# axgf-cms — interface strings, English.
#
# This file is the source of truth: every other locale is measured against it,
# and a key missing elsewhere falls back to what is written here.
#
# THE RULE: these are *interface* strings. Nothing here ever renders a person's
# name, a place, a note, an occupation or a source title — those come from the
# bundle, in their own language and script, whatever the interface is set to.
#
# Plurals use Fluent's selector so that CLDR's rules apply per language.
# English has two forms; Polish has three, Arabic six, Japanese one. Never
# write `{ $n } people` and hope.

app-name = axgf-cms

## Chrome

nav-tree = Tree
nav-convert = Convert GEDCOM
nav-admin = Admin
nav-sign-in = Sign in
nav-sign-out = Sign out
footer-served-from = Served from a single .axgf bundle. All genealogy logic lives in axgf-rs; the format is specified by axgf-spec.

## Preferences

prefs-title = Language and appearance
prefs-language = Language
prefs-language-note = This changes the interface only. Names, places and notes are always shown in their own language and script.
prefs-theme = Appearance
prefs-apply = Apply
prefs-reviewed = reviewed
prefs-machine = machine, { $coverage }%
prefs-machine-title = Translated without review by a native speaker. Genealogical vocabulary especially may be wrong. Corrections are welcome — see CONTRIBUTING.md.

theme-light = Light
theme-dark = Dark
theme-system = Match my system
theme-high-contrast = High contrast
theme-sepia = Sepia
theme-deuteranopia = Deuteranopia
theme-protanopia = Protanopia
theme-tritanopia = Tritanopia
theme-colour-blind-note = colour-blind safe
theme-contrast-note = maximum contrast

## Tree

tree-title-around = Around { $name }
tree-title-whole = The whole tree
tree-lede-focused = { $ancestors ->
        [one] One ancestor
       *[other] { $ancestors } ancestors
    }, { $descendants ->
        [one] one descendant
       *[other] { $descendants } descendants
    } and { $spouses ->
        [one] one partner
       *[other] { $spouses } partners
    }, { $depth } generations each way. Oldest at the bottom. Connector opacity is the relationship's confidence — a faint line is a claim the record is not sure of.
tree-lede-whole = Every person in the bundle. Oldest at the bottom, youngest at the top. Connector opacity is the relationship's confidence.
tree-filter-label = Filter visible cards
tree-filter-placeholder = Type a name…
tree-centre-on = Centre on
tree-depth = Generations each way
tree-show = Show
tree-whole-tree = Whole tree
tree-focused = Focused
tree-hidden-notice = { $n ->
        [one] One person is shown without their details
       *[other] { $n } people are shown without their details
    }
tree-hidden-because-role = , because their visibility is above what your account may read.
tree-hidden-because-anonymous = , because they are not public.
tree-hidden-sign-in = Sign in if you have an account.
tree-restricted-card = This person's record is not visible to you
tree-width-warning = This canvas is { $width } pixels wide. Nobody scrolls that far to find an ancestor — the focused view exists for this reason.
tree-empty = This bundle contains nobody to draw.
tree-unplaced = In no recorded family
tree-legend-confidence = Line opacity is confidence
tree-recentre = Centre the tree here
tree-open-record = Open the full record

## The record

record-identity = Identity
record-life-events = Life events
record-family = Family
record-other-relationships = Other relationships
record-occupations = Occupations
record-places = Places
record-sources-documents = Sources and documents
record-notes = Notes
record-history = History
record-raw = Raw entity
record-raw-summary-note = the JSON this page was built from

record-identity-help = Every recorded name with its type, the period it was used and the source behind it, with the native script and its Latin transliteration side by side where they differ, plus gender, living status and visibility.
record-life-events-help = Birth, death and every event this person took part in, in date order, each with their role — so a marriage they merely witnessed appears alongside their own. An undated fact sorts last rather than pretending to come first.
record-family-help = Parents and siblings, then each union with its type, dates, place, how it ended and its children in birth order.
record-other-relationships-help = Every link with this person at either end, read from their side, so the same record shows as "godfather of" from one end and "godchild of" from the other.
record-occupations-help = Occupations as spans on one shared axis, so two posts can be compared by eye, with open-ended bars where a bound is missing.
record-places-help = Every place this record touches, with what happened at each and the border history that makes a place meaningful across time.
record-sources-documents-help = Every source names the facts on this page that rest on it, ordered by how strong the evidence is.
record-notes-help = Notes on this record, including text a converter could not parse and kept verbatim rather than dropping.
record-history-help = Every saved change to this record, newest first, from the edit journal beside the bundle. Who corrected what is a fact about this family's editors rather than about the family, so it is kept out of the .axgf and shown only to people signed in.
record-raw-help = Nothing here is generated for display: this is the record as the bundle stores it. A format worth arguing for should be readable without a tool.
record-help-toggle = What this section shows

record-gender = Gender
record-living = Living
record-visibility = Visibility
record-yes = yes
record-no = no
record-name-type = Name type
record-name-used = Used
record-name-evidence = Evidence
record-name-primary = primary name
record-transliteration = Latin transliteration
record-born = Born
record-died = Died
record-parents = Parents
record-siblings = Siblings
record-children = Children
record-spouse = Partner
record-union-ended = Ended
record-no-date = Date unknown
record-unknown-person = [Unknown]
record-restricted-person = Private
record-restricted-title = This person's record is not visible to you
record-absent-person-title = Referenced by this bundle but not present in it
record-confidence = Confidence
record-source = Source
record-role = Role
record-download = Download
record-attach-file = Attach a file
record-attach-hint = Up to { $mb } MB. Images are shown in the gallery; everything else is listed with a download link.
record-no-documents = No files are attached to this record.

## Access

access-restricted-title = Not visible to you
access-restricted-signed-in = This record's visibility puts it above what your account may read. An administrator can change either the record's visibility or your role.
access-restricted-anonymous = This record is not public. Sign in to see whether your account may read it.
access-role-title = Not for your role
access-role-admin = This is an administrator's page. Your account can create and edit records, but not manage accounts, delete entities or export the bundle.
access-role-write = Your account can read this bundle but not change it. An administrator can raise your role to contributor.
access-scope-title = Outside your branch
access-scope-named = Your account is restricted to one branch of the tree, and this record concerns somebody outside it. Every person a record names has to be inside your branch — a family with one partner from outside would otherwise be a way to rewrite that person's parentage.
access-scope-unnamed = Your account is restricted to one branch of the tree, and this record names nobody it could be measured against. Sources and places are edited by accounts with access to the whole tree.

## Errors

error-not-found-title = Not found
error-not-found-detail = That page does not exist in this bundle.
error-no-such-person-title = No such person
error-no-such-person-detail = This bundle contains no person with that id.
error-no-such-entity-title = No such entity
error-no-such-entity-detail = This bundle contains no entity with that id.
error-deleted-while-editing = This bundle contains no entity with that id. It may have been deleted while you were editing it.
error-no-such-file-title = No such file
error-no-such-file-detail = This bundle has no document with that id, or the document is recorded without a file — a referenced document names something held somewhere else.
error-not-an-image-title = Not an image
error-not-an-image-detail = There is no thumbnail for this document, because it is not an image this build can decode.
error-back = Back

## Sign in

login-title = Sign in
login-lede = Accounts are created by an administrator.
login-username = Username
login-password = Password
login-submit = Sign in
login-wrong = That username and password do not match.
login-token-wrong = That token is not correct.
login-throttled = Too many failed attempts. Wait a few minutes and try again.
login-no-accounts-title = This installation has no accounts yet.
login-no-accounts-detail = There is deliberately no setup page here — the window between deployment and the first login is exactly when an installation is unprotected, so the first administrator is created from the command line instead.
login-no-accounts-note = It prints a generated password to stderr once and never again. Until then the only way in is the emergency token below.
login-emergency-summary = Emergency access
login-emergency-detail = The shared token still opens an administrator session, and it exists for one purpose: getting back in when the .acl file has been lost or every administrator is locked out. It is not an account — it owns no preferences, and the edit journal records it as emergency-token rather than as a person. Its use is logged as a warning.
login-emergency-label = Emergency token
login-emergency-submit = Use emergency token
login-sign-in-prompt = Sign in to reach the admin panel.

## Admin

admin-title = Admin
admin-lede = Editing { $path } — { $total } entities, { $files ->
        [one] one attached file
       *[other] { $files } attached files
    }, { $size } on disk. Every change is written atomically; a refused change leaves the file untouched.
admin-entities = Entities
admin-create = Create
admin-new-kind = New { $kind }
admin-operations = Operations
admin-validate = Validate
admin-deduplicate = Deduplicate
admin-export = Export bundle
admin-accounts = Accounts
admin-roles-note = Validate, deduplicate, export, delete and account management are administrator-only. A contributor reaches every other page here.
admin-dedup-confirm = Deduplicate merges entities and rewrites the bundle. Continue?
admin-recent-changes = Recent changes
admin-recent-note = The last { $shown } of { $total ->
        [one] one recorded change
       *[other] { $total } recorded changes
    }, from { $path }. The journal lives beside the bundle, not inside it: a .axgf is copied and published, and who corrected what is a fact about this family's editors rather than about the family.
admin-sessions-open = { $n ->
        [one] One session open now.
       *[other] { $n } sessions open now.
    }
admin-no-changes-yet = Nothing has been changed through this application yet. Every save from here on is recorded in { $path }.
admin-last-validation = Last validation
admin-bundle-heavy = This bundle is { $size }. The whole file is read into memory at startup and held there, so past about { $warn } the application starts costing real memory and restarts get slow. That design suits a family archive, not a media library — if the attachments are growing without bound, they belong in a file store with the bundle referencing them.

admin-fields = Fields
admin-raw-json = Raw JSON
admin-raw-json-help = The whole entity, so nothing is uneditable — lists such as a family's partners and children, or a place's border history, live here. This is the starting document; the fields above are then written over the paths they own, so edit a value in one place or the other, not both. It must parse as JSON or nothing is saved.
admin-save = Save
admin-cancel = Cancel
admin-delete = Delete
admin-not-set = — not set —
admin-edit = Edit
admin-search = Search
admin-page-of = Page { $page } of { $pages }
admin-previous = Previous
admin-next = Next
admin-nothing-here = Nothing of this kind is recorded in this bundle yet.
admin-saved = Saved as version { $version } — { $summary }
admin-not-saved = Not saved
admin-created = Created
admin-not-created = Not created
admin-deleted = Deleted
admin-not-deleted = Not deleted — the bundle is unchanged
admin-delete-policy = Referential integrity
admin-what-changed = what changed
admin-field = Field
admin-from = From
admin-to = To
admin-version = version { $version }

## Accounts

accounts-title = Accounts
accounts-lede = Stored in { $path }, at mode 600, beside the bundle and never inside it. A .axgf is copied, mailed and published; password hashes in it would make every copy of the family tree a copy of the credential store.
accounts-existing = Existing
accounts-username = Username
accounts-role = Role
accounts-status = Status
accounts-branch = Branch
accounts-last-seen = Last seen
accounts-change = Change
accounts-you = (you)
accounts-active = active
accounts-disabled = disabled
accounts-never = never
accounts-whole-tree = whole tree
accounts-roots = { $n ->
        [one] one root
       *[other] { $n } roots
    }
accounts-add = Add an account
accounts-no-registration = There is no self-registration and no invitation flow, deliberately. For a family archive an administrator who knows everyone is enough, and it removes an abuse surface entirely rather than defending one.
accounts-password-hint = Leave blank and one is generated and shown once. At least { $min } characters if you set it yourself.
accounts-new-password-placeholder = new password (blank = keep)
accounts-email = Email
accounts-optional = (optional)
accounts-create = Create account
accounts-role-viewer = viewer — reads public and members records
accounts-role-contributor = contributor — also creates, edits and uploads
accounts-role-admin = admin — also manages accounts, deletes and exports
accounts-branch-hint = Limits what this account may edit to those people, their descendants and their spouses. It does not limit what they may read — that is governed by each record's visibility, and the two are kept separate on purpose.
accounts-branch-placeholder = one person id per line
accounts-ids-in-bundle = Person ids in this bundle
accounts-emergency-warning = You are signed in with the emergency token. It grants administrator rights for this session but is not an account: it owns no preferences, and the edit journal will record your changes as emergency-token rather than as a person. Create yourself a real account below and sign in with it.
accounts-created-with-password = Created { $username }. Their password is { $password } — it is shown once and stored only as an Argon2id hash, so pass it on now.
accounts-created = Created { $username }.
accounts-updated = Updated { $username }. Any session it had open has been signed out.
accounts-username-taken = That username is taken.
accounts-pick-role = Pick a role.
accounts-no-such = No such account.
accounts-last-admin = That is the only active administrator. Promote somebody else first — an installation with no administrator can only be recovered by editing the .acl file or using the emergency token.
accounts-not-saved = Not saved: { $error }

## Conflicts

conflict-title = Someone else changed this first
conflict-lede = { $who } saved a change to this { $kind } at { $when }, after you opened it. Your edit has not been saved, and nothing has been overwritten.
conflict-no-merge = Nothing is merged automatically here. A merge of two people's edits produces a record neither of them chose, and in a genealogy two editors disagreeing about a date usually means they are reading different sources — which is a question for a person, not for a program. Compare the two below and decide.
conflict-versions = You started from version { $expected }; the bundle now holds version { $current }.
conflict-both-changed = You both changed these
conflict-both-changed-detail = These fields were edited by both of you. Whatever you save will replace what { $who } put there:
conflict-different-fields = You changed different fields, so nothing of { $who }'s work is contested — but re-applying still writes your whole entity over theirs. Check the two columns before you save.
conflict-field-by-field = Field by field
conflict-theirs = What { $who } changed it to
conflict-yours = What you changed it to
conflict-unchanged-by-you = unchanged by you
conflict-unchanged-by-them = unchanged by them
conflict-nothing-differs = Neither version differs from the one you started from in any field this page shows. The version number moved, so somebody saved the record without changing anything it records.
conflict-what-now = What now
conflict-reapply = Re-apply your version on top of theirs
conflict-reapply-hint = This is your edit, carried forward against version { $version }. Edit it here to keep any of { $who }'s work you want, then save. Their version is shown below to copy from.
conflict-save-over = Save this over theirs
conflict-discard = Discard mine and start again
conflict-their-version = { $who }'s version, as the bundle currently holds it
conflict-history-of = This { $kind }'s history

## Home

home-lede = { $family } — { $total ->
        [one] one entity
       *[other] { $total } entities
    } in one .axgf bundle.
home-why-title = Why AXGF
home-what-this-bundle-has = What this bundle actually contains
home-browse-tree = Browse the tree
home-convert-gedcom = Convert a GEDCOM
home-see-example = See an example

## Conversion

convert-title = Convert GEDCOM to AXGF
convert-choose-file = GEDCOM file
convert-submit = Convert
convert-result-title = Conversion result
convert-download = Download the .axgf bundle
convert-diagnostics = What the converter reported
convert-unchanged-note = Conversion never touches the bundle this site is serving.

## Completeness

completeness-title = Bundle completeness
completeness-recorded = recorded
completeness-empty = empty
completeness-spec-field = AXGF field

## Dates
#
# The words of a date are interface text; its VALUE and its PRECISION are data
# and are never touched. "circa 1500" translates the word, not the number, and
# a date the source left at year precision stays at year precision.

date-unknown = Date unknown
date-not-recorded = Not recorded
date-circa = circa { $date }
date-between = between { $from } and { $to }
date-before = before { $date }
date-after = after { $date }
date-preserved = recorded as “{ $text }”
date-day-month-year = { $day } { $month } { $year }
date-month-year = { $month } { $year }
date-decade = the { $decade }s
date-century = the { $century ->
        [1] 1st
        [2] 2nd
        [3] 3rd
        [21] 21st
       *[other] { $century }th
    } century
date-quarter-century = the { $quarter ->
        [1] first
        [2] second
        [3] third
       *[other] fourth
    } quarter of the { $century ->
        [1] 1st
        [2] 2nd
        [3] 3rd
        [21] 21st
       *[other] { $century }th
    } century

month-1 = January
month-2 = February
month-3 = March
month-4 = April
month-5 = May
month-6 = June
month-7 = July
month-8 = August
month-9 = September
month-10 = October
month-11 = November
month-12 = December

## More error pages

error-back-to-start = Back to the start
error-payload-missing-title = No such file
error-payload-missing-detail = The payload for that document is not in the cache.
error-payload-unopenable-detail = The payload for that document could not be opened.
error-no-such-document-detail = This bundle has no document with that id.
error-bad-preference-title = Not one of the choices
error-bad-preference-detail = That is not a language or theme this site offers. Nothing was changed.
error-unknown-kind-title = Unknown kind
error-unknown-kind-detail = “{ $kind }” is not an entity kind. This bundle holds: { $kinds }.
error-io-title = Could not write the bundle
error-io-detail = { $error }. The bundle on disk is unchanged.
error-upload-too-large = That file is larger than the { $mb } MB limit. Nothing was stored, and the bundle is unchanged.
error-upload-empty = That upload carried no bytes. Nothing was stored.
error-upload-type = This archive does not store files of that type. Nothing was stored.
error-upload-refused = The library refused the document: { $reason }. The bundle is unchanged.
error-back-to-person = Back to the record
error-no-such-person-to-attach = This bundle contains no person with that id, so there is nothing to attach a document to.
error-upload-title = That upload was not stored
error-download-expired-title = That download has expired
error-download-expired-detail = Converted bundles are held for fifteen minutes. Convert the file again.
error-upload-none = No file was uploaded. Choose a file first.
error-upload-unsupported = That file is not a type this archive stores. Images, PDF, plain text, audio and video are accepted; the type is read from the file's own bytes, so renaming an executable does not get it in. SVG is refused outright, because an SVG can carry script.
error-export-unreadable-title = Could not read the exported bundle
error-export-unreadable-detail = { $error }

## Tree page, continued

tree-title-suffix = tree
tree-back-to-focused = Back to the focused view
tree-show-all = Show all { $n }
tree-width-notice = This view is { $width }px wide. Every generation is one row, and the widest one here sets that width — on a 1500px screen that is { $screens ->
        [one] one screen
       *[other] { $screens } screens
    } of horizontal scrolling. The focused view shows a few dozen people around one person instead, and every card re-centres it.
tree-confidence-label = Confidence:
tree-band-certain = certain
tree-band-high = high
tree-band-medium = medium
tree-band-low = speculative
tree-counts = { $drawn } of { $total } people · { $generations ->
        [one] one generation
       *[other] { $generations } generations
    }
tree-unplaced-count = { $n } unplaced
tree-contradicts-title = This bundle contradicts itself.
tree-contradicts-detail = Somebody is recorded as their own ancestor, or two people on one line of descent are recorded as a couple. No arrangement of rows can satisfy that, so the offending relationship was left out of the generation numbering and some rows may be wrong. Run the validator from the admin dashboard to find it.
tree-no-people = This bundle has no people yet.
tree-no-people-cta = Convert a GEDCOM to fill it.
tree-nobody-selected = Nobody to draw for that selection.
tree-nobody-selected-cta = Start from the default view.
tree-click-hint = Click any card to open that person's record in the panel; “Centre the tree here” in the panel re-roots the view.
tree-filter-matches = { $n ->
        [one] one match
       *[other] { $n } matches
    }
tree-edge-union = A recorded union
tree-edge-parentage = A recorded parentage

## Home page

home-empty = This bundle is empty. Convert a GEDCOM file to see what AXGF records that GEDCOM discards.
home-count = { $total ->
        [one] One entity
       *[other] { $total } entities
    } in one .axgf file — no database, no external services.
home-browse = Browse the tree
home-convert = Convert a GEDCOM file
home-why-1 = GEDCOM records what a genealogist concluded. AXGF also records how sure they were, and why. Every fact in this bundle carries a confidence from 0.0 to 1.0, and this site renders that confidence visually — a birth date recorded at 98% and a speculative parentage at 35% do not look alike anywhere on this site.
home-why-2 = AXGF also keeps the shape of what a source actually said. “circa 1500”, “before 1430” and “between 1920 and 1925” survive as distinct statements instead of collapsing into one blank field, and text no converter could parse is preserved rather than dropped. Relationships beyond blood — godparent, employer, witness, mentor — are first-class entities with their own dates, sources and confidence. Occupations are spans with a duration, not events with a single date.
home-why-spec = Read the specification at github.com/plkarin/axgf-spec.
home-in-this-bundle = What is in this bundle
home-showcase-title = AXGF-only features present in this bundle
home-showcase-note = These are the things this data expresses that a GEDCOM export could not carry.
home-showcase-example = See an example →
home-nothing-title = Nothing to show yet.
home-nothing-detail = Upload a GEDCOM on the conversion page to see what the format captures, or install with --with-sample to seed a small demonstration bundle.

## Showcase cards

showcase-links-title = { $n ->
        [one] One non-family relationship
       *[other] { $n } non-family relationships
    }
showcase-links-detail = Godparents, employers, witnesses and mentors, each with its own dates, source and confidence. GEDCOM has no way to state these at all.
showcase-occupations-title = { $n ->
        [one] One occupation recorded as a span
       *[other] { $n } occupations recorded as spans
    }
showcase-occupations-detail = “Schoolteacher, 1948–1978” is a state with a duration, rendered as a timeline bar rather than flattened into a dated event.
showcase-uncertain-title = { $n ->
        [one] One date that is honestly imprecise
       *[other] { $n } dates that are honestly imprecise
    }
showcase-uncertain-detail = Circa, before, after and between are preserved as distinct statements. A date the source could not pin down is not shown as if it were.
showcase-preserved-title = { $n ->
        [one] An unparseable date kept verbatim
       *[other] { $n } unparseable dates kept verbatim
    }
showcase-preserved-detail = Text no converter could interpret survives as a note instead of being silently dropped.
showcase-sources-title = { $n ->
        [one] One source graded by reliability
       *[other] { $n } sources graded by reliability
    }
showcase-sources-detail = { $primary ->
        [one] One primary source.
       *[other] { $primary } primary.
    } Every fact shows which evidence it rests on, and how strong that evidence is.
showcase-places-title = { $n ->
        [one] One place with border history
       *[other] { $n } places with border history
    }
showcase-places-detail = A town can belong to different countries at different times, and the record says which one applied when.
