# axgf-cms — teksty interfejsu, polski.
#
# JAKOŚĆ MASZYNOWA — nieprzejrzane przez osobę, dla której polski jest językiem
# ojczystym. Słownictwo genealogiczne ma ustalone odpowiedniki zależne od
# tradycji archiwalnej i te tłumaczenia mogą być błędne. Poprawki mile widziane
# — zob. CONTRIBUTING.md.
#
# Przyjęte odpowiedniki (do zakwestionowania przez native speakera):
#   union → związek · link → powiązanie · confidence → pewność
#   reliability → wiarygodność · source → źródło · occupation → zawód
#   record → wpis · archive → archiwum · godparent → rodzic chrzestny
#   witness → świadek · speculative → przypuszczalny
#
# Liczba mnoga: reguły CLDR one / few / many / other. Nigdy nie zastępować ich
# angielską logiką „jeden albo więcej”.
#
# Daty: nazwa miesiąca w pełnej dacie stoi w dopełniaczu — „12 kwietnia 1923”,
# nie „12 kwiecień 1923”. Stąd dwa zestawy: month-N i month-in-date-N.
#
# ZASADA: ten plik tłumaczy wyłącznie interfejs. Nazwiska, miejsca, notatki
# i zawody pochodzą z archiwum i pozostają w swoim języku i piśmie.

app-name = ax-genealogy

## Nagłówek i stopka

nav-tree = Drzewo
nav-convert = Import
nav-admin = Administracja
nav-sign-in = Zaloguj się
nav-sign-out = Wyloguj się
footer-open-format = Archiwum Twojej rodziny to jeden plik, który zostaje u Ciebie, zapisany w otwartym formacie — otworzysz go długo po tym, jak ta strona zniknie.
footer-open-format-link = O formacie

## Ustawienia

settings-title = Ustawienia
settings-tabs-label = Sekcje ustawień
settings-tab-theme = Motyw
settings-tab-language = Język
settings-tab-appearance = Wygląd
settings-done = Gotowe
prefs-language = Język
prefs-theme = Motyw
prefs-background = Tło
prefs-background-on = Delikatna poświata koloru za stroną
prefs-apply = Zastosuj
prefs-reviewed = przejrzane
prefs-machine = maszynowe, { $coverage }%
prefs-machine-complete = kompletne, jeszcze nieprzejrzane
prefs-machine-title = Przetłumaczone bez weryfikacji przez osobę, dla której to język ojczysty. Zwłaszcza słownictwo genealogiczne może być błędne — słowa oznaczające związek, rodzica chrzestnego czy źródło pierwotne różnią się w zależności od tradycji archiwalnej danego kraju. Poprawki są mile widziane, a CONTRIBUTING.md mówi, od czego zacząć.

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

## Drzewo

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
    }, po { $depth } pokoleń w każdą stronę.
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
tree-restricted-card = Wpis tej osoby nie jest dla Ciebie widoczny
tree-empty = Nie ma jeszcze kogo narysować.
tree-unplaced = W żadnej zapisanej rodzinie

## Wpis osoby

record-identity = Tożsamość
record-life-events = Wydarzenia z życia
record-family = Rodzina
record-other-relationships = Inne relacje
record-occupations = Zawody
record-places = Miejsca
record-sources-documents = Źródła i dokumenty
record-notes = Notatki
record-history = Historia zmian
record-raw = Dane surowe
record-raw-summary-note = JSON, z którego zbudowano tę stronę

record-identity-help = Każde zapisane imię lub nazwisko wraz z jego typem, okresem używania i źródłem, z pismem oryginalnym obok transliteracji łacińskiej tam, gdzie się różnią, a także płeć, status żyjącej osoby i widoczność.
record-life-events-help = Narodziny, śmierć i każde wydarzenie, w którym ta osoba brała udział, w porządku dat, każde z jej rolą — dzięki temu ślub, którego była jedynie świadkiem, stoi obok jej własnego. Fakt bez daty trafia na koniec, zamiast udawać, że był pierwszy.
record-family-help = Rodzice i rodzeństwo, a następnie każdy związek wraz z typem, datami, miejscem, sposobem zakończenia i dziećmi w kolejności urodzenia.
record-other-relationships-help = Każde powiązanie, którego ta osoba jest jednym z końców, odczytane z jej strony — ten sam zapis widnieje jako „ojciec chrzestny” z jednej strony i „chrześniak” z drugiej.
record-occupations-help = Zawody jako okresy na jednej wspólnej osi, aby dwie posady dało się porównać wzrokiem; pasek jest otwarty tam, gdzie brakuje granicy.
record-places-help = Każde miejsce, którego dotyczy ten wpis, wraz z tym, co się w nim wydarzyło, oraz z historią granic, która nadaje miejscu sens w czasie.
record-sources-documents-help = Każde źródło wymienia fakty na tej stronie, które się na nim opierają, uporządkowane według siły dowodu.
record-notes-help = Notatki do tego wpisu, w tym tekst, którego konwerter nie potrafił rozczytać i zachował dosłownie, zamiast go usunąć.
record-history-help = Każda zapisana zmiana tego wpisu, od najnowszej. Kto co poprawił, to fakt o osobach prowadzących drzewo, a nie o rodzinie w nim zapisanej — dlatego pozostaje poza eksportowanym archiwum i jest widoczny tylko dla zalogowanych krewnych.
record-raw-help = Nic tutaj nie powstaje na potrzeby wyświetlania: to wpis dokładnie taki, jaki jest przechowywany, aż po nazwy pól. Gdybyś kiedyś musiał odczytać archiwum bez tej strony, zobaczyłbyś właśnie to.
record-help-toggle = Co pokazuje ta sekcja

record-gender = Płeć
record-living = Żyje
record-visibility = Widoczność
record-yes = tak
record-no = nie
record-name-type = Typ nazwy
record-name-used = Używane
record-name-evidence = Podstawa
record-transliteration = Transliteracja łacińska
record-born = Urodzony(a)
record-died = Zmarły(a)
record-parents = Rodzice
record-siblings = Rodzeństwo
record-children = Dzieci
record-unknown-person = [Nieznany]
record-restricted-person = Prywatne
record-restricted-title = Wpis tej osoby nie jest dla Ciebie widoczny
record-absent-person-title = Wymieniony w tym drzewie, ale bez własnego wpisu
record-confidence = Pewność
record-source = Źródło
record-download = Pobierz

## Dostęp

access-restricted-title = Niewidoczne dla Ciebie
access-restricted-signed-in = Widoczność tego wpisu przekracza to, co może czytać Twoje konto. Administrator może zmienić albo widoczność wpisu, albo Twoją rolę.
access-restricted-anonymous = Ten wpis nie jest publiczny. Zaloguj się, aby sprawdzić, czy Twoje konto może go czytać.
access-role-title = Nie dla Twojej roli
access-role-admin = To strona administratora. Twoje konto może tworzyć i edytować wpisy, ale nie może zarządzać kontami, usuwać wpisów ani eksportować archiwum.
access-role-write = Twoje konto może czytać to drzewo, ale nie może go zmieniać. Administrator może podnieść Twoją rolę do współtwórcy.
access-scope-title = Poza Twoją gałęzią
access-scope-named = Twoje konto jest ograniczone do jednej gałęzi drzewa, a ten wpis dotyczy kogoś spoza niej. Każda osoba wymieniona we wpisie musi należeć do Twojej gałęzi — inaczej rodzina z jednym partnerem z zewnątrz byłaby sposobem na przepisanie pochodzenia tej osoby.
access-scope-unnamed = Twoje konto jest ograniczone do jednej gałęzi drzewa, a ten wpis nie wymienia nikogo, względem kogo można by go zmierzyć. Źródła i miejsca edytują konta mające dostęp do całego drzewa.

## Błędy

error-not-found-title = Nie znaleziono
error-not-found-detail = Taka strona tu nie istnieje.
error-no-such-person-title = Nie ma takiej osoby
error-no-such-person-detail = Nie ma tutaj osoby o tym identyfikatorze.
error-no-such-entity-title = Nie ma takiego obiektu
error-no-such-entity-detail = Nie ma tutaj wpisu o tym identyfikatorze.
error-deleted-while-editing = Nie ma tutaj wpisu o tym identyfikatorze. Mógł zostać usunięty, kiedy go edytowałeś.
error-no-such-file-title = Nie ma takiego pliku
error-no-such-file-detail = Nie ma tutaj dokumentu o tym identyfikatorze albo dokument zapisano bez pliku — dokument przywoływany wskazuje coś przechowywanego gdzie indziej.
error-not-an-image-title = To nie jest obraz
error-not-an-image-detail = Dla tego dokumentu nie ma miniatury, ponieważ nie jest to obraz, który ta wersja potrafi odczytać.
error-back = Wróć

## Logowanie

login-title = Zaloguj się
login-lede = Konta zakłada administrator.
login-username = Nazwa użytkownika
login-password = Hasło
login-submit = Zaloguj się
login-wrong = Ta nazwa użytkownika i hasło nie pasują do siebie.
login-token-wrong = Ten token jest nieprawidłowy.
login-throttled = Zbyt wiele nieudanych prób. Odczekaj kilka minut i spróbuj ponownie.
login-no-accounts-title = Ta instalacja nie ma jeszcze żadnych kont.
login-no-accounts-detail = Celowo nie ma tu strony konfiguracji — okno między wdrożeniem a pierwszym logowaniem to dokładnie ten moment, w którym instalacja jest bezbronna, więc pierwszego administratora tworzy się z wiersza poleceń.
login-no-accounts-note = Wypisuje wygenerowane hasło na stderr raz i nigdy więcej. Do tego czasu jedynym wejściem jest awaryjny token poniżej.
login-emergency-summary = Dostęp awaryjny
login-emergency-detail = Wspólny token nadal otwiera sesję administratora i istnieje w jednym celu: żeby wrócić do środka, gdy plik .acl został utracony albo wszyscy administratorzy są zablokowani. To nie jest konto — nie ma własnych ustawień, a dziennik zmian zapisuje je jako emergency-token, a nie jako osobę. Jego użycie jest logowane jako ostrzeżenie.
login-emergency-label = Token awaryjny
login-emergency-submit = Użyj tokenu awaryjnego
login-sign-in-prompt = Zaloguj się, aby wejść do panelu administracyjnego.

## Administracja

admin-title = Administracja
admin-lede = Edytowanie { $path } — { $total } obiektów, { $files ->
        [one] jeden załączony plik
        [few] { $files } załączone pliki
        [many] { $files } załączonych plików
       *[other] { $files } załączonych plików
    }, { $size } na dysku. Każda zmiana zapisywana jest atomowo; odrzucona zmiana zostawia plik nietknięty.
admin-entities = Obiekty
admin-create = Utwórz
admin-new-kind = Nowy: { $kind }
admin-operations = Operacje
admin-validate = Sprawdź poprawność
admin-deduplicate = Usuń duplikaty
admin-export = Eksportuj archiwum
admin-accounts = Konta
admin-roles-note = Sprawdzanie poprawności, usuwanie duplikatów, eksport, usuwanie i zarządzanie kontami są wyłącznie dla administratora. Współtwórca dociera do każdej innej strony tutaj.
admin-dedup-confirm = Usuwanie duplikatów scala wpisy i przepisuje archiwum. Kontynuować?
admin-recent-changes = Ostatnie zmiany
admin-recent-note = Ostatnie { $shown } z { $total ->
        [one] jednej zapisanej zmiany
        [few] { $total } zapisanych zmian
        [many] { $total } zapisanych zmian
       *[other] { $total } zapisanych zmian
    }, z { $path }.
admin-sessions-open = { $n ->
        [one] Otwarta jedna sesja.
        [few] Otwarte { $n } sesje.
        [many] Otwartych { $n } sesji.
       *[other] Otwartych { $n } sesji.
    }
admin-no-changes-yet = Przez tę aplikację nic jeszcze nie zostało zmienione. Każdy kolejny zapis zostanie odnotowany w { $path }.
admin-last-validation = Ostatnie sprawdzenie
admin-bundle-heavy = To archiwum waży { $size }. Całość jest wczytywana przy starcie i trzymana w pamięci, więc powyżej mniej więcej { $warn } strona zaczyna kosztować realną pamięć, a restarty stają się wolne. To rozwiązanie pasuje do archiwum rodzinnego, nie do biblioteki multimediów — jeśli załączniki rosną bez ograniczeń, trzymaj je w magazynie plików, a archiwum niech na nie wskazuje.

admin-fields = Pola
admin-raw-json = Surowy JSON
admin-raw-json-help = Cały obiekt, więc nic nie jest nieedytowalne — listy takie jak partnerzy i dzieci rodziny albo historia granic miejsca żyją właśnie tu. To dokument wyjściowy; pola powyżej są następnie zapisywane po ścieżkach, które posiadają, więc edytuj wartość albo w jednym miejscu, albo w drugim, nie w obu. Musi się parsować jako JSON, inaczej nic nie zostanie zapisane.
admin-save = Zapisz
admin-cancel = Anuluj
place-editor-title = Edytuj miejsce
place-add-detail = Uzupełnij to miejsce
place-names = Nazwy
place-name-primary = Główna
place-name-lang = Język
place-name-value = Nazwa
place-names-hint = Jeden wiersz na zapisaną nazwę. Miejsce administrowane przez trzy imperia nosi trzy nazwy; główna jest pokazywana wszędzie indziej.
place-where = Położenie
place-type = Rodzaj
place-region = Region
place-country-current = Kraj dzisiaj
place-country-hint = ISO 3166-1 alfa-2, np. PL, FR, DE.
place-country-history = Historia granic
place-history-country = Państwo
place-history-from = Od
place-history-until = Do
place-country-history-hint = Które państwo władało tym miejscem i w jakim okresie. Ma to znaczenie genealogiczne: akt spisany po rosyjsku w 1880 i po polsku w 1930 może nazywać tę samą wieś.
place-coordinates = Współrzędne
place-lat = Szerokość
place-lon = Długość
place-precision = Dokładność
place-identifiers = Identyfikatory
place-wikidata = Wikidata
place-geonames = GeoNames
place-used-by = { $name } jest używane przez { $n ->
        [one] jeden inny zapis
        [few] { $n } inne zapisy
        [many] { $n } innych zapisów
       *[other] { $n } innych zapisów
    }.
place-error-no-name = Miejsce musi mieć co najmniej jedną nazwę.
place-error-coords-pair = Szerokość i długość idą w parze: albo obie, albo żadna.
place-error-coords-number = Szerokość i długość muszą być liczbami.
place-error-coords-range = Szerokość od -90 do 90, długość od -180 do 180.
place-type-continent = kontynent
place-type-country = kraj
place-type-region = region
place-type-department = departament
place-type-city = miasto
place-type-village = wieś
place-type-district = dzielnica
place-type-street = ulica
place-type-building = budynek
place-type-farm = gospodarstwo
place-type-island = wyspa
place-type-historical = historyczne
place-type-unknown = nieznane
place-precision-exact = dokładna
place-precision-building = budynek
place-precision-street = ulica
place-precision-city_center = centrum miasta
place-precision-region_center = centrum regionu
place-precision-country_center = centrum kraju
place-precision-approximate = przybliżona

place-coordinates-hint = Zwykle wpisuje się je ręcznie. Wielu miejsc zapisanych pod dawną administracją współczesne wyszukiwanie w ogóle nie znajdzie.
place-geocode-search = Wyszukaj tę nazwę
place-geocode-hint = Wysyła nazwę, region i kraj do usługi geokodowania, po jednym miejscu naraz. Nic nie zostaje zapisane, dopóki nie zapiszesz.
place-geocode-off = Wyszukiwanie nazw jest wyłączone. Wymaga adresu kontaktowego, po którym usługa rozpozna tę instalację; uruchom serwer z --geocoder-contact, aby je włączyć.
place-geocode-query = Szukano: { $q }
place-geocode-error = Nie udało się połączyć z usługą wyszukiwania. Pola współrzędnych powyżej nadal działają.
place-geocode-none = Nic nie znaleziono. Dla wsi zapisanej pod administracją rosyjską, pruską lub austriacką jest to wynik zwyczajny; wpisz położenie ręcznie.
place-geocode-not-a-place = to nie miejscowość
place-geocode-use = Użyj tego
place-geocode-attribution = Wyniki z OpenStreetMap przez Nominatim, na licencji Open Database.

place-paste = Wklej położenie
place-paste-placeholder = odnośnik do mapy albo 52.0782795, 21.2508068
place-paste-read = Odczytaj
place-paste-hint = Odnośnik Google Maps lub OpenStreetMap, adres geo:, zwykła para liczb albo stopnie-minuty-sekundy, np. 52°04'41.8"N 21°15'02.9"E.
place-paste-read-ok = Odczytano do pól powyżej. Sprawdź i zapisz.
place-paste-unreadable = Tego położenia nie udało się odczytać. Pola powyżej nadal przyjmują zwykłą parę liczb.

place-map-hint = Kliknij mapę, aby postawić punkt, albo przeciągnij pinezkę. Zapisem są pola powyżej.
place-map-clear = Usuń punkt
place-open-in-map = Znajdź to miejsce w OpenStreetMap i wklej odnośnik z powrotem

person-tab-record = Zapis
person-tab-life = Życie
person-tab-media = Materiały
person-tab-tree = Drzewo
person-tab-history = Historia
person-tree-depth = Po { $n } pokolenia w każdą stronę. Całe drzewo jest niżej.
person-tree-alone = Ten zapis nie wymienia rodziców, małżonków ani dzieci, więc nie ma wokół czego rysować kształtu.

record-no-evidence = Do tego zapisu nic nie dołączono — ani źródła, ani dokumentu. To zwykły stan pliku po konwersji, a nie jego wada: GEDCOM przenosi fakty, a zostawia to, co je potwierdzało.
record-no-evidence-signed-out = Zaloguj się, aby coś dołączyć.
admin-delete = Usuń
admin-not-set = — nie ustawiono —
admin-edit = Edytuj
admin-page-of = Strona { $page } z { $pages }
admin-previous = Poprzednia
admin-next = Następna
admin-saved = Zapisano jako wersja { $version } — { $summary }
admin-not-saved = Nie zapisano
admin-created = Utworzono
admin-not-created = Nie utworzono
admin-deleted = Usunięto
admin-not-deleted = Nie usunięto — nic nie zostało zmienione
admin-what-changed = co się zmieniło
admin-field = Pole
admin-from = Z
admin-to = Na
admin-version = wersja { $version }

## Konta

accounts-title = Konta
accounts-lede = Przechowywane w { $path }, z prawami 600, obok archiwum i nigdy w środku. Archiwum się kopiuje, wysyła i publikuje; skróty haseł podróżujące w jego wnętrzu zamieniłyby każdą kopię drzewa rodzinnego w kopię danych logowania.
accounts-existing = Istniejące
accounts-username = Nazwa użytkownika
accounts-role = Rola
accounts-status = Status
accounts-branch = Gałąź
accounts-last-seen = Ostatnio widziany
accounts-change = Zmień
accounts-you = (to Ty)
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
accounts-no-registration = Celowo nie ma tu samodzielnej rejestracji ani zaproszeń. Dla archiwum rodzinnego wystarczy administrator, który zna wszystkich, a to całkowicie usuwa pole do nadużyć, zamiast go bronić.
accounts-password-hint = Zostaw puste, a hasło zostanie wygenerowane i pokazane raz. Co najmniej { $min } znaków, jeśli ustawiasz je sam.
accounts-new-password-placeholder = nowe hasło (puste = bez zmiany)
accounts-email = E-mail
accounts-optional = (opcjonalnie)
accounts-create = Utwórz konto
accounts-role-viewer = czytelnik — czyta wpisy publiczne i rodzinne
accounts-role-contributor = współtwórca — także tworzy, edytuje i wysyła pliki
accounts-role-admin = administrator — także zarządza kontami, usuwa i eksportuje
accounts-branch-hint = Ogranicza to, co konto może edytować, do tych osób, ich potomków i małżonków.
accounts-branch-reading = Nie ogranicza tego, co mogą czytać — o tym decyduje widoczność każdego wpisu, a te dwie rzeczy są celowo rozdzielone.
accounts-branch-placeholder = jeden identyfikator osoby w wierszu
accounts-ids-in-bundle = Identyfikatory osób w tym drzewie
accounts-emergency-warning = Jesteś zalogowany tokenem awaryjnym. Daje on prawa administratora na tę sesję, ale nie jest kontem: nie ma własnych ustawień, a dziennik zmian zapisze Twoje zmiany jako emergency-token, a nie jako osobę. Załóż sobie poniżej prawdziwe konto i zaloguj się na nie.
accounts-created-with-password = Utworzono { $username }. Hasło to { $password } — pokazywane jest raz i przechowywane wyłącznie jako skrót Argon2id, więc przekaż je teraz.
accounts-created = Utworzono { $username }.
accounts-updated = Zaktualizowano { $username }. Każda otwarta sesja tego konta została wylogowana.
accounts-username-taken = Ta nazwa użytkownika jest zajęta.
accounts-pick-role = Wybierz rolę.
accounts-no-such = Nie ma takiego konta.
accounts-last-admin = To jedyny aktywny administrator. Najpierw awansuj kogoś innego — instalację bez administratora da się odzyskać wyłącznie edytując plik .acl albo używając tokenu awaryjnego.
accounts-not-saved = Nie zapisano: { $error }

