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

app-name = ax-genealogy

## Chrome

nav-tree = Tree
nav-convert = Import
nav-admin = Admin
nav-sign-in = Sign in
nav-sign-out = Sign out
account-emergency = Emergency token
footer-credit = © 2026 · Designed and built by { $author }
footer-format-link = The AXGF format on GitHub

## Preferences

settings-title = Settings
edit-mode-label = Editing — what you save changes the family's records
display-menu-label = Language and display
display-settings = Theme and display…
settings-tabs-label = Settings sections
settings-tab-theme = Theme
settings-tab-language = Language
settings-tab-appearance = Appearance
settings-done = Done
prefs-language = Language
prefs-theme = Theme
prefs-background = Background
prefs-background-on = A soft wash of colour behind the page
prefs-apply = Apply
busy-saving = Saving…
busy-working = Working…
prefs-reviewed = reviewed
prefs-machine = machine, { $coverage }%
prefs-machine-complete = complete, not yet reviewed
prefs-machine-title = Translated without review by a native speaker. Genealogical vocabulary especially may be wrong — the words for a union, a godparent or a primary source differ by national record-keeping tradition. Corrections are welcome, and CONTRIBUTING.md says where to start.

theme-canopee = Canopy
theme-canopee-note = dark, forest green · the default
theme-fusion = Fusion
theme-fusion-note = dark, neon · a living tree
theme-galaxy = Galaxy
theme-galaxy-note = deep space · a solar system
theme-summit = Summit
theme-summit-note = dark, golden dawn · an ascent

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
    }, { $depth } generations each way.
tree-filter-label = Filter visible cards
tree-centre-on = Centre on
tree-depth = Generations each way
tree-show = Show
tree-hidden-notice = { $n ->
        [one] One person is shown without their details
       *[other] { $n } people are shown without their details
    }
tree-hidden-because-role = , because their visibility is above what your account may read.
tree-hidden-because-anonymous = , because they are not public.
tree-hidden-sign-in = Sign in if you have an account.
tree-restricted-card = This person's record is not visible to you
tree-empty = There is nobody to draw yet.
tree-unplaced = In no recorded family

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
life-nothing-recorded = No event, occupation or place is recorded for this person. On a converted file that is ordinary: GEDCOM carries what somebody wrote down, and most entries are a name and a date.
life-add-first = Record the first event
record-family-help = Parents and siblings, then each union with its type, dates, place, how it ended and its children in birth order.
record-other-relationships-help = Every link with this person at either end, read from their side, so the same record shows as "godfather of" from one end and "godchild of" from the other.
record-occupations-help = Occupations as spans on one shared axis, so two posts can be compared by eye, with open-ended bars where a bound is missing.
record-places-help = Every place this record touches, with what happened at each and the border history that makes a place meaningful across time.
record-sources-documents-help = Every source names the facts on this page that rest on it, ordered by how strong the evidence is.
record-notes-help = Notes on this record, including text a converter could not parse and kept verbatim rather than dropping.
record-history-help = Every saved change to this record, newest first. Who corrected what is a fact about the people keeping the tree rather than about the family in it, so it is kept out of the exported archive and shown only to relatives who are signed in.
record-raw-help = Nothing here is generated for display: this is the record exactly as it is stored, down to the field names. If you ever need to read the archive without this site, this is what you would see.
record-help-toggle = What this section shows

record-gender = Gender
record-living = Living
record-visibility = Visibility
record-yes = yes
record-no = no
record-name-type = Name type
record-name-used = Used
record-name-evidence = Evidence
record-transliteration = Latin transliteration
record-born = Born
record-died = Died
record-parents = Parents
record-siblings = Siblings
record-children = Children
record-unknown-person = [Unknown]
record-restricted-person = Private
record-restricted-title = This person's record is not visible to you
record-absent-person-title = Named in this tree but not recorded in it
record-confidence = Confidence
record-source = Source
record-download = Download

## Access

access-restricted-title = Not visible to you
access-restricted-signed-in = This record's visibility puts it above what your account may read. An administrator can change either the record's visibility or your role.
access-restricted-anonymous = This record is not public. Sign in to see whether your account may read it.
access-role-title = Not for your role
access-role-admin = This is an administrator's page. Your account can create and edit records, but not manage accounts, delete records or export the archive.
access-role-write = Your account can read this tree but not change it. An administrator can raise your role to contributor.
access-scope-title = Outside your branch
access-scope-named = Your account is restricted to one branch of the tree, and this record concerns somebody outside it. Every person a record names has to be inside your branch — a family with one partner from outside would otherwise be a way to rewrite that person's parentage.
access-scope-unnamed = Your account is restricted to one branch of the tree, and this record names nobody it could be measured against. Sources and places are edited by accounts with access to the whole tree.

## Errors

error-not-found-title = Not found
error-not-found-detail = That page does not exist here.
error-no-such-person-title = No such person
error-no-such-person-detail = There is no person here with that id.
error-merged-title = This record no longer exists.
error-merged-detail = It was merged into { $name } on { $date }. Links to the old address now lead to the merged record.
error-merged-open = Open the merged record
error-back-to-river = Back to the river
view-tree-back = Back to the tree
view-galaxy-back = Back to the galaxy
view-summit-back = Back to the mountain
error-no-such-entity-title = No such entity
error-no-such-entity-detail = There is no record here with that id.
error-deleted-while-editing = There is no record here with that id. It may have been deleted while you were editing it.
error-no-such-file-title = No such file
error-no-such-file-detail = There is no document here with that id, or the document is recorded without a file — a referenced document names something held somewhere else.
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
setup-title = Set up this archive
setup-lede = This installation holds no family yet. Choose how it begins. Nothing here is ever written over a family: this page closes as soon as there is one.
setup-door-archive-title = From an AXGF archive
setup-door-archive-detail = An .axgf file from another installation or from a backup. Its people, sources and photographs come across as they are.
setup-door-archive-file = Archive (.axgf)
setup-door-archive-go = Set up from this archive
setup-limit-gb = Up to { $n } GB, written straight to disk.
setup-door-scratch-title = From nothing
setup-door-scratch-detail = An empty archive. Add the first person, then their family, one record at a time.
setup-door-scratch-go = Add the first person
setup-door-gedcom-title = From a GEDCOM file
setup-door-gedcom-detail = The file every genealogy program exports: GEDCOM 5.5, 5.5.1, 5.5.5 or 7.0, as .ged or .gdz, in whatever encoding it was written. You see how it was read and what comes across before anything is kept.
setup-door-gedcom-file = GEDCOM file (.ged, .gdz)
setup-door-gedcom-go = Read this file
setup-never-over = To replace a family that is already here, restore a backup from the command line (axgf-cms restore); no page writes over one.
setup-adopt-title = Make this the archive’s family
setup-adopt-detail = Records that would become this installation’s family: { $total }. Nothing is kept until you press the button.
setup-adopt-go = Set up the archive with this family
setup-adopt-back = Choose another way in
setup-done-title = The archive is set up
setup-done-detail = Records in this installation’s family now: { $total }.
setup-done-photos = Photographs and documents that came with it: { $n }.
setup-done-mismatches = Files whose bytes did not match the fingerprint recorded for them: { $n }. They are kept; check them.
setup-done-river = Open the river
view-tree-open = Open the tree
view-galaxy-open = Open the galaxy
view-summit-open = Open the mountain
setup-done-admin = Administration
setup-refused-title = This could not become the archive
setup-refused-detail = Nothing was written. What stopped it:
setup-closed-title = This archive is already set up
setup-closed-detail = It holds a family, and no page writes over one. To replace it, restore a backup from the command line (axgf-cms restore).
setup-not-archive = This is not an .axgf archive.
setup-archive-too-large = This archive is larger than an upload may be.
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
admin-export = Export the archive
admin-accounts = Accounts
admin-roles-note = Validate, deduplicate, export, delete and account management are administrator-only. A contributor reaches every other page here.
admin-dedup-confirm = Deduplicating merges records and rewrites the archive. Continue?
admin-recent-changes = Recent changes
admin-recent-note = The last { $shown } of { $total ->
        [one] one recorded change
       *[other] { $total } recorded changes
    }, from { $path }.
admin-sessions-open = { $n ->
        [one] One session open now.
       *[other] { $n } sessions open now.
    }
admin-no-changes-yet = Nothing has been changed through this application yet. Every save from here on is recorded in { $path }.
admin-last-validation = Last validation
admin-bundle-heavy = This archive is { $size }. The whole of it is loaded at startup and held in memory, so past about { $warn } the site starts costing real memory and restarts get slow. That suits a family archive rather than a media library — if the attachments are growing without bound, keep them in a file store and have the archive point at them.

admin-fields = Fields
admin-raw-json = Raw JSON
form-more-fields = { $n ->
        [one] One more field
       *[other] { $n } more fields
    }
admin-raw-json-help = The whole entity, so nothing is uneditable — lists such as a family's partners and children, or a place's border history, live here. This is the starting document; the fields above are then written over the paths they own, so edit a value in one place or the other, not both. It must parse as JSON or nothing is saved.
admin-save = Save
admin-cancel = Cancel
place-editor-title = Edit a place
place-add-detail = Add detail to this place
place-names = Names
place-name-primary = Primary
place-name-lang = Language
place-name-value = Name
place-names-hint = One row per recorded name. A place administered by three empires carries three names; the primary is the one shown everywhere else.
place-where = Where it is
place-type = Type
place-region = Region
place-country-current = Country today
place-country-hint = ISO 3166-1 alpha-2, e.g. PL, FR, DE.
place-country-history = Border history
place-history-country = State
place-history-from = From
place-history-until = Until
place-country-history-hint = Which state held this place over which period. Genealogically significant: a record written in Russian in 1880 and one written in Polish in 1930 can name the same village.
place-coordinates = Coordinates
place-lat = Latitude
place-lon = Longitude
place-precision = Precision
place-identifiers = Identifiers
place-wikidata = Wikidata
place-geonames = GeoNames
place-used-by = { $name } is used by { $n ->
        [one] one other record
       *[other] { $n } other records
    }.
place-error-no-name = A place needs at least one name.
place-error-coords-pair = Latitude and longitude go together: give both, or neither.
place-error-coords-number = Latitude and longitude must be numbers.
place-error-coords-range = Latitude runs -90 to 90 and longitude -180 to 180.
place-type-continent = continent
place-type-country = country
place-type-region = region
place-type-department = department
place-type-city = city
place-type-village = village
place-type-district = district
place-type-street = street
place-type-building = building
place-type-farm = farm
place-type-island = island
place-type-historical = historical
place-type-unknown = unknown
place-precision-exact = exact
place-precision-building = building
place-precision-street = street
place-precision-city_center = city centre
place-precision-region_center = region centre
place-precision-country_center = country centre
place-precision-approximate = approximate

place-coordinates-hint = Typed by hand is the usual way. Many places recorded under a former administration cannot be found by a modern search at all.
place-geocode-search = Look this name up
place-geocode-hint = Sends the name, region and country to the geocoding service, one place at a time. Nothing is saved until you save it.
place-geocode-off = Name lookup is off. It needs a contact address the service can identify this installation by; start the server with --geocoder-contact to turn it on.
place-geocode-query = Searched for: { $q }
place-geocode-error = The lookup service could not be reached. The coordinate fields above still work.
place-geocode-none = Nothing found. For a village recorded under Russian, Prussian or Austrian administration this is the ordinary result; enter the position by hand.
place-geocode-not-a-place = not a settlement
place-geocode-use = Use this
place-geocode-attribution = Results from OpenStreetMap via Nominatim, under the Open Database Licence.

place-paste = Paste a position
place-paste-placeholder = a map link, or 52.0782795, 21.2508068
place-paste-read = Read it
place-paste-hint = A Google Maps or OpenStreetMap link, a geo: URI, a plain pair, or degrees-minutes-seconds such as 52°04'41.8"N 21°15'02.9"E.
place-paste-read-ok = Read into the fields above. Check it, then save.
place-paste-unreadable = That is not a position this can read. The fields above still take a plain pair of numbers.

place-map-hint = Click the map to place the point, or drag the pin. The fields above are the record.
place-map-clear = Clear the point
place-open-in-map = Find this place in OpenStreetMap, then paste the link back

person-tab-record = Record
person-tab-life = Events
person-tab-media = Media
person-tab-tree = Tree
person-tab-history = History
person-tree-depth = { $n } generations either way. The whole tree is below.
person-tree-alone = This record names no parents, partners or children, so there is no shape to draw around it.

record-no-evidence = Nothing is attached to this record — no source and no document. That is the ordinary state of a converted file rather than a fault in it: GEDCOM carries the facts and leaves behind whatever proved them.
record-no-evidence-signed-out = Sign in to attach one.
admin-delete = Delete
admin-not-set = — not set —
admin-edit = Edit
admin-page-of = Page { $page } of { $pages }
list-select-row = Select { $name }
list-kind-label = Type
list-select-page = Select every row on this page
list-selected = { $n ->
        [one] { $n } selected
       *[other] { $n } selected
    }
list-export = Export
list-escape = Esc to clear the selection
bulk-confirm-title = { $n ->
        [one] Delete { $n } record?
       *[other] Delete { $n } records?
    }
bulk-confirm-detail = A record that another one still refers to is refused, and then nothing is deleted.
bulk-confirm-go = Delete them
bulk-deleted = { $n ->
        [one] { $n } record deleted.
       *[other] { $n } records deleted.
    }
bulk-refused = Nothing was deleted: { $name } is still referred to elsewhere. Remove that reference first, then try again.
bulk-changed = { $n ->
        [one] { $n } record changed since the list was shown, so nothing was deleted. Reload the list and try again.
       *[other] { $n } records changed since the list was shown, so nothing was deleted. Reload the list and try again.
    }
admin-range = { $from }–{ $to } of { $total }
list-filiation = Filiation
admin-previous = Previous
admin-next = Next
admin-saved = Saved as version { $version } — { $summary }
admin-not-saved = Not saved
admin-created = Created
admin-not-created = Not created
admin-deleted = Deleted
admin-not-deleted = Not deleted — nothing was changed
admin-what-changed = what changed
admin-field = Field
admin-from = From
admin-to = To
admin-version = version { $version }

## Accounts

accounts-title = Accounts
accounts-lede = Stored in { $path }, at mode 600, beside the archive and never inside it. An archive gets copied, mailed and published; password hashes travelling inside it would make every copy of the family tree a copy of the sign-in details.
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
accounts-no-registration = Visitors cannot open accounts by themselves; registration is set above. For most families an administrator who knows everyone is enough.
accounts-password-hint = Leave blank and one is generated and shown once. At least { $min } characters if you set it yourself.
accounts-new-password-placeholder = new password (blank = keep)
accounts-email = Email
accounts-optional = (optional)
accounts-create = Create account
accounts-role-viewer = viewer — reads public and members records
accounts-role-contributor = contributor — also creates, edits and uploads
accounts-role-admin = admin — also manages accounts, deletes and exports
accounts-branch-hint = Limits what this account may edit to those people, their descendants and their spouses.
accounts-branch-reading = It does not limit what they may read — that is governed by each record's visibility, and the two are kept separate on purpose.
accounts-branch-placeholder = one person id per line
accounts-ids-in-bundle = Person ids in this tree
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
conflict-versions = You started from version { $expected }; the record now holds version { $current }.
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
conflict-their-version = { $who }'s version, as it currently stands
conflict-history-of = This { $kind }'s history

## Conversion

convert-title = Import a family file
convert-submit = Import
convert-result-title = Import report
convert-download = Download the archive

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
date-day-month-year = { $day } { $month ->
        [1] January
        [2] February
        [3] March
        [4] April
        [5] May
        [6] June
        [7] July
        [8] August
        [9] September
        [10] October
        [11] November
        [12] December
        *[other] { $month }
    } { $year }
date-month-year = { $month ->
        [1] January
        [2] February
        [3] March
        [4] April
        [5] May
        [6] June
        [7] July
        [8] August
        [9] September
        [10] October
        [11] November
        [12] December
        *[other] { $month }
    } { $year }
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

## More error pages

error-back-to-start = Back to the start
error-payload-missing-title = No such file
error-payload-missing-detail = The payload for that document is not in the cache.
error-payload-unopenable-detail = The payload for that document could not be opened.
error-no-such-document-detail = There is no document here with that id.
error-bad-preference-title = Not one of the choices
error-bad-preference-detail = That is not a language or theme this site offers. Nothing was changed.
error-unknown-kind-title = Unknown kind
error-unknown-kind-detail = “{ $kind }” is not a kind of record. This archive holds: { $kinds }.
error-io-title = Could not save
error-io-detail = { $error }. Nothing on disk was changed.
error-upload-too-large = That file is larger than the { $mb } MB limit. Nothing was stored, and the archive is unchanged.
error-upload-refused = The document was refused: { $reason }. The archive is unchanged.
error-back-to-person = Back to the record
error-no-such-person-to-attach = There is no person here with that id, so there is nothing to attach a document to.
error-upload-title = That upload was not stored
error-download-expired-title = That download has expired
error-download-expired-detail = An import is held for fifteen minutes, then discarded. Import the file again.
error-upload-none = No file was uploaded. Choose a file first.
error-upload-unsupported = That file is not a type this archive stores. Images, PDF, plain text, audio and video are accepted; the type is read from the file's own bytes, so renaming an executable does not get it in. SVG is refused outright, because an SVG can carry script.
error-export-unreadable-title = Could not read the exported archive
error-export-unreadable-detail = { $error }

## Tree page, continued

tree-title-suffix = tree
tree-back-to-focused = Back to the focused view
tree-show-all = Show all { $n }
tree-width-notice = This view is { $width }px wide — on a 1500px screen, { $screens ->
        [one] one screen
       *[other] { $screens } screens
    } of horizontal scrolling.
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
tree-contradicts-title = This tree contradicts itself.
tree-contradicts-detail = No arrangement of rows can satisfy that, so the relationship below was left out of the generation numbering and some rows may be drawn in the wrong place. Correct whichever of the two records is wrong.
tree-contradicts-pair = Recorded both as a couple and as parent and child:
tree-contradicts-more = { $n ->
        [one] One further contradiction is not listed.
       *[other] { $n } further contradictions are not listed.
    }
tree-no-people = There is nobody in this tree yet.
tree-no-people-cta = Import a family file, or add the first person.
tree-nobody-selected = Nobody to draw for that selection.
tree-nobody-selected-cta = Start from the default view.
tree-edge-union = A recorded union
tree-edge-parentage = A recorded parentage

## Home page

home-empty = Nothing recorded yet.
home-count = { $total ->
        [one] One record
       *[other] { $total } records
    }, held in one file the family owns.
home-browse = Browse the tree
home-convert = Import a family file
home-unnamed-family = This family tree
home-in-this-tree = What the family has recorded so far
home-showcase-title = Where this tree already says more than names and dates
home-showcase-example = See an example →
home-nothing-title = Nothing to show yet.
home-nothing-admin = This installation holds no family yet: set it up from an archive, from a GEDCOM file, or from nothing.
home-nothing-visitor = This family archive is being set up.
home-eyebrow = Family archive
home-hero-1 = Every life is a source.
home-hero-2 = Every family, a river.
view-tree-hero-1 = Every life is a leaf.
view-tree-hero-2 = Every family, a tree.
view-galaxy-hero-1 = Every life is a star.
view-galaxy-hero-2 = Every family, a galaxy.
view-summit-hero-1 = Every life is a step.
view-summit-hero-2 = Every family, an ascent.
home-cta-river = Explore the river
view-tree-cta = Explore the tree
view-galaxy-cta = Explore the galaxy
view-summit-cta = Climb the mountain
home-cta-tree = See the tree
home-scroll = Scroll to explore
home-trust-title = Confidence in each parentage
home-trust-certain = practically certain
home-trust-high = well supported
home-trust-medium = plausible
home-trust-low = hypothetical
home-trust-unrated = not yet rated
home-showcase-eyebrow = Beyond names and dates
home-cta-title = Together, let’s keep the family’s memory.
home-cta-detail = Every name added and every source checked makes the river truer for those who come after.
view-tree-cta-detail = Every name added and every source checked makes the tree truer for those who come after.
view-galaxy-cta-detail = Every name added and every source checked makes the galaxy truer for those who come after.
view-summit-cta-detail = Every name added and every source checked makes the mountain truer for those who come after.
home-cta-add = Add a person

## Showcase cards

showcase-links-title = { $n ->
        [one] One relationship beyond the family
       *[other] { $n } relationships beyond the family
    }
showcase-links-detail = Godparents, employers, witnesses and mentors, each with its own dates, source and how sure you are of it.
showcase-occupations-title = { $n ->
        [one] One job with a start and an end
       *[other] { $n } jobs with a start and an end
    }
showcase-occupations-detail = “Schoolteacher, 1948–1978” keeps its length, and is drawn as a bar across the years rather than as a single dated line.
showcase-uncertain-title = { $n ->
        [one] One date left as uncertain as it was given
       *[other] { $n } dates left as uncertain as they were given
    }
showcase-uncertain-detail = Circa, before, after and between stay four different statements. A date the source could not pin down is never shown as though it had been.
showcase-preserved-title = { $n ->
        [one] One date kept in the words it was written in
       *[other] { $n } dates kept in the words they were written in
    }
showcase-preserved-detail = Wording nobody could read as a date is kept exactly as written, rather than quietly discarded.
showcase-sources-title = { $n ->
        [one] One source with its reliability recorded
       *[other] { $n } sources with their reliability recorded
    }
showcase-sources-detail = { $primary ->
        [one] One primary source.
       *[other] { $primary } primary.
    } Every fact shows which evidence it rests on, and how strong that evidence is.
showcase-places-title = { $n ->
        [one] One place whose borders moved
       *[other] { $n } places whose borders moved
    }
showcase-places-detail = A town can belong to different countries at different times, and the record says which one applied when.

## Record details

record-also-recorded-as = also recorded as
record-borders-moved = Borders moved:
record-display-name = display name
record-read-as = read as
record-note = Note
record-living-yes = living
record-deceased = deceased
record-centre-tree-here = Centre the tree here
record-centre-tree-title = Move the tree to centre on this person
record-see-in-tree = See in the tree
record-see-in-tree-title = Open the grid of cards around this person
record-open-full-page = Open full page ↗
record-open-full-title = Open the standalone, shareable page
record-edit = Edit
record-see-in-river = See in the river
view-tree-see = See in the tree
view-galaxy-see = See in the galaxy
view-summit-see = See on the mountain
panel-empty = Select a card to see that person's full record here.
person-see-in-tree = See this person in the tree
person-visibility-inline = visibility:
person-age-at-death = died at { $n }
person-age-now = { $n } years old
person-born-in = born in { $place }
person-died-in = died in { $place }
person-children-count = { $n ->
        [one] one child
       *[other] { $n } children
    }
