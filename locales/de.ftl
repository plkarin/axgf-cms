# axgf-cms — Oberflächentexte, Deutsch.
#
# MASCHINENQUALITÄT — nicht von einer Person mit Deutsch als Muttersprache
# geprüft. Gerade das genealogische Vokabular hat feste Entsprechungen, die je
# nach Archivtradition abweichen, und diese Übersetzung kann falsch sein.
# Korrekturen sind willkommen — siehe CONTRIBUTING.md.
#
# Gewählte Entsprechungen (von Muttersprachlern gern zu bestreiten):
#   union → Verbindung · link → Beziehung · confidence → Sicherheit
#   reliability → Zuverlässigkeit · source → Quelle
#   primary source → Primärquelle · occupation → Beruf · record → Eintrag
#   archive → Archiv · godparent → Pate · witness → Zeuge
#   speculative → vermutet
#
# Plural: CLDR-Regeln one / other. Niemals durch eine eigene „eins oder mehr“-
# Logik ersetzen.
#
# Datum: „12. April 1923“ — Tag mit Punkt, Monat ausgeschrieben, Jahr ohne
# Komma. Die Monatstabelle steht im Datumsmuster selbst.
#
# REGEL: Diese Datei übersetzt nur die Oberfläche. Namen, Orte, Notizen und
# Berufsbezeichnungen kommen aus dem Archiv und bleiben in ihrer Sprache und
# Schrift.

app-name = ax-genealogy

## Kopf- und Fußzeile

nav-tree = Stammbaum
nav-convert = Import
nav-admin = Verwaltung
nav-sign-in = Anmelden
nav-sign-out = Abmelden
footer-open-format = Das Archiv Ihrer Familie ist eine einzige Datei, die bei Ihnen bleibt, in einem offenen Format geschrieben — sie lässt sich noch lange öffnen, wenn es diese Website nicht mehr gibt.
footer-open-format-link = Über das Format

## Einstellungen

prefs-title = Sprache und Darstellung
prefs-language = Sprache
prefs-theme = Darstellung
prefs-background = Hintergrund
prefs-background-on = Sanfter Farbverlauf hinter der Seite
prefs-apply = Übernehmen
prefs-reviewed = geprüft
prefs-machine = maschinell, { $coverage } %
prefs-machine-complete = vollständig, noch ungeprüft
prefs-machine-title = Ohne Prüfung durch eine Person mit dieser Muttersprache übersetzt. Besonders das genealogische Vokabular kann falsch sein — die Wörter für eine Verbindung, einen Paten oder eine Primärquelle unterscheiden sich je nach Archivtradition des Landes. Korrekturen sind willkommen, und CONTRIBUTING.md sagt, wo man anfängt.

theme-light = Hell
theme-dark = Dunkel
theme-system = Wie im System
theme-high-contrast = Hoher Kontrast
theme-sepia = Sepia
theme-deuteranopia = Deuteranopie
theme-protanopia = Protanopie
theme-tritanopia = Tritanopie
theme-colour-blind-note = farbfehlsichtigkeitssicher
theme-contrast-note = maximaler Kontrast

## Stammbaum

tree-title-around = Rund um { $name }
tree-title-whole = Der ganze Stammbaum
tree-lede-focused = { $ancestors ->
        [one] Ein Vorfahr
       *[other] { $ancestors } Vorfahren
    }, { $descendants ->
        [one] ein Nachfahre
       *[other] { $descendants } Nachfahren
    } und { $spouses ->
        [one] ein Partner
       *[other] { $spouses } Partner
    }, { $depth } Generationen in jede Richtung.
tree-filter-label = Sichtbare Karten filtern
tree-filter-placeholder = Namen eingeben …
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
tree-restricted-card = Dieser Eintrag ist für Sie nicht sichtbar
tree-empty = Es gibt noch niemanden zu zeichnen.
tree-unplaced = In keiner erfassten Familie

## Der Eintrag

record-identity = Identität
record-life-events = Ereignisse im Leben
record-family = Familie
record-other-relationships = Weitere Beziehungen
record-occupations = Berufe
record-places = Orte
record-sources-documents = Quellen und Dokumente
record-notes = Notizen
record-history = Änderungsverlauf
record-raw = Rohdaten
record-raw-summary-note = das JSON, aus dem diese Seite gebaut wurde

record-identity-help = Jeder erfasste Name mit seiner Art, dem Zeitraum seiner Verwendung und der Quelle dahinter, mit der eigenen Schrift neben der lateinischen Umschrift, wo beide sich unterscheiden, dazu Geschlecht, Lebensstatus und Sichtbarkeit.
record-life-events-help = Geburt, Tod und jedes Ereignis, an dem diese Person beteiligt war, nach Datum geordnet, jeweils mit ihrer Rolle — so steht eine Hochzeit, bei der sie nur Zeuge war, neben ihrer eigenen. Eine Angabe ohne Datum steht zuletzt, statt so zu tun, als käme sie zuerst.
record-family-help = Eltern und Geschwister, dann jede Verbindung mit Art, Daten, Ort, Ende und Kindern in der Reihenfolge ihrer Geburt.
record-other-relationships-help = Jede Beziehung mit dieser Person an einem der Enden, von ihrer Seite gelesen — derselbe Eintrag erscheint am einen Ende als „Pate von“ und am anderen als „Patenkind von“.
record-occupations-help = Berufe als Zeiträume auf einer gemeinsamen Achse, damit zwei Stellen sich mit dem Auge vergleichen lassen; wo eine Grenze fehlt, bleibt der Balken offen.
record-places-help = Jeder Ort, den dieser Eintrag berührt, mit dem, was dort geschah, und mit der Grenzgeschichte, die einen Ort über die Zeit hinweg erst verständlich macht.
record-sources-documents-help = Jede Quelle nennt die Angaben dieser Seite, die auf ihr beruhen, geordnet nach der Stärke des Belegs.
record-notes-help = Notizen zu diesem Eintrag, einschließlich Text, den kein Konverter deuten konnte und der wörtlich erhalten blieb, statt verworfen zu werden.
record-history-help = Jede gespeicherte Änderung an diesem Eintrag, die neueste zuerst. Wer was korrigiert hat, ist eine Tatsache über die Personen, die den Stammbaum pflegen, nicht über die Familie darin — deshalb bleibt sie außerhalb des ausgegebenen Archivs und wird nur angemeldeten Verwandten gezeigt.
record-raw-help = Nichts hier ist für die Anzeige erzeugt: Das ist der Eintrag genau so, wie er gespeichert ist, bis hin zu den Feldnamen. Müssten Sie das Archiv je ohne diese Website lesen, sähen Sie genau das.
record-help-toggle = Was dieser Abschnitt zeigt