## Konflikty

conflict-title = Ktoś inny zmienił to pierwszy
conflict-lede = { $who } zapisał(a) zmianę w tym obiekcie ({ $kind }) o { $when }, po tym jak go otworzyłeś. Twoja zmiana nie została zapisana i nic nie zostało nadpisane.
conflict-no-merge = Nic nie jest tu scalane automatycznie. Scalenie zmian dwóch osób daje wpis, którego żadna z nich nie wybrała, a w genealogii spór dwóch redaktorów o datę zwykle znaczy, że czytają różne źródła — a to pytanie do człowieka, nie do programu. Porównaj oba poniżej i zdecyduj.
conflict-versions = Zacząłeś od wersji { $expected }; wpis ma teraz wersję { $current }.
conflict-both-changed = Oboje zmieniliście to samo
conflict-both-changed-detail = Te pola zostały zmienione przez was oboje. Cokolwiek zapiszesz, zastąpi to, co wpisał(a) { $who }:
conflict-different-fields = Zmieniliście różne pola, więc nic z pracy { $who } nie jest sporne — ale ponowne zastosowanie i tak zapisze cały Twój obiekt na ich obiekcie. Sprawdź obie kolumny przed zapisem.
conflict-field-by-field = Pole po polu
conflict-theirs = Na co zmienił(a) to { $who }
conflict-yours = Na co Ty to zmieniłeś
conflict-unchanged-by-you = niezmienione przez Ciebie
conflict-unchanged-by-them = niezmienione przez nich
conflict-nothing-differs = Żadna z wersji nie różni się od tej, od której zacząłeś, w żadnym polu pokazywanym na tej stronie. Numer wersji się zmienił, więc ktoś zapisał wpis, nie zmieniając niczego, co ten wpis zawiera.
conflict-what-now = Co teraz
conflict-reapply = Zastosuj swoją wersję na ich wersji
conflict-reapply-hint = To Twoja zmiana, przeniesiona na wersję { $version }. Popraw ją tutaj, aby zachować z pracy { $who } to, co chcesz, a potem zapisz. Ich wersja jest pokazana poniżej do skopiowania.
conflict-save-over = Zapisz to na ich wersji
conflict-discard = Odrzuć moją i zacznij od nowa
conflict-their-version = Wersja { $who }, w obecnej postaci
conflict-history-of = Historia tego obiektu ({ $kind })

## Import

convert-title = Importuj plik rodzinny
convert-submit = Importuj
convert-result-title = Raport z importu
convert-download = Pobierz archiwum

## Daty
#
# Słowa daty są tekstem interfejsu; jej WARTOŚĆ i PRECYZJA są danymi i nigdy
# nie są ruszane. Data zapisana z dokładnością do roku pozostaje roczna.

date-unknown = Data nieznana
date-not-recorded = Niezapisana
date-circa = około { $date }
date-between = między { $from } a { $to }
date-before = przed { $date }
date-after = po { $date }
date-preserved = zapisano jako „{ $text }”
date-day-month-year = { $day } { $month ->
        [1] stycznia
        [2] lutego
        [3] marca
        [4] kwietnia
        [5] maja
        [6] czerwca
        [7] lipca
        [8] sierpnia
        [9] września
        [10] października
        [11] listopada
        [12] grudnia
        *[other] { $month }
    } { $year }
date-month-year = { $month ->
        [1] styczeń
        [2] luty
        [3] marzec
        [4] kwiecień
        [5] maj
        [6] czerwiec
        [7] lipiec
        [8] sierpień
        [9] wrzesień
        [10] październik
        [11] listopad
        [12] grudzień
        *[other] { $month }
    } { $year }
date-decade = lata { $decade }.
date-century = wiek { $century ->
        [1] I
        [2] II
        [3] III
        [4] IV
        [5] V
        [6] VI
        [7] VII
        [8] VIII
        [9] IX
        [10] X
        [11] XI
        [12] XII
        [13] XIII
        [14] XIV
        [15] XV
        [16] XVI
        [17] XVII
        [18] XVIII
        [19] XIX
        [20] XX
        [21] XXI
        *[other] { $century }.
    }
date-quarter-century = { $quarter ->
        [1] pierwsza
        [2] druga
        [3] trzecia
       *[other] czwarta
    } ćwierć wieku { $century ->
        [1] I
        [2] II
        [3] III
        [4] IV
        [5] V
        [6] VI
        [7] VII
        [8] VIII
        [9] IX
        [10] X
        [11] XI
        [12] XII
        [13] XIII
        [14] XIV
        [15] XV
        [16] XVI
        [17] XVII
        [18] XVIII
        [19] XIX
        [20] XX
        [21] XXI
        *[other] { $century }.
    }

## Dalsze strony błędów

error-back-to-start = Wróć na początek
error-payload-missing-title = Nie ma takiego pliku
error-payload-missing-detail = Zawartości tego dokumentu nie ma w pamięci podręcznej.
error-payload-unopenable-detail = Nie udało się otworzyć zawartości tego dokumentu.
error-no-such-document-detail = Nie ma tutaj dokumentu o tym identyfikatorze.
error-bad-preference-title = To nie jest jedna z możliwości
error-bad-preference-detail = To nie jest język ani motyw oferowany przez tę stronę. Nic nie zostało zmienione.
error-unknown-kind-title = Nieznany rodzaj
error-unknown-kind-detail = „{ $kind }” nie jest rodzajem wpisu. To archiwum zawiera: { $kinds }.
error-io-title = Nie udało się zapisać
error-io-detail = { $error }. Nic na dysku nie zostało zmienione.
error-upload-too-large = Ten plik jest większy niż limit { $mb } MB. Nic nie zostało zapisane, a archiwum jest nietknięte.
error-upload-refused = Dokument został odrzucony: { $reason }. Archiwum jest nietknięte.
error-back-to-person = Wróć do wpisu
error-no-such-person-to-attach = Nie ma tutaj osoby o tym identyfikatorze, więc nie ma do czego dołączyć dokumentu.
error-upload-title = Ten plik nie został zapisany
error-download-expired-title = Ten plik do pobrania wygasł
error-download-expired-detail = Import jest przechowywany przez piętnaście minut, a potem usuwany. Zaimportuj plik ponownie.
error-upload-none = Nie wysłano żadnego pliku. Najpierw wybierz plik.
error-upload-unsupported = To nie jest typ pliku, który archiwum przechowuje. Przyjmowane są obrazy, PDF, zwykły tekst, dźwięk i wideo; typ odczytywany jest z samych bajtów pliku, więc zmiana nazwy programu wykonywalnego nic nie da. SVG jest odrzucany wprost, ponieważ SVG może nieść skrypt.
error-export-unreadable-title = Nie udało się odczytać wyeksportowanego archiwum
error-export-unreadable-detail = { $error }

## Strona drzewa, ciąg dalszy

tree-title-suffix = drzewo
tree-back-to-focused = Wróć do widoku skupionego
tree-show-all = Pokaż wszystkie: { $n }
tree-width-notice = Ten widok ma { $width } pikseli szerokości — na ekranie 1500 pikseli to { $screens ->
        [one] jeden ekran
        [few] { $screens } ekrany
        [many] { $screens } ekranów
       *[other] { $screens } ekranów
    } przewijania w poziomie.
tree-confidence-label = Pewność:
tree-band-certain = pewne
tree-band-high = wysoka
tree-band-medium = średnia
tree-band-low = przypuszczalne
tree-counts = { $drawn } z { $total } osób · { $generations ->
        [one] jedno pokolenie
        [few] { $generations } pokolenia
        [many] { $generations } pokoleń
       *[other] { $generations } pokoleń
    }
tree-unplaced-count = { $n } bez miejsca
tree-contradicts-title = To drzewo samo sobie przeczy.
tree-contradicts-detail = Żaden układ rzędów tego nie spełni, więc poniższa relacja została pominięta w numerowaniu pokoleń i niektóre rzędy mogą być narysowane w złym miejscu. Popraw ten z dwóch wpisów, który jest błędny.
tree-contradicts-pair = Zapisani zarazem jako para i jako rodzic z dzieckiem:
tree-contradicts-more = { $n ->
        [one] Jedna dalsza sprzeczność nie została wymieniona.
        [few] { $n } dalsze sprzeczności nie zostały wymienione.
        [many] { $n } dalszych sprzeczności nie zostało wymienionych.
       *[other] { $n } dalszych sprzeczności nie zostało wymienionych.
    }
tree-no-people = W tym drzewie nie ma jeszcze nikogo.
tree-no-people-cta = Zaimportuj plik rodzinny albo dodaj pierwszą osobę.
tree-nobody-selected = Dla tego wyboru nie ma kogo narysować.
tree-nobody-selected-cta = Zacznij od widoku domyślnego.
tree-edge-union = Zapisany związek
tree-edge-parentage = Zapisane pochodzenie

## Strona główna

home-empty = Nic jeszcze nie zapisano. Zaimportuj plik rodzinny, aby przenieść istniejące drzewo, albo dodaj pierwszą osobę ręcznie.
home-count = { $total ->
        [one] Jeden wpis
        [few] { $total } wpisy
        [many] { $total } wpisów
       *[other] { $total } wpisów
    }, w jednym pliku należącym do rodziny.
home-browse = Przeglądaj drzewo
home-convert = Zaimportuj plik rodzinny
home-unnamed-family = To drzewo rodzinne
home-in-this-tree = Co rodzina zapisała do tej pory
home-showcase-title = Tam, gdzie to drzewo mówi już więcej niż imiona i daty
home-showcase-example = Zobacz przykład →
home-nothing-title = Nie ma jeszcze czego pokazać.
home-nothing-detail = Zaimportuj plik rodzinny, aby przenieść istniejące drzewo, albo zacznij od zera i sam dodaj pierwszą osobę.

## Karty przeglądu

showcase-links-title = { $n ->
        [one] Jedna relacja spoza rodziny
        [few] { $n } relacje spoza rodziny
        [many] { $n } relacji spoza rodziny
       *[other] { $n } relacji spoza rodziny
    }
showcase-links-detail = Rodzice chrzestni, pracodawcy, świadkowie i mentorzy, każdy z własnymi datami, źródłem i Twoim stopniem pewności.
showcase-occupations-title = { $n ->
        [one] Jeden zawód z początkiem i końcem
        [few] { $n } zawody z początkiem i końcem
        [many] { $n } zawodów z początkiem i końcem
       *[other] { $n } zawodów z początkiem i końcem
    }
showcase-occupations-detail = „Nauczyciel, 1948–1978” zachowuje swoją długość i rysowany jest jako pasek przez lata, a nie jako pojedynczy datowany wiersz.
showcase-uncertain-title = { $n ->
        [one] Jedna data pozostawiona tak niepewna, jak ją podano
        [few] { $n } daty pozostawione tak niepewne, jak je podano
        [many] { $n } dat pozostawionych tak niepewnych, jak je podano
       *[other] { $n } dat pozostawionych tak niepewnych, jak je podano
    }
showcase-uncertain-detail = Około, przed, po i między pozostają czterema różnymi twierdzeniami. Data, której źródło nie potrafiło ustalić, nigdy nie jest pokazywana tak, jakby potrafiło.
showcase-preserved-title = { $n ->
        [one] Jedna data zachowana w słowach, w których ją zapisano
        [few] { $n } daty zachowane w słowach, w których je zapisano
        [many] { $n } dat zachowanych w słowach, w których je zapisano
       *[other] { $n } dat zachowanych w słowach, w których je zapisano
    }
showcase-preserved-detail = Sformułowanie, którego nikt nie potrafił odczytać jako daty, zostaje dokładnie takie, jak je zapisano, zamiast zostać po cichu odrzucone.
showcase-sources-title = { $n ->
        [one] Jedno źródło z zapisaną wiarygodnością
        [few] { $n } źródła z zapisaną wiarygodnością
        [many] { $n } źródeł z zapisaną wiarygodnością
       *[other] { $n } źródeł z zapisaną wiarygodnością
    }
showcase-sources-detail = { $primary ->
        [one] Jedno źródło pierwotne.
        [few] { $primary } źródła pierwotne.
        [many] { $primary } źródeł pierwotnych.
       *[other] { $primary } źródeł pierwotnych.
    } Każdy fakt pokazuje, na jakim dowodzie się opiera i jak silny jest ten dowód.
showcase-places-title = { $n ->
        [one] Jedno miejsce, którego granice się przesunęły
        [few] { $n } miejsca, których granice się przesunęły
        [many] { $n } miejsc, których granice się przesunęły
       *[other] { $n } miejsc, których granice się przesunęły
    }
showcase-places-detail = Miasto może w różnym czasie należeć do różnych państw, a zapis mówi, które obowiązywało kiedy.

## Szczegóły wpisu

record-also-recorded-as = zapisany także jako
record-borders-moved = Granice się przesunęły:
record-display-name = nazwa wyświetlana
record-read-as = czytane jako
record-note = Notatka
record-living-yes = żyje
record-deceased = zmarły(a)
record-centre-tree-here = Wyśrodkuj drzewo tutaj
record-centre-tree-title = Przesuń drzewo, aby wyśrodkować je na tej osobie
record-open-full-page = Otwórz pełną stronę ↗
record-open-full-title = Otwórz samodzielną stronę, którą można udostępnić
record-edit = Edytuj
panel-empty = Wybierz kartę, aby zobaczyć tutaj pełny wpis tej osoby.
person-see-in-tree = Zobacz tę osobę w drzewie
person-visibility-inline = widoczność:
person-age-at-death = zmarł w wieku { $n } lat
person-age-now = { $n } lat
person-born-in = urodzony w { $place }
person-died-in = zmarł w { $place }
person-children-count = { $n ->
        [one] jedno dziecko
        [few] { $n } dzieci
        [many] { $n } dzieci
       *[other] { $n } dzieci
    }
person-generations-below = { $n ->
        [one] jedno pokolenie poniżej
        [few] { $n } pokolenia poniżej
        [many] { $n } pokoleń poniżej
       *[other] { $n } pokoleń poniżej
    }
person-portrait-of = Fotografia: { $name }
person-no-portrait = Brak zapisanej fotografii

## Wyniki operacji

result-diagnostics = Diagnostyka
result-diagnostics-note = Każdy komunikat zwrócony przez bibliotekę, w tym ostrzeżenia, które nie zablokowały operacji. Nic nie jest odfiltrowywane.
result-no-diagnostics = Biblioteka nie zwróciła żadnych komunikatów.
result-continue = Dalej
result-dashboard = Pulpit
person-sections-label = Sekcje na tej stronie

## Vocabulary the structured editors offer

name-part-nasab = nasab (rodowód)
name-part-laqab = lakab (przydomek)
name-part-kunya = kunja (teknonim)
name-part-nisbah = nisba (pochodzenie)
name-part-alias = alias
name-part-religious_name = imię zakonne
name-part-pen_name = pseudonim literacki
name-type-pen_name = pseudonim literacki
gender-U = Nie zapisano

## Sekcje wpisu, szczegóły

record-notes-title = Uwagi do tego wpisu:
record-name = Nazwa
record-type = Typ
record-cause = Przyczyna:
record-as = jako
record-partner-not-recorded = Partner niezapisany
record-union-from = Od
record-union-at = w
record-union-until = do
record-occupation-from = od
record-occupation-until = do
record-source-reliability = Wiarygodność
record-source-supports = Potwierdza
record-photographs = Fotografie
record-documents = Dokumenty
record-file = Plik
record-status = Status
record-size = Rozmiar
record-absent-document = Wymieniony przez tę osobę, ale nieprzechowywany tutaj.
record-no-file = brak pliku
record-attach-document = Dołącz dokument
record-upload = Wyślij
record-upload-help = Do { $mb } MB na plik. Załączniki trzymane są obok drzewa i zapisywane z powrotem do archiwum przy eksporcie, więc fotografia podróżuje razem z rodziną, do której należy. Rodzaj pliku odczytywany jest z jego własnej zawartości, a nie z nazwy: przyjmowane są obrazy, PDF, zwykły tekst, dźwięk i wideo. SVG jest odrzucany, ponieważ SVG może nieść skrypt.
record-upload-help-short = Do { $mb } MB. SVG jest odrzucany.
record-verbatim-note = Zachowane dokładnie tak, jak podawał to zapis, ponieważ żaden konwerter nie potrafił tego zinterpretować.
record-file-to-attach = Plik do dołączenia
record-document-type = Typ dokumentu
record-caption = Podpis
record-caption-placeholder = Podpis (opcjonalnie)
record-history-entry-meta = — { $at }
record-history-entry-version = , { $version }

## Rodzaje obiektów

kind-person = osoba
kind-family = rodzina
kind-event = wydarzenie
kind-link = powiązanie
kind-occupation = zawód
kind-source = źródło
kind-place = miejsce
kind-document = dokument

kind-person-plural = { $n ->
        [one] osoba
        [few] osoby
        [many] osób
       *[other] osób
    }
kind-family-plural = { $n ->
        [one] rodzina
        [few] rodziny
        [many] rodzin
       *[other] rodzin
    }
kind-event-plural = { $n ->
        [one] wydarzenie
        [few] wydarzenia
        [many] wydarzeń
       *[other] wydarzeń
    }
kind-link-plural = { $n ->
        [one] powiązanie
        [few] powiązania
        [many] powiązań
       *[other] powiązań
    }
kind-occupation-plural = { $n ->
        [one] zawód
        [few] zawody
        [many] zawodów
       *[other] zawodów
    }
kind-source-plural = { $n ->
        [one] źródło
        [few] źródła
        [many] źródeł
       *[other] źródeł
    }
kind-place-plural = { $n ->
        [one] miejsce
        [few] miejsca
        [many] miejsc
       *[other] miejsc
    }
kind-document-plural = { $n ->
        [one] dokument
        [few] dokumenty
        [many] dokumentów
       *[other] dokumentów
    }

## Listy

list-matching = { $total ->
        [one] Jedno dopasowanie
        [few] { $total } dopasowania
        [many] { $total } dopasowań
       *[other] { $total } dopasowań
    }, { $per_page } na stronę.
list-filter-placeholder = Filtruj po nazwie lub identyfikatorze
list-filter = Filtruj
list-clear = Wyczyść
list-summary = Opis
list-id = Identyfikator
list-actions = Działania
list-nothing = Nic tutaj nie ma.
list-nothing-matching = Nic tutaj nie pasuje do „{ $q }”.
list-delete-confirm = Usunąć ten obiekt ({ $kind })? Wybierz, co ma się stać z obiektami, które go przywołują:
list-policy-reject = Odmów
list-policy-reject-detail = — odmów, jeśli cokolwiek nadal go przywołuje. Nic nie ginie.
list-policy-cascade = Kaskadowo
list-policy-cascade-detail = — usuń go i fizycznie usuń każde odwołanie do niego.
list-policy-orphan = Osieroć
list-policy-orphan-detail = — usuń go, ale zachowaj przywołujące wpisy z wyzerowanym odwołaniem.