person-generations-below = { $n ->
        [one] one generation below
       *[other] { $n } generations below
    }
person-portrait-of = Photograph of { $name }
person-no-portrait = No photograph recorded

## Operation results

result-diagnostics = Diagnostics
result-diagnostics-note = Every diagnostic the library returned, including warnings that did not block the operation. None are filtered out.
result-no-diagnostics = The library returned no diagnostics.
result-continue = Continue
result-dashboard = Dashboard
person-sections-label = Sections on this page

## Vocabulary the structured editors offer

name-part-nasab = nasab (lineage)
name-part-laqab = laqab (epithet)
name-part-kunya = kunya (teknonym)
name-part-nisbah = nisbah (origin)
name-part-alias = alias
name-part-religious_name = religious name
name-part-pen_name = pen name
name-type-pen_name = pen name
gender-U = Unrecorded

## Record sections, detail

record-notes-title = Notes on this record:
record-name = Name
record-type = Type
record-cause = Cause:
record-as = as
record-partner-not-recorded = Partner not recorded
record-union-from = From
record-union-at = at
record-union-until = until
record-occupation-from = from
record-occupation-until = until
record-source-reliability = Reliability
record-source-supports = Supports
record-photographs = Photographs
record-documents = Documents
record-file = File
record-status = Status
record-size = Size
record-absent-document = Named by this person but not held here.
record-no-file = no file
record-attach-document = Attach a document
record-upload = Upload
record-upload-help = Up to { $mb } MB per file. Attachments are held beside the tree and written back into the archive when you export, so a photograph travels with the family it belongs to. The kind of file is read from its own contents rather than its name: images, PDF, plain text, audio and video are accepted. SVG is refused, because an SVG can carry a script.
record-upload-help-short = Up to { $mb } MB. SVG is refused.
record-verbatim-note = Kept exactly as the record stated it, because no converter could interpret it.
record-file-to-attach = File to attach
record-document-type = Document type
record-caption = Caption
record-history-entry-meta = — { $at }
record-history-entry-version = , { $version }

## Entity kinds
#
# Plural forms are per-kind messages rather than "{ $kind }s", which produced
# "familys" in English and would be meaningless in most other languages.

kind-person = person
kind-family = family
kind-event = event
kind-link = link
kind-occupation = occupation
kind-source = source
kind-place = place
kind-document = document

kind-person-plural = { $n ->
        [one] person
       *[other] persons
    }
kind-family-plural = { $n ->
        [one] family
       *[other] families
    }
kind-event-plural = { $n ->
        [one] event
       *[other] events
    }
kind-link-plural = { $n ->
        [one] link
       *[other] links
    }
kind-occupation-plural = { $n ->
        [one] occupation
       *[other] occupations
    }
kind-source-plural = { $n ->
        [one] source
       *[other] sources
    }
kind-place-plural = { $n ->
        [one] place
       *[other] places
    }
kind-document-plural = { $n ->
        [one] document
       *[other] documents
    }

## Listings

list-matching = { $total ->
        [one] One match
       *[other] { $total } matching
    }, { $per_page } per page.
list-filter-placeholder = Filter by name or id
list-filter = Filter
list-clear = Clear
list-summary = Summary
list-id = Id
list-actions = Actions
list-nothing = Nothing here.
list-nothing-matching = Nothing here matching “{ $q }”.
list-delete-confirm = Delete this { $kind }? Choose what happens to entities that reference it:
list-policy-reject = Reject
list-policy-reject-detail = — refuse if anything still references it. Nothing is lost.
list-policy-cascade = Cascade
list-policy-cascade-detail = — delete it and physically remove every reference to it.
list-policy-orphan = Orphan
list-policy-orphan-detail = — delete it but keep the referring records, with the link nulled.

## Completeness

completeness-dates-title = Dates, by the shape they actually have
completeness-no-dates = No dates recorded yet.
completeness-dates-note = A date somebody pinned to a day and a date somebody could only place in a decade are different claims, and both are kept as they were given. Text that could not be read as a date at all is preserved word for word rather than dropped.
completeness-shape-exact = exact
completeness-shape-exact-note = a full calendar day
completeness-shape-approximate = approximate
completeness-shape-approximate-note = circa, or a year/decade only
completeness-shape-ranged = ranged
completeness-shape-ranged-note = before, after, or between
completeness-shape-preserved = preserved
completeness-shape-preserved-note = unparsable text, kept verbatim
completeness-shape-unknown = unknown
completeness-shape-unknown-note = recorded as not known

## Conversion page

convert-page-title = Import a family file
convert-lede = Bring an existing tree across from a GEDCOM file — the export most genealogy programs produce. Nothing is stored here, and the tree this site already shows is left exactly as it was.
convert-file-label = Family file (.ged)
convert-file-hint = Up to { $mb } MB. A tree of 767 people is about 320 KB.
convert-confidence-label = How sure these facts are, to begin with
convert-confidence-hint = The file being imported does not say how certain anyone was, so every fact needs a starting point. Set it low for a tree assembled quickly, higher for one worked from documents. The honest reading of this number is “imported, and not checked by anyone since” — you can raise or lower any fact afterwards, one at a time.
convert-lang-label = Language of the place names
convert-lang-hint = A tag such as en, fr or pl.

## Conversion result

convert-failed = The import did not go through
convert-try-another = Try another file
convert-converted = Imported { $filename }
convert-result-lede = { $total ->
        [one] One record
       *[other] { $total } records
    }, { $size } KB. Everything came in at a certainty of { $confidence }, with place names read as { $lang }. The tree this site shows was not touched.
convert-produced = What came across
convert-skipped-title = { $n ->
        [one] One entry that could not be read
       *[other] { $n } entries that could not be read
    }
convert-skipped-note = These entries held nothing that could be brought across.
convert-other-diagnostics = { $n ->
        [one] One other thing worth knowing
       *[other] { $n } other things worth knowing
    }
convert-clean = Nothing was left behind — every entry in the file came across.
convert-download-title = Download
convert-download-named = Download { $name }
convert-download-note = Kept here for fifteen minutes and then discarded, so download it now.
convert-another = Import another file
admin-history-on = on
admin-history-meta = — { $kind }, { $at }
admin-validation-counts = { $errors ->
        [one] One error
       *[other] { $errors } errors
    }, { $warnings ->
        [one] one warning
       *[other] { $warnings } warnings
    }, { $infos ->
        [one] one note
       *[other] { $infos } notes
    }.
admin-warnings-never-block = Warnings never block — they are information, not gates.
admin-validator-clean = The validator reported nothing.
record-occupations-help-undated = A job is recorded with a start and an end, so several can be compared along one timeline. This archive has the job titles but no dates for them — usual after an import, since most family files have nowhere to keep them — so there is no scale to draw yet.
record-occupations-help-axis = An occupation is a state with a duration, not an event on a single date. All spans share one axis, { $from }–{ $to }.
admin-value-not-set = not set
admin-validation-report = Validation report
admin-dedup-complete = Deduplication complete
admin-dedup-refused = Deduplication refused
record-birth-order = birth order
record-start-not-recorded = start not recorded
record-end-not-recorded = end not recorded
record-document-no-file = The document is recorded here, but the file itself is not held
panel-selected-person = Selected person

## Tree bands

tree-band-generation = Generation { $g }
tree-band-people = { $n ->
        [one] one person
       *[other] { $n } people
    }
tree-band-unplaced = Unplaced
tree-band-unplaced-note = { $n ->
        [one] one person in no family — shown rather than omitted
       *[other] { $n } people in no family — shown rather than omitted
    }

## Controlled vocabulary
#
# These are the specification's own enum values, rendered for a reader. They
# are interface, not data: the bundle stores `given_name`, and "given name" is
# this application saying that value out loud. An unrecognised value falls
# through to itself with its underscores opened up, so a bundle using a term
# this build has never heard of still renders something true.

gender-M = Male
gender-F = Female
gender-NB = Non-binary
gender-unrecorded = Unrecorded

name-part-given_name = given name
name-part-family_name = family name
name-part-patronymic = patronymic
name-part-matronymic = matronymic
name-part-middle_name = middle name
name-part-nickname = nickname
name-part-prefix = prefix
name-part-suffix = suffix
name-part-particle = particle
name-part-part = part

name-type-primary = primary
name-type-other = other
name-type-alias = alias
name-type-birth = birth
name-type-married = married
name-type-religious = religious
name-type-transliteration = transliteration
name-type-nickname = nickname

## Showcase notes on a record

note-links = { $n ->
        [one] a relationship outside the family, with its own dates and sources
       *[other] { $n } relationships outside the family, with their own dates and sources
    }
note-occupations = { $n ->
        [one] a job recorded with a start and an end
       *[other] { $n } jobs recorded with a start and an end
    }
note-birth-imprecise = a birth date the source could not pin down, shown as recorded
note-death-imprecise = a death date the source could not pin down, shown as recorded
note-names = { $n ->
        [one] one recorded name
       *[other] { $n } recorded names
    }
note-transliteration = a name in its own script beside its Latin transliteration
note-witnessed = { $n ->
        [one] an event they witnessed rather than owned
       *[other] { $n } events they witnessed rather than owned
    }

visibility-public = public
visibility-members = members
visibility-contributors = contributors
visibility-private = private

## Admin list summaries
#
# A row in these listings has to be told apart from its neighbours at a
# glance. A family entity usually has no `name` of its own — the converter
# does not invent one — so its label is derived from the people in it. Each
# label is one whole message rather than fragments joined in code, because
# word order and the placement of a count are a translator's decision.

family-label-couple = { $children ->
        [0] { $a } & { $b }
        [one] { $a } & { $b } — one child
       *[other] { $a } & { $b } — { $children } children
    }
family-label-half = { $children ->
        [0] { $a } & { $unknown }
        [one] { $a } & { $unknown } — one child
       *[other] { $a } & { $unknown } — { $children } children
    }
# Naming the eldest child rather than saying "children of [unknown]": the
# point of the label is to tell one row from another, and every childless-
# parent family would otherwise carry the same words.
family-label-children = { $others ->
        [0] { $first } — parents not recorded
        [one] { $first } and one sibling — parents not recorded
       *[other] { $first } and { $others } siblings — parents not recorded
    }
family-label-empty = Family with nobody recorded

event-label = { $category } — { $who }, { $date }
event-label-nobody = { $category } — { $date }
event-two-people = { $a } & { $b }
event-more-people = { $a } & { $b } and { $others ->
        [one] one other
       *[other] { $others } others
    }

link-label = { $label }: { $from } → { $to }
occupation-label = { $who } — { $title }
source-label = { $title } — { $reliability }
source-label-plain = { $title }
document-label = { $filename } — { $type }
document-label-untitled = Untitled { $type }
list-unnamed = Unnamed { $kind }

## Specification vocabularies used in listings
#
# Open vocabularies: a value with no message here renders as itself with its
# underscores opened, so a bundle using a term this build has not seen still
# reads as something true.

event-category-birth = Birth
event-category-death = Death
event-category-marriage = Marriage
event-category-divorce = Divorce
event-category-baptism = Baptism
event-category-burial = Burial
event-category-immigration = Immigration
event-category-emigration = Emigration
event-category-census = Census
event-category-residence = Residence
event-category-military = Military service
event-category-education = Education
event-category-other = Event

reliability-primary = primary source
reliability-secondary = secondary source
reliability-tertiary = tertiary source
reliability-recollection = recollection
reliability-derivative = derivative work
reliability-authored = authored work
reliability-oral = oral tradition
reliability-unknown = reliability unknown

document-type-photo = photograph
document-type-certificate = certificate
document-type-letter = letter
document-type-record = record
document-type-newspaper = newspaper clipping
document-type-other = document

## What this record could say more about
#
# This readout used to be an argument about a file format: "these rows are
# empty because GEDCOM cannot express them", with links into a specification.
# For a family it is more useful as a to-do list — here is where the record is
# thin, and here is why filling it in is worth the trouble.

completeness-title = Where this tree could say more
completeness-intro = What is recorded and what is still blank.
completeness-import-title = What the import brought over
completeness-import-intro = Counted from the file you just uploaded. A blank row is something the original file did not record — it is not something the import lost.

completeness-headline-full = Every kind of detail below is recorded somewhere in this tree.
completeness-headline-empty = { $total ->
        [one] The one kind of detail below is not recorded anywhere yet.
       *[other] None of the { $total } kinds of detail below are recorded yet.
    } Each is somewhere the record could say more.
completeness-headline-partial = { $carried ->
        [one] One kind of detail below is recorded
       *[other] { $carried } kinds of detail below are recorded
    }; { $empty ->
        [one] one is still blank
       *[other] { $empty } are still blank
    }.

completeness-metric-confidence = How sure each fact is
completeness-metric-confidence-none = None of the { $slots } facts here says how sure it is. A date somebody read off a certificate and one somebody guessed look the same until they do.
completeness-metric-confidence-uniform = { $with } of { $slots } facts carry a score and every one is the same number ({ $modal }). That is what a bulk import leaves behind: a placeholder nobody has revisited. None has been judged individually yet.
completeness-metric-confidence-some = { $with } of { $slots } facts carry a score. { $modal_count } share one value ({ $modal }); { $assessed } differ from it and so have been looked at one at a time.
completeness-metric-confidence-many = { $with } of { $slots } facts carry a score, { $assessed } of them differing from the commonest value ({ $modal }) across { $distinct } distinct levels. This tree records real, varying uncertainty.

completeness-metric-parentage = How sure each parent–child link is
completeness-metric-parentage-none = No parentage here says how sure it is. Adoptions, disputed lines and reconstructions from a single mention are exactly where a family needs to record doubt — and the tree draws a less certain link as a fainter line.
completeness-metric-parentage-some = { $n ->
        [one] One parentage carries its own score
       *[other] { $n } parentages carry their own score
    }, so a speculative line is visibly weaker than a documented one.

completeness-metric-links = Relationships beyond blood and marriage
completeness-metric-links-none = Godparents, employers, witnesses, mentors, guardians. None are recorded yet. Each can carry its own dates, its source and how sure you are.
completeness-metric-links-some = { $n ->
        [one] One recorded, with its own dates, source and how sure you are.
       *[other] { $n } recorded, each with its own dates, source and how sure you are.
    }

completeness-metric-occupations = Work recorded with a start and an end
completeness-metric-occupations-none = No occupations recorded. A trade held for thirty years says more about a life than a single dated entry.
completeness-metric-occupations-undated = { $total ->
        [one] One occupation is recorded, without dates
       *[other] { $total } occupations are recorded, without dates
    }. Add a start and an end and they can be compared side by side on one timeline.
completeness-metric-occupations-some = { $span } of { $total } have a start or an end, so they can be compared side by side on one timeline.

completeness-metric-sources = Sources graded for how reliable they are
completeness-metric-sources-none = No sources recorded. Naming where a fact came from is what lets a relative check it later — or disagree with it and say why.
completeness-metric-sources-some = { $graded } of { $total } say how strong they are, so a claim resting on a birth certificate is visibly not the same as one resting on a recollection.

completeness-what-is-recorded = What the record can say
completeness-in-this-tree = In this tree
completeness-not-yet = not yet recorded

## Roles a participant holds in an event
#
# Open, like the other vocabularies: an unrecognised value renders as itself
# with its underscores opened rather than as a message id.

role-spouse = spouse
role-spouse_1 = first spouse
role-spouse_2 = second spouse
role-subject = subject
role-participant = participant
role-witness = witness
role-officiant = officiant
role-informant = informant
role-godparent = godparent

phys-no-source = no source
phys-col-date = When
phys-col-source = Source
phys-col-confidence = Confidence
phys-col-note = Note
phys-field-height-cm = Height
phys-field-weight-kg = Weight
phys-field-eye-colour = Eye colour
phys-field-hair-colour = Hair colour
phys-field-build = Build
phys-field-handedness = Handedness
phys-field-features = Distinguishing features
phys-field-military = Military service
phys-field-languages = Languages spoken
phys-field-blood-group = Blood group
phys-field-conditions = Known conditions
phys-field-operations = Operations and injuries
phys-field-cause-of-death = Cause of death
phys-field-religion = Religion or affiliations
phys-field-health-notes = Notes
admin-export-health-note = The plain export leaves out every sensitive class — health and belief, biometrics, genomic data and criminal records — and the behavioural profile of anyone living, so a file sent to a relative carries none of them. Tick what one particular file should carry; the archive itself records which classes were left out.
avatar-picker-title = Choose a picture
avatar-choose-link = Choose picture
avatar-choose = Which picture stands for this person
avatar-mode-auto = Let the software pick
avatar-mode-auto-note = The first portrait, or failing that the first image linked to this record.
avatar-mode-none = Show initials instead
avatar-mode-none-note = For a record whose only images are documents rather than faces.
avatar-focal-hint = Click a picture to choose it, and click again on the part that should stay in frame — an avatar is square and most scans are not.
avatar-no-images = No images are linked to this record yet.
avatar-upload-title = Upload a picture and use it
avatar-upload-button = Upload and use as picture
avatar-not-available-title = That picture is not available
avatar-not-available-detail = The chosen file is not linked to this person, or is not one you may read.
ai-badge = AI
ai-portraits-title = Portraits reworked by AI
ai-portraits-band = Retouched
ai-portraits-upload = Upload AI-reworked pictures
ai-portraits-manage = Manage the AI pictures
ai-portraits-intro = Pictures an AI made from this person's photographs: what they might have looked like. They appear beside the face on the record, marked “AI”, and only members who are signed in see them. They are never the person's picture, never among their files, never evidence.
ai-portraits-current = On the record
ai-portraits-add = Add pictures — room for { $n }
ai-portraits-files = Pictures (PNG, JPEG, WebP or GIF), several at once
ai-portraits-generator = Made with
ai-portraits-from = Made from
ai-portraits-from-none = Not said
ai-portraits-remove = Remove
ai-portraits-alt = Portrait of { $name }, reworked by an AI
ai-portraits-not-evidence = Made by an AI from the photographs: not pictures of the time.
ai-portraits-full = The record has its three AI pictures. Remove one to add another.
ai-portraits-too-many = A record holds three AI pictures at most. Remove one before adding another.
ai-portraits-images-only = An AI portrait has to be a picture: PNG, JPEG, WebP or GIF.
ai-portraits-stale = The record changed while this page was open. Nothing was saved: open it again and repeat.
ai-portraits-back = Back to the AI pictures

# The value of a recorded change this reader may not see. It sits in a
# table cell, so it is a phrase rather than a sentence.
record-history-withheld = withheld from you

## A death nobody wrote down
#
# GEDCOM cannot say "died, date unknown", so a converter marks those
# people living. These three say that the page worked it out rather than
# read it, which is a distinction this product does not blur.

record-life-status = Status
record-presumed-deceased = presumed deceased
record-presumed-short = presumed
record-presumed-why = No death is recorded and the birth is more than { $years } years ago, so this record cannot be right. The bundle is unchanged: this is what the page infers, not what the source says.

## The identity editor

identity-editor-title = Names and identity
identity-primary-name = The name shown everywhere
identity-primary-help = What the tree card, the heading and every list use. The other names below are what a source called this person at some other time.
identity-display = Name
identity-display-latin = In Latin script
identity-culture = Language
identity-direction = Direction
identity-direction-ltr = left to right
identity-direction-rtl = right to left
identity-direction-auto = from the text
identity-components = Parts of the name
identity-components-help = Which part is the given name and which the family name, in the order they are written. A record with no parts still renders: the parts are what a search can match against.
identity-part = Part
identity-value = Text
identity-other-names = Other names
identity-other-help = A married name, a religious name, a name a later record used. Each carries when it was in use and which source says so.
identity-name-type = Kind of name
identity-valid-from = In use from
identity-valid-until = In use until
identity-about = About the person
identity-living-help = This is the flag the source set. The page separately presumes a death when a birth is too long ago, and that presumption never changes this box or the bundle.
identity-error-no-display = A record needs a name to be shown by. Nothing was saved.
editor-blank-to-remove = Clear the name to remove this entry.
# A BCP 47 tag, shown as an example in the language field.
identity-edit-link = Edit names and identity

## Union types, statuses and date precision, said out loud

union-type-marriage = married
union-type-civil_union = civil union
union-type-cohabitation = cohabited
union-type-religious_only = religious union
union-type-polygamous = polygamous
union-type-unknown = not recorded
union-role-spouse = spouse
union-role-partner = partner
union-role-husband = husband
union-role-wife = wife
union-status-active = ongoing
union-status-ended_by_death = ended by a death
union-status-ended_by_divorce = ended by divorce
union-status-ended_by_separation = ended by separation
union-status-annulled = annulled
union-status-unknown = not recorded
union-status-ended = ended
union-status-ended-by = ended by { $reason }
union-reason-death_of_spouse = the death of a spouse
precision-exact = to the day
precision-year = to the year
precision-month = to the month
precision-decade = to the decade
precision-century = to the century
precision-unknown = not known
record-precision = Precision
record-approximate = Approximate
record-place = Place

## The relationships editor

family-editor-title = Family and relationships
family-unions = Unions
family-no-unions = No union is recorded for this person.
family-union-legend = Union { $n }
family-writes-family = Saving this changes the family record #{ $id }, which both people share. The other person's page changes with it.
family-partners = Partners
family-partner = Partner
family-role = Role
family-children = Children
family-children-help = Birth order is the record's own claim about who came first. Left blank, nothing is stated: a number taken from the row position would be a fact nobody wrote down.
family-child = Child
family-birth-order = Birth order
family-the-union = The union itself
family-type = Kind of union
family-status = How it stands
family-started = It started
family-ended = It ended
family-leave = Take this person out of this union
family-open-entity = Open the family record
family-new-union = A new union
family-new-union-help = This creates a new family record with this person in it. A partner is optional: a parent the record names with no partner at all is a union of one.
family-create-union = Create the union
family-parents = Parents
family-no-parents = This person is not recorded as a child of any family.
family-child-of = A child of this family
family-detach-child = Take this person out of this family
family-attach-parents = Attach to parents
family-attach-help = Choose the family this person is a child of. This adds them to that family's record, so it appears on the parents' pages too.
family-the-family = The family
family-attach = Attach
family-error-last-partner = A union needs at least one person in it. Delete the family record instead, which asks what to do about everything that refers to it.
family-error-no-family = No family was chosen. Nothing was saved.
family-error-already-child = This person is already a child of that family.
pick-error-empty = No person was named. Nothing was saved.
pick-error-not-found = No person of that name is in this archive. Nothing was saved.
pick-error-ambiguous = More than one person answers to that. Pick one from the list so the record says which. Nothing was saved.

