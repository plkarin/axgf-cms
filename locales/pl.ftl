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

prefs-title = Język i wygląd
prefs-language = Język
prefs-theme = Wygląd
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
record-doc-photo = zdjęcie
record-doc-certificate = akt
record-doc-letter = list
record-doc-record = zapis
record-doc-newspaper = gazeta
record-doc-other = inne
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