## Kompletność

completeness-dates-title = Daty według kształtu, jaki naprawdę mają
completeness-no-dates = Nie zapisano jeszcze żadnych dat.
completeness-dates-note = Data, którą ktoś ustalił co do dnia, i data, którą ktoś potrafił umieścić tylko w dekadzie, to dwa różne twierdzenia — oba zachowane tak, jak zostały podane. Tekst, którego w ogóle nie dało się odczytać jako daty, zachowywany jest słowo w słowo, zamiast zostać usunięty.
completeness-shape-exact = dokładna
completeness-shape-exact-note = pełny dzień kalendarzowy
completeness-shape-approximate = przybliżona
completeness-shape-approximate-note = około albo sam rok lub dekada
completeness-shape-ranged = zakres
completeness-shape-ranged-note = przed, po albo między
completeness-shape-preserved = zachowana dosłownie
completeness-shape-preserved-note = tekst nie do rozczytania, zachowany bez zmian
completeness-shape-unknown = nieznana
completeness-shape-unknown-note = zapisana jako nieznana

## Strona importu

convert-page-title = Importuj plik rodzinny
convert-lede = Przenieś istniejące drzewo z pliku GEDCOM — to eksport, który tworzy większość programów genealogicznych. Nic nie jest tu przechowywane, a drzewo, które ta strona już pokazuje, zostaje dokładnie takie, jakie było.
convert-file-label = Plik rodzinny (.ged)
convert-file-hint = Do { $mb } MB. Drzewo 767 osób to około 320 KB.
convert-confidence-label = Jak pewne są te fakty na początek
convert-confidence-hint = Importowany plik nie mówi, jak pewny był ktokolwiek, więc każdy fakt potrzebuje punktu wyjścia. Ustaw nisko dla drzewa złożonego pospiesznie, wyżej dla opracowanego na podstawie dokumentów. Uczciwe odczytanie tej liczby brzmi „zaimportowane i przez nikogo od tamtej pory niesprawdzone” — każdy fakt możesz później podnieść lub obniżyć, po jednym.
convert-lang-label = Język nazw miejscowości
convert-lang-hint = Oznaczenie takie jak en, fr czy pl.

## Raport z importu

convert-failed = Import się nie powiódł
convert-try-another = Spróbuj innego pliku
convert-converted = Zaimportowano { $filename }
convert-result-lede = { $total ->
        [one] Jeden wpis
        [few] { $total } wpisy
        [many] { $total } wpisów
       *[other] { $total } wpisów
    }, { $size } KB. Wszystko weszło ze stopniem pewności { $confidence }, a nazwy miejscowości odczytano jako { $lang }. Drzewo pokazywane przez tę stronę nie zostało ruszone.
convert-produced = Co przeszło
convert-skipped-title = { $n ->
        [one] Jeden wpis, którego nie dało się odczytać
        [few] { $n } wpisy, których nie dało się odczytać
        [many] { $n } wpisów, których nie dało się odczytać
       *[other] { $n } wpisów, których nie dało się odczytać
    }
convert-skipped-note = Te wpisy nie zawierały nic, co dałoby się przenieść.
convert-other-diagnostics = { $n ->
        [one] Jedna inna rzecz warta uwagi
        [few] { $n } inne rzeczy warte uwagi
        [many] { $n } innych rzeczy wartych uwagi
       *[other] { $n } innych rzeczy wartych uwagi
    }
convert-clean = Nic nie zostało z tyłu — każdy wpis z pliku przeszedł.
convert-download-title = Pobieranie
convert-download-named = Pobierz { $name }
convert-download-note = Trzymane tutaj przez piętnaście minut, a potem usuwane, więc pobierz teraz.
convert-another = Zaimportuj kolejny plik
admin-history-on = w
admin-history-meta = — { $kind }, { $at }
admin-validation-counts = { $errors ->
        [one] Jeden błąd
        [few] { $errors } błędy
        [many] { $errors } błędów
       *[other] { $errors } błędów
    }, { $warnings ->
        [one] jedno ostrzeżenie
        [few] { $warnings } ostrzeżenia
        [many] { $warnings } ostrzeżeń
       *[other] { $warnings } ostrzeżeń
    }, { $infos ->
        [one] jedna uwaga
        [few] { $infos } uwagi
        [many] { $infos } uwag
       *[other] { $infos } uwag
    }.
admin-warnings-never-block = Ostrzeżenia nigdy nie blokują — są informacją, a nie bramką.
admin-validator-clean = Sprawdzanie poprawności nic nie zgłosiło.
record-occupations-help-undated = Zawód zapisuje się z początkiem i końcem, dzięki czemu kilka da się porównać na jednej osi czasu. To archiwum ma nazwy zawodów, ale bez dat — zwykłe po imporcie, bo większość plików rodzinnych nie ma gdzie ich trzymać — więc nie ma jeszcze czego wyskalować.
record-occupations-help-axis = Zawód to stan trwający w czasie, a nie wydarzenie w jednej dacie. Wszystkie odcinki dzielą jedną oś, { $from }–{ $to }.
admin-value-not-set = nie ustawiono
admin-validation-report = Raport ze sprawdzenia poprawności
admin-dedup-complete = Usuwanie duplikatów zakończone
admin-dedup-refused = Usuwanie duplikatów odrzucone
record-birth-order = kolejność urodzenia
record-start-not-recorded = początek niezapisany
record-end-not-recorded = koniec niezapisany
record-document-no-file = Dokument jest tu zapisany, ale samego pliku nie ma
panel-selected-person = Wybrana osoba

## Pasy pokoleń

tree-band-generation = Pokolenie { $g }
tree-band-people = { $n ->
        [one] jedna osoba
        [few] { $n } osoby
        [many] { $n } osób
       *[other] { $n } osób
    }
tree-band-unplaced = Bez miejsca
tree-band-unplaced-note = { $n ->
        [one] jedna osoba bez rodziny — pokazana, a nie pominięta
        [few] { $n } osoby bez rodziny — pokazane, a nie pominięte
        [many] { $n } osób bez rodziny — pokazanych, a nie pominiętych
       *[other] { $n } osób bez rodziny — pokazanych, a nie pominiętych
    }

## Słownictwo kontrolowane

gender-M = Mężczyzna
gender-F = Kobieta
gender-NB = Osoba niebinarna
gender-unrecorded = Niezapisana

name-part-given_name = imię
name-part-family_name = nazwisko
name-part-patronymic = imię odojcowskie
name-part-matronymic = imię odmatczyne
name-part-middle_name = drugie imię
name-part-nickname = przezwisko
name-part-prefix = przedrostek
name-part-suffix = przyrostek
name-part-particle = partykuła
name-part-part = człon

name-type-primary = główne
name-type-other = inne
name-type-alias = używane
name-type-birth = rodowe
name-type-married = po mężu
name-type-religious = zakonne
name-type-transliteration = transliteracja
name-type-nickname = przezwisko

## Uwagi do wpisu

note-links = { $n ->
        [one] relacja spoza rodziny, z własnymi datami i źródłami
        [few] { $n } relacje spoza rodziny, z własnymi datami i źródłami
        [many] { $n } relacji spoza rodziny, z własnymi datami i źródłami
       *[other] { $n } relacji spoza rodziny, z własnymi datami i źródłami
    }
note-occupations = { $n ->
        [one] zawód zapisany z początkiem i końcem
        [few] { $n } zawody zapisane z początkiem i końcem
        [many] { $n } zawodów zapisanych z początkiem i końcem
       *[other] { $n } zawodów zapisanych z początkiem i końcem
    }
note-birth-imprecise = data urodzenia, której źródło nie potrafiło ustalić, pokazana tak, jak ją zapisano
note-death-imprecise = data śmierci, której źródło nie potrafiło ustalić, pokazana tak, jak ją zapisano
note-names = { $n ->
        [one] jedna zapisana nazwa
        [few] { $n } zapisane nazwy
        [many] { $n } zapisanych nazw
       *[other] { $n } zapisanych nazw
    }
note-transliteration = nazwa w swoim własnym piśmie obok transliteracji łacińskiej
note-witnessed = { $n ->
        [one] wydarzenie, którego była świadkiem, a nie bohaterem
        [few] { $n } wydarzenia, których była świadkiem, a nie bohaterem
        [many] { $n } wydarzeń, których była świadkiem, a nie bohaterem
       *[other] { $n } wydarzeń, których była świadkiem, a nie bohaterem
    }

visibility-public = publiczne
visibility-members = członkowie rodziny
visibility-contributors = współtwórcy
visibility-private = prywatne

## Opisy wierszy na listach administracyjnych

family-label-couple = { $children ->
        [0] { $a } i { $b }
        [one] { $a } i { $b } — jedno dziecko
        [few] { $a } i { $b } — { $children } dzieci
        [many] { $a } i { $b } — { $children } dzieci
       *[other] { $a } i { $b } — { $children } dzieci
    }
family-label-half = { $children ->
        [0] { $a } i { $unknown }
        [one] { $a } i { $unknown } — jedno dziecko
        [few] { $a } i { $unknown } — { $children } dzieci
        [many] { $a } i { $unknown } — { $children } dzieci
       *[other] { $a } i { $unknown } — { $children } dzieci
    }
family-label-children = { $others ->
        [0] { $first } — rodzice niezapisani
        [one] { $first } i jedno rodzeństwo — rodzice niezapisani
        [few] { $first } i { $others } rodzeństwa — rodzice niezapisani
        [many] { $first } i { $others } rodzeństwa — rodzice niezapisani
       *[other] { $first } i { $others } rodzeństwa — rodzice niezapisani
    }
family-label-empty = Rodzina bez zapisanych osób

event-label = { $category } — { $who }, { $date }
event-label-nobody = { $category } — { $date }
event-two-people = { $a } i { $b }
event-more-people = { $a } i { $b } oraz { $others ->
        [one] jedna inna osoba
        [few] { $others } inne osoby
        [many] { $others } innych osób
       *[other] { $others } innych osób
    }

link-label = { $label }: { $from } → { $to }
occupation-label = { $who } — { $title }
source-label = { $title } — { $reliability }
source-label-plain = { $title }
document-label = { $filename } — { $type }
document-label-untitled = { $type } bez tytułu
list-unnamed = { $kind } bez nazwy

## Słowniki specyfikacji używane na listach

event-category-birth = Narodziny
event-category-death = Zgon
event-category-marriage = Ślub
event-category-divorce = Rozwód
event-category-baptism = Chrzest
event-category-burial = Pogrzeb
event-category-immigration = Imigracja
event-category-emigration = Emigracja
event-category-census = Spis ludności
event-category-residence = Miejsce zamieszkania
event-category-military = Służba wojskowa
event-category-education = Wykształcenie
event-category-other = Wydarzenie

reliability-primary = źródło pierwotne
reliability-secondary = źródło wtórne
reliability-tertiary = opracowanie
reliability-recollection = relacja ustna
reliability-derivative = opracowanie wtórne
reliability-authored = opracowanie autorskie
reliability-oral = przekaz ustny
reliability-unknown = wiarygodność nieznana

document-type-photo = fotografia
document-type-certificate = akt
document-type-letter = list
document-type-record = zapis archiwalny
document-type-newspaper = wycinek prasowy
document-type-other = dokument

## Gdzie ten wpis mógłby powiedzieć więcej

completeness-title = Gdzie to drzewo mogłoby powiedzieć więcej
completeness-intro = Co jest zapisane, a co wciąż puste.
completeness-import-title = Co przyniósł import
completeness-import-intro = Policzone z pliku, który przed chwilą wysłałeś. Pusty wiersz to coś, czego pierwotny plik nie zapisywał — a nie coś, co import zgubił.

completeness-headline-full = Każdy rodzaj szczegółu poniżej jest gdzieś w tym drzewie zapisany.
completeness-headline-empty = { $total ->
        [one] Jedyny rodzaj szczegółu poniżej nie jest jeszcze nigdzie zapisany.
        [few] Żaden z { $total } rodzajów szczegółów poniżej nie jest jeszcze zapisany.
        [many] Żaden z { $total } rodzajów szczegółów poniżej nie jest jeszcze zapisany.
       *[other] Żaden z { $total } rodzajów szczegółów poniżej nie jest jeszcze zapisany.
    } Każdy z nich to miejsce, w którym zapis mógłby powiedzieć więcej.
completeness-headline-partial = { $carried ->
        [one] Jeden rodzaj szczegółu poniżej jest zapisany
        [few] { $carried } rodzaje szczegółów poniżej są zapisane
        [many] { $carried } rodzajów szczegółów poniżej jest zapisanych
       *[other] { $carried } rodzajów szczegółów poniżej jest zapisanych
    }; { $empty ->
        [one] jeden jest wciąż pusty
        [few] { $empty } są wciąż puste
        [many] { $empty } jest wciąż pustych
       *[other] { $empty } jest wciąż pustych
    }.

completeness-metric-confidence = Jak pewny jest każdy fakt
completeness-metric-confidence-none = Żaden z { $slots } faktów tutaj nie mówi, jak jest pewny. Data odczytana z aktu i data zgadnięta wyglądają tak samo — do chwili, gdy przestają.
completeness-metric-confidence-uniform = { $with } z { $slots } faktów niesie ocenę i każda z nich to ta sama liczba ({ $modal }). Tyle zostawia po sobie import masowy: wartość zastępcza, do której nikt nie wrócił. Żaden nie został jeszcze oceniony pojedynczo.
completeness-metric-confidence-some = { $with } z { $slots } faktów niesie ocenę. { $modal_count } dzieli jedną wartość ({ $modal }); { $assessed } różni się od niej, więc zostały obejrzane po kolei.
completeness-metric-confidence-many = { $with } z { $slots } faktów niesie ocenę, z czego { $assessed } różni się od najczęstszej wartości ({ $modal }), w { $distinct } odrębnych poziomach. To drzewo zapisuje prawdziwą, zróżnicowaną niepewność.

completeness-metric-parentage = Jak pewne jest każde powiązanie rodzic–dziecko
completeness-metric-parentage-none = Żadne pochodzenie tutaj nie mówi, jak jest pewne. Adopcje, linie sporne i rekonstrukcje z jednej wzmianki to dokładnie te miejsca, w których rodzina musi zapisać wątpliwość — a drzewo rysuje mniej pewne powiązanie bledszą linią.
completeness-metric-parentage-some = { $n ->
        [one] Jedno pochodzenie niesie własną ocenę
        [few] { $n } pochodzenia niosą własną ocenę
        [many] { $n } pochodzeń niesie własną ocenę
       *[other] { $n } pochodzeń niesie własną ocenę
    }, więc linia przypuszczalna jest widocznie słabsza niż udokumentowana.

completeness-metric-links = Relacje poza krwią i małżeństwem
completeness-metric-links-none = Rodzice chrzestni, pracodawcy, świadkowie, mentorzy, opiekunowie. Żadnych jeszcze nie zapisano. Każda może nieść własne daty, swoje źródło i Twój stopień pewności.
completeness-metric-links-some = { $n ->
        [one] Jedna zapisana, z własnymi datami, źródłem i Twoim stopniem pewności.
        [few] { $n } zapisane, każda z własnymi datami, źródłem i Twoim stopniem pewności.
        [many] { $n } zapisanych, każda z własnymi datami, źródłem i Twoim stopniem pewności.
       *[other] { $n } zapisanych, każda z własnymi datami, źródłem i Twoim stopniem pewności.
    }

completeness-metric-occupations = Praca zapisana z początkiem i końcem
completeness-metric-occupations-none = Nie zapisano żadnych zawodów. Rzemiosło uprawiane przez trzydzieści lat mówi o życiu więcej niż pojedynczy datowany wpis.
completeness-metric-occupations-undated = { $total ->
        [one] Zapisano jeden zawód, bez dat
        [few] Zapisano { $total } zawody, bez dat
        [many] Zapisano { $total } zawodów, bez dat
       *[other] Zapisano { $total } zawodów, bez dat
    }. Dodaj początek i koniec, a da się je porównać obok siebie na jednej osi czasu.
completeness-metric-occupations-some = { $span } z { $total } ma początek albo koniec, więc da się je porównać obok siebie na jednej osi czasu.

completeness-metric-sources = Źródła z oceną wiarygodności
completeness-metric-sources-none = Nie zapisano żadnych źródeł. Wskazanie, skąd wziął się fakt, pozwala krewnemu sprawdzić go później — albo się z nim nie zgodzić i powiedzieć dlaczego.
completeness-metric-sources-some = { $graded } z { $total } mówi, jak są mocne, więc twierdzenie oparte na akcie urodzenia widocznie nie jest tym samym co oparte na wspomnieniu.

completeness-what-is-recorded = Co zapis może powiedzieć
completeness-in-this-tree = W tym drzewie
completeness-not-yet = jeszcze niezapisane

## Role uczestnika wydarzenia

role-spouse = małżonek
role-spouse_1 = pierwszy małżonek
role-spouse_2 = drugi małżonek
role-subject = osoba, której dotyczy
role-participant = uczestnik
role-witness = świadek
role-officiant = celebrans
role-informant = zgłaszający
role-godparent = rodzic chrzestny

phys-no-source = bez źródła
phys-col-date = Kiedy
phys-col-source = Źródło
phys-col-confidence = Pewność
phys-col-note = Uwaga
phys-field-height-cm = Wzrost
phys-field-weight-kg = Waga
phys-field-eye-colour = Kolor oczu
phys-field-hair-colour = Kolor włosów
phys-field-build = Budowa ciała
phys-field-handedness = Ręczność
phys-field-features = Znaki szczególne
phys-field-military = Służba wojskowa
phys-field-languages = Znane języki
phys-field-blood-group = Grupa krwi
phys-field-conditions = Znane schorzenia
phys-field-operations = Operacje i urazy
phys-field-cause-of-death = Przyczyna zgonu
phys-field-religion = Religia lub przynależność
phys-field-health-notes = Uwagi
admin-export-health-note = Zwykły eksport pomija wszystkie kategorie wrażliwe — zdrowie i przekonania, dane biometryczne, dane genetyczne i karalność — oraz profil behawioralny każdej osoby żyjącej, więc plik wysłany krewnemu nie zawiera żadnej z nich. Zaznacz, co ma zawierać konkretny plik; samo archiwum zapisuje, które kategorie pominięto.
avatar-picker-title = Wybierz zdjęcie
avatar-choose-link = Wybierz zdjęcie
avatar-choose = Które zdjęcie reprezentuje tę osobę
avatar-mode-auto = Niech wybierze program
avatar-mode-auto-note = Pierwszy portret, a jeśli go nie ma — pierwszy obraz powiązany z tym zapisem.
avatar-mode-none = Pokaż inicjały
avatar-mode-none-note = Dla zapisu, w którym obrazy są dokumentami, a nie twarzami.
avatar-focal-hint = Kliknij zdjęcie, aby je wybrać, i kliknij ponownie miejsce, które ma pozostać w kadrze — awatar jest kwadratowy, a większość skanów nie.
avatar-no-images = Do tego zapisu nie dołączono jeszcze żadnych obrazów.
avatar-upload-title = Prześlij zdjęcie i użyj go
avatar-upload-button = Prześlij i ustaw jako zdjęcie
avatar-not-available-title = To zdjęcie jest niedostępne
avatar-not-available-detail = Wybrany plik nie jest powiązany z tą osobą albo nie masz prawa go odczytać.

record-history-withheld = nieudostępnione

## A death nobody wrote down
#
# GEDCOM cannot say "died, date unknown", so a converter marks those
# people living. These three say that the page worked it out rather than
# read it, which is a distinction this product does not blur.

record-life-status = Stan
record-presumed-deceased = zgon domniemany
record-presumed-short = domniem.
record-presumed-why = Nie zapisano zgonu, a urodzenie miało miejsce ponad { $years } lat temu, więc ten zapis nie może być prawdziwy. Archiwum pozostaje bez zmian: to wniosek strony, a nie treść źródła.

## The identity editor