## Links and occupations

links-editor-title = Links
links-editor-help = The relationships that are not family: a godparent, an employer, a witness, a regiment. Each is a record of its own naming two people, so editing one here changes what the other record shows.
links-none = No link is recorded for this person.
links-new = A new link
links-create = Create the link
links-remove = Remove this link
links-other-end = The other end
links-label = What it is
links-label-reverse = Read the other way
links-category = Category
links-bidirectional = Reads the same both ways
links-from = From
links-until = Until
links-reversed = This link was drawn from the other record. Editing it here changes the same entity.
link-error-no-label = A link needs to say what it is. Nothing was saved.
occupations-editor-title = Occupations
occupations-editor-help = An occupation is a period with a start and an end, not a job title. Each carries its own dates and its own source.
occupations-none = No occupation is recorded for this person.
occupations-new = A new occupation
occupations-create = Create the occupation
occupations-remove = Remove this occupation
occupations-title = What they did
occupations-employer = For whom
occupations-employer-place = Where they were
occupations-from = From
occupations-until = Until
occupation-error-no-title = An occupation needs to say what somebody did. Nothing was saved.
link-category-spiritual = spiritual
link-category-professional = professional
link-category-social = social
link-category-legal = legal
link-category-medical = medical
link-category-educational = educational
link-category-conflict = conflict
link-category-other = other
links-edit-link = Edit links
occupations-edit-link = Edit occupations
family-edit-link = Edit family and relationships

## Events and documents

events-editor-title = Events
events-editor-help = An event names several people at once — a marriage, a baptism, a census — so each one is a record of its own and appears on every page it names.
events-none = No event names this person.
events-new = A new event
events-new-help = This person is added as its subject if you name nobody else. An event with nobody in it is just a date.
events-create = Create the event
events-remove = Remove this event
events-category = What happened
events-subcategory = More precisely
events-description = Description
events-participants = Who was there
events-participants-help = Saving this changes the event record, which every other person named in it shows too.
events-who = Who
event-error-no-category = An event has to say what happened. Nothing was saved.
documents-editor-title = Documents
documents-editor-help = Which files this record points at, and what each is to it. Clearing a row detaches the file: the document and its bytes stay in the archive.
documents-attached = Attached to this record
documents-upload = Upload a file
documents-upload-help = Up to { $mb } MB. The file is stored in the archive and attached to this record.
documents-caption = Caption
documents-edit-link = Attach and detach documents
events-edit-link = Edit events

## Presentation styles: density, never colour

prefs-style = Density
prefs-style-help = How much room the page takes. Separate from the theme, which is only about colour, so you can have any of these with any of those.
style-comfortable = Comfortable
style-comfortable-note = the default, with room to read
style-compact = Compact
style-compact-note = more of a record per screen, for working through several
style-paper = Paper
style-paper-note = a serif face and rules instead of cards, for reading once or printing

## Sensitive classes

admin-export-choose = Include in this export
scope-health = Health and belief
scope-biometrics = Biometrics
scope-genomics = Genomic data
scope-legal = Criminal records
scope-behaviour = Behavioural profiles of living people
admin-export-with-chosen = Export with what is ticked

## Profile

