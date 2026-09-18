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
footer-open-format = Your family’s archive is one file you keep, written in an open format so it will still open long after this site is gone.
footer-open-format-link = About the format

## Preferences

settings-title = Settings
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
prefs-reviewed = reviewed
prefs-machine = machine, { $coverage }%
prefs-machine-complete = complete, not yet reviewed
prefs-machine-title = Translated without review by a native speaker. Genealogical vocabulary especially may be wrong — the words for a union, a godparent or a primary source differ by national record-keeping tradition. Corrections are welcome, and CONTRIBUTING.md says where to start.

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
    }, { $depth } generations each way.
tree-filter-label = Filter visible cards
tree-filter-placeholder = Type a name…
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
person-tab-life = Life
person-tab-media = Media
person-tab-tree = Tree
person-tree-depth = { $n } generations either way. The whole tree is below.
person-tree-alone = This record names no parents, partners or children, so there is no shape to draw around it.

record-no-evidence = Nothing is attached to this record — no source and no document. That is the ordinary state of a converted file rather than a fault in it: GEDCOM carries the facts and leaves behind whatever proved them.
record-no-evidence-signed-out = Sign in to attach one.
admin-delete = Delete
admin-not-set = — not set —
admin-edit = Edit
admin-page-of = Page { $page } of { $pages }
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
accounts-no-registration = There is no self-registration and no invitation flow, deliberately. For a family archive an administrator who knows everyone is enough, and it removes an abuse surface entirely rather than defending one.
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

home-empty = Nothing recorded yet. Import a family file to bring an existing tree across, or add the first person by hand.
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
home-nothing-detail = Import a family file to bring an existing tree across, or start from nothing and add the first person yourself.

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
record-open-full-page = Open full page ↗
record-open-full-title = Open the standalone, shareable page
record-edit = Edit
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
record-caption-placeholder = Caption (optional)
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
identity-culture-placeholder = en
identity-edit-link = Edit names and identity

## Union types, statuses and date precision, said out loud

union-type-marriage = married
union-type-civil_union = civil union
union-type-cohabitation = cohabited
union-type-religious_only = religious union
union-type-polygamous = polygamous
union-type-unknown = not recorded
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
history-created = created
history-deleted = deleted
history-attached = attached a file
admin-raw-json-unparsed = The raw JSON did not parse ({ $error }). Nothing was saved.
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
radar-section-help = Three readings of what this record holds, each axis from 0 to 100. Every number is worked out from the facts listed beside it, by rules written down in the application's documentation; nothing is stored, nothing is guessed, and an axis with nothing to read is left empty rather than given a middling score. The markers and bars say how sure each number is: a filled dot is effectively certain, a ringed dot well supported, a ring plausible, a broken ring speculative, and the longer the bar, the less certain.
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
radar-axis-conscientiousness = Conscientiousness
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
