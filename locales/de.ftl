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
account-emergency = Notfall-Token
footer-credit = © 2026 · Entworfen und entwickelt von { $author }
footer-format-link = Das AXGF-Format auf GitHub

## Einstellungen

settings-title = Einstellungen
edit-mode-label = Bearbeitung — was Sie speichern, ändert die Einträge der Familie
display-menu-label = Sprache und Anzeige
display-settings = Farbschema und Anzeige …
settings-tabs-label = Abschnitte der Einstellungen
settings-tab-theme = Farbschema
settings-tab-language = Sprache
settings-tab-appearance = Darstellung
settings-done = Fertig
prefs-language = Sprache
prefs-theme = Farbschema
prefs-background = Hintergrund
prefs-background-on = Sanfter Farbverlauf hinter der Seite
prefs-apply = Übernehmen
busy-saving = Wird gespeichert…
busy-working = Wird ausgeführt…
prefs-reviewed = geprüft
prefs-machine = maschinell, { $coverage } %
prefs-machine-complete = vollständig, noch ungeprüft
prefs-machine-title = Ohne Prüfung durch eine Person mit dieser Muttersprache übersetzt. Besonders das genealogische Vokabular kann falsch sein — die Wörter für eine Verbindung, einen Paten oder eine Primärquelle unterscheiden sich je nach Archivtradition des Landes. Korrekturen sind willkommen, und CONTRIBUTING.md sagt, wo man anfängt.

theme-canopee = Blätterdach
theme-canopee-note = dunkel, waldgrün · Standard
theme-fusion = Fusion
theme-fusion-note = dunkel, Neon · ein lebender Baum
theme-galaxy = Galaxie
theme-galaxy-note = tiefer Weltraum · ein Sonnensystem
theme-summit = Gipfel
theme-summit-note = dunkel, goldene Dämmerung · ein Aufstieg

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
life-nothing-recorded = Für diese Person sind weder Ereignis noch Beruf noch Ort verzeichnet. Bei einer konvertierten Datei ist das der Normalfall: GEDCOM trägt weiter, was jemand aufgeschrieben hat, und die meisten Einträge sind ein Name und ein Datum.
life-add-first = Erstes Ereignis erfassen
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
error-merged-title = Dieser Eintrag existiert nicht mehr.
error-merged-detail = Er wurde am { $date } mit { $name } zusammengeführt. Links auf die alte Adresse führen jetzt zum zusammengeführten Eintrag.
error-merged-open = Zusammengeführten Eintrag öffnen
error-back-to-river = Zurück zum Fluss
view-tree-back = Zurück zum Baum
view-galaxy-back = Zurück zur Galaxie
view-summit-back = Zurück zum Berg
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
setup-title = Dieses Archiv einrichten
setup-lede = Diese Installation enthält noch keine Familie. Wählen Sie, wie sie beginnt. Hier wird nie etwas über eine Familie geschrieben: Diese Seite schließt sich, sobald es eine gibt.
setup-door-archive-title = Aus einem AXGF-Archiv
setup-door-archive-detail = Eine .axgf-Datei aus einer anderen Installation oder aus einer Sicherung. Personen, Quellen und Fotos kommen unverändert herüber.
setup-door-archive-file = Archiv (.axgf)
setup-door-archive-go = Mit diesem Archiv einrichten
setup-limit-gb = Bis zu { $n } GB, direkt auf die Festplatte geschrieben.
setup-door-scratch-title = Aus dem Nichts
setup-door-scratch-detail = Ein leeres Archiv. Fügen Sie die erste Person hinzu, dann ihre Familie, Datensatz für Datensatz.
setup-door-scratch-go = Erste Person hinzufügen
setup-door-gedcom-title = Aus einer GEDCOM-Datei
setup-door-gedcom-detail = Die Datei, die jedes Genealogieprogramm exportiert: GEDCOM 5.5, 5.5.1, 5.5.5 oder 7.0, als .ged oder .gdz, in welcher Kodierung auch immer. Sie sehen, wie sie gelesen wurde und was herüberkommt, bevor etwas behalten wird.
setup-door-gedcom-file = GEDCOM-Datei (.ged, .gdz)
setup-door-gedcom-go = Diese Datei lesen
setup-never-over = Um eine schon vorhandene Familie zu ersetzen, stellen Sie eine Sicherung über die Kommandozeile wieder her (axgf-cms restore); keine Seite schreibt darüber.
setup-adopt-title = Zur Familie des Archivs machen
setup-adopt-detail = Datensätze, die zur Familie dieser Installation würden: { $total }. Nichts wird behalten, bevor Sie den Knopf drücken.
setup-adopt-go = Archiv mit dieser Familie einrichten
setup-adopt-back = Einen anderen Weg wählen
setup-done-title = Das Archiv ist eingerichtet
setup-done-detail = Datensätze in der Familie dieser Installation jetzt: { $total }.
setup-done-photos = Fotos und Dokumente, die mitgekommen sind: { $n }.
setup-done-mismatches = Dateien, deren Inhalt nicht zum gespeicherten Fingerabdruck passt: { $n }. Sie bleiben erhalten; prüfen Sie sie.
setup-done-river = Den Fluss öffnen
view-tree-open = Den Baum öffnen
view-galaxy-open = Die Galaxie öffnen
view-summit-open = Den Berg öffnen
setup-done-admin = Verwaltung
setup-refused-title = Daraus konnte kein Archiv werden
setup-refused-detail = Nichts wurde geschrieben. Was es verhindert hat:
setup-closed-title = Dieses Archiv ist bereits eingerichtet
setup-closed-detail = Es enthält eine Familie, und keine Seite schreibt darüber. Um sie zu ersetzen, stellen Sie eine Sicherung über die Kommandozeile wieder her (axgf-cms restore).
setup-not-archive = Das ist kein .axgf-Archiv.
setup-archive-too-large = Dieses Archiv ist größer, als ein Upload sein darf.
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
form-more-fields = { $n ->
        [one] Ein weiteres Feld
       *[other] { $n } weitere Felder
    }
admin-raw-json-help = Der ganze Datensatz, damit nichts unbearbeitbar ist — Listen wie die Partner und Kinder einer Familie oder die Grenzgeschichte eines Ortes stehen genau hier. Das ist das Ausgangsdokument; die Felder darüber werden anschließend über die ihnen gehörenden Pfade geschrieben, bearbeiten Sie einen Wert also an der einen oder der anderen Stelle, nicht an beiden. Es muss sich als JSON lesen lassen, sonst wird nichts gespeichert.
admin-save = Speichern
admin-cancel = Abbrechen
place-editor-title = Ort bearbeiten
place-add-detail = Diesen Ort ergänzen
place-names = Namen
place-name-primary = Primär
place-name-lang = Sprache
place-name-value = Name
place-names-hint = Eine Zeile je erfasstem Namen. Ein Ort, den drei Reiche verwaltet haben, trägt drei Namen; der Hauptname ist der, der überall sonst erscheint.
place-where = Lage
place-type = Art
place-region = Region
place-country-current = Land heute
place-country-hint = ISO 3166-1 alpha-2, zum Beispiel PL, FR, DE.
place-country-history = Grenzverlauf
place-history-country = Staat
place-history-from = Von
place-history-until = Bis
place-country-history-hint = Welcher Staat diesen Ort in welchem Zeitraum innehatte. Genealogisch wichtig: Ein 1880 auf Russisch und ein 1930 auf Polnisch geschriebener Eintrag können dasselbe Dorf nennen.
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
place-error-no-name = Ein Ort braucht mindestens einen Namen.
place-error-coords-pair = Breite und Länge gehören zusammen: beide angeben oder keine.
place-error-coords-number = Breite und Länge müssen Zahlen sein.
place-error-coords-range = Die Breite reicht von -90 bis 90, die Länge von -180 bis 180.
place-type-continent = Kontinent
place-type-country = Staat
place-type-region = Region
place-type-department = Verwaltungsbezirk
place-type-city = Stadt
place-type-village = Dorf
place-type-district = Stadtteil
place-type-street = Straße
place-type-building = Gebäude
place-type-farm = Hof
place-type-island = Insel
place-type-historical = historisch
place-type-unknown = unbekannt
place-precision-exact = genau
place-precision-building = Gebäude
place-precision-street = Straße
place-precision-city_center = Stadtmitte
place-precision-region_center = Mitte der Region
place-precision-country_center = Mitte des Landes
place-precision-approximate = ungefähr

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
person-tab-life = Ereignisse
person-tab-media = Medien
person-tab-tree = Stammbaum
person-tab-history = Verlauf
person-tree-depth = { $n } Generationen in jede Richtung. Der ganze Baum steht darunter.
person-tree-alone = Dieser Eintrag nennt weder Eltern noch Partner noch Kinder, also gibt es um ihn herum keine Form zu zeichnen.

record-no-evidence = An diesem Eintrag hängt nichts — keine Quelle und kein Dokument. Das ist der Normalzustand einer konvertierten Datei und kein Fehler darin: GEDCOM trägt die Fakten und lässt zurück, was sie belegte.
record-no-evidence-signed-out = Melden Sie sich an, um etwas anzuhängen.
admin-delete = Löschen
admin-not-set = — nicht gesetzt —
admin-edit = Bearbeiten
admin-page-of = Seite { $page } von { $pages }
list-select-row = { $name } auswählen
list-kind-label = Art
list-select-page = Alle Zeilen dieser Seite auswählen
list-selected = { $n ->
        [one] { $n } ausgewählt
       *[other] { $n } ausgewählt
    }
list-export = Exportieren
list-escape = Esc hebt die Auswahl auf
bulk-confirm-title = { $n ->
        [one] { $n } Eintrag löschen?
       *[other] { $n } Einträge löschen?
    }
bulk-confirm-detail = Ein Eintrag, auf den ein anderer noch verweist, wird abgelehnt – dann wird nichts gelöscht.
bulk-confirm-go = Löschen
bulk-deleted = { $n ->
        [one] { $n } Eintrag gelöscht.
       *[other] { $n } Einträge gelöscht.
    }
bulk-refused = Nichts wurde gelöscht: Auf { $name } wird noch an anderer Stelle verwiesen. Entfernen Sie zuerst diesen Verweis und versuchen Sie es erneut.
bulk-changed = { $n ->
        [one] { $n } Eintrag hat sich seit dem Anzeigen der Liste geändert, daher wurde nichts gelöscht. Laden Sie die Liste neu und versuchen Sie es erneut.
       *[other] { $n } Einträge haben sich seit dem Anzeigen der Liste geändert, daher wurde nichts gelöscht. Laden Sie die Liste neu und versuchen Sie es erneut.
    }
admin-range = { $from }–{ $to } von { $total }
list-filiation = Abstammung
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
accounts-no-registration = Besucher können keine Konten selbst eröffnen; die Registrierung wird oben festgelegt. Für die meisten Familien genügt ein Administrator, der alle kennt.
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

home-empty = Noch nichts erfasst.
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
home-nothing-admin = Diese Installation enthält noch keine Familie: Richten Sie sie aus einem Archiv, aus einer GEDCOM-Datei oder aus dem Nichts ein.
home-nothing-visitor = Dieses Familienarchiv wird gerade eingerichtet.
home-eyebrow = Familienarchiv
home-hero-1 = Jedes Leben ist eine Quelle.
home-hero-2 = Jede Familie ein Fluss.
view-tree-hero-1 = Jedes Leben ist ein Blatt.
view-tree-hero-2 = Jede Familie ein Baum.
view-galaxy-hero-1 = Jedes Leben ist ein Stern.
view-galaxy-hero-2 = Jede Familie eine Galaxie.
view-summit-hero-1 = Jedes Leben ist ein Schritt.
view-summit-hero-2 = Jede Familie ein Aufstieg.
home-cta-river = Den Fluss erkunden
view-tree-cta = Den Baum erkunden
view-galaxy-cta = Die Galaxie erkunden
view-summit-cta = Den Berg besteigen
home-cta-tree = Den Stammbaum ansehen
home-scroll = Zum Erkunden scrollen
home-trust-title = Gewissheit der Abstammungen
home-trust-certain = praktisch sicher
home-trust-high = gut belegt
home-trust-medium = plausibel
home-trust-low = hypothetisch
home-trust-unrated = noch nicht bewertet
home-showcase-eyebrow = Mehr als Namen und Daten
home-cta-title = Gemeinsam bewahren wir das Gedächtnis der Familie.
home-cta-detail = Jeder hinzugefügte Name und jede geprüfte Quelle macht den Fluss genauer für alle, die nach uns kommen.
view-tree-cta-detail = Jeder hinzugefügte Name und jede geprüfte Quelle macht den Baum genauer für alle, die nach uns kommen.
view-galaxy-cta-detail = Jeder hinzugefügte Name und jede geprüfte Quelle macht die Galaxie genauer für alle, die nach uns kommen.
view-summit-cta-detail = Jeder hinzugefügte Name und jede geprüfte Quelle macht den Berg wahrer für die, die nach uns kommen.
home-cta-add = Person hinzufügen

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
record-see-in-tree = Im Stammbaum zeigen
record-see-in-tree-title = Das Kartenraster um diese Person öffnen
record-open-full-page = Ganze Seite öffnen ↗
record-open-full-title = Die eigenständige, teilbare Seite öffnen
record-edit = Bearbeiten
record-see-in-river = Im Fluss ansehen
view-tree-see = Im Baum ansehen
view-galaxy-see = In der Galaxie ansehen
view-summit-see = Auf dem Berg ansehen
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

## Vocabulary the structured editors offer

name-part-nasab = Nasab (Abstammung)
name-part-laqab = Laqab (Beiname)
name-part-kunya = Kunya (Teknonym)
name-part-nisbah = Nisba (Herkunft)
name-part-alias = Alias
name-part-religious_name = Ordensname
name-part-pen_name = Künstlername
name-type-pen_name = Künstlername
gender-U = Nicht erfasst

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
record-upload = Hochladen
record-upload-help = Bis zu { $mb } MB je Datei. Anhänge liegen neben dem Stammbaum und werden beim Ausgeben wieder ins Archiv geschrieben, so reist eine Fotografie mit der Familie, zu der sie gehört. Die Art der Datei wird aus ihrem eigenen Inhalt gelesen, nicht aus ihrem Namen: Bilder, PDF, einfacher Text, Ton und Video werden angenommen. SVG wird abgelehnt, weil ein SVG ein Skript enthalten kann.
record-upload-help-short = Bis zu { $mb } MB. SVG wird abgelehnt.
record-verbatim-note = Genau so bewahrt, wie der Eintrag es angab, weil kein Konverter es deuten konnte.
record-file-to-attach = Anzuhängende Datei
record-document-type = Art des Dokuments
record-caption = Bildunterschrift
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

phys-no-source = ohne Quelle
phys-col-date = Wann
phys-col-source = Quelle
phys-col-confidence = Sicherheit
phys-col-note = Anmerkung
phys-field-height-cm = Größe
phys-field-weight-kg = Gewicht
phys-field-eye-colour = Augenfarbe
phys-field-hair-colour = Haarfarbe
phys-field-build = Statur
phys-field-handedness = Händigkeit
phys-field-features = Besondere Kennzeichen
phys-field-military = Militärdienst
phys-field-languages = Gesprochene Sprachen
phys-field-blood-group = Blutgruppe
phys-field-conditions = Bekannte Erkrankungen
phys-field-operations = Operationen und Verletzungen
phys-field-cause-of-death = Todesursache
phys-field-religion = Religion oder Zugehörigkeit
phys-field-health-notes = Anmerkungen
admin-export-health-note = Der einfache Export lässt jede sensible Kategorie weg — Gesundheit und Weltanschauung, biometrische Daten, genomische Daten und Strafregister — ebenso das Verhaltensprofil jeder lebenden Person, sodass eine an Verwandte geschickte Datei nichts davon enthält. Kreuzen Sie an, was eine bestimmte Datei enthalten soll; das Archiv selbst vermerkt, welche Kategorien weggelassen wurden.
avatar-picker-title = Ein Bild wählen
avatar-choose-link = Bild wählen
avatar-choose = Welches Bild steht für diese Person
avatar-mode-auto = Die Software wählen lassen
avatar-mode-auto-note = Das erste Porträt, sonst das erste mit diesem Eintrag verknüpfte Bild.
avatar-mode-none = Stattdessen Initialen zeigen
avatar-mode-none-note = Für einen Eintrag, dessen Bilder Dokumente sind und keine Gesichter.
avatar-focal-hint = Klicken Sie ein Bild an, um es zu wählen, und klicken Sie noch einmal auf die Stelle, die im Ausschnitt bleiben soll — ein Avatar ist quadratisch, die meisten Scans sind es nicht.
avatar-no-images = Mit diesem Eintrag sind noch keine Bilder verknüpft.
avatar-upload-title = Ein Bild hochladen und verwenden
avatar-upload-button = Hochladen und als Bild verwenden
avatar-not-available-title = Dieses Bild steht nicht zur Verfügung
avatar-not-available-detail = Die gewählte Datei ist nicht mit dieser Person verknüpft oder Sie dürfen sie nicht lesen.
ai-badge = KI
ai-portraits-title = Von KI überarbeitete Porträts
ai-portraits-band = Retusche
ai-portraits-upload = Von KI überarbeitete Bilder hochladen
ai-portraits-manage = KI-Bilder verwalten
ai-portraits-intro = Bilder, die eine KI aus den Fotografien dieser Person gemacht hat: wie sie ausgesehen haben könnte. Sie erscheinen neben dem Gesicht im Eintrag, mit „KI“ gekennzeichnet, und nur angemeldete Mitglieder sehen sie. Sie sind nie das Bild der Person, nie unter ihren Dateien, nie ein Beleg.
ai-portraits-current = Im Eintrag
ai-portraits-add = Bilder hinzufügen — Platz für { $n }
ai-portraits-files = Bilder (PNG, JPEG, WebP oder GIF), mehrere auf einmal
ai-portraits-generator = Erstellt mit
ai-portraits-from = Nach
ai-portraits-from-none = Nicht angegeben
ai-portraits-remove = Entfernen
ai-portraits-alt = Porträt von { $name }, von einer KI überarbeitet
ai-portraits-not-evidence = Von einer KI aus den Fotografien gemacht: keine Bilder aus der Zeit.
ai-portraits-full = Der Eintrag hat seine drei KI-Bilder. Entfernen Sie eines, um ein anderes hinzuzufügen.
ai-portraits-too-many = Ein Eintrag fasst höchstens drei KI-Bilder. Entfernen Sie eines, bevor Sie ein weiteres hinzufügen.
ai-portraits-images-only = Ein KI-Porträt muss ein Bild sein: PNG, JPEG, WebP oder GIF.
ai-portraits-stale = Der Eintrag hat sich geändert, während diese Seite offen war. Nichts wurde gespeichert: Öffnen Sie ihn erneut und wiederholen Sie es.
ai-portraits-back = Zurück zu den KI-Bildern

record-history-withheld = Ihnen nicht gezeigt

## A death nobody wrote down
#
# GEDCOM cannot say "died, date unknown", so a converter marks those
# people living. These three say that the page worked it out rather than
# read it, which is a distinction this product does not blur.

record-life-status = Status
record-presumed-deceased = mutmaßlich verstorben
record-presumed-short = mutmaßl.
record-presumed-why = Es ist kein Tod erfasst und die Geburt liegt mehr als { $years } Jahre zurück, also kann dieser Eintrag nicht stimmen. Das Archiv bleibt unverändert: das ist die Folgerung der Seite, nicht die Aussage der Quelle.

## The identity editor

identity-editor-title = Namen und Identität
identity-primary-name = Der überall gezeigte Name
identity-primary-help = Was die Baumkarte, die Überschrift und jede Liste verwenden. Die anderen Namen unten sind die, mit denen eine Quelle diese Person zu anderer Zeit bezeichnet hat.
identity-display = Name
identity-display-latin = In lateinischer Schrift
identity-culture = Sprache
identity-direction = Schreibrichtung
identity-direction-ltr = von links nach rechts
identity-direction-rtl = von rechts nach links
identity-direction-auto = aus dem Text
identity-components = Namensbestandteile
identity-components-help = Welcher Teil der Vorname ist und welcher der Familienname, in der Schreibreihenfolge. Ein Eintrag ohne Teile wird trotzdem angezeigt: die Teile sind das, wonach eine Suche greifen kann.
identity-part = Teil
identity-value = Text
identity-other-names = Weitere Namen
identity-other-help = Ein Ehename, ein Ordensname, ein Name aus einem späteren Dokument. Jeder trägt seine Gebrauchszeit und die Quelle, die es sagt.
identity-name-type = Art des Namens
identity-valid-from = In Gebrauch ab
identity-valid-until = In Gebrauch bis
identity-about = Zur Person
identity-living-help = Das ist das Kennzeichen, das die Quelle gesetzt hat. Die Seite vermutet davon getrennt einen Tod, wenn die Geburt zu lange zurückliegt, und diese Vermutung ändert weder dieses Feld noch das Archiv.
identity-error-no-display = Ein Eintrag braucht einen Namen, unter dem er angezeigt wird. Nichts wurde gespeichert.
editor-blank-to-remove = Namen leeren, um diesen Eintrag zu entfernen.
# A BCP 47 tag, shown as an example in the language field.
identity-edit-link = Namen und Identität bearbeiten

## Union types, statuses and date precision, said out loud

union-type-marriage = Ehe
union-type-civil_union = eingetragene Partnerschaft
union-type-cohabitation = Lebensgemeinschaft
union-type-religious_only = kirchliche Ehe
union-type-polygamous = polygam
union-type-unknown = nicht erfasst
union-role-spouse = Ehepartner
union-role-partner = Partner
union-role-husband = Ehemann
union-role-wife = Ehefrau
union-status-active = bestehend
union-status-ended_by_death = durch Tod beendet
union-status-ended_by_divorce = durch Scheidung beendet
union-status-ended_by_separation = durch Trennung beendet
union-status-annulled = annulliert
union-status-unknown = nicht erfasst
union-status-ended = beendet
union-status-ended-by = beendet durch { $reason }
union-reason-death_of_spouse = den Tod eines Ehepartners
precision-exact = auf den Tag
precision-year = auf das Jahr
precision-month = auf den Monat
precision-decade = auf das Jahrzehnt
precision-century = auf das Jahrhundert
precision-unknown = unbekannt
record-precision = Genauigkeit
record-approximate = Ungefähr
record-place = Ort

## The relationships editor