pg-identity = Identity and civil status
pg-identity-intro = Who the record says the person was, and what the civil registers wrote down.
pg-morphology = Morphology
pg-morphology-intro = The body as it was measured and described.
pg-biometrics = Biometrics
pg-biometrics-intro = The voice, the hands and the senses, and the templates that identify a person.
pg-health = Health
pg-health-intro = Conditions, treatment, measurements and results.
pg-genomics = Genomics
pg-genomics-intro = DNA tests, haplogroups, variants and other molecular results.
pg-death = Death
pg-death-intro = How, when and where a life ended, and what happened to the body.
pg-residence = Residence and nationality
pg-residence-intro = Where the person lived, which states claimed them, and the languages they spoke.
pg-education = Education and work
pg-education-intro = Schooling, qualifications, income and property.
pg-military = Military and honours
pg-military-intro = Service, ranks, units and distinctions.
pg-legal = Legal
pg-legal-intro = Criminal proceedings and their outcomes.
pg-belief = Belief and affiliation
pg-belief-intro = Religion, rites, convictions and memberships.
pg-personality = Personality and behaviour
pg-personality-intro = Temperament, habits and pastimes, as sources describe them.
pg-relationships = Relationships
pg-relationships-intro = Parents, partners, children and the other people in a life.
pg-digital-legacy = Digital legacy
pg-digital-legacy-intro = Scans, models, recordings and archives that stand for a person.
pa-identity-titles = Titles
pa-identity-sex-at-birth = Sex at birth
pa-identity-gender-identity = Gender identity
pa-birth-time = Time of birth
pa-birth-coordinates = Place of birth, as coordinates
pa-civil-status-birth-certificate-number = Birth certificate number
pa-civil-status-register-entries = Civil register entries
pa-civil-status-marginal-annotations = Marginal annotations
pa-morphology-height = Height
pa-morphology-weight = Weight
pa-morphology-bmi = Body mass index
pa-morphology-body-composition = Body composition
pa-morphology-build = Build
pa-morphology-eye-colour = Eye colour
pa-morphology-eye-shape = Eye shape
pa-morphology-eye-spacing = Eye spacing
pa-morphology-hair-colour = Natural hair colour
pa-morphology-hair-texture = Hair texture
pa-morphology-hairline = Hairline
pa-morphology-facial-hair = Facial hair
pa-morphology-body-hair = Body hair
pa-morphology-skin-tone = Skin tone (Fitzpatrick)
pa-morphology-skin-undertone = Skin undertone
pa-morphology-freckles = Freckles
pa-morphology-pigmentation = Pigmentation marks
pa-morphology-scars = Scars
pa-morphology-tattoos = Tattoos
pa-morphology-moles = Moles
pa-morphology-facial-asymmetries = Facial asymmetries
pa-morphology-face-shape = Face shape
pa-morphology-nose-shape = Nose shape
pa-morphology-ear-shape = Ear shape
pa-morphology-lip-shape = Lip shape
pa-morphology-dentition = Dentition
pa-morphology-malocclusion = Malocclusion (Angle class)
pa-morphology-posture = Posture
pa-morphology-gait = Gait
pa-morphology-distinguishing-features = Distinguishing features
pa-biometrics-fingerprints = Fingerprints
pa-biometrics-retinal-print = Retinal print
pa-biometrics-voice-signature = Voiceprint
pa-biometrics-voice-frequency = Fundamental voice frequency
pa-biometrics-vocal-timbre = Vocal timbre
pa-biometrics-spoken-accent = Accent
pa-biometrics-speech-rate = Speech rate
pa-biometrics-verbal-tics = Verbal tics
pa-biometrics-frequent-vocabulary = Frequent vocabulary
pa-biometrics-speech-register = Register of speech
pa-biometrics-motor-tics = Motor tics
pa-biometrics-handedness = Handedness
pa-biometrics-hearing = Hearing
pa-biometrics-visual-acuity = Visual acuity
pa-biometrics-optical-correction = Optical correction
pa-health-blood-group = Blood group (ABO)
pa-health-rhesus = Rhesus (RhD)
pa-health-blood-pressure = Blood pressure
pa-health-resting-heart-rate = Resting heart rate
pa-health-respiratory-capacity = Respiratory capacity
pa-health-conditions = Conditions
pa-health-surgeries = Surgical history
pa-health-injuries = Injuries
pa-health-deformities = Deformities
pa-health-amputations = Amputations
pa-health-prostheses = Prostheses
pa-health-implants = Implants
pa-health-devices = Implanted devices
pa-health-medications = Medications
pa-health-allergies = Allergies
pa-health-vaccinations = Vaccinations
pa-health-serology = Serology
pa-health-lab-results = Laboratory results
pa-health-deficiencies = Deficiencies
pa-health-sleep-disorders = Sleep disorders
pa-health-mental-health-assessments = Mental health assessments
pa-genomics-autosomal-mapping = Autosomal DNA test
pa-genomics-y-haplogroup = Y-DNA haplogroup
pa-genomics-mt-haplogroup = Mitochondrial haplogroup
pa-genomics-whole-genome-sequencing = Whole-genome sequencing
pa-genomics-risk-variants = Risk variants
pa-genomics-hereditary-conditions = Hereditary conditions
pa-genomics-predispositions = Predispositions
pa-genomics-epigenetic-markers = Epigenetic markers
pa-genomics-epigenetic-age = Epigenetic age
pa-genomics-gut-microbiome = Gut microbiome
pa-genomics-skin-microbiome = Skin microbiome
pa-genomics-toxicological-sensitivities = Drug and toxin sensitivities
pa-death-time = Time of death
pa-death-coordinates = Place of death, as coordinates
pa-death-causes = Causes of death
pa-death-contributing-factors = Contributing factors
pa-death-autopsy = Autopsy
pa-death-disposition = Disposition of the body
pa-death-grave = Grave
pa-residence-addresses = Addresses
pa-residence-nationality-of-origin = Nationality of origin
pa-residence-acquired-nationalities = Acquired nationalities
pa-residence-mother-tongue = Mother tongue
pa-residence-spoken-languages = Languages spoken
pa-education-level = Level of education
pa-education-diplomas = Diplomas and degrees
pa-education-institutions = Schools and institutions
pa-education-income = Income
pa-education-real-estate = Real estate
pa-military-distinctions = Distinctions
pa-military-citations = Citations
pa-military-ranks = Ranks
pa-military-units = Units
pa-military-service-numbers = Service numbers
pa-legal-criminal-record = Criminal record
pa-belief-religions = Religion
pa-belief-sacraments = Sacraments and rites
pa-belief-beliefs = Beliefs
pa-belief-political-leanings = Political leanings
pa-belief-memberships = Memberships
pa-personality-big-five = Big Five scores
pa-personality-mbti = MBTI type
pa-personality-introversion-extraversion = Introversion and extraversion
pa-personality-stress-tolerance = Stress tolerance
pa-personality-decision-style = Decision-making style
pa-personality-interests = Interests
pa-personality-hobbies = Hobbies
pa-personality-sports = Sports
pa-personality-dietary-habits = Diet
pa-personality-dependencies = Dependencies
pa-digital-legacy-body-models = Body models
pa-digital-legacy-skin-textures = Skin textures
pa-digital-legacy-rigs = Skeletal rigs
pa-digital-legacy-voice-corpora = Voice recordings for synthesis
pa-digital-legacy-text-corpora = Writings for a language model
pa-digital-legacy-digital-traces = Digital traces
pa-digital-legacy-carbon-footprint = Carbon footprint
pa-digital-legacy-behaviour-models = Behaviour models
pf-identity-titles-text = Title as written
pf-identity-titles-kind = Kind of title
pf-civil-status-marginal-annotations-text = Annotation
pf-morphology-pigmentation-kind = Kind of mark
pf-biometrics-spoken-accent-description = How it is described
pf-biometrics-optical-correction-kind = Correction
pf-health-amputations-level = Level of amputation
pf-health-prostheses-kind = Prosthesis
pf-health-implants-kind = Implant
pf-health-devices-kind = Device
pf-health-allergies-type = Allergy type
pf-health-vaccinations-status = Vaccination status
pf-health-sleep-disorders-category = Category of disorder
pf-death-autopsy-kind = Autopsy
pf-education-institutions-name = Name of the institution
pf-military-distinctions-name = Name of the distinction
pf-military-distinctions-kind = Kind of distinction
pf-military-citations-text = Citation
pf-military-ranks-category = Rank category
pf-belief-political-leanings-position = Position on the left–right axis
pf-belief-memberships-kind = Kind of organisation
pf-digital-legacy-carbon-footprint-method = How it was estimated
pf-age-years = Age in years
pf-agreeableness = Agreeableness
pf-allergen = Allergen
pf-amount = Amount
pf-analyte = Analyte
pf-artefact-type = Kind of artefact
pf-autoimmune = Autoimmune
pf-body-region = Body region
pf-bone-percent = Bone
pf-carrier-status = Carrier status
pf-cause = Cause
pf-chronic = Chronic
pf-clock = Clock
pf-condition = Condition
pf-conferred-by = Conferred by
pf-congenital = Congenital
pf-conscientiousness = Conscientiousness
pf-consent = Consent
pf-coordinates = Coordinates
pf-corrected = With correction
pf-country = Country
pf-court = Court
pf-coverage = Coverage
pf-currency = Currency
pf-decimal = Acuity (decimal)
pf-denomination = Denomination
pf-derived-from-id = Derived from
pf-description = Description
pf-details = Details
pf-diagnosis = Diagnosis
pf-diameter-mm = Diameter
pf-diastolic = Diastolic
pf-diet = Diet
pf-document-id = Document
pf-dose = Dose
pf-ear = Ear
pf-entry-number = Entry number
pf-extraversion = Extraversion
pf-eye = Eye
pf-fat-percent = Fat
pf-fev1-fvc-ratio = FEV1/FVC ratio
pf-fev1-litres = FEV1
pf-file-format = File format
pf-findings = Findings
pf-flag = Flag
pf-format = Format
pf-fracture = Fracture
pf-fvc-litres = FVC
pf-gene = Gene
pf-generator = Made with
pf-grade = Grade
pf-iccs-section = Offence section (ICCS)
pf-icd10-chapter = ICD-10 chapter
pf-indication = Indication
pf-inheritance = Inheritance
pf-inscription = Inscription
pf-institution = Institution
pf-instrument = Instrument
pf-isced-level = ISCED level
pf-jurisdiction = Jurisdiction
pf-language = Language
pf-lat = Latitude
pf-level = Level
pf-lines = Address
pf-location = Location
pf-lon = Longitude
pf-major = Major haplogroup
pf-marker = Marker
pf-metaboliser-status = Metaboliser status
pf-method = Method
pf-mode = How acquired
pf-muscle-percent = Muscle
pf-neuroticism = Neuroticism
pf-number = Number
pf-nutrient = Nutrient
pf-offence = Offence
pf-office = Office
pf-openness = Openness
pf-organisation = Organisation
pf-outcome = Outcome
pf-pace = Pace of ageing
pf-page = Page
pf-panel = Panel
pf-party = Party
pf-pathogen = Pathogen
pf-pattern = Pattern of use
pf-percentile = Percentile
pf-period = Pay period
pf-place-id = Place
pf-plot = Plot
pf-polygenic-score = Polygenic score
pf-postal-code = Postal code
pf-precision = Precision
pf-prescription = Prescription
pf-proficiency = Proficiency
pf-provider = Provider
pf-quintile = Income quintile
pf-rank = Rank
pf-rank-text = Rank as written
pf-reaction = Reaction
pf-reference-build = Reference genome
pf-reference-high = Reference range, upper
pf-reference-low = Reference range, lower
pf-register-type = Kind of entry
pf-result = Result
pf-role = Role
pf-sacrament = Sacrament or rite
pf-score = Score
pf-sentence = Sentence
pf-sequence = Place in the chain
pf-service = Service
pf-severity = Severity
pf-shannon-diversity = Shannon diversity
pf-shape = Shape
pf-significance = Clinical significance
pf-snp-count = SNPs tested
pf-sport = Sport
pf-subclade = Subclade
pf-substance = Substance
pf-summary = Summary
pf-systolic = Systolic
pf-tenure = Tenure
pf-test = Test
pf-threshold-db = Hearing threshold
pf-title = Title
pf-tonnes-co2e-per-year = Emissions
pf-tradition = Tradition
pf-tree-version = Tree version
pf-unit = Unit
pf-use = Use
pf-variant = Variant
pf-volume = Volume
pf-zygosity = Zygosity
pu-cm = { $n } cm
pu-kg = { $n } kg
pu-kg-m2 = { $n } kg/m²
pu-percent = { $n } %
pu-mm = { $n } mm
pu-hz = { $n } Hz
pu-words-min = { $n } words/min
pu-db-hl = { $n } dB HL
pu-mmhg = { $n } mmHg
pu-bpm = { $n } bpm
pu-litres = { $n } L
pu-coverage = { $n }×
pu-years = { $n } years
pu-t-co2e-yr = { $n } t CO₂e a year
pv-sensitive-class-health = Health and belief
pv-sensitive-class-biometrics = Biometrics
pv-sensitive-class-genomics = Genomic data
pv-sensitive-class-legal = Criminal records
pv-laterality-left = Left
pv-laterality-right = Right
pv-laterality-both = Both
pv-body-region-head = Head
pv-body-region-face = Face
pv-body-region-neck = Neck
pv-body-region-left-shoulder = Left shoulder
pv-body-region-right-shoulder = Right shoulder
pv-body-region-left-arm = Left arm
pv-body-region-right-arm = Right arm
pv-body-region-left-hand = Left hand
pv-body-region-right-hand = Right hand
pv-body-region-chest = Chest
pv-body-region-abdomen = Abdomen
pv-body-region-upper-back = Upper back
pv-body-region-lower-back = Lower back
pv-body-region-pelvis = Pelvis and hips
pv-body-region-left-leg = Left leg
pv-body-region-right-leg = Right leg
pv-body-region-left-foot = Left foot
pv-body-region-right-foot = Right foot
pv-body-region-internal = Internal
pv-body-region-whole-body = Whole body
pv-body-region-other = Other region
pv-artefact-type-mesh = Mesh
pv-artefact-type-point-cloud = Point cloud
pv-artefact-type-skin-texture-map = Skin texture map
pv-artefact-type-skeletal-rig = Skeletal rig
pv-artefact-type-voice-corpus = Voice corpus
pv-artefact-type-text-corpus = Text corpus
pv-artefact-type-trace-archive = Archive of online activity
pv-artefact-type-behaviour-model = Behaviour model
pv-artefact-type-fingerprint-card = Fingerprint card
pv-artefact-type-fingerprint-template = Fingerprint template
pv-artefact-type-retinal-image = Retinal image
pv-artefact-type-voiceprint = Voiceprint
pv-consent-given = Given
pv-consent-given-by-estate = Given by the estate
pv-consent-refused = Refused
pv-consent-withdrawn = Withdrawn
pv-consent-not-asked = Not asked
pv-consent-unknown = Not known
pv-sex-at-birth-female = Female
pv-sex-at-birth-male = Male
pv-sex-at-birth-intersex = Intersex
pv-sex-at-birth-undetermined = Undetermined
pv-sex-at-birth-unknown = Not known
pv-gender-identity-woman = Woman
pv-gender-identity-man = Man
pv-gender-identity-non-binary = Non-binary
pv-gender-identity-other = Other
pv-gender-identity-undisclosed = Not disclosed
pv-gender-identity-unknown = Not known
pv-title-kind-nobility = Nobility
pv-title-kind-academic = Academic
pv-title-kind-professional = Professional
pv-title-kind-religious = Religious
pv-title-kind-military = Military
pv-title-kind-civic = Civic or honorary
pv-title-kind-courtesy = Courtesy
pv-title-kind-other = Other
pv-register-type-birth = Birth
pv-register-type-baptism = Baptism
pv-register-type-marriage = Marriage
pv-register-type-death = Death
pv-register-type-burial = Burial
pv-register-type-divorce = Divorce
pv-register-type-recognition = Recognition of a child
pv-register-type-legitimation = Legitimation
pv-register-type-adoption = Adoption
pv-register-type-name-change = Change of name
pv-register-type-other = Other
pv-build-slight = Slight
pv-build-slim = Slim
pv-build-average = Average
pv-build-sturdy = Sturdy
pv-build-stout = Stout
pv-build-heavy = Heavy
pv-eye-colour-light-blue = Light blue
pv-eye-colour-blue = Blue
pv-eye-colour-dark-blue = Dark blue
pv-eye-colour-grey = Grey
pv-eye-colour-blue-grey = Blue-grey
pv-eye-colour-green = Green
pv-eye-colour-grey-green = Grey-green
pv-eye-colour-hazel = Hazel
pv-eye-colour-amber = Amber
pv-eye-colour-light-brown = Light brown
pv-eye-colour-brown = Brown
pv-eye-colour-dark-brown = Dark brown
pv-eye-colour-black = Black
pv-eye-colour-mixed = Mixed
pv-eye-colour-other = Other
pv-eye-shape-almond = Almond
pv-eye-shape-round = Round
pv-eye-shape-hooded = Hooded
pv-eye-shape-monolid = Monolid
pv-eye-shape-deep-set = Deep-set
pv-eye-shape-protruding = Protruding
pv-eye-shape-upturned = Upturned
pv-eye-shape-downturned = Downturned
pv-eye-shape-other = Other
pv-eye-spacing-close-set = Close-set
pv-eye-spacing-average = Average
pv-eye-spacing-wide-set = Wide-set
pv-hair-colour-black = Black
pv-hair-colour-dark-brown = Dark brown
pv-hair-colour-brown = Brown
pv-hair-colour-light-brown = Light brown
pv-hair-colour-auburn = Auburn
pv-hair-colour-red = Red
pv-hair-colour-strawberry-blond = Strawberry blond
pv-hair-colour-dark-blond = Dark blond
pv-hair-colour-blond = Blond
pv-hair-colour-light-blond = Light blond
pv-hair-colour-grey = Grey
pv-hair-colour-white = White
pv-hair-colour-none = No hair
pv-hair-colour-other = Other
pv-hair-texture-straight = Straight
pv-hair-texture-wavy = Wavy
pv-hair-texture-curly = Curly
pv-hair-texture-coily = Coily
pv-hair-texture-other = Other
pv-hairline-straight = Straight
pv-hairline-rounded = Rounded
pv-hairline-widows-peak = Widow’s peak
pv-hairline-m-shaped = M-shaped
pv-hairline-bell-shaped = Bell-shaped
pv-hairline-uneven = Uneven
pv-hairline-receding = Receding
pv-hairline-bald = Bald
pv-facial-hair-none = None
pv-facial-hair-stubble = Stubble
pv-facial-hair-moustache = Moustache
pv-facial-hair-goatee = Goatee
pv-facial-hair-full-beard = Full beard
pv-facial-hair-sideburns = Sideburns
pv-facial-hair-other = Other
pv-body-hair-none = None
pv-body-hair-sparse = Sparse
pv-body-hair-moderate = Moderate
pv-body-hair-dense = Dense
pv-skin-tone-type-i = Type I — always burns, never tans
pv-skin-tone-type-ii = Type II — usually burns, tans little
pv-skin-tone-type-iii = Type III — sometimes burns, tans evenly
pv-skin-tone-type-iv = Type IV — rarely burns, tans well
pv-skin-tone-type-v = Type V — very rarely burns
pv-skin-tone-type-vi = Type VI — never burns
pv-skin-undertone-cool = Cool
pv-skin-undertone-neutral = Neutral
pv-skin-undertone-warm = Warm
pv-skin-undertone-olive = Olive
pv-freckles-none = None
pv-freckles-few = Few
pv-freckles-moderate = Moderate
pv-freckles-many = Many
pv-pigmentation-mark-birthmark = Birthmark
pv-pigmentation-mark-port-wine-stain = Port-wine stain
pv-pigmentation-mark-cafe-au-lait-spot = Café-au-lait spot
pv-pigmentation-mark-depigmented-patch = Depigmented patch
pv-pigmentation-mark-hyperpigmented-patch = Hyperpigmented patch
pv-pigmentation-mark-other = Other
pv-mole-shape-round = Round
pv-mole-shape-oval = Oval
pv-mole-shape-irregular = Irregular
pv-mole-shape-other = Other
pv-face-shape-oval = Oval
pv-face-shape-round = Round
pv-face-shape-square = Square
pv-face-shape-oblong = Oblong
pv-face-shape-heart = Heart-shaped
pv-face-shape-diamond = Diamond
pv-face-shape-triangular = Triangular
pv-nose-shape-straight = Straight
pv-nose-shape-aquiline = Aquiline
pv-nose-shape-snub = Snub
pv-nose-shape-upturned = Upturned
pv-nose-shape-flat = Flat
pv-nose-shape-broad = Broad
pv-nose-shape-bulbous = Bulbous
pv-nose-shape-crooked = Crooked
pv-nose-shape-other = Other
pv-ear-shape-free-lobe = Free lobes
pv-ear-shape-attached-lobe = Attached lobes
pv-ear-shape-protruding = Protruding
pv-ear-shape-close-set = Close to the head
pv-ear-shape-pointed = Pointed
pv-ear-shape-other = Other
pv-lip-shape-thin = Thin
pv-lip-shape-medium = Medium
pv-lip-shape-full = Full
pv-lip-shape-bow-shaped = Bow-shaped
pv-lip-shape-wide = Wide
pv-lip-shape-downturned = Downturned
pv-lip-shape-other = Other
pv-dentition-primary = Primary teeth
pv-dentition-mixed = Mixed
pv-dentition-permanent-complete = Permanent, complete
pv-dentition-permanent-partial-loss = Permanent, some lost
pv-dentition-edentulous = No natural teeth
pv-dentition-partial-denture = Partial denture
pv-dentition-full-denture = Full denture
pv-dentition-implants = Dental implants
pv-malocclusion-normal = Normal occlusion
pv-malocclusion-class-i = Class I
pv-malocclusion-class-ii-division-1 = Class II, division 1
pv-malocclusion-class-ii-division-2 = Class II, division 2
pv-malocclusion-class-iii = Class III
pv-posture-ideal = Ideal
pv-posture-kyphotic-lordotic = Kyphosis-lordosis
pv-posture-flat-back = Flat back
pv-posture-sway-back = Sway back
pv-posture-stooped = Stooped
pv-posture-scoliotic = Scoliotic
pv-posture-other = Other
pv-gait-brisk = Brisk
pv-gait-average = Average
pv-gait-slow = Slow
pv-gait-shuffling = Shuffling
pv-gait-limping = Limping
pv-gait-waddling = Waddling
pv-gait-unsteady = Unsteady
pv-gait-stiff = Stiff
pv-gait-other = Other
pv-vocal-timbre-bright = Bright
pv-vocal-timbre-dark = Dark
pv-vocal-timbre-warm = Warm
pv-vocal-timbre-breathy = Breathy
pv-vocal-timbre-nasal = Nasal
pv-vocal-timbre-hoarse = Hoarse
pv-vocal-timbre-resonant = Resonant
pv-vocal-timbre-thin = Thin
pv-vocal-timbre-other = Other
pv-speech-register-frozen = Frozen
pv-speech-register-formal = Formal
pv-speech-register-consultative = Consultative
pv-speech-register-casual = Casual
pv-speech-register-intimate = Intimate
pv-handedness-left = Left-handed
pv-handedness-right = Right-handed
pv-handedness-ambidextrous = Ambidextrous
pv-handedness-mixed = Mixed
pv-handedness-unknown = Not known
pv-hearing-grade-normal = Normal
pv-hearing-grade-mild = Mild
pv-hearing-grade-moderate = Moderate
pv-hearing-grade-moderately-severe = Moderately severe
pv-hearing-grade-severe = Severe
pv-hearing-grade-profound = Profound
pv-hearing-grade-complete = Complete
pv-optical-correction-none = None
pv-optical-correction-glasses = Glasses
pv-optical-correction-contact-lenses = Contact lenses
pv-optical-correction-glasses-and-contact-lenses = Glasses and contact lenses
pv-optical-correction-refractive-surgery = Refractive surgery
pv-optical-correction-intraocular-lens = Intraocular lens
pv-optical-correction-other = Other
pv-rhesus-positive = RhD positive
pv-rhesus-negative = RhD negative
pv-rhesus-weak-d = Weak D
pv-rhesus-unknown = Not known
pv-icd10-chapter-infectious-parasitic = I Infectious and parasitic diseases
pv-icd10-chapter-neoplasms = II Neoplasms
pv-icd10-chapter-blood-immune = III Blood and immune disorders
pv-icd10-chapter-endocrine-metabolic = IV Endocrine, nutritional and metabolic
pv-icd10-chapter-mental-behavioural = V Mental and behavioural disorders
pv-icd10-chapter-nervous-system = VI Nervous system
pv-icd10-chapter-eye-adnexa = VII Eye and adnexa
pv-icd10-chapter-ear-mastoid = VIII Ear and mastoid process
pv-icd10-chapter-circulatory = IX Circulatory system
pv-icd10-chapter-respiratory = X Respiratory system
pv-icd10-chapter-digestive = XI Digestive system
pv-icd10-chapter-skin = XII Skin and subcutaneous tissue
pv-icd10-chapter-musculoskeletal = XIII Musculoskeletal system
pv-icd10-chapter-genitourinary = XIV Genitourinary system
pv-icd10-chapter-pregnancy-childbirth = XV Pregnancy and childbirth
pv-icd10-chapter-perinatal = XVI Perinatal conditions
pv-icd10-chapter-congenital = XVII Congenital malformations
pv-icd10-chapter-ill-defined = XVIII Symptoms and ill-defined causes
pv-icd10-chapter-injury-poisoning = XIX Injury and poisoning
pv-icd10-chapter-external-causes = XX External causes
pv-icd10-chapter-health-factors = XXI Factors influencing health
pv-icd10-chapter-special-purposes = XXII Codes for special purposes
pv-diagnosis-status-diagnosed = Diagnosed
pv-diagnosis-status-suspected = Suspected
pv-diagnosis-status-self-reported = Self-reported
pv-diagnosis-status-unknown = Not known
pv-prosthesis-kind-limb = Limb
pv-prosthesis-kind-joint = Joint replacement
pv-prosthesis-kind-ocular = Ocular
pv-prosthesis-kind-dental = Dental
pv-prosthesis-kind-auditory = Auditory
pv-prosthesis-kind-breast = Breast
pv-prosthesis-kind-other = Other
pv-implant-kind-orthopaedic = Orthopaedic
pv-implant-kind-dental = Dental
pv-implant-kind-cochlear = Cochlear
pv-implant-kind-breast = Breast
pv-implant-kind-intraocular-lens = Intraocular lens
pv-implant-kind-contraceptive = Contraceptive
pv-implant-kind-cosmetic = Cosmetic
pv-implant-kind-other = Other
pv-device-kind-pacemaker = Pacemaker
pv-device-kind-implantable-defibrillator = Implantable defibrillator
pv-device-kind-cardiac-resynchronisation = Cardiac resynchronisation device
pv-device-kind-ventricular-assist = Ventricular assist device
pv-device-kind-neurostimulator = Neurostimulator
pv-device-kind-insulin-pump = Insulin pump
pv-device-kind-drug-port = Drug delivery port
pv-device-kind-shunt = Shunt
pv-device-kind-stent = Stent
pv-device-kind-other = Other
pv-allergy-type-drug = Drug
pv-allergy-type-food = Food
pv-allergy-type-environmental = Environmental
pv-allergy-type-insect-venom = Insect venom
pv-allergy-type-latex = Latex
pv-allergy-type-other = Other
pv-allergy-severity-mild = Mild
pv-allergy-severity-moderate = Moderate
pv-allergy-severity-severe = Severe
pv-allergy-severity-anaphylactic = Anaphylactic
pv-allergy-severity-unknown = Not known
pv-pathogen-diphtheria = Diphtheria
pv-pathogen-tetanus = Tetanus
pv-pathogen-pertussis = Whooping cough
pv-pathogen-poliomyelitis = Poliomyelitis
pv-pathogen-measles = Measles
pv-pathogen-mumps = Mumps
pv-pathogen-rubella = Rubella
pv-pathogen-varicella = Chickenpox
pv-pathogen-smallpox = Smallpox
pv-pathogen-tuberculosis = Tuberculosis
pv-pathogen-hepatitis-a = Hepatitis A
pv-pathogen-hepatitis-b = Hepatitis B
pv-pathogen-hepatitis-c = Hepatitis C
pv-pathogen-haemophilus-influenzae-b = Haemophilus influenzae type b
pv-pathogen-pneumococcal = Pneumococcal disease
pv-pathogen-meningococcal = Meningococcal disease
pv-pathogen-human-papillomavirus = Human papillomavirus
pv-pathogen-influenza = Influenza
pv-pathogen-covid-19 = COVID-19
pv-pathogen-rotavirus = Rotavirus
pv-pathogen-yellow-fever = Yellow fever
pv-pathogen-typhoid = Typhoid
pv-pathogen-cholera = Cholera
pv-pathogen-rabies = Rabies
pv-pathogen-japanese-encephalitis = Japanese encephalitis
pv-pathogen-tick-borne-encephalitis = Tick-borne encephalitis
pv-pathogen-hiv = HIV
pv-pathogen-syphilis = Syphilis
pv-pathogen-toxoplasmosis = Toxoplasmosis
pv-pathogen-cytomegalovirus = Cytomegalovirus
pv-pathogen-epstein-barr = Epstein–Barr virus
pv-pathogen-other = Other
pv-vaccination-status-vaccinated = Vaccinated
pv-vaccination-status-partially-vaccinated = Partially vaccinated
pv-vaccination-status-unvaccinated = Not vaccinated
pv-vaccination-status-contraindicated = Contraindicated
pv-vaccination-status-unknown = Not known
pv-serology-result-positive = Positive
pv-serology-result-negative = Negative
pv-serology-result-equivocal = Equivocal
pv-serology-result-unknown = Not known
pv-lab-panel-basic-metabolic = Basic metabolic panel
pv-lab-panel-lipid = Lipid panel
pv-lab-panel-liver = Liver panel
pv-lab-panel-renal = Renal panel
pv-lab-panel-glycated-haemoglobin = Glycated haemoglobin
pv-lab-panel-iron = Iron studies
pv-lab-analyte-sodium = Sodium
pv-lab-analyte-potassium = Potassium
pv-lab-analyte-chloride = Chloride
pv-lab-analyte-bicarbonate = Bicarbonate
pv-lab-analyte-urea = Urea
pv-lab-analyte-creatinine = Creatinine
pv-lab-analyte-glucose = Glucose
pv-lab-analyte-calcium = Calcium
pv-lab-analyte-total-cholesterol = Total cholesterol
pv-lab-analyte-ldl-cholesterol = LDL cholesterol
pv-lab-analyte-hdl-cholesterol = HDL cholesterol
pv-lab-analyte-triglycerides = Triglycerides
pv-lab-analyte-non-hdl-cholesterol = Non-HDL cholesterol
pv-lab-analyte-alt = Alanine aminotransferase (ALT)
pv-lab-analyte-ast = Aspartate aminotransferase (AST)
pv-lab-analyte-alp = Alkaline phosphatase (ALP)
pv-lab-analyte-ggt = Gamma-glutamyl transferase (GGT)
pv-lab-analyte-total-bilirubin = Total bilirubin
pv-lab-analyte-direct-bilirubin = Direct bilirubin
pv-lab-analyte-albumin = Albumin
pv-lab-analyte-total-protein = Total protein
pv-lab-analyte-egfr = Estimated GFR
pv-lab-analyte-uric-acid = Uric acid
pv-lab-analyte-phosphate = Phosphate
pv-lab-analyte-urine-albumin-creatinine-ratio = Urine albumin-to-creatinine ratio
pv-lab-analyte-hba1c = HbA1c
pv-lab-analyte-serum-iron = Serum iron
pv-lab-analyte-ferritin = Ferritin
pv-lab-analyte-transferrin = Transferrin
pv-lab-analyte-transferrin-saturation = Transferrin saturation
pv-lab-analyte-tibc = Total iron-binding capacity
pv-lab-flag-low = Low
pv-lab-flag-normal = Normal
pv-lab-flag-high = High
pv-lab-flag-critical-low = Critically low
pv-lab-flag-critical-high = Critically high
pv-nutrient-vitamin-a = Vitamin A
pv-nutrient-thiamine = Thiamine (B1)
pv-nutrient-riboflavin = Riboflavin (B2)
pv-nutrient-niacin = Niacin (B3)
pv-nutrient-vitamin-b6 = Vitamin B6
pv-nutrient-folate = Folate (B9)
pv-nutrient-vitamin-b12 = Vitamin B12
pv-nutrient-vitamin-c = Vitamin C
pv-nutrient-vitamin-d = Vitamin D
pv-nutrient-vitamin-e = Vitamin E
pv-nutrient-vitamin-k = Vitamin K
pv-nutrient-iron = Iron
pv-nutrient-zinc = Zinc
pv-nutrient-magnesium = Magnesium
pv-nutrient-calcium = Calcium
pv-nutrient-iodine = Iodine
pv-nutrient-selenium = Selenium
pv-nutrient-copper = Copper
pv-nutrient-potassium = Potassium
pv-nutrient-phosphorus = Phosphorus
pv-nutrient-other = Other
pv-sleep-disorder-insomnia = Insomnia
pv-sleep-disorder-sleep-related-breathing = Sleep-related breathing disorder
pv-sleep-disorder-central-hypersomnolence = Central hypersomnolence
pv-sleep-disorder-circadian-rhythm = Circadian rhythm disorder
pv-sleep-disorder-parasomnia = Parasomnia
pv-sleep-disorder-sleep-related-movement = Sleep-related movement disorder
pv-sleep-disorder-other = Other
pv-assessment-instrument-phq-9 = PHQ-9
pv-assessment-instrument-gad-7 = GAD-7
pv-assessment-instrument-bdi-ii = BDI-II
pv-assessment-instrument-hads = HADS
pv-assessment-instrument-k10 = K10
pv-assessment-instrument-gds-15 = GDS-15
pv-assessment-instrument-mmse = MMSE
pv-assessment-instrument-moca = MoCA
pv-assessment-instrument-audit = AUDIT
pv-assessment-instrument-clinical-interview = Clinical interview
pv-assessment-instrument-other = Other
pv-assessment-severity-none-minimal = None or minimal
pv-assessment-severity-mild = Mild
pv-assessment-severity-moderate = Moderate
pv-assessment-severity-moderately-severe = Moderately severe
pv-assessment-severity-severe = Severe
pv-reference-build-grch36 = GRCh36 (hg18)
pv-reference-build-grch37 = GRCh37 (hg19)
pv-reference-build-grch38 = GRCh38 (hg38)
pv-reference-build-t2t-chm13 = T2T-CHM13
pv-genomic-file-format-raw-microarray = Raw microarray data
pv-genomic-file-format-fastq = FASTQ
pv-genomic-file-format-bam = BAM
pv-genomic-file-format-cram = CRAM
pv-genomic-file-format-vcf = VCF
pv-genomic-file-format-gvcf = gVCF
pv-genomic-file-format-other = Other
pv-zygosity-heterozygous = Heterozygous
pv-zygosity-homozygous = Homozygous
pv-zygosity-hemizygous = Hemizygous
pv-zygosity-compound-heterozygous = Compound heterozygous
pv-clinical-significance-pathogenic = Pathogenic
pv-clinical-significance-likely-pathogenic = Likely pathogenic
pv-clinical-significance-uncertain-significance = Uncertain significance
pv-clinical-significance-likely-benign = Likely benign
pv-clinical-significance-benign = Benign
pv-inheritance-pattern-autosomal-dominant = Autosomal dominant
pv-inheritance-pattern-autosomal-recessive = Autosomal recessive
pv-inheritance-pattern-x-linked-dominant = X-linked dominant
pv-inheritance-pattern-x-linked-recessive = X-linked recessive
pv-inheritance-pattern-y-linked = Y-linked
pv-inheritance-pattern-mitochondrial = Mitochondrial
pv-inheritance-pattern-multifactorial = Multifactorial
pv-inheritance-pattern-unknown = Not known
pv-carrier-status-affected = Affected
pv-carrier-status-carrier = Carrier
pv-carrier-status-not-carrier = Not a carrier
pv-carrier-status-unknown = Not known
pv-epigenetic-clock-horvath = Horvath
pv-epigenetic-clock-hannum = Hannum
pv-epigenetic-clock-phenoage = PhenoAge
pv-epigenetic-clock-grimage = GrimAge
pv-epigenetic-clock-dunedinpace = DunedinPACE
pv-epigenetic-clock-other = Other
pv-metaboliser-status-poor = Poor metaboliser
pv-metaboliser-status-intermediate = Intermediate metaboliser
pv-metaboliser-status-normal = Normal metaboliser
pv-metaboliser-status-rapid = Rapid metaboliser
pv-metaboliser-status-ultrarapid = Ultrarapid metaboliser
pv-autopsy-not-performed = Not performed
pv-autopsy-clinical = Clinical
pv-autopsy-forensic = Forensic
pv-autopsy-external-examination = External examination only
pv-autopsy-unknown = Not known
pv-disposition-burial = Burial
pv-disposition-cremation = Cremation
pv-disposition-entombment = Entombment
pv-disposition-burial-at-sea = Burial at sea
pv-disposition-natural-burial = Natural burial
pv-disposition-body-donation = Donated to science
pv-disposition-other = Other
pv-disposition-unknown = Not known
pv-address-use-principal = Principal residence
pv-address-use-secondary = Secondary residence
pv-address-use-temporary = Temporary residence
pv-address-use-postal = Postal address
pv-address-use-other = Other
pv-nationality-mode-descent = By descent
pv-nationality-mode-birth-in-territory = By birth in the territory
pv-nationality-mode-naturalisation = By naturalisation
pv-nationality-mode-marriage = By marriage
pv-nationality-mode-registration = By registration
pv-nationality-mode-restoration = By restoration
pv-nationality-mode-state-succession = By change of sovereignty
pv-nationality-mode-other = Other
pv-language-proficiency-a1 = A1 Beginner
pv-language-proficiency-a2 = A2 Elementary
pv-language-proficiency-b1 = B1 Intermediate
pv-language-proficiency-b2 = B2 Upper intermediate
pv-language-proficiency-c1 = C1 Advanced
pv-language-proficiency-c2 = C2 Mastery
pv-language-proficiency-native = First language
pv-isced-level-isced-0 = 0 Early childhood
pv-isced-level-isced-1 = 1 Primary
pv-isced-level-isced-2 = 2 Lower secondary
pv-isced-level-isced-3 = 3 Upper secondary
pv-isced-level-isced-4 = 4 Post-secondary non-tertiary
pv-isced-level-isced-5 = 5 Short-cycle tertiary
pv-isced-level-isced-6 = 6 Bachelor’s or equivalent
pv-isced-level-isced-7 = 7 Master’s or equivalent
pv-isced-level-isced-8 = 8 Doctoral or equivalent
pv-income-quintile-q1 = Lowest fifth
pv-income-quintile-q2 = Second fifth
pv-income-quintile-q3 = Middle fifth
pv-income-quintile-q4 = Fourth fifth
pv-income-quintile-q5 = Highest fifth
pv-pay-period-hourly = Per hour
pv-pay-period-daily = Per day
pv-pay-period-weekly = Per week
pv-pay-period-monthly = Per month
pv-pay-period-annual = Per year
pv-tenure-owned = Owned
pv-tenure-co-owned = Owned jointly
pv-tenure-leasehold = Leasehold
pv-tenure-rented = Rented
pv-tenure-usufruct = Usufruct
pv-tenure-other = Other
pv-distinction-kind-order = Order
pv-distinction-kind-decoration = Decoration
pv-distinction-kind-medal = Medal
pv-distinction-kind-title = Honorific title
pv-distinction-kind-other = Other
pv-military-service-army = Army
pv-military-service-navy = Navy
pv-military-service-air-force = Air force
pv-military-service-marines = Marines
pv-military-service-gendarmerie = Gendarmerie
pv-military-service-border-guard = Border guard
pv-military-service-national-guard = National guard
pv-military-service-other = Other
pv-rank-category-enlisted = Enlisted
pv-rank-category-non-commissioned = Non-commissioned officer
pv-rank-category-warrant = Warrant officer
pv-rank-category-officer-cadet = Officer cadet
pv-rank-category-junior-officer = Junior officer
pv-rank-category-senior-officer = Senior officer
pv-rank-category-general-officer = General officer
pv-iccs-section-acts-leading-to-death = 01 Acts leading to death
pv-iccs-section-acts-causing-harm = 02 Acts causing harm
pv-iccs-section-sexual-acts = 03 Injurious acts of a sexual nature
pv-iccs-section-property-with-violence = 04 Against property, with violence
pv-iccs-section-property-only = 05 Against property only
pv-iccs-section-controlled-substances = 06 Controlled substances
pv-iccs-section-fraud-deception-corruption = 07 Fraud, deception or corruption
pv-iccs-section-public-order-and-state = 08 Against public order and the State
pv-iccs-section-public-safety-and-security = 09 Against public safety and security
pv-iccs-section-natural-environment = 10 Against the natural environment
pv-iccs-section-other-criminal-acts = 11 Other criminal acts
pv-case-outcome-convicted = Convicted
pv-case-outcome-acquitted = Acquitted
pv-case-outcome-dismissed = Dismissed
pv-case-outcome-conviction-quashed = Conviction quashed
pv-case-outcome-pardoned = Pardoned
pv-case-outcome-amnestied = Amnestied
pv-case-outcome-expunged = Expunged
pv-case-outcome-pending = Pending
pv-case-outcome-unknown = Not known
pv-religion-buddhism = Buddhism
pv-religion-christianity-catholic = Christianity: Catholic
pv-religion-christianity-orthodox = Christianity: Orthodox
pv-religion-christianity-protestant = Christianity: Protestant
pv-religion-christianity-other = Christianity: other
pv-religion-hinduism = Hinduism
pv-religion-islam-sunni = Islam: Sunni
pv-religion-islam-shia = Islam: Shia
pv-religion-islam-other = Islam: other
pv-religion-jainism = Jainism
pv-religion-judaism = Judaism
pv-religion-sikhism = Sikhism
pv-religion-bahai = Bahá’í Faith
pv-religion-shinto = Shinto
pv-religion-taoism = Taoism
pv-religion-zoroastrianism = Zoroastrianism
pv-religion-traditional = Traditional or folk religion
pv-religion-other = Other
pv-religion-none = No religion
pv-religion-unknown = Not known
pv-sacrament-baptism = Baptism
pv-sacrament-confirmation = Confirmation
pv-sacrament-first-communion = First communion
pv-sacrament-reconciliation = Reconciliation
pv-sacrament-anointing-of-the-sick = Anointing of the sick
pv-sacrament-holy-orders = Holy orders
pv-sacrament-matrimony = Matrimony
pv-sacrament-other-rite = Another rite
pv-political-position-far-left = Far left
pv-political-position-left = Left
pv-political-position-centre-left = Centre-left
pv-political-position-centre = Centre
pv-political-position-centre-right = Centre-right
pv-political-position-right = Right
pv-political-position-far-right = Far right
pv-political-position-apolitical = Apolitical
pv-political-position-other = Off this axis
pv-political-position-unknown = Not known
pv-membership-kind-trade-union = Trade union
pv-membership-kind-political-party = Political party
pv-membership-kind-professional-body = Professional body
pv-membership-kind-religious-order = Religious order
pv-membership-kind-religious-association = Religious association
pv-membership-kind-fraternal-order = Fraternal order
pv-membership-kind-veterans-association = Veterans’ association
pv-membership-kind-sports-club = Sports club
pv-membership-kind-cultural-association = Cultural association
pv-membership-kind-charitable-association = Charitable association
pv-membership-kind-other = Other
pv-personality-instrument-neo-pi-3 = NEO-PI-3
pv-personality-instrument-neo-ffi-3 = NEO-FFI-3
pv-personality-instrument-bfi-2 = BFI-2
pv-personality-instrument-ipip-neo-120 = IPIP-NEO-120
pv-personality-instrument-tipi = TIPI
pv-personality-instrument-hexaco-pi-r = HEXACO-PI-R
pv-personality-instrument-observer-rating = Rated by someone who knew them
pv-personality-instrument-inferred = Inferred from records
pv-personality-instrument-other = Other
pv-introversion-extraversion-strongly-introverted = Strongly introverted
pv-introversion-extraversion-introverted = Introverted
pv-introversion-extraversion-ambiverted = Ambiverted
pv-introversion-extraversion-extraverted = Extraverted
pv-introversion-extraversion-strongly-extraverted = Strongly extraverted
pv-stress-tolerance-very-low = Very low
pv-stress-tolerance-low = Low
pv-stress-tolerance-moderate = Moderate
pv-stress-tolerance-high = High
pv-stress-tolerance-very-high = Very high
pv-decision-style-rational = Rational
pv-decision-style-intuitive = Intuitive
pv-decision-style-dependent = Dependent
pv-decision-style-avoidant = Avoidant
pv-decision-style-spontaneous = Spontaneous
pv-sport-level-recreational = Recreational
pv-sport-level-amateur-competitive = Competitive amateur
pv-sport-level-semi-professional = Semi-professional
pv-sport-level-professional = Professional
pv-diet-omnivore = Omnivore
pv-diet-flexitarian = Flexitarian
pv-diet-pescatarian = Pescatarian
pv-diet-vegetarian = Vegetarian
pv-diet-vegan = Vegan
pv-diet-other = Other
pv-substance-tobacco = Tobacco and nicotine
pv-substance-alcohol = Alcohol
pv-substance-cannabis = Cannabis
pv-substance-opioids = Opioids
pv-substance-stimulants = Stimulants
pv-substance-sedatives-hypnotics = Sedatives and hypnotics
pv-substance-hallucinogens = Hallucinogens
pv-substance-inhalants = Inhalants
pv-substance-gambling = Gambling
pv-substance-gaming = Gaming
pv-substance-other = Other
pv-use-pattern-occasional-use = Occasional use
pv-use-pattern-regular-use = Regular use
pv-use-pattern-harmful-use = Harmful use
pv-use-pattern-dependence = Dependence
pv-use-pattern-in-remission = In remission
pv-lineage-biological = Biological
pv-lineage-adoptive = Adoptive
pv-lineage-foster = Foster
pv-lineage-step = Step
pv-lineage-guardianship = Guardianship
pv-lineage-unknown = Not known
pv-link-relation-godparent = Godparent
pv-link-relation-godchild = Godchild
pv-link-relation-witness = Witness
pv-link-relation-officiant = Officiant
pv-link-relation-business-partner = Business partner
pv-link-relation-employer = Employer
pv-link-relation-employee = Employee
pv-link-relation-mentor = Mentor
pv-link-relation-apprentice = Apprentice
pv-link-relation-close-friend = Close friend
pv-link-relation-neighbour = Neighbour
pv-link-relation-guardian = Guardian
pv-link-relation-ward = Ward
pv-link-relation-other = Other
pv-country-AD = Andorra
pv-country-AE = United Arab Emirates
pv-country-AF = Afghanistan
pv-country-AG = Antigua & Barbuda
pv-country-AI = Anguilla
pv-country-AL = Albania
pv-country-AM = Armenia
pv-country-AO = Angola
pv-country-AQ = Antarctica
pv-country-AR = Argentina
pv-country-AS = American Samoa
pv-country-AT = Austria
pv-country-AU = Australia
pv-country-AW = Aruba
pv-country-AX = Åland Islands
pv-country-AZ = Azerbaijan
pv-country-BA = Bosnia & Herzegovina
pv-country-BB = Barbados
pv-country-BD = Bangladesh
pv-country-BE = Belgium
pv-country-BF = Burkina Faso
pv-country-BG = Bulgaria
pv-country-BH = Bahrain
pv-country-BI = Burundi
pv-country-BJ = Benin
pv-country-BL = St. Barthélemy
pv-country-BM = Bermuda
pv-country-BN = Brunei
pv-country-BO = Bolivia
pv-country-BQ = Caribbean Netherlands
pv-country-BR = Brazil
pv-country-BS = Bahamas
pv-country-BT = Bhutan
pv-country-BV = Bouvet Island
pv-country-BW = Botswana
pv-country-BY = Belarus
pv-country-BZ = Belize
pv-country-CA = Canada
pv-country-CC = Cocos (Keeling) Islands
pv-country-CD = Congo - Kinshasa
pv-country-CF = Central African Republic
pv-country-CG = Congo - Brazzaville
pv-country-CH = Switzerland
pv-country-CI = Côte d’Ivoire
pv-country-CK = Cook Islands
pv-country-CL = Chile
pv-country-CM = Cameroon
pv-country-CN = China
pv-country-CO = Colombia
pv-country-CR = Costa Rica
pv-country-CU = Cuba
pv-country-CV = Cape Verde
pv-country-CW = Curaçao
pv-country-CX = Christmas Island
pv-country-CY = Cyprus
pv-country-CZ = Czechia
pv-country-DE = Germany
pv-country-DJ = Djibouti
pv-country-DK = Denmark
pv-country-DM = Dominica
pv-country-DO = Dominican Republic
pv-country-DZ = Algeria
pv-country-EC = Ecuador
pv-country-EE = Estonia
pv-country-EG = Egypt
pv-country-EH = Western Sahara
pv-country-ER = Eritrea
pv-country-ES = Spain
pv-country-ET = Ethiopia
pv-country-FI = Finland
pv-country-FJ = Fiji
pv-country-FK = Falkland Islands
pv-country-FM = Micronesia
pv-country-FO = Faroe Islands
pv-country-FR = France
pv-country-GA = Gabon
pv-country-GB = United Kingdom
pv-country-GD = Grenada
pv-country-GE = Georgia
pv-country-GF = French Guiana
pv-country-GG = Guernsey
pv-country-GH = Ghana
pv-country-GI = Gibraltar
pv-country-GL = Greenland
pv-country-GM = Gambia
pv-country-GN = Guinea
pv-country-GP = Guadeloupe
pv-country-GQ = Equatorial Guinea
pv-country-GR = Greece
pv-country-GS = South Georgia & South Sandwich Islands
pv-country-GT = Guatemala
pv-country-GU = Guam
pv-country-GW = Guinea-Bissau
pv-country-GY = Guyana
pv-country-HK = Hong Kong SAR China
pv-country-HM = Heard & McDonald Islands
pv-country-HN = Honduras
pv-country-HR = Croatia
pv-country-HT = Haiti
pv-country-HU = Hungary
pv-country-ID = Indonesia
pv-country-IE = Ireland
pv-country-IL = Israel
pv-country-IM = Isle of Man
pv-country-IN = India
pv-country-IO = British Indian Ocean Territory
pv-country-IQ = Iraq
pv-country-IR = Iran
pv-country-IS = Iceland
pv-country-IT = Italy
pv-country-JE = Jersey
pv-country-JM = Jamaica
pv-country-JO = Jordan
pv-country-JP = Japan
pv-country-KE = Kenya
pv-country-KG = Kyrgyzstan
pv-country-KH = Cambodia
pv-country-KI = Kiribati
pv-country-KM = Comoros
pv-country-KN = St. Kitts & Nevis
pv-country-KP = North Korea
pv-country-KR = South Korea
pv-country-KW = Kuwait
pv-country-KY = Cayman Islands
pv-country-KZ = Kazakhstan
pv-country-LA = Laos
pv-country-LB = Lebanon
pv-country-LC = St. Lucia
pv-country-LI = Liechtenstein
pv-country-LK = Sri Lanka
pv-country-LR = Liberia
pv-country-LS = Lesotho
pv-country-LT = Lithuania
pv-country-LU = Luxembourg
pv-country-LV = Latvia
pv-country-LY = Libya
pv-country-MA = Morocco
pv-country-MC = Monaco
pv-country-MD = Moldova
pv-country-ME = Montenegro
pv-country-MF = St. Martin
pv-country-MG = Madagascar
pv-country-MH = Marshall Islands
pv-country-MK = North Macedonia
pv-country-ML = Mali
pv-country-MM = Myanmar (Burma)
pv-country-MN = Mongolia
pv-country-MO = Macao SAR China
pv-country-MP = Northern Mariana Islands
pv-country-MQ = Martinique
pv-country-MR = Mauritania
pv-country-MS = Montserrat
pv-country-MT = Malta
pv-country-MU = Mauritius
pv-country-MV = Maldives
pv-country-MW = Malawi
pv-country-MX = Mexico
pv-country-MY = Malaysia
pv-country-MZ = Mozambique
pv-country-NA = Namibia
pv-country-NC = New Caledonia
pv-country-NE = Niger
pv-country-NF = Norfolk Island
pv-country-NG = Nigeria
pv-country-NI = Nicaragua
pv-country-NL = Netherlands
pv-country-NO = Norway
pv-country-NP = Nepal
pv-country-NR = Nauru
pv-country-NU = Niue
pv-country-NZ = New Zealand
pv-country-OM = Oman
pv-country-PA = Panama
pv-country-PE = Peru
pv-country-PF = French Polynesia
pv-country-PG = Papua New Guinea
pv-country-PH = Philippines
pv-country-PK = Pakistan
pv-country-PL = Poland
pv-country-PM = St. Pierre & Miquelon
pv-country-PN = Pitcairn Islands
pv-country-PR = Puerto Rico
pv-country-PS = Palestinian Territories
pv-country-PT = Portugal
pv-country-PW = Palau
pv-country-PY = Paraguay
pv-country-QA = Qatar
pv-country-RE = Réunion
pv-country-RO = Romania
pv-country-RS = Serbia
pv-country-RU = Russia
pv-country-RW = Rwanda
pv-country-SA = Saudi Arabia
pv-country-SB = Solomon Islands
pv-country-SC = Seychelles
pv-country-SD = Sudan
pv-country-SE = Sweden
pv-country-SG = Singapore
pv-country-SH = St. Helena
pv-country-SI = Slovenia
pv-country-SJ = Svalbard & Jan Mayen
pv-country-SK = Slovakia
pv-country-SL = Sierra Leone
pv-country-SM = San Marino
pv-country-SN = Senegal
pv-country-SO = Somalia
pv-country-SR = Suriname
pv-country-SS = South Sudan
pv-country-ST = São Tomé & Príncipe
pv-country-SV = El Salvador
pv-country-SX = Sint Maarten
pv-country-SY = Syria
pv-country-SZ = Eswatini
pv-country-TC = Turks & Caicos Islands
pv-country-TD = Chad
pv-country-TF = French Southern Territories
pv-country-TG = Togo
pv-country-TH = Thailand
pv-country-TJ = Tajikistan
pv-country-TK = Tokelau
pv-country-TL = Timor-Leste
pv-country-TM = Turkmenistan
pv-country-TN = Tunisia
pv-country-TO = Tonga
pv-country-TR = Turkey
pv-country-TT = Trinidad & Tobago
pv-country-TV = Tuvalu
pv-country-TW = Taiwan
pv-country-TZ = Tanzania
pv-country-UA = Ukraine
pv-country-UG = Uganda
pv-country-UM = U.S. Outlying Islands
pv-country-US = United States
pv-country-UY = Uruguay
pv-country-UZ = Uzbekistan
pv-country-VA = Vatican City
pv-country-VC = St. Vincent & Grenadines
pv-country-VE = Venezuela
pv-country-VG = British Virgin Islands
pv-country-VI = U.S. Virgin Islands
pv-country-VN = Vietnam
pv-country-VU = Vanuatu
pv-country-WF = Wallis & Futuna
pv-country-WS = Samoa
pv-country-YE = Yemen
pv-country-YT = Mayotte
pv-country-ZA = South Africa
pv-country-ZM = Zambia
pv-country-ZW = Zimbabwe
pv-country-SU = Soviet Union
pv-country-DD = East Germany
pv-country-YU = Yugoslavia
pv-country-CS = Czechoslovakia
pv-country-OT = Ottoman Empire
lang-aa = Afar
lang-ab = Abkhazian
lang-ae = Avestan
lang-af = Afrikaans
lang-ak = Akan
lang-am = Amharic
lang-an = Aragonese
lang-ar = Arabic
lang-as = Assamese
lang-av = Avaric
lang-ay = Aymara
lang-az = Azerbaijani
lang-ba = Bashkir
lang-be = Belarusian
lang-bg = Bulgarian
lang-bi = Bislama
lang-bm = Bambara
lang-bn = Bangla
lang-bo = Tibetan
lang-br = Breton
lang-bs = Bosnian
lang-ca = Catalan
lang-ce = Chechen
lang-ch = Chamorro
lang-co = Corsican
lang-cr = Cree
lang-cs = Czech
lang-cu = Church Slavic
lang-cv = Chuvash
lang-cy = Welsh
lang-da = Danish
lang-de = German
lang-dv = Divehi
lang-dz = Dzongkha
lang-ee = Ewe
lang-el = Greek
lang-en = English
lang-eo = Esperanto
lang-es = Spanish
lang-et = Estonian
lang-eu = Basque
lang-fa = Persian
lang-ff = Fulah
lang-fi = Finnish
lang-fj = Fijian
lang-fo = Faroese
lang-fr = French
lang-fy = Western Frisian
lang-ga = Irish
lang-gd = Scottish Gaelic
lang-gl = Galician
lang-gn = Guarani
lang-gu = Gujarati
lang-gv = Manx
lang-ha = Hausa
lang-he = Hebrew
lang-hi = Hindi
lang-ho = Hiri Motu
lang-hr = Croatian
lang-ht = Haitian Creole
lang-hu = Hungarian
lang-hy = Armenian
lang-hz = Herero
lang-ia = Interlingua
lang-id = Indonesian
lang-ie = Interlingue
lang-ig = Igbo
lang-ii = Sichuan Yi
lang-ik = Inupiaq
lang-io = Ido
lang-is = Icelandic
lang-it = Italian
lang-iu = Inuktitut
lang-ja = Japanese
lang-jv = Javanese
lang-ka = Georgian
lang-kg = Kongo
lang-ki = Kikuyu
lang-kj = Kuanyama
lang-kk = Kazakh
lang-kl = Kalaallisut
lang-km = Khmer
lang-kn = Kannada
lang-ko = Korean
lang-kr = Kanuri
lang-ks = Kashmiri
lang-ku = Kurdish
lang-kv = Komi
lang-kw = Cornish
lang-ky = Kyrgyz
lang-la = Latin
lang-lb = Luxembourgish
lang-lg = Ganda
lang-li = Limburgish
lang-ln = Lingala
lang-lo = Lao
lang-lt = Lithuanian
lang-lu = Luba-Katanga
lang-lv = Latvian
lang-mg = Malagasy
lang-mh = Marshallese
lang-mi = Māori
lang-mk = Macedonian
lang-ml = Malayalam
lang-mn = Mongolian
lang-mr = Marathi
lang-ms = Malay
lang-mt = Maltese
lang-my = Burmese
lang-na = Nauru
lang-nb = Norwegian Bokmål
lang-nd = North Ndebele
lang-ne = Nepali
lang-ng = Ndonga
lang-nl = Dutch
lang-nn = Norwegian Nynorsk
lang-no = Norwegian
lang-nr = South Ndebele
lang-nv = Navajo
lang-ny = Nyanja
lang-oc = Occitan
lang-oj = Ojibwa
lang-om = Oromo
lang-or = Odia
lang-os = Ossetic
lang-pa = Punjabi
lang-pi = Pali
lang-pl = Polish
lang-ps = Pashto
lang-pt = Portuguese
lang-qu = Quechua
lang-rm = Romansh
lang-rn = Rundi
lang-ro = Romanian
lang-ru = Russian
lang-rw = Kinyarwanda
lang-sa = Sanskrit
lang-sc = Sardinian
lang-sd = Sindhi
lang-se = Northern Sami
lang-sg = Sango
lang-sh = Serbo-Croatian
lang-si = Sinhala
lang-sk = Slovak
lang-sl = Slovenian
lang-sm = Samoan
lang-sn = Shona
lang-so = Somali
lang-sq = Albanian
lang-sr = Serbian
lang-ss = Swati
lang-st = Southern Sotho
lang-su = Sundanese
lang-sv = Swedish
lang-sw = Swahili
lang-ta = Tamil
lang-te = Telugu
lang-tg = Tajik
lang-th = Thai
lang-ti = Tigrinya
lang-tk = Turkmen
lang-tl = Tagalog
lang-tn = Tswana
lang-to = Tongan
lang-tr = Turkish
lang-ts = Tsonga
lang-tt = Tatar
lang-tw = Twi
lang-ty = Tahitian
lang-ug = Uyghur
lang-uk = Ukrainian
lang-ur = Urdu
lang-uz = Uzbek
lang-ve = Venda
lang-vi = Vietnamese
lang-vo = Volapük
lang-wa = Walloon
lang-wo = Wolof
lang-xh = Xhosa
lang-yi = Yiddish
lang-yo = Yoruba
lang-za = Zhuang
lang-zh = Chinese
lang-zu = Zulu
person-tab-profile = Profile
profile-groups-label = Groups of the profile
profile-group-withheld = Some of this group is withheld from you
profile-withheld = Recorded for this person and withheld from you: { $classes }.
profile-empty = Nothing is recorded in this group yet.
profile-earlier = earlier form
profile-earlier-title = Recorded by an earlier version of this application, in a field AXGF 1.1 has no place for. It is kept as it was written.
profile-other-names = { $n ->
        [one] and one other name
       *[other] and { $n } other names
    }
