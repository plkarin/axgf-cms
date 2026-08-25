# axgf-cms — teksty interfejsu, polski.
#
# MASZYNOWA JAKOŚĆ — nieprzejrzane przez osobę, dla której polski jest językiem
# ojczystym. Słownictwo genealogiczne ("union", "affiliation", "confidence")
# ma ustalone odpowiedniki, które różnią się w zależności od tradycji
# archiwalnej, i te tłumaczenia mogą być błędne. Poprawki mile widziane —
# zob. CONTRIBUTING.md.
#
# Formy mnogie używają reguł CLDR: one / few / many. Nie należy ich zastępować
# angielską logiką "jeden albo więcej".
#
# ZASADA: ten plik tłumaczy wyłącznie interfejs. Nazwiska, miejsca, notatki
# i zawody pochodzą z pliku .axgf i pozostają w swoim własnym języku i piśmie.

app-name = axgf-cms

nav-tree = Drzewo
nav-admin = Administracja
nav-sign-in = Zaloguj się
nav-sign-out = Wyloguj się

prefs-title = Język i wygląd
prefs-language = Język
prefs-language-note = Zmienia to tylko interfejs. Nazwiska, miejsca i notatki są zawsze pokazywane w swoim języku i piśmie.
prefs-theme = Wygląd
prefs-apply = Zastosuj
prefs-reviewed = przejrzane
prefs-machine = maszynowe, { $coverage }%

theme-light = Jasny
theme-dark = Ciemny
theme-system = Zgodnie z systemem
theme-high-contrast = Wysoki kontrast
theme-sepia = Sepia
theme-deuteranopia = Deuteranopia
theme-protanopia = Protanopia
theme-tritanopia = Tritanopia
theme-colour-blind-note = bezpieczny dla daltonistów
theme-contrast-note = maksymalny kontrast

tree-title-around = Wokół osoby { $name }
tree-title-whole = Całe drzewo
tree-lede-focused = { $ancestors ->
        [one] Jeden przodek
        [few] { $ancestors } przodkowie
        [many] { $ancestors } przodków
       *[other] { $ancestors } przodków
    }, { $descendants ->
        [one] jeden potomek
        [few] { $descendants } potomkowie
        [many] { $descendants } potomków
       *[other] { $descendants } potomków
    } i { $spouses ->
        [one] jeden partner
        [few] { $spouses } partnerzy
        [many] { $spouses } partnerów
       *[other] { $spouses } partnerów
    }, { $depth } pokoleń w każdą stronę. Najstarsi na dole. Przezroczystość linii oznacza pewność relacji — blada linia to twierdzenie, którego zapis nie jest pewien.
tree-lede-whole = Wszystkie osoby w pliku. Najstarsi na dole, najmłodsi na górze. Przezroczystość linii oznacza pewność relacji.
tree-filter-label = Filtruj widoczne karty
tree-filter-placeholder = Wpisz nazwisko…
tree-centre-on = Wyśrodkuj na
tree-depth = Pokoleń w każdą stronę
tree-show = Pokaż
tree-hidden-notice = { $n ->
        [one] Jedna osoba jest pokazana bez szczegółów
        [few] { $n } osoby są pokazane bez szczegółów
        [many] { $n } osób jest pokazanych bez szczegółów
       *[other] { $n } osób jest pokazanych bez szczegółów
    }
tree-hidden-because-role = , ponieważ ich widoczność przekracza to, co może czytać Twoje konto.
tree-hidden-because-anonymous = , ponieważ nie są publiczne.
tree-hidden-sign-in = Zaloguj się, jeśli masz konto.
tree-restricted-card = Zapis tej osoby nie jest dla Ciebie widoczny
tree-empty = Ten plik nie zawiera nikogo do narysowania.
tree-unplaced = W żadnej zapisanej rodzinie