family-editor-title = Familie und Beziehungen
family-unions = Verbindungen
family-no-unions = Für diese Person ist keine Verbindung erfasst.
family-union-legend = Verbindung { $n }
family-writes-family = Das Speichern ändert den Familieneintrag #{ $id }, den beide teilen. Die Seite der anderen Person ändert sich mit.
family-partners = Partner
family-partner = Partner
family-role = Rolle
family-children = Kinder
family-children-help = Die Geburtsreihenfolge ist die Aussage des Eintrags selbst. Leer gelassen sagt sie nichts: eine Zahl aus der Zeilenposition wäre eine Tatsache, die niemand aufgeschrieben hat.
family-child = Kind
family-birth-order = Geburtsreihenfolge
family-the-union = Die Verbindung selbst
family-type = Art der Verbindung
family-status = Stand
family-started = Beginn
family-ended = Ende
family-leave = Diese Person aus dieser Verbindung nehmen
family-open-entity = Familieneintrag öffnen
family-new-union = Eine neue Verbindung
family-new-union-help = Das legt einen neuen Familieneintrag mit dieser Person an. Ein Partner ist freiwillig: ein Elternteil, den der Eintrag ohne Partner nennt, ist eine Verbindung von einem.
family-create-union = Verbindung anlegen
family-parents = Eltern
family-no-parents = Diese Person ist als Kind keiner Familie erfasst.
family-child-of = Kind dieser Familie
family-detach-child = Diese Person aus dieser Familie nehmen
family-attach-parents = An Eltern anhängen
family-attach-help = Wählen Sie die Familie, deren Kind diese Person ist. Das trägt sie in deren Eintrag ein, also erscheint es auch auf den Seiten der Eltern.
family-the-family = Die Familie
family-attach = Anhängen
family-error-last-partner = Eine Verbindung braucht mindestens eine Person. Löschen Sie stattdessen den Familieneintrag; dabei wird gefragt, was mit allem geschehen soll, was darauf verweist.
family-error-no-family = Es wurde keine Familie gewählt. Nichts wurde gespeichert.
family-error-already-child = Diese Person ist bereits Kind dieser Familie.
pick-error-empty = Es wurde keine Person genannt. Nichts wurde gespeichert.
pick-error-not-found = In diesem Archiv gibt es niemanden dieses Namens. Nichts wurde gespeichert.
pick-error-ambiguous = Darauf hört mehr als eine Person. Wählen Sie eine aus der Liste, damit der Eintrag sagt welche. Nichts wurde gespeichert.

## Links and occupations

links-editor-title = Verknüpfungen
links-editor-help = Beziehungen, die keine Familie sind: ein Pate, ein Arbeitgeber, ein Zeuge, ein Regiment. Jede ist ein eigener Eintrag, der zwei Personen nennt, also ändert eine Änderung hier auch den anderen Eintrag.
links-none = Für diese Person ist keine Verknüpfung erfasst.
links-new = Eine neue Verknüpfung
links-create = Verknüpfung anlegen
links-remove = Diese Verknüpfung entfernen
links-other-end = Das andere Ende
links-label = Was es ist
links-label-reverse = In die andere Richtung
links-category = Kategorie
links-bidirectional = Liest sich in beide Richtungen gleich
links-from = Ab
links-until = Bis
links-reversed = Diese Verknüpfung wurde vom anderen Eintrag aus angelegt. Sie hier zu ändern ändert dieselbe Entität.
link-error-no-label = Eine Verknüpfung muss sagen, was sie ist. Nichts wurde gespeichert.
occupations-editor-title = Tätigkeiten
occupations-editor-help = Eine Tätigkeit ist ein Zeitraum mit Anfang und Ende, keine Stellenbezeichnung. Jede trägt eigene Daten und eine eigene Quelle.
occupations-none = Für diese Person ist keine Tätigkeit erfasst.
occupations-new = Eine neue Tätigkeit
occupations-create = Tätigkeit anlegen
occupations-remove = Diese Tätigkeit entfernen
occupations-title = Was sie tat
occupations-employer = Für wen
occupations-employer-place = Wo sie waren
occupations-from = Ab
occupations-until = Bis
occupation-error-no-title = Eine Tätigkeit muss sagen, was jemand tat. Nichts wurde gespeichert.
link-category-spiritual = geistlich
link-category-professional = beruflich
link-category-social = gesellschaftlich
link-category-legal = rechtlich
link-category-medical = medizinisch
link-category-educational = Ausbildung
link-category-conflict = Konflikt
link-category-other = sonstige
links-edit-link = Verknüpfungen bearbeiten
occupations-edit-link = Tätigkeiten bearbeiten
family-edit-link = Familie und Beziehungen bearbeiten

## Events and documents

events-editor-title = Ereignisse
events-editor-help = Ein Ereignis nennt mehrere Personen zugleich — eine Hochzeit, eine Taufe, eine Zählung — also ist jedes ein eigener Eintrag und erscheint auf jeder Seite, die es nennt.
events-none = Kein Ereignis nennt diese Person.
events-new = Ein neues Ereignis
events-new-help = Diese Person wird als Gegenstand eingetragen, wenn Sie niemanden sonst nennen. Ein Ereignis ohne Personen ist nur ein Datum.
events-create = Ereignis anlegen
events-remove = Dieses Ereignis entfernen
events-category = Was geschah
events-subcategory = Genauer
events-description = Beschreibung
events-participants = Wer dabei war
events-participants-help = Das Speichern ändert den Ereigniseintrag, den auch jede andere darin genannte Person zeigt.
events-who = Wer
event-error-no-category = Ein Ereignis muss sagen, was geschah. Nichts wurde gespeichert.
documents-editor-title = Dokumente
documents-editor-help = Auf welche Dateien dieser Eintrag zeigt und was jede für ihn ist. Eine Zeile zu leeren löst die Datei: Dokument und Bytes bleiben im Archiv.
documents-attached = An diesen Eintrag angehängt
documents-upload = Datei hochladen
documents-upload-help = Bis { $mb } MB. Die Datei wird im Archiv abgelegt und an diesen Eintrag gehängt.
documents-caption = Bildunterschrift
documents-edit-link = Dokumente anhängen und lösen
events-edit-link = Ereignisse bearbeiten

## Presentation styles: density, never colour

prefs-style = Dichte
prefs-style-help = Wie viel Raum die Seite einnimmt. Unabhängig vom Thema, bei dem es nur um Farbe geht — jede Kombination ist möglich.
style-comfortable = Bequem
style-comfortable-note = Standard, mit Raum zum Lesen
style-compact = Kompakt
style-compact-note = mehr Eintrag pro Bildschirm, zum Durcharbeiten mehrerer
style-paper = Papier
style-paper-note = eine Serifenschrift und Linien statt Karten, zum ruhigen Lesen oder Drucken

## Sensitive classes

admin-export-choose = In diesen Export aufnehmen
scope-health = Gesundheit und Weltanschauung
scope-biometrics = Biometrische Daten
scope-genomics = Genomische Daten
scope-legal = Strafregister
scope-behaviour = Verhaltensprofile lebender Personen
admin-export-with-chosen = Mit dem Angekreuzten exportieren

## Profile