profile-edit-group = Edit { $group }
profile-summary-link = { $n ->
        [one] One fact in the profile
       *[other] { $n } facts in the profile
    }
profile-from = from
profile-until = until
profile-yes = Yes
profile-no = No
profile-value = Value
profile-editor-title = Profile
profile-group-filled = { $n } filled
profile-problems = Some of what was entered could not be saved. Each problem is named beside its field, and nothing was written.
profile-editor-withheld = This group also holds { $classes } for this person, which you may not read. It is not shown here, and saving this form leaves it exactly as it is.
profile-living-class-note = This person is recorded as living. What you enter in a sensitive class here is shown to administrators only.
profile-relationships-elsewhere = Parents, partners, children, godparents and witnesses are not stored on this person. They are families, links and events that name them — so each change here also changes the record of everyone else in it.
profile-documents-first = An artefact refers to a document attached to this person. Attach the file first.
profile-editor-nothing = Nothing in this group is yours to edit.
profile-new-entry = New entry
profile-provenance = Date, source and confidence
profile-from-date = True from
profile-until-date = True until
profile-remove-entry = Remove this entry
profile-add-entry = Add another entry
profile-no-such-group-title = No such group
profile-no-such-group-detail = The profile has no group by that name.
profile-error-number = A value here is a number.
profile-error-integer = A value here is a whole number.
profile-error-range = A number is outside the range this attribute allows.
profile-error-term = A value is not one of the choices listed.
profile-error-required = An entry is missing a field it needs.
profile-error-one-of = An entry needs at least one of its main fields filled in.
profile-error-confidence = Confidence runs from 0 to 1, for example 0.8.
profile-error-time = A time is written as hours and minutes, for example 05:40.
profile-error-currency = A currency is its three-letter code, for example PLN.
profile-error-language = A language is its code, for example pl or zh-Hans.
profile-error-coordinates = Coordinates need both a latitude between −90 and 90 and a longitude between −180 and 180.
profile-error-rank-country = The rank belongs to a different country from the one chosen.
record-unknown-place = [Unknown place]
record-missing-document = [Missing document]

## Interface

confidence-certain = Confidence { $percent }% — effectively certain
confidence-high = Confidence { $percent }% — well supported
confidence-medium = Confidence { $percent }% — plausible but unconfirmed
confidence-low = Confidence { $percent }% — speculative
tree-edge-union-between = { $from } and { $to } — { $confidence }
tree-edge-parentage-of = { $from }, parent of { $to } — { $confidence }
record-note-biography = Biography
record-note-birth-date-as-recorded = Birth date, as recorded
record-note-death-date-as-recorded = Death date, as recorded
record-note-event-date-as-recorded = Date of { $event }, as recorded
record-unknown-source = [Unknown source]
record-untitled-source = [Untitled source]
record-unnamed = [Unnamed]
record-untitled = [Untitled]
record-period-from = from { $date }
record-period-until = until { $date }
record-dates-unrecorded = dates unrecorded
record-link-unlabelled = linked to
record-link-reverse = { $label } (of)
record-place-worked-as = Worked as { $title }
record-place-married-to = Married { $name }
record-place-married = Married
record-source-use-name = the name “{ $name }”
record-source-use-working-as = working as { $title }
record-source-use-union-with = the union with { $name }
record-source-use-union = the union
record-lifespan-born = b. { $year }
record-lifespan-died = d. { $year }
size-bytes = { $n ->
        [one] { $n } byte
       *[other] { $n } bytes
    }
