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
nav-convert = Import
nav-admin = Admin
nav-sign-in = Sign in
nav-sign-out = Sign out
footer-open-format = Your family’s archive is one file you keep, written in an open format so it will still open long after this site is gone.
footer-open-format-link = About the format

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
tree-lede-whole = Every person in the tree. Oldest at the bottom, youngest at the top. Connector opacity is the relationship's confidence.
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
    }, from { $path }. The journal is kept beside the archive rather than inside it: an archive gets copied, mailed and published, and who corrected what is a fact about the people keeping the tree rather than about the family in it.
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
accounts-branch-hint = Limits what this account may edit to those people, their descendants and their spouses. It does not limit what they may read — that is governed by each record's visibility, and the two are kept separate on purpose.
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
tree-contradicts-title = This tree contradicts itself.
tree-contradicts-detail = Somebody is recorded as their own ancestor, or two people on one line of descent are recorded as a couple. No arrangement of rows can satisfy that, so the offending relationship was left out of the generation numbering and some rows may be wrong. Run the validator from the admin dashboard to find it.
tree-no-people = There is nobody in this tree yet.
tree-no-people-cta = Import a family file, or add the first person.
tree-nobody-selected = Nobody to draw for that selection.
tree-nobody-selected-cta = Start from the default view.
tree-click-hint = Click any card to open that person's record in the panel; “Centre the tree here” in the panel re-roots the view.
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
home-what-title = What this does for a family
home-what-archive-title = One place for the whole archive
home-what-archive-body = The tree, the documents and the photographs sit together. A scan of a marriage certificate hangs off the marriage itself, not in somebody’s inbox, and a photograph names the people in it.
home-what-together-title = Several relatives, different roles
home-what-together-body = An aunt with thirty years of notes and a cousin who only wants to correct a spelling do not need the same powers. Each relative is invited with their own role, and every change records who made it and when.
home-what-privacy-title = Privacy decided person by person
home-what-privacy-body = A living relative can be visible to the family and invisible to visitors, while their great-grandmother is open to anyone. The choice is made for each person, not once for the whole tree.
home-what-languages-title = Ten languages
home-what-languages-body = Relatives read the site in their own language, and a name can be kept in its own script beside a transliteration. Nothing has to be flattened into one alphabet for the site to work.
home-what-export-title = The archive stays yours
home-what-export-body = Export the whole thing as a single file whenever you like — people, relationships, documents and photographs together. If you ever decide to leave, you leave with the archive intact.
home-in-this-tree = What the family has recorded so far
home-showcase-title = Where this tree already says more than names and dates
home-showcase-note = Each of these is drawn from what is actually recorded here, not from a list of things the site could do.
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

## Operation results

result-diagnostics = Diagnostics
result-diagnostics-note = Every diagnostic the library returned, including warnings that did not block the operation. None are filtered out.
result-no-diagnostics = The library returned no diagnostics.
result-continue = Continue
result-dashboard = Dashboard
person-sections-label = Sections on this page

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
record-doc-photo = photo
record-doc-certificate = certificate
record-doc-letter = letter
record-doc-record = record
record-doc-newspaper = newspaper
record-doc-other = other
record-upload = Upload
record-upload-help = Up to { $mb } MB per file. Attachments are held beside the tree and written back into the archive when you export, so a photograph travels with the family it belongs to. The kind of file is read from its own contents rather than its name: images, PDF, plain text, audio and video are accepted. SVG is refused, because an SVG can carry a script.
record-upload-help-short = Up to { $mb } MB. SVG is refused.
record-verbatim-note = Kept exactly as the record stated it, because no converter could interpret it. Dropping it would have been the alternative.
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
convert-lede = Bring an existing tree across from a GEDCOM file — the export most genealogy programs produce. You get an archive back to keep. Nothing is stored here, and the tree this site already shows is left exactly as it was.
convert-file-label = Family file (.ged)
convert-file-hint = Up to { $mb } MB. A tree of 767 people is about 320 KB.
convert-confidence-label = How sure these facts are, to begin with
convert-confidence-hint = The file being imported does not say how certain anyone was, so every fact needs a starting point. Set it low for a tree assembled quickly, higher for one worked from documents. The honest reading of this number is “imported, and not checked by anyone since” — you can raise or lower any fact afterwards, one at a time.
convert-lang-label = Language of the place names
convert-lang-hint = A tag such as en, fr or pl. A place can hold its name in several languages; this says which language the names in your file are written in.
convert-what-you-get = What the import adds
convert-what-you-get-1 = Every fact gains a level of certainty you can adjust later, so a doubt can be written down instead of thrown away. Dates keep their shape: circa 1500, before 1430 and between 1920 and 1925 stay three different statements, and wording nobody could read as a date is kept word for word. A job becomes a length of time with a start and an end. Each place becomes an entry of its own, so a town that changed country keeps that history.
convert-no-way-back = Writing a .ged file back out is not offered. That format has nowhere to put how sure a fact is, a relationship outside the family, the length of a job, or a date nobody could pin down — so the return trip would quietly drop them. Your archive exports whole instead, as a single file.

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
convert-skipped-note = These entries held nothing that could be brought across. They are listed rather than swallowed: knowing exactly what was left behind is the difference between an import you can trust and one you cannot.
convert-other-diagnostics = { $n ->
        [one] One other thing worth knowing
       *[other] { $n } other things worth knowing
    }
convert-clean = Nothing was left behind — every entry in the file came across.
convert-download-title = Download
convert-download-named = Download { $name }
convert-download-note = Kept here for fifteen minutes and then discarded, so download it now. That one file is the whole tree; keep it somewhere safe.
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
completeness-intro = What is recorded and what is still blank. Nothing here is an error: a blank row is somewhere the record could grow, not something that has gone wrong.
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