pg-identity = Identität und Personenstand
pg-identity-intro = Wer die Person laut den Unterlagen war und was die Personenstandsregister festhielten.
pg-morphology = Morphologie
pg-morphology-intro = Der Körper, wie er gemessen und beschrieben wurde.
pg-biometrics = Biometrie
pg-biometrics-intro = Stimme, Hände und Sinne sowie die Merkmale, an denen man eine Person erkennt.
pg-health = Gesundheit
pg-health-intro = Erkrankungen, Behandlungen, Messwerte und Befunde.
pg-genomics = Genomik
pg-genomics-intro = DNA-Tests, Haplogruppen, Varianten und andere molekulare Befunde.
pg-death = Tod
pg-death-intro = Wie, wann und wo ein Leben endete und was mit dem Leichnam geschah.
pg-residence = Wohnort und Staatsangehörigkeit
pg-residence-intro = Wo die Person lebte, welche Staaten sie als Angehörige führten und welche Sprachen sie sprach.
pg-education = Bildung und Arbeit
pg-education-intro = Schulbildung, Abschlüsse, Einkommen und Besitz.
pg-military = Militär und Auszeichnungen
pg-military-intro = Dienst, Dienstgrade, Einheiten und Auszeichnungen.
pg-legal = Strafsachen
pg-legal-intro = Strafverfahren und ihr Ausgang.
pg-belief = Glaube und Zugehörigkeit
pg-belief-intro = Religion, Riten, Überzeugungen und Mitgliedschaften.
pg-personality = Persönlichkeit und Verhalten
pg-personality-intro = Temperament, Gewohnheiten und Freizeit, wie Quellen sie beschreiben.
pg-relationships = Beziehungen
pg-relationships-intro = Eltern, Partner, Kinder und die anderen Menschen eines Lebens.
pg-digital-legacy = Digitales Erbe
pg-digital-legacy-intro = Scans, Modelle, Aufnahmen und Archive, die für eine Person stehen.
pa-identity-titles = Titel
pa-identity-sex-at-birth = Geschlecht bei Geburt
pa-identity-gender-identity = Geschlechtsidentität
pa-birth-time = Geburtszeit
pa-birth-coordinates = Geburtsort als Koordinaten
pa-civil-status-birth-certificate-number = Nummer der Geburtsurkunde
pa-civil-status-register-entries = Registereinträge
pa-civil-status-marginal-annotations = Randvermerke
pa-morphology-height = Körpergröße
pa-morphology-weight = Gewicht
pa-morphology-bmi = Body-Mass-Index
pa-morphology-body-composition = Körperzusammensetzung
pa-morphology-build = Statur
pa-morphology-eye-colour = Augenfarbe
pa-morphology-eye-shape = Augenform
pa-morphology-eye-spacing = Augenabstand
pa-morphology-hair-colour = Natürliche Haarfarbe
pa-morphology-hair-texture = Haarstruktur
pa-morphology-hairline = Haaransatz
pa-morphology-facial-hair = Gesichtsbehaarung
pa-morphology-body-hair = Körperbehaarung
pa-morphology-skin-tone = Hauttyp (Fitzpatrick)
pa-morphology-skin-undertone = Unterton der Haut
pa-morphology-freckles = Sommersprossen
pa-morphology-pigmentation = Pigmentmale
pa-morphology-scars = Narben
pa-morphology-tattoos = Tätowierungen
pa-morphology-moles = Muttermale
pa-morphology-facial-asymmetries = Gesichtsasymmetrien
pa-morphology-face-shape = Gesichtsform
pa-morphology-nose-shape = Nasenform
pa-morphology-ear-shape = Ohrform
pa-morphology-lip-shape = Lippenform
pa-morphology-dentition = Gebiss
pa-morphology-malocclusion = Zahnfehlstellung (Angle-Klasse)
pa-morphology-posture = Körperhaltung
pa-morphology-gait = Gang
pa-morphology-distinguishing-features = Besondere Kennzeichen
pa-biometrics-fingerprints = Fingerabdrücke
pa-biometrics-retinal-print = Netzhautmuster
pa-biometrics-voice-signature = Stimmabdruck
pa-biometrics-voice-frequency = Grundfrequenz der Stimme
pa-biometrics-vocal-timbre = Stimmklang
pa-biometrics-spoken-accent = Akzent
pa-biometrics-speech-rate = Sprechtempo
pa-biometrics-verbal-tics = Sprachliche Ticks
pa-biometrics-frequent-vocabulary = Häufiger Wortschatz
pa-biometrics-speech-register = Sprachregister
pa-biometrics-motor-tics = Motorische Tics
pa-biometrics-handedness = Händigkeit
pa-biometrics-hearing = Gehör
pa-biometrics-visual-acuity = Sehschärfe
pa-biometrics-optical-correction = Sehhilfe
pa-health-blood-group = Blutgruppe (AB0)
pa-health-rhesus = Rhesusfaktor (RhD)
pa-health-blood-pressure = Blutdruck
pa-health-resting-heart-rate = Ruhepuls
pa-health-respiratory-capacity = Lungenfunktion
pa-health-conditions = Erkrankungen
pa-health-surgeries = Operationen
pa-health-injuries = Verletzungen
pa-health-deformities = Fehlbildungen
pa-health-amputations = Amputationen
pa-health-prostheses = Prothesen
pa-health-implants = Implantate
pa-health-devices = Implantierte Geräte
pa-health-medications = Medikamente
pa-health-allergies = Allergien
pa-health-vaccinations = Impfungen
pa-health-serology = Serologie
pa-health-lab-results = Laborwerte
pa-health-deficiencies = Mangelzustände
pa-health-sleep-disorders = Schlafstörungen
pa-health-mental-health-assessments = Psychische Gesundheit: Einschätzungen
pa-genomics-autosomal-mapping = Autosomaler DNA-Test
pa-genomics-y-haplogroup = Y-DNA-Haplogruppe
pa-genomics-mt-haplogroup = Mitochondriale Haplogruppe
pa-genomics-whole-genome-sequencing = Vollgenomsequenzierung
pa-genomics-risk-variants = Risikovarianten
pa-genomics-hereditary-conditions = Erbkrankheiten
pa-genomics-predispositions = Veranlagungen
pa-genomics-epigenetic-markers = Epigenetische Marker
pa-genomics-epigenetic-age = Epigenetisches Alter
pa-genomics-gut-microbiome = Darmmikrobiom
pa-genomics-skin-microbiome = Hautmikrobiom
pa-genomics-toxicological-sensitivities = Empfindlichkeit für Arzneistoffe und Gifte
pa-death-time = Todeszeit
pa-death-coordinates = Sterbeort als Koordinaten
pa-death-causes = Todesursachen
pa-death-contributing-factors = Mitwirkende Umstände
pa-death-autopsy = Obduktion
pa-death-disposition = Bestattungsart
pa-death-grave = Grab
pa-residence-addresses = Adressen
pa-residence-nationality-of-origin = Ursprüngliche Staatsangehörigkeit
pa-residence-acquired-nationalities = Erworbene Staatsangehörigkeiten
pa-residence-mother-tongue = Muttersprache
pa-residence-spoken-languages = Gesprochene Sprachen
pa-education-level = Bildungsstand
pa-education-diplomas = Abschlüsse und Titel
pa-education-institutions = Schulen und Einrichtungen
pa-education-income = Einkommen
pa-education-real-estate = Grundbesitz
pa-military-distinctions = Auszeichnungen
pa-military-citations = Belobigungen
pa-military-ranks = Dienstgrade
pa-military-units = Einheiten
pa-military-service-numbers = Personenkennziffern
pa-legal-criminal-record = Vorstrafen
pa-belief-religions = Religion
pa-belief-sacraments = Sakramente und Riten
pa-belief-beliefs = Überzeugungen
pa-belief-political-leanings = Politische Ausrichtung
pa-belief-memberships = Mitgliedschaften
pa-personality-big-five = Big-Five-Werte
pa-personality-mbti = MBTI-Typ
pa-personality-introversion-extraversion = Introversion und Extraversion
pa-personality-stress-tolerance = Belastbarkeit
pa-personality-decision-style = Entscheidungsstil
pa-personality-interests = Interessen
pa-personality-hobbies = Hobbys
pa-personality-sports = Sport
pa-personality-dietary-habits = Ernährung
pa-personality-dependencies = Abhängigkeiten
pa-digital-legacy-body-models = Körpermodelle
pa-digital-legacy-skin-textures = Hauttexturen
pa-digital-legacy-rigs = Skelett-Rigs
pa-digital-legacy-voice-corpora = Sprachaufnahmen für Stimmsynthese
pa-digital-legacy-text-corpora = Texte für ein Sprachmodell
pa-digital-legacy-digital-traces = Digitale Spuren
pa-digital-legacy-carbon-footprint = CO₂-Fußabdruck
pa-digital-legacy-behaviour-models = Verhaltensmodelle
pf-identity-titles-text = Titel laut Quelle
pf-identity-titles-kind = Art des Titels
pf-civil-status-marginal-annotations-text = Vermerk
pf-morphology-pigmentation-kind = Art des Mals
pf-biometrics-spoken-accent-description = Wie er beschrieben wird
pf-biometrics-optical-correction-kind = Sehhilfe
pf-health-amputations-level = Amputationshöhe
pf-health-prostheses-kind = Prothese
pf-health-implants-kind = Implantat
pf-health-devices-kind = Gerät
pf-health-allergies-type = Allergieart
pf-health-vaccinations-status = Impfstatus
pf-health-sleep-disorders-category = Art der Störung
pf-death-autopsy-kind = Obduktion
pf-education-institutions-name = Name der Einrichtung
pf-military-distinctions-name = Name der Auszeichnung
pf-military-distinctions-kind = Art der Auszeichnung
pf-military-citations-text = Wortlaut der Belobigung
pf-military-ranks-category = Dienstgradgruppe
pf-belief-political-leanings-position = Position auf der Links-rechts-Achse
pf-belief-memberships-kind = Art der Organisation
pf-digital-legacy-carbon-footprint-method = Schätzverfahren
pf-age-years = Alter in Jahren
pf-agreeableness = Verträglichkeit
pf-allergen = Allergen
pf-amount = Betrag
pf-analyte = Messgröße
pf-artefact-type = Art des Artefakts
pf-autoimmune = Autoimmun
pf-body-region = Körperregion
pf-bone-percent = Knochen
pf-carrier-status = Trägerstatus
pf-cause = Ursache
pf-chronic = Chronisch
pf-clock = Uhr
pf-condition = Erkrankung
pf-conferred-by = Verliehen von
pf-congenital = Angeboren
pf-conscientiousness = Gewissenhaftigkeit
pf-consent = Einwilligung
pf-coordinates = Koordinaten
pf-corrected = Mit Korrektur
pf-country = Land
pf-court = Gericht
pf-coverage = Abdeckung
pf-currency = Währung
pf-decimal = Visus (dezimal)
pf-denomination = Konfession
pf-derived-from-id = Abgeleitet von
pf-description = Beschreibung
pf-details = Einzelheiten
pf-diagnosis = Diagnose
pf-diameter-mm = Durchmesser
pf-diastolic = Diastolisch
pf-diet = Ernährungsform
pf-document-id = Dokument
pf-dose = Dosis
pf-ear = Ohr
pf-entry-number = Eintragsnummer
pf-extraversion = Extraversion
pf-eye = Auge
pf-fat-percent = Fett
pf-fev1-fvc-ratio = FEV1/FVC-Verhältnis
pf-fev1-litres = FEV1
pf-file-format = Dateiformat
pf-findings = Befund
pf-flag = Kennzeichnung
pf-format = Format
pf-fracture = Knochenbruch
pf-fvc-litres = FVC
pf-gene = Gen
pf-generator = Erstellt mit
pf-grade = Grad
pf-iccs-section = Deliktabschnitt (ICCS)
pf-icd10-chapter = ICD-10-Kapitel
pf-indication = Indikation
pf-inheritance = Erbgang
pf-inscription = Inschrift
pf-institution = Einrichtung
pf-instrument = Verfahren
pf-isced-level = ISCED-Stufe
pf-jurisdiction = Rechtsordnung
pf-language = Sprache
pf-lat = Breitengrad
pf-level = Stufe
pf-lines = Anschrift
pf-location = Lage
pf-lon = Längengrad
pf-major = Haupt-Haplogruppe
pf-marker = Marker
pf-metaboliser-status = Metabolisierertyp
pf-method = Methode
pf-mode = Erwerbsart
pf-muscle-percent = Muskeln
pf-neuroticism = Neurotizismus
pf-number = Nummer
pf-nutrient = Nährstoff
pf-offence = Delikt
pf-office = Amt
pf-openness = Offenheit
pf-organisation = Organisation
pf-outcome = Ausgang
pf-pace = Alterungstempo
pf-page = Seite
pf-panel = Laborprofil
pf-party = Partei
pf-pathogen = Erreger
pf-pattern = Konsummuster
pf-percentile = Perzentil
pf-period = Auszahlungszeitraum
pf-place-id = Ort
pf-plot = Grabstelle
pf-polygenic-score = Polygener Score
pf-postal-code = Postleitzahl
pf-precision = Genauigkeit
pf-prescription = Verordnung
pf-proficiency = Sprachniveau
pf-provider = Anbieter
pf-quintile = Einkommensquintil
pf-rank = Dienstgrad
pf-rank-text = Dienstgrad laut Quelle
pf-reaction = Reaktion
pf-reference-build = Referenzgenom
pf-reference-high = Referenzbereich, obere Grenze
pf-reference-low = Referenzbereich, untere Grenze
pf-register-type = Eintragsart
pf-result = Ergebnis
pf-role = Funktion
pf-sacrament = Sakrament oder Ritus
pf-score = Punktwert
pf-sentence = Strafe
pf-sequence = Stelle in der Kausalkette
pf-service = Teilstreitkraft
pf-severity = Schweregrad
pf-shannon-diversity = Shannon-Diversität
pf-shape = Form
pf-significance = Klinische Bedeutung
pf-snp-count = Untersuchte SNPs
pf-sport = Sportart
pf-subclade = Subklade
pf-substance = Substanz
pf-summary = Zusammenfassung
pf-systolic = Systolisch
pf-tenure = Besitzverhältnis
pf-test = Test
pf-threshold-db = Hörschwelle
pf-title = Bezeichnung
pf-tonnes-co2e-per-year = Emissionen
pf-tradition = Tradition
pf-tree-version = Stammbaumversion
pf-unit = Einheit
pf-use = Nutzung
pf-variant = Variante
pf-volume = Band
pf-zygosity = Zygotie
pu-cm = { $n } cm
pu-kg = { $n } kg
pu-kg-m2 = { $n } kg/m²
pu-percent = { $n } %
pu-mm = { $n } mm
pu-hz = { $n } Hz
pu-words-min = { $n } Wörter/min
pu-db-hl = { $n } dB HL
pu-mmhg = { $n } mmHg
pu-bpm = { $n }/min
pu-litres = { $n } l
pu-coverage = { $n }×
pu-years = { $n } Jahre
pu-t-co2e-yr = { $n } t CO₂e pro Jahr
pv-sensitive-class-health = Gesundheit und Weltanschauung
pv-sensitive-class-biometrics = Biometrische Daten
pv-sensitive-class-genomics = Genomische Daten
pv-sensitive-class-legal = Strafregister
pv-laterality-left = Links
pv-laterality-right = Rechts
pv-laterality-both = Beidseitig
pv-body-region-head = Kopf
pv-body-region-face = Gesicht
pv-body-region-neck = Hals
pv-body-region-left-shoulder = Linke Schulter
pv-body-region-right-shoulder = Rechte Schulter
pv-body-region-left-arm = Linker Arm
pv-body-region-right-arm = Rechter Arm
pv-body-region-left-hand = Linke Hand
pv-body-region-right-hand = Rechte Hand
pv-body-region-chest = Brust
pv-body-region-abdomen = Bauch
pv-body-region-upper-back = Oberer Rücken
pv-body-region-lower-back = Unterer Rücken
pv-body-region-pelvis = Becken und Hüfte
pv-body-region-left-leg = Linkes Bein
pv-body-region-right-leg = Rechtes Bein
pv-body-region-left-foot = Linker Fuß
pv-body-region-right-foot = Rechter Fuß
pv-body-region-internal = Im Körperinneren
pv-body-region-whole-body = Ganzer Körper
pv-body-region-other = Andere Region
pv-artefact-type-mesh = Polygonnetz
pv-artefact-type-point-cloud = Punktwolke
pv-artefact-type-skin-texture-map = Hauttextur
pv-artefact-type-skeletal-rig = Skelett-Rig
pv-artefact-type-voice-corpus = Sprachkorpus
pv-artefact-type-text-corpus = Textkorpus
pv-artefact-type-trace-archive = Archiv der Online-Aktivität
pv-artefact-type-behaviour-model = Verhaltensmodell
pv-artefact-type-fingerprint-card = Fingerabdruckblatt
pv-artefact-type-fingerprint-template = Fingerabdruck-Template
pv-artefact-type-retinal-image = Netzhautbild
pv-artefact-type-voiceprint = Stimmabdruck
pv-consent-given = Erteilt
pv-consent-given-by-estate = Von den Erben erteilt
pv-consent-refused = Verweigert
pv-consent-withdrawn = Widerrufen
pv-consent-not-asked = Nicht erfragt
pv-consent-unknown = Unbekannt
pv-sex-at-birth-female = Weiblich
pv-sex-at-birth-male = Männlich
pv-sex-at-birth-intersex = Intergeschlechtlich
pv-sex-at-birth-undetermined = Unbestimmt
pv-sex-at-birth-unknown = Unbekannt
pv-gender-identity-woman = Frau
pv-gender-identity-man = Mann
pv-gender-identity-non-binary = Nichtbinär
pv-gender-identity-other = Andere
pv-gender-identity-undisclosed = Nicht angegeben
pv-gender-identity-unknown = Unbekannt
pv-title-kind-nobility = Adelstitel
pv-title-kind-academic = Akademischer Titel
pv-title-kind-professional = Berufstitel
pv-title-kind-religious = Geistlicher Titel
pv-title-kind-military = Militärischer Titel
pv-title-kind-civic = Ehrentitel
pv-title-kind-courtesy = Höflichkeitstitel
pv-title-kind-other = Anderer
pv-register-type-birth = Geburt
pv-register-type-baptism = Taufe
pv-register-type-marriage = Eheschließung
pv-register-type-death = Tod
pv-register-type-burial = Beerdigung
pv-register-type-divorce = Scheidung
pv-register-type-recognition = Vaterschaftsanerkennung
pv-register-type-legitimation = Legitimation
pv-register-type-adoption = Adoption
pv-register-type-name-change = Namensänderung
pv-register-type-other = Andere
pv-build-slight = Zierlich
pv-build-slim = Schlank
pv-build-average = Mittel
pv-build-sturdy = Kräftig
pv-build-stout = Untersetzt
pv-build-heavy = Schwer
pv-eye-colour-light-blue = Hellblau
pv-eye-colour-blue = Blau
pv-eye-colour-dark-blue = Dunkelblau
pv-eye-colour-grey = Grau
pv-eye-colour-blue-grey = Blaugrau
pv-eye-colour-green = Grün
pv-eye-colour-grey-green = Graugrün
pv-eye-colour-hazel = Haselnussbraun
pv-eye-colour-amber = Bernsteinfarben
pv-eye-colour-light-brown = Hellbraun
pv-eye-colour-brown = Braun
pv-eye-colour-dark-brown = Dunkelbraun
pv-eye-colour-black = Schwarz
pv-eye-colour-mixed = Gemischt
pv-eye-colour-other = Andere
pv-eye-shape-almond = Mandelförmig
pv-eye-shape-round = Rund
pv-eye-shape-hooded = Schlupflid
pv-eye-shape-monolid = Monolid
pv-eye-shape-deep-set = Tief liegend
pv-eye-shape-protruding = Hervortretend
pv-eye-shape-upturned = Nach oben geneigt
pv-eye-shape-downturned = Nach unten geneigt
pv-eye-shape-other = Andere
pv-eye-spacing-close-set = Eng stehend
pv-eye-spacing-average = Mittel
pv-eye-spacing-wide-set = Weit auseinander
pv-hair-colour-black = Schwarz
pv-hair-colour-dark-brown = Dunkelbraun
pv-hair-colour-brown = Braun
pv-hair-colour-light-brown = Hellbraun
pv-hair-colour-auburn = Kastanienbraun
pv-hair-colour-red = Rot
pv-hair-colour-strawberry-blond = Rotblond
pv-hair-colour-dark-blond = Dunkelblond
pv-hair-colour-blond = Blond
pv-hair-colour-light-blond = Hellblond
pv-hair-colour-grey = Grau
pv-hair-colour-white = Weiß
pv-hair-colour-none = Keine Haare
pv-hair-colour-other = Andere
pv-hair-texture-straight = Glatt
pv-hair-texture-wavy = Wellig
pv-hair-texture-curly = Lockig
pv-hair-texture-coily = Kraus
pv-hair-texture-other = Andere
pv-hairline-straight = Gerade
pv-hairline-rounded = Gerundet
pv-hairline-widows-peak = Witwenspitze
pv-hairline-m-shaped = M-förmig
pv-hairline-bell-shaped = Glockenförmig
pv-hairline-uneven = Unregelmäßig
pv-hairline-receding = Zurückweichend
pv-hairline-bald = Glatze
pv-facial-hair-none = Keine
pv-facial-hair-stubble = Dreitagebart
pv-facial-hair-moustache = Schnurrbart
pv-facial-hair-goatee = Spitzbart
pv-facial-hair-full-beard = Vollbart
pv-facial-hair-sideburns = Koteletten
pv-facial-hair-other = Andere
pv-body-hair-none = Keine
pv-body-hair-sparse = Spärlich
pv-body-hair-moderate = Mäßig
pv-body-hair-dense = Dicht
pv-skin-tone-type-i = Typ I — verbrennt immer, bräunt nie
pv-skin-tone-type-ii = Typ II — verbrennt meist, bräunt kaum
pv-skin-tone-type-iii = Typ III — verbrennt manchmal, bräunt gleichmäßig
pv-skin-tone-type-iv = Typ IV — verbrennt selten, bräunt gut
pv-skin-tone-type-v = Typ V — verbrennt sehr selten
pv-skin-tone-type-vi = Typ VI — verbrennt nie
pv-skin-undertone-cool = Kühl
pv-skin-undertone-neutral = Neutral
pv-skin-undertone-warm = Warm
pv-skin-undertone-olive = Oliv
pv-freckles-none = Keine
pv-freckles-few = Wenige
pv-freckles-moderate = Mäßig viele
pv-freckles-many = Viele
pv-pigmentation-mark-birthmark = Muttermal
pv-pigmentation-mark-port-wine-stain = Feuermal
pv-pigmentation-mark-cafe-au-lait-spot = Café-au-lait-Fleck
pv-pigmentation-mark-depigmented-patch = Pigmentarmer Fleck
pv-pigmentation-mark-hyperpigmented-patch = Pigmentreicher Fleck
pv-pigmentation-mark-other = Andere
pv-mole-shape-round = Rund
pv-mole-shape-oval = Oval
pv-mole-shape-irregular = Unregelmäßig
pv-mole-shape-other = Andere
pv-face-shape-oval = Oval
pv-face-shape-round = Rund
pv-face-shape-square = Eckig
pv-face-shape-oblong = Länglich
pv-face-shape-heart = Herzförmig
pv-face-shape-diamond = Rautenförmig
pv-face-shape-triangular = Dreieckig
pv-nose-shape-straight = Gerade
pv-nose-shape-aquiline = Adlernase
pv-nose-shape-snub = Stupsnase
pv-nose-shape-upturned = Nach oben gebogen
pv-nose-shape-flat = Flach
pv-nose-shape-broad = Breit
pv-nose-shape-bulbous = Knollennase
pv-nose-shape-crooked = Schief
pv-nose-shape-other = Andere
pv-ear-shape-free-lobe = Freie Ohrläppchen
pv-ear-shape-attached-lobe = Angewachsene Ohrläppchen
pv-ear-shape-protruding = Abstehend
pv-ear-shape-close-set = Anliegend
pv-ear-shape-pointed = Spitz
pv-ear-shape-other = Andere
pv-lip-shape-thin = Schmal
pv-lip-shape-medium = Mittel
pv-lip-shape-full = Voll
pv-lip-shape-bow-shaped = Bogenförmig
pv-lip-shape-wide = Breit
pv-lip-shape-downturned = Hängende Mundwinkel
pv-lip-shape-other = Andere
pv-dentition-primary = Milchgebiss
pv-dentition-mixed = Wechselgebiss
pv-dentition-permanent-complete = Bleibend, vollständig
pv-dentition-permanent-partial-loss = Bleibend, lückenhaft
pv-dentition-edentulous = Zahnlos
pv-dentition-partial-denture = Teilprothese
pv-dentition-full-denture = Vollprothese
pv-dentition-implants = Zahnimplantate
pv-malocclusion-normal = Normale Okklusion
pv-malocclusion-class-i = Klasse I
pv-malocclusion-class-ii-division-1 = Klasse II, Unterklasse 1
pv-malocclusion-class-ii-division-2 = Klasse II, Unterklasse 2
pv-malocclusion-class-iii = Klasse III
pv-posture-ideal = Aufrecht
pv-posture-kyphotic-lordotic = Hohlrundrücken
pv-posture-flat-back = Flachrücken
pv-posture-sway-back = Hohlrücken
pv-posture-stooped = Gebeugt
pv-posture-scoliotic = Skoliotisch
pv-posture-other = Andere
pv-gait-brisk = Zügig
pv-gait-average = Normal
pv-gait-slow = Langsam
pv-gait-shuffling = Schlurfend
pv-gait-limping = Hinkend
pv-gait-waddling = Watschelnd
pv-gait-unsteady = Unsicher
pv-gait-stiff = Steif
pv-gait-other = Andere
pv-vocal-timbre-bright = Hell
pv-vocal-timbre-dark = Dunkel
pv-vocal-timbre-warm = Warm
pv-vocal-timbre-breathy = Behaucht
pv-vocal-timbre-nasal = Nasal
pv-vocal-timbre-hoarse = Heiser
pv-vocal-timbre-resonant = Volltönend
pv-vocal-timbre-thin = Dünn
pv-vocal-timbre-other = Anderer
pv-speech-register-frozen = Feierlich
pv-speech-register-formal = Förmlich
pv-speech-register-consultative = Sachlich
pv-speech-register-casual = Umgangssprachlich
pv-speech-register-intimate = Vertraut
pv-handedness-left = Linkshänder
pv-handedness-right = Rechtshänder
pv-handedness-ambidextrous = Beidhänder
pv-handedness-mixed = Gemischt
pv-handedness-unknown = Unbekannt
pv-hearing-grade-normal = Normal
pv-hearing-grade-mild = Leicht
pv-hearing-grade-moderate = Mittelgradig
pv-hearing-grade-moderately-severe = Mittelgradig bis schwer
pv-hearing-grade-severe = Schwer
pv-hearing-grade-profound = Hochgradig
pv-hearing-grade-complete = Taub
pv-optical-correction-none = Keine
pv-optical-correction-glasses = Brille
pv-optical-correction-contact-lenses = Kontaktlinsen
pv-optical-correction-glasses-and-contact-lenses = Brille und Kontaktlinsen
pv-optical-correction-refractive-surgery = Refraktive Chirurgie
pv-optical-correction-intraocular-lens = Intraokularlinse
pv-optical-correction-other = Andere
pv-rhesus-positive = RhD positiv
pv-rhesus-negative = RhD negativ
pv-rhesus-weak-d = Schwach D
pv-rhesus-unknown = Unbekannt
pv-icd10-chapter-infectious-parasitic = I Infektiöse und parasitäre Krankheiten
pv-icd10-chapter-neoplasms = II Neubildungen
pv-icd10-chapter-blood-immune = III Blut und Immunsystem
pv-icd10-chapter-endocrine-metabolic = IV Endokrine, Ernährungs- und Stoffwechselkrankheiten
pv-icd10-chapter-mental-behavioural = V Psychische und Verhaltensstörungen
pv-icd10-chapter-nervous-system = VI Nervensystem
pv-icd10-chapter-eye-adnexa = VII Auge und Augenanhangsgebilde
pv-icd10-chapter-ear-mastoid = VIII Ohr und Warzenfortsatz
pv-icd10-chapter-circulatory = IX Kreislaufsystem
pv-icd10-chapter-respiratory = X Atmungssystem
pv-icd10-chapter-digestive = XI Verdauungssystem
pv-icd10-chapter-skin = XII Haut und Unterhaut
pv-icd10-chapter-musculoskeletal = XIII Muskel-Skelett-System
pv-icd10-chapter-genitourinary = XIV Urogenitalsystem
pv-icd10-chapter-pregnancy-childbirth = XV Schwangerschaft und Geburt
pv-icd10-chapter-perinatal = XVI Perinatalperiode
pv-icd10-chapter-congenital = XVII Angeborene Fehlbildungen
pv-icd10-chapter-ill-defined = XVIII Symptome und ungenau bezeichnete Ursachen
pv-icd10-chapter-injury-poisoning = XIX Verletzungen und Vergiftungen
pv-icd10-chapter-external-causes = XX Äußere Ursachen
pv-icd10-chapter-health-factors = XXI Faktoren, die den Gesundheitszustand beeinflussen
pv-icd10-chapter-special-purposes = XXII Schlüsselnummern für besondere Zwecke
pv-diagnosis-status-diagnosed = Ärztlich festgestellt
pv-diagnosis-status-suspected = Vermutet
pv-diagnosis-status-self-reported = Selbst angegeben
pv-diagnosis-status-unknown = Unbekannt
pv-prosthesis-kind-limb = Gliedmaße
pv-prosthesis-kind-joint = Gelenkersatz
pv-prosthesis-kind-ocular = Augenprothese
pv-prosthesis-kind-dental = Zahnprothese
pv-prosthesis-kind-auditory = Hörprothese
pv-prosthesis-kind-breast = Brustprothese
pv-prosthesis-kind-other = Andere
pv-implant-kind-orthopaedic = Orthopädisch
pv-implant-kind-dental = Zahnimplantat
pv-implant-kind-cochlear = Cochlea-Implantat
pv-implant-kind-breast = Brustimplantat
pv-implant-kind-intraocular-lens = Intraokularlinse
pv-implant-kind-contraceptive = Verhütungsimplantat
pv-implant-kind-cosmetic = Kosmetisch
pv-implant-kind-other = Anderes
pv-device-kind-pacemaker = Herzschrittmacher
pv-device-kind-implantable-defibrillator = Implantierbarer Defibrillator
pv-device-kind-cardiac-resynchronisation = Resynchronisationsgerät
pv-device-kind-ventricular-assist = Herzunterstützungssystem
pv-device-kind-neurostimulator = Neurostimulator
pv-device-kind-insulin-pump = Insulinpumpe
pv-device-kind-drug-port = Portsystem
pv-device-kind-shunt = Shunt
pv-device-kind-stent = Stent
pv-device-kind-other = Anderes
pv-allergy-type-drug = Arzneimittel
pv-allergy-type-food = Nahrungsmittel
pv-allergy-type-environmental = Umwelt
pv-allergy-type-insect-venom = Insektengift
pv-allergy-type-latex = Latex
pv-allergy-type-other = Andere
pv-allergy-severity-mild = Leicht
pv-allergy-severity-moderate = Mittel
pv-allergy-severity-severe = Schwer
pv-allergy-severity-anaphylactic = Anaphylaktisch
pv-allergy-severity-unknown = Unbekannt
pv-pathogen-diphtheria = Diphtherie
pv-pathogen-tetanus = Tetanus
pv-pathogen-pertussis = Keuchhusten
pv-pathogen-poliomyelitis = Kinderlähmung
pv-pathogen-measles = Masern
pv-pathogen-mumps = Mumps
pv-pathogen-rubella = Röteln
pv-pathogen-varicella = Windpocken
pv-pathogen-smallpox = Pocken
pv-pathogen-tuberculosis = Tuberkulose
pv-pathogen-hepatitis-a = Hepatitis A
pv-pathogen-hepatitis-b = Hepatitis B
pv-pathogen-hepatitis-c = Hepatitis C
pv-pathogen-haemophilus-influenzae-b = Haemophilus influenzae Typ b
pv-pathogen-pneumococcal = Pneumokokken
pv-pathogen-meningococcal = Meningokokken
pv-pathogen-human-papillomavirus = Humane Papillomviren
pv-pathogen-influenza = Grippe
pv-pathogen-covid-19 = COVID-19
pv-pathogen-rotavirus = Rotaviren
pv-pathogen-yellow-fever = Gelbfieber
pv-pathogen-typhoid = Typhus
pv-pathogen-cholera = Cholera
pv-pathogen-rabies = Tollwut
pv-pathogen-japanese-encephalitis = Japanische Enzephalitis
pv-pathogen-tick-borne-encephalitis = FSME
pv-pathogen-hiv = HIV
pv-pathogen-syphilis = Syphilis
pv-pathogen-toxoplasmosis = Toxoplasmose
pv-pathogen-cytomegalovirus = Zytomegalievirus
pv-pathogen-epstein-barr = Epstein-Barr-Virus
pv-pathogen-other = Anderer
pv-vaccination-status-vaccinated = Geimpft
pv-vaccination-status-partially-vaccinated = Teilweise geimpft
pv-vaccination-status-unvaccinated = Ungeimpft
pv-vaccination-status-contraindicated = Kontraindiziert
pv-vaccination-status-unknown = Unbekannt
pv-serology-result-positive = Positiv
pv-serology-result-negative = Negativ
pv-serology-result-equivocal = Grenzwertig
pv-serology-result-unknown = Unbekannt
pv-lab-panel-basic-metabolic = Basis-Stoffwechselprofil
pv-lab-panel-lipid = Lipidprofil
pv-lab-panel-liver = Leberwerte
pv-lab-panel-renal = Nierenwerte
pv-lab-panel-glycated-haemoglobin = Glykiertes Hämoglobin
pv-lab-panel-iron = Eisenstoffwechsel
pv-lab-analyte-sodium = Natrium
pv-lab-analyte-potassium = Kalium
pv-lab-analyte-chloride = Chlorid
pv-lab-analyte-bicarbonate = Bicarbonat
pv-lab-analyte-urea = Harnstoff
pv-lab-analyte-creatinine = Kreatinin
pv-lab-analyte-glucose = Glukose
pv-lab-analyte-calcium = Calcium
pv-lab-analyte-total-cholesterol = Gesamtcholesterin
pv-lab-analyte-ldl-cholesterol = LDL-Cholesterin
pv-lab-analyte-hdl-cholesterol = HDL-Cholesterin
pv-lab-analyte-triglycerides = Triglyceride
pv-lab-analyte-non-hdl-cholesterol = Non-HDL-Cholesterin
pv-lab-analyte-alt = Alanin-Aminotransferase (ALT)
pv-lab-analyte-ast = Aspartat-Aminotransferase (AST)
pv-lab-analyte-alp = Alkalische Phosphatase (AP)
pv-lab-analyte-ggt = Gamma-GT
pv-lab-analyte-total-bilirubin = Gesamtbilirubin
pv-lab-analyte-direct-bilirubin = Direktes Bilirubin
pv-lab-analyte-albumin = Albumin
pv-lab-analyte-total-protein = Gesamteiweiß
pv-lab-analyte-egfr = Geschätzte GFR
pv-lab-analyte-uric-acid = Harnsäure
pv-lab-analyte-phosphate = Phosphat
pv-lab-analyte-urine-albumin-creatinine-ratio = Albumin-Kreatinin-Quotient im Urin
pv-lab-analyte-hba1c = HbA1c
pv-lab-analyte-serum-iron = Serumeisen
pv-lab-analyte-ferritin = Ferritin
pv-lab-analyte-transferrin = Transferrin
pv-lab-analyte-transferrin-saturation = Transferrinsättigung
pv-lab-analyte-tibc = Totale Eisenbindungskapazität
pv-lab-flag-low = Erniedrigt
pv-lab-flag-normal = Normal
pv-lab-flag-high = Erhöht
pv-lab-flag-critical-low = Kritisch erniedrigt
pv-lab-flag-critical-high = Kritisch erhöht
pv-nutrient-vitamin-a = Vitamin A
pv-nutrient-thiamine = Thiamin (B1)
pv-nutrient-riboflavin = Riboflavin (B2)
pv-nutrient-niacin = Niacin (B3)
pv-nutrient-vitamin-b6 = Vitamin B6
pv-nutrient-folate = Folsäure (B9)
pv-nutrient-vitamin-b12 = Vitamin B12
pv-nutrient-vitamin-c = Vitamin C
pv-nutrient-vitamin-d = Vitamin D
pv-nutrient-vitamin-e = Vitamin E
pv-nutrient-vitamin-k = Vitamin K
pv-nutrient-iron = Eisen
pv-nutrient-zinc = Zink
pv-nutrient-magnesium = Magnesium
pv-nutrient-calcium = Calcium
pv-nutrient-iodine = Jod
pv-nutrient-selenium = Selen
pv-nutrient-copper = Kupfer
pv-nutrient-potassium = Kalium
pv-nutrient-phosphorus = Phosphor
pv-nutrient-other = Anderer
pv-sleep-disorder-insomnia = Insomnie
pv-sleep-disorder-sleep-related-breathing = Schlafbezogene Atmungsstörung
pv-sleep-disorder-central-hypersomnolence = Zentrale Hypersomnie
pv-sleep-disorder-circadian-rhythm = Störung des Schlaf-Wach-Rhythmus
pv-sleep-disorder-parasomnia = Parasomnie
pv-sleep-disorder-sleep-related-movement = Schlafbezogene Bewegungsstörung
pv-sleep-disorder-other = Andere
pv-assessment-instrument-phq-9 = PHQ-9
pv-assessment-instrument-gad-7 = GAD-7
pv-assessment-instrument-bdi-ii = BDI-II
pv-assessment-instrument-hads = HADS
pv-assessment-instrument-k10 = K10
pv-assessment-instrument-gds-15 = GDS-15
pv-assessment-instrument-mmse = MMSE
pv-assessment-instrument-moca = MoCA
pv-assessment-instrument-audit = AUDIT
pv-assessment-instrument-clinical-interview = Klinisches Interview
pv-assessment-instrument-other = Anderes
pv-assessment-severity-none-minimal = Keine oder minimal
pv-assessment-severity-mild = Leicht
pv-assessment-severity-moderate = Mittelgradig
pv-assessment-severity-moderately-severe = Mittelgradig bis schwer
pv-assessment-severity-severe = Schwer
pv-reference-build-grch36 = GRCh36 (hg18)
pv-reference-build-grch37 = GRCh37 (hg19)
pv-reference-build-grch38 = GRCh38 (hg38)
pv-reference-build-t2t-chm13 = T2T-CHM13
pv-genomic-file-format-raw-microarray = Rohdaten eines Genotypisierungschips
pv-genomic-file-format-fastq = FASTQ
pv-genomic-file-format-bam = BAM
pv-genomic-file-format-cram = CRAM
pv-genomic-file-format-vcf = VCF
pv-genomic-file-format-gvcf = gVCF
pv-genomic-file-format-other = Anderes
pv-zygosity-heterozygous = Heterozygot
pv-zygosity-homozygous = Homozygot
pv-zygosity-hemizygous = Hemizygot
pv-zygosity-compound-heterozygous = Compound-heterozygot
pv-clinical-significance-pathogenic = Pathogen
pv-clinical-significance-likely-pathogenic = Wahrscheinlich pathogen
pv-clinical-significance-uncertain-significance = Unklare Bedeutung
pv-clinical-significance-likely-benign = Wahrscheinlich benigne
pv-clinical-significance-benign = Benigne
pv-inheritance-pattern-autosomal-dominant = Autosomal-dominant
pv-inheritance-pattern-autosomal-recessive = Autosomal-rezessiv
pv-inheritance-pattern-x-linked-dominant = X-chromosomal-dominant
pv-inheritance-pattern-x-linked-recessive = X-chromosomal-rezessiv
pv-inheritance-pattern-y-linked = Y-chromosomal
pv-inheritance-pattern-mitochondrial = Mitochondrial
pv-inheritance-pattern-multifactorial = Multifaktoriell
pv-inheritance-pattern-unknown = Unbekannt
pv-carrier-status-affected = Betroffen
pv-carrier-status-carrier = Anlageträger
pv-carrier-status-not-carrier = Kein Anlageträger
pv-carrier-status-unknown = Unbekannt
pv-epigenetic-clock-horvath = Horvath
pv-epigenetic-clock-hannum = Hannum
pv-epigenetic-clock-phenoage = PhenoAge
pv-epigenetic-clock-grimage = GrimAge
pv-epigenetic-clock-dunedinpace = DunedinPACE
pv-epigenetic-clock-other = Andere
pv-metaboliser-status-poor = Langsamer Metabolisierer
pv-metaboliser-status-intermediate = Intermediärer Metabolisierer
pv-metaboliser-status-normal = Normaler Metabolisierer
pv-metaboliser-status-rapid = Schneller Metabolisierer
pv-metaboliser-status-ultrarapid = Ultraschneller Metabolisierer
pv-autopsy-not-performed = Nicht durchgeführt
pv-autopsy-clinical = Klinische Obduktion
pv-autopsy-forensic = Gerichtliche Obduktion
pv-autopsy-external-examination = Nur äußere Leichenschau
pv-autopsy-unknown = Unbekannt
pv-disposition-burial = Erdbestattung
pv-disposition-cremation = Feuerbestattung
pv-disposition-entombment = Beisetzung in einer Gruft
pv-disposition-burial-at-sea = Seebestattung
pv-disposition-natural-burial = Naturbestattung
pv-disposition-body-donation = Körperspende
pv-disposition-other = Andere
pv-disposition-unknown = Unbekannt
pv-address-use-principal = Hauptwohnsitz
pv-address-use-secondary = Nebenwohnsitz
pv-address-use-temporary = Vorübergehender Aufenthalt
pv-address-use-postal = Postanschrift
pv-address-use-other = Andere
pv-nationality-mode-descent = Durch Abstammung
pv-nationality-mode-birth-in-territory = Durch Geburt im Staatsgebiet
pv-nationality-mode-naturalisation = Durch Einbürgerung
pv-nationality-mode-marriage = Durch Eheschließung
pv-nationality-mode-registration = Durch Erklärung
pv-nationality-mode-restoration = Durch Wiedereinbürgerung
pv-nationality-mode-state-succession = Durch Gebietswechsel
pv-nationality-mode-other = Anders
pv-language-proficiency-a1 = A1 Anfänger
pv-language-proficiency-a2 = A2 Grundlegende Kenntnisse
pv-language-proficiency-b1 = B1 Fortgeschrittene Sprachverwendung
pv-language-proficiency-b2 = B2 Selbständige Sprachverwendung
pv-language-proficiency-c1 = C1 Fachkundige Sprachkenntnisse
pv-language-proficiency-c2 = C2 Annähernd muttersprachlich
pv-language-proficiency-native = Erstsprache
pv-isced-level-isced-0 = 0 Frühkindliche Bildung
pv-isced-level-isced-1 = 1 Primarbereich
pv-isced-level-isced-2 = 2 Sekundarbereich I
pv-isced-level-isced-3 = 3 Sekundarbereich II
pv-isced-level-isced-4 = 4 Postsekundärer nichttertiärer Bereich
pv-isced-level-isced-5 = 5 Kurzes tertiäres Bildungsprogramm
pv-isced-level-isced-6 = 6 Bachelor oder gleichwertig
pv-isced-level-isced-7 = 7 Master oder gleichwertig
pv-isced-level-isced-8 = 8 Promotion oder gleichwertig
pv-income-quintile-q1 = Unterstes Fünftel
pv-income-quintile-q2 = Zweites Fünftel
pv-income-quintile-q3 = Mittleres Fünftel
pv-income-quintile-q4 = Viertes Fünftel
pv-income-quintile-q5 = Oberstes Fünftel
pv-pay-period-hourly = Pro Stunde
pv-pay-period-daily = Pro Tag
pv-pay-period-weekly = Pro Woche
pv-pay-period-monthly = Pro Monat
pv-pay-period-annual = Pro Jahr
pv-tenure-owned = Eigentum
pv-tenure-co-owned = Miteigentum
pv-tenure-leasehold = Erbbaurecht
pv-tenure-rented = Miete
pv-tenure-usufruct = Nießbrauch
pv-tenure-other = Anderes
pv-distinction-kind-order = Orden
pv-distinction-kind-decoration = Ehrenzeichen
pv-distinction-kind-medal = Medaille
pv-distinction-kind-title = Ehrentitel
pv-distinction-kind-other = Andere
pv-military-service-army = Heer
pv-military-service-navy = Marine
pv-military-service-air-force = Luftwaffe
pv-military-service-marines = Marineinfanterie
pv-military-service-gendarmerie = Gendarmerie
pv-military-service-border-guard = Grenzschutz
pv-military-service-national-guard = Nationalgarde
pv-military-service-other = Andere
pv-rank-category-enlisted = Mannschaften
pv-rank-category-non-commissioned = Unteroffiziere
pv-rank-category-warrant = Stabsunteroffiziere und Gleichgestellte
pv-rank-category-officer-cadet = Offizieranwärter
pv-rank-category-junior-officer = Leutnante und Hauptleute
pv-rank-category-senior-officer = Stabsoffiziere
pv-rank-category-general-officer = Generale
pv-iccs-section-acts-leading-to-death = 01 Handlungen mit Todesfolge
pv-iccs-section-acts-causing-harm = 02 Körperverletzende Handlungen
pv-iccs-section-sexual-acts = 03 Schädigende Handlungen sexueller Art
pv-iccs-section-property-with-violence = 04 Gegen Eigentum mit Gewalt
pv-iccs-section-property-only = 05 Nur gegen Eigentum
pv-iccs-section-controlled-substances = 06 Kontrollierte Substanzen
pv-iccs-section-fraud-deception-corruption = 07 Betrug, Täuschung oder Korruption
pv-iccs-section-public-order-and-state = 08 Gegen die öffentliche Ordnung und den Staat
pv-iccs-section-public-safety-and-security = 09 Gegen die öffentliche Sicherheit
pv-iccs-section-natural-environment = 10 Gegen die Umwelt
pv-iccs-section-other-criminal-acts = 11 Andere Straftaten
pv-case-outcome-convicted = Verurteilt
pv-case-outcome-acquitted = Freigesprochen
pv-case-outcome-dismissed = Eingestellt
pv-case-outcome-conviction-quashed = Urteil aufgehoben
pv-case-outcome-pardoned = Begnadigt
pv-case-outcome-amnestied = Amnestiert
pv-case-outcome-expunged = Getilgt
pv-case-outcome-pending = Anhängig
pv-case-outcome-unknown = Unbekannt
pv-religion-buddhism = Buddhismus
pv-religion-christianity-catholic = Christentum: katholisch
pv-religion-christianity-orthodox = Christentum: orthodox
pv-religion-christianity-protestant = Christentum: evangelisch
pv-religion-christianity-other = Christentum: andere
pv-religion-hinduism = Hinduismus
pv-religion-islam-sunni = Islam: sunnitisch
pv-religion-islam-shia = Islam: schiitisch
pv-religion-islam-other = Islam: andere
pv-religion-jainism = Jainismus
pv-religion-judaism = Judentum
pv-religion-sikhism = Sikhismus
pv-religion-bahai = Bahaitum
pv-religion-shinto = Shintō
pv-religion-taoism = Daoismus
pv-religion-zoroastrianism = Zoroastrismus
pv-religion-traditional = Volks- oder Naturreligion
pv-religion-other = Andere
pv-religion-none = Konfessionslos
pv-religion-unknown = Unbekannt
pv-sacrament-baptism = Taufe
pv-sacrament-confirmation = Firmung oder Konfirmation
pv-sacrament-first-communion = Erstkommunion
pv-sacrament-reconciliation = Beichte
pv-sacrament-anointing-of-the-sick = Krankensalbung
pv-sacrament-holy-orders = Weihe
pv-sacrament-matrimony = Ehe
pv-sacrament-other-rite = Anderer Ritus
pv-political-position-far-left = Linksaußen
pv-political-position-left = Links
pv-political-position-centre-left = Mitte-links
pv-political-position-centre = Mitte
pv-political-position-centre-right = Mitte-rechts
pv-political-position-right = Rechts
pv-political-position-far-right = Rechtsaußen
pv-political-position-apolitical = Unpolitisch
pv-political-position-other = Außerhalb dieser Achse
pv-political-position-unknown = Unbekannt
pv-membership-kind-trade-union = Gewerkschaft
pv-membership-kind-political-party = Politische Partei
pv-membership-kind-professional-body = Berufsverband
pv-membership-kind-religious-order = Orden
pv-membership-kind-religious-association = Religiöse Vereinigung
pv-membership-kind-fraternal-order = Bruderschaft
pv-membership-kind-veterans-association = Veteranenverband
pv-membership-kind-sports-club = Sportverein
pv-membership-kind-cultural-association = Kulturverein
pv-membership-kind-charitable-association = Wohltätigkeitsverein
pv-membership-kind-other = Andere
pv-personality-instrument-neo-pi-3 = NEO-PI-3
pv-personality-instrument-neo-ffi-3 = NEO-FFI-3
pv-personality-instrument-bfi-2 = BFI-2
pv-personality-instrument-ipip-neo-120 = IPIP-NEO-120
pv-personality-instrument-tipi = TIPI
pv-personality-instrument-hexaco-pi-r = HEXACO-PI-R
pv-personality-instrument-observer-rating = Fremdeinschätzung durch Bekannte
pv-personality-instrument-inferred = Aus Unterlagen erschlossen
pv-personality-instrument-other = Anderes
pv-introversion-extraversion-strongly-introverted = Stark introvertiert
pv-introversion-extraversion-introverted = Introvertiert
pv-introversion-extraversion-ambiverted = Ambivertiert
pv-introversion-extraversion-extraverted = Extravertiert
pv-introversion-extraversion-strongly-extraverted = Stark extravertiert
pv-stress-tolerance-very-low = Sehr niedrig
pv-stress-tolerance-low = Niedrig
pv-stress-tolerance-moderate = Mittel
pv-stress-tolerance-high = Hoch
pv-stress-tolerance-very-high = Sehr hoch
pv-decision-style-rational = Rational
pv-decision-style-intuitive = Intuitiv
pv-decision-style-dependent = Abhängig
pv-decision-style-avoidant = Vermeidend
pv-decision-style-spontaneous = Spontan
pv-sport-level-recreational = Freizeit
pv-sport-level-amateur-competitive = Amateurwettkampf
pv-sport-level-semi-professional = Halbprofessionell
pv-sport-level-professional = Professionell
pv-diet-omnivore = Mischkost
pv-diet-flexitarian = Flexitarisch
pv-diet-pescatarian = Pescetarisch
pv-diet-vegetarian = Vegetarisch
pv-diet-vegan = Vegan
pv-diet-other = Andere
pv-substance-tobacco = Tabak und Nikotin
pv-substance-alcohol = Alkohol
pv-substance-cannabis = Cannabis
pv-substance-opioids = Opioide
pv-substance-stimulants = Stimulanzien
pv-substance-sedatives-hypnotics = Beruhigungs- und Schlafmittel
pv-substance-hallucinogens = Halluzinogene
pv-substance-inhalants = Schnüffelstoffe
pv-substance-gambling = Glücksspiel
pv-substance-gaming = Computerspiele
pv-substance-other = Anderes
pv-use-pattern-occasional-use = Gelegentlicher Konsum
pv-use-pattern-regular-use = Regelmäßiger Konsum
pv-use-pattern-harmful-use = Schädlicher Gebrauch
pv-use-pattern-dependence = Abhängigkeit
pv-use-pattern-in-remission = Remission
pv-lineage-biological = Leiblich
pv-lineage-adoptive = Adoptiv
pv-lineage-foster = Pflege
pv-lineage-step = Stief
pv-lineage-guardianship = Vormundschaft
pv-lineage-unknown = Unbekannt
pv-link-relation-godparent = Pate
pv-link-relation-godchild = Patenkind
pv-link-relation-witness = Zeuge
pv-link-relation-officiant = Amtsperson
pv-link-relation-business-partner = Geschäftspartner
pv-link-relation-employer = Arbeitgeber
pv-link-relation-employee = Angestellter
pv-link-relation-mentor = Mentor
pv-link-relation-apprentice = Lehrling
pv-link-relation-close-friend = Enger Freund
pv-link-relation-neighbour = Nachbar
pv-link-relation-guardian = Vormund
pv-link-relation-ward = Mündel
pv-link-relation-other = Andere
pv-country-AD = Andorra
pv-country-AE = Vereinigte Arabische Emirate
pv-country-AF = Afghanistan
pv-country-AG = Antigua und Barbuda
pv-country-AI = Anguilla
pv-country-AL = Albanien
pv-country-AM = Armenien
pv-country-AO = Angola
pv-country-AQ = Antarktis
pv-country-AR = Argentinien
pv-country-AS = Amerikanisch-Samoa
pv-country-AT = Österreich
pv-country-AU = Australien
pv-country-AW = Aruba
pv-country-AX = Ålandinseln
pv-country-AZ = Aserbaidschan
pv-country-BA = Bosnien und Herzegowina
pv-country-BB = Barbados
pv-country-BD = Bangladesch
pv-country-BE = Belgien
pv-country-BF = Burkina Faso
pv-country-BG = Bulgarien
pv-country-BH = Bahrain
pv-country-BI = Burundi
pv-country-BJ = Benin
pv-country-BL = St. Barthélemy
pv-country-BM = Bermuda
pv-country-BN = Brunei Darussalam
pv-country-BO = Bolivien
pv-country-BQ = Karibische Niederlande
pv-country-BR = Brasilien
pv-country-BS = Bahamas
pv-country-BT = Bhutan
pv-country-BV = Bouvetinsel
pv-country-BW = Botsuana
pv-country-BY = Belarus
pv-country-BZ = Belize
pv-country-CA = Kanada
pv-country-CC = Kokosinseln
pv-country-CD = Kongo-Kinshasa
pv-country-CF = Zentralafrikanische Republik
pv-country-CG = Kongo-Brazzaville
pv-country-CH = Schweiz
pv-country-CI = Côte d’Ivoire
pv-country-CK = Cookinseln
pv-country-CL = Chile
pv-country-CM = Kamerun
pv-country-CN = China
pv-country-CO = Kolumbien
pv-country-CR = Costa Rica
pv-country-CU = Kuba
pv-country-CV = Cabo Verde
pv-country-CW = Curaçao
pv-country-CX = Weihnachtsinsel
pv-country-CY = Zypern
pv-country-CZ = Tschechien
pv-country-DE = Deutschland
pv-country-DJ = Dschibuti
pv-country-DK = Dänemark
pv-country-DM = Dominica
pv-country-DO = Dominikanische Republik
pv-country-DZ = Algerien
pv-country-EC = Ecuador
pv-country-EE = Estland
pv-country-EG = Ägypten
pv-country-EH = Westsahara
pv-country-ER = Eritrea
pv-country-ES = Spanien
pv-country-ET = Äthiopien
pv-country-FI = Finnland
pv-country-FJ = Fidschi
pv-country-FK = Falklandinseln
pv-country-FM = Mikronesien
pv-country-FO = Färöer
pv-country-FR = Frankreich
pv-country-GA = Gabun
pv-country-GB = Vereinigtes Königreich
pv-country-GD = Grenada
pv-country-GE = Georgien
pv-country-GF = Französisch-Guayana
pv-country-GG = Guernsey
pv-country-GH = Ghana
pv-country-GI = Gibraltar
pv-country-GL = Grönland
pv-country-GM = Gambia
pv-country-GN = Guinea
pv-country-GP = Guadeloupe
pv-country-GQ = Äquatorialguinea
pv-country-GR = Griechenland
pv-country-GS = Südgeorgien und die Südlichen Sandwichinseln
pv-country-GT = Guatemala
pv-country-GU = Guam
pv-country-GW = Guinea-Bissau
pv-country-GY = Guyana
pv-country-HK = Sonderverwaltungsregion Hongkong
pv-country-HM = Heard und McDonaldinseln
pv-country-HN = Honduras
pv-country-HR = Kroatien
pv-country-HT = Haiti
pv-country-HU = Ungarn
pv-country-ID = Indonesien
pv-country-IE = Irland
pv-country-IL = Israel
pv-country-IM = Isle of Man
pv-country-IN = Indien
pv-country-IO = Britisches Territorium im Indischen Ozean
pv-country-IQ = Irak
pv-country-IR = Iran
pv-country-IS = Island
pv-country-IT = Italien
pv-country-JE = Jersey
pv-country-JM = Jamaika
pv-country-JO = Jordanien
pv-country-JP = Japan
pv-country-KE = Kenia
pv-country-KG = Kirgisistan
pv-country-KH = Kambodscha
pv-country-KI = Kiribati
pv-country-KM = Komoren
pv-country-KN = St. Kitts und Nevis
pv-country-KP = Nordkorea
pv-country-KR = Südkorea
pv-country-KW = Kuwait
pv-country-KY = Kaimaninseln
pv-country-KZ = Kasachstan
pv-country-LA = Laos
pv-country-LB = Libanon
pv-country-LC = St. Lucia
pv-country-LI = Liechtenstein
pv-country-LK = Sri Lanka
pv-country-LR = Liberia
pv-country-LS = Lesotho
pv-country-LT = Litauen
pv-country-LU = Luxemburg
pv-country-LV = Lettland
pv-country-LY = Libyen
pv-country-MA = Marokko
pv-country-MC = Monaco
pv-country-MD = Republik Moldau
pv-country-ME = Montenegro
pv-country-MF = St. Martin
pv-country-MG = Madagaskar
pv-country-MH = Marshallinseln
pv-country-MK = Nordmazedonien
pv-country-ML = Mali
pv-country-MM = Myanmar
pv-country-MN = Mongolei
pv-country-MO = Sonderverwaltungsregion Macau
pv-country-MP = Nördliche Marianen
pv-country-MQ = Martinique
pv-country-MR = Mauretanien
pv-country-MS = Montserrat
pv-country-MT = Malta
pv-country-MU = Mauritius
pv-country-MV = Malediven
pv-country-MW = Malawi
pv-country-MX = Mexiko
pv-country-MY = Malaysia
pv-country-MZ = Mosambik
pv-country-NA = Namibia
pv-country-NC = Neukaledonien
pv-country-NE = Niger
pv-country-NF = Norfolkinsel
pv-country-NG = Nigeria
pv-country-NI = Nicaragua
pv-country-NL = Niederlande
pv-country-NO = Norwegen
pv-country-NP = Nepal
pv-country-NR = Nauru
pv-country-NU = Niue
pv-country-NZ = Neuseeland
pv-country-OM = Oman
pv-country-PA = Panama
pv-country-PE = Peru
pv-country-PF = Französisch-Polynesien
pv-country-PG = Papua-Neuguinea
pv-country-PH = Philippinen
pv-country-PK = Pakistan
pv-country-PL = Polen
pv-country-PM = St. Pierre und Miquelon
pv-country-PN = Pitcairninseln
pv-country-PR = Puerto Rico
pv-country-PS = Palästinensische Autonomiegebiete
pv-country-PT = Portugal
pv-country-PW = Palau
pv-country-PY = Paraguay
pv-country-QA = Katar
pv-country-RE = Réunion
pv-country-RO = Rumänien
pv-country-RS = Serbien
pv-country-RU = Russland
pv-country-RW = Ruanda
pv-country-SA = Saudi-Arabien
pv-country-SB = Salomonen
pv-country-SC = Seychellen
pv-country-SD = Sudan
pv-country-SE = Schweden
pv-country-SG = Singapur
pv-country-SH = St. Helena
pv-country-SI = Slowenien
pv-country-SJ = Spitzbergen und Jan Mayen
pv-country-SK = Slowakei
pv-country-SL = Sierra Leone
pv-country-SM = San Marino
pv-country-SN = Senegal
pv-country-SO = Somalia
pv-country-SR = Suriname
pv-country-SS = Südsudan
pv-country-ST = São Tomé und Príncipe
pv-country-SV = El Salvador
pv-country-SX = Sint Maarten
pv-country-SY = Syrien
pv-country-SZ = Eswatini
pv-country-TC = Turks- und Caicosinseln
pv-country-TD = Tschad
pv-country-TF = Französische Süd- und Antarktisgebiete
pv-country-TG = Togo
pv-country-TH = Thailand
pv-country-TJ = Tadschikistan
pv-country-TK = Tokelau
pv-country-TL = Timor-Leste
pv-country-TM = Turkmenistan
pv-country-TN = Tunesien
pv-country-TO = Tonga
pv-country-TR = Türkei
pv-country-TT = Trinidad und Tobago
pv-country-TV = Tuvalu
pv-country-TW = Taiwan
pv-country-TZ = Tansania
pv-country-UA = Ukraine
pv-country-UG = Uganda
pv-country-UM = Amerikanische Überseeinseln
pv-country-US = Vereinigte Staaten
pv-country-UY = Uruguay
pv-country-UZ = Usbekistan
pv-country-VA = Vatikanstadt
pv-country-VC = St. Vincent und die Grenadinen
pv-country-VE = Venezuela
pv-country-VG = Britische Jungferninseln
pv-country-VI = Amerikanische Jungferninseln
pv-country-VN = Vietnam
pv-country-VU = Vanuatu
pv-country-WF = Wallis und Futuna
pv-country-WS = Samoa
pv-country-YE = Jemen
pv-country-YT = Mayotte
pv-country-ZA = Südafrika
pv-country-ZM = Sambia
pv-country-ZW = Simbabwe
pv-country-SU = Sowjetunion
pv-country-DD = Deutsche Demokratische Republik
pv-country-YU = Jugoslawien
pv-country-CS = Tschechoslowakei
pv-country-OT = Osmanisches Reich
lang-aa = Afar
lang-ab = Abchasisch
lang-ae = Avestisch
lang-af = Afrikaans
lang-ak = Akan
lang-am = Amharisch
lang-an = Aragonesisch
lang-ar = Arabisch
lang-as = Assamesisch
lang-av = Awarisch
lang-ay = Aymara
lang-az = Aserbaidschanisch
lang-ba = Baschkirisch
lang-be = Belarussisch
lang-bg = Bulgarisch
lang-bi = Bislama
lang-bm = Bambara
lang-bn = Bengalisch
lang-bo = Tibetisch
lang-br = Bretonisch
lang-bs = Bosnisch
lang-ca = Katalanisch
lang-ce = Tschetschenisch
lang-ch = Chamorro
lang-co = Korsisch
lang-cr = Cree
lang-cs = Tschechisch
lang-cu = Kirchenslawisch
lang-cv = Tschuwaschisch
lang-cy = Walisisch
lang-da = Dänisch
lang-de = Deutsch
lang-dv = Dhivehi
lang-dz = Dzongkha
lang-ee = Ewe
lang-el = Griechisch
lang-en = Englisch
lang-eo = Esperanto
lang-es = Spanisch
lang-et = Estnisch
lang-eu = Baskisch
lang-fa = Persisch
lang-ff = Ful
lang-fi = Finnisch
lang-fj = Fidschi
lang-fo = Färöisch
lang-fr = Französisch
lang-fy = Westfriesisch
lang-ga = Irisch
lang-gd = Gälisch (Schottland)
lang-gl = Galicisch
lang-gn = Guaraní
lang-gu = Gujarati
lang-gv = Manx
lang-ha = Haussa
lang-he = Hebräisch
lang-hi = Hindi
lang-ho = Hiri-Motu
lang-hr = Kroatisch
lang-ht = Haiti-Kreolisch
lang-hu = Ungarisch
lang-hy = Armenisch
lang-hz = Herero
lang-ia = Interlingua
lang-id = Indonesisch
lang-ie = Interlingue
lang-ig = Igbo
lang-ii = Yi
lang-ik = Inupiak
lang-io = Ido
lang-is = Isländisch
lang-it = Italienisch
lang-iu = Inuktitut
lang-ja = Japanisch
lang-jv = Javanisch
lang-ka = Georgisch
lang-kg = Kongolesisch
lang-ki = Kikuyu
lang-kj = Kwanyama
lang-kk = Kasachisch
lang-kl = Grönländisch
lang-km = Khmer
lang-kn = Kannada
lang-ko = Koreanisch
lang-kr = Kanuri
lang-ks = Kaschmiri
lang-ku = Kurdisch
lang-kv = Komi
lang-kw = Kornisch
lang-ky = Kirgisisch
lang-la = Latein
lang-lb = Luxemburgisch
lang-lg = Ganda
lang-li = Limburgisch
lang-ln = Lingala
lang-lo = Laotisch
lang-lt = Litauisch
lang-lu = Luba-Katanga
lang-lv = Lettisch
lang-mg = Malagasy
lang-mh = Marschallesisch
lang-mi = Māori
lang-mk = Mazedonisch
lang-ml = Malayalam
lang-mn = Mongolisch
lang-mr = Marathi
lang-ms = Malaiisch
lang-mt = Maltesisch
lang-my = Birmanisch
lang-na = Nauruisch
lang-nb = Norwegisch (Bokmål)
lang-nd = Nord-Ndebele
lang-ne = Nepalesisch
lang-ng = Ndonga
lang-nl = Niederländisch
lang-nn = Norwegisch (Nynorsk)
lang-no = Norwegisch
lang-nr = Süd-Ndebele
lang-nv = Navajo
lang-ny = Nyanja
lang-oc = Okzitanisch
lang-oj = Ojibwa
lang-om = Oromo
lang-or = Oriya
lang-os = Ossetisch
lang-pa = Punjabi
lang-pi = Pali
lang-pl = Polnisch
lang-ps = Paschtu
lang-pt = Portugiesisch
lang-qu = Quechua
lang-rm = Rätoromanisch
lang-rn = Rundi
lang-ro = Rumänisch
lang-ru = Russisch
lang-rw = Kinyarwanda
lang-sa = Sanskrit
lang-sc = Sardisch
lang-sd = Sindhi
lang-se = Nordsamisch
lang-sg = Sango
lang-sh = Serbo-Kroatisch
lang-si = Singhalesisch
lang-sk = Slowakisch
lang-sl = Slowenisch
lang-sm = Samoanisch
lang-sn = Shona
lang-so = Somali
lang-sq = Albanisch
lang-sr = Serbisch
lang-ss = Swazi
lang-st = Süd-Sotho
lang-su = Sundanesisch
lang-sv = Schwedisch
lang-sw = Suaheli
lang-ta = Tamil
lang-te = Telugu
lang-tg = Tadschikisch
lang-th = Thailändisch
lang-ti = Tigrinya
lang-tk = Turkmenisch
lang-tl = Tagalog
lang-tn = Tswana
lang-to = Tongaisch
lang-tr = Türkisch
lang-ts = Tsonga
lang-tt = Tatarisch
lang-tw = Twi
lang-ty = Tahitisch
lang-ug = Uigurisch
lang-uk = Ukrainisch
lang-ur = Urdu
lang-uz = Usbekisch
lang-ve = Venda
lang-vi = Vietnamesisch
lang-vo = Volapük
lang-wa = Wallonisch
lang-wo = Wolof
lang-xh = Xhosa
lang-yi = Jiddisch
lang-yo = Yoruba
lang-za = Zhuang
lang-zh = Chinesisch
lang-zu = Zulu
person-tab-profile = Profil
profile-groups-label = Bereiche des Profils
profile-group-withheld = Ein Teil dieses Bereichs ist für Sie nicht sichtbar
profile-withheld = Für diese Person erfasst und für Sie nicht sichtbar: { $classes }.
profile-empty = In diesem Bereich ist noch nichts erfasst.
profile-earlier = früheres Formular
profile-earlier-title = Von einer früheren Version dieser Anwendung in einem Feld erfasst, für das AXGF 1.1 keinen Platz hat. Es bleibt so erhalten, wie es geschrieben wurde.
profile-other-names = { $n ->
        [one] und ein weiterer Name
       *[other] und { $n } weitere Namen
    }