size-kb = { $n } KB
size-mb = { $n } MB
size-gb = { $n } GB
calendar-gregorian = Gregorian
calendar-julian = Julian
calendar-hebrew = Hebrew
calendar-hijri = Hijri
calendar-persian = Persian
calendar-chinese = Chinese
calendar-ethiopian = Ethiopian
calendar-japanese_era = Japanese era
calendar-republican_french = French Republican
calendar-roman = Roman
diff-summary-none = no field changed
diff-summary-one = changed { $a }
diff-summary-two = changed { $a } and { $b }
diff-summary-many = changed { $a }, { $b } and { $n ->
        [one] one more
       *[other] { $n } more
    }
diff-saved-none = no field changed
diff-saved-one = changed { $a }
diff-saved-two = changed { $a } and { $b }
diff-saved-many = changed { $a }, { $b } and { $n ->
        [one] one more
       *[other] { $n } more
    }
diff-section-other = another field
diff-section-avatar = Portrait photo
diff-section-partners = Partners
diff-section-ai = AI assistance
diff-section-birth = Birth
diff-section-civil-status = Civil status
diff-section-documents = Attached documents
diff-section-extensions = Application data
diff-section-children = Children
diff-section-union = Union
diff-section-date = Date
diff-section-participants = Participants
diff-section-from = Linked from
diff-section-to = Linked to
diff-section-valid-from = Valid from
diff-section-valid-until = Valid until
diff-section-visibility = Visibility
diff-section-employer = Employer
diff-section-title-normalized = Standardised title
diff-section-conflicts = Conflicting sources
diff-section-dna = DNA
diff-section-document = Document
diff-section-language = Language
diff-section-place = Place
diff-section-repository = Repository
diff-section-script = Script
diff-section-coordinates = Coordinates
diff-section-country-history = Country history
diff-section-identifiers = Identifiers
diff-section-names = Names
diff-section-file = File
diff-section-linked-to = Attached to
diff-section-ocr = Recognised text
diff-section-format-version = Format version
history-created = created
history-deleted = deleted
history-merged = merged into another record
history-setup = set up the archive
history-attached = attached a file
admin-raw-json-unparsed = The raw JSON did not parse ({ $error }). Nothing was saved.
admin-raw-json-dangling = The raw JSON points { $field } at { $target }, which is not in this archive. Pick it with a field above, or correct the id. Nothing was saved.
conflict-someone = Someone
conflict-unrecorded-time = a time nobody recorded
dedup-merged-persons = { $n ->
        [one] one person merged
       *[other] { $n } people merged
    }
dedup-merged-families = { $n ->
        [one] one family merged
       *[other] { $n } families merged
    }
dedup-manual-review = { $n ->
        [one] one case left for a person to review
       *[other] { $n } cases left for a person to review
    }
dedup-nothing = Nothing to report.
record-union-duplicate = One couple, more than one record.
record-union-duplicate-detail = The bundle keeps separate Family records for these two people. That is a defect in the data, not a second union.
record-union-duplicate-action = Merge duplicates
record-union-duplicate-confirm = Deduplicate the whole bundle? Every pair the library can merge is merged; the rest are reported.
dedup-pair-merged = The pair you asked about is now one record.
dedup-pair-refused = The pair you asked about was not merged: the library refused it and left it for a person to review.

validate-errors = { $n ->
        [one] one error
       *[other] { $n } errors
    }
validate-warnings = { $n ->
        [one] one warning
       *[other] { $n } warnings
    }
validate-notes = { $n ->
        [one] one note
       *[other] { $n } notes
    }
validate-nothing = Nothing to report.
list-separator = { ", " }
result-written = The archive was written to disk.
result-refused = The library refused this operation. The archive on disk is unchanged.
convert-error-no-file = No file was uploaded. Choose a .ged file first.
convert-error-file-too-large = That file is { $size } MB and the limit is { $limit } MB. Nothing was converted.
convert-error-too-large = That upload is larger than the { $limit } MB limit. Nothing was converted.
convert-error-unreadable = The upload could not be read ({ $error }). Nothing was converted.
convert-error-not-gedcom = That does not look like a GEDCOM file: a GEDCOM 5.5.1 file starts with a “0 HEAD” line. Nothing was converted.
gedcom-read-title = How the file was read
gedcom-read-as = Read as GEDCOM { $version }, in { $encoding }.
gedcom-read-as-unversioned = Read as GEDCOM (its header gives no version), in { $encoding }.
gedcom-read-declared = Its header says { $declared }; its bytes say otherwise, and were read as they are.
gedcom-read-gedzip = A GEDCOM 7 archive (.gdz): its dataset was read. Other files in it (photographs, scans), not carried across: { $n }.
gedcom-read-crs = Lines ending the old Macintosh way, mended: { $n }.
gedcom-read-voids = Pointers to @VOID@ — GEDCOM 7’s “no record” — set aside rather than read as people: { $n }.
gedcom-read-notes = Shared notes (SNOTE) read as notes: { $n }.
convert-error-no-dataset = This zip holds no gedcom.ged, and no single .ged file to read instead.
convert-error-refused = The converter refused this file. The diagnostics below say why. Nothing was kept.
convert-error-packaging = The file was converted but could not be packaged ({ $error }).
completeness-fraction = { $part } of { $whole }
event-category-adoption = Adoption
event-category-migration = Migration
event-category-naturalization = Naturalisation
event-category-incarceration = Imprisonment
event-category-name_change = Change of name
event-category-legal = Legal matter
event-category-religious = Religious event
event-category-social = Social event
event-category-historical = Historical event
precision-quarter_century = to the quarter century
source-type-birth_certificate = birth certificate
source-type-death_certificate = death certificate
source-type-marriage_certificate = marriage certificate
source-type-census = census
source-type-baptism_record = baptism record
source-type-burial_record = burial record
source-type-will = will
source-type-land_record = land record
source-type-military_record = military record
source-type-immigration_record = immigration record
source-type-naturalization = naturalisation record
source-type-passport = passport
source-type-photograph = photograph
source-type-letter = letter
source-type-diary = diary
source-type-newspaper = newspaper
source-type-oral_tradition = oral tradition
source-type-dna = DNA test
source-type-family_bible = family Bible
source-type-gravestone = gravestone
source-type-published_genealogy = published genealogy
source-type-other = other source
source-status-verified = checked against the original
source-status-unverified = not yet checked
source-status-lost = lost
source-status-known_missing = known to be missing
document-type-birth_certificate = birth certificate
document-type-death_certificate = death certificate
document-type-marriage_certificate = marriage certificate
document-type-census_page = census page
document-type-baptism_record = baptism record
document-type-military_record = military record
document-type-will = will
document-type-land_record = land record
document-type-diary = diary
document-type-newspaper_clipping = newspaper clipping
document-type-gravestone_photo = photograph of a gravestone
document-type-family_tree_drawing = drawn family tree
document-type-audio = sound recording
document-type-video = video recording
document-status-present = held here
document-status-referenced = named, held elsewhere
document-status-known_missing = known to be missing
document-status-lost = lost
document-status-unknown = whereabouts unknown
diag-unsupported_spec_version = The archive declares a version of AXGF this build cannot read.
diag-invalid_json = Something that should be JSON does not parse.
diag-invalid_bundle_structure = The archive is not laid out the way AXGF requires.
diag-schema_validation_failed = A record does not match the AXGF schema.
diag-dangling_reference = A record points at another record that is not in the archive.
diag-duplicate_entity_id = Two records share one id.
diag-duplicate_unique_ref = Two records claim the same reference that should be unique.
diag-cycle_detected = The family links go round in a circle: someone would be their own ancestor.
diag-chronology_conflict = Dates contradict each other, such as a child born before a parent.
diag-out_of_vocabulary = A value is not one of the terms its list allows.
diag-claim_inconsistent = A claim contradicts itself or another claim about the same thing.
diag-spec_version_mismatch = A record's declared AXGF version does not fit what it contains.
diag-unknown_attribute = A record carries an attribute AXGF does not define.
diag-entity_not_found = The record to change is not in the archive.
diag-entity_already_exists = A record with this id already exists.
diag-unknown_entity_kind = This is not a kind of record AXGF has.
diag-delete_blocked_by_reference = The record cannot be deleted while other records still point at it.
diag-manual_review_required = A person needs to look at this; it was not changed automatically.
diag-zip_read_error = The archive file could not be read.
diag-zip_write_error = The archive file could not be written.
diag-payloads_external = The attached files are kept outside the archive's own data.
diag-payload_source_failed = An attached file could not be read.
diag-payload_sink_failed = An attached file could not be written.
diag-gedcom_parse_error = A line of the GEDCOM file could not be understood.
diag-gedcom_unrecognized_tag = The GEDCOM file uses a tag the importer does not know, so that entry was not brought across.
diag-internal = Something went wrong inside the library.
field-person-display-name = Display name
field-person-display-name-hint = The name shown everywhere on the site.
field-person-gender = Gender
field-person-living = Living
field-person-birth-date = Birth date
field-date-value-hint = A year, a year and month, or a whole date: 1923, 1923-04 or 1923-04-12. Leave it blank when nobody knows.
field-person-birth-precision = Birth precision
field-precision-hint = How precisely the source pins this down.
field-person-birth-circa = Birth is approximate
field-circa-hint = Shown as “circa 1923” rather than as an exact claim.
field-person-birth-place = Birth place id
field-person-birth-confidence = Birth confidence
field-person-confidence-hint = How sure you are. This is what the site draws.
field-person-death-date = Death date
field-person-death-precision = Death precision
field-person-death-circa = Death is approximate
field-person-death-place = Death place id
field-person-death-confidence = Death confidence
field-person-death-cause = Cause of death
field-person-bio = Biography
field-notes = Notes
field-family-name = Family name
field-description = Description
field-family-union-type = Union type
field-family-union-status = Union status
field-family-union-confidence = Union confidence
field-family-union-confidence-hint = Sets how strongly the line between the partners is drawn on the tree.
field-family-union-start = Union start
field-family-union-end = Union end
field-family-notes-hint = Partners and children are lists — edit them in the raw JSON below, or on the person's relationships page.
field-category = Category
field-required-hint = Required.
field-event-subcategory = Subcategory
field-date = Date
field-event-date-hint = Required by the schema.
field-precision = Precision
field-circa = Approximate
field-place-id = Place id
field-confidence = Confidence
field-source-id = Source id
field-link-from-type = From type
field-link-from-id = From id
field-link-to-type = To type
field-link-to-id = To id
field-link-label = Label
field-link-label-hint = Reads forward: “godfather”, “employer”, “witness”. Required.
field-link-label-reverse = Reverse label
field-link-label-reverse-hint = How it reads from the other end: “godson”, “employee”.
field-link-bidirectional = Reads the same both ways
field-valid-from = Valid from
field-link-valid-from-hint = When the relationship started.
field-valid-until = Valid until
field-link-confidence-hint = “85% sure, per a family letter” — the thing GEDCOM cannot say.
field-note = Note
field-occupation-person-id = Person id
field-occupation-title = Occupation
field-occupation-title-hint = Required, for example Schoolteacher.
field-occupation-title-latin = Occupation (Latin script)
field-occupation-employer = Employer
field-occupation-from = From
field-occupation-from-hint = An occupation is a span. Giving both ends is what draws it as a bar.
field-occupation-until = Until
field-source-title = Title
field-source-type = Kind of source
field-source-reliability = Reliability
field-source-reliability-hint = Required. Shown as a badge beside every fact that rests on this source.
field-source-status = State of the source
field-source-repository = Repository
field-source-repository-reference = Reference in the repository
field-source-transcription = Transcription
field-place-name = Primary name
field-place-name-lang = Language of the name
field-place-name-lang-hint = A language tag, for example en, fr or pl.
field-place-type = Kind of place
field-place-region = Region
field-place-country-current = Country today
field-place-country-current-hint = The history of its borders is a list — edit it in the raw JSON below.
field-document-filename = File name
field-document-mime-type = Media type
field-document-mime-type-hint = Required, for example image/jpeg.
field-document-type = Kind of document
field-document-status = State of the file
field-document-url = Web address
field-document-caption = Caption
lang-zh-Hans = Simplified Chinese
family-lineage = Parentage
links-relation = Kind of tie
occupations-position = Post held
field-link-relation = Kind of tie
field-link-relation-hint = One of the ties AXGF 1.1 names. The label above keeps the record's own words.
field-occupation-position = Post held
field-occupation-position-hint = The post within the occupation: Headmistress, where the occupation is Teacher.
error-delete-changed-title = Changed since you looked
error-delete-changed-detail = This record was saved again after the page you deleted it from was drawn, and is now version { $version }. Nothing was deleted. Look at it as it stands before deciding again.
error-delete-changed-look = Look at it again
documents-files = Files attached here
documents-files-help = Edit a file's details, or delete the file itself. Deleting removes the document and its bytes from the archive, for everyone it is attached to; to take it off this person alone, clear its row above.
documents-edit-details = Edit details
documents-delete = Delete this file

## Charts

radar-section = Charts from the record
radar-section-help = Three readings of what this record holds, each axis from 0 to 100. Every number is worked out from the facts listed beside it, by rules written down in the application's documentation; nothing is stored, nothing is guessed, and an axis with nothing to read is left empty rather than given a middling score.
radar-physique = Physique
radar-mind = Temperament and mind
radar-vitality = Health and vitality
radar-axis-stature = Stature
radar-axis-build = Build
radar-axis-lean-mass = Lean mass
radar-axis-posture = Posture
radar-axis-gait = Gait
radar-axis-dentition = Teeth
radar-axis-openness = Openness
radar-axis-conscientiousness = Conscien{"\u00AD"}tiousness
radar-axis-extraversion = Extraversion
radar-axis-agreeableness = Agreeableness
radar-axis-stability = Emotional stability
radar-axis-cognition = Cognition
radar-axis-circulation = Circulation
radar-axis-breathing = Breathing
radar-axis-metabolism = Metabolism
radar-axis-illness = Freedom from illness
radar-axis-senses = Senses
radar-axis-rest = Sleep and mood
radar-folded-open = Show this chart
radar-folded-why = This person is recorded as living. A portrait of a living person's temperament stays folded until somebody who may read it asks to see it.
radar-empty = Nothing in this record can be read into this chart yet.
radar-table-caption = { $chart }: each axis, its score and what it was read from
radar-col-axis = Axis
radar-col-score = Score
radar-col-from = Read from
radar-no-score = no score
radar-from-none = none
record-link-outgoing = from this person
record-link-incoming = to this person
# The operational banner on the dashboard. `/health` answers a monitor, in
# English; these say the same four things to whoever is signed in, in their
# own language, and they are the only warning a household without a monitor
# will ever get.
health-attention = Needs attention:
health-standing-token = An emergency administrator token is still set. It signs in past every account and every permission — which is what it is for on the day nobody can sign in, and not something to leave in place now that somebody can. Remove it from the service's environment file and restart.
health-bundle-invalid = { $errors ->
        [one] The family data no longer passes validation: one error, listed under Validation below.
       *[other] The family data no longer passes validation: { $errors } errors, listed under Validation below.
    }
health-disk-unknown = The free space on this machine could not be read, so nothing here can promise that the next save will fit.
health-disk-no-room-to-save = Saving is not possible: { $free } is free, and rebuilding this bundle needs { $need }. Nothing has been lost, and nothing can be changed until there is room.
health-disk-critical = The disk is { $percent }% free — { $free } left. Saving will stop working shortly.
health-disk-low = The disk is { $percent }% free — { $free } left. Worth attending to before it becomes urgent.
health-backup-unconfigured = Nothing is being backed up. One failed disk would take every record with it.
health-backup-never = A backup directory is set, but no backup has ever been written to it.
health-backup-stale = { $days ->
        [one] The newest backup is a day old. One should be written every day.
       *[other] The newest backup is { $days } days old. One should be written every day.
    }
health-cache-missing = { $missing ->
        [one] One of { $declared } attached files is missing from the cache and cannot be downloaded until the next save rebuilds it.
       *[other] { $missing } of { $declared } attached files are missing from the cache and cannot be downloaded until the next save rebuilds them.
    }

## River

nav-river = River
nav-home = Family
river-aria = The river around { $name }: ancestors below, descendants above.
river-band-living = LIVING
river-circa = c.
river-trail = Trail
river-back = ← Back
river-range = Range
river-motion = Motion
river-travel = travel
river-cut = cut
river-legend-attested = attested
river-legend-inferred = inferred
river-legend-speculative = speculative
river-legend-lost = record ends
river-legend-repeat = ×n reached by n lines, drawn under each
river-legend-again = ↔ drawn again beside a later partner
river-legend-discharge = descendants
river-legend-direction = ↓ toward the source · ↑ toward the living
view-tree-direction = ↓ the roots: ancestors · ↑ the crown: descendants
view-galaxy-direction = ↓ ancestors below the sun · ↑ descendants above
view-summit-direction = ↓ ancestors in the valley · ↑ descendants up toward the summit
river-legend-generation = one generation ≈ 29 years
river-zoom = Zoom
river-still = Hold the landscape still
view-tree-still = Hold the tree still
view-galaxy-still = Hold the sky still
view-summit-still = Hold the mountain still
river-whole = Whole family
river-names-all = All names
river-zoom-hint = Zoom: Ctrl + wheel, pinch, or + / − · zoomed in, drag to move
river-grid-link = Grid view
river-tab-sources = Sources
river-legend-title = How to read the river
view-tree-name = Living tree
view-tree-legend = How to read the tree
view-galaxy-name = Galaxy
view-galaxy-legend = How to read the galaxy
view-summit-name = Ascent
view-summit-legend = How to read the mountain
hud-ancestors = Ancestors
hud-descendants = Descendants
hud-generations = Generations
hud-living = Living
hud-attested = Attested links

# Masthead search
search-label = Search for a person
search-short = Person
date-qualifier = Qualifier
date-qualifier-circa = circa
search-title = Search
search-heading = People matching “{ $q }”
search-none = Nobody you can see is called that.
search-open-record = Open the record
search-hint = Enter travels the river to them; Shift+Enter opens their record.
view-tree-search-hint = Enter grows the tree around them; Shift+Enter opens their record.
view-galaxy-search-hint = Enter makes them the sun; Shift+Enter opens their record.
view-summit-search-hint = Enter takes you to them; Shift+Enter opens their record.
search-summary = { $n ->
        [one] { $n } person
       *[other] { $n } people
    } · ↵ travel · ⇧↵ open the record
search-empty-query = Type a name to search.

# Data quality (/quality)
nav-quality = Data quality
quality-title = Things worth a look
quality-none = Nothing worth a look: every check passed.
quality-intro = Nothing here is changed for you. Each line is a pattern worth checking — often there is a good reason for it. Open the record and decide, or mark it as checked.
quality-groups-label = Kinds of finding
quality-all = All
quality-show-label = Show
quality-show-open-option = To check
quality-download = Download
quality-group-names = Names
quality-group-relations = Relations
quality-group-dates = Dates
quality-group-places = Places
quality-group-evidence = Evidence
quality-export-csv = Download as CSV
quality-export-json = Download as JSON
quality-open = Open
quality-none-file = Nothing worth a look in this file.
quality-show-list = Show the list
quality-after-import = Nothing in the file was changed. Once it is the site’s archive, the data-quality page lists the same things with a link to each record.
quality-rule-name-sentence = A sentence in a name field
quality-name-sentence = “{ $value }” looks more like a sentence than a name: if it is an explanation, keep the name in the field and move the rest into a note.
quality-rule-union-parent-child = A couple who are also parent and child
quality-union-parent-child = { $parent } and { $child } appear both as a couple and as parent and child: worth checking which relationship the sources support.
quality-rule-child-listed-twice = A child listed twice
quality-child-listed-twice = { $child } appears twice among this family’s children: if it is the same child, one entry can go.
quality-rule-duplicate-union = The same couple recorded twice
quality-duplicate-union = { $a } and { $b } have { $count } unions recorded: if they married only once, these may be one union entered twice.
quality-rule-partner-twice = One person on both sides of a union
quality-partner-twice = { $person } appears on both sides of this union: worth checking who the other partner was, or leaving that side empty if nobody knows.
quality-rule-foreign-surnames = A son’s surname that matches neither parent
quality-foreign-surnames = A surname matching neither parent’s: { $who }. That can be normal — a name change, a step-child, a different spelling — but is worth a look. (Daughters are left out, since a married name is the usual reason.)
quality-rule-death-before-birth = Death before birth
quality-death-before-birth = { $person } has a death in { $death }, before their birth in { $birth }: one of the two dates may be mistyped; worth checking against the sources.
quality-rule-child-before-parent = A child born before a parent
quality-child-before-parent = { $child } (born { $childborn }) appears to be born before their parent { $parent } (born { $parentborn }): worth checking the dates or the parentage.
quality-rule-parent-too-young = A parent younger than twelve
quality-parent-too-young = { $parent } would have been { $age } when { $child } was born, which is unusual: worth checking the dates or the parentage.
quality-rule-parent-too-old = A parent older than sixty-five
quality-parent-too-old = { $parent } would have been { $age } when { $child } was born, which is unusual: worth checking the dates or the parentage.
quality-rule-marriage-before-birth = A marriage before a partner’s birth
quality-marriage-before-birth = This marriage is dated { $married }, before { $person } was born in { $born }: worth checking the marriage date.
quality-rule-isolated-record = A record with no date and no relations
quality-isolated-record = { $person } has no date and no link to anyone in the family: worth adding how they belong, or removing the record if it was entered by mistake.
quality-rule-place-no-region = A place with no region
quality-place-no-region = { $place } has no region: adding one helps tell apart places with the same name.
quality-rule-region-free-text = A region that reads like a note
quality-region-free-text = The region of { $place } reads “{ $value }”, which looks more like a note than a region: worth moving the note to the record it describes.
quality-rule-place-near-duplicate = Places that may be the same
quality-place-near-duplicate = { $names } may be the same place written differently: if so, they can be merged into one.
quality-rule-not-a-place = An entry that may not be a place
quality-not-a-place = “{ $place }” is in the list of places but may not be a place — it starts with a lowercase letter, which place names usually do not: worth checking whether it is a note about a place.
quality-rule-certain-without-source = Certain, with no source
quality-certain-without-source = { $record } has a claim marked fully certain ({ $claims }) with no source attached: worth attaching the source, or lowering the confidence.
quality-dismiss = Checked — normal for us
quality-restore = Show it again
quality-note-label = Why (optional)
quality-show-dismissed = Checked and kept
quality-show-open = Back to the open list
quality-dismissed-by = Checked by { $by } on { $at }
quality-gone-title = That finding is no longer there
quality-gone-detail = The record changed since the list was opened, so the finding no longer reads the same. Reload the list and look again.
quality-summary = { $n ->
    [one] One thing worth a look
   *[other] { $n } things worth a look
}
quality-summary-file = { $n ->
    [one] One thing worth a look in this file
   *[other] { $n } things worth a look in this file
}
quality-dismissed-summary = { $n ->
    [one] One finding checked and kept
   *[other] { $n } findings checked and kept
}
quality-dismiss-all = Mark all as checked ({ $n })
quality-restore-all = Show all again ({ $n })
quality-bulk-conflict = Records changed since the list was opened: { $n }. Nothing was marked; reload the list and try again.