record-gender = Geschlecht
record-living = Lebt
record-visibility = Sichtbarkeit
record-yes = ja
record-no = nein
record-name-type = Namensart
record-name-used = Verwendet
record-name-evidence = Beleg
record-transliteration = Lateinische Umschrift
record-born = Geboren
record-died = Gestorben
record-parents = Eltern
record-siblings = Geschwister
record-children = Kinder
record-unknown-person = [Unbekannt]
record-restricted-person = Privat
record-restricted-title = Dieser Eintrag ist für Sie nicht sichtbar
record-absent-person-title = In diesem Stammbaum genannt, aber nicht darin erfasst
record-confidence = Sicherheit
record-source = Quelle
record-download = Herunterladen

## Zugriff

access-restricted-title = Für Sie nicht sichtbar
access-restricted-signed-in = Die Sichtbarkeit dieses Eintrags liegt über dem, was Ihr Konto lesen darf. Eine Verwaltung kann entweder die Sichtbarkeit des Eintrags oder Ihre Rolle ändern.
access-restricted-anonymous = Dieser Eintrag ist nicht öffentlich. Melden Sie sich an, um zu sehen, ob Ihr Konto ihn lesen darf.
access-role-title = Nicht für Ihre Rolle
access-role-admin = Das ist eine Seite der Verwaltung. Ihr Konto kann Einträge anlegen und bearbeiten, aber keine Konten verwalten, keine Einträge löschen und das Archiv nicht ausgeben.
access-role-write = Ihr Konto darf diesen Stammbaum lesen, aber nicht ändern. Eine Verwaltung kann Ihre Rolle auf Mitwirkende anheben.
access-scope-title = Außerhalb Ihres Zweigs
access-scope-named = Ihr Konto ist auf einen Zweig des Stammbaums beschränkt, und dieser Eintrag betrifft jemanden außerhalb davon. Jede in einem Eintrag genannte Person muss innerhalb Ihres Zweigs liegen — eine Familie mit einem Partner von außen wäre sonst ein Weg, die Abstammung dieser Person umzuschreiben.
access-scope-unnamed = Ihr Konto ist auf einen Zweig des Stammbaums beschränkt, und dieser Eintrag nennt niemanden, an dem er sich messen ließe. Quellen und Orte bearbeiten Konten mit Zugriff auf den ganzen Stammbaum.

## Fehler

error-not-found-title = Nicht gefunden
error-not-found-detail = Diese Seite gibt es hier nicht.
error-no-such-person-title = Keine solche Person
error-no-such-person-detail = Hier gibt es keine Person mit dieser Kennung.
error-no-such-entity-title = Kein solcher Datensatz
error-no-such-entity-detail = Hier gibt es keinen Eintrag mit dieser Kennung.
error-deleted-while-editing = Hier gibt es keinen Eintrag mit dieser Kennung. Möglicherweise wurde er gelöscht, während Sie ihn bearbeitet haben.
error-no-such-file-title = Keine solche Datei
error-no-such-file-detail = Hier gibt es kein Dokument mit dieser Kennung, oder das Dokument ist ohne Datei erfasst — ein verwiesenes Dokument benennt etwas, das anderswo liegt.
error-not-an-image-title = Kein Bild
error-not-an-image-detail = Für dieses Dokument gibt es kein Vorschaubild, weil es kein Bild ist, das dieser Stand entschlüsseln kann.
error-back = Zurück

## Anmeldung

login-title = Anmelden
login-lede = Konten legt die Verwaltung an.
login-username = Benutzername
login-password = Passwort
login-submit = Anmelden
login-wrong = Benutzername und Passwort passen nicht zusammen.
login-token-wrong = Dieses Token ist nicht richtig.
login-throttled = Zu viele Fehlversuche. Warten Sie ein paar Minuten und versuchen Sie es erneut.
login-no-accounts-title = Diese Installation hat noch keine Konten.
login-no-accounts-detail = Eine Einrichtungsseite gibt es hier bewusst nicht — die Lücke zwischen Inbetriebnahme und erster Anmeldung ist genau der Moment, in dem eine Installation ungeschützt ist, deshalb wird die erste Verwaltung auf der Kommandozeile angelegt.
login-no-accounts-note = Sie gibt ein erzeugtes Passwort einmal auf stderr aus und nie wieder. Bis dahin ist der einzige Weg hinein das Notfall-Token unten.
login-emergency-summary = Notzugang
login-emergency-detail = Das gemeinsame Token öffnet weiterhin eine Verwaltungssitzung und existiert zu einem einzigen Zweck: wieder hineinzukommen, wenn die .acl-Datei verloren ist oder alle Verwaltungen ausgesperrt sind. Es ist kein Konto — es hat keine eigenen Einstellungen, und das Änderungsjournal führt es als emergency-token statt als Person. Seine Verwendung wird als Warnung protokolliert.
login-emergency-label = Notfall-Token
login-emergency-submit = Notfall-Token verwenden
login-sign-in-prompt = Melden Sie sich an, um in die Verwaltung zu gelangen.

## Verwaltung

admin-title = Verwaltung
admin-lede = Bearbeitet wird { $path } — { $total } Datensätze, { $files ->
        [one] eine angehängte Datei
       *[other] { $files } angehängte Dateien
    }, { $size } auf der Festplatte. Jede Änderung wird in einem Zug geschrieben; eine abgelehnte Änderung lässt die Datei unberührt.
admin-entities = Datensätze
admin-create = Anlegen
admin-new-kind = Neu: { $kind }
admin-operations = Vorgänge
admin-validate = Prüfen
admin-deduplicate = Doppelte zusammenführen
admin-export = Archiv ausgeben
admin-accounts = Konten
admin-roles-note = Prüfen, Zusammenführen, Ausgeben, Löschen und Kontenverwaltung sind allein der Verwaltung vorbehalten. Mitwirkende erreichen jede andere Seite hier.
admin-dedup-confirm = Das Zusammenführen verschmilzt Einträge und schreibt das Archiv neu. Fortfahren?
admin-recent-changes = Letzte Änderungen
admin-recent-note = Die letzten { $shown } von { $total ->
        [one] einer erfassten Änderung
       *[other] { $total } erfassten Änderungen
    }, aus { $path }.
admin-sessions-open = { $n ->
        [one] Eine Sitzung ist gerade offen.
       *[other] { $n } Sitzungen sind gerade offen.
    }
admin-no-changes-yet = Über diese Anwendung wurde noch nichts geändert. Jede Speicherung von jetzt an wird in { $path } festgehalten.
admin-last-validation = Letzte Prüfung
admin-bundle-heavy = Dieses Archiv ist { $size } groß. Es wird beim Start vollständig geladen und im Speicher gehalten, also kostet die Website ab etwa { $warn } echten Speicher, und Neustarts werden langsam. Das passt zu einem Familienarchiv, nicht zu einer Mediathek — wenn die Anhänge unbegrenzt wachsen, legen Sie sie in einen Dateispeicher und lassen Sie das Archiv darauf verweisen.