profile-edit-group = „{ $group }“ bearbeiten
profile-summary-link = { $n ->
        [one] Eine Angabe im Profil
       *[other] { $n } Angaben im Profil
    }
profile-from = ab
profile-until = bis
profile-yes = Ja
profile-no = Nein
profile-value = Wert
profile-editor-title = Profil
profile-group-filled = { $n } ausgefüllt
profile-problems = Ein Teil der Eingaben konnte nicht gespeichert werden. Jedes Problem ist neben seinem Feld genannt, und es wurde nichts geschrieben.
profile-editor-withheld = Dieser Bereich enthält für diese Person auch Angaben der Kategorie { $classes }, die Sie nicht lesen dürfen. Sie werden hier nicht angezeigt, und das Speichern dieses Formulars lässt sie unverändert.
profile-living-class-note = Diese Person ist als lebend erfasst. Was Sie hier in einer sensiblen Kategorie eintragen, sehen nur Administratoren.
profile-relationships-elsewhere = Eltern, Partner, Kinder, Paten und Zeugen sind nicht bei dieser Person gespeichert. Es sind Familien, Verknüpfungen und Ereignisse, in denen sie vorkommt — jede Änderung hier ändert daher auch den Eintrag aller anderen Beteiligten.
profile-documents-first = Ein Artefakt verweist auf ein Dokument, das dieser Person zugeordnet ist. Fügen Sie zuerst die Datei hinzu.
profile-editor-nothing = In diesem Bereich gibt es nichts, was Sie bearbeiten dürfen.
profile-new-entry = Neuer Eintrag
profile-provenance = Datum, Quelle und Verlässlichkeit
profile-from-date = Gültig ab
profile-until-date = Gültig bis
profile-remove-entry = Diesen Eintrag entfernen
profile-add-entry = Weiteren Eintrag hinzufügen
profile-no-such-group-title = Bereich nicht gefunden
profile-no-such-group-detail = Das Profil hat keinen Bereich mit diesem Namen.
profile-error-number = Ein Wert in diesem Feld muss eine Zahl sein.
profile-error-integer = Ein Wert in diesem Feld muss eine ganze Zahl sein.
profile-error-range = Eine Zahl liegt außerhalb des für dieses Merkmal zulässigen Bereichs.
profile-error-term = Ein Wert gehört nicht zu den angebotenen Auswahlmöglichkeiten.
profile-error-required = Einem Eintrag fehlt ein notwendiges Feld.
profile-error-one-of = Ein Eintrag braucht mindestens eines seiner Hauptfelder.
profile-error-confidence = Die Verlässlichkeit reicht von 0 bis 1, zum Beispiel 0,8.
profile-error-time = Eine Uhrzeit wird in Stunden und Minuten geschrieben, zum Beispiel 05:40.
profile-error-currency = Eine Währung wird mit ihrem dreistelligen Code geschrieben, zum Beispiel EUR.
profile-error-language = Eine Sprache wird mit ihrem Code geschrieben, zum Beispiel de oder zh-Hans.
profile-error-coordinates = Koordinaten brauchen einen Breitengrad zwischen −90 und 90 und einen Längengrad zwischen −180 und 180.
profile-error-rank-country = Der Dienstgrad gehört zu einem anderen Land als dem gewählten.
record-unknown-place = [Unbekannter Ort]
record-missing-document = [Fehlendes Dokument]