## The record's editor (v0.4.2)

edit-tab-changed = changed, not saved yet
edit-remove = Remove
edit-add-partner = Add a partner
edit-add-child = Add a child
edit-union-with = Union with { $names }
edit-union-alone = Union with no partner recorded
edit-child-of = Child of { $names }
edit-new-family = Start a family
edit-cite-title = What rests on a source
edit-attach-to = Cite it for
edit-add-source = Add a source
edit-error-source-title = A new source needs a title. Nothing was saved.
edit-detach-file = Detach from this record
edit-no-sources = None of this person's own facts cites a source yet.
edit-no-files = No file is attached to this record yet.
edit-saved = { $n ->
        [one] Saved: one record written.
       *[other] Saved: { $n } records written in one save.
    }
edit-saved-none = Nothing had changed, so nothing was saved.
edit-back-to-record = Back to the record
edit-avatar = Profile picture
edit-avatar-frame = Frame the face
edit-avatar-upload = Use as the profile picture
edit-stale = Someone else changed { $what } since this page was opened. Nothing was saved; everything you entered is still here.
edit-refused = Nothing was saved: { $reason }

# Administration console (v0.5.0, docs/adr/0011)
error-cross-site-title = Refused: posted from another site
error-cross-site-detail = This form came from a page on another site, so it was not acted on. If you sent it from this site, reload the page and send it again.
admin-nav-label = Administration sections
admin-nav-overview = Overview
admin-nav-bundle = AXGF bundle
admin-nav-infrastructure = Infrastructure
admin-nav-accounts = Accounts
admin-nav-connectivity = Connectivity

# Administration console: the AXGF bundle (v0.5.0, docs/adr/0011)
bundle-title = AXGF bundle
bundle-lede = The family's archive as this site holds it: { $path }. What it conforms to, what looks wrong, and the two ways to take a copy away.
bundle-unset-title = No family yet.
bundle-unset-detail = This installation holds an empty bundle, so there is nothing to check yet. Set it up from an archive, from a GEDCOM file or from nothing.
bundle-kpi-conformity = Conformity
bundle-verdict-ok = Conforms
bundle-verdict-warn = To watch
bundle-verdict-fail = Does not conform
findings-tally = Green: { $ok } · Warnings: { $warn } · Critical: { $fail }
bundle-kpi-entities = Entities
bundle-kpi-entities-note = { $persons ->
    [one] 1 person
    *[other] { $persons } people
    } · { $files ->
    [one] 1 attached file
    *[other] { $files } attached files
    }
bundle-kpi-size = Size on disk
bundle-kpi-members = { $n ->
    [one] 1 file in the archive
    *[other] { $n } files in the archive
    }
bundle-kpi-format = Format
bundle-kpi-format-note = This build reads and writes AXGF { $current }.
bundle-conformity = Conformity
bundle-conformity-intro = Nine checks, read from the bundle as it is now; none of them changes it. Open a line to see what to do.
bundle-area-format = Format
bundle-area-integrity = Integrity
bundle-area-privacy = Privacy
bundle-area-content = Content
level-ok = Green
level-warn = Warning
level-fail = Critical
findings-what-to-do = What to do
findings-go = Go there →
bundle-anomalies = Anomalies
bundle-anomalies-intro = What the data quality page finds, rule by rule: still open, and already marked checked.
bundle-anomalies-rule = Rule
bundle-anomalies-open = Open
bundle-anomalies-checked = Checked
bundle-anomalies-none = The data quality page finds nothing to look at.
bundle-diagnostics = Validator diagnostics
bundle-facts = Particulars
bundle-fact-path = File
bundle-fact-size = Size
bundle-fact-spec = Format
bundle-fact-schema = Schema carried
bundle-fact-none = none
bundle-fact-created = Created
bundle-fact-updated = Last written
bundle-fact-generator = Written by
bundle-fact-gedcom = Converted from
bundle-export = Export
bundle-export-raw = The bundle as it is (.axgf)
bundle-export-raw-go = Download the bundle
bundle-export-sealed = Encrypted for one person (.axgf.gpg)
bundle-export-sealed-intro = The same bundle, as an OpenPGP message that only the holder of the private key can open. Paste their public key: the text that begins “-----BEGIN PGP PUBLIC KEY BLOCK-----”.
bundle-export-sealed-go = Download encrypted
seal-recipient = For
seal-fingerprint = Fingerprint
seal-algorithm = Algorithms
seal-fips-yes = in the FIPS-approved set
seal-fips-no = outside the FIPS-approved set
seal-expires = Key expires
seal-never = never
seal-key-label = The recipient's public key
seal-key-hint = An RSA key of at least 2048 bits, or an elliptic-curve key. Check the fingerprint with its owner before sending anything.
seal-remember = Keep this key for next time
seal-check = Read the key only
seal-fips-note = The bundle is encrypted with AES-256 to the recipient's key. FIPS-approved algorithms are RSA of 3072 bits or more and the NIST curves P-256, P-384 and P-521: ask for P-384. The library used here is not itself a FIPS 140-3 validated module.
seal-decrypt-note = The recipient opens it with GnuPG:
seal-forget = Forget the kept key
seal-key-kept = The key { $fingerprint } is kept for the next encrypted export.
seal-error-no-key = Paste the recipient's public key first.
seal-error-not-a-key = That is not an OpenPGP public key. Paste the whole block, from its BEGIN line to its END line, and never a private key.
seal-error-bad-signatures = That key's own signatures do not verify: it may have been altered. Ask its owner for a fresh copy.
seal-error-expired = That key, or every key in it that can encrypt, has expired. Ask its owner to extend it or to send a new one.
seal-error-revoked = Its owner has revoked that key. Ask them for their new one.
seal-error-no-encryption-key = That key can sign but not encrypt: it has no encryption subkey. Ask its owner for one (gpg --quick-add-key … encr).
seal-error-too-weak = That RSA key has { $bits } bits; an export needs at least { $min }.
seal-error-unsupported = That key's algorithm is not one an export uses: RSA or elliptic-curve keys only.
seal-error-io = The encrypted copy could not be made: { $error }
date-time-utc = { $date }, { $time } UTC

# Administration console: the bundle's conformity (v0.5.0, docs/adr/0011)
bundle-check-format-title = Format version
bundle-check-format-ok = AXGF { $version }, the version this build reads and writes.
bundle-check-format-missing = The manifest names no AXGF version.
bundle-check-format-missing-fix = The bundle was not written by an AXGF tool, or it was damaged. Restore the last backup that validated (axgf-cms restore).
bundle-check-format-older = Written as AXGF { $version }; the next save writes it as AXGF { $current }.
bundle-check-format-older-fix = Nothing to do: the next edit rewrites it. Keep a copy of the old file if another program still reads only { $version }.
bundle-check-format-newer = Written as AXGF { $version }, newer than this build's { $current }: saving could lose what this build does not understand.
bundle-check-format-newer-fix = Upgrade axgf-cms to a release that reads AXGF { $version } before anyone edits (Infrastructure, Version).
bundle-check-manifest-title = The manifest's counts
bundle-check-manifest-ok = The manifest's counts match what the bundle holds.
bundle-check-manifest-drift = The manifest declares { $declared } where the bundle holds { $actual } ({ $collection }).
bundle-check-manifest-fix = Nothing is lost: the counts are rewritten at the next save. If this archive came from elsewhere, check it was copied whole.
bundle-check-validation-title = Validation
bundle-check-validation-ok = The library's validator finds nothing wrong.
bundle-check-validation-errors = { $errors ->
    [one] 1 error
    *[other] { $errors } errors
    }: the bundle does not validate.
bundle-check-validation-errors-fix = Each diagnostic below names the record and the field. Correct the record, or restore the last backup that validated.
bundle-check-validation-warnings = { $warnings ->
    [one] 1 warning
    *[other] { $warnings } warnings
    }. They never block a save; each says something may be off.
bundle-check-validation-warnings-fix = Read them below. Most are references to records that are not in this bundle, which a GEDCOM import often leaves.
bundle-check-payloads-title = Attached files
bundle-check-payloads-none = No attached files.
bundle-check-payloads-ok = { $declared ->
    [one] The one file is present.
    *[other] All { $declared } files are present.
    }
bundle-check-payloads-missing = { $missing } of { $declared } attached files are missing from the cache.
bundle-check-payloads-fix = The next save rebuilds the cache from the bundle. If they are missing from the bundle itself, restore them from a backup.
bundle-check-binding-title = Accounts and bundle
bundle-check-binding-ok = The accounts file was made for this family.
bundle-check-binding-mismatch = The accounts file was made for { $expected }; this bundle is { $found }.
bundle-check-binding-fix = If the bundle was restored or renamed on purpose, nothing to do. Otherwise one family's accounts apply to another's tree: put the right .acl back beside it.
bundle-check-living-title = The living, in public
bundle-check-living-ok = No one recorded as living is visible to visitors.
bundle-check-living-public = { $n ->
    [one] 1 person recorded as living is
    *[other] { $n } people recorded as living are
    } marked public: any visitor reads their record.
bundle-check-living-fix = Unless they agreed to it, set their visibility to members (Edit, Identity).
bundle-check-special-title = Special categories
bundle-check-special-none = No record holds health, biometric, genomic, criminal or behavioural data.
bundle-check-special-some = { $n ->
    [one] 1 record holds
    *[other] { $n } records hold
    } health, biometric, genomic, criminal or behavioural data. For the living it stays with administrators, whatever the record says.
bundle-check-quality-title = Data quality
bundle-check-quality-ok = Nothing to look at.
bundle-check-quality-open = { $n ->
    [one] 1 thing to look at
    *[other] { $n } things to look at
    }, in { $groups ->
    [one] 1 group
    *[other] { $groups } groups
    }.
bundle-check-quality-fix = Open the data quality page: each line names the record and what looks wrong. Correct it, or mark it checked.
bundle-check-size-title = Size
bundle-check-size-ok = { $size }, under the { $warn } past which this page warns.
bundle-check-size-heavy = { $size }, past { $warn }.
bundle-check-size-fix = The archive is the family's to grow. Watch the disk and the backups (Infrastructure, Volumes), and raise --size-warn-mb if this size is expected.
seal-algorithm-value = { $key } · AES-256
seal-key-placeholder = -----BEGIN PGP PUBLIC KEY BLOCK-----

# Administration console: the infrastructure's checks (v0.5.0, docs/adr/0011 §3)
infra-area-security = Security
infra-area-access = Access
infra-area-data = Data
infra-area-capacity = Capacity
infra-area-release = Version
infra-area-platform = Platform
infra-verdict-ok = Healthy
infra-verdict-warn = Needs attention
infra-verdict-fail = Critical
duration-days = { $n ->
    [one] { $n } day
    *[other] { $n } days
    }
duration-hours = { $n ->
    [one] { $n } hour
    *[other] { $n } hours
    }
duration-minutes = { $n ->
    [one] { $n } minute
    *[other] { $n } minutes
    }
infra-transport-title = How the site is reached
infra-transport-ok = This page arrived over HTTPS.
infra-transport-exposed = This page arrived unencrypted, and the server listens on { $bind }, beyond this machine: passwords and session cookies cross the network in clear.
infra-transport-exposed-fix = Put an HTTPS reverse proxy (Caddy, nginx, HAProxy) in front, and bind the service to 127.0.0.1 again: rerun bootstrap.sh with --bind 127.0.0.1:8080, or remove the drop-in that widened it.
infra-transport-plain = This page arrived unencrypted, from a server listening on { $bind }. That is fine on this machine alone, and only there.
infra-transport-plain-fix = Reach the site through its HTTPS address. If a proxy does terminate TLS, have it send X-Forwarded-Proto: https so this server knows.
infra-forwarded-title = Forwarded addresses
infra-forwarded-ok = The server listens on this machine only, so the client address a proxy forwards can be trusted to slow down repeated failed sign-ins.
infra-forwarded-exposed = The server can be reached directly, and it trusts the X-Forwarded-For header: a client that sets that header itself can escape the slow-down after failed sign-ins.
infra-forwarded-fix = Bind the service to 127.0.0.1 behind the proxy, or firewall the port so that only the proxy reaches it.
infra-cross-site-title = Posts from other sites
infra-cross-site-ok = No form was posted here from another site in the last day.
infra-cross-site-refused = { $n ->
    [one] One form posted from another site was refused in the last day.
    *[other] { $n } forms posted from other sites were refused in the last day.
    }
infra-cross-site-fix = Nothing got through. If the refusals are yours — the same site reached by a second address — set the public address under Connectivity, or use that one address.
infra-sso-title = Sign-in services
infra-sso-none = No sign-in service is enabled; accounts sign in with a password.
infra-sso-ok = { $n ->
    [one] One sign-in service is enabled, and the public address is HTTPS.
    *[other] { $n } sign-in services are enabled, and the public address is HTTPS.
    }
infra-sso-no-https = A sign-in service is enabled, but the public address is not HTTPS: providers refuse to send their answer to a plain address, and signing in fails.
infra-sso-fix = Under Connectivity, set the public address to the https:// address readers use.
infra-admins-title = Administrators
infra-admins-none = No active account can administer this site. Only the emergency token can, and only while it is set.
infra-admins-none-fix = Sign in with the emergency token, create an administrator under Accounts, then remove the token.
infra-admins-one = One administrator. If that person is away or loses the password, nobody can manage the site.
infra-admins-one-fix = Give a second, trusted person the administrator role under Accounts.
infra-admins-ok = { $n ->
    [one] { $n } administrator can sign in.
    *[other] { $n } administrators can sign in.
    }
infra-token-title = Emergency token
infra-token-generated = No emergency token is set.
infra-token-only-way = An emergency token is set, and with no administrator account it is the only way in. Keep it until an administrator exists.
infra-token-standing = An emergency token is still set. It signs in past every account and every permission.
infra-token-fix = Remove AXGF_CMS_ADMIN_TOKEN from the service's environment file, then restart the service.
infra-registration-title = Self-registration
infra-registration-closed = Visitors cannot create accounts; only an administrator can.
infra-registration-secret = Visitors can create a member account if they know the registration secret.
infra-registration-open = Any visitor can create a member account.
infra-registration-open-fix = That suits a site meant to be open. Otherwise require the registration secret, or close registration, under Accounts.
infra-failures-title = Failed sign-ins
infra-failures-ok = { $n ->
    [one] { $n } failed sign-in in the last day.
    *[other] { $n } failed sign-ins in the last day.
    }
infra-failures-many = { $n ->
    [one] { $n } failed sign-in in the last day: somebody may be guessing passwords.
    *[other] { $n } failed sign-ins in the last day: somebody may be guessing passwords.
    }
infra-failures-fix = Each address is already slowed down after repeated failures. If the count keeps climbing, find where the attempts come from in the proxy's log and block it there, and make sure every administrator has a long password or signs in through a sign-in service.
infra-health-bundle-title = Family data
infra-health-bundle-ok = The bundle opens and passes validation.
infra-health-bundle-fix = Open the AXGF bundle page for the errors and correct them, or restore the last backup that validated (axgf-cms restore).
infra-health-cache-title = Attachment cache
infra-health-cache-ok = Every attached file is in the cache.
infra-health-cache-fix = Save any record once: the save rebuilds the cache. Files still missing after that are missing from the bundle itself.
infra-health-backup-title = Backups
infra-health-backup-ok = { $count ->
    [one] A recent backup exists; { $count } archive is kept.
    *[other] A recent backup exists; { $count } archives are kept.
    }
infra-health-backup-fix = Without a backup directory, rerun bootstrap.sh with --backup-dir. With one, check the timer (systemctl status axgf-cms-backup.timer) and its log (journalctl -u axgf-cms-backup).
infra-disk-title = Disk space
infra-disk-ok = { $free } free of { $total } ({ $percent }% free).
infra-disk-fix = Free space on the partition that holds the data directory — old backups also kept elsewhere, logs, other programs' files — or move the data to a larger disk.
infra-growth-title = Growth
infra-growth-ok = { $days ->
    [one] Growing about { $rate } a month; room for { $days } more day at that rate.
    *[other] Growing about { $rate } a month; room for { $days } more days at that rate.
    }
infra-growth-soon = { $days ->
    [one] Growing about { $rate } a month: at that rate the disk is full in { $days } day.
    *[other] Growing about { $rate } a month: at that rate the disk is full in { $days } days.
    }
infra-growth-fix = Plan the space now: a larger disk, fewer backups kept on the same disk, or backups moved to another one.
infra-growth-flat = The backups have not grown; there is nothing to plan.
infra-growth-unknown = Not enough backups yet to measure growth: it takes at least a day between the oldest and the newest.
infra-memory-title = Memory
infra-memory-ok = { $used } used of the service's { $max } ceiling ({ $percent }%).
infra-memory-high = { $used } used of the service's { $max } ceiling ({ $percent }%): close to where the system stops the service.
infra-memory-fix = Raise MemoryMax= in a drop-in for axgf-cms.service, or find what holds the memory — a very large import, a large attachment — and restart the service.
infra-memory-unlimited = { $used } in use, with no ceiling set for the service.
infra-version-title = Version
infra-version-ok = v{ $current } is the newest published release.
infra-version-behind = v{ $current } is running; { $latest } is published.
infra-version-behind-fix = Read the release notes, then upgrade from the Version section below. A backup is taken first, and the previous version comes back on its own if the new one does not start.
infra-version-unknown = Whether a newer version exists is not known: the list of releases has not been fetched.
infra-version-unknown-fix = Use “Check for new versions” below. The production sandbox cannot reach GitHub itself; once the upgrade helper is installed, it fetches the list every day.
infra-version-stale = { $days ->
    [one] The list of releases was last fetched { $days } day ago.
    *[other] The list of releases was last fetched { $days } days ago.
    }
infra-upgrades-title = Upgrades
infra-upgrades-ok = The upgrade helper is installed: the button below can change the version.
infra-upgrades-manual = The upgrade helper is not installed, so the version is changed by hand on the server.
infra-upgrades-fix = Run the current bootstrap.sh once as root: it installs the helper along with the service.
infra-sandbox-title = Sandbox
infra-sandbox-ok = The process runs with no-new-privileges and a system-call filter.
infra-sandbox-unknown = This system does not report the process's sandbox.
infra-sandbox-off = The process runs without part of its sandbox: no-new-privileges or the system-call filter is off.
infra-sandbox-fix = Run the service from the unit bootstrap.sh writes, without drop-ins that loosen it. On a development machine this is expected.
infra-clock-title = Clock
infra-clock-ok = The system clock reads { $year }, which is plausible.
infra-clock-wrong = The system clock reads { $year }: dates, backup ages and sessions are all wrong.
infra-clock-fix = Turn on time synchronisation: timedatectl set-ntp true.

# Administration console: the Infrastructure page (v0.5.0, docs/adr/0011)
infra-title = Infrastructure
infra-lede = The machine this site runs on, checked each time this page opens: how it is reached, who holds the keys, whether the data is safe and still fits, which version runs, and the sandbox around it.
infra-kpi-health = Health
infra-kpi-version = Version
infra-kpi-version-newer = { $version } available
infra-kpi-version-current = Up to date
infra-kpi-version-unknown = Not checked yet
infra-kpi-disk = Free space
infra-kpi-disk-note = { $percent }% of { $total } used
infra-kpi-compute = Machine
infra-kpi-cpus = { $n ->
    [one] { $n } core
    *[other] { $n } cores
    }
infra-kpi-load = Load { $load }
infra-kpi-rss = { $size } in use
infra-unknown = Not known
infra-health = Health
infra-health-intro = Green is fine, a warning is worth a look, critical needs attention now. Each finding opens on what to do about it.
infra-json = The same checks as JSON, for a monitor
infra-version = Version
infra-version-running = Running
infra-version-latest = Latest published
infra-version-up-to-date = up to date
infra-version-checked = List fetched
infra-version-never = never
infra-source-direct = by this server
infra-source-helper = by the upgrade helper
infra-version-token = Private repository
infra-token-env-set = A GitHub token is set for listing and downloading releases.
infra-token-env-unset = No token. If the repository is private, put one in { $name }, readable by root only: the upgrade helper reads it there.
infra-releases-error = The last attempt to fetch the list failed:
infra-releases-refresh = Check for new versions
infra-releases-asked = The upgrade helper has been asked for a fresh list. Reload this page in a minute.
infra-releases-not-asked = The request could not be written: { $error }
infra-upgrade-status-running = Changing from { $from } to { $to }…
infra-upgrade-status-succeeded = Changed from { $from } to { $to }.
infra-upgrade-status-failed = The change from { $from } to { $to } failed; { $from } is still the version running.
infra-upgrade-status-refused = The upgrade helper refused the change to { $to }.
infra-upgrade-log = Helper output
infra-upgrade-pending = { $by } asked for { $version } on { $at }; the upgrade helper has not picked it up yet.
infra-releases = Published releases
infra-releases-tag = Version
infra-releases-published = Published
infra-releases-relation = Compared with this one
infra-releases-notes = Release notes
infra-releases-open = Read
infra-relation-newer = newer
infra-relation-current = this one
infra-relation-older = older
infra-prerelease = release candidate
infra-releases-none = No list of releases yet.
infra-upgrade = Change version
infra-upgrade-intro = The button asks the upgrade helper, which runs as root outside this site's sandbox. It takes a backup, installs the release once its checksum is verified, restarts the service and checks it; if the new version does not answer, the previous one is put back.
infra-upgrade-version = Version to install
infra-upgrade-confirm = To go back to an older version, type its tag again
infra-upgrade-confirm-hint = Not needed for a newer version. An older version may not read data a newer one wrote: read its notes first.
infra-upgrade-go = Install this version
infra-upgrade-safety = The site is unavailable for a few seconds while the service restarts.
infra-upgrade-manual = The upgrade helper is not installed on this server, so this page cannot change the version. As root, on the server:
infra-upgrade-manual-helper = Running the current bootstrap.sh once installs the helper, and the button here works from then on.
infra-upgrade-asked = { $version } has been requested. The upgrade helper picks it up within a few seconds; this page shows how it went.
infra-upgrade-no-helper = The upgrade helper is not installed, so { $version } cannot be installed from here.
infra-upgrade-unknown-version = { $version } is not in the list of published releases.
infra-upgrade-same-version = { $version } is the version already running.
infra-upgrade-confirm-older = { $version } is older than the running version. Type its tag in the confirmation field to go back to it.
infra-volumes = Storage
infra-volumes-intro = The partition that holds the data directory ({ $path }), and what this site keeps on it.
infra-volumes-partition = Partition
infra-of = { $used } of { $total }
infra-volumes-ours = This site
infra-volumes-breakdown = What this site keeps
infra-part-bundle = Family bundle
infra-part-cache = Attachment cache
infra-part-backups = Backups
infra-part-binary = Program
infra-part-journal = Change journal
infra-part-accounts = Accounts and settings
infra-files = { $n ->
    [one] { $n } file
    *[other] { $n } files
    }