admin-fields = Felder
admin-raw-json = Roh-JSON
admin-raw-json-help = Der ganze Datensatz, damit nichts unbearbeitbar ist — Listen wie die Partner und Kinder einer Familie oder die Grenzgeschichte eines Ortes stehen genau hier. Das ist das Ausgangsdokument; die Felder darüber werden anschließend über die ihnen gehörenden Pfade geschrieben, bearbeiten Sie einen Wert also an der einen oder der anderen Stelle, nicht an beiden. Es muss sich als JSON lesen lassen, sonst wird nichts gespeichert.
admin-save = Speichern
admin-cancel = Abbrechen
place-editor-title = Ort bearbeiten
place-add-detail = Diesen Ort ergänzen
place-names = Namen
place-name-primary = Primär
place-name-lang = Sprache
place-name-value = Name
place-names-hint = One row per recorded name. A place administered by three empires carries three names; the primary is the one shown everywhere else.
place-where = Lage
place-type = Art
place-region = Region
place-country-current = Land heute
place-country-hint = ISO 3166-1 alpha-2, e.g. PL, FR, DE.
place-country-history = Grenzverlauf
place-history-country = Staat
place-history-from = Von
place-history-until = Bis
place-country-history-hint = Which state held this place over which period. Genealogically significant: a record written in Russian in 1880 and one written in Polish in 1930 can name the same village.
place-coordinates = Koordinaten
place-lat = Breite
place-lon = Länge
place-precision = Genauigkeit
place-identifiers = Kennungen
place-wikidata = Wikidata
place-geonames = GeoNames
place-used-by = { $name } wird von { $n ->
        [one] einem weiteren Eintrag
       *[other] { $n } weiteren Einträgen
    } verwendet.
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

place-coordinates-hint = Von Hand eingetragen ist der Normalfall. Viele unter einer früheren Verwaltung erfasste Orte sind über eine moderne Suche gar nicht auffindbar.
place-geocode-search = Diesen Namen nachschlagen
place-geocode-hint = Sendet Name, Region und Land an den Geokodierungsdienst, immer nur einen Ort. Nichts wird gespeichert, bis Sie speichern.
place-geocode-off = Die Namenssuche ist aus. Sie braucht eine Kontaktadresse, an der der Dienst diese Installation erkennt; starten Sie den Server mit --geocoder-contact, um sie einzuschalten.
place-geocode-query = Gesucht wurde: { $q }
place-geocode-error = Der Suchdienst war nicht erreichbar. Die Koordinatenfelder oben funktionieren weiterhin.
place-geocode-none = Nichts gefunden. Für ein unter russischer, preußischer oder österreichischer Verwaltung erfasstes Dorf ist das der Normalfall; tragen Sie die Position von Hand ein.
place-geocode-not-a-place = keine Siedlung
place-geocode-use = Diesen übernehmen
place-geocode-attribution = Ergebnisse von OpenStreetMap über Nominatim, unter der Open Database License.

place-paste = Position einfügen
place-paste-placeholder = ein Kartenlink oder 52.0782795, 21.2508068
place-paste-read = Auslesen
place-paste-hint = Ein Google-Maps- oder OpenStreetMap-Link, eine geo:-URI, ein einfaches Zahlenpaar oder Grad-Minuten-Sekunden wie 52°04'41.8"N 21°15'02.9"E.
place-paste-read-ok = In die Felder oben übernommen. Prüfen Sie es und speichern Sie dann.
place-paste-unreadable = Das ist keine Position, die sich hier lesen lässt. Die Felder oben nehmen weiterhin ein einfaches Zahlenpaar.

place-map-hint = Klicken Sie in die Karte, um den Punkt zu setzen, oder ziehen Sie die Nadel. Maßgeblich sind die Felder oben.
place-map-clear = Punkt entfernen
place-open-in-map = Diesen Ort in OpenStreetMap suchen und den Link zurück einfügen

person-tab-record = Eintrag
person-tab-life = Leben
person-tab-media = Medien
person-tab-tree = Stammbaum
person-tree-depth = { $n } Generationen in jede Richtung. Der ganze Baum steht darunter.
person-tree-alone = Dieser Eintrag nennt weder Eltern noch Partner noch Kinder, also gibt es um ihn herum keine Form zu zeichnen.
admin-delete = Löschen
admin-not-set = — nicht gesetzt —
admin-edit = Bearbeiten
admin-page-of = Seite { $page } von { $pages }
admin-previous = Zurück
admin-next = Weiter
admin-saved = Gespeichert als Fassung { $version } — { $summary }
admin-not-saved = Nicht gespeichert
admin-created = Angelegt
admin-not-created = Nicht angelegt
admin-deleted = Gelöscht
admin-not-deleted = Nicht gelöscht — nichts wurde geändert
admin-what-changed = was sich geändert hat
admin-field = Feld
admin-from = Von
admin-to = Auf
admin-version = Fassung { $version }

## Konten

accounts-title = Konten
accounts-lede = Gespeichert in { $path }, mit Rechten 600, neben dem Archiv und niemals darin. Ein Archiv wird kopiert, verschickt und veröffentlicht; Passwort-Hashes darin würden jede Kopie des Stammbaums zu einer Kopie der Zugangsdaten machen.
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
accounts-no-registration = Es gibt bewusst weder Selbstregistrierung noch Einladungen. Für ein Familienarchiv genügt eine Verwaltung, die alle kennt, und das nimmt eine Angriffsfläche ganz weg, statt sie zu verteidigen.
accounts-password-hint = Leer lassen, dann wird eines erzeugt und einmal angezeigt. Mindestens { $min } Zeichen, wenn Sie es selbst setzen.
accounts-new-password-placeholder = neues Passwort (leer = beibehalten)
accounts-email = E-Mail
accounts-optional = (optional)
accounts-create = Konto anlegen
accounts-role-viewer = Lesend — liest öffentliche Einträge und die der Familie
accounts-role-contributor = Mitwirkend — legt außerdem an, bearbeitet und lädt hoch
accounts-role-admin = Verwaltung — verwaltet außerdem Konten, löscht und gibt aus
accounts-branch-hint = Beschränkt, was dieses Konto bearbeiten darf, auf diese Personen, ihre Nachfahren und ihre Ehepartner.
accounts-branch-reading = Es beschränkt nicht, was sie lesen dürfen — das regelt die Sichtbarkeit jedes Eintrags, und beides wird absichtlich getrennt gehalten.
accounts-branch-placeholder = eine Personenkennung je Zeile
accounts-ids-in-bundle = Personenkennungen in diesem Stammbaum
accounts-emergency-warning = Sie sind mit dem Notfall-Token angemeldet. Es gewährt für diese Sitzung Verwaltungsrechte, ist aber kein Konto: Es hat keine eigenen Einstellungen, und das Änderungsjournal führt Ihre Änderungen als emergency-token statt als Person. Legen Sie sich unten ein richtiges Konto an und melden Sie sich damit an.
accounts-created-with-password = { $username } angelegt. Das Passwort lautet { $password } — es wird einmal gezeigt und nur als Argon2id-Hash gespeichert, geben Sie es also jetzt weiter.
accounts-created = { $username } angelegt.
accounts-updated = { $username } aktualisiert. Alle offenen Sitzungen dieses Kontos wurden beendet.
accounts-username-taken = Dieser Benutzername ist vergeben.
accounts-pick-role = Wählen Sie eine Rolle.
accounts-no-such = Dieses Konto gibt es nicht.
accounts-last-admin = Das ist die einzige aktive Verwaltung. Befördern Sie zuerst jemand anderen — eine Installation ohne Verwaltung lässt sich nur durch Bearbeiten der .acl-Datei oder mit dem Notfall-Token zurückholen.
accounts-not-saved = Nicht gespeichert: { $error }