## Interface

confidence-certain = Sicherheit { $percent } % — so gut wie sicher
confidence-high = Sicherheit { $percent } % — gut belegt
confidence-medium = Sicherheit { $percent } % — plausibel, aber unbestätigt
confidence-low = Sicherheit { $percent } % — Vermutung
tree-edge-union-between = { $from } und { $to } — { $confidence }
tree-edge-parentage-of = { $from }, Elternteil von { $to } — { $confidence }
record-note-biography = Lebenslauf
record-note-birth-date-as-recorded = Geburtsdatum, wie es erfasst wurde
record-note-death-date-as-recorded = Sterbedatum, wie es erfasst wurde
record-note-event-date-as-recorded = Datum von „{ $event }“, wie es erfasst wurde
record-unknown-source = [Unbekannte Quelle]
record-untitled-source = [Quelle ohne Titel]
record-unnamed = [Ohne Namen]
record-untitled = [Ohne Bezeichnung]
record-period-from = ab { $date }
record-period-until = bis { $date }
record-dates-unrecorded = Daten nicht erfasst
record-link-unlabelled = verbunden mit
record-link-reverse = { $label } (von)
record-place-worked-as = Tätig als { $title }
record-place-married-to = Heirat mit { $name }
record-place-married = Heirat
record-source-use-name = der Name „{ $name }“
record-source-use-working-as = die Tätigkeit als { $title }
record-source-use-union-with = die Verbindung mit { $name }
record-source-use-union = die Verbindung
record-lifespan-born = geb. { $year }
record-lifespan-died = gest. { $year }
size-bytes = { $n ->
        [one] { $n } Byte
       *[other] { $n } Bytes
    }
size-kb = { $n } KB
size-mb = { $n } MB
size-gb = { $n } GB
calendar-gregorian = gregorianisch
calendar-julian = julianisch
calendar-hebrew = hebräisch
calendar-hijri = islamisch
calendar-persian = persisch
calendar-chinese = chinesisch
calendar-ethiopian = äthiopisch
calendar-japanese_era = japanische Ära
calendar-republican_french = französischer Revolutionskalender
calendar-roman = römisch
diff-summary-none = hat kein Feld geändert
diff-summary-one = hat { $a } geändert
diff-summary-two = hat { $a } und { $b } geändert
diff-summary-many = hat { $a }, { $b } und { $n ->
        [one] ein weiteres Feld geändert
       *[other] { $n } weitere Felder geändert
    }
diff-saved-none = kein Feld geändert
diff-saved-one = { $a } geändert
diff-saved-two = { $a } und { $b } geändert
diff-saved-many = { $a }, { $b } und { $n ->
        [one] ein weiteres Feld geändert
       *[other] { $n } weitere Felder geändert
    }
diff-section-other = ein weiteres Feld
diff-section-avatar = Porträtfoto
diff-section-partners = Partner
diff-section-ai = KI-Unterstützung
diff-section-birth = Geburt
diff-section-civil-status = Personenstand
diff-section-documents = Angehängte Dokumente
diff-section-extensions = Anwendungsdaten
diff-section-children = Kinder
diff-section-union = Verbindung
diff-section-date = Datum
diff-section-participants = Beteiligte
diff-section-from = Verknüpft von
diff-section-to = Verknüpft mit
diff-section-valid-from = Gültig ab
diff-section-valid-until = Gültig bis
diff-section-visibility = Sichtbarkeit
diff-section-employer = Arbeitgeber
diff-section-title-normalized = Normierter Titel
diff-section-conflicts = Widersprüchliche Quellen
diff-section-dna = DNA
diff-section-document = Dokument
diff-section-language = Sprache
diff-section-place = Ort
diff-section-repository = Archiv
diff-section-script = Schrift
diff-section-coordinates = Koordinaten
diff-section-country-history = Staatszugehörigkeit im Lauf der Zeit
diff-section-identifiers = Kennungen
diff-section-names = Namen
diff-section-file = Datei
diff-section-linked-to = Zugeordnet zu
diff-section-ocr = Erkannter Text
diff-section-format-version = Formatversion
history-created = hat angelegt
history-deleted = hat gelöscht
history-merged = hat mit einem anderen Eintrag zusammengeführt
history-setup = hat das Archiv eingerichtet
history-attached = hat eine Datei angehängt
admin-raw-json-unparsed = Das Roh-JSON ließ sich nicht lesen ({ $error }). Nichts wurde gespeichert.
admin-raw-json-dangling = Das Roh-JSON verweist mit { $field } auf { $target }, das es in diesem Archiv nicht gibt. Wählen Sie es über ein Feld oben aus oder korrigieren Sie die Kennung. Nichts wurde gespeichert.
conflict-someone = Jemand
conflict-unrecorded-time = (Zeitpunkt nicht erfasst)
dedup-merged-persons = { $n ->
        [one] eine Person zusammengeführt
       *[other] { $n } Personen zusammengeführt
    }
dedup-merged-families = { $n ->
        [one] eine Familie zusammengeführt
       *[other] { $n } Familien zusammengeführt
    }
dedup-manual-review = { $n ->
        [one] ein Fall zur Prüfung durch einen Menschen offen
       *[other] { $n } Fälle zur Prüfung durch einen Menschen offen
    }
dedup-nothing = Nichts zu melden.
record-union-duplicate = Ein Paar, mehrere Einträge.
record-union-duplicate-detail = Das Bündel führt für diese beiden Personen getrennte Familieneinträge. Das ist ein Datenfehler, keine zweite Verbindung.
record-union-duplicate-action = Duplikate zusammenführen
record-union-duplicate-confirm = Das gesamte Bündel entdoppeln? Jedes Paar, das die Bibliothek zusammenführen kann, wird zusammengeführt; der Rest wird gemeldet.
dedup-pair-merged = Das betreffende Paar ist jetzt ein Eintrag.
dedup-pair-refused = Das betreffende Paar wurde nicht zusammengeführt: Die Bibliothek hat abgelehnt und es einer Person zur Prüfung überlassen.

validate-errors = { $n ->
        [one] ein Fehler
       *[other] { $n } Fehler
    }
validate-warnings = { $n ->
        [one] eine Warnung
       *[other] { $n } Warnungen
    }
validate-notes = { $n ->
        [one] ein Hinweis
       *[other] { $n } Hinweise
    }
validate-nothing = Nichts zu melden.
list-separator = { ", " }
result-written = Das Archiv wurde auf die Festplatte geschrieben.
result-refused = Die Bibliothek hat diesen Vorgang abgelehnt. Das Archiv auf der Festplatte ist unverändert.
convert-error-no-file = Es wurde keine Datei hochgeladen. Wählen Sie zuerst eine .ged-Datei.
convert-error-file-too-large = Diese Datei ist { $size } MB groß, die Grenze liegt bei { $limit } MB. Nichts wurde umgewandelt.
convert-error-too-large = Der Upload überschreitet die Grenze von { $limit } MB. Nichts wurde umgewandelt.
convert-error-unreadable = Der Upload ließ sich nicht lesen ({ $error }). Nichts wurde umgewandelt.
convert-error-not-gedcom = Das sieht nicht nach einer GEDCOM-Datei aus: Eine GEDCOM-5.5.1-Datei beginnt mit der Zeile „0 HEAD“. Nichts wurde umgewandelt.
gedcom-read-title = Wie die Datei gelesen wurde
gedcom-read-as = Gelesen als GEDCOM { $version }, in { $encoding }.
gedcom-read-as-unversioned = Gelesen als GEDCOM (der Kopf nennt keine Version), in { $encoding }.
gedcom-read-declared = Der Kopf nennt { $declared }; die Bytes sagen etwas anderes und wurden so gelesen, wie sie sind.
gedcom-read-gedzip = Ein GEDCOM-7-Archiv (.gdz): sein Datensatz wurde gelesen. Weitere Dateien darin (Fotos, Scans), nicht übernommen: { $n }.
gedcom-read-crs = Zeilen mit altem Macintosh-Zeilenende, repariert: { $n }.
gedcom-read-voids = Verweise auf @VOID@ – GEDCOM 7s „kein Datensatz“ – beiseitegelegt statt als Personen gelesen: { $n }.
gedcom-read-notes = Gemeinsame Notizen (SNOTE) als Notizen gelesen: { $n }.
gedcom-read-stray = Bytes, die in dieser UTF-8-Datei kein UTF-8 waren, einzeln als Windows-1252 gelesen: { $n }.
gedcom-read-tidied = Zeilen in einer Form, die der Konverter nicht liest (eingerückt, Tabulatoren oder zusätzliche Leerzeichen zwischen Feldern, Tag in Kleinbuchstaben, Leerzeichen vor einer CONC-Fortsetzung, Steuerzeichen, gregorianische Kalendermarke), repariert: { $n }.
gedcom-read-too-deep = Zeilen, tiefer verschachtelt als jede GEDCOM-Struktur, mit ihrem Inhalt beiseitegelegt: { $n }.
convert-error-no-dataset = Dieses ZIP enthält weder gedcom.ged noch eine einzelne .ged-Datei, die stattdessen gelesen werden könnte.
convert-error-refused = Der Konverter hat diese Datei abgelehnt. Die Hinweise unten sagen, warum. Nichts wurde behalten.
convert-error-packaging = Die Datei wurde umgewandelt, ließ sich aber nicht verpacken ({ $error }).
completeness-fraction = { $part } von { $whole }
event-category-adoption = Adoption
event-category-migration = Auswanderung
event-category-naturalization = Einbürgerung
event-category-incarceration = Haft
event-category-name_change = Namensänderung
event-category-legal = Rechtssache
event-category-religious = Kirchliches Ereignis
event-category-social = Gesellschaftliches Ereignis
event-category-historical = Historisches Ereignis
precision-quarter_century = auf das Vierteljahrhundert
source-type-birth_certificate = Geburtsurkunde
source-type-death_certificate = Sterbeurkunde
source-type-marriage_certificate = Heiratsurkunde
source-type-census = Volkszählung
source-type-baptism_record = Taufeintrag
source-type-burial_record = Begräbniseintrag
source-type-will = Testament
source-type-land_record = Grundbucheintrag
source-type-military_record = Militärakte
source-type-immigration_record = Einwanderungsakte
source-type-naturalization = Einbürgerungsakte
source-type-passport = Reisepass
source-type-photograph = Fotografie
source-type-letter = Brief
source-type-diary = Tagebuch
source-type-newspaper = Zeitung
source-type-oral_tradition = mündliche Überlieferung
source-type-dna = DNA-Test
source-type-family_bible = Familienbibel
source-type-gravestone = Grabstein
source-type-published_genealogy = veröffentlichte Genealogie
source-type-other = andere Quelle
source-status-verified = am Original geprüft
source-status-unverified = noch nicht geprüft
source-status-lost = verloren
source-status-known_missing = bekanntermaßen fehlend
document-type-birth_certificate = Geburtsurkunde
document-type-death_certificate = Sterbeurkunde
document-type-marriage_certificate = Heiratsurkunde
document-type-census_page = Seite einer Volkszählung
document-type-baptism_record = Taufeintrag
document-type-military_record = Militärakte
document-type-will = Testament
document-type-land_record = Grundbucheintrag
document-type-diary = Tagebuch
document-type-newspaper_clipping = Zeitungsausschnitt
document-type-gravestone_photo = Foto eines Grabsteins
document-type-family_tree_drawing = gezeichneter Stammbaum
document-type-audio = Tonaufnahme
document-type-video = Videoaufnahme
document-status-present = hier vorhanden
document-status-referenced = genannt, anderswo aufbewahrt
document-status-known_missing = bekanntermaßen fehlend
document-status-lost = verloren
document-status-unknown = Verbleib unbekannt
diag-unsupported_spec_version = Das Archiv gibt eine AXGF-Version an, die dieses Programm nicht lesen kann.
diag-invalid_json = Etwas, das JSON sein sollte, lässt sich nicht lesen.
diag-invalid_bundle_structure = Das Archiv ist nicht so aufgebaut, wie AXGF es verlangt.
diag-schema_validation_failed = Ein Datensatz entspricht nicht dem AXGF-Schema.
diag-dangling_reference = Ein Datensatz verweist auf einen anderen, der nicht im Archiv ist.
diag-duplicate_entity_id = Zwei Datensätze haben dieselbe Kennung.
diag-duplicate_unique_ref = Zwei Datensätze beanspruchen denselben Verweis, der eindeutig sein sollte.
diag-cycle_detected = Die Familienverbindungen laufen im Kreis: Jemand wäre sein eigener Vorfahr.
diag-chronology_conflict = Daten widersprechen sich, etwa ein Kind, das vor einem Elternteil geboren ist.
diag-out_of_vocabulary = Ein Wert gehört nicht zu den Begriffen, die seine Liste erlaubt.
diag-claim_inconsistent = Eine Angabe widerspricht sich selbst oder einer anderen Angabe zur selben Sache.
diag-spec_version_mismatch = Die angegebene AXGF-Version eines Datensatzes passt nicht zu seinem Inhalt.
diag-unknown_attribute = Ein Datensatz enthält ein Attribut, das AXGF nicht definiert.
diag-entity_not_found = Der zu ändernde Datensatz ist nicht im Archiv.
diag-entity_already_exists = Ein Datensatz mit dieser Kennung existiert bereits.
diag-unknown_entity_kind = Diese Art von Datensatz gibt es in AXGF nicht.
diag-delete_blocked_by_reference = Der Datensatz kann nicht gelöscht werden, solange andere auf ihn verweisen.
diag-manual_review_required = Das muss sich ein Mensch ansehen; es wurde nicht automatisch geändert.
diag-zip_read_error = Die Archivdatei ließ sich nicht lesen.
diag-zip_write_error = Die Archivdatei ließ sich nicht schreiben.
diag-payloads_external = Die angehängten Dateien liegen außerhalb der Archivdaten.
diag-payload_source_failed = Eine angehängte Datei ließ sich nicht lesen.
diag-payload_sink_failed = Eine angehängte Datei ließ sich nicht schreiben.
diag-gedcom_parse_error = Eine Zeile der GEDCOM-Datei ließ sich nicht verstehen.
diag-gedcom_unrecognized_tag = Die GEDCOM-Datei verwendet ein Kennzeichen, das der Import nicht kennt; dieser Eintrag wurde nicht übernommen.
diag-internal = In der Bibliothek ist etwas schiefgegangen.
field-person-display-name = Angezeigter Name
field-person-display-name-hint = Der Name, der überall auf der Website erscheint.
field-person-gender = Geschlecht
field-person-living = Lebt
field-person-birth-date = Geburtsdatum
field-date-value-hint = Ein Jahr, Jahr und Monat oder ein vollständiges Datum: 1923, 1923-04 oder 1923-04-12. Leer lassen, wenn es niemand weiß.
field-person-birth-precision = Genauigkeit der Geburt
field-precision-hint = Wie genau die Quelle das festlegt.
field-person-birth-circa = Geburt ungefähr
field-circa-hint = Wird als „um 1923“ angezeigt statt als genaue Angabe.
field-person-birth-place = Kennung des Geburtsorts
field-person-birth-confidence = Sicherheit der Geburt
field-person-confidence-hint = Wie sicher Sie sind. Genau das zeichnet die Website.
field-person-death-date = Sterbedatum
field-person-death-precision = Genauigkeit des Todes
field-person-death-circa = Tod ungefähr
field-person-death-place = Kennung des Sterbeorts
field-person-death-confidence = Sicherheit des Todes
field-person-death-cause = Todesursache
field-person-bio = Lebenslauf
field-notes = Notizen
field-family-name = Name der Familie
field-description = Beschreibung
field-family-union-type = Art der Verbindung
field-family-union-status = Stand der Verbindung
field-family-union-confidence = Sicherheit der Verbindung
field-family-union-confidence-hint = Bestimmt, wie kräftig die Linie zwischen den Partnern im Stammbaum gezeichnet wird.
field-family-union-start = Beginn der Verbindung
field-family-union-end = Ende der Verbindung
field-family-notes-hint = Partner und Kinder sind Listen — bearbeiten Sie sie unten im Roh-JSON oder auf der Beziehungsseite der Person.
field-category = Kategorie
field-required-hint = Pflichtfeld.
field-event-subcategory = Unterkategorie
field-date = Datum
field-event-date-hint = Vom Schema verlangt.
field-precision = Genauigkeit
field-circa = Ungefähr
field-place-id = Kennung des Orts
field-confidence = Sicherheit
field-source-id = Kennung der Quelle
field-link-from-type = Art am Anfang
field-link-from-id = Kennung am Anfang
field-link-to-type = Art am Ende
field-link-to-id = Kennung am Ende
field-link-label = Bezeichnung
field-link-label-hint = Liest sich in Richtung der Verbindung: „Pate“, „Arbeitgeber“, „Trauzeuge“. Pflichtfeld.
field-link-label-reverse = Bezeichnung in Gegenrichtung
field-link-label-reverse-hint = Wie es sich vom anderen Ende liest: „Patenkind“, „Angestellter“.
field-link-bidirectional = Liest sich in beide Richtungen gleich
field-valid-from = Gültig ab
field-link-valid-from-hint = Wann die Beziehung begann.
field-valid-until = Gültig bis
field-link-confidence-hint = „Zu 85 % sicher, laut einem Familienbrief“ — das, was GEDCOM nicht sagen kann.
field-note = Anmerkung
field-occupation-person-id = Kennung der Person
field-occupation-title = Tätigkeit
field-occupation-title-hint = Pflichtfeld, zum Beispiel Lehrerin.
field-occupation-title-latin = Tätigkeit (lateinische Schrift)
field-occupation-employer = Arbeitgeber
field-occupation-from = Von
field-occupation-from-hint = Eine Tätigkeit ist ein Zeitraum. Erst beide Enden machen daraus einen Balken.
field-occupation-until = Bis
field-source-title = Titel
field-source-type = Art der Quelle
field-source-reliability = Verlässlichkeit
field-source-reliability-hint = Pflichtfeld. Erscheint als Plakette neben jeder Angabe, die auf dieser Quelle beruht.
field-source-status = Zustand der Quelle
field-source-repository = Aufbewahrungsort
field-source-repository-reference = Signatur am Aufbewahrungsort
field-source-transcription = Abschrift
field-place-name = Hauptname
field-place-name-lang = Sprache des Namens
field-place-name-lang-hint = Ein Sprachkürzel, zum Beispiel en, fr oder pl.
field-place-type = Art des Orts
field-place-region = Region
field-place-country-current = Heutiger Staat
field-place-country-current-hint = Die Geschichte seiner Grenzen ist eine Liste — bearbeiten Sie sie unten im Roh-JSON.
field-document-filename = Dateiname
field-document-mime-type = Medientyp
field-document-mime-type-hint = Pflichtfeld, zum Beispiel image/jpeg.
field-document-type = Art des Dokuments
field-document-status = Zustand der Datei
field-document-url = Webadresse
field-document-caption = Bildunterschrift
lang-zh-Hans = Chinesisch (vereinfacht)
family-lineage = Abstammung
links-relation = Art der Beziehung
occupations-position = Stellung
field-link-relation = Art der Beziehung
field-link-relation-hint = Eine der Beziehungen, die AXGF 1.1 benennt. Die Bezeichnung oben bewahrt die Worte des Eintrags.
field-occupation-position = Stellung
field-occupation-position-hint = Die Stellung innerhalb der Tätigkeit: Schulleiterin, wo die Tätigkeit Lehrerin ist.
error-delete-changed-title = Seit Ihrem Blick darauf geändert
error-delete-changed-detail = Dieser Datensatz wurde erneut gespeichert, nachdem die Seite gezeichnet wurde, von der aus Sie ihn löschen; er ist jetzt Fassung { $version }. Nichts wurde gelöscht. Sehen Sie ihn sich im jetzigen Stand an, bevor Sie erneut entscheiden.
error-delete-changed-look = Erneut ansehen
documents-files = Hier angehängte Dateien
documents-files-help = Bearbeiten Sie die Angaben zu einer Datei oder löschen Sie die Datei selbst. Löschen entfernt das Dokument und seine Bytes aus dem Archiv, für alle, an die es angehängt ist; um es nur von dieser Person zu lösen, leeren Sie oben seine Zeile.
documents-edit-details = Angaben bearbeiten
documents-delete = Diese Datei löschen