identity-editor-title = Nazwiska i tożsamość
identity-primary-name = Nazwa pokazywana wszędzie
identity-primary-help = To, czego używa karta drzewa, nagłówek i każda lista. Pozostałe nazwy poniżej to te, których źródło używało w innym czasie.
identity-display = Nazwa
identity-display-latin = Zapis łaciński
identity-culture = Język
identity-direction = Kierunek pisma
identity-direction-ltr = od lewej do prawej
identity-direction-rtl = od prawej do lewej
identity-direction-auto = z tekstu
identity-components = Części nazwy
identity-components-help = Która część jest imieniem, a która nazwiskiem, w kolejności zapisu. Zapis bez części i tak się wyświetli: części służą do wyszukiwania.
identity-part = Część
identity-value = Tekst
identity-other-names = Inne nazwy
identity-other-help = Nazwisko po ślubie, imię zakonne, nazwa użyta w późniejszym zapisie. Każda ma okres używania i źródło, które o tym mówi.
identity-name-type = Rodzaj nazwy
identity-valid-from = Używane od
identity-valid-until = Używane do
identity-about = O osobie
identity-living-help = To znacznik ustawiony przez źródło. Strona osobno domniemywa zgon, gdy urodzenie jest zbyt dawne, a to domniemanie nigdy nie zmienia tego pola ani archiwum.
identity-error-no-display = Zapis potrzebuje nazwy, pod którą będzie pokazywany. Nic nie zapisano.
editor-blank-to-remove = Wyczyść nazwę, aby usunąć ten wpis.
# A BCP 47 tag, shown as an example in the language field.
identity-culture-placeholder = pl
identity-edit-link = Edytuj nazwy i tożsamość

## Union types, statuses and date precision, said out loud

union-type-marriage = małżeństwo
union-type-civil_union = związek cywilny
union-type-cohabitation = konkubinat
union-type-religious_only = związek religijny
union-type-polygamous = poligamiczny
union-type-unknown = nie zapisano
union-status-active = trwa
union-status-ended_by_death = zakończony śmiercią
union-status-ended_by_divorce = zakończony rozwodem
union-status-ended_by_separation = zakończony separacją
union-status-annulled = unieważniony
union-status-unknown = nie zapisano
union-status-ended = zakończony
union-status-ended-by = zakończony: { $reason }
union-reason-death_of_spouse = śmierć małżonka
precision-exact = co do dnia
precision-year = co do roku
precision-month = co do miesiąca
precision-decade = co do dekady
precision-century = co do wieku
precision-unknown = nieznana
record-precision = Dokładność
record-approximate = Przybliżona
record-place = Miejsce

## The relationships editor

family-editor-title = Rodzina i relacje
family-unions = Związki
family-no-unions = Dla tej osoby nie zapisano żadnego związku.
family-union-legend = Związek { $n }
family-writes-family = Zapis zmienia zapis rodziny #{ $id }, wspólny dla obu osób. Strona drugiej osoby też się zmieni.
family-partners = Partnerzy
family-partner = Partner
family-role = Rola
family-children = Dzieci
family-children-help = Kolejność urodzenia to własne twierdzenie zapisu. Pozostawiona pusta, nie twierdzi nic: numer wzięty z pozycji wiersza byłby faktem, którego nikt nie zapisał.
family-child = Dziecko
family-birth-order = Kolejność urodzenia
family-the-union = Sam związek
family-type = Rodzaj związku
family-status = Stan
family-started = Początek
family-ended = Koniec
family-leave = Usuń tę osobę z tego związku
family-open-entity = Otwórz zapis rodziny
family-new-union = Nowy związek
family-new-union-help = Tworzy to nowy zapis rodziny z tą osobą. Partner jest opcjonalny: rodzic wymieniony bez żadnego partnera to związek jednoosobowy.
family-create-union = Utwórz związek
family-parents = Rodzice
family-no-parents = Ta osoba nie jest zapisana jako dziecko żadnej rodziny.
family-child-of = Dziecko tej rodziny
family-detach-child = Usuń tę osobę z tej rodziny
family-attach-parents = Przypisz do rodziców
family-attach-help = Wybierz rodzinę, której dzieckiem jest ta osoba. Doda ją to do zapisu tej rodziny, więc pojawi się także na stronach rodziców.
family-the-family = Rodzina
family-attach = Przypisz
family-error-last-partner = Związek musi zawierać co najmniej jedną osobę. Usuń zamiast tego zapis rodziny — zapyta, co zrobić ze wszystkim, co się do niego odwołuje.
family-error-no-family = Nie wybrano rodziny. Nic nie zapisano.
family-error-already-child = Ta osoba jest już dzieckiem tej rodziny.
pick-error-empty = Nie wskazano osoby. Nic nie zapisano.
pick-error-not-found = W tym archiwum nie ma osoby o tej nazwie. Nic nie zapisano.
pick-error-ambiguous = Odpowiada temu więcej niż jedna osoba. Wybierz jedną z listy, aby zapis wskazywał którą. Nic nie zapisano.

## Links and occupations

links-editor-title = Powiązania
links-editor-help = Relacje, które nie są rodzinne: chrzestny, pracodawca, świadek, pułk. Każda jest osobnym zapisem wskazującym dwie osoby, więc edycja tutaj zmienia też to, co pokazuje drugi zapis.
links-none = Dla tej osoby nie zapisano żadnego powiązania.
links-new = Nowe powiązanie
links-create = Utwórz powiązanie
links-remove = Usuń to powiązanie
links-other-end = Druga strona
links-label = Czym jest
links-label-reverse = W drugą stronę
links-category = Kategoria
links-bidirectional = Czyta się tak samo w obie strony
links-from = Od
links-until = Do
links-reversed = To powiązanie utworzono z drugiego zapisu. Edycja tutaj zmienia ten sam byt.
link-error-no-label = Powiązanie musi mówić, czym jest. Nic nie zapisano.
occupations-editor-title = Zajęcia
occupations-editor-help = Zajęcie to okres z początkiem i końcem, a nie nazwa stanowiska. Każde ma własne daty i własne źródło.
occupations-none = Dla tej osoby nie zapisano żadnego zajęcia.
occupations-new = Nowe zajęcie
occupations-create = Utwórz zajęcie
occupations-remove = Usuń to zajęcie
occupations-title = Czym się zajmował
occupations-employer = Dla kogo
occupations-employer-place = Gdzie się mieścił
occupations-from = Od
occupations-until = Do
occupation-error-no-title = Zajęcie musi mówić, co ktoś robił. Nic nie zapisano.
link-category-spiritual = duchowa
link-category-professional = zawodowa
link-category-social = towarzyska
link-category-legal = prawna
link-category-medical = medyczna
link-category-educational = edukacyjna
link-category-conflict = konflikt
link-category-other = inna
links-edit-link = Edytuj powiązania
occupations-edit-link = Edytuj zajęcia
family-edit-link = Edytuj rodzinę i relacje

## Events and documents

events-editor-title = Wydarzenia
events-editor-help = Wydarzenie wymienia naraz kilka osób — ślub, chrzest, spis — więc każde jest osobnym zapisem i pojawia się na każdej wymienionej stronie.
events-none = Żadne wydarzenie nie wymienia tej osoby.
events-new = Nowe wydarzenie
events-new-help = Ta osoba zostanie dodana jako podmiot, jeśli nie wskażesz nikogo innego. Wydarzenie bez nikogo to tylko data.
events-create = Utwórz wydarzenie
events-remove = Usuń to wydarzenie
events-category = Co się wydarzyło
events-subcategory = Dokładniej
events-description = Opis
events-participants = Kto brał udział
events-participants-help = Zapis zmienia zapis wydarzenia, który pokazuje też każda inna wymieniona osoba.
events-who = Kto
event-error-no-category = Wydarzenie musi mówić, co się stało. Nic nie zapisano.
documents-editor-title = Dokumenty
documents-editor-help = Na jakie pliki wskazuje ten zapis i czym każdy z nich dla niego jest. Wyczyszczenie wiersza odłącza plik: dokument i jego zawartość zostają w archiwum.
documents-attached = Dołączone do tego zapisu
documents-upload = Prześlij plik
documents-upload-help = Do { $mb } MB. Plik trafia do archiwum i zostaje dołączony do tego zapisu.
documents-caption = Podpis
documents-edit-link = Dołącz i odłącz dokumenty
events-edit-link = Edytuj wydarzenia

## Presentation styles: density, never colour

prefs-style = Gęstość
prefs-style-help = Ile miejsca zajmuje strona. Niezależne od motywu, który dotyczy tylko koloru, więc każde z tych ustawień można łączyć z każdym motywem.
style-comfortable = Wygodna
style-comfortable-note = domyślna, z miejscem do czytania
style-compact = Zwarta
style-compact-note = więcej zapisu na ekranie, do przeglądania wielu
style-paper = Papier
style-paper-note = szeryfowy krój i linie zamiast kart, do czytania na spokojnie lub druku

## Sensitive classes

admin-export-choose = Dołącz do tego eksportu
scope-health = Zdrowie i przekonania
scope-biometrics = Dane biometryczne
scope-genomics = Dane genetyczne
scope-legal = Karalność
scope-behaviour = Profile behawioralne osób żyjących
admin-export-with-chosen = Eksportuj z zaznaczonymi

## Profile