## Konflikte

conflict-title = Jemand anderes war zuerst
conflict-lede = { $who } hat um { $when } eine Änderung an diesem Datensatz ({ $kind }) gespeichert, nachdem Sie ihn geöffnet hatten. Ihre Bearbeitung wurde nicht gespeichert, und nichts wurde überschrieben.
conflict-no-merge = Hier wird nichts automatisch zusammengeführt. Das Verschmelzen der Bearbeitungen zweier Menschen ergibt einen Eintrag, den keiner von beiden gewählt hat, und in der Genealogie heißt Streit zweier Bearbeiter über ein Datum meist, dass sie verschiedene Quellen lesen — und das ist eine Frage an einen Menschen, nicht an ein Programm. Vergleichen Sie beide unten und entscheiden Sie.
conflict-versions = Sie sind von Fassung { $expected } ausgegangen; der Eintrag steht jetzt auf Fassung { $current }.
conflict-both-changed = Das haben Sie beide geändert
conflict-both-changed-detail = Diese Felder haben Sie beide bearbeitet. Was Sie auch speichern, es ersetzt das, was { $who } dort eingetragen hat:
conflict-different-fields = Sie haben verschiedene Felder geändert, es steht also nichts von der Arbeit von { $who } in Frage — aber ein erneutes Anwenden schreibt trotzdem Ihren ganzen Datensatz über den ihren. Prüfen Sie beide Spalten vor dem Speichern.
conflict-field-by-field = Feld für Feld
conflict-theirs = Worauf { $who } es geändert hat
conflict-yours = Worauf Sie es geändert haben
conflict-unchanged-by-you = von Ihnen nicht geändert
conflict-unchanged-by-them = von ihnen nicht geändert
conflict-nothing-differs = Keine der beiden Fassungen unterscheidet sich in einem auf dieser Seite gezeigten Feld von der, mit der Sie begonnen haben. Die Fassungsnummer ist weitergerückt, jemand hat den Eintrag also gespeichert, ohne etwas daran zu ändern.
conflict-what-now = Wie weiter
conflict-reapply = Ihre Fassung auf die ihre anwenden
conflict-reapply-hint = Das ist Ihre Bearbeitung, übertragen auf Fassung { $version }. Passen Sie sie hier an, um von der Arbeit von { $who } zu behalten, was Sie wollen, und speichern Sie dann. Deren Fassung steht unten zum Abschreiben.
conflict-save-over = Das über die ihre speichern
conflict-discard = Meine verwerfen und neu anfangen
conflict-their-version = Die Fassung von { $who }, so wie sie jetzt steht
conflict-history-of = Verlauf dieses Datensatzes ({ $kind })

## Import

convert-title = Familiendatei importieren
convert-submit = Importieren
convert-result-title = Importbericht
convert-download = Archiv herunterladen

## Datum

date-unknown = Datum unbekannt
date-not-recorded = Nicht erfasst
date-circa = um { $date }
date-between = zwischen { $from } und { $to }
date-before = vor { $date }
date-after = nach { $date }
date-preserved = erfasst als „{ $text }“
date-day-month-year = { $day }. { $month ->
        [1] Januar
        [2] Februar
        [3] März
        [4] April
        [5] Mai
        [6] Juni
        [7] Juli
        [8] August
        [9] September
        [10] Oktober
        [11] November
        [12] Dezember
        *[other] { $month }
    } { $year }
date-month-year = { $month ->
        [1] Januar
        [2] Februar
        [3] März
        [4] April
        [5] Mai
        [6] Juni
        [7] Juli
        [8] August
        [9] September
        [10] Oktober
        [11] November
        [12] Dezember
        *[other] { $month }
    } { $year }
date-decade = die { $decade }er Jahre
date-century = das { $century }. Jahrhundert
date-quarter-century = das { $quarter ->
        [1] erste
        [2] zweite
        [3] dritte
       *[other] vierte
    } Viertel des { $century }. Jahrhunderts

## Weitere Fehlerseiten

error-back-to-start = Zurück zum Anfang
error-payload-missing-title = Keine solche Datei
error-payload-missing-detail = Der Inhalt dieses Dokuments liegt nicht im Zwischenspeicher.
error-payload-unopenable-detail = Der Inhalt dieses Dokuments ließ sich nicht öffnen.
error-no-such-document-detail = Hier gibt es kein Dokument mit dieser Kennung.
error-bad-preference-title = Nicht eine der Möglichkeiten
error-bad-preference-detail = Das ist weder eine Sprache noch eine Darstellung, die diese Website anbietet. Nichts wurde geändert.
error-unknown-kind-title = Unbekannte Art
error-unknown-kind-detail = „{ $kind }“ ist keine Art von Eintrag. Dieses Archiv enthält: { $kinds }.
error-io-title = Speichern nicht möglich
error-io-detail = { $error }. Auf der Festplatte wurde nichts geändert.
error-upload-too-large = Diese Datei ist größer als die Grenze von { $mb } MB. Nichts wurde gespeichert, und das Archiv ist unverändert.
error-upload-refused = Das Dokument wurde abgelehnt: { $reason }. Das Archiv ist unverändert.
error-back-to-person = Zurück zum Eintrag
error-no-such-person-to-attach = Hier gibt es keine Person mit dieser Kennung, also gibt es auch nichts, woran ein Dokument gehängt werden könnte.
error-upload-title = Dieser Upload wurde nicht gespeichert
error-download-expired-title = Dieser Download ist abgelaufen
error-download-expired-detail = Ein Import wird fünfzehn Minuten aufbewahrt und dann verworfen. Importieren Sie die Datei erneut.
error-upload-none = Es wurde keine Datei hochgeladen. Wählen Sie zuerst eine Datei.
error-upload-unsupported = Diese Art von Datei bewahrt das Archiv nicht auf. Angenommen werden Bilder, PDF, einfacher Text, Ton und Video; die Art wird aus den Bytes der Datei selbst gelesen, das Umbenennen eines ausführbaren Programms hilft also nicht. SVG wird rundheraus abgelehnt, weil ein SVG ein Skript enthalten kann.
error-export-unreadable-title = Das ausgegebene Archiv ließ sich nicht lesen
error-export-unreadable-detail = { $error }