record-identity = Tożsamość
record-life-events = Wydarzenia życiowe
record-family = Rodzina
record-other-relationships = Inne relacje
record-occupations = Zawody
record-places = Miejsca
record-sources-documents = Źródła i dokumenty
record-notes = Notatki
record-history = Historia
record-raw = Surowy zapis
record-raw-summary-note = JSON, z którego zbudowano tę stronę
record-sources-documents-help = Każde źródło wymienia fakty na tej stronie, które się na nim opierają, uporządkowane według siły dowodu.
record-notes-help = Notatki do tego zapisu, w tym tekst, którego konwerter nie potrafił zinterpretować i który zachowano dosłownie, zamiast go usunąć.
record-help-toggle = Co pokazuje ta sekcja

record-gender = Płeć
record-living = Żyjąca
record-visibility = Widoczność
record-yes = tak
record-no = nie
record-name-type = Rodzaj nazwiska
record-name-used = Używane
record-name-evidence = Dowód
record-transliteration = Transliteracja łacińska
record-born = Urodzony(a)
record-died = Zmarły(a)
record-parents = Rodzice
record-siblings = Rodzeństwo
record-children = Dzieci
record-unknown-person = [Nieznany]
record-restricted-person = Prywatne
record-restricted-title = Zapis tej osoby nie jest dla Ciebie widoczny
record-absent-person-title = Wspomniany w tym pliku, ale w nim nieobecny
record-confidence = Pewność
record-source = Źródło
record-download = Pobierz

access-restricted-title = Niewidoczne dla Ciebie
access-restricted-anonymous = Ten zapis nie jest publiczny. Zaloguj się, aby sprawdzić, czy Twoje konto może go czytać.
access-role-title = Nie dla Twojej roli
access-role-write = Twoje konto może czytać ten plik, ale nie może go zmieniać. Administrator może podnieść Twoją rolę do współtwórcy.
access-scope-title = Poza Twoją gałęzią

error-not-found-title = Nie znaleziono
error-not-found-detail = Ta strona nie istnieje w tym pliku.
error-no-such-person-title = Nie ma takiej osoby
error-no-such-person-detail = Ten plik nie zawiera osoby o tym identyfikatorze.
error-no-such-entity-title = Nie ma takiego obiektu
error-no-such-entity-detail = Ten plik nie zawiera obiektu o tym identyfikatorze.
error-deleted-while-editing = Ten plik nie zawiera obiektu o tym identyfikatorze. Mógł zostać usunięty podczas edycji.
error-no-such-file-title = Nie ma takiego pliku
error-not-an-image-title = To nie jest obraz
error-not-an-image-detail = Nie ma miniatury tego dokumentu, ponieważ nie jest to obraz, który ta wersja potrafi odczytać.
error-back = Wstecz

login-title = Logowanie
login-lede = Konta zakłada administrator.
login-username = Nazwa użytkownika
login-password = Hasło
login-submit = Zaloguj się
login-wrong = Ta nazwa użytkownika i hasło nie pasują do siebie.
login-token-wrong = Ten token jest nieprawidłowy.
login-throttled = Zbyt wiele nieudanych prób. Odczekaj kilka minut i spróbuj ponownie.
login-no-accounts-title = Ta instalacja nie ma jeszcze żadnych kont.
login-emergency-summary = Dostęp awaryjny
login-emergency-label = Token awaryjny
login-emergency-submit = Użyj tokenu awaryjnego
login-sign-in-prompt = Zaloguj się, aby wejść do panelu administracyjnego.

admin-title = Administracja
admin-entities = Obiekty
admin-create = Utwórz
admin-new-kind = Nowy: { $kind }
admin-operations = Operacje
admin-validate = Sprawdź
admin-deduplicate = Usuń duplikaty
admin-export = Wyeksportuj plik
admin-accounts = Konta
admin-dedup-confirm = Usuwanie duplikatów łączy obiekty i przepisuje plik. Kontynuować?
admin-recent-changes = Ostatnie zmiany
admin-sessions-open = { $n ->
        [one] Otwarta jedna sesja.
        [few] Otwarte { $n } sesje.
        [many] Otwartych { $n } sesji.
       *[other] Otwartych { $n } sesji.
    }