pg-identity = Tożsamość i stan cywilny
pg-identity-intro = Kim według zapisów była ta osoba i co odnotowały akta stanu cywilnego.
pg-morphology = Morfologia
pg-morphology-intro = Ciało, tak jak je zmierzono i opisano.
pg-biometrics = Biometria
pg-biometrics-intro = Głos, ręce i zmysły oraz wzorce, po których można rozpoznać osobę.
pg-health = Zdrowie
pg-health-intro = Choroby, leczenie, pomiary i wyniki badań.
pg-genomics = Genomika
pg-genomics-intro = Testy DNA, haplogrupy, warianty i inne wyniki badań molekularnych.
pg-death = Śmierć
pg-death-intro = Jak, kiedy i gdzie zakończyło się życie oraz co stało się z ciałem.
pg-residence = Miejsce zamieszkania i obywatelstwo
pg-residence-intro = Gdzie osoba mieszkała, jakie państwa uznawały ją za obywatela i jakimi językami mówiła.
pg-education = Wykształcenie i praca
pg-education-intro = Nauka, kwalifikacje, dochody i majątek.
pg-military = Wojsko i odznaczenia
pg-military-intro = Służba, stopnie, jednostki i odznaczenia.
pg-legal = Sprawy karne
pg-legal-intro = Postępowania karne i ich rozstrzygnięcia.
pg-belief = Wyznanie i przynależność
pg-belief-intro = Religia, obrzędy, przekonania i członkostwa.
pg-personality = Osobowość i zachowanie
pg-personality-intro = Temperament, nawyki i zainteresowania, tak jak opisują je źródła.
pg-relationships = Relacje
pg-relationships-intro = Rodzice, partnerzy, dzieci i inne osoby w czyimś życiu.
pg-digital-legacy = Dziedzictwo cyfrowe
pg-digital-legacy-intro = Skany, modele, nagrania i archiwa, które reprezentują osobę.
pa-identity-titles = Tytuły
pa-identity-sex-at-birth = Płeć przy urodzeniu
pa-identity-gender-identity = Tożsamość płciowa
pa-birth-time = Godzina urodzenia
pa-birth-coordinates = Miejsce urodzenia we współrzędnych
pa-civil-status-birth-certificate-number = Numer aktu urodzenia
pa-civil-status-register-entries = Wpisy w aktach stanu cywilnego
pa-civil-status-marginal-annotations = Wzmianki dodatkowe
pa-morphology-height = Wzrost
pa-morphology-weight = Waga
pa-morphology-bmi = Wskaźnik masy ciała
pa-morphology-body-composition = Skład ciała
pa-morphology-build = Budowa ciała
pa-morphology-eye-colour = Kolor oczu
pa-morphology-eye-shape = Kształt oczu
pa-morphology-eye-spacing = Rozstaw oczu
pa-morphology-hair-colour = Naturalny kolor włosów
pa-morphology-hair-texture = Rodzaj włosów
pa-morphology-hairline = Linia włosów
pa-morphology-facial-hair = Zarost
pa-morphology-body-hair = Owłosienie ciała
pa-morphology-skin-tone = Fototyp (Fitzpatrick)
pa-morphology-skin-undertone = Podton cery
pa-morphology-freckles = Piegi
pa-morphology-pigmentation = Zmiany barwnikowe
pa-morphology-scars = Blizny
pa-morphology-tattoos = Tatuaże
pa-morphology-moles = Znamiona
pa-morphology-facial-asymmetries = Asymetrie twarzy
pa-morphology-face-shape = Kształt twarzy
pa-morphology-nose-shape = Kształt nosa
pa-morphology-ear-shape = Kształt uszu
pa-morphology-lip-shape = Kształt ust
pa-morphology-dentition = Uzębienie
pa-morphology-malocclusion = Wada zgryzu (klasa Angle’a)
pa-morphology-posture = Postawa
pa-morphology-gait = Chód
pa-morphology-distinguishing-features = Znaki szczególne
pa-biometrics-fingerprints = Odciski palców
pa-biometrics-retinal-print = Wzór siatkówki
pa-biometrics-voice-signature = Odcisk głosu
pa-biometrics-voice-frequency = Podstawowa częstotliwość głosu
pa-biometrics-vocal-timbre = Barwa głosu
pa-biometrics-spoken-accent = Akcent
pa-biometrics-speech-rate = Tempo mowy
pa-biometrics-verbal-tics = Tiki słowne
pa-biometrics-frequent-vocabulary = Często używane słownictwo
pa-biometrics-speech-register = Rejestr mowy
pa-biometrics-motor-tics = Tiki ruchowe
pa-biometrics-handedness = Ręczność
pa-biometrics-hearing = Słuch
pa-biometrics-visual-acuity = Ostrość wzroku
pa-biometrics-optical-correction = Korekcja wzroku
pa-health-blood-group = Grupa krwi (AB0)
pa-health-rhesus = Czynnik Rh (RhD)
pa-health-blood-pressure = Ciśnienie tętnicze
pa-health-resting-heart-rate = Tętno w spoczynku
pa-health-respiratory-capacity = Wydolność oddechowa
pa-health-conditions = Choroby
pa-health-surgeries = Przebyte operacje
pa-health-injuries = Urazy
pa-health-deformities = Deformacje
pa-health-amputations = Amputacje
pa-health-prostheses = Protezy
pa-health-implants = Implanty
pa-health-devices = Wszczepione urządzenia
pa-health-medications = Leki
pa-health-allergies = Alergie
pa-health-vaccinations = Szczepienia
pa-health-serology = Serologia
pa-health-lab-results = Wyniki badań laboratoryjnych
pa-health-deficiencies = Niedobory
pa-health-sleep-disorders = Zaburzenia snu
pa-health-mental-health-assessments = Oceny zdrowia psychicznego
pa-genomics-autosomal-mapping = Test autosomalnego DNA
pa-genomics-y-haplogroup = Haplogrupa Y-DNA
pa-genomics-mt-haplogroup = Haplogrupa mitochondrialna
pa-genomics-whole-genome-sequencing = Sekwencjonowanie całego genomu
pa-genomics-risk-variants = Warianty ryzyka
pa-genomics-hereditary-conditions = Choroby dziedziczne
pa-genomics-predispositions = Predyspozycje
pa-genomics-epigenetic-markers = Markery epigenetyczne
pa-genomics-epigenetic-age = Wiek epigenetyczny
pa-genomics-gut-microbiome = Mikrobiom jelitowy
pa-genomics-skin-microbiome = Mikrobiom skóry
pa-genomics-toxicological-sensitivities = Wrażliwość na leki i toksyny
pa-death-time = Godzina śmierci
pa-death-coordinates = Miejsce śmierci we współrzędnych
pa-death-causes = Przyczyny śmierci
pa-death-contributing-factors = Czynniki współistniejące
pa-death-autopsy = Sekcja zwłok
pa-death-disposition = Postępowanie ze zwłokami
pa-death-grave = Grób
pa-residence-addresses = Adresy
pa-residence-nationality-of-origin = Obywatelstwo pochodzenia
pa-residence-acquired-nationalities = Nabyte obywatelstwa
pa-residence-mother-tongue = Język ojczysty
pa-residence-spoken-languages = Znane języki
pa-education-level = Poziom wykształcenia
pa-education-diplomas = Dyplomy i stopnie
pa-education-institutions = Szkoły i uczelnie
pa-education-income = Dochód
pa-education-real-estate = Nieruchomości
pa-military-distinctions = Odznaczenia
pa-military-citations = Pochwały
pa-military-ranks = Stopnie
pa-military-units = Jednostki
pa-military-service-numbers = Numery ewidencyjne
pa-legal-criminal-record = Karalność
pa-belief-religions = Wyznanie
pa-belief-sacraments = Sakramenty i obrzędy
pa-belief-beliefs = Przekonania
pa-belief-political-leanings = Poglądy polityczne
pa-belief-memberships = Członkostwa
pa-personality-big-five = Wyniki Wielkiej Piątki
pa-personality-mbti = Typ MBTI
pa-personality-introversion-extraversion = Introwersja i ekstrawersja
pa-personality-stress-tolerance = Odporność na stres
pa-personality-decision-style = Styl podejmowania decyzji
pa-personality-interests = Zainteresowania
pa-personality-hobbies = Hobby
pa-personality-sports = Sport
pa-personality-dietary-habits = Dieta
pa-personality-dependencies = Uzależnienia
pa-digital-legacy-body-models = Modele ciała
pa-digital-legacy-skin-textures = Tekstury skóry
pa-digital-legacy-rigs = Szkielety animacyjne
pa-digital-legacy-voice-corpora = Nagrania do syntezy głosu
pa-digital-legacy-text-corpora = Teksty dla modelu językowego
pa-digital-legacy-digital-traces = Ślady cyfrowe
pa-digital-legacy-carbon-footprint = Ślad węglowy
pa-digital-legacy-behaviour-models = Modele zachowania
pf-identity-titles-text = Tytuł w brzmieniu źródła
pf-identity-titles-kind = Rodzaj tytułu
pf-civil-status-marginal-annotations-text = Wzmianka
pf-morphology-pigmentation-kind = Rodzaj zmiany
pf-biometrics-spoken-accent-description = Jak jest opisywany
pf-biometrics-optical-correction-kind = Korekcja
pf-health-amputations-level = Poziom amputacji
pf-health-prostheses-kind = Proteza
pf-health-implants-kind = Implant
pf-health-devices-kind = Urządzenie
pf-health-allergies-type = Rodzaj alergii
pf-health-vaccinations-status = Status szczepienia
pf-health-sleep-disorders-category = Kategoria zaburzenia
pf-death-autopsy-kind = Sekcja
pf-education-institutions-name = Nazwa szkoły lub uczelni
pf-military-distinctions-name = Nazwa odznaczenia
pf-military-distinctions-kind = Rodzaj odznaczenia
pf-military-citations-text = Treść pochwały
pf-military-ranks-category = Kategoria stopnia
pf-belief-political-leanings-position = Pozycja na osi lewica–prawica
pf-belief-memberships-kind = Rodzaj organizacji
pf-digital-legacy-carbon-footprint-method = Metoda szacowania
pf-age-years = Wiek w latach
pf-agreeableness = Ugodowość
pf-allergen = Alergen
pf-amount = Kwota
pf-analyte = Oznaczany składnik
pf-artefact-type = Rodzaj artefaktu
pf-autoimmune = Autoimmunologiczna
pf-body-region = Okolica ciała
pf-bone-percent = Kości
pf-carrier-status = Nosicielstwo
pf-cause = Przyczyna
pf-chronic = Przewlekła
pf-clock = Zegar
pf-condition = Choroba
pf-conferred-by = Nadane przez
pf-congenital = Wrodzona
pf-conscientiousness = Sumienność
pf-consent = Zgoda
pf-coordinates = Współrzędne
pf-corrected = Z korekcją
pf-country = Państwo
pf-court = Sąd
pf-coverage = Pokrycie
pf-currency = Waluta
pf-decimal = Ostrość (dziesiętnie)
pf-denomination = Wyznanie
pf-derived-from-id = Na podstawie
pf-description = Opis
pf-details = Szczegóły
pf-diagnosis = Rozpoznanie
pf-diameter-mm = Średnica
pf-diastolic = Rozkurczowe
pf-diet = Dieta
pf-document-id = Dokument
pf-dose = Dawka
pf-ear = Ucho
pf-entry-number = Numer wpisu
pf-extraversion = Ekstrawersja
pf-eye = Oko
pf-fat-percent = Tkanka tłuszczowa
pf-fev1-fvc-ratio = Wskaźnik FEV1/FVC
pf-fev1-litres = FEV1
pf-file-format = Format pliku
pf-findings = Ustalenia
pf-flag = Oznaczenie
pf-format = Format
pf-fracture = Złamanie
pf-fvc-litres = FVC
pf-gene = Gen
pf-generator = Wykonano w
pf-grade = Stopień
pf-iccs-section = Sekcja przestępstwa (ICCS)
pf-icd10-chapter = Rozdział ICD-10
pf-indication = Wskazanie
pf-inheritance = Sposób dziedziczenia
pf-inscription = Napis
pf-institution = Uczelnia lub szkoła
pf-instrument = Narzędzie
pf-isced-level = Poziom ISCED
pf-jurisdiction = Jurysdykcja
pf-language = Język
pf-lat = Szerokość geograficzna
pf-level = Poziom
pf-lines = Adres
pf-location = Umiejscowienie
pf-lon = Długość geograficzna
pf-major = Główna haplogrupa
pf-marker = Marker
pf-metaboliser-status = Typ metabolizmu
pf-method = Metoda
pf-mode = Sposób nabycia
pf-muscle-percent = Tkanka mięśniowa
pf-neuroticism = Neurotyczność
pf-number = Numer
pf-nutrient = Składnik odżywczy
pf-offence = Przestępstwo
pf-office = Urząd
pf-openness = Otwartość
pf-organisation = Organizacja
pf-outcome = Rozstrzygnięcie
pf-pace = Tempo starzenia
pf-page = Strona
pf-panel = Panel
pf-party = Partia
pf-pathogen = Patogen
pf-pattern = Wzorzec używania
pf-percentile = Centyl
pf-period = Okres rozliczenia
pf-place-id = Miejsce
pf-plot = Kwatera
pf-polygenic-score = Wynik poligeniczny
pf-postal-code = Kod pocztowy
pf-precision = Dokładność
pf-prescription = Recepta
pf-proficiency = Stopień znajomości
pf-provider = Dostawca
pf-quintile = Kwintyl dochodu
pf-rank = Stopień
pf-rank-text = Stopień w brzmieniu źródła
pf-reaction = Reakcja
pf-reference-build = Genom referencyjny
pf-reference-high = Górna granica normy
pf-reference-low = Dolna granica normy
pf-register-type = Rodzaj wpisu
pf-result = Wynik
pf-role = Funkcja
pf-sacrament = Sakrament lub obrzęd
pf-score = Wynik
pf-sentence = Kara
pf-sequence = Miejsce w ciągu przyczyn
pf-service = Rodzaj sił zbrojnych
pf-severity = Nasilenie
pf-shannon-diversity = Różnorodność Shannona
pf-shape = Kształt
pf-significance = Znaczenie kliniczne
pf-snp-count = Liczba zbadanych SNP
pf-sport = Dyscyplina
pf-subclade = Podklad
pf-substance = Substancja
pf-summary = Podsumowanie
pf-systolic = Skurczowe
pf-tenure = Tytuł prawny
pf-test = Test
pf-threshold-db = Próg słyszenia
pf-title = Tytuł
pf-tonnes-co2e-per-year = Emisje
pf-tradition = Tradycja religijna
pf-tree-version = Wersja drzewa
pf-unit = Jednostka
pf-use = Przeznaczenie
pf-variant = Wariant
pf-volume = Tom
pf-zygosity = Zygotyczność
pu-cm = { $n } cm
pu-kg = { $n } kg
pu-kg-m2 = { $n } kg/m²
pu-percent = { $n }%
pu-mm = { $n } mm
pu-hz = { $n } Hz
pu-words-min = { $n } słów/min
pu-db-hl = { $n } dB HL
pu-mmhg = { $n } mmHg
pu-bpm = { $n } ud./min
pu-litres = { $n } l
pu-coverage = { $n }×
pu-years = { $n } lat
pu-t-co2e-yr = { $n } t CO₂e rocznie
pv-sensitive-class-health = Zdrowie i przekonania
pv-sensitive-class-biometrics = Dane biometryczne
pv-sensitive-class-genomics = Dane genetyczne
pv-sensitive-class-legal = Karalność
pv-laterality-left = Lewe
pv-laterality-right = Prawe
pv-laterality-both = Obustronnie
pv-body-region-head = Głowa
pv-body-region-face = Twarz
pv-body-region-neck = Szyja
pv-body-region-left-shoulder = Lewy bark
pv-body-region-right-shoulder = Prawy bark
pv-body-region-left-arm = Lewa ręka
pv-body-region-right-arm = Prawa ręka
pv-body-region-left-hand = Lewa dłoń
pv-body-region-right-hand = Prawa dłoń
pv-body-region-chest = Klatka piersiowa
pv-body-region-abdomen = Brzuch
pv-body-region-upper-back = Górna część pleców
pv-body-region-lower-back = Dolna część pleców
pv-body-region-pelvis = Miednica i biodra
pv-body-region-left-leg = Lewa noga
pv-body-region-right-leg = Prawa noga
pv-body-region-left-foot = Lewa stopa
pv-body-region-right-foot = Prawa stopa
pv-body-region-internal = Wewnątrz ciała
pv-body-region-whole-body = Całe ciało
pv-body-region-other = Inna okolica
pv-artefact-type-mesh = Siatka 3D
pv-artefact-type-point-cloud = Chmura punktów
pv-artefact-type-skin-texture-map = Mapa tekstury skóry
pv-artefact-type-skeletal-rig = Szkielet animacyjny
pv-artefact-type-voice-corpus = Korpus nagrań głosu
pv-artefact-type-text-corpus = Korpus tekstów
pv-artefact-type-trace-archive = Archiwum aktywności w sieci
pv-artefact-type-behaviour-model = Model zachowania
pv-artefact-type-fingerprint-card = Karta daktyloskopijna
pv-artefact-type-fingerprint-template = Wzorzec odcisku palca
pv-artefact-type-retinal-image = Obraz siatkówki
pv-artefact-type-voiceprint = Odcisk głosu
pv-consent-given = Udzielona
pv-consent-given-by-estate = Udzielona przez spadkobierców
pv-consent-refused = Odmówiona
pv-consent-withdrawn = Wycofana
pv-consent-not-asked = Nie pytano
pv-consent-unknown = Nieznana
pv-sex-at-birth-female = Żeńska
pv-sex-at-birth-male = Męska
pv-sex-at-birth-intersex = Interpłciowa
pv-sex-at-birth-undetermined = Nieustalona
pv-sex-at-birth-unknown = Nieznana
pv-gender-identity-woman = Kobieta
pv-gender-identity-man = Mężczyzna
pv-gender-identity-non-binary = Osoba niebinarna
pv-gender-identity-other = Inna
pv-gender-identity-undisclosed = Nieujawniona
pv-gender-identity-unknown = Nieznana
pv-title-kind-nobility = Szlachecki
pv-title-kind-academic = Naukowy
pv-title-kind-professional = Zawodowy
pv-title-kind-religious = Religijny
pv-title-kind-military = Wojskowy
pv-title-kind-civic = Honorowy
pv-title-kind-courtesy = Grzecznościowy
pv-title-kind-other = Inny
pv-register-type-birth = Urodzenie
pv-register-type-baptism = Chrzest
pv-register-type-marriage = Małżeństwo
pv-register-type-death = Zgon
pv-register-type-burial = Pochówek
pv-register-type-divorce = Rozwód
pv-register-type-recognition = Uznanie dziecka
pv-register-type-legitimation = Legitymacja dziecka
pv-register-type-adoption = Przysposobienie
pv-register-type-name-change = Zmiana nazwiska
pv-register-type-other = Inny
pv-build-slight = Drobna
pv-build-slim = Szczupła
pv-build-average = Przeciętna
pv-build-sturdy = Krępa
pv-build-stout = Tęga
pv-build-heavy = Masywna
pv-eye-colour-light-blue = Jasnoniebieski
pv-eye-colour-blue = Niebieski
pv-eye-colour-dark-blue = Ciemnoniebieski
pv-eye-colour-grey = Szary
pv-eye-colour-blue-grey = Szaroniebieski
pv-eye-colour-green = Zielony
pv-eye-colour-grey-green = Szarozielony
pv-eye-colour-hazel = Piwny
pv-eye-colour-amber = Bursztynowy
pv-eye-colour-light-brown = Jasnobrązowy
pv-eye-colour-brown = Brązowy
pv-eye-colour-dark-brown = Ciemnobrązowy
pv-eye-colour-black = Czarny
pv-eye-colour-mixed = Mieszany
pv-eye-colour-other = Inny
pv-eye-shape-almond = Migdałowy
pv-eye-shape-round = Okrągły
pv-eye-shape-hooded = Z opadającą powieką
pv-eye-shape-monolid = Bez fałdy powieki
pv-eye-shape-deep-set = Głęboko osadzone
pv-eye-shape-protruding = Wyłupiaste
pv-eye-shape-upturned = Uniesione
pv-eye-shape-downturned = Opadające
pv-eye-shape-other = Inny
pv-eye-spacing-close-set = Blisko osadzone
pv-eye-spacing-average = Przeciętny
pv-eye-spacing-wide-set = Szeroko osadzone
pv-hair-colour-black = Czarny
pv-hair-colour-dark-brown = Ciemnobrązowy
pv-hair-colour-brown = Brązowy
pv-hair-colour-light-brown = Jasnobrązowy
pv-hair-colour-auburn = Kasztanowy
pv-hair-colour-red = Rudy
pv-hair-colour-strawberry-blond = Rudoblond
pv-hair-colour-dark-blond = Ciemny blond
pv-hair-colour-blond = Blond
pv-hair-colour-light-blond = Jasny blond
pv-hair-colour-grey = Siwy
pv-hair-colour-white = Biały
pv-hair-colour-none = Brak włosów
pv-hair-colour-other = Inny
pv-hair-texture-straight = Proste
pv-hair-texture-wavy = Falowane
pv-hair-texture-curly = Kręcone
pv-hair-texture-coily = Mocno skręcone
pv-hair-texture-other = Inne
pv-hairline-straight = Prosta
pv-hairline-rounded = Zaokrąglona
pv-hairline-widows-peak = Z wdowim szpicem
pv-hairline-m-shaped = W kształcie litery M
pv-hairline-bell-shaped = Dzwonowata
pv-hairline-uneven = Nierówna
pv-hairline-receding = Cofająca się
pv-hairline-bald = Łysina
pv-facial-hair-none = Brak
pv-facial-hair-stubble = Kilkudniowy zarost
pv-facial-hair-moustache = Wąsy
pv-facial-hair-goatee = Kozia bródka
pv-facial-hair-full-beard = Pełna broda
pv-facial-hair-sideburns = Bokobrody
pv-facial-hair-other = Inny
pv-body-hair-none = Brak
pv-body-hair-sparse = Rzadkie
pv-body-hair-moderate = Umiarkowane
pv-body-hair-dense = Gęste
pv-skin-tone-type-i = Typ I — zawsze się pali, nigdy nie opala
pv-skin-tone-type-ii = Typ II — zwykle się pali, słabo opala
pv-skin-tone-type-iii = Typ III — czasem się pali, opala równo
pv-skin-tone-type-iv = Typ IV — rzadko się pali, dobrze opala
pv-skin-tone-type-v = Typ V — bardzo rzadko się pali
pv-skin-tone-type-vi = Typ VI — nigdy się nie pali
pv-skin-undertone-cool = Chłodny
pv-skin-undertone-neutral = Neutralny
pv-skin-undertone-warm = Ciepły
pv-skin-undertone-olive = Oliwkowy
pv-freckles-none = Brak
pv-freckles-few = Nieliczne
pv-freckles-moderate = Umiarkowane
pv-freckles-many = Liczne
pv-pigmentation-mark-birthmark = Znamię wrodzone
pv-pigmentation-mark-port-wine-stain = Naczyniak płaski
pv-pigmentation-mark-cafe-au-lait-spot = Plama kawowa
pv-pigmentation-mark-depigmented-patch = Plama odbarwiona
pv-pigmentation-mark-hyperpigmented-patch = Plama przebarwiona
pv-pigmentation-mark-other = Inna
pv-mole-shape-round = Okrągłe
pv-mole-shape-oval = Owalne
pv-mole-shape-irregular = Nieregularne
pv-mole-shape-other = Inne
pv-face-shape-oval = Owalna
pv-face-shape-round = Okrągła
pv-face-shape-square = Kwadratowa
pv-face-shape-oblong = Pociągła
pv-face-shape-heart = W kształcie serca
pv-face-shape-diamond = Romboidalna
pv-face-shape-triangular = Trójkątna
pv-nose-shape-straight = Prosty
pv-nose-shape-aquiline = Orli
pv-nose-shape-snub = Zadarty krótki
pv-nose-shape-upturned = Zadarty
pv-nose-shape-flat = Płaski
pv-nose-shape-broad = Szeroki
pv-nose-shape-bulbous = Bulwiasty
pv-nose-shape-crooked = Skrzywiony
pv-nose-shape-other = Inny
pv-ear-shape-free-lobe = Wolne płatki
pv-ear-shape-attached-lobe = Przyrośnięte płatki
pv-ear-shape-protruding = Odstające
pv-ear-shape-close-set = Przylegające
pv-ear-shape-pointed = Spiczaste
pv-ear-shape-other = Inne
pv-lip-shape-thin = Wąskie
pv-lip-shape-medium = Średnie
pv-lip-shape-full = Pełne
pv-lip-shape-bow-shaped = W kształcie łuku
pv-lip-shape-wide = Szerokie
pv-lip-shape-downturned = Opadające kąciki
pv-lip-shape-other = Inne
pv-dentition-primary = Mleczne
pv-dentition-mixed = Mieszane
pv-dentition-permanent-complete = Stałe, pełne
pv-dentition-permanent-partial-loss = Stałe, z brakami
pv-dentition-edentulous = Bezzębie
pv-dentition-partial-denture = Proteza częściowa
pv-dentition-full-denture = Proteza całkowita
pv-dentition-implants = Implanty zębowe
pv-malocclusion-normal = Zgryz prawidłowy
pv-malocclusion-class-i = Klasa I
pv-malocclusion-class-ii-division-1 = Klasa II, podklasa 1
pv-malocclusion-class-ii-division-2 = Klasa II, podklasa 2
pv-malocclusion-class-iii = Klasa III
pv-posture-ideal = Prawidłowa
pv-posture-kyphotic-lordotic = Kifotyczno-lordotyczna
pv-posture-flat-back = Plecy płaskie
pv-posture-sway-back = Plecy wklęsło-okrągłe
pv-posture-stooped = Pochylona
pv-posture-scoliotic = Skoliotyczna
pv-posture-other = Inna
pv-gait-brisk = Żwawy
pv-gait-average = Przeciętny
pv-gait-slow = Powolny
pv-gait-shuffling = Powłóczący
pv-gait-limping = Utykający
pv-gait-waddling = Kaczy
pv-gait-unsteady = Chwiejny
pv-gait-stiff = Sztywny
pv-gait-other = Inny
pv-vocal-timbre-bright = Jasna
pv-vocal-timbre-dark = Ciemna
pv-vocal-timbre-warm = Ciepła
pv-vocal-timbre-breathy = Z przydechem
pv-vocal-timbre-nasal = Nosowa
pv-vocal-timbre-hoarse = Chrapliwa
pv-vocal-timbre-resonant = Dźwięczna
pv-vocal-timbre-thin = Cienka
pv-vocal-timbre-other = Inna
pv-speech-register-frozen = Rytualny
pv-speech-register-formal = Oficjalny
pv-speech-register-consultative = Rzeczowy
pv-speech-register-casual = Potoczny
pv-speech-register-intimate = Intymny
pv-handedness-left = Leworęczność
pv-handedness-right = Praworęczność
pv-handedness-ambidextrous = Oburęczność
pv-handedness-mixed = Mieszana
pv-handedness-unknown = Nieznana
pv-hearing-grade-normal = Prawidłowy
pv-hearing-grade-mild = Lekki ubytek
pv-hearing-grade-moderate = Umiarkowany ubytek
pv-hearing-grade-moderately-severe = Umiarkowanie ciężki ubytek
pv-hearing-grade-severe = Ciężki ubytek
pv-hearing-grade-profound = Głęboki ubytek
pv-hearing-grade-complete = Całkowita głuchota
pv-optical-correction-none = Brak
pv-optical-correction-glasses = Okulary
pv-optical-correction-contact-lenses = Soczewki kontaktowe
pv-optical-correction-glasses-and-contact-lenses = Okulary i soczewki
pv-optical-correction-refractive-surgery = Chirurgia refrakcyjna
pv-optical-correction-intraocular-lens = Soczewka wewnątrzgałkowa
pv-optical-correction-other = Inna
pv-rhesus-positive = RhD dodatni
pv-rhesus-negative = RhD ujemny
pv-rhesus-weak-d = Słaby antygen D
pv-rhesus-unknown = Nieznany
pv-icd10-chapter-infectious-parasitic = I Choroby zakaźne i pasożytnicze
pv-icd10-chapter-neoplasms = II Nowotwory
pv-icd10-chapter-blood-immune = III Choroby krwi i układu odpornościowego
pv-icd10-chapter-endocrine-metabolic = IV Choroby endokrynologiczne, żywieniowe i metaboliczne
pv-icd10-chapter-mental-behavioural = V Zaburzenia psychiczne i zachowania
pv-icd10-chapter-nervous-system = VI Choroby układu nerwowego
pv-icd10-chapter-eye-adnexa = VII Choroby oka i przydatków
pv-icd10-chapter-ear-mastoid = VIII Choroby ucha i wyrostka sutkowatego
pv-icd10-chapter-circulatory = IX Choroby układu krążenia
pv-icd10-chapter-respiratory = X Choroby układu oddechowego
pv-icd10-chapter-digestive = XI Choroby układu trawiennego
pv-icd10-chapter-skin = XII Choroby skóry i tkanki podskórnej
pv-icd10-chapter-musculoskeletal = XIII Choroby układu mięśniowo-szkieletowego
pv-icd10-chapter-genitourinary = XIV Choroby układu moczowo-płciowego
pv-icd10-chapter-pregnancy-childbirth = XV Ciąża, poród i połóg
pv-icd10-chapter-perinatal = XVI Stany okresu okołoporodowego
pv-icd10-chapter-congenital = XVII Wady rozwojowe wrodzone
pv-icd10-chapter-ill-defined = XVIII Objawy i przyczyny nieokreślone
pv-icd10-chapter-injury-poisoning = XIX Urazy i zatrucia
pv-icd10-chapter-external-causes = XX Zewnętrzne przyczyny
pv-icd10-chapter-health-factors = XXI Czynniki wpływające na stan zdrowia
pv-icd10-chapter-special-purposes = XXII Kody do celów specjalnych
pv-diagnosis-status-diagnosed = Rozpoznana przez lekarza
pv-diagnosis-status-suspected = Podejrzewana
pv-diagnosis-status-self-reported = Zgłoszona przez rodzinę
pv-diagnosis-status-unknown = Nieznany
pv-prosthesis-kind-limb = Kończyna
pv-prosthesis-kind-joint = Endoproteza stawu
pv-prosthesis-kind-ocular = Oczna
pv-prosthesis-kind-dental = Zębowa
pv-prosthesis-kind-auditory = Słuchowa
pv-prosthesis-kind-breast = Piersi
pv-prosthesis-kind-other = Inna
pv-implant-kind-orthopaedic = Ortopedyczny
pv-implant-kind-dental = Zębowy
pv-implant-kind-cochlear = Ślimakowy
pv-implant-kind-breast = Piersi
pv-implant-kind-intraocular-lens = Soczewka wewnątrzgałkowa
pv-implant-kind-contraceptive = Antykoncepcyjny
pv-implant-kind-cosmetic = Kosmetyczny
pv-implant-kind-other = Inny
pv-device-kind-pacemaker = Rozrusznik serca
pv-device-kind-implantable-defibrillator = Wszczepialny defibrylator
pv-device-kind-cardiac-resynchronisation = Urządzenie resynchronizujące
pv-device-kind-ventricular-assist = Urządzenie wspomagania komór
pv-device-kind-neurostimulator = Neurostymulator
pv-device-kind-insulin-pump = Pompa insulinowa
pv-device-kind-drug-port = Port naczyniowy
pv-device-kind-shunt = Zastawka
pv-device-kind-stent = Stent
pv-device-kind-other = Inne
pv-allergy-type-drug = Lek
pv-allergy-type-food = Pokarm
pv-allergy-type-environmental = Czynnik środowiskowy
pv-allergy-type-insect-venom = Jad owadów
pv-allergy-type-latex = Lateks
pv-allergy-type-other = Inny
pv-allergy-severity-mild = Łagodna
pv-allergy-severity-moderate = Umiarkowana
pv-allergy-severity-severe = Ciężka
pv-allergy-severity-anaphylactic = Anafilaktyczna
pv-allergy-severity-unknown = Nieznana
pv-pathogen-diphtheria = Błonica
pv-pathogen-tetanus = Tężec
pv-pathogen-pertussis = Krztusiec
pv-pathogen-poliomyelitis = Polio
pv-pathogen-measles = Odra
pv-pathogen-mumps = Świnka
pv-pathogen-rubella = Różyczka
pv-pathogen-varicella = Ospa wietrzna
pv-pathogen-smallpox = Ospa prawdziwa
pv-pathogen-tuberculosis = Gruźlica
pv-pathogen-hepatitis-a = WZW typu A
pv-pathogen-hepatitis-b = WZW typu B
pv-pathogen-hepatitis-c = WZW typu C
pv-pathogen-haemophilus-influenzae-b = Haemophilus influenzae typu b
pv-pathogen-pneumococcal = Zakażenia pneumokokowe
pv-pathogen-meningococcal = Zakażenia meningokokowe
pv-pathogen-human-papillomavirus = Wirus brodawczaka ludzkiego
pv-pathogen-influenza = Grypa
pv-pathogen-covid-19 = COVID-19
pv-pathogen-rotavirus = Rotawirusy
pv-pathogen-yellow-fever = Żółta gorączka
pv-pathogen-typhoid = Dur brzuszny
pv-pathogen-cholera = Cholera
pv-pathogen-rabies = Wścieklizna
pv-pathogen-japanese-encephalitis = Japońskie zapalenie mózgu
pv-pathogen-tick-borne-encephalitis = Kleszczowe zapalenie mózgu
pv-pathogen-hiv = HIV
pv-pathogen-syphilis = Kiła
pv-pathogen-toxoplasmosis = Toksoplazmoza
pv-pathogen-cytomegalovirus = Cytomegalowirus
pv-pathogen-epstein-barr = Wirus Epsteina-Barr
pv-pathogen-other = Inny
pv-vaccination-status-vaccinated = Zaszczepiony
pv-vaccination-status-partially-vaccinated = Zaszczepiony częściowo
pv-vaccination-status-unvaccinated = Niezaszczepiony
pv-vaccination-status-contraindicated = Przeciwwskazane
pv-vaccination-status-unknown = Nieznany
pv-serology-result-positive = Dodatni
pv-serology-result-negative = Ujemny
pv-serology-result-equivocal = Niejednoznaczny
pv-serology-result-unknown = Nieznany
pv-lab-panel-basic-metabolic = Podstawowy panel metaboliczny
pv-lab-panel-lipid = Lipidogram
pv-lab-panel-liver = Próby wątrobowe
pv-lab-panel-renal = Parametry nerkowe
pv-lab-panel-glycated-haemoglobin = Hemoglobina glikowana
pv-lab-panel-iron = Gospodarka żelazem
pv-lab-analyte-sodium = Sód
pv-lab-analyte-potassium = Potas
pv-lab-analyte-chloride = Chlorki
pv-lab-analyte-bicarbonate = Wodorowęglany
pv-lab-analyte-urea = Mocznik
pv-lab-analyte-creatinine = Kreatynina
pv-lab-analyte-glucose = Glukoza
pv-lab-analyte-calcium = Wapń
pv-lab-analyte-total-cholesterol = Cholesterol całkowity
pv-lab-analyte-ldl-cholesterol = Cholesterol LDL
pv-lab-analyte-hdl-cholesterol = Cholesterol HDL
pv-lab-analyte-triglycerides = Triglicerydy
pv-lab-analyte-non-hdl-cholesterol = Cholesterol nie-HDL
pv-lab-analyte-alt = Aminotransferaza alaninowa (ALT)
pv-lab-analyte-ast = Aminotransferaza asparaginianowa (AST)
pv-lab-analyte-alp = Fosfataza zasadowa (ALP)
pv-lab-analyte-ggt = Gamma-glutamylotransferaza (GGTP)
pv-lab-analyte-total-bilirubin = Bilirubina całkowita
pv-lab-analyte-direct-bilirubin = Bilirubina bezpośrednia
pv-lab-analyte-albumin = Albumina
pv-lab-analyte-total-protein = Białko całkowite
pv-lab-analyte-egfr = eGFR
pv-lab-analyte-uric-acid = Kwas moczowy
pv-lab-analyte-phosphate = Fosforany
pv-lab-analyte-urine-albumin-creatinine-ratio = Stosunek albuminy do kreatyniny w moczu
pv-lab-analyte-hba1c = HbA1c
pv-lab-analyte-serum-iron = Żelazo w surowicy
pv-lab-analyte-ferritin = Ferrytyna
pv-lab-analyte-transferrin = Transferyna
pv-lab-analyte-transferrin-saturation = Wysycenie transferyny
pv-lab-analyte-tibc = Całkowita zdolność wiązania żelaza
pv-lab-flag-low = Poniżej normy
pv-lab-flag-normal = W normie
pv-lab-flag-high = Powyżej normy
pv-lab-flag-critical-low = Krytycznie niski
pv-lab-flag-critical-high = Krytycznie wysoki
pv-nutrient-vitamin-a = Witamina A
pv-nutrient-thiamine = Tiamina (B1)
pv-nutrient-riboflavin = Ryboflawina (B2)
pv-nutrient-niacin = Niacyna (B3)
pv-nutrient-vitamin-b6 = Witamina B6
pv-nutrient-folate = Kwas foliowy (B9)
pv-nutrient-vitamin-b12 = Witamina B12
pv-nutrient-vitamin-c = Witamina C
pv-nutrient-vitamin-d = Witamina D
pv-nutrient-vitamin-e = Witamina E
pv-nutrient-vitamin-k = Witamina K
pv-nutrient-iron = Żelazo
pv-nutrient-zinc = Cynk
pv-nutrient-magnesium = Magnez
pv-nutrient-calcium = Wapń
pv-nutrient-iodine = Jod
pv-nutrient-selenium = Selen
pv-nutrient-copper = Miedź
pv-nutrient-potassium = Potas
pv-nutrient-phosphorus = Fosfor
pv-nutrient-other = Inny
pv-sleep-disorder-insomnia = Bezsenność
pv-sleep-disorder-sleep-related-breathing = Zaburzenia oddychania w czasie snu
pv-sleep-disorder-central-hypersomnolence = Hipersomnia ośrodkowa
pv-sleep-disorder-circadian-rhythm = Zaburzenia rytmu okołodobowego
pv-sleep-disorder-parasomnia = Parasomnie
pv-sleep-disorder-sleep-related-movement = Ruchowe zaburzenia snu
pv-sleep-disorder-other = Inne
pv-assessment-instrument-phq-9 = PHQ-9
pv-assessment-instrument-gad-7 = GAD-7
pv-assessment-instrument-bdi-ii = BDI-II
pv-assessment-instrument-hads = HADS
pv-assessment-instrument-k10 = K10
pv-assessment-instrument-gds-15 = GDS-15
pv-assessment-instrument-mmse = MMSE
pv-assessment-instrument-moca = MoCA
pv-assessment-instrument-audit = AUDIT
pv-assessment-instrument-clinical-interview = Wywiad kliniczny
pv-assessment-instrument-other = Inne
pv-assessment-severity-none-minimal = Brak lub minimalne
pv-assessment-severity-mild = Łagodne
pv-assessment-severity-moderate = Umiarkowane
pv-assessment-severity-moderately-severe = Umiarkowanie ciężkie
pv-assessment-severity-severe = Ciężkie
pv-reference-build-grch36 = GRCh36 (hg18)
pv-reference-build-grch37 = GRCh37 (hg19)
pv-reference-build-grch38 = GRCh38 (hg38)
pv-reference-build-t2t-chm13 = T2T-CHM13
pv-genomic-file-format-raw-microarray = Surowe dane z mikromacierzy
pv-genomic-file-format-fastq = FASTQ
pv-genomic-file-format-bam = BAM
pv-genomic-file-format-cram = CRAM
pv-genomic-file-format-vcf = VCF
pv-genomic-file-format-gvcf = gVCF
pv-genomic-file-format-other = Inny
pv-zygosity-heterozygous = Heterozygota
pv-zygosity-homozygous = Homozygota
pv-zygosity-hemizygous = Hemizygota
pv-zygosity-compound-heterozygous = Złożona heterozygota
pv-clinical-significance-pathogenic = Patogenny
pv-clinical-significance-likely-pathogenic = Prawdopodobnie patogenny
pv-clinical-significance-uncertain-significance = O niepewnym znaczeniu
pv-clinical-significance-likely-benign = Prawdopodobnie łagodny
pv-clinical-significance-benign = Łagodny
pv-inheritance-pattern-autosomal-dominant = Autosomalny dominujący
pv-inheritance-pattern-autosomal-recessive = Autosomalny recesywny
pv-inheritance-pattern-x-linked-dominant = Dominujący sprzężony z X
pv-inheritance-pattern-x-linked-recessive = Recesywny sprzężony z X
pv-inheritance-pattern-y-linked = Sprzężony z Y
pv-inheritance-pattern-mitochondrial = Mitochondrialny
pv-inheritance-pattern-multifactorial = Wieloczynnikowy
pv-inheritance-pattern-unknown = Nieznany
pv-carrier-status-affected = Chory
pv-carrier-status-carrier = Nosiciel
pv-carrier-status-not-carrier = Nie jest nosicielem
pv-carrier-status-unknown = Nieznane
pv-epigenetic-clock-horvath = Horvath
pv-epigenetic-clock-hannum = Hannum
pv-epigenetic-clock-phenoage = PhenoAge
pv-epigenetic-clock-grimage = GrimAge
pv-epigenetic-clock-dunedinpace = DunedinPACE
pv-epigenetic-clock-other = Inny
pv-metaboliser-status-poor = Wolny metabolizm
pv-metaboliser-status-intermediate = Pośredni metabolizm
pv-metaboliser-status-normal = Prawidłowy metabolizm
pv-metaboliser-status-rapid = Szybki metabolizm
pv-metaboliser-status-ultrarapid = Ultraszybki metabolizm
pv-autopsy-not-performed = Nie przeprowadzono
pv-autopsy-clinical = Kliniczna
pv-autopsy-forensic = Sądowo-lekarska
pv-autopsy-external-examination = Tylko oględziny zewnętrzne
pv-autopsy-unknown = Nieznana
pv-disposition-burial = Pochówek w ziemi
pv-disposition-cremation = Kremacja
pv-disposition-entombment = Pochówek w grobowcu
pv-disposition-burial-at-sea = Pochówek w morzu
pv-disposition-natural-burial = Pochówek naturalny
pv-disposition-body-donation = Przekazanie ciała na cele naukowe
pv-disposition-other = Inne
pv-disposition-unknown = Nieznane
pv-address-use-principal = Miejsce stałego zamieszkania
pv-address-use-secondary = Drugie miejsce zamieszkania
pv-address-use-temporary = Pobyt czasowy
pv-address-use-postal = Adres korespondencyjny
pv-address-use-other = Inny
pv-nationality-mode-descent = Przez urodzenie z obywatela
pv-nationality-mode-birth-in-territory = Przez urodzenie na terytorium
pv-nationality-mode-naturalisation = Przez naturalizację
pv-nationality-mode-marriage = Przez małżeństwo
pv-nationality-mode-registration = Przez oświadczenie
pv-nationality-mode-restoration = Przez przywrócenie
pv-nationality-mode-state-succession = Przez zmianę granic państwa
pv-nationality-mode-other = Inny
pv-language-proficiency-a1 = A1 Początkujący
pv-language-proficiency-a2 = A2 Podstawowy
pv-language-proficiency-b1 = B1 Średnio zaawansowany
pv-language-proficiency-b2 = B2 Wyższy średnio zaawansowany
pv-language-proficiency-c1 = C1 Zaawansowany
pv-language-proficiency-c2 = C2 Biegły
pv-language-proficiency-native = Pierwszy język
pv-isced-level-isced-0 = 0 Wczesna edukacja
pv-isced-level-isced-1 = 1 Szkoła podstawowa
pv-isced-level-isced-2 = 2 Gimnazjum lub klasy 7–8
pv-isced-level-isced-3 = 3 Szkoła średnia
pv-isced-level-isced-4 = 4 Policealne
pv-isced-level-isced-5 = 5 Krótkie studia wyższe
pv-isced-level-isced-6 = 6 Licencjat lub równorzędne
pv-isced-level-isced-7 = 7 Magisterium lub równorzędne
pv-isced-level-isced-8 = 8 Doktorat lub równorzędny
pv-income-quintile-q1 = Najniższa piąta część
pv-income-quintile-q2 = Druga piąta część
pv-income-quintile-q3 = Środkowa piąta część
pv-income-quintile-q4 = Czwarta piąta część
pv-income-quintile-q5 = Najwyższa piąta część
pv-pay-period-hourly = Za godzinę
pv-pay-period-daily = Dziennie
pv-pay-period-weekly = Tygodniowo
pv-pay-period-monthly = Miesięcznie
pv-pay-period-annual = Rocznie
pv-tenure-owned = Własność
pv-tenure-co-owned = Współwłasność
pv-tenure-leasehold = Użytkowanie wieczyste
pv-tenure-rented = Najem
pv-tenure-usufruct = Użytkowanie
pv-tenure-other = Inny
pv-distinction-kind-order = Order
pv-distinction-kind-decoration = Odznaczenie
pv-distinction-kind-medal = Medal
pv-distinction-kind-title = Tytuł honorowy
pv-distinction-kind-other = Inne
pv-military-service-army = Wojska lądowe
pv-military-service-navy = Marynarka wojenna
pv-military-service-air-force = Siły powietrzne
pv-military-service-marines = Piechota morska
pv-military-service-gendarmerie = Żandarmeria
pv-military-service-border-guard = Straż graniczna
pv-military-service-national-guard = Wojska obrony terytorialnej
pv-military-service-other = Inne
pv-rank-category-enlisted = Szeregowi
pv-rank-category-non-commissioned = Podoficerowie
pv-rank-category-warrant = Chorążowie
pv-rank-category-officer-cadet = Podchorążowie
pv-rank-category-junior-officer = Oficerowie młodsi
pv-rank-category-senior-officer = Oficerowie starsi
pv-rank-category-general-officer = Generałowie
pv-iccs-section-acts-leading-to-death = 01 Czyny prowadzące do śmierci
pv-iccs-section-acts-causing-harm = 02 Czyny wyrządzające krzywdę
pv-iccs-section-sexual-acts = 03 Czyny o charakterze seksualnym
pv-iccs-section-property-with-violence = 04 Przeciwko mieniu z użyciem przemocy
pv-iccs-section-property-only = 05 Przeciwko mieniu
pv-iccs-section-controlled-substances = 06 Substancje kontrolowane
pv-iccs-section-fraud-deception-corruption = 07 Oszustwo lub korupcja
pv-iccs-section-public-order-and-state = 08 Przeciwko porządkowi publicznemu i państwu
pv-iccs-section-public-safety-and-security = 09 Przeciwko bezpieczeństwu publicznemu
pv-iccs-section-natural-environment = 10 Przeciwko środowisku
pv-iccs-section-other-criminal-acts = 11 Inne przestępstwa
pv-case-outcome-convicted = Skazany
pv-case-outcome-acquitted = Uniewinniony
pv-case-outcome-dismissed = Umorzone
pv-case-outcome-conviction-quashed = Wyrok uchylony
pv-case-outcome-pardoned = Ułaskawiony
pv-case-outcome-amnestied = Objęty amnestią
pv-case-outcome-expunged = Zatarcie skazania
pv-case-outcome-pending = W toku
pv-case-outcome-unknown = Nieznane
pv-religion-buddhism = Buddyzm
pv-religion-christianity-catholic = Chrześcijaństwo: katolicyzm
pv-religion-christianity-orthodox = Chrześcijaństwo: prawosławie
pv-religion-christianity-protestant = Chrześcijaństwo: protestantyzm
pv-religion-christianity-other = Chrześcijaństwo: inne
pv-religion-hinduism = Hinduizm
pv-religion-islam-sunni = Islam: sunnizm
pv-religion-islam-shia = Islam: szyizm
pv-religion-islam-other = Islam: inne
pv-religion-jainism = Dżinizm
pv-religion-judaism = Judaizm
pv-religion-sikhism = Sikhizm
pv-religion-bahai = Bahaizm
pv-religion-shinto = Sintoizm
pv-religion-taoism = Taoizm
pv-religion-zoroastrianism = Zaratusztrianizm
pv-religion-traditional = Religia ludowa lub tradycyjna
pv-religion-other = Inna
pv-religion-none = Bezwyznaniowość
pv-religion-unknown = Nieznane
pv-sacrament-baptism = Chrzest
pv-sacrament-confirmation = Bierzmowanie
pv-sacrament-first-communion = Pierwsza komunia
pv-sacrament-reconciliation = Spowiedź
pv-sacrament-anointing-of-the-sick = Namaszczenie chorych
pv-sacrament-holy-orders = Święcenia
pv-sacrament-matrimony = Małżeństwo
pv-sacrament-other-rite = Inny obrzęd
pv-political-position-far-left = Skrajna lewica
pv-political-position-left = Lewica
pv-political-position-centre-left = Centrolewica
pv-political-position-centre = Centrum
pv-political-position-centre-right = Centroprawica
pv-political-position-right = Prawica
pv-political-position-far-right = Skrajna prawica
pv-political-position-apolitical = Apolityczność
pv-political-position-other = Poza tą osią
pv-political-position-unknown = Nieznane
pv-membership-kind-trade-union = Związek zawodowy
pv-membership-kind-political-party = Partia polityczna
pv-membership-kind-professional-body = Samorząd zawodowy
pv-membership-kind-religious-order = Zakon
pv-membership-kind-religious-association = Stowarzyszenie religijne
pv-membership-kind-fraternal-order = Bractwo
pv-membership-kind-veterans-association = Związek kombatantów
pv-membership-kind-sports-club = Klub sportowy
pv-membership-kind-cultural-association = Towarzystwo kulturalne
pv-membership-kind-charitable-association = Towarzystwo dobroczynne
pv-membership-kind-other = Inna
pv-personality-instrument-neo-pi-3 = NEO-PI-3
pv-personality-instrument-neo-ffi-3 = NEO-FFI-3
pv-personality-instrument-bfi-2 = BFI-2
pv-personality-instrument-ipip-neo-120 = IPIP-NEO-120
pv-personality-instrument-tipi = TIPI
pv-personality-instrument-hexaco-pi-r = HEXACO-PI-R
pv-personality-instrument-observer-rating = Ocena osoby, która ją znała
pv-personality-instrument-inferred = Wywnioskowane z zapisów
pv-personality-instrument-other = Inne
pv-introversion-extraversion-strongly-introverted = Wyraźny introwertyk
pv-introversion-extraversion-introverted = Introwertyk
pv-introversion-extraversion-ambiverted = Ambiwertyk
pv-introversion-extraversion-extraverted = Ekstrawertyk
pv-introversion-extraversion-strongly-extraverted = Wyraźny ekstrawertyk
pv-stress-tolerance-very-low = Bardzo niska
pv-stress-tolerance-low = Niska
pv-stress-tolerance-moderate = Umiarkowana
pv-stress-tolerance-high = Wysoka
pv-stress-tolerance-very-high = Bardzo wysoka
pv-decision-style-rational = Racjonalny
pv-decision-style-intuitive = Intuicyjny
pv-decision-style-dependent = Zależny
pv-decision-style-avoidant = Unikający
pv-decision-style-spontaneous = Spontaniczny
pv-sport-level-recreational = Rekreacyjny
pv-sport-level-amateur-competitive = Amatorski wyczynowy
pv-sport-level-semi-professional = Półzawodowy
pv-sport-level-professional = Zawodowy
pv-diet-omnivore = Wszystkożerna
pv-diet-flexitarian = Fleksitariańska
pv-diet-pescatarian = Peskatariańska
pv-diet-vegetarian = Wegetariańska
pv-diet-vegan = Wegańska
pv-diet-other = Inna
pv-substance-tobacco = Tytoń i nikotyna
pv-substance-alcohol = Alkohol
pv-substance-cannabis = Konopie
pv-substance-opioids = Opioidy
pv-substance-stimulants = Stymulanty
pv-substance-sedatives-hypnotics = Leki uspokajające i nasenne
pv-substance-hallucinogens = Halucynogeny
pv-substance-inhalants = Środki wziewne
pv-substance-gambling = Hazard
pv-substance-gaming = Gry komputerowe
pv-substance-other = Inne
pv-use-pattern-occasional-use = Okazjonalne używanie
pv-use-pattern-regular-use = Regularne używanie
pv-use-pattern-harmful-use = Szkodliwe używanie
pv-use-pattern-dependence = Uzależnienie
pv-use-pattern-in-remission = W remisji
pv-lineage-biological = Biologiczne
pv-lineage-adoptive = Adopcyjne
pv-lineage-foster = Zastępcze
pv-lineage-step = Przybrane
pv-lineage-guardianship = Opieka prawna
pv-lineage-unknown = Nieznane
pv-link-relation-godparent = Rodzic chrzestny
pv-link-relation-godchild = Chrześniak
pv-link-relation-witness = Świadek
pv-link-relation-officiant = Celebrans
pv-link-relation-business-partner = Wspólnik
pv-link-relation-employer = Pracodawca
pv-link-relation-employee = Pracownik
pv-link-relation-mentor = Mentor
pv-link-relation-apprentice = Uczeń
pv-link-relation-close-friend = Bliski przyjaciel
pv-link-relation-neighbour = Sąsiad
pv-link-relation-guardian = Opiekun
pv-link-relation-ward = Podopieczny
pv-link-relation-other = Inna
pv-country-AD = Andora
pv-country-AE = Zjednoczone Emiraty Arabskie
pv-country-AF = Afganistan
pv-country-AG = Antigua i Barbuda
pv-country-AI = Anguilla
pv-country-AL = Albania
pv-country-AM = Armenia
pv-country-AO = Angola
pv-country-AQ = Antarktyda
pv-country-AR = Argentyna
pv-country-AS = Samoa Amerykańskie
pv-country-AT = Austria
pv-country-AU = Australia
pv-country-AW = Aruba
pv-country-AX = Wyspy Alandzkie
pv-country-AZ = Azerbejdżan
pv-country-BA = Bośnia i Hercegowina
pv-country-BB = Barbados
pv-country-BD = Bangladesz
pv-country-BE = Belgia
pv-country-BF = Burkina Faso
pv-country-BG = Bułgaria
pv-country-BH = Bahrajn
pv-country-BI = Burundi
pv-country-BJ = Benin
pv-country-BL = Saint-Barthélemy
pv-country-BM = Bermudy
pv-country-BN = Brunei
pv-country-BO = Boliwia
pv-country-BQ = Niderlandy Karaibskie
pv-country-BR = Brazylia
pv-country-BS = Bahamy
pv-country-BT = Bhutan
pv-country-BV = Wyspa Bouveta
pv-country-BW = Botswana
pv-country-BY = Białoruś
pv-country-BZ = Belize
pv-country-CA = Kanada
pv-country-CC = Wyspy Kokosowe
pv-country-CD = Demokratyczna Republika Konga
pv-country-CF = Republika Środkowoafrykańska
pv-country-CG = Kongo
pv-country-CH = Szwajcaria
pv-country-CI = Côte d’Ivoire
pv-country-CK = Wyspy Cooka
pv-country-CL = Chile
pv-country-CM = Kamerun
pv-country-CN = Chiny
pv-country-CO = Kolumbia
pv-country-CR = Kostaryka
pv-country-CU = Kuba
pv-country-CV = Republika Zielonego Przylądka
pv-country-CW = Curaçao
pv-country-CX = Wyspa Bożego Narodzenia
pv-country-CY = Cypr
pv-country-CZ = Czechy
pv-country-DE = Niemcy
pv-country-DJ = Dżibuti
pv-country-DK = Dania
pv-country-DM = Dominika
pv-country-DO = Dominikana
pv-country-DZ = Algieria
pv-country-EC = Ekwador
pv-country-EE = Estonia
pv-country-EG = Egipt
pv-country-EH = Sahara Zachodnia
pv-country-ER = Erytrea
pv-country-ES = Hiszpania
pv-country-ET = Etiopia
pv-country-FI = Finlandia
pv-country-FJ = Fidżi
pv-country-FK = Falklandy
pv-country-FM = Mikronezja
pv-country-FO = Wyspy Owcze
pv-country-FR = Francja
pv-country-GA = Gabon
pv-country-GB = Wielka Brytania
pv-country-GD = Grenada
pv-country-GE = Gruzja
pv-country-GF = Gujana Francuska
pv-country-GG = Guernsey
pv-country-GH = Ghana
pv-country-GI = Gibraltar
pv-country-GL = Grenlandia
pv-country-GM = Gambia
pv-country-GN = Gwinea
pv-country-GP = Gwadelupa
pv-country-GQ = Gwinea Równikowa
pv-country-GR = Grecja
pv-country-GS = Georgia Południowa i Sandwich Południowy
pv-country-GT = Gwatemala
pv-country-GU = Guam
pv-country-GW = Gwinea Bissau
pv-country-GY = Gujana
pv-country-HK = SRA Hongkong (Chiny)
pv-country-HM = Wyspy Heard i McDonalda
pv-country-HN = Honduras
pv-country-HR = Chorwacja
pv-country-HT = Haiti
pv-country-HU = Węgry
pv-country-ID = Indonezja
pv-country-IE = Irlandia
pv-country-IL = Izrael
pv-country-IM = Wyspa Man
pv-country-IN = Indie
pv-country-IO = Brytyjskie Terytorium Oceanu Indyjskiego
pv-country-IQ = Irak
pv-country-IR = Iran
pv-country-IS = Islandia
pv-country-IT = Włochy
pv-country-JE = Jersey
pv-country-JM = Jamajka
pv-country-JO = Jordania
pv-country-JP = Japonia
pv-country-KE = Kenia
pv-country-KG = Kirgistan
pv-country-KH = Kambodża
pv-country-KI = Kiribati
pv-country-KM = Komory
pv-country-KN = Saint Kitts i Nevis
pv-country-KP = Korea Północna
pv-country-KR = Korea Południowa
pv-country-KW = Kuwejt
pv-country-KY = Kajmany
pv-country-KZ = Kazachstan
pv-country-LA = Laos
pv-country-LB = Liban
pv-country-LC = Saint Lucia
pv-country-LI = Liechtenstein
pv-country-LK = Sri Lanka
pv-country-LR = Liberia
pv-country-LS = Lesotho
pv-country-LT = Litwa
pv-country-LU = Luksemburg
pv-country-LV = Łotwa
pv-country-LY = Libia
pv-country-MA = Maroko
pv-country-MC = Monako
pv-country-MD = Mołdawia
pv-country-ME = Czarnogóra
pv-country-MF = Saint-Martin
pv-country-MG = Madagaskar
pv-country-MH = Wyspy Marshalla
pv-country-MK = Macedonia Północna
pv-country-ML = Mali
pv-country-MM = Mjanma (Birma)
pv-country-MN = Mongolia
pv-country-MO = SRA Makau (Chiny)
pv-country-MP = Mariany Północne
pv-country-MQ = Martynika
pv-country-MR = Mauretania
pv-country-MS = Montserrat
pv-country-MT = Malta
pv-country-MU = Mauritius
pv-country-MV = Malediwy
pv-country-MW = Malawi
pv-country-MX = Meksyk
pv-country-MY = Malezja
pv-country-MZ = Mozambik
pv-country-NA = Namibia
pv-country-NC = Nowa Kaledonia
pv-country-NE = Niger
pv-country-NF = Norfolk
pv-country-NG = Nigeria
pv-country-NI = Nikaragua
pv-country-NL = Holandia
pv-country-NO = Norwegia
pv-country-NP = Nepal
pv-country-NR = Nauru
pv-country-NU = Niue
pv-country-NZ = Nowa Zelandia
pv-country-OM = Oman
pv-country-PA = Panama
pv-country-PE = Peru
pv-country-PF = Polinezja Francuska
pv-country-PG = Papua-Nowa Gwinea
pv-country-PH = Filipiny
pv-country-PK = Pakistan
pv-country-PL = Polska
pv-country-PM = Saint-Pierre i Miquelon
pv-country-PN = Pitcairn
pv-country-PR = Portoryko
pv-country-PS = Terytoria Palestyńskie
pv-country-PT = Portugalia
pv-country-PW = Palau
pv-country-PY = Paragwaj
pv-country-QA = Katar
pv-country-RE = Reunion
pv-country-RO = Rumunia
pv-country-RS = Serbia
pv-country-RU = Rosja
pv-country-RW = Rwanda
pv-country-SA = Arabia Saudyjska
pv-country-SB = Wyspy Salomona
pv-country-SC = Seszele
pv-country-SD = Sudan
pv-country-SE = Szwecja
pv-country-SG = Singapur
pv-country-SH = Wyspa Świętej Heleny
pv-country-SI = Słowenia
pv-country-SJ = Svalbard i Jan Mayen
pv-country-SK = Słowacja
pv-country-SL = Sierra Leone
pv-country-SM = San Marino
pv-country-SN = Senegal
pv-country-SO = Somalia
pv-country-SR = Surinam
pv-country-SS = Sudan Południowy
pv-country-ST = Wyspy Świętego Tomasza i Książęca
pv-country-SV = Salwador
pv-country-SX = Sint Maarten
pv-country-SY = Syria
pv-country-SZ = Eswatini
pv-country-TC = Turks i Caicos
pv-country-TD = Czad
pv-country-TF = Francuskie Terytoria Południowe i Antarktyczne
pv-country-TG = Togo
pv-country-TH = Tajlandia
pv-country-TJ = Tadżykistan
pv-country-TK = Tokelau
pv-country-TL = Timor Wschodni
pv-country-TM = Turkmenistan
pv-country-TN = Tunezja
pv-country-TO = Tonga
pv-country-TR = Turcja
pv-country-TT = Trynidad i Tobago
pv-country-TV = Tuvalu
pv-country-TW = Tajwan
pv-country-TZ = Tanzania
pv-country-UA = Ukraina
pv-country-UG = Uganda
pv-country-UM = Dalekie Wyspy Mniejsze Stanów Zjednoczonych
pv-country-US = Stany Zjednoczone
pv-country-UY = Urugwaj
pv-country-UZ = Uzbekistan
pv-country-VA = Watykan
pv-country-VC = Saint Vincent i Grenadyny
pv-country-VE = Wenezuela
pv-country-VG = Brytyjskie Wyspy Dziewicze
pv-country-VI = Wyspy Dziewicze Stanów Zjednoczonych
pv-country-VN = Wietnam
pv-country-VU = Vanuatu
pv-country-WF = Wallis i Futuna
pv-country-WS = Samoa
pv-country-YE = Jemen
pv-country-YT = Majotta
pv-country-ZA = Republika Południowej Afryki
pv-country-ZM = Zambia
pv-country-ZW = Zimbabwe
pv-country-SU = Związek Radziecki
pv-country-DD = Niemiecka Republika Demokratyczna
pv-country-YU = Jugosławia
pv-country-CS = Czechosłowacja
pv-country-OT = Imperium Osmańskie
lang-aa = Afar
lang-ab = Abchaski
lang-ae = Awestyjski
lang-af = Afrikaans
lang-ak = Akan
lang-am = Amharski
lang-an = Aragoński
lang-ar = Arabski
lang-as = Asamski
lang-av = Awarski
lang-ay = Ajmara
lang-az = Azerbejdżański
lang-ba = Baszkirski
lang-be = Białoruski
lang-bg = Bułgarski
lang-bi = Bislama
lang-bm = Bambara
lang-bn = Bengalski
lang-bo = Tybetański
lang-br = Bretoński
lang-bs = Bośniacki
lang-ca = Kataloński
lang-ce = Czeczeński
lang-ch = Czamorro
lang-co = Korsykański
lang-cr = Kri
lang-cs = Czeski
lang-cu = Cerkiewnosłowiański
lang-cv = Czuwaski
lang-cy = Walijski
lang-da = Duński
lang-de = Niemiecki
lang-dv = Malediwski
lang-dz = Dzongkha
lang-ee = Ewe
lang-el = Grecki
lang-en = Angielski
lang-eo = Esperanto
lang-es = Hiszpański
lang-et = Estoński
lang-eu = Baskijski
lang-fa = Perski
lang-ff = Fulani
lang-fi = Fiński
lang-fj = Fidżijski
lang-fo = Farerski
lang-fr = Francuski
lang-fy = Zachodniofryzyjski
lang-ga = Irlandzki
lang-gd = Szkocki gaelicki
lang-gl = Galicyjski
lang-gn = Guarani
lang-gu = Gudżarati
lang-gv = Manx
lang-ha = Hausa
lang-he = Hebrajski
lang-hi = Hindi
lang-ho = Hiri motu
lang-hr = Chorwacki
lang-ht = Kreolski haitański
lang-hu = Węgierski
lang-hy = Ormiański
lang-hz = Herero
lang-ia = Interlingua
lang-id = Indonezyjski
lang-ie = Interlingue
lang-ig = Igbo
lang-ii = Syczuański
lang-ik = Inupiak
lang-io = Ido
lang-is = Islandzki
lang-it = Włoski
lang-iu = Inuktitut
lang-ja = Japoński
lang-jv = Jawajski
lang-ka = Gruziński
lang-kg = Kongo
lang-ki = Kikuju
lang-kj = Kwanyama
lang-kk = Kazachski
lang-kl = Grenlandzki
lang-km = Khmerski
lang-kn = Kannada
lang-ko = Koreański
lang-kr = Kanuri
lang-ks = Kaszmirski
lang-ku = Kurdyjski
lang-kv = Komi
lang-kw = Kornijski
lang-ky = Kirgiski
lang-la = Łaciński
lang-lb = Luksemburski
lang-lg = Ganda
lang-li = Limburski
lang-ln = Lingala
lang-lo = Laotański
lang-lt = Litewski
lang-lu = Luba-katanga
lang-lv = Łotewski
lang-mg = Malgaski
lang-mh = Marszalski
lang-mi = Maoryjski
lang-mk = Macedoński
lang-ml = Malajalam
lang-mn = Mongolski
lang-mr = Marathi
lang-ms = Malajski
lang-mt = Maltański
lang-my = Birmański
lang-na = Nauruański
lang-nb = Norweski (bokmål)
lang-nd = Ndebele północny
lang-ne = Nepalski
lang-ng = Ndonga
lang-nl = Niderlandzki
lang-nn = Norweski (nynorsk)
lang-no = Norweski
lang-nr = Ndebele południowy
lang-nv = Nawaho
lang-ny = Njandża
lang-oc = Oksytański
lang-oj = Odżibwa
lang-om = Oromo
lang-or = Orija
lang-os = Osetyjski
lang-pa = Pendżabski
lang-pi = Palijski
lang-pl = Polski
lang-ps = Paszto
lang-pt = Portugalski
lang-qu = Keczua
lang-rm = Retoromański
lang-rn = Rundi
lang-ro = Rumuński
lang-ru = Rosyjski
lang-rw = Kinya-ruanda
lang-sa = Sanskryt
lang-sc = Sardyński
lang-sd = Sindhi
lang-se = Północnolapoński
lang-sg = Sango
lang-sh = Serbsko-chorwacki
lang-si = Syngaleski
lang-sk = Słowacki
lang-sl = Słoweński
lang-sm = Samoański
lang-sn = Shona
lang-so = Somalijski
lang-sq = Albański
lang-sr = Serbski
lang-ss = Suazi
lang-st = Sotho południowy
lang-su = Sundajski
lang-sv = Szwedzki
lang-sw = Suahili
lang-ta = Tamilski
lang-te = Telugu
lang-tg = Tadżycki
lang-th = Tajski
lang-ti = Tigrinia
lang-tk = Turkmeński
lang-tl = Tagalski
lang-tn = Setswana
lang-to = Tonga
lang-tr = Turecki
lang-ts = Tsonga
lang-tt = Tatarski
lang-tw = Twi
lang-ty = Tahitański
lang-ug = Ujgurski
lang-uk = Ukraiński
lang-ur = Urdu
lang-uz = Uzbecki
lang-ve = Venda
lang-vi = Wietnamski
lang-vo = Wolapik
lang-wa = Waloński
lang-wo = Wolof
lang-xh = Khosa
lang-yi = Jidysz
lang-yo = Joruba
lang-za = Czuang
lang-zh = Chiński
lang-zu = Zulu
person-tab-profile = Profil
profile-groups-label = Działy profilu
profile-group-withheld = Część tego działu jest przed tobą ukryta
profile-withheld = Zapisano dla tej osoby, ale ukryto przed tobą: { $classes }.
profile-empty = W tym dziale nic jeszcze nie zapisano.
profile-earlier = wcześniejszy formularz
profile-earlier-title = Zapisane przez wcześniejszą wersję tej aplikacji w polu, dla którego AXGF 1.1 nie ma miejsca. Zachowano je w pierwotnym brzmieniu.
profile-other-names = { $n ->
        [one] i jedno inne imię lub nazwisko
        [few] i { $n } inne imiona lub nazwiska
        [many] i { $n } innych imion lub nazwisk
       *[other] i { $n } innego imienia lub nazwiska
    }