## Stammbaumseite, Fortsetzung

tree-title-suffix = Stammbaum
tree-back-to-focused = Zurück zur Ansicht um eine Person
tree-show-all = Alle { $n } anzeigen
tree-width-notice = Diese Ansicht ist { $width } Pixel breit — auf einem Bildschirm mit 1500 Pixeln sind das { $screens ->
        [one] ein Bildschirm
       *[other] { $screens } Bildschirme
    } waagerechtes Scrollen.
tree-confidence-label = Sicherheit:
tree-band-certain = sicher
tree-band-high = hoch
tree-band-medium = mittel
tree-band-low = vermutet
tree-counts = { $drawn } von { $total } Personen · { $generations ->
        [one] eine Generation
       *[other] { $generations } Generationen
    }
tree-unplaced-count = { $n } ohne Platz
tree-contradicts-title = Dieser Stammbaum widerspricht sich selbst.
tree-contradicts-detail = Keine Anordnung von Zeilen kann das erfüllen, deshalb blieb die Beziehung unten bei der Nummerierung der Generationen außen vor und einige Zeilen stehen womöglich falsch. Berichtigen Sie den der beiden Einträge, der falsch ist.
tree-contradicts-pair = Zugleich als Paar und als Elternteil und Kind erfasst:
tree-contradicts-more = { $n ->
        [one] Ein weiterer Widerspruch ist nicht aufgeführt.
       *[other] { $n } weitere Widersprüche sind nicht aufgeführt.
    }
tree-no-people = In diesem Stammbaum ist noch niemand.
tree-no-people-cta = Importieren Sie eine Familiendatei oder legen Sie die erste Person an.
tree-nobody-selected = Für diese Auswahl gibt es niemanden zu zeichnen.
tree-nobody-selected-cta = Beginnen Sie mit der Standardansicht.
tree-edge-union = Eine erfasste Verbindung
tree-edge-parentage = Eine erfasste Abstammung

## Startseite

home-empty = Noch nichts erfasst. Importieren Sie eine Familiendatei, um einen vorhandenen Stammbaum zu übernehmen, oder legen Sie die erste Person von Hand an.
home-count = { $total ->
        [one] Ein Eintrag
       *[other] { $total } Einträge
    }, in einer Datei, die der Familie gehört.
home-browse = Stammbaum ansehen
home-convert = Familiendatei importieren
home-unnamed-family = Dieser Stammbaum
home-in-this-tree = Was die Familie bisher erfasst hat
home-showcase-title = Wo dieser Stammbaum bereits mehr sagt als Namen und Daten
home-showcase-example = Ein Beispiel ansehen →
home-nothing-title = Noch nichts zu zeigen.
home-nothing-detail = Importieren Sie eine Familiendatei, um einen vorhandenen Stammbaum zu übernehmen, oder fangen Sie bei null an und legen Sie die erste Person selbst an.

## Übersichtskarten

showcase-links-title = { $n ->
        [one] Eine Beziehung außerhalb der Familie
       *[other] { $n } Beziehungen außerhalb der Familie
    }
showcase-links-detail = Paten, Arbeitgeber, Zeugen und Mentoren, jede mit eigenen Daten, eigener Quelle und Ihrer Sicherheit.
showcase-occupations-title = { $n ->
        [one] Ein Beruf mit Anfang und Ende
       *[other] { $n } Berufe mit Anfang und Ende
    }
showcase-occupations-detail = „Lehrerin, 1948–1978“ behält ihre Dauer und wird als Balken über die Jahre gezeichnet statt als einzelne datierte Zeile.
showcase-uncertain-title = { $n ->
        [one] Ein Datum, so ungenau belassen, wie es überliefert ist
       *[other] { $n } Daten, so ungenau belassen, wie sie überliefert sind
    }
showcase-uncertain-detail = Um, vor, nach und zwischen bleiben vier verschiedene Aussagen. Ein Datum, das die Quelle nicht festlegen konnte, wird nie so gezeigt, als hätte sie es gekonnt.
showcase-preserved-title = { $n ->
        [one] Ein Datum, in den Worten bewahrt, in denen es geschrieben stand
       *[other] { $n } Daten, in den Worten bewahrt, in denen sie geschrieben standen
    }
showcase-preserved-detail = Eine Formulierung, die niemand als Datum lesen konnte, bleibt genau so stehen, wie sie geschrieben ist, statt stillschweigend verworfen zu werden.
showcase-sources-title = { $n ->
        [one] Eine Quelle mit erfasster Zuverlässigkeit
       *[other] { $n } Quellen mit erfasster Zuverlässigkeit
    }
showcase-sources-detail = { $primary ->
        [one] Eine Primärquelle.
       *[other] { $primary } Primärquellen.
    } Jede Angabe zeigt, auf welchem Beleg sie ruht und wie stark dieser Beleg ist.
showcase-places-title = { $n ->
        [one] Ein Ort, dessen Grenzen sich verschoben haben
       *[other] { $n } Orte, deren Grenzen sich verschoben haben
    }
showcase-places-detail = Eine Stadt kann zu verschiedenen Zeiten zu verschiedenen Staaten gehören, und der Eintrag sagt, welcher wann galt.

## Einzelheiten des Eintrags

record-also-recorded-as = auch erfasst als
record-borders-moved = Grenzen verschoben:
record-display-name = Anzeigename
record-read-as = gelesen als
record-note = Notiz
record-living-yes = lebend
record-deceased = verstorben
record-centre-tree-here = Stammbaum hier zentrieren
record-centre-tree-title = Den Stammbaum auf diese Person zentrieren
record-open-full-page = Ganze Seite öffnen ↗
record-open-full-title = Die eigenständige, teilbare Seite öffnen
record-edit = Bearbeiten
panel-empty = Wählen Sie eine Karte, um hier den vollständigen Eintrag dieser Person zu sehen.
person-see-in-tree = Diese Person im Stammbaum ansehen
person-visibility-inline = Sichtbarkeit:
person-age-at-death = gestorben mit { $n }
person-age-now = { $n } Jahre alt
person-born-in = geboren in { $place }
person-died-in = gestorben in { $place }
person-children-count = { $n ->
        [one] ein Kind
       *[other] { $n } Kinder
    }