admin-no-changes-yet = Nic jeszcze nie zostało zmienione przez tę aplikację. Każdy zapis od teraz jest odnotowywany w { $path }.
admin-last-validation = Ostatnie sprawdzenie
admin-fields = Pola
admin-raw-json = Surowy JSON
admin-save = Zapisz
admin-cancel = Anuluj
admin-delete = Usuń
admin-not-set = — nie podano —
admin-edit = Edytuj
admin-page-of = Strona { $page } z { $pages }
admin-previous = Poprzednia
admin-next = Następna
admin-saved = Zapisano jako wersja { $version } — { $summary }
admin-not-saved = Nie zapisano
admin-created = Utworzono
admin-not-created = Nie utworzono
admin-deleted = Usunięto
admin-not-deleted = Nie usunięto — plik jest niezmieniony
admin-what-changed = co się zmieniło
admin-field = Pole
admin-from = Z
admin-to = Na
admin-version = wersja { $version }

accounts-title = Konta
accounts-existing = Istniejące
accounts-username = Nazwa użytkownika
accounts-role = Rola
accounts-status = Stan
accounts-branch = Gałąź
accounts-last-seen = Ostatnio widziano
accounts-change = Zmień
accounts-you = (Ty)
accounts-active = aktywne
accounts-disabled = wyłączone
accounts-never = nigdy
accounts-whole-tree = całe drzewo
accounts-roots = { $n ->
        [one] jeden korzeń
        [few] { $n } korzenie
        [many] { $n } korzeni
       *[other] { $n } korzeni
    }
accounts-add = Dodaj konto
accounts-password-hint = Zostaw puste, a hasło zostanie wygenerowane i pokazane raz. Co najmniej { $min } znaków, jeśli ustawiasz je samodzielnie.
accounts-new-password-placeholder = nowe hasło (puste = bez zmian)
accounts-email = E-mail
accounts-optional = (opcjonalnie)
accounts-create = Utwórz konto
accounts-role-viewer = czytelnik — czyta zapisy publiczne i dla członków
accounts-role-contributor = współtwórca — dodatkowo tworzy, edytuje i wysyła pliki
accounts-role-admin = administrator — dodatkowo zarządza kontami, usuwa i eksportuje
accounts-branch-placeholder = jeden identyfikator osoby w wierszu
accounts-ids-in-bundle = Identyfikatory osób w tym pliku
accounts-created = Utworzono { $username }.
accounts-updated = Zaktualizowano { $username }. Każda otwarta sesja została wylogowana.
accounts-username-taken = Ta nazwa użytkownika jest zajęta.
accounts-pick-role = Wybierz rolę.
accounts-no-such = Nie ma takiego konta.
accounts-not-saved = Nie zapisano: { $error }

conflict-title = Ktoś inny zmienił to przed Tobą
conflict-versions = Zacząłeś od wersji { $expected }; plik zawiera teraz wersję { $current }.
conflict-both-changed = Oboje zmieniliście te pola
conflict-both-changed-detail = Te pola zostały zmienione przez was oboje. To, co zapiszesz, zastąpi to, co wpisał(a) { $who }:
conflict-field-by-field = Pole po polu
conflict-theirs = Na co zmienił(a) to { $who }
conflict-yours = Na co Ty to zmieniłeś(-aś)
conflict-unchanged-by-you = niezmienione przez Ciebie
conflict-unchanged-by-them = niezmienione przez nich
conflict-what-now = Co teraz
conflict-reapply = Zastosuj swoją wersję na ich
conflict-save-over = Zapisz to zamiast ich wersji
conflict-discard = Odrzuć moją i zacznij od nowa
conflict-their-version = Wersja { $who }, tak jak plik ją teraz zawiera
conflict-history-of = Historia tego obiektu ({ $kind })

## Dates

date-unknown = Data nieznana
date-not-recorded = Nie odnotowano
date-circa = około { $date }
date-between = między { $from } a { $to }
date-before = przed { $date }
date-after = po { $date }
date-preserved = zapisano jako „{ $text }”
date-day-month-year = { $day } { $month } { $year }
date-month-year = { $month } { $year }
date-decade = lata { $decade }.
date-century = { $century }. wiek

month-1 = styczeń
month-2 = luty
month-3 = marzec
month-4 = kwiecień
month-5 = maj
month-6 = czerwiec
month-7 = lipiec
month-8 = sierpień
month-9 = wrzesień
month-10 = październik
month-11 = listopad
month-12 = grudzień