profile-edit-group = Edytuj dział „{ $group }”
profile-summary-link = { $n ->
        [one] Jeden fakt w profilu
        [few] { $n } fakty w profilu
        [many] { $n } faktów w profilu
       *[other] { $n } faktu w profilu
    }
profile-from = od
profile-until = do
profile-yes = Tak
profile-no = Nie
profile-value = Wartość
profile-editor-title = Profil
profile-problems = Części wprowadzonych danych nie udało się zapisać. Każdy problem opisano przy jego polu i nic nie zostało zapisane.
profile-editor-withheld = Ten dział zawiera też dla tej osoby dane z kategorii: { $classes }, do których nie masz dostępu. Nie są tu pokazane, a zapisanie formularza pozostawi je bez zmian.
profile-living-class-note = Ta osoba jest zapisana jako żyjąca. To, co wpiszesz tutaj w kategorii wrażliwej, zobaczą tylko administratorzy.
profile-relationships-elsewhere = Rodzice, partnerzy, dzieci, rodzice chrzestni i świadkowie nie są zapisani przy tej osobie. To rodziny, powiązania i wydarzenia, które ją wymieniają — dlatego każda zmiana tutaj zmienia też zapis wszystkich pozostałych osób, których dotyczy.
profile-documents-first = Artefakt odsyła do dokumentu dołączonego do tej osoby. Najpierw dołącz plik.
profile-editor-nothing = Nic w tym dziale nie jest do edycji dla ciebie.
profile-new-entry = Nowy wpis
profile-provenance = Data, źródło i pewność
profile-from-date = Prawdziwe od
profile-until-date = Prawdziwe do
profile-remove-entry = Usuń ten wpis
profile-add-entry = Dodaj kolejny wpis
profile-no-such-group-title = Nie ma takiego działu
profile-no-such-group-detail = Profil nie ma działu o tej nazwie.
profile-error-number = Wartość w tym polu musi być liczbą.
profile-error-integer = Wartość w tym polu musi być liczbą całkowitą.
profile-error-range = Liczba wykracza poza zakres dozwolony dla tego atrybutu.
profile-error-term = Wartość nie należy do dostępnych opcji.
profile-error-required = We wpisie brakuje wymaganego pola.
profile-error-one-of = Wpis wymaga wypełnienia co najmniej jednego z głównych pól.
profile-error-confidence = Pewność wyraża się liczbą od 0 do 1, na przykład 0,8.
profile-error-time = Godzinę zapisuje się w godzinach i minutach, na przykład 05:40.
profile-error-currency = Walutę zapisuje się trzyliterowym kodem, na przykład PLN.
profile-error-language = Język zapisuje się jego kodem, na przykład pl albo zh-Hans.
profile-error-coordinates = Współrzędne wymagają szerokości od −90 do 90 i długości od −180 do 180.
profile-error-rank-country = Stopień należy do innego państwa niż wybrane.
record-unknown-place = [Nieznane miejsce]
record-missing-document = [Brakujący dokument]