person-generations-below = { $n ->
        [one] eine Generation darunter
       *[other] { $n } Generationen darunter
    }
person-portrait-of = Fotografie von { $name }
person-no-portrait = Keine Fotografie erfasst

## Ergebnisse von Vorgängen

result-diagnostics = Meldungen
result-diagnostics-note = Jede Meldung, die die Bibliothek zurückgegeben hat, auch Warnungen, die den Vorgang nicht aufgehalten haben. Nichts wird herausgefiltert.
result-no-diagnostics = Die Bibliothek hat keine Meldungen zurückgegeben.
result-continue = Weiter
result-dashboard = Übersicht
person-sections-label = Abschnitte dieser Seite

## Abschnitte des Eintrags, Einzelheiten

record-notes-title = Anmerkungen zu diesem Eintrag:
record-name = Name
record-type = Art
record-cause = Ursache:
record-as = als
record-partner-not-recorded = Partner nicht erfasst
record-union-from = Ab
record-union-at = in
record-union-until = bis
record-occupation-from = ab
record-occupation-until = bis
record-source-reliability = Zuverlässigkeit
record-source-supports = Stützt
record-photographs = Fotografien
record-documents = Dokumente
record-file = Datei
record-status = Status
record-size = Größe
record-absent-document = Von dieser Person genannt, aber hier nicht vorhanden.
record-no-file = keine Datei
record-attach-document = Dokument anhängen
record-doc-photo = Foto
record-doc-certificate = Urkunde
record-doc-letter = Brief
record-doc-record = Aufzeichnung
record-doc-newspaper = Zeitung
record-doc-other = sonstiges
record-upload = Hochladen
record-upload-help = Bis zu { $mb } MB je Datei. Anhänge liegen neben dem Stammbaum und werden beim Ausgeben wieder ins Archiv geschrieben, so reist eine Fotografie mit der Familie, zu der sie gehört. Die Art der Datei wird aus ihrem eigenen Inhalt gelesen, nicht aus ihrem Namen: Bilder, PDF, einfacher Text, Ton und Video werden angenommen. SVG wird abgelehnt, weil ein SVG ein Skript enthalten kann.
record-upload-help-short = Bis zu { $mb } MB. SVG wird abgelehnt.
record-verbatim-note = Genau so bewahrt, wie der Eintrag es angab, weil kein Konverter es deuten konnte.
record-file-to-attach = Anzuhängende Datei
record-document-type = Art des Dokuments
record-caption = Bildunterschrift
record-caption-placeholder = Bildunterschrift (optional)
record-history-entry-meta = — { $at }
record-history-entry-version = , { $version }

## Arten von Datensätzen

kind-person = Person
kind-family = Familie
kind-event = Ereignis
kind-link = Beziehung
kind-occupation = Beruf
kind-source = Quelle
kind-place = Ort
kind-document = Dokument

kind-person-plural = { $n ->
        [one] Person
       *[other] Personen
    }
kind-family-plural = { $n ->
        [one] Familie
       *[other] Familien
    }
kind-event-plural = { $n ->
        [one] Ereignis
       *[other] Ereignisse
    }
kind-link-plural = { $n ->
        [one] Beziehung
       *[other] Beziehungen
    }
kind-occupation-plural = { $n ->
        [one] Beruf
       *[other] Berufe
    }
kind-source-plural = { $n ->
        [one] Quelle
       *[other] Quellen
    }
kind-place-plural = { $n ->
        [one] Ort
       *[other] Orte
    }
kind-document-plural = { $n ->
        [one] Dokument
       *[other] Dokumente
    }

## Listen

list-matching = { $total ->
        [one] Ein Treffer
       *[other] { $total } Treffer
    }, { $per_page } je Seite.
list-filter-placeholder = Nach Name oder Kennung filtern
list-filter = Filtern
list-clear = Zurücksetzen
list-summary = Beschreibung
list-id = Kennung
list-actions = Aktionen
list-nothing = Hier ist nichts.
list-nothing-matching = Hier passt nichts zu „{ $q }“.
list-delete-confirm = Diesen Datensatz ({ $kind }) löschen? Wählen Sie, was mit Datensätzen geschieht, die auf ihn verweisen:
list-policy-reject = Ablehnen
list-policy-reject-detail = — ablehnen, solange noch etwas darauf verweist. Nichts geht verloren.
list-policy-cascade = Durchreichen
list-policy-cascade-detail = — ihn löschen und jeden Verweis darauf tatsächlich entfernen.
list-policy-orphan = Verwaisen lassen
list-policy-orphan-detail = — ihn löschen, aber die verweisenden Einträge behalten, mit geleertem Verweis.

## Vollständigkeit

completeness-dates-title = Daten nach der Form, die sie wirklich haben
completeness-no-dates = Noch keine Daten erfasst.
completeness-dates-note = Ein Datum, das jemand auf den Tag festlegen konnte, und eines, das jemand nur einem Jahrzehnt zuordnen konnte, sind zwei verschiedene Aussagen, und beide bleiben so erhalten, wie sie überliefert sind. Text, der sich überhaupt nicht als Datum lesen ließ, wird Wort für Wort bewahrt statt verworfen.
completeness-shape-exact = genau
completeness-shape-exact-note = ein vollständiger Kalendertag
completeness-shape-approximate = ungefähr
completeness-shape-approximate-note = um, oder nur ein Jahr beziehungsweise Jahrzehnt
completeness-shape-ranged = eingegrenzt
completeness-shape-ranged-note = vor, nach oder zwischen
completeness-shape-preserved = wörtlich
completeness-shape-preserved-note = nicht deutbarer Text, unverändert bewahrt
completeness-shape-unknown = unbekannt
completeness-shape-unknown-note = als unbekannt erfasst

## Importseite

convert-page-title = Familiendatei importieren
convert-lede = Übernehmen Sie einen vorhandenen Stammbaum aus einer GEDCOM-Datei — der Ausgabe, die die meisten Genealogieprogramme erzeugen. Hier wird nichts gespeichert, und der Stammbaum, den diese Website bereits zeigt, bleibt genau so, wie er war.
convert-file-label = Familiendatei (.ged)
convert-file-hint = Bis zu { $mb } MB. Ein Stammbaum mit 767 Personen ist etwa 320 KB groß.
convert-confidence-label = Wie sicher diese Angaben zu Beginn sind
convert-confidence-hint = Die eingelesene Datei sagt nicht, wie sicher sich jemand war, deshalb braucht jede Angabe einen Ausgangspunkt. Setzen Sie ihn niedrig für einen rasch zusammengetragenen Stammbaum, höher für einen aus Dokumenten erarbeiteten. Ehrlich gelesen heißt diese Zahl „eingelesen und seither von niemandem geprüft“ — Sie können jede Angabe später einzeln anheben oder senken.
convert-lang-label = Sprache der Ortsnamen
convert-lang-hint = Ein Kürzel wie en, fr oder de.