## Charts

radar-section = Diagramme aus dem Datensatz
radar-section-help = Drei Lesarten dessen, was dieser Datensatz enthält, jede Achse von 0 bis 100. Jede Zahl wird aus den daneben aufgeführten Angaben berechnet, nach Regeln, die in der Dokumentation der Anwendung stehen; nichts wird gespeichert, nichts geraten, und eine Achse, aus der sich nichts lesen lässt, bleibt leer, statt einen mittleren Wert zu bekommen.
radar-physique = Körperbau
radar-mind = Temperament und Geist
radar-vitality = Gesundheit und Lebenskraft
radar-axis-stature = Körpergröße
radar-axis-build = Statur
radar-axis-lean-mass = Magermasse
radar-axis-posture = Haltung
radar-axis-gait = Gang
radar-axis-dentition = Zähne
radar-axis-openness = Offenheit
radar-axis-conscientiousness = Gewissenhaftigkeit
radar-axis-extraversion = Extraversion
radar-axis-agreeableness = Verträglichkeit
radar-axis-stability = Emotionale Stabilität
radar-axis-cognition = Kognition
radar-axis-circulation = Kreislauf
radar-axis-breathing = Atmung
radar-axis-metabolism = Stoffwechsel
radar-axis-illness = Freiheit von Krankheit
radar-axis-senses = Sinne
radar-axis-rest = Schlaf und Stimmung
radar-folded-open = Dieses Diagramm zeigen
radar-folded-why = Diese Person ist als lebend erfasst. Das Bild des Temperaments einer lebenden Person bleibt zugeklappt, bis jemand, der es lesen darf, es sehen möchte.
radar-empty = Aus diesem Datensatz lässt sich für dieses Diagramm noch nichts lesen.
radar-table-caption = { $chart }: jede Achse, ihr Wert und woraus er gelesen wurde
radar-col-axis = Achse
radar-col-score = Wert
radar-col-from = Gelesen aus
radar-no-score = kein Wert
radar-from-none = nichts
record-link-outgoing = von dieser Person
record-link-incoming = zu dieser Person
# Der Betriebshinweis im Verwaltungsbereich.
health-attention = Erfordert Aufmerksamkeit:
health-standing-token = Ein Notfall-Administratortoken ist weiterhin gesetzt. Es meldet sich an jedem Konto und jeder Berechtigung vorbei an — dafür ist es an dem Tag da, an dem sich niemand anmelden kann, und nicht dafür, danach liegen zu bleiben. Entfernen Sie es aus der Umgebungsdatei des Dienstes und starten Sie ihn neu.
health-bundle-invalid = { $errors ->
        [one] Die Familiendaten sind nicht mehr gültig: ein Fehler, unten unter „Prüfen“ aufgeführt.
       *[other] Die Familiendaten sind nicht mehr gültig: { $errors } Fehler, unten unter „Prüfen“ aufgeführt.
    }
health-disk-unknown = Der freie Speicherplatz dieses Rechners konnte nicht gelesen werden; damit lässt sich nicht zusagen, dass das nächste Speichern hineinpasst.
health-disk-no-room-to-save = Speichern ist nicht möglich: { $free } sind frei, der Neuaufbau dieses Bündels braucht { $need }. Nichts ist verloren, und nichts lässt sich ändern, bis Platz da ist.
health-disk-critical = Die Festplatte ist zu { $percent } % frei — { $free } übrig. Speichern wird in Kürze nicht mehr funktionieren.
health-disk-low = Die Festplatte ist zu { $percent } % frei — { $free } übrig. Sollte behoben werden, bevor es dringend wird.
health-backup-unconfigured = Es wird nichts gesichert. Eine ausgefallene Festplatte würde jeden Datensatz mitnehmen.
health-backup-never = Ein Sicherungsverzeichnis ist gesetzt, aber es wurde noch nie eine Sicherung darin geschrieben.
health-backup-stale = { $days ->
        [one] Die neueste Sicherung ist einen Tag alt. Täglich sollte eine geschrieben werden.
       *[other] Die neueste Sicherung ist { $days } Tage alt. Täglich sollte eine geschrieben werden.
    }
health-cache-missing = { $missing ->
        [one] Eine von { $declared } angehängten Dateien fehlt im Zwischenspeicher und kann erst heruntergeladen werden, wenn das nächste Speichern sie neu aufbaut.
       *[other] { $missing } von { $declared } angehängten Dateien fehlen im Zwischenspeicher und können erst heruntergeladen werden, wenn das nächste Speichern sie neu aufbaut.
    }

## River

nav-river = Fluss
nav-home = Überblick
river-aria = Der Fluss um { $name }: Vorfahren unten, Nachkommen oben.
river-band-living = LEBENDE
river-circa = um{" "}
river-trail = Weg
river-back = ← Zurück
river-range = Reichweite
river-motion = Bewegung
river-travel = Reise
river-cut = Schnitt
river-legend-attested = belegt
river-legend-inferred = erschlossen
river-legend-speculative = vermutet
river-legend-lost = Aufzeichnung endet
river-legend-repeat = ×n über n Linien erreicht, unter jeder gezeichnet
river-legend-again = ↔ neben einem späteren Partner erneut gezeichnet
river-legend-discharge = Nachkommen
river-legend-direction = ↓ zur Quelle · ↑ zu den Lebenden
view-tree-direction = ↓ die Wurzeln: Vorfahren · ↑ die Krone: Nachkommen
view-galaxy-direction = ↓ Vorfahren unter der Sonne · ↑ Nachkommen darüber
view-summit-direction = ↓ die Vorfahren im Tal · ↑ die Nachkommen zum Gipfel hin
river-legend-generation = eine Generation ≈ 29 Jahre
river-zoom = Zoom
river-still = Landschaft anhalten
view-tree-still = Baum anhalten
view-galaxy-still = Himmel anhalten
view-summit-still = Berg anhalten
river-whole = Die ganze Familie
river-zoom-hint = Zoom: Strg + Mausrad, zwei Finger oder + / − · vergrößert ziehen, um sich zu bewegen
river-grid-link = Rasteransicht
river-tab-sources = Quellen
river-legend-title = So liest man den Fluss
view-tree-name = Lebender Baum
view-tree-legend = Den Baum lesen
view-galaxy-name = Galaxie
view-galaxy-legend = Die Galaxie lesen
view-summit-name = Aufstieg
view-summit-legend = Den Berg lesen
hud-ancestors = Vorfahren
hud-descendants = Nachkommen
hud-generations = Generationen
hud-living = Lebende
hud-attested = Belegte Verbindungen

# Masthead search
search-label = Person suchen
search-short = Person
date-qualifier = Zusatz
date-qualifier-circa = um
search-title = Suche
search-heading = Personen zu „{ $q }“
search-none = Niemand, den Sie sehen können, heißt so.
search-open-record = Datensatz öffnen
search-hint = Enter führt den Fluss zu dieser Person; Umschalt+Enter öffnet ihren Datensatz.
view-tree-search-hint = Enter lässt den Baum um diese Person wachsen; Umschalt+Enter öffnet ihren Datensatz.
view-galaxy-search-hint = Enter macht diese Person zur Sonne; Umschalt+Enter öffnet ihren Datensatz.
view-summit-search-hint = Enter führt zu dieser Person; Umschalt+Enter öffnet ihren Datensatz.
search-summary = { $n ->
        [one] { $n } Person
       *[other] { $n } Personen
    } · ↵ hinreisen · ⇧↵ Eintrag öffnen
search-empty-query = Geben Sie einen Namen ein, um zu suchen.

# Data quality (/quality)
nav-quality = Datenqualität
quality-title = Was einen Blick wert ist
quality-none = Nichts zu sehen: Alle Prüfungen sind bestanden.
quality-intro = Hier wird nichts für Sie geändert. Jede Zeile ist ein Fall, der eine Prüfung wert ist — oft gibt es einen guten Grund dafür. Öffnen Sie den Datensatz und entscheiden Sie, oder markieren Sie ihn als geprüft.
quality-groups-label = Arten von Befunden
quality-all = Alle
quality-show-label = Anzeigen
quality-show-open-option = Zu prüfen
quality-download = Herunterladen
quality-group-names = Namen
quality-group-relations = Beziehungen
quality-group-dates = Daten
quality-group-places = Orte
quality-group-evidence = Belege
quality-export-csv = Als CSV herunterladen
quality-export-json = Als JSON herunterladen
quality-open = Öffnen
quality-none-file = In dieser Datei ist nichts einen Blick wert.
quality-show-list = Liste anzeigen
quality-after-import = In der Datei wurde nichts geändert. Sobald sie das Archiv der Website ist, listet die Seite Datenqualität dieselben Punkte mit einem Link zu jedem Datensatz.
quality-file-list-cut = Hier sind nur die ersten { $shown } aufgeführt. Sobald die Datei das Archiv der Website ist, listet die Seite Datenqualität alle auf.
quality-rule-name-sentence = Ein Satz in einem Namensfeld
quality-name-sentence = „{ $value }“ sieht eher nach einem Satz als nach einem Namen aus: Ist es eine Erklärung, lassen Sie den Namen im Feld und verschieben Sie den Rest in eine Notiz.
quality-rule-union-parent-child = Ein Paar, das auch Elternteil und Kind ist
quality-union-parent-child = { $parent } und { $child } erscheinen sowohl als Paar als auch als Elternteil und Kind: Es lohnt sich zu prüfen, welche Beziehung die Quellen belegen.
quality-rule-child-listed-twice = Ein Kind doppelt eingetragen
quality-child-listed-twice = { $child } erscheint zweimal unter den Kindern dieser Familie: Ist es dasselbe Kind, kann ein Eintrag entfallen.
quality-rule-duplicate-union = Dasselbe Paar doppelt erfasst
quality-duplicate-union = Für { $a } und { $b } sind { $count } Verbindungen erfasst: Haben sie nur einmal geheiratet, ist es vielleicht eine doppelt eingetragene Verbindung.
quality-rule-partner-twice = Eine Person auf beiden Seiten einer Verbindung
quality-partner-twice = { $person } erscheint auf beiden Seiten dieser Verbindung: Es lohnt sich zu prüfen, wer der andere Partner war, oder die Seite leer zu lassen, wenn es niemand weiß.
quality-rule-foreign-surnames = Ein Sohn, dessen Nachname zu keinem Elternteil passt
quality-foreign-surnames = Ein Nachname, der zu keinem Elternteil passt: { $who }. Das kann normal sein — eine Namensänderung, ein Stiefkind, eine andere Schreibweise —, ist aber einen Blick wert. (Töchter bleiben außen vor, da meist ein Ehename der Grund ist.)
quality-rule-death-before-birth = Tod vor der Geburt
quality-death-before-birth = Für { $person } steht der Tod { $death } vor der Geburt { $birth }: Eines der beiden Daten ist vielleicht vertippt; ein Abgleich mit den Quellen lohnt sich.
quality-rule-child-before-parent = Ein Kind vor einem Elternteil geboren
quality-child-before-parent = { $child } (geb. { $childborn }) scheint vor dem Elternteil { $parent } (geb. { $parentborn }) geboren: Ein Blick auf die Daten oder die Abstammung lohnt sich.
quality-rule-parent-too-young = Ein Elternteil jünger als zwölf
quality-parent-too-young = { $parent } wäre bei der Geburt von { $child } { $age } Jahre alt gewesen, was ungewöhnlich ist: Ein Blick auf die Daten oder die Abstammung lohnt sich.
quality-rule-parent-too-old = Ein Elternteil älter als fünfundsechzig
quality-parent-too-old = { $parent } wäre bei der Geburt von { $child } { $age } Jahre alt gewesen, was ungewöhnlich ist: Ein Blick auf die Daten oder die Abstammung lohnt sich.
quality-rule-marriage-before-birth = Eine Heirat vor der Geburt eines Partners
quality-marriage-before-birth = Diese Heirat ist auf { $married } datiert, vor der Geburt von { $person } { $born }: Das Heiratsdatum ist einen Blick wert.
quality-rule-isolated-record = Ein Datensatz ohne Datum und ohne Beziehungen
quality-isolated-record = { $person } hat kein Datum und keine Verbindung zur Familie: Es lohnt sich zu ergänzen, wie die Person dazugehört, oder den Datensatz zu löschen, falls er versehentlich angelegt wurde.
quality-rule-place-no-region = Ein Ort ohne Region
quality-place-no-region = { $place } hat keine Region: Eine Region hilft, gleichnamige Orte auseinanderzuhalten.
quality-rule-region-free-text = Eine Region, die wie eine Notiz klingt
quality-region-free-text = Die Region von { $place } lautet „{ $value }“, was eher nach einer Notiz als nach einer Region aussieht: Es lohnt sich, die Notiz in den betreffenden Datensatz zu verschieben.
quality-rule-place-near-duplicate = Orte, die vielleicht derselbe sind
quality-place-near-duplicate = { $names } sind vielleicht derselbe Ort in anderer Schreibweise: Wenn ja, lassen sie sich zu einem zusammenführen.
quality-rule-not-a-place = Ein Eintrag, der vielleicht kein Ort ist
quality-not-a-place = „{ $place }“ steht in der Ortsliste, ist aber vielleicht kein Ort — es beginnt mit einem Kleinbuchstaben, was Ortsnamen selten tun: Es lohnt sich zu prüfen, ob es eine Notiz zu einem Ort ist.
quality-rule-certain-without-source = Sicher, ohne Quelle
quality-certain-without-source = { $record } enthält eine als völlig sicher markierte Angabe ({ $claims }) ohne Quelle: Es lohnt sich, die Quelle hinzuzufügen oder die Gewissheit zu senken.
quality-dismiss = Geprüft — bei uns normal
quality-restore = Wieder anzeigen
quality-note-label = Warum (optional)
quality-show-dismissed = Geprüft und belassen
quality-show-open = Zurück zur offenen Liste
quality-dismissed-by = Geprüft von { $by } am { $at }
quality-gone-title = Dieser Befund ist nicht mehr da
quality-gone-detail = Der Datensatz hat sich seit dem Öffnen der Liste geändert, der Befund lautet nicht mehr gleich. Laden Sie die Liste neu und sehen Sie noch einmal nach.
quality-summary = { $n ->
    [one] { $n } Sache, die einen Blick wert ist
   *[other] { $n } Dinge, die einen Blick wert sind
}
quality-summary-file = { $n ->
    [one] { $n } Sache in dieser Datei, die einen Blick wert ist
   *[other] { $n } Dinge in dieser Datei, die einen Blick wert sind
}
quality-dismissed-summary = { $n ->
    [one] { $n } Befund geprüft und belassen
   *[other] { $n } Befunde geprüft und belassen
}
quality-dismiss-all = Alle als geprüft markieren ({ $n })
quality-restore-all = Alle wieder anzeigen ({ $n })
quality-bulk-conflict = Seit dem Öffnen der Liste geänderte Datensätze: { $n }. Nichts wurde markiert; laden Sie die Liste neu und versuchen Sie es erneut.

## The record's editor (v0.4.2)

edit-tab-changed = geändert, noch nicht gespeichert
edit-remove = Entfernen
edit-add-partner = Partner hinzufügen
edit-add-child = Kind hinzufügen
edit-union-with = Verbindung mit { $names }
edit-union-alone = Verbindung ohne erfassten Partner
edit-child-of = Kind von { $names }
edit-new-family = Familie anlegen
edit-cite-title = Was sich auf eine Quelle stützt
edit-attach-to = Zitieren für
edit-add-source = Quelle hinzufügen
edit-error-source-title = Eine neue Quelle braucht einen Titel. Nichts wurde gespeichert.
edit-detach-file = Von diesem Eintrag lösen
edit-no-sources = Noch keine eigene Angabe dieser Person zitiert eine Quelle.
edit-no-files = Diesem Eintrag ist noch keine Datei angehängt.
edit-saved = { $n ->
        [one] Gespeichert: ein Eintrag geschrieben.
       *[other] Gespeichert: { $n } Einträge in einem Speichervorgang geschrieben.
    }
edit-saved-none = Nichts hatte sich geändert, also wurde nichts gespeichert.
edit-back-to-record = Zurück zum Eintrag
edit-avatar = Profilbild
edit-avatar-frame = Gesicht ausrichten
edit-avatar-upload = Als Profilbild verwenden
edit-stale = Jemand anderes hat { $what } geändert, seit diese Seite geöffnet wurde. Nichts wurde gespeichert; alle Ihre Eingaben sind noch da.
edit-refused = Nichts wurde gespeichert: { $reason }

# Administration console (v0.5.0, docs/adr/0011)
error-cross-site-title = Abgelehnt: von einer anderen Website gesendet
error-cross-site-detail = Dieses Formular kam von einer Seite einer anderen Website und wurde daher nicht ausgeführt. Wenn Sie es von dieser Website aus gesendet haben, laden Sie die Seite neu und senden Sie es erneut.
admin-nav-label = Bereiche der Verwaltung
admin-nav-overview = Übersicht
admin-nav-bundle = AXGF-Archiv
admin-nav-infrastructure = Infrastruktur
admin-nav-accounts = Konten
admin-nav-connectivity = Konnektivität

# Administration console: the AXGF bundle (v0.5.0, docs/adr/0011)
bundle-title = AXGF-Archiv
bundle-lede = Das Archiv der Familie, wie diese Website es hält: { $path }. Woran es sich hält, was falsch aussieht, und die zwei Wege, eine Kopie mitzunehmen.
bundle-unset-title = Noch keine Familie.
bundle-unset-detail = Diese Installation hält ein leeres Archiv, es gibt also noch nichts zu prüfen. Richten Sie sie aus einem Archiv, einer GEDCOM-Datei oder aus dem Nichts ein.
bundle-kpi-conformity = Konformität
bundle-verdict-ok = Konform
bundle-verdict-warn = Im Auge behalten
bundle-verdict-fail = Nicht konform
findings-tally = Grün: { $ok } · Warnungen: { $warn } · Kritisch: { $fail }
bundle-kpi-entities = Einträge
bundle-kpi-entities-note = { $persons ->
    [one] 1 Person
    *[other] { $persons } Personen
    } · { $files ->
    [one] 1 angehängte Datei
    *[other] { $files } angehängte Dateien
    }
bundle-kpi-size = Größe auf der Festplatte
bundle-kpi-members = { $n ->
    [one] 1 Datei im Archiv
    *[other] { $n } Dateien im Archiv
    }