## Interface

confidence-certain = Pewność { $percent }% — praktycznie pewne
confidence-high = Pewność { $percent }% — dobrze udokumentowane
confidence-medium = Pewność { $percent }% — prawdopodobne, ale niepotwierdzone
confidence-low = Pewność { $percent }% — przypuszczenie
tree-edge-union-between = { $from } i { $to } — { $confidence }
tree-edge-parentage-of = { $from }, rodzic: { $to } — { $confidence }
record-note-biography = Biografia
record-note-birth-date-as-recorded = Data urodzenia w zapisie źródłowym
record-note-death-date-as-recorded = Data śmierci w zapisie źródłowym
record-note-event-date-as-recorded = Data zdarzenia „{ $event }” w zapisie źródłowym
record-unknown-source = [Nieznane źródło]
record-untitled-source = [Źródło bez tytułu]
record-unnamed = [Bez nazwy]
record-untitled = [Bez tytułu]
record-period-from = od { $date }
record-period-until = do { $date }
record-dates-unrecorded = daty nieznane
record-link-unlabelled = powiązanie z
record-link-reverse = { $label } (dla)
record-place-worked-as = Pracował(a) jako { $title }
record-place-married-to = Ślub z { $name }
record-place-married = Ślub
record-source-use-name = imię i nazwisko „{ $name }”
record-source-use-working-as = praca jako { $title }
record-source-use-union-with = związek z { $name }
record-source-use-union = związek
record-lifespan-born = ur. { $year }
record-lifespan-died = zm. { $year }
size-bytes = { $n ->
        [one] { $n } bajt
        [few] { $n } bajty
        [many] { $n } bajtów
       *[other] { $n } bajta
    }