## Importbericht

convert-failed = Der Import ist nicht durchgegangen
convert-try-another = Eine andere Datei versuchen
convert-converted = { $filename } importiert
convert-result-lede = { $total ->
        [one] Ein Eintrag
       *[other] { $total } Einträge
    }, { $size } KB. Alles kam mit einer Sicherheit von { $confidence } herein, die Ortsnamen wurden als { $lang } gelesen. Der Stammbaum, den diese Website zeigt, blieb unberührt.
convert-produced = Was herübergekommen ist
convert-skipped-title = { $n ->
        [one] Ein Eintrag, der sich nicht lesen ließ
       *[other] { $n } Einträge, die sich nicht lesen ließen
    }
convert-skipped-note = In diesen Einträgen stand nichts, was sich hätte übernehmen lassen.
convert-other-diagnostics = { $n ->
        [one] Eine weitere Sache, die man wissen sollte
       *[other] { $n } weitere Dinge, die man wissen sollte
    }
convert-clean = Nichts blieb zurück — jeder Eintrag der Datei kam herüber.
convert-download-title = Herunterladen
convert-download-named = { $name } herunterladen
convert-download-note = Wird hier fünfzehn Minuten aufbewahrt und dann verworfen, laden Sie es also jetzt herunter.
convert-another = Eine weitere Datei importieren
admin-history-on = am
admin-history-meta = — { $kind }, { $at }
admin-validation-counts = { $errors ->
        [one] Ein Fehler
       *[other] { $errors } Fehler
    }, { $warnings ->
        [one] eine Warnung
       *[other] { $warnings } Warnungen
    }, { $infos ->
        [one] ein Hinweis
       *[other] { $infos } Hinweise
    }.
admin-warnings-never-block = Warnungen halten nie auf — sie sind Auskunft, keine Schranke.
admin-validator-clean = Die Prüfung hat nichts gemeldet.
record-occupations-help-undated = Ein Beruf wird mit Anfang und Ende erfasst, damit sich mehrere auf einer Zeitachse vergleichen lassen. Dieses Archiv hat die Berufsbezeichnungen, aber keine Daten dazu — nach einem Import üblich, weil die meisten Familiendateien keinen Platz dafür haben —, es gibt also noch keine Skala zu zeichnen.
record-occupations-help-axis = Ein Beruf ist ein Zustand mit Dauer, kein Ereignis an einem einzelnen Datum. Alle Zeitspannen teilen eine Achse, { $from }–{ $to }.
admin-value-not-set = nicht gesetzt
admin-validation-report = Prüfbericht
admin-dedup-complete = Zusammenführen abgeschlossen
admin-dedup-refused = Zusammenführen abgelehnt
record-birth-order = Geburtenfolge
record-start-not-recorded = Anfang nicht erfasst
record-end-not-recorded = Ende nicht erfasst
record-document-no-file = Das Dokument ist hier erfasst, die Datei selbst liegt aber nicht vor
panel-selected-person = Gewählte Person

## Generationenbänder

tree-band-generation = Generation { $g }
tree-band-people = { $n ->
        [one] eine Person
       *[other] { $n } Personen
    }
tree-band-unplaced = Ohne Platz
tree-band-unplaced-note = { $n ->
        [one] eine Person ohne Familie — gezeigt statt weggelassen
       *[other] { $n } Personen ohne Familie — gezeigt statt weggelassen
    }

## Kontrolliertes Vokabular

gender-M = Männlich
gender-F = Weiblich
gender-NB = Nichtbinär
gender-unrecorded = Nicht erfasst

name-part-given_name = Vorname
name-part-family_name = Familienname
name-part-patronymic = Vatersname
name-part-matronymic = Muttersname
name-part-middle_name = zweiter Vorname
name-part-nickname = Beiname
name-part-prefix = Präfix
name-part-suffix = Suffix
name-part-particle = Namenszusatz
name-part-part = Bestandteil

name-type-primary = Hauptname
name-type-other = anderer
name-type-alias = Rufname
name-type-birth = Geburtsname
name-type-married = Ehename
name-type-religious = Ordensname
name-type-transliteration = Umschrift
name-type-nickname = Beiname

## Anmerkungen zum Eintrag

note-links = { $n ->
        [one] eine Beziehung außerhalb der Familie, mit eigenen Daten und Quellen
       *[other] { $n } Beziehungen außerhalb der Familie, mit eigenen Daten und Quellen
    }
note-occupations = { $n ->
        [one] ein Beruf mit Anfang und Ende erfasst
       *[other] { $n } Berufe mit Anfang und Ende erfasst
    }
note-birth-imprecise = ein Geburtsdatum, das die Quelle nicht festlegen konnte, so gezeigt, wie es erfasst ist
note-death-imprecise = ein Sterbedatum, das die Quelle nicht festlegen konnte, so gezeigt, wie es erfasst ist
note-names = { $n ->
        [one] ein erfasster Name
       *[other] { $n } erfasste Namen
    }
note-transliteration = ein Name in eigener Schrift neben seiner lateinischen Umschrift
note-witnessed = { $n ->
        [one] ein Ereignis, bei dem sie Zeuge waren statt Beteiligte
       *[other] { $n } Ereignisse, bei denen sie Zeugen waren statt Beteiligte
    }

visibility-public = öffentlich
visibility-members = Familienmitglieder
visibility-contributors = Mitwirkende
visibility-private = privat

## Zeilenbeschriftungen in den Verwaltungslisten

family-label-couple = { $children ->
        [0] { $a } & { $b }
        [one] { $a } & { $b } — ein Kind
       *[other] { $a } & { $b } — { $children } Kinder
    }
family-label-half = { $children ->
        [0] { $a } & { $unknown }
        [one] { $a } & { $unknown } — ein Kind
       *[other] { $a } & { $unknown } — { $children } Kinder
    }
family-label-children = { $others ->
        [0] { $first } — Eltern nicht erfasst
        [one] { $first } und ein Geschwister — Eltern nicht erfasst
       *[other] { $first } und { $others } Geschwister — Eltern nicht erfasst
    }
family-label-empty = Familie ohne erfasste Personen

event-label = { $category } — { $who }, { $date }
event-label-nobody = { $category } — { $date }
event-two-people = { $a } & { $b }
event-more-people = { $a } & { $b } und { $others ->
        [one] ein weiterer
       *[other] { $others } weitere
    }