bundle-kpi-format = Format
bundle-kpi-format-note = Diese Version liest und schreibt AXGF { $current }.
bundle-conformity = Konformität
bundle-conformity-intro = Neun Prüfungen am Archiv, wie es jetzt ist; keine verändert es. Öffnen Sie eine Zeile, um zu sehen, was zu tun ist.
bundle-area-format = Format
bundle-area-integrity = Integrität
bundle-area-privacy = Privatsphäre
bundle-area-content = Inhalt
level-ok = Grün
level-warn = Warnung
level-fail = Kritisch
findings-what-to-do = Was zu tun ist
findings-go = Dorthin →
bundle-anomalies = Auffälligkeiten
bundle-anomalies-intro = Was die Seite zur Datenqualität findet, Regel für Regel: noch offen und bereits als geprüft markiert.
bundle-anomalies-rule = Regel
bundle-anomalies-open = Offen
bundle-anomalies-checked = Geprüft
bundle-anomalies-none = Die Seite zur Datenqualität findet nichts zu prüfen.
bundle-diagnostics = Meldungen des Validators
bundle-facts = Eckdaten
bundle-fact-path = Datei
bundle-fact-size = Größe
bundle-fact-spec = Format
bundle-fact-schema = Mitgeführtes Schema
bundle-fact-none = keines
bundle-fact-created = Erstellt
bundle-fact-updated = Zuletzt geschrieben
bundle-fact-generator = Geschrieben von
bundle-fact-gedcom = Umgewandelt aus
bundle-export = Exportieren
bundle-export-raw = Das Archiv, wie es ist (.axgf)
bundle-export-raw-go = Archiv herunterladen
bundle-export-sealed = Für eine Person verschlüsselt (.axgf.gpg)
bundle-export-sealed-intro = Dasselbe Archiv als OpenPGP-Nachricht, die nur der Inhaber des privaten Schlüssels öffnen kann. Fügen Sie seinen öffentlichen Schlüssel ein: den Text, der mit „-----BEGIN PGP PUBLIC KEY BLOCK-----“ beginnt.
bundle-export-sealed-go = Verschlüsselt herunterladen
seal-recipient = Für
seal-fingerprint = Fingerabdruck
seal-algorithm = Algorithmen
seal-fips-yes = im FIPS-zugelassenen Satz
seal-fips-no = außerhalb des FIPS-zugelassenen Satzes
seal-expires = Schlüssel läuft ab
seal-never = nie
seal-key-label = Der öffentliche Schlüssel des Empfängers
seal-key-hint = Ein RSA-Schlüssel mit mindestens 2048 Bit oder ein Schlüssel mit elliptischer Kurve. Prüfen Sie den Fingerabdruck mit dem Inhaber, bevor Sie etwas senden.
seal-remember = Diesen Schlüssel für das nächste Mal behalten
seal-check = Nur den Schlüssel lesen
seal-fips-note = Das Archiv wird mit AES-256 für den Schlüssel des Empfängers verschlüsselt. FIPS-zugelassen sind RSA ab 3072 Bit und die NIST-Kurven P-256, P-384 und P-521: Bitten Sie um P-384. Die hier verwendete Bibliothek ist selbst kein nach FIPS 140-3 validiertes Modul.
seal-decrypt-note = Der Empfänger öffnet es mit GnuPG:
seal-forget = Gespeicherten Schlüssel vergessen
seal-key-kept = Der Schlüssel { $fingerprint } wird für den nächsten verschlüsselten Export behalten.
seal-error-no-key = Fügen Sie zuerst den öffentlichen Schlüssel des Empfängers ein.
seal-error-not-a-key = Das ist kein öffentlicher OpenPGP-Schlüssel. Fügen Sie den ganzen Block ein, von seiner BEGIN- bis zu seiner END-Zeile, und niemals einen privaten Schlüssel.
seal-error-bad-signatures = Die eigenen Signaturen dieses Schlüssels lassen sich nicht prüfen: Er wurde vielleicht verändert. Bitten Sie den Inhaber um eine frische Kopie.
seal-error-expired = Dieser Schlüssel oder jeder darin, der verschlüsseln kann, ist abgelaufen. Bitten Sie den Inhaber, ihn zu verlängern oder einen neuen zu senden.
seal-error-revoked = Der Inhaber hat diesen Schlüssel widerrufen. Bitten Sie ihn um den neuen.
seal-error-no-encryption-key = Dieser Schlüssel kann signieren, aber nicht verschlüsseln: Er hat keinen Verschlüsselungs-Unterschlüssel. Bitten Sie den Inhaber um einen (gpg --quick-add-key … encr).
seal-error-too-weak = Dieser RSA-Schlüssel hat { $bits } Bit; ein Export braucht mindestens { $min }.
seal-error-unsupported = Der Algorithmus dieses Schlüssels wird für den Export nicht verwendet: nur RSA- oder Elliptische-Kurven-Schlüssel.
seal-error-io = Die verschlüsselte Kopie konnte nicht erstellt werden: { $error }
date-time-utc = { $date }, { $time } UTC

# Administration console: the bundle's conformity (v0.5.0, docs/adr/0011)
bundle-check-format-title = Formatversion
bundle-check-format-ok = AXGF { $version }, die Version, die dieser Build liest und schreibt.
bundle-check-format-missing = Das Manifest nennt keine AXGF-Version.
bundle-check-format-missing-fix = Das Archiv stammt nicht von einem AXGF-Werkzeug oder wurde beschädigt. Stellen Sie die letzte gültige Sicherung wieder her (axgf-cms restore).
bundle-check-format-older = Als AXGF { $version } geschrieben; das nächste Speichern schreibt es als AXGF { $current }.
bundle-check-format-older-fix = Nichts zu tun: Die nächste Änderung schreibt es neu. Behalten Sie eine Kopie der alten Datei, falls ein anderes Programm nur { $version } liest.
bundle-check-format-newer = Als AXGF { $version } geschrieben, neuer als die { $current } dieses Builds: Speichern könnte verlieren, was er nicht versteht.
bundle-check-format-newer-fix = Aktualisieren Sie axgf-cms auf eine Version, die AXGF { $version } liest, bevor jemand etwas ändert (Infrastruktur, Version).
bundle-check-manifest-title = Die Zählungen des Manifests
bundle-check-manifest-ok = Die Zählungen des Manifests stimmen mit dem Inhalt des Archivs überein.
bundle-check-manifest-drift = Das Manifest gibt { $declared } an, das Archiv enthält { $actual } ({ $collection }).
bundle-check-manifest-fix = Nichts geht verloren: Die Zählungen werden beim nächsten Speichern neu geschrieben. Stammt das Archiv von anderswo, prüfen Sie, ob es vollständig kopiert wurde.
bundle-check-validation-title = Validierung
bundle-check-validation-ok = Der Validator der Bibliothek findet keinen Fehler.
bundle-check-validation-errors = { $errors ->
    [one] 1 Fehler
    *[other] { $errors } Fehler
    }: Das Archiv ist nicht gültig.
bundle-check-validation-errors-fix = Jede Meldung unten nennt den Eintrag und das Feld. Korrigieren Sie den Eintrag oder stellen Sie die letzte gültige Sicherung wieder her.
bundle-check-validation-warnings = { $warnings ->
    [one] 1 Warnung
    *[other] { $warnings } Warnungen
    }. Sie blockieren nie das Speichern; jede deutet an, dass etwas nicht stimmen könnte.
bundle-check-validation-warnings-fix = Lesen Sie sie unten. Die meisten sind Verweise auf Einträge, die nicht im Archiv sind; ein GEDCOM-Import hinterlässt sie oft.
bundle-check-payloads-title = Angehängte Dateien
bundle-check-payloads-none = Keine angehängten Dateien.
bundle-check-payloads-ok = { $declared ->
    [one] Die eine Datei ist vorhanden.
    *[other] Alle { $declared } Dateien sind vorhanden.
    }
bundle-check-payloads-missing = { $missing } von { $declared } angehängten Dateien fehlen im Cache.
bundle-check-payloads-fix = Das nächste Speichern baut den Cache aus dem Archiv neu auf. Fehlen sie im Archiv selbst, stellen Sie sie aus einer Sicherung wieder her.
bundle-check-binding-title = Konten und Archiv
bundle-check-binding-ok = Die Kontendatei wurde für diese Familie angelegt.
bundle-check-binding-mismatch = Die Kontendatei wurde für { $expected } angelegt; dieses Archiv ist { $found }.
bundle-check-binding-fix = Wurde das Archiv absichtlich wiederhergestellt oder umbenannt, ist nichts zu tun. Sonst gelten die Konten einer Familie für den Baum einer anderen: Legen Sie die richtige .acl daneben.
bundle-check-living-title = Lebende, öffentlich
bundle-check-living-ok = Niemand, der als lebend erfasst ist, ist für Besucher sichtbar.
bundle-check-living-public = { $n ->
    [one] 1 als lebend erfasste Person ist
    *[other] { $n } als lebend erfasste Personen sind
    } öffentlich: Jeder Besucher liest ihren Eintrag.
bundle-check-living-fix = Sofern sie nicht zugestimmt haben, setzen Sie ihre Sichtbarkeit auf Mitglieder (Bearbeiten, Identität).
bundle-check-special-title = Besondere Kategorien
bundle-check-special-none = Kein Eintrag enthält Gesundheits-, biometrische, genomische, strafrechtliche oder Verhaltensdaten.
bundle-check-special-some = { $n ->
    [one] 1 Eintrag enthält
    *[other] { $n } Einträge enthalten
    } Gesundheits-, biometrische, genomische, strafrechtliche oder Verhaltensdaten. Bei Lebenden bleiben sie bei den Administratoren, was auch immer der Eintrag sagt.
bundle-check-quality-title = Datenqualität
bundle-check-quality-ok = Nichts zu prüfen.
bundle-check-quality-open = { $n ->
    [one] 1 Ding zu prüfen
    *[other] { $n } Dinge zu prüfen
    }, in { $groups ->
    [one] 1 Gruppe
    *[other] { $groups } Gruppen
    }.
bundle-check-quality-fix = Öffnen Sie die Seite zur Datenqualität: Jede Zeile nennt den Eintrag und was falsch aussieht. Korrigieren Sie ihn oder markieren Sie ihn als geprüft.
bundle-check-size-title = Größe
bundle-check-size-ok = { $size }, unter den { $warn }, ab denen diese Seite warnt.
bundle-check-size-heavy = { $size }, mehr als { $warn }.
bundle-check-size-fix = Das Archiv gehört der Familie und darf wachsen. Behalten Sie Festplatte und Sicherungen im Blick (Infrastruktur, Volumen) und erhöhen Sie --size-warn-mb, wenn diese Größe erwartet ist.
seal-algorithm-value = { $key } · AES-256
seal-key-placeholder = -----BEGIN PGP PUBLIC KEY BLOCK-----

# Administration console: the infrastructure's checks (v0.5.0, docs/adr/0011 §3)
infra-area-security = Sicherheit
infra-area-access = Zugang
infra-area-data = Daten
infra-area-capacity = Kapazität
infra-area-release = Version
infra-area-platform = Plattform
infra-verdict-ok = Gesund
infra-verdict-warn = Braucht Aufmerksamkeit
infra-verdict-fail = Kritisch
duration-days = { $n ->
    [one] { $n } Tag
    *[other] { $n } Tage
    }
duration-hours = { $n ->
    [one] { $n } Stunde
    *[other] { $n } Stunden
    }
duration-minutes = { $n ->
    [one] { $n } Minute
    *[other] { $n } Minuten
    }
infra-transport-title = Wie die Website erreicht wird
infra-transport-ok = Diese Seite kam über HTTPS.
infra-transport-exposed = Diese Seite kam unverschlüsselt, und der Server lauscht auf { $bind }, über diese Maschine hinaus: Passwörter und Sitzungscookies gehen im Klartext durchs Netz.
infra-transport-exposed-fix = Stellen Sie einen HTTPS-Reverse-Proxy (Caddy, nginx, HAProxy) davor und binden Sie den Dienst wieder an 127.0.0.1: bootstrap.sh mit --bind 127.0.0.1:8080 erneut ausführen oder das Drop-in entfernen, das ihn geöffnet hat.
infra-transport-plain = Diese Seite kam unverschlüsselt von einem Server, der auf { $bind } lauscht. Das ist auf dieser Maschine allein in Ordnung, und nur dort.
infra-transport-plain-fix = Rufen Sie die Website über ihre HTTPS-Adresse auf. Wenn ein Proxy TLS beendet, lassen Sie ihn X-Forwarded-Proto: https senden, damit dieser Server es weiß.
infra-forwarded-title = Weitergeleitete Adressen
infra-forwarded-ok = Der Server lauscht nur auf dieser Maschine, daher ist die vom Proxy weitergereichte Client-Adresse verlässlich, um wiederholte Fehlanmeldungen zu bremsen.
infra-forwarded-exposed = Der Server ist direkt erreichbar und vertraut dem Header X-Forwarded-For: Ein Client, der ihn selbst setzt, entgeht der Bremse nach Fehlanmeldungen.
infra-forwarded-fix = Binden Sie den Dienst hinter dem Proxy an 127.0.0.1, oder sperren Sie den Port per Firewall, sodass nur der Proxy ihn erreicht.
infra-cross-site-title = Formulare von anderen Websites
infra-cross-site-ok = Am letzten Tag wurde kein Formular von einer anderen Website hierher geschickt.
infra-cross-site-refused = { $n ->
    [one] Am letzten Tag wurde ein Formular von einer anderen Website abgewiesen.
    *[other] Am letzten Tag wurden { $n } Formulare von anderen Websites abgewiesen.
    }
infra-cross-site-fix = Nichts ist durchgekommen. Wenn die Abweisungen Ihre eigenen sind — dieselbe Website über eine zweite Adresse —, legen Sie unter Konnektivität die öffentliche Adresse fest oder nutzen Sie nur diese.
infra-sso-title = Anmeldedienste
infra-sso-none = Kein Anmeldedienst ist aktiviert; Konten melden sich mit einem Passwort an.
infra-sso-ok = { $n ->
    [one] Ein Anmeldedienst ist aktiviert, und die öffentliche Adresse ist HTTPS.
    *[other] { $n } Anmeldedienste sind aktiviert, und die öffentliche Adresse ist HTTPS.
    }
infra-sso-no-https = Ein Anmeldedienst ist aktiviert, aber die öffentliche Adresse ist nicht HTTPS: Anbieter schicken ihre Antwort nicht an eine unverschlüsselte Adresse, und die Anmeldung scheitert.
infra-sso-fix = Legen Sie unter Konnektivität als öffentliche Adresse die https://-Adresse fest, die Leser verwenden.
infra-admins-title = Administratoren
infra-admins-none = Kein aktives Konto kann diese Website verwalten. Nur das Notfall-Token kann es, und nur solange es gesetzt ist.
infra-admins-none-fix = Melden Sie sich mit dem Notfall-Token an, legen Sie unter Konten einen Administrator an und entfernen Sie dann das Token.
infra-admins-one = Ein einziger Administrator. Ist diese Person abwesend oder verliert das Passwort, kann niemand die Website verwalten.
infra-admins-one-fix = Geben Sie unter Konten einer zweiten Vertrauensperson die Administratorrolle.
infra-admins-ok = { $n ->
    [one] { $n } Administrator kann sich anmelden.
    *[other] { $n } Administratoren können sich anmelden.
    }
infra-token-title = Notfall-Token
infra-token-generated = Kein Notfall-Token ist gesetzt.
infra-token-only-way = Ein Notfall-Token ist gesetzt, und ohne Administratorkonto ist es der einzige Zugang. Behalten Sie es, bis es einen Administrator gibt.
infra-token-standing = Ein Notfall-Token ist noch gesetzt. Es meldet an allen Konten und Berechtigungen vorbei an.
infra-token-fix = Entfernen Sie AXGF_CMS_ADMIN_TOKEN aus der Umgebungsdatei des Dienstes und starten Sie den Dienst neu.
infra-registration-title = Selbstregistrierung
infra-registration-closed = Besucher können keine Konten anlegen; das kann nur ein Administrator.
infra-registration-secret = Besucher können ein Mitgliedskonto anlegen, wenn sie das Registrierungsgeheimnis kennen.
infra-registration-open = Jeder Besucher kann ein Mitgliedskonto anlegen.
infra-registration-open-fix = Das passt zu einer Website, die offen sein soll. Andernfalls verlangen Sie unter Konten das Registrierungsgeheimnis oder schließen die Registrierung.
infra-failures-title = Fehlgeschlagene Anmeldungen
infra-failures-ok = { $n ->
    [one] { $n } fehlgeschlagene Anmeldung am letzten Tag.
    *[other] { $n } fehlgeschlagene Anmeldungen am letzten Tag.
    }
infra-failures-many = { $n ->
    [one] { $n } fehlgeschlagene Anmeldung am letzten Tag: Jemand rät womöglich Passwörter.
    *[other] { $n } fehlgeschlagene Anmeldungen am letzten Tag: Jemand rät womöglich Passwörter.
    }
infra-failures-fix = Jede Adresse wird nach wiederholten Fehlschlägen bereits gebremst. Steigt die Zahl weiter, suchen Sie im Proxy-Log die Herkunft der Versuche und sperren Sie sie dort; sorgen Sie dafür, dass jeder Administrator ein langes Passwort hat oder sich über einen Anmeldedienst anmeldet.
infra-health-bundle-title = Familiendaten
infra-health-bundle-ok = Das Archiv öffnet sich und besteht die Validierung.
infra-health-bundle-fix = Öffnen Sie die Seite AXGF-Archiv, um die Fehler zu sehen und zu beheben, oder stellen Sie die letzte gültige Sicherung wieder her (axgf-cms restore).
infra-health-cache-title = Anhang-Cache
infra-health-cache-ok = Jede angehängte Datei liegt im Cache.
infra-health-cache-fix = Speichern Sie einen beliebigen Datensatz einmal: Das Speichern baut den Cache neu auf. Dateien, die danach noch fehlen, fehlen im Archiv selbst.
infra-health-backup-title = Sicherungen
infra-health-backup-ok = { $count ->
    [one] Eine aktuelle Sicherung existiert; { $count } Archiv wird aufbewahrt.
    *[other] Eine aktuelle Sicherung existiert; { $count } Archive werden aufbewahrt.
    }
infra-health-backup-fix = Ohne Sicherungsverzeichnis führen Sie bootstrap.sh erneut mit --backup-dir aus. Mit einem prüfen Sie den Timer (systemctl status axgf-cms-backup.timer) und sein Log (journalctl -u axgf-cms-backup).
infra-disk-title = Speicherplatz
infra-disk-ok = { $free } frei von { $total } ({ $percent } % frei).
infra-disk-fix = Schaffen Sie Platz auf der Partition mit dem Datenverzeichnis — alte Sicherungen, die auch anderswo liegen, Logs, Dateien anderer Programme — oder ziehen Sie die Daten auf eine größere Platte um.
infra-growth-title = Wachstum
infra-growth-ok = { $days ->
    [one] Wächst um etwa { $rate } im Monat; Platz für noch { $days } Tag bei diesem Tempo.
    *[other] Wächst um etwa { $rate } im Monat; Platz für noch { $days } Tage bei diesem Tempo.
    }
infra-growth-soon = { $days ->
    [one] Wächst um etwa { $rate } im Monat: Bei diesem Tempo ist die Platte in { $days } Tag voll.
    *[other] Wächst um etwa { $rate } im Monat: Bei diesem Tempo ist die Platte in { $days } Tagen voll.
    }
infra-growth-fix = Planen Sie den Platz jetzt: eine größere Platte, weniger Sicherungen auf derselben Platte oder die Sicherungen auf eine andere verlegen.
infra-growth-flat = Die Sicherungen sind nicht gewachsen; es gibt nichts zu planen.
infra-growth-unknown = Noch nicht genug Sicherungen, um das Wachstum zu messen: Zwischen der ältesten und der neuesten muss mindestens ein Tag liegen.
infra-memory-title = Arbeitsspeicher
infra-memory-ok = { $used } belegt von der Obergrenze des Dienstes, { $max } ({ $percent } %).
infra-memory-high = { $used } belegt von der Obergrenze des Dienstes, { $max } ({ $percent } %): nahe an dem Punkt, an dem das System den Dienst beendet.
infra-memory-fix = Erhöhen Sie MemoryMax= in einem Drop-in für axgf-cms.service, oder finden Sie, was den Speicher hält — ein sehr großer Import, ein großer Anhang — und starten Sie den Dienst neu.
infra-memory-unlimited = { $used } belegt, ohne Obergrenze für den Dienst.
infra-version-title = Version
infra-version-ok = v{ $current } ist die neueste veröffentlichte Version.
infra-version-behind = v{ $current } läuft; { $latest } ist veröffentlicht.
infra-version-behind-fix = Lesen Sie die Versionshinweise und aktualisieren Sie dann im Abschnitt Version unten. Zuerst wird gesichert, und die vorige Version kehrt von selbst zurück, wenn die neue nicht startet.
infra-version-unknown = Ob es eine neuere Version gibt, ist unbekannt: Die Liste der Versionen wurde nicht abgerufen.
infra-version-unknown-fix = Nutzen Sie unten „Nach neuen Versionen suchen“. Die Sandbox der Produktion erreicht GitHub nicht selbst; ist der Upgrade-Helfer installiert, holt er die Liste täglich.
infra-version-stale = { $days ->
    [one] Die Liste der Versionen wurde zuletzt vor { $days } Tag abgerufen.
    *[other] Die Liste der Versionen wurde zuletzt vor { $days } Tagen abgerufen.
    }
infra-upgrades-title = Aktualisierungen
infra-upgrades-ok = Der Upgrade-Helfer ist installiert: Die Schaltfläche unten kann die Version wechseln.
infra-upgrades-manual = Der Upgrade-Helfer ist nicht installiert, daher wird die Version von Hand auf dem Server gewechselt.
infra-upgrades-fix = Führen Sie das aktuelle bootstrap.sh einmal als root aus: Es installiert den Helfer zusammen mit dem Dienst.
infra-sandbox-title = Sandbox
infra-sandbox-ok = Der Prozess läuft mit no-new-privileges und einem Systemaufruf-Filter.
infra-sandbox-unknown = Dieses System meldet die Sandbox des Prozesses nicht.
infra-sandbox-off = Der Prozess läuft ohne einen Teil seiner Sandbox: no-new-privileges oder der Systemaufruf-Filter ist aus.
infra-sandbox-fix = Betreiben Sie den Dienst mit der Unit, die bootstrap.sh schreibt, ohne Drop-ins, die sie lockern. Auf einer Entwicklungsmaschine ist das zu erwarten.
infra-clock-title = Uhr
infra-clock-ok = Die Systemuhr zeigt { $year }, was plausibel ist.
infra-clock-wrong = Die Systemuhr zeigt { $year }: Daten, das Alter der Sicherungen und Sitzungen sind alle falsch.
infra-clock-fix = Schalten Sie die Zeitsynchronisation ein: timedatectl set-ntp true.

# Administration console: the Infrastructure page (v0.5.0, docs/adr/0011)
infra-title = Infrastruktur
infra-lede = Die Maschine, auf der diese Website läuft, bei jedem Öffnen dieser Seite geprüft: wie sie erreicht wird, wer die Schlüssel hat, ob die Daten sicher sind und noch Platz haben, welche Version läuft und die Sandbox darum.
infra-kpi-health = Zustand
infra-kpi-version = Version
infra-kpi-version-newer = { $version } verfügbar
infra-kpi-version-current = Aktuell
infra-kpi-version-unknown = Noch nicht geprüft
infra-kpi-disk = Freier Platz
infra-kpi-disk-note = { $percent } % von { $total } belegt
infra-kpi-compute = Maschine
infra-kpi-cpus = { $n ->
    [one] { $n } Kern
    *[other] { $n } Kerne
    }