size-kb = { $n } KB
size-mb = { $n } MB
size-gb = { $n } GB
calendar-gregorian = gregoriański
calendar-julian = juliański
calendar-hebrew = hebrajski
calendar-hijri = muzułmański
calendar-persian = perski
calendar-chinese = chiński
calendar-ethiopian = etiopski
calendar-japanese_era = ery japońskie
calendar-republican_french = republikański francuski
calendar-roman = rzymski
diff-summary-none = nie zmienił(a) żadnego pola
diff-summary-one = zmienił(a) { $a }
diff-summary-two = zmienił(a) { $a } i { $b }
diff-summary-many = zmienił(a) { $a }, { $b } i { $n ->
        [one] jeszcze jedno pole
        [few] jeszcze { $n } pola
        [many] jeszcze { $n } pól
       *[other] jeszcze { $n } pola
    }
diff-saved-none = żadne pole się nie zmieniło
diff-saved-one = zmieniono { $a }
diff-saved-two = zmieniono { $a } i { $b }
diff-saved-many = zmieniono { $a }, { $b } i { $n ->
        [one] jeszcze jedno pole
        [few] jeszcze { $n } pola
        [many] jeszcze { $n } pól
       *[other] jeszcze { $n } pola
    }
history-created = utworzył(a)
history-deleted = usunął/usunęła
history-attached = dołączył(a) plik
admin-raw-json-unparsed = Surowego JSON-a nie udało się odczytać ({ $error }). Nic nie zostało zapisane.
conflict-someone = Ktoś
conflict-unrecorded-time = godzinie, której nikt nie zapisał
dedup-merged-persons = { $n ->
        [one] scalono jedną osobę
        [few] scalono { $n } osoby
        [many] scalono { $n } osób
       *[other] scalono { $n } osoby
    }
dedup-merged-families = { $n ->
        [one] scalono jedną rodzinę
        [few] scalono { $n } rodziny
        [many] scalono { $n } rodzin
       *[other] scalono { $n } rodziny
    }
dedup-manual-review = { $n ->
        [one] jeden przypadek zostawiono do przejrzenia przez człowieka
        [few] { $n } przypadki zostawiono do przejrzenia przez człowieka
        [many] { $n } przypadków zostawiono do przejrzenia przez człowieka
       *[other] { $n } przypadku zostawiono do przejrzenia przez człowieka
    }
dedup-nothing = Nie ma nic do zgłoszenia.
record-union-duplicate = Jedna para, więcej niż jeden zapis.
record-union-duplicate-detail = Pakiet przechowuje osobne rekordy Rodziny dla tych dwojga ludzi. To błąd w danych, a nie drugi związek.
record-union-duplicate-action = Scal duplikaty
record-union-duplicate-confirm = Zdeduplikować cały pakiet? Każda para, którą biblioteka potrafi scalić, zostanie scalona; pozostałe zostaną zgłoszone.
dedup-pair-merged = Wskazana para to teraz jeden zapis.
dedup-pair-refused = Wskazana para nie została scalona: biblioteka odmówiła i pozostawiła ją do przejrzenia przez człowieka.

validate-errors = { $n ->
        [one] jeden błąd
        [few] { $n } błędy
        [many] { $n } błędów
       *[other] { $n } błędu
    }
validate-warnings = { $n ->
        [one] jedno ostrzeżenie
        [few] { $n } ostrzeżenia
        [many] { $n } ostrzeżeń
       *[other] { $n } ostrzeżenia
    }
validate-notes = { $n ->
        [one] jedna uwaga
        [few] { $n } uwagi
        [many] { $n } uwag
       *[other] { $n } uwagi
    }
validate-nothing = Nie ma nic do zgłoszenia.
list-separator = { ", " }
result-written = Archiwum zostało zapisane na dysku.
result-refused = Biblioteka odmówiła wykonania tej operacji. Archiwum na dysku pozostało bez zmian.
convert-error-no-file = Nie przesłano żadnego pliku. Najpierw wybierz plik .ged.
convert-error-file-too-large = Ten plik ma { $size } MB, a limit wynosi { $limit } MB. Niczego nie przekonwertowano.
convert-error-too-large = Przesyłany plik przekracza limit { $limit } MB. Niczego nie przekonwertowano.
convert-error-unreadable = Nie udało się odczytać przesłanych danych ({ $error }). Niczego nie przekonwertowano.
convert-error-not-gedcom = To nie wygląda na plik GEDCOM: plik GEDCOM 5.5.1 zaczyna się od wiersza „0 HEAD”. Niczego nie przekonwertowano.
convert-error-packaging = Plik został przekonwertowany, ale nie udało się go spakować ({ $error }).
completeness-fraction = { $part } z { $whole }
event-category-adoption = Adopcja
event-category-migration = Migracja
event-category-naturalization = Naturalizacja
event-category-incarceration = Uwięzienie
event-category-name_change = Zmiana nazwiska
event-category-legal = Sprawa prawna
event-category-religious = Wydarzenie religijne
event-category-social = Wydarzenie towarzyskie
event-category-historical = Wydarzenie historyczne
precision-quarter_century = co do ćwierćwiecza
source-type-birth_certificate = akt urodzenia
source-type-death_certificate = akt zgonu
source-type-marriage_certificate = akt małżeństwa
source-type-census = spis ludności
source-type-baptism_record = akt chrztu
source-type-burial_record = akt pochówku
source-type-will = testament
source-type-land_record = księga wieczysta
source-type-military_record = akta wojskowe
source-type-immigration_record = akta imigracyjne
source-type-naturalization = akta naturalizacji
source-type-passport = paszport
source-type-photograph = fotografia
source-type-letter = list
source-type-diary = pamiętnik
source-type-newspaper = gazeta
source-type-oral_tradition = przekaz ustny
source-type-dna = badanie DNA
source-type-family_bible = Biblia rodzinna
source-type-gravestone = nagrobek
source-type-published_genealogy = opublikowana genealogia
source-type-other = inne źródło
source-status-verified = sprawdzone z oryginałem
source-status-unverified = jeszcze niesprawdzone
source-status-lost = zaginione
source-status-known_missing = wiadomo, że brakuje
document-type-birth_certificate = akt urodzenia
document-type-death_certificate = akt zgonu
document-type-marriage_certificate = akt małżeństwa
document-type-census_page = strona spisu ludności
document-type-baptism_record = akt chrztu
document-type-military_record = akta wojskowe
document-type-will = testament
document-type-land_record = księga wieczysta
document-type-diary = pamiętnik
document-type-newspaper_clipping = wycinek prasowy
document-type-gravestone_photo = zdjęcie nagrobka
document-type-family_tree_drawing = rysunek drzewa genealogicznego
document-type-audio = nagranie dźwiękowe
document-type-video = nagranie wideo
document-status-present = przechowywany tutaj
document-status-referenced = wymieniony, przechowywany gdzie indziej
document-status-known_missing = wiadomo, że brakuje
document-status-lost = zaginiony
document-status-unknown = miejsce nieznane
diag-unsupported_spec_version = Archiwum deklaruje wersję AXGF, której ta wersja programu nie umie odczytać.
diag-invalid_json = Coś, co powinno być JSON-em, nie daje się odczytać.
diag-invalid_bundle_structure = Archiwum nie jest ułożone tak, jak wymaga AXGF.
diag-schema_validation_failed = Rekord nie zgadza się ze schematem AXGF.
diag-dangling_reference = Rekord wskazuje na inny rekord, którego nie ma w archiwum.
diag-duplicate_entity_id = Dwa rekordy mają ten sam identyfikator.
diag-duplicate_unique_ref = Dwa rekordy zgłaszają to samo odwołanie, które powinno być jedyne.
diag-cycle_detected = Powiązania rodzinne zataczają koło: ktoś byłby własnym przodkiem.
diag-chronology_conflict = Daty sobie przeczą, na przykład dziecko urodziło się przed rodzicem.
diag-out_of_vocabulary = Wartość nie należy do terminów dozwolonych przez jej listę.
diag-claim_inconsistent = Twierdzenie przeczy samemu sobie albo innemu twierdzeniu o tej samej rzeczy.
diag-spec_version_mismatch = Wersja AXGF zadeklarowana w rekordzie nie pasuje do jego zawartości.
diag-unknown_attribute = Rekord zawiera atrybut, którego AXGF nie definiuje.
diag-entity_not_found = Rekordu do zmiany nie ma w archiwum.
diag-entity_already_exists = Rekord o tym identyfikatorze już istnieje.
diag-unknown_entity_kind = AXGF nie zna takiego rodzaju rekordu.
diag-delete_blocked_by_reference = Rekordu nie można usunąć, dopóki inne rekordy na niego wskazują.
diag-manual_review_required = Musi to przejrzeć człowiek; nie zostało zmienione automatycznie.
diag-zip_read_error = Nie udało się odczytać pliku archiwum.
diag-zip_write_error = Nie udało się zapisać pliku archiwum.
diag-payloads_external = Dołączone pliki są przechowywane poza danymi archiwum.
diag-payload_source_failed = Nie udało się odczytać dołączonego pliku.
diag-payload_sink_failed = Nie udało się zapisać dołączonego pliku.
diag-gedcom_parse_error = Nie udało się zrozumieć wiersza pliku GEDCOM.
diag-gedcom_unrecognized_tag = Plik GEDCOM używa znacznika, którego import nie zna, więc ten wpis nie został przeniesiony.
diag-internal = Coś poszło nie tak wewnątrz biblioteki.
field-person-display-name = Wyświetlane imię i nazwisko
field-person-display-name-hint = Nazwa pokazywana wszędzie w serwisie.
field-person-gender = Płeć
field-person-living = Żyje
field-person-birth-date = Data urodzenia
field-date-value-hint = Rok, rok i miesiąc albo pełna data: 1923, 1923-04 lub 1923-04-12. Zostaw puste, jeśli nikt jej nie zna.
field-person-birth-precision = Dokładność daty urodzenia
field-precision-hint = Jak dokładnie źródło to ustala.
field-person-birth-circa = Data urodzenia przybliżona
field-circa-hint = Pokazywana jako „około 1923”, a nie jako dokładne twierdzenie.
field-person-birth-place = Identyfikator miejsca urodzenia
field-person-birth-confidence = Pewność urodzenia
field-person-confidence-hint = Jak bardzo jesteś pewien. To właśnie rysuje serwis.
field-person-death-date = Data śmierci
field-person-death-precision = Dokładność daty śmierci
field-person-death-circa = Data śmierci przybliżona
field-person-death-place = Identyfikator miejsca śmierci
field-person-death-confidence = Pewność śmierci
field-person-death-cause = Przyczyna śmierci
field-person-bio = Biografia
field-notes = Notatki
field-family-name = Nazwa rodziny
field-description = Opis
field-family-union-type = Rodzaj związku
field-family-union-status = Stan związku
field-family-union-confidence = Pewność związku
field-family-union-confidence-hint = Określa, jak wyraźnie linia między partnerami jest rysowana w drzewie.
field-family-union-start = Początek związku
field-family-union-end = Koniec związku
field-family-notes-hint = Partnerzy i dzieci są listami — edytuj je w surowym JSON-ie poniżej albo na stronie relacji danej osoby.
field-category = Kategoria
field-required-hint = Wymagane.
field-event-subcategory = Podkategoria
field-date = Data
field-event-date-hint = Wymagana przez schemat.
field-precision = Dokładność
field-circa = Przybliżona
field-place-id = Identyfikator miejsca
field-confidence = Pewność
field-source-id = Identyfikator źródła
field-link-from-type = Rodzaj początku
field-link-from-id = Identyfikator początku
field-link-to-type = Rodzaj końca
field-link-to-id = Identyfikator końca
field-link-label = Nazwa powiązania
field-link-label-hint = Czytana w kierunku powiązania: „ojciec chrzestny”, „pracodawca”, „świadek”. Wymagane.
field-link-label-reverse = Nazwa w drugą stronę
field-link-label-reverse-hint = Jak czyta się to z drugiego końca: „chrześniak”, „pracownik”.
field-link-bidirectional = Czyta się tak samo w obie strony
field-valid-from = Obowiązuje od
field-link-valid-from-hint = Kiedy relacja się zaczęła.
field-valid-until = Obowiązuje do
field-link-confidence-hint = „Pewne w 85%, według listu rodzinnego” — tego GEDCOM nie potrafi wyrazić.
field-note = Uwaga
field-occupation-person-id = Identyfikator osoby
field-occupation-title = Zajęcie
field-occupation-title-hint = Wymagane, na przykład Nauczyciel.
field-occupation-title-latin = Zajęcie (alfabet łaciński)
field-occupation-employer = Pracodawca
field-occupation-from = Od
field-occupation-from-hint = Zajęcie to okres. Podanie obu końców pozwala narysować je jako pasek.
field-occupation-until = Do
field-source-title = Tytuł
field-source-type = Rodzaj źródła
field-source-reliability = Wiarygodność
field-source-reliability-hint = Wymagane. Pokazywana jako plakietka przy każdym fakcie, który opiera się na tym źródle.
field-source-status = Stan źródła
field-source-repository = Miejsce przechowywania
field-source-repository-reference = Sygnatura w miejscu przechowywania
field-source-transcription = Transkrypcja
field-place-name = Nazwa główna
field-place-name-lang = Język nazwy
field-place-name-lang-hint = Kod języka, na przykład en, fr lub pl.
field-place-type = Rodzaj miejsca
field-place-region = Region
field-place-country-current = Dzisiejsze państwo
field-place-country-current-hint = Historia granic jest listą — edytuj ją w surowym JSON-ie poniżej.
field-document-filename = Nazwa pliku
field-document-mime-type = Typ nośnika
field-document-mime-type-hint = Wymagane, na przykład image/jpeg.
field-document-type = Rodzaj dokumentu
field-document-status = Stan pliku
field-document-url = Adres internetowy
field-document-caption = Podpis
lang-zh-Hans = Chiński uproszczony
family-lineage = Pochodzenie
links-relation = Rodzaj więzi
occupations-position = Stanowisko
field-link-relation = Rodzaj więzi
field-link-relation-hint = Jedna z więzi nazwanych w AXGF 1.1. Nazwa powyżej zachowuje słowa zapisu.
field-occupation-position = Stanowisko
field-occupation-position-hint = Stanowisko w ramach zajęcia: Dyrektorka, gdy zajęciem jest Nauczycielka.
error-delete-changed-title = Zmienione od chwili, gdy to oglądałeś
error-delete-changed-detail = Ten rekord zapisano ponownie po wyświetleniu strony, z której go usuwasz; ma teraz wersję { $version }. Nic nie zostało usunięte. Obejrzyj go w obecnym stanie, zanim zdecydujesz ponownie.
error-delete-changed-look = Obejrzyj ponownie
documents-files = Pliki dołączone tutaj
documents-files-help = Edytuj szczegóły pliku albo usuń sam plik. Usunięcie zabiera dokument i jego bajty z archiwum dla wszystkich, do których jest dołączony; aby odłączyć go tylko od tej osoby, wyczyść jego wiersz powyżej.
documents-edit-details = Edytuj szczegóły
documents-delete = Usuń ten plik

## Charts

radar-section = Wykresy z zapisu
radar-section-help = Trzy odczyty tego, co zawiera ten zapis, każda oś od 0 do 100. Każda liczba jest wyliczana z faktów podanych obok, według reguł spisanych w dokumentacji aplikacji; nic nie jest zapisywane ani zgadywane, a oś, z której nie ma czego odczytać, zostaje pusta, zamiast dostać średnią ocenę. Znaczniki i paski mówią, jak pewna jest każda liczba: pełna kropka to praktycznie pewne, kropka w okręgu dobrze udokumentowane, okrąg prawdopodobne, przerywany okrąg przypuszczenie — a im dłuższy pasek, tym mniejsza pewność.
radar-physique = Budowa ciała
radar-mind = Temperament i umysł
radar-vitality = Zdrowie i witalność
radar-axis-stature = Wzrost
radar-axis-build = Budowa
radar-axis-lean-mass = Beztłuszczowa masa ciała
radar-axis-posture = Postawa
radar-axis-gait = Chód
radar-axis-dentition = Zęby
radar-axis-openness = Otwartość
radar-axis-conscientiousness = Sumienność
radar-axis-extraversion = Ekstrawersja
radar-axis-agreeableness = Ugodowość
radar-axis-stability = Stabilność emocjonalna
radar-axis-cognition = Funkcje poznawcze
radar-axis-circulation = Krążenie
radar-axis-breathing = Oddychanie
radar-axis-metabolism = Metabolizm
radar-axis-illness = Brak chorób
radar-axis-senses = Zmysły
radar-axis-rest = Sen i nastrój
radar-folded-open = Pokaż ten wykres
radar-folded-why = Ta osoba jest zapisana jako żyjąca. Portret temperamentu żyjącej osoby pozostaje zwinięty, dopóki ktoś, kto może go czytać, nie poprosi o jego pokazanie.
radar-empty = Z tego zapisu nie da się jeszcze nic odczytać do tego wykresu.
radar-table-caption = { $chart }: każda oś, jej wynik i to, z czego go odczytano
radar-col-axis = Oś
radar-col-score = Wynik
radar-col-from = Odczytano z
radar-no-score = brak wyniku
radar-from-none = nic
record-link-outgoing = od tej osoby
record-link-incoming = do tej osoby