link-label = { $label }: { $from } → { $to }
occupation-label = { $who } — { $title }
source-label = { $title } — { $reliability }
source-label-plain = { $title }
document-label = { $filename } — { $type }
document-label-untitled = { $type } ohne Titel
list-unnamed = { $kind } ohne Namen

## Vokabulare der Spezifikation in den Listen

event-category-birth = Geburt
event-category-death = Tod
event-category-marriage = Heirat
event-category-divorce = Scheidung
event-category-baptism = Taufe
event-category-burial = Bestattung
event-category-immigration = Einwanderung
event-category-emigration = Auswanderung
event-category-census = Volkszählung
event-category-residence = Wohnsitz
event-category-military = Militärdienst
event-category-education = Ausbildung
event-category-other = Ereignis

reliability-primary = Primärquelle
reliability-secondary = Sekundärquelle
reliability-tertiary = Tertiärquelle
reliability-recollection = Erinnerung
reliability-derivative = abgeleitete Arbeit
reliability-authored = verfasstes Werk
reliability-oral = mündliche Überlieferung
reliability-unknown = Zuverlässigkeit unbekannt

document-type-photo = Fotografie
document-type-certificate = Urkunde
document-type-letter = Brief
document-type-record = Aufzeichnung
document-type-newspaper = Zeitungsausschnitt
document-type-other = Dokument

## Wo dieser Eintrag mehr sagen könnte

completeness-title = Wo dieser Stammbaum mehr sagen könnte
completeness-intro = Was erfasst ist und was noch leer ist.
completeness-import-title = Was der Import mitgebracht hat
completeness-import-intro = Gezählt aus der Datei, die Sie gerade hochgeladen haben. Eine leere Zeile ist etwas, das die ursprüngliche Datei nicht erfasst hat — nicht etwas, das der Import verloren hat.

completeness-headline-full = Jede Art von Angabe unten ist irgendwo in diesem Stammbaum erfasst.
completeness-headline-empty = { $total ->
        [one] Die eine Art von Angabe unten ist noch nirgends erfasst.
       *[other] Keine der { $total } Arten von Angaben unten ist bisher erfasst.
    } Jede ist eine Stelle, an der der Eintrag mehr sagen könnte.
completeness-headline-partial = { $carried ->
        [one] Eine Art von Angabe unten ist erfasst
       *[other] { $carried } Arten von Angaben unten sind erfasst
    }; { $empty ->
        [one] eine ist noch leer
       *[other] { $empty } sind noch leer
    }.

completeness-metric-confidence = Wie sicher jede Angabe ist
completeness-metric-confidence-none = Keine der { $slots } Angaben hier sagt, wie sicher sie ist. Ein Datum, das jemand von einer Urkunde abgelesen hat, und eines, das jemand geraten hat, sehen gleich aus — bis sie es nicht mehr tun.
completeness-metric-confidence-uniform = { $with } von { $slots } Angaben tragen einen Wert, und jeder davon ist dieselbe Zahl ({ $modal }). Genau das hinterlässt ein Masseneinlesen: ein Platzhalter, zu dem niemand zurückgekehrt ist. Keine ist bisher einzeln beurteilt worden.
completeness-metric-confidence-some = { $with } von { $slots } Angaben tragen einen Wert. { $modal_count } teilen sich einen Wert ({ $modal }); { $assessed } weichen davon ab und wurden also einzeln angesehen.
completeness-metric-confidence-many = { $with } von { $slots } Angaben tragen einen Wert, davon weichen { $assessed } vom häufigsten Wert ({ $modal }) ab, über { $distinct } verschiedene Stufen. Dieser Stammbaum erfasst echte, abgestufte Unsicherheit.

completeness-metric-parentage = Wie sicher jede Eltern-Kind-Beziehung ist
completeness-metric-parentage-none = Keine Abstammung hier sagt, wie sicher sie ist. Adoptionen, umstrittene Linien und Rekonstruktionen aus einer einzigen Erwähnung sind genau die Stellen, an denen eine Familie Zweifel festhalten muss — und der Stammbaum zeichnet eine weniger sichere Beziehung als blassere Linie.
completeness-metric-parentage-some = { $n ->
        [one] Eine Abstammung trägt einen eigenen Wert
       *[other] { $n } Abstammungen tragen einen eigenen Wert
    }, sodass eine vermutete Linie sichtbar schwächer ist als eine belegte.

completeness-metric-links = Beziehungen jenseits von Blut und Ehe
completeness-metric-links-none = Paten, Arbeitgeber, Zeugen, Mentoren, Vormunde. Bisher ist keine erfasst. Jede kann eigene Daten, ihre Quelle und Ihre Sicherheit tragen.
completeness-metric-links-some = { $n ->
        [one] Eine erfasst, mit eigenen Daten, eigener Quelle und Ihrer Sicherheit.
       *[other] { $n } erfasst, jede mit eigenen Daten, eigener Quelle und Ihrer Sicherheit.
    }

completeness-metric-occupations = Berufe mit Anfang und Ende erfasst
completeness-metric-occupations-none = Keine Berufe erfasst. Ein Handwerk, das jemand dreißig Jahre lang ausübte, sagt mehr über ein Leben als ein einzelner datierter Eintrag.
completeness-metric-occupations-undated = { $total ->
        [one] Ein Beruf ist erfasst, ohne Daten
       *[other] { $total } Berufe sind erfasst, ohne Daten
    }. Fügen Sie Anfang und Ende hinzu, dann lassen sie sich auf einer Zeitachse nebeneinander vergleichen.
completeness-metric-occupations-some = { $span } von { $total } haben einen Anfang oder ein Ende, lassen sich also auf einer Zeitachse nebeneinander vergleichen.

completeness-metric-sources = Quellen mit Angabe ihrer Zuverlässigkeit
completeness-metric-sources-none = Keine Quellen erfasst. Zu benennen, woher eine Angabe stammt, ist das, was einem Verwandten erlaubt, sie später zu prüfen — oder ihr zu widersprechen und zu sagen, warum.
completeness-metric-sources-some = { $graded } von { $total } sagen, wie stark sie sind, sodass eine Aussage, die auf einer Geburtsurkunde ruht, sichtbar nicht dasselbe ist wie eine, die auf einer Erinnerung ruht.

completeness-what-is-recorded = Was der Eintrag sagen kann
completeness-in-this-tree = In diesem Stammbaum
completeness-not-yet = noch nicht erfasst

## Rollen, die ein Beteiligter in einem Ereignis hat

role-spouse = Ehepartner
role-spouse_1 = erster Ehepartner
role-spouse_2 = zweiter Ehepartner
role-subject = betroffene Person
role-participant = Beteiligter
role-witness = Zeuge
role-officiant = Amtsperson
role-informant = Anzeigender
role-godparent = Pate