infra-growth = Growth
infra-growth-month = Per month
infra-growth-not-enough = not enough backups to tell
infra-growth-left = Room left at this rate
infra-backups = Backups
infra-backups-dir = Directory
infra-backups-count = Archives
infra-backups-newest = Newest
infra-backups-oldest = Oldest
infra-backups-room = Room where they are kept
infra-free-of = { $free } free of { $total }
infra-paths = Paths
infra-path-data = Data directory
infra-path-binary = Program
infra-compute = Computing
infra-compute-hidden = The service's sandbox hides the machine's own figures from it; what shows here is what the process can see.
infra-compute-memory = Machine memory
infra-available-of = { $available } free of { $total }
infra-compute-ceiling = Service memory
infra-compute-no-ceiling = { $used }, no ceiling
infra-compute-os = System
infra-compute-kernel = Kernel
infra-compute-arch = Architecture
infra-compute-cpu = Processor
infra-compute-cpus = Cores
infra-compute-load = Load (1, 5, 15 min)
infra-compute-rss = This process
infra-compute-threads = Threads
infra-compute-files = Open files
infra-compute-uptime = Service running for
infra-compute-sys-uptime = Machine running for
infra-compute-sandbox = Sandbox
infra-compute-nnp-on = no new privileges
infra-compute-nnp-off = privileges can be gained
infra-compute-seccomp-on = system calls filtered
infra-compute-seccomp-off = system calls unfiltered
infra-activity = Activity
infra-activity-intro = Counted over { $span }: the service keeps one day, and starts again from zero when it restarts.
infra-activity-sessions = Open sessions
infra-activity-signed-in = Sign-ins
infra-activity-failed = Failed sign-ins
infra-activity-registered = Accounts created
infra-activity-cross-site = Cross-site posts refused

# Sign-in services: what the reader is told when one fails (v0.5.0, docs/adr/0012)
sso-error-not-configured = This sign-in service is not set up on this site.
sso-error-no-public-url = Signing in through a service needs this site's public address, which is not set yet. An administrator sets it under Connectivity.
sso-error-unreachable = The sign-in service could not be reached. Try again in a moment, or sign in another way.
sso-error-refused = The sign-in service did not let you in, or the sign-in was cancelled there.
sso-error-stale = That sign-in had expired, or was started in another browser. Start again.
sso-error-invalid = The sign-in service's answer did not pass this site's checks, so it was not used. Start again; if it keeps happening, tell an administrator.
infra-upgrade-already-pending = { $version } is already waiting for the upgrade helper. Wait for it to finish before asking for another version.

# Administration console: Connectivity, sign-in services and registration (v0.5.0, docs/adr/0012)
connectivity-title = Connectivity
connectivity-lede = How readers reach this site and how they sign in: the public address, the sign-in services and the rules every sign-in follows, administrators' included.
connectivity-kpi-address = Public address
connectivity-address-unset = Not set
connectivity-address-https = HTTPS
connectivity-address-plain = Unencrypted
connectivity-address-unset-note = Every sign-in service needs it
connectivity-kpi-services = Sign-in services
connectivity-kpi-services-note = { $n ->
    [one] ready to offer
    *[other] ready to offer
    }
connectivity-kpi-passwords = Passwords
connectivity-policy-short-everyone = Everyone
connectivity-policy-short-non-admins = Not administrators
connectivity-policy-short-nobody = Nobody
connectivity-policy-note-everyone = Every account may sign in with its password, and with a linked service too.
connectivity-policy-note-non-admins = Administrators sign in through a service only; everyone else may still use a password. A stolen administrator password then opens nothing.
connectivity-policy-note-nobody = Everyone signs in through a service. An account with no linked service and no address to link by cannot sign in.
connectivity-kpi-admins-linked = { $n ->
    [one] { $n } administrator with a linked service
    *[other] { $n } administrators with a linked service
    }
connectivity-kpi-registration = Registration
connectivity-kpi-registration-go = Set on the Accounts page →
registration-short-closed = Administrators only
registration-short-secret = With the family's secret
registration-short-open = Open to visitors
connectivity-address = Public address
connectivity-address-intro = The address readers type, as they type it: https://family.example. Sign-in services send readers back to it, so it must be the public address, not the one this server listens on.
connectivity-address-label = Public address
connectivity-address-hint = Leave it empty while nobody signs in through a service.
connectivity-address-invalid = That is not an http or https address with a host name.
connectivity-address-saved = The public address is saved.
connectivity-policy = Sign-in rules
connectivity-policy-intro = They apply to every account, administrators' included. A rule that would leave no administrator able to sign in is refused.
connectivity-policy-passwords = Who may sign in with a password
connectivity-policy-without-way = { $n ->
    [one] { $n } active account has no linked service and no address: under “Nobody” it could not sign in.
    *[other] { $n } active accounts have no linked service and no address: under “Nobody” they could not sign in.
    }
connectivity-link-by-email = Link a service to the account with the same verified address
connectivity-link-by-email-note = How a relative gets a one-click sign-in without any registration: create their account with their address, and their first sign-in through a service that verified that address links it. Only an address typed by an administrator or verified by a service is ever matched.
connectivity-domains = Domains allowed to open an account
connectivity-domains-hint = One per line, such as family.example. Empty: any. When set, only an address a service verified at one of these domains can open an account, and the registration form is not offered.
connectivity-domain-invalid = “{ $domain }” is not a domain name.
connectivity-policy-unknown = Choose who may sign in with a password.
connectivity-policy-lockout = Refused: with this change no active administrator could sign in. Link a sign-in service to an administrator's account first, by signing in through it once, or keep passwords for administrators.
connectivity-policy-saved = The sign-in rules are saved.
connectivity-not-saved = Not saved: { $error }
connectivity-services = Sign-in services
connectivity-services-intro = Register this site in the service's own console with the redirect address shown, then paste the client ID and secret here. Visitors then see “Continue with …” on the sign-in and registration pages.
connectivity-outbound-note = Signing in through a service needs this server to reach the service. The unit bootstrap.sh writes refuses outgoing connections (IPAddressDeny=any): allow them with sudo systemctl edit axgf-cms, adding IPAddressAllow=any under [Service], then restart. The test button says whether it can.
connectivity-service-ready = Ready
connectivity-service-incomplete = Incomplete
connectivity-service-off = Off
connectivity-service-enable = Offer this service on the sign-in and registration pages
connectivity-service-issuer = Issuer
connectivity-service-issuer-hint = The provider's address, such as https://auth.family.example/realms/family; its settings are read from /.well-known/openid-configuration under it.
connectivity-service-label = Button label
connectivity-service-tenant = Directory (tenant)
connectivity-service-tenant-hint = common for any Microsoft account, organizations for work and school accounts, consumers for personal ones, or your directory's id.
connectivity-service-client-id = Client ID
connectivity-service-secret = Client secret
connectivity-service-secret-set = A secret is set. Leave this empty to keep it.
connectivity-service-secret-unset = No secret set yet.
connectivity-service-secret-clear = Remove the secret
connectivity-service-redirect = Redirect address to register
connectivity-service-redirect-needs-address = Set the public address first
connectivity-service-console = Where to register
connectivity-service-console-open = { $name }'s console
connectivity-service-test = Test the connection
connectivity-issuer-invalid = The issuer must be an https:// address.
connectivity-tenant-invalid = A directory is a name or an id: letters, digits, dots and dashes.
connectivity-provider-incomplete = { $name } is not complete: it needs a client ID and a client secret, and an issuer for a generic provider.
connectivity-provider-saved = { $name } is saved.
connectivity-test-ok = { $name } answered: { $answer }.
connectivity-test-failed = { $name } could not be reached from this server: { $detail }
registration-title = Registration
registration-intro = Whether a visitor may open an account here by themselves. Such an account is a member's: it reads what members read and changes nothing. Giving it another role is an administrator's choice, below.
registration-who = Who may open an account
registration-note-closed = Only an administrator creates accounts, on this page.
registration-note-secret = A visitor who knows the family's secret may open a member's account. Share the secret the way you would a door code.
registration-note-open = Any visitor may open a member's account. Members read the records kept for members.
registration-rotate = Draw a new secret
registration-secret = The family's secret
registration-secret-note = Anyone who has it can open an account while this mode is on. Drawing a new one shuts out the old one; accounts already opened stay.
registration-link = Registration page
registration-quick = Quick ways in
registration-quick-none = No sign-in service is ready: set one up under Connectivity
registration-changed = Last changed by { $by }, { $at }.
registration-unknown = Choose who may open an account.
accounts-kpi-total = Active accounts
accounts-kpi-total-note = { $disabled ->
    [one] { $disabled } disabled
    *[other] { $disabled } disabled
    }
accounts-kpi-admins = Administrators
accounts-kpi-admins-few = Add a second
accounts-kpi-admins-ok = More than one, as it should be
accounts-kpi-linked = Linked to a service
accounts-kpi-linked-note = { $n ->
    [one] { $n } service ready
    *[other] { $n } services ready
    }
accounts-kpi-registered = { $n ->
    [one] { $n } opened by a visitor
    *[other] { $n } opened by visitors
    }
accounts-roles = Roles
accounts-sign-in = Signs in with
accounts-sign-in-password = password
accounts-sign-in-google = Google
accounts-sign-in-microsoft = Microsoft
accounts-sign-in-github = GitHub
accounts-sign-in-oidc = single sign-on
accounts-sign-in-none = no way in yet
accounts-created-via-register = opened by the visitor
accounts-created-via-sso = opened through a service
accounts-add-also-register = Visitors may also open member accounts by themselves, as set above. An account created here can have any role.
login-password-not-allowed = This site's sign-in rules do not let this account use a password: sign in through a service.
login-register = No account yet? Create one
register-title = Create an account
register-lede = A member's account reads the records kept for the family's members. The quickest way is an account you already have.
register-closed = Accounts here are created by an administrator. Ask one of them for yours.
register-have-account = Already have an account? Sign in
register-or = or
register-secret = The family's secret
register-secret-hint = An administrator gave it to you, or someone in the family did.
register-secret-hint-services = Type it before choosing a service, or the next step will ask for it.
register-domains = Only addresses at { $domains } can open an account.
register-username-hint = 2 to 64 letters, digits, dots, dashes or underscores.
register-submit = Create the account
register-member-note = No address is asked for: nothing is sent to you, and nobody can claim yours.
register-form-closed = This site does not open accounts with this form.
register-throttled = Too many attempts from here. Wait a quarter of an hour and try again.
register-secret-wrong = That is not the family's secret.
register-username-invalid = A username has 2 to 64 letters, digits, dots, dashes or underscores.
register-password-short = The password needs at least { $min } characters.
register-username-taken = That username is taken. Choose another.
register-not-saved = The account could not be saved. Try again in a moment.
register-finish-title = One more step
register-finish-lede = The service confirmed who you are. To open an account here, type the family's secret.
register-finish-lede-email = The service confirmed you as { $email }. To open an account here, type the family's secret.
sso-continue = Continue
sso-continue-with = Continue with { $name }
sso-error-title = Sign-in did not complete
sso-account-disabled = The account linked to this sign-in is disabled. Ask an administrator.
sso-no-account = No account here matches this sign-in, and this site does not open accounts by itself. Ask an administrator to create yours with this address: your next sign-in will link to it.
sso-domain-refused = Accounts here can only be opened with an address at the family's domains, confirmed by the service.

# Administration console: the overview's tiles (v0.5.0, docs/adr/0011)
overview-accounts-note = { $admins ->
    [one] { $admins } administrator
    *[other] { $admins } administrators
    }
register-password-hint = At least { $min } characters. A phrase of several words is easier to remember than a jumble.

# Clarity: the light theme and its funnel (v0.5.0, docs/adr/0013)
theme-clarity = Clarity
theme-clarity-note = light, white and orange · a funnel
view-funnel-back = Back to the funnel
view-funnel-open = Open the funnel
view-funnel-hero-1 = Every generation is a ring.
view-funnel-hero-2 = Every family, a funnel.
view-funnel-cta = Explore the funnel
view-funnel-cta-detail = Every name added and every source checked makes the funnel truer for those who come after.
view-funnel-see = See in the funnel
view-funnel-direction = ↓ the oldest at the narrow end · ↑ the newest at the mouth
view-funnel-still = Hold the funnel still
view-funnel-name = Funnel
view-funnel-legend = How to read the funnel
view-funnel-search-hint = Enter brings them to the front; Shift+Enter opens their record.
bundle-binding-unnamed = an unnamed archive
bundle-binding-created = an archive created on { $date }
bundle-binding-named = “{ $family }”
bundle-binding-named-created = “{ $family }”, created on { $date }

# Notes and contributions (docs/adr/0014)
notes-section-title = Notes and contributions
notes-section-intro = What the family's members have written down: memories, research, letters, documents. Search them, browse them by letter, or open one to read it and save it as a PDF.
notes-search-label = Search the notes
notes-search-go = Search
notes-all = All
notes-letters-label = Notes by the first letter of their title
notes-shown = Shown: { $shown } of { $total }
notes-none = No note has been written yet.
notes-none-found = No note matches this search.
notes-write = Write a note
notes-family-wide = The whole family
notes-by = by { $author }
notes-about = About: { $names }
notes-about-prefix = About
notes-about-family = About the whole family
notes-pdf = Download as PDF
notes-edit = Edit
notes-close = Close
notes-back = Back to the notes
notes-add-about = Write a note about this person
notes-sign-in = The family's notes and contributions are for its members. Sign in to read them.
notes-admin-title = Notes and contributions
notes-admin-intro = Write what the records cannot hold: a memory, a piece of research, a letter, with pictures. A note linked to people appears on their records; a note linked to no one belongs to the whole family. Every member reads them all on the Family page.
notes-deleted = The note was deleted.
notes-col-title = Title
notes-col-category = Category
notes-col-date = Date
notes-col-about = About
notes-col-author = Author
notes-col-actions = Actions
notes-new-title = New note
notes-edit-title = Edit the note
notes-section-about = The note
notes-field-title = Title
notes-field-category = Category
notes-field-category-hint = Free text; the categories already used are suggested.
notes-field-date = Date
notes-field-date-hint = A year, a month or a day: 1952, 1952-06 or 1952-06-14. Leave it empty if the note has none.
notes-field-people = People concerned
notes-field-people-hint = Type a name and pick it from the list. With no one named, the note belongs to the whole family.
notes-person-add = Add
notes-person-remove = Remove this person
notes-field-text = Text
notes-text-hint = Bold, italics, headings, lists and links are written in Markdown; the buttons above write them for you. Paste or drop a picture into the text to add it, up to { $mb } MB each.
notes-tool-bold = Bold
notes-tool-italic = Italic
notes-tool-heading = Heading
notes-tool-list = List
notes-tool-quote = Quotation
notes-tool-link = Link
notes-tool-image = Add a picture
notes-tool-preview = Preview
notes-tool-write = Back to writing
notes-uploading = Adding the picture…
notes-save = Save the note
notes-cancel = Cancel
notes-delete = Delete this note
notes-delete-confirm = The note and its pictures will be removed from the archive.
notes-category-suggestions = Memory, Research, Letter, Story, Recipe, Document
notes-error-title = The note needs a title. Nothing was saved.
notes-error-title-long = The title is too long: 200 characters at most. Nothing was saved.
notes-error-text-long = The text is too long. Nothing was saved.
notes-error-date = This date is not understood. Write a year, a month or a day: 1952, 1952-06 or 1952-06-14. Nothing was saved.
notes-error-person = People concerned: “{ $name }”.
notes-error-image-gone = A pasted picture was no longer waiting to be saved; pictures wait six hours. Add it again. Nothing was saved.
notes-error-too-many-images = A note holds 40 pictures at most. Nothing was saved.
notes-error-stale = Someone changed this note while you were working on it. Open it again to see their version, then make your change. Nothing was saved.
notes-error-refused = The archive refused this change. Nothing was saved.
notes-error-image = This file is not a picture this site can show: PNG, JPEG, GIF or WebP.
notes-error-image-large = This picture is too large.
notes-stale-title = The note was not changed
notes-missing-title = Note not found
notes-missing = There is no note here, or it is not yours to read.
notes-not-yours = A contributor changes only the notes they wrote. An administrator can change this one.
notes-pdf-failed-title = The PDF could not be made
notes-pdf-failed = Something went wrong while making the PDF of this note. Read it on its page in the meantime, and tell an administrator.
notes-dashboard-count = Notes written: { $n }
notes-dashboard-open = Write and manage the notes

# The family the archive describes: the bundle room's form (2026-10-23)
kind-bundle = archive
manifest-title = The family
manifest-intro = What the archive says about the family it holds. Its name is the one the Family page, the page titles and every note's PDF show; the rest travels with the file. The figures and dates of the file itself are counted when it is written.
manifest-name = Family name
manifest-name-hint = Shown on the Family page and in the titles. The accounts follow the new name.
manifest-description = Description
manifest-culture = Main culture
manifest-culture-hint = A language tag: fr, pl, pt-BR…
manifest-place = Main place
manifest-place-hint = The continent the family mostly lived on.
manifest-span = Period covered
manifest-span-hint = From the earliest birth to the latest the archive records; counted, and written into the file when the details are saved.
manifest-license = Licence
manifest-license-type = Licence
manifest-save = Save the family's details
manifest-saved = The family's details are saved. The accounts follow its name.
manifest-error-long = A field is too long: 120 characters for the name, 2,000 for the description. Nothing was saved.
manifest-error-culture = The culture is a language tag, such as fr, pl or pt-BR. Nothing was saved.
manifest-error-stale = Someone changed the family's details while you were editing them. Your version is below: saving it again replaces theirs. Nothing was saved.
manifest-error-io = The archive could not be written; the server's log says why. Nothing was saved.

# The family form: places, licences and the period (2026-10-23)
manifest-none = Not said
manifest-span-value = { $earliest } – { $latest }
manifest-span-none = No birth date recorded yet.
manifest-license-pick = Choose a licence to see what it lets others do with the archive.
manifest-license-author = Author
manifest-license-author-value = { $who }, { $at }
manifest-license-author-none = Recorded when a licence is chosen.
manifest-error-place = Choose the main place from the list. Nothing was saved.
manifest-error-licence = Choose the licence from the list. Nothing was saved.
continent-africa = Africa
continent-antarctica = Antarctica
continent-asia = Asia
continent-europe = Europe
continent-north-america = North America
continent-south-america = South America
continent-oceania = Oceania
continent-several = Several continents
licence-all-rights-reserved-name = All rights reserved
licence-all-rights-reserved-about = Nobody may copy, share or reuse the archive without the family's permission. It is what applies when nothing is said.
licence-family-private-name = Private, for the family
licence-family-private-about = Meant for the family's members only: not to be published or passed on outside the family.
licence-cc0-name = CC0 1.0 — public domain
licence-cc0-about = Anyone may copy, change and use the archive for any purpose, even commercially, without asking or naming anyone.
licence-cc-by-name = CC BY 4.0 — Attribution
licence-cc-by-about = Anyone may share and adapt the archive, even commercially, as long as they name its authors and say what they changed.
licence-cc-by-sa-name = CC BY-SA 4.0 — Attribution, ShareAlike
licence-cc-by-sa-about = Anyone may share and adapt it, even commercially, naming its authors; what they make from it must be shared under the same licence.
licence-cc-by-nc-name = CC BY-NC 4.0 — Attribution, NonCommercial
licence-cc-by-nc-about = Anyone may share and adapt it, naming its authors, but not for commercial purposes.
licence-cc-by-nc-sa-name = CC BY-NC-SA 4.0 — Attribution, NonCommercial, ShareAlike
licence-cc-by-nc-sa-about = Anyone may share and adapt it, naming its authors, not for commercial purposes, and only under the same licence.
licence-cc-by-nd-name = CC BY-ND 4.0 — Attribution, NoDerivatives
licence-cc-by-nd-about = Anyone may share it as it is, even commercially, naming its authors, but may not share a changed version.
licence-cc-by-nc-nd-name = CC BY-NC-ND 4.0 — Attribution, NonCommercial, NoDerivatives
licence-cc-by-nc-nd-about = Anyone may share it as it is, naming its authors, but neither for commercial purposes nor in a changed version: the strictest Creative Commons licence.
licence-odbl-name = ODbL 1.0 — Open Database License
licence-odbl-about = Made for databases: anyone may use, share and adapt the data, naming its source, and a database made from it must stay open on the same terms.

# An account's own page and its record in the tree (2026-10-23)
nav-account = My account
account-title = My account
account-who = The account
account-email = Email address
account-since = Since
account-person-title = My record in the tree
account-person-intro = Say which person in the tree you are. Nobody checks it: each account chooses its own record, on its own responsibility, and may change or remove it at any time.
account-person-is = This account says it is
account-person-unlink = Remove the link
account-person-pick = A person in the tree
account-person-link = Link my account to this record
account-person-change = Link to this record instead
account-person-linked = Your account is linked to its record.
account-person-unlinked = Your account is no longer linked to a record.
account-person-error = This record could not be linked:
account-person-io = The account could not be saved; the server's log says why. Nothing was changed.
account-emergency-page = The emergency token is not an account: it has no page of its own and no record in the tree.
accounts-person = Record:
