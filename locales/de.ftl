# axgf-cms — Oberflächentexte, Deutsch.
#
# MASCHINELLE QUALITÄT — nicht von einer Person mit deutscher Muttersprache
# geprüft. Gerade genealogisches Fachvokabular („union", „affiliation",
# „confidence") hat etablierte Entsprechungen, die sich je nach
# Archivtradition unterscheiden. Korrekturen willkommen — siehe CONTRIBUTING.md.
#
# REGEL: Diese Datei übersetzt nur die Oberfläche. Namen, Orte, Notizen und
# Berufe stammen aus der .axgf-Datei und bleiben in ihrer eigenen Sprache und
# Schrift.

app-name = axgf-cms

nav-tree = Stammbaum
nav-convert = GEDCOM umwandeln
nav-admin = Verwaltung
nav-sign-in = Anmelden
nav-sign-out = Abmelden
footer-served-from = Ausgeliefert aus einer einzigen .axgf-Datei. Die gesamte genealogische Logik liegt in axgf-rs; das Format ist in axgf-spec spezifiziert.

prefs-title = Sprache und Darstellung
prefs-language = Sprache
prefs-language-note = Dies ändert nur die Oberfläche. Namen, Orte und Notizen erscheinen stets in ihrer eigenen Sprache und Schrift.
prefs-theme = Darstellung
prefs-apply = Übernehmen
prefs-reviewed = geprüft
prefs-machine = maschinell, { $coverage } %

theme-light = Hell
theme-dark = Dunkel
theme-system = Systemeinstellung folgen
theme-high-contrast = Hoher Kontrast
theme-sepia = Sepia
theme-deuteranopia = Deuteranopie
theme-protanopia = Protanopie
theme-tritanopia = Tritanopie
theme-colour-blind-note = farbfehlsichtigkeitssicher
theme-contrast-note = maximaler Kontrast

tree-title-around = Rund um { $name }
tree-title-whole = Der ganze Stammbaum
tree-lede-focused = { $ancestors ->
        [one] Ein Vorfahre
       *[other] { $ancestors } Vorfahren
    }, { $descendants ->
        [one] ein Nachkomme
       *[other] { $descendants } Nachkommen
    } und { $spouses ->
        [one] ein Partner
       *[other] { $spouses } Partner
    }, { $depth } Generationen in jede Richtung. Die Ältesten unten. Die Deckkraft der Verbindungslinien zeigt die Sicherheit der Beziehung — eine blasse Linie ist eine Aussage, deren sich die Aufzeichnung nicht sicher ist.
tree-lede-whole = Alle Personen der Datei. Die Ältesten unten, die Jüngsten oben. Die Deckkraft der Verbindungslinien zeigt die Sicherheit der Beziehung.
tree-filter-label = Sichtbare Karten filtern
tree-filter-placeholder = Namen eingeben…
tree-centre-on = Zentrieren auf
tree-depth = Generationen in jede Richtung
tree-show = Anzeigen
tree-hidden-notice = { $n ->
        [one] Eine Person wird ohne ihre Angaben gezeigt
       *[other] { $n } Personen werden ohne ihre Angaben gezeigt
    }
tree-hidden-because-role = , weil ihre Sichtbarkeit über dem liegt, was Ihr Konto lesen darf.
tree-hidden-because-anonymous = , weil sie nicht öffentlich sind.
tree-hidden-sign-in = Melden Sie sich an, wenn Sie ein Konto haben.
tree-restricted-card = Der Eintrag dieser Person ist für Sie nicht sichtbar
tree-empty = Diese Datei enthält niemanden zum Zeichnen.
tree-unplaced = In keiner erfassten Familie

record-identity = Identität
record-life-events = Lebensereignisse
record-family = Familie
record-other-relationships = Weitere Beziehungen
record-occupations = Berufe
record-places = Orte
record-sources-documents = Quellen und Dokumente
record-notes = Notizen
record-history = Verlauf
record-raw = Rohdaten
record-raw-summary-note = das JSON, aus dem diese Seite erzeugt wurde
record-sources-documents-help = Jede Quelle nennt die Aussagen auf dieser Seite, die auf ihr beruhen, geordnet nach der Stärke des Belegs.
record-notes-help = Notizen zu diesem Eintrag, einschließlich Text, den kein Umwandler deuten konnte und der wörtlich bewahrt statt verworfen wurde.
record-help-toggle = Was dieser Abschnitt zeigt

record-gender = Geschlecht
record-living = Lebend
record-visibility = Sichtbarkeit
record-yes = ja
record-no = nein
record-name-type = Namensart
record-name-used = Geführt
record-name-evidence = Beleg
record-transliteration = Lateinische Umschrift
record-born = Geboren
record-died = Gestorben
record-parents = Eltern
record-siblings = Geschwister
record-children = Kinder
record-unknown-person = [Unbekannt]
record-restricted-person = Privat
record-restricted-title = Der Eintrag dieser Person ist für Sie nicht sichtbar
record-absent-person-title = In dieser Datei erwähnt, aber nicht darin enthalten
record-confidence = Sicherheit
record-source = Quelle
record-download = Herunterladen

access-restricted-title = Für Sie nicht sichtbar
access-restricted-anonymous = Dieser Eintrag ist nicht öffentlich. Melden Sie sich an, um zu sehen, ob Ihr Konto ihn lesen darf.
access-role-title = Nicht für Ihre Rolle
access-role-write = Ihr Konto darf diese Datei lesen, aber nicht ändern. Ein Administrator kann Ihre Rolle auf Beitragende anheben.
access-scope-title = Außerhalb Ihres Zweigs

error-not-found-title = Nicht gefunden
error-not-found-detail = Diese Seite gibt es in dieser Datei nicht.
error-no-such-person-title = Keine solche Person
error-no-such-person-detail = Diese Datei enthält keine Person mit dieser Kennung.
error-no-such-entity-title = Kein solcher Eintrag
error-no-such-entity-detail = Diese Datei enthält keinen Eintrag mit dieser Kennung.
error-deleted-while-editing = Diese Datei enthält keinen Eintrag mit dieser Kennung. Er wurde möglicherweise gelöscht, während Sie ihn bearbeitet haben.
error-no-such-file-title = Keine solche Datei
error-not-an-image-title = Kein Bild
error-not-an-image-detail = Für dieses Dokument gibt es keine Vorschau, da es kein Bild ist, das diese Fassung lesen kann.
error-back = Zurück

login-title = Anmelden
login-lede = Konten werden von einem Administrator angelegt.
login-username = Benutzername
login-password = Passwort
login-submit = Anmelden
login-wrong = Benutzername und Passwort passen nicht zusammen.
login-token-wrong = Dieses Token ist nicht richtig.
login-throttled = Zu viele Fehlversuche. Warten Sie einige Minuten und versuchen Sie es erneut.
login-no-accounts-title = Diese Installation hat noch keine Konten.
login-emergency-summary = Notzugang
login-emergency-label = Not-Token
login-emergency-submit = Not-Token verwenden
login-sign-in-prompt = Melden Sie sich an, um die Verwaltung zu erreichen.

admin-title = Verwaltung
admin-entities = Einträge
admin-create = Anlegen
admin-new-kind = Neu: { $kind }
admin-operations = Vorgänge
admin-validate = Prüfen
admin-deduplicate = Duplikate zusammenführen
admin-export = Datei exportieren
admin-accounts = Konten
admin-dedup-confirm = Das Zusammenführen von Duplikaten verschmilzt Einträge und schreibt die Datei neu. Fortfahren?
admin-recent-changes = Letzte Änderungen
admin-sessions-open = { $n ->
        [one] Eine Sitzung ist derzeit offen.
       *[other] { $n } Sitzungen sind derzeit offen.
    }
admin-no-changes-yet = Über diese Anwendung wurde noch nichts geändert. Jede Speicherung ab jetzt wird in { $path } festgehalten.
admin-last-validation = Letzte Prüfung
admin-fields = Felder
admin-raw-json = Roh-JSON
admin-save = Speichern
admin-cancel = Abbrechen
admin-delete = Löschen
admin-not-set = — nicht gesetzt —
admin-edit = Bearbeiten
admin-page-of = Seite { $page } von { $pages }
admin-previous = Zurück
admin-next = Weiter
admin-saved = Als Fassung { $version } gespeichert — { $summary }
admin-not-saved = Nicht gespeichert
admin-created = Angelegt
admin-not-created = Nicht angelegt
admin-deleted = Gelöscht
admin-not-deleted = Nicht gelöscht — die Datei ist unverändert
admin-what-changed = was sich geändert hat
admin-field = Feld
admin-from = Von
admin-to = Auf
admin-version = Fassung { $version }

accounts-title = Konten
accounts-existing = Vorhanden
accounts-username = Benutzername
accounts-role = Rolle
accounts-status = Status
accounts-branch = Zweig
accounts-last-seen = Zuletzt gesehen
accounts-change = Ändern
accounts-you = (Sie)
accounts-active = aktiv
accounts-disabled = deaktiviert
accounts-never = nie
accounts-whole-tree = ganzer Stammbaum
accounts-roots = { $n ->
        [one] eine Wurzel
       *[other] { $n } Wurzeln
    }
accounts-add = Konto hinzufügen
accounts-password-hint = Leer lassen, dann wird eines erzeugt und einmalig angezeigt. Mindestens { $min } Zeichen, wenn Sie es selbst festlegen.
accounts-new-password-placeholder = neues Passwort (leer = unverändert)
accounts-email = E-Mail
accounts-optional = (optional)
accounts-create = Konto anlegen
accounts-role-viewer = Leser — liest öffentliche und Mitglieder-Einträge
accounts-role-contributor = Beitragende — legt zudem an, bearbeitet und lädt hoch
accounts-role-admin = Administrator — verwaltet zudem Konten, löscht und exportiert
accounts-branch-placeholder = eine Personenkennung je Zeile
accounts-ids-in-bundle = Personenkennungen in dieser Datei
accounts-created = { $username } angelegt.
accounts-updated = { $username } geändert. Alle offenen Sitzungen wurden abgemeldet.
accounts-username-taken = Dieser Benutzername ist vergeben.
accounts-pick-role = Wählen Sie eine Rolle.
accounts-no-such = Kein solches Konto.
accounts-not-saved = Nicht gespeichert: { $error }

conflict-title = Jemand anderes hat dies zuerst geändert
conflict-versions = Sie sind von Fassung { $expected } ausgegangen; die Datei enthält nun Fassung { $current }.
conflict-both-changed = Das haben Sie beide geändert
conflict-both-changed-detail = Diese Felder wurden von Ihnen beiden bearbeitet. Was Sie speichern, ersetzt, was { $who } dort eingetragen hat:
conflict-field-by-field = Feld für Feld
conflict-theirs = Worauf { $who } es geändert hat
conflict-yours = Worauf Sie es geändert haben
conflict-unchanged-by-you = von Ihnen unverändert
conflict-unchanged-by-them = von ihnen unverändert
conflict-what-now = Wie weiter
conflict-reapply = Ihre Fassung auf ihre anwenden
conflict-save-over = Dies über ihre speichern
conflict-discard = Meine verwerfen und neu beginnen
conflict-their-version = Die Fassung von { $who }, so wie die Datei sie derzeit enthält
conflict-history-of = Verlauf dieses Eintrags ({ $kind })

home-why-title = Warum AXGF

convert-title = GEDCOM in AXGF umwandeln
convert-submit = Umwandeln
convert-result-title = Ergebnis der Umwandlung
convert-download = Die .axgf-Datei herunterladen

completeness-title = Vollständigkeit der Datei
completeness-empty = leer
completeness-spec-field = AXGF-Feld

## Dates

date-unknown = Datum unbekannt
date-not-recorded = Nicht erfasst
date-circa = um { $date }
date-between = zwischen { $from } und { $to }
date-before = vor { $date }
date-after = nach { $date }
date-preserved = erfasst als „{ $text }“
date-day-month-year = { $day }. { $month } { $year }
date-month-year = { $month } { $year }
date-decade = die { $decade }er Jahre
date-century = das { $century }. Jahrhundert

month-1 = Januar
month-2 = Februar
month-3 = März
month-4 = April
month-5 = Mai
month-6 = Juni
month-7 = Juli
month-8 = August
month-9 = September
month-10 = Oktober
month-11 = November
month-12 = Dezember