infra-kpi-load = Last { $load }
infra-kpi-rss = { $size } belegt
infra-unknown = Unbekannt
infra-health = Zustand
infra-health-intro = Grün ist in Ordnung, eine Warnung ist einen Blick wert, kritisch braucht sofort Aufmerksamkeit. Jeder Befund öffnet sich auf das, was zu tun ist.
infra-json = Dieselben Prüfungen als JSON, für eine Überwachung
infra-version = Version
infra-version-running = Läuft
infra-version-latest = Zuletzt veröffentlicht
infra-version-up-to-date = aktuell
infra-version-checked = Liste abgerufen
infra-version-never = nie
infra-source-direct = von diesem Server
infra-source-helper = vom Upgrade-Helfer
infra-version-token = Privates Repository
infra-token-env-set = Ein GitHub-Token ist gesetzt, um Versionen aufzulisten und herunterzuladen.
infra-token-env-unset = Kein Token. Ist das Repository privat, legen Sie eines in { $name } ab, nur für root lesbar: Der Upgrade-Helfer liest es dort.
infra-releases-error = Der letzte Versuch, die Liste abzurufen, ist gescheitert:
infra-releases-refresh = Nach neuen Versionen suchen
infra-releases-asked = Der Upgrade-Helfer wurde um eine frische Liste gebeten. Laden Sie diese Seite in einer Minute neu.
infra-releases-not-asked = Die Anfrage konnte nicht geschrieben werden: { $error }
infra-upgrade-status-running = Wechsel von { $from } auf { $to } läuft …
infra-upgrade-status-succeeded = Von { $from } auf { $to } gewechselt.
infra-upgrade-status-failed = Der Wechsel von { $from } auf { $to } ist gescheitert; es läuft weiterhin { $from }.
infra-upgrade-status-refused = Der Upgrade-Helfer hat den Wechsel auf { $to } abgelehnt.
infra-upgrade-log = Ausgabe des Helfers
infra-upgrade-pending = { $by } hat am { $at } { $version } angefordert; der Upgrade-Helfer hat die Anfrage noch nicht übernommen.
infra-releases = Veröffentlichte Versionen
infra-releases-tag = Version
infra-releases-published = Veröffentlicht
infra-releases-relation = Im Vergleich zu dieser
infra-releases-notes = Versionshinweise
infra-releases-open = Lesen
infra-relation-newer = neuer
infra-relation-current = diese
infra-relation-older = älter
infra-prerelease = Release Candidate
infra-releases-none = Noch keine Liste der Versionen.
infra-upgrade = Version wechseln
infra-upgrade-intro = Die Schaltfläche fragt den Upgrade-Helfer, der als root außerhalb der Sandbox dieser Website läuft. Er sichert, installiert die Version nach geprüfter Prüfsumme, startet den Dienst neu und prüft ihn; antwortet die neue Version nicht, wird die vorige wiederhergestellt.
infra-upgrade-version = Zu installierende Version
infra-upgrade-confirm = Um zu einer älteren Version zurückzukehren, geben Sie ihr Tag erneut ein
infra-upgrade-confirm-hint = Für eine neuere Version nicht nötig. Eine ältere Version liest Daten einer neueren womöglich nicht: Lesen Sie zuerst ihre Hinweise.
infra-upgrade-go = Diese Version installieren
infra-upgrade-safety = Die Website ist einige Sekunden nicht erreichbar, während der Dienst neu startet.
infra-upgrade-manual = Der Upgrade-Helfer ist auf diesem Server nicht installiert, daher kann diese Seite die Version nicht wechseln. Als root auf dem Server:
infra-upgrade-manual-helper = Einmal das aktuelle bootstrap.sh ausführen installiert den Helfer, und die Schaltfläche hier funktioniert ab dann.
infra-upgrade-asked = { $version } wurde angefordert. Der Upgrade-Helfer übernimmt die Anfrage in wenigen Sekunden; diese Seite zeigt, wie es gelaufen ist.
infra-upgrade-no-helper = Der Upgrade-Helfer ist nicht installiert, daher kann { $version } nicht von hier installiert werden.
infra-upgrade-unknown-version = { $version } ist nicht in der Liste der veröffentlichten Versionen.
infra-upgrade-same-version = { $version } ist die Version, die bereits läuft.
infra-upgrade-confirm-older = { $version } ist älter als die laufende Version. Geben Sie ihr Tag im Bestätigungsfeld ein, um zu ihr zurückzukehren.
infra-volumes = Speicher
infra-volumes-intro = Die Partition mit dem Datenverzeichnis ({ $path }) und was diese Website darauf aufbewahrt.
infra-volumes-partition = Partition
infra-of = { $used } von { $total }
infra-volumes-ours = Diese Website
infra-volumes-breakdown = Was diese Website aufbewahrt
infra-part-bundle = Familienarchiv
infra-part-cache = Anhang-Cache
infra-part-backups = Sicherungen
infra-part-binary = Programm
infra-part-journal = Änderungsjournal
infra-part-accounts = Konten und Einstellungen
infra-files = { $n ->
    [one] { $n } Datei
    *[other] { $n } Dateien
    }
infra-growth = Wachstum
infra-growth-month = Pro Monat
infra-growth-not-enough = zu wenige Sicherungen, um es zu sagen
infra-growth-left = Platz bei diesem Tempo
infra-backups = Sicherungen
infra-backups-dir = Verzeichnis
infra-backups-count = Archive
infra-backups-newest = Neueste
infra-backups-oldest = Älteste
infra-backups-room = Platz am Aufbewahrungsort
infra-free-of = { $free } frei von { $total }
infra-paths = Pfade
infra-path-data = Datenverzeichnis
infra-path-binary = Programm
infra-compute = Rechenleistung
infra-compute-hidden = Die Sandbox des Dienstes verbirgt ihm die Zahlen der Maschine selbst; was hier steht, ist das, was der Prozess sehen kann.
infra-compute-memory = Speicher der Maschine
infra-available-of = { $available } frei von { $total }
infra-compute-ceiling = Speicher des Dienstes
infra-compute-no-ceiling = { $used }, ohne Obergrenze
infra-compute-os = System
infra-compute-kernel = Kernel
infra-compute-arch = Architektur
infra-compute-cpu = Prozessor
infra-compute-cpus = Kerne
infra-compute-load = Last (1, 5, 15 Min.)
infra-compute-rss = Dieser Prozess
infra-compute-threads = Threads
infra-compute-files = Offene Dateien
infra-compute-uptime = Dienst läuft seit
infra-compute-sys-uptime = Maschine läuft seit
infra-compute-sandbox = Sandbox
infra-compute-nnp-on = keine neuen Rechte
infra-compute-nnp-off = Rechte können erlangt werden
infra-compute-seccomp-on = Systemaufrufe gefiltert
infra-compute-seccomp-off = Systemaufrufe ungefiltert
infra-activity = Aktivität
infra-activity-intro = Gezählt über { $span }: Der Dienst behält einen Tag und beginnt bei jedem Neustart bei null.
infra-activity-sessions = Offene Sitzungen
infra-activity-signed-in = Anmeldungen
infra-activity-failed = Fehlgeschlagene Anmeldungen
infra-activity-registered = Angelegte Konten
infra-activity-cross-site = Abgewiesene fremde Formulare

# Sign-in services: what the reader is told when one fails (v0.5.0, docs/adr/0012)
sso-error-not-configured = Dieser Anmeldedienst ist auf dieser Website nicht eingerichtet.
sso-error-no-public-url = Die Anmeldung über einen Dienst braucht die öffentliche Adresse dieser Website, die noch nicht festgelegt ist. Ein Administrator legt sie unter Konnektivität fest.
sso-error-unreachable = Der Anmeldedienst war nicht erreichbar. Versuchen Sie es gleich noch einmal oder melden Sie sich anders an.
sso-error-refused = Der Anmeldedienst hat Sie nicht hereingelassen, oder die Anmeldung wurde dort abgebrochen.
sso-error-stale = Diese Anmeldung war abgelaufen oder wurde in einem anderen Browser begonnen. Beginnen Sie erneut.
sso-error-invalid = Die Antwort des Anmeldedienstes hat die Prüfungen dieser Website nicht bestanden und wurde nicht verwendet. Beginnen Sie erneut; wenn es wieder passiert, sagen Sie einem Administrator Bescheid.
infra-upgrade-already-pending = { $version } wartet bereits auf den Upgrade-Helfer. Warten Sie das Ende ab, bevor Sie eine andere Version anfordern.

# Administration console: Connectivity, sign-in services and registration (v0.5.0, docs/adr/0012)
connectivity-title = Konnektivität
connectivity-lede = Wie Leser diese Website erreichen und wie sie sich anmelden: die öffentliche Adresse, die Anmeldedienste und die Regeln jeder Anmeldung, die der Administratoren eingeschlossen.
connectivity-kpi-address = Öffentliche Adresse
connectivity-address-unset = Nicht festgelegt
connectivity-address-https = HTTPS
connectivity-address-plain = Unverschlüsselt
connectivity-address-unset-note = Jeder Anmeldedienst braucht sie
connectivity-kpi-services = Anmeldedienste
connectivity-kpi-services-note = { $n ->
    [one] bereit
    *[other] bereit
    }
connectivity-kpi-passwords = Passwörter
connectivity-policy-short-everyone = Alle
connectivity-policy-short-non-admins = Nicht für Administratoren
connectivity-policy-short-nobody = Niemand
connectivity-policy-note-everyone = Jedes Konto kann sich mit seinem Passwort anmelden und auch über einen verknüpften Dienst.
connectivity-policy-note-non-admins = Administratoren melden sich nur über einen Dienst an; alle anderen dürfen weiter ein Passwort nutzen. Ein gestohlenes Administratorpasswort öffnet dann nichts mehr.
connectivity-policy-note-nobody = Alle melden sich über einen Dienst an. Ein Konto ohne verknüpften Dienst und ohne Adresse zum Verknüpfen kann sich nicht anmelden.
connectivity-kpi-admins-linked = { $n ->
    [one] { $n } Administrator mit verknüpftem Dienst
    *[other] { $n } Administratoren mit verknüpftem Dienst
    }
connectivity-kpi-registration = Registrierung
connectivity-kpi-registration-go = Wird unter Konten festgelegt →
registration-short-closed = Nur Administratoren
registration-short-secret = Mit dem Familiengeheimnis
registration-short-open = Offen für Besucher
connectivity-address = Öffentliche Adresse
connectivity-address-intro = Die Adresse, die Leser eintippen, so wie sie sie eintippen: https://family.example. Anmeldedienste schicken Leser dorthin zurück, also muss es die öffentliche Adresse sein, nicht die, auf der dieser Server lauscht.
connectivity-address-label = Öffentliche Adresse
connectivity-address-hint = Leer lassen, solange sich niemand über einen Dienst anmeldet.
connectivity-address-invalid = Das ist keine http- oder https-Adresse mit Hostnamen.
connectivity-address-saved = Die öffentliche Adresse ist gespeichert.
connectivity-policy = Anmelderegeln
connectivity-policy-intro = Sie gelten für jedes Konto, auch für Administratoren. Eine Regel, nach der sich kein Administrator mehr anmelden könnte, wird abgelehnt.
connectivity-policy-passwords = Wer sich mit Passwort anmelden darf
connectivity-policy-without-way = { $n ->
    [one] { $n } aktives Konto hat weder verknüpften Dienst noch Adresse: Mit „Niemand“ könnte es sich nicht anmelden.
    *[other] { $n } aktive Konten haben weder verknüpften Dienst noch Adresse: Mit „Niemand“ könnten sie sich nicht anmelden.
    }
connectivity-link-by-email = Einen Dienst mit dem Konto gleicher bestätigter Adresse verknüpfen
connectivity-link-by-email-note = So bekommt ein Verwandter eine Anmeldung mit einem Klick ohne Registrierung: Legen Sie sein Konto mit seiner Adresse an, und seine erste Anmeldung über einen Dienst, der diese Adresse bestätigt hat, verknüpft es. Abgeglichen wird nur eine Adresse, die ein Administrator eingegeben oder ein Dienst bestätigt hat.
connectivity-domains = Domains, die ein Konto eröffnen dürfen
connectivity-domains-hint = Eine pro Zeile, etwa family.example. Leer: alle. Sind welche gesetzt, kann nur eine von einem Dienst bestätigte Adresse aus diesen Domains ein Konto eröffnen, und das Registrierungsformular wird nicht angeboten.
connectivity-domain-invalid = „{ $domain }“ ist kein Domainname.
connectivity-policy-unknown = Wählen Sie, wer sich mit Passwort anmelden darf.
connectivity-policy-lockout = Abgelehnt: Mit dieser Änderung könnte sich kein aktiver Administrator anmelden. Verknüpfen Sie zuerst einen Anmeldedienst mit einem Administratorkonto, indem Sie sich einmal darüber anmelden, oder behalten Sie Passwörter für Administratoren.
connectivity-policy-saved = Die Anmelderegeln sind gespeichert.
connectivity-not-saved = Nicht gespeichert: { $error }
connectivity-services = Anmeldedienste
connectivity-services-intro = Registrieren Sie diese Website in der Konsole des Dienstes mit der angezeigten Weiterleitungsadresse und fügen Sie dann Client-ID und Geheimnis hier ein. Besucher sehen dann „Weiter mit …“ auf den Anmelde- und Registrierungsseiten.
connectivity-outbound-note = Die Anmeldung über einen Dienst setzt voraus, dass dieser Server den Dienst erreicht. Die von bootstrap.sh geschriebene Unit verweigert ausgehende Verbindungen (IPAddressDeny=any): Erlauben Sie sie mit sudo systemctl edit axgf-cms, indem Sie unter [Service] IPAddressAllow=any ergänzen, und starten Sie neu. Die Testschaltfläche zeigt, ob es klappt.
connectivity-service-ready = Bereit
connectivity-service-incomplete = Unvollständig
connectivity-service-off = Aus
connectivity-service-enable = Diesen Dienst auf den Anmelde- und Registrierungsseiten anbieten
connectivity-service-issuer = Aussteller
connectivity-service-issuer-hint = Die Adresse des Anbieters, etwa https://auth.family.example/realms/family; seine Einstellungen werden aus /.well-known/openid-configuration darunter gelesen.
connectivity-service-label = Beschriftung der Schaltfläche
connectivity-service-tenant = Verzeichnis (Tenant)
connectivity-service-tenant-hint = common für jedes Microsoft-Konto, organizations für Arbeits- und Schulkonten, consumers für private Konten oder die ID Ihres Verzeichnisses.
connectivity-service-client-id = Client-ID
connectivity-service-secret = Client-Geheimnis
connectivity-service-secret-set = Ein Geheimnis ist gesetzt. Leer lassen, um es zu behalten.
connectivity-service-secret-unset = Noch kein Geheimnis gesetzt.
connectivity-service-secret-clear = Geheimnis entfernen
connectivity-service-redirect = Zu registrierende Weiterleitungsadresse
connectivity-service-redirect-needs-address = Zuerst die öffentliche Adresse festlegen
connectivity-service-console = Wo registrieren
connectivity-service-console-open = Konsole von { $name }
connectivity-service-test = Verbindung testen
connectivity-issuer-invalid = Der Aussteller muss eine https://-Adresse sein.
connectivity-tenant-invalid = Ein Verzeichnis ist ein Name oder eine ID: Buchstaben, Ziffern, Punkte und Bindestriche.
connectivity-provider-incomplete = { $name } ist unvollständig: Nötig sind eine Client-ID und ein Client-Geheimnis, bei einem generischen Anbieter auch ein Aussteller.
connectivity-provider-saved = { $name } ist gespeichert.
connectivity-test-ok = { $name } hat geantwortet: { $answer }.
connectivity-test-failed = { $name } war von diesem Server aus nicht erreichbar: { $detail }
registration-title = Registrierung
registration-intro = Ob ein Besucher hier selbst ein Konto eröffnen darf. Ein solches Konto ist das eines Mitglieds: Es liest, was Mitglieder lesen, und ändert nichts. Ihm eine andere Rolle zu geben, entscheidet ein Administrator, weiter unten.
registration-who = Wer ein Konto eröffnen darf
registration-note-closed = Nur ein Administrator legt Konten an, auf dieser Seite.
registration-note-secret = Ein Besucher, der das Familiengeheimnis kennt, kann ein Mitgliedskonto eröffnen. Geben Sie das Geheimnis weiter wie einen Türcode.
registration-note-open = Jeder Besucher kann ein Mitgliedskonto eröffnen. Mitglieder lesen die Datensätze, die Mitgliedern vorbehalten sind.
registration-rotate = Neues Geheimnis ziehen
registration-secret = Das Familiengeheimnis
registration-secret-note = Wer es hat, kann ein Konto eröffnen, solange dieser Modus aktiv ist. Ein neues zu ziehen sperrt das alte aus; bereits eröffnete Konten bleiben.
registration-link = Registrierungsseite
registration-quick = Schnelle Wege hinein
registration-quick-none = Kein Anmeldedienst ist bereit: Richten Sie unter Konnektivität einen ein
registration-changed = Zuletzt geändert von { $by }, { $at }.
registration-unknown = Wählen Sie, wer ein Konto eröffnen darf.
accounts-kpi-total = Aktive Konten
accounts-kpi-total-note = { $disabled ->
    [one] { $disabled } deaktiviert
    *[other] { $disabled } deaktiviert
    }
accounts-kpi-admins = Administratoren
accounts-kpi-admins-few = Einen zweiten hinzufügen
accounts-kpi-admins-ok = Mehr als einer, wie es sein soll
accounts-kpi-linked = Mit Dienst verknüpft
accounts-kpi-linked-note = { $n ->
    [one] { $n } Dienst bereit
    *[other] { $n } Dienste bereit
    }
accounts-kpi-registered = { $n ->
    [one] { $n } von Besuchern eröffnet
    *[other] { $n } von Besuchern eröffnet
    }
accounts-roles = Rollen
accounts-sign-in = Meldet sich an mit
accounts-sign-in-password = Passwort
accounts-sign-in-google = Google
accounts-sign-in-microsoft = Microsoft
accounts-sign-in-github = GitHub
accounts-sign-in-oidc = Single Sign-on
accounts-sign-in-none = noch kein Zugang
accounts-created-via-register = vom Besucher eröffnet
accounts-created-via-sso = über einen Dienst eröffnet
accounts-add-also-register = Besucher können auch selbst Mitgliedskonten eröffnen, wie oben festgelegt. Ein hier angelegtes Konto kann jede Rolle haben.
login-password-not-allowed = Die Anmelderegeln dieser Website erlauben diesem Konto kein Passwort: Melden Sie sich über einen Dienst an.
login-register = Noch kein Konto? Eines anlegen
register-title = Konto anlegen
register-lede = Ein Mitgliedskonto liest die Datensätze, die den Mitgliedern der Familie vorbehalten sind. Am schnellsten geht es mit einem Konto, das Sie schon haben.
register-closed = Konten legt hier ein Administrator an. Fragen Sie dort nach Ihrem.
register-have-account = Schon ein Konto? Anmelden
register-or = oder
register-secret = Das Familiengeheimnis
register-secret-hint = Ein Administrator hat es Ihnen gegeben, oder jemand aus der Familie.
register-secret-hint-services = Geben Sie es ein, bevor Sie einen Dienst wählen, sonst fragt der nächste Schritt danach.
register-domains = Nur Adressen bei { $domains } können ein Konto eröffnen.
register-username-hint = 2 bis 64 Buchstaben, Ziffern, Punkte, Binde- oder Unterstriche.
register-submit = Konto anlegen
register-member-note = Es wird keine Adresse verlangt: Ihnen wird nichts geschickt, und niemand kann Ihre beanspruchen.
register-form-closed = Diese Website eröffnet mit diesem Formular keine Konten.
register-throttled = Zu viele Versuche von hier. Warten Sie eine Viertelstunde und versuchen Sie es erneut.
register-secret-wrong = Das ist nicht das Familiengeheimnis.
register-username-invalid = Ein Benutzername hat 2 bis 64 Buchstaben, Ziffern, Punkte, Binde- oder Unterstriche.
register-password-short = Das Passwort braucht mindestens { $min } Zeichen.
register-username-taken = Dieser Benutzername ist vergeben. Wählen Sie einen anderen.
register-not-saved = Das Konto konnte nicht gespeichert werden. Versuchen Sie es gleich noch einmal.
register-finish-title = Noch ein Schritt
register-finish-lede = Der Dienst hat bestätigt, wer Sie sind. Um hier ein Konto zu eröffnen, geben Sie das Familiengeheimnis ein.
register-finish-lede-email = Der Dienst hat Sie als { $email } bestätigt. Um hier ein Konto zu eröffnen, geben Sie das Familiengeheimnis ein.
sso-continue = Weiter
sso-continue-with = Weiter mit { $name }
sso-error-title = Die Anmeldung wurde nicht abgeschlossen
sso-account-disabled = Das mit dieser Anmeldung verknüpfte Konto ist deaktiviert. Wenden Sie sich an einen Administrator.
sso-no-account = Kein Konto hier passt zu dieser Anmeldung, und diese Website eröffnet keine Konten von selbst. Bitten Sie einen Administrator, Ihres mit dieser Adresse anzulegen: Ihre nächste Anmeldung wird damit verknüpft.
sso-domain-refused = Konten können hier nur mit einer vom Dienst bestätigten Adresse aus den Domains der Familie eröffnet werden.

# Administration console: the overview's tiles (v0.5.0, docs/adr/0011)
overview-accounts-note = { $admins ->
    [one] { $admins } Administrator
    *[other] { $admins } Administratoren
    }
register-password-hint = Mindestens { $min } Zeichen. Ein Satz aus mehreren Wörtern ist leichter zu merken als ein Zeichensalat.

# Clarity: the light theme and its funnel (v0.5.0, docs/adr/0013)
theme-clarity = Klarheit
theme-clarity-note = hell, Weiß und Orange · ein Trichter
view-funnel-back = Zurück zum Trichter
view-funnel-open = Den Trichter öffnen
view-funnel-hero-1 = Jede Generation ist ein Ring.
view-funnel-hero-2 = Jede Familie ein Trichter.
view-funnel-cta = Den Trichter erkunden
view-funnel-cta-detail = Jeder hinzugefügte Name und jede geprüfte Quelle machen den Trichter wahrer für die, die nach uns kommen.
view-funnel-see = Im Trichter ansehen
view-funnel-direction = ↓ die Ältesten am schmalen Ende · ↑ die Jüngsten an der Öffnung
view-funnel-still = Den Trichter anhalten
view-funnel-name = Trichter
view-funnel-legend = So liest man den Trichter
view-funnel-search-hint = Eingabe holt die Person nach vorn; Umschalt+Eingabe öffnet ihren Datensatz.
bundle-binding-unnamed = ein Archiv ohne Namen
bundle-binding-created = ein Archiv, angelegt am { $date }
bundle-binding-named = „{ $family }“
bundle-binding-named-created = „{ $family }“, angelegt am { $date }
