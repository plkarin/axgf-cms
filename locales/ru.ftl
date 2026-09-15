# axgf-cms — тексты интерфейса, русский.
#
# МАШИННОЕ КАЧЕСТВО — не вычитано носителем языка. У генеалогической лексики
# есть устоявшиеся соответствия, которые зависят от традиции делопроизводства,
# и этот перевод может быть неверным. Исправления приветствуются — см.
# CONTRIBUTING.md.
#
# Русский добавлен не для полноты списка: метрические книги и документы
# гражданского состояния на территории бывшей Российской империи велись
# по-русски, поэтому исследователь польских, литовских, украинских или
# белорусских корней читает именно русские записи.
#
# Принятые соответствия (носителю языка есть что оспорить):
#   union → союз · link → связь · confidence → уверенность
#   reliability → достоверность · source → источник
#   primary source → первичный источник · occupation → род занятий
#   record → запись · archive → архив · godparent → восприемник
#   witness → свидетель · speculative → предположительный
#
# Множественное число: правила CLDR one / few / many / other. Никогда не
# заменять их английской логикой «один или больше».
#
# Даты: в полной дате месяц стоит в родительном падеже — «12 апреля 1923»,
# а не «12 апрель 1923». Отсюда два набора: month-N и month-in-date-N.
#
# ПРАВИЛО: этот файл переводит только интерфейс. Имена, места, заметки и
# занятия берутся из архива и остаются на своём языке и в своей графике.

app-name = ax-genealogy

## Шапка и подвал

nav-tree = Древо
nav-convert = Импорт
nav-admin = Управление
nav-sign-in = Войти
nav-sign-out = Выйти
footer-open-format = Архив вашей семьи — это один файл, который остаётся у вас, записанный в открытом формате: он откроется и много позже, чем исчезнет этот сайт.
footer-open-format-link = О формате

## Настройки

prefs-title = Язык и оформление
prefs-language = Язык
prefs-theme = Оформление
prefs-background = Фон
prefs-background-on = Мягкая цветовая подложка за страницей
prefs-apply = Применить
prefs-reviewed = вычитано
prefs-machine = машинный, { $coverage }%
prefs-machine-complete = полный, ещё не вычитан
prefs-machine-title = Переведено без вычитки носителем языка. Особенно ненадёжна генеалогическая лексика — слова для союза, восприемника или первичного источника различаются по традиции делопроизводства каждой страны. Исправления приветствуются, а CONTRIBUTING.md говорит, с чего начать.

theme-light = Светлая
theme-dark = Тёмная
theme-system = Как в системе
theme-high-contrast = Высокий контраст
theme-sepia = Сепия
theme-deuteranopia = Дейтеранопия
theme-protanopia = Протанопия
theme-tritanopia = Тританопия
theme-colour-blind-note = безопасно при дальтонизме
theme-contrast-note = максимальный контраст

## Древо

tree-title-around = Вокруг { $name }
tree-title-whole = Всё древо
tree-lede-focused = { $ancestors ->
        [one] Один предок
        [few] { $ancestors } предка
        [many] { $ancestors } предков
       *[other] { $ancestors } предков
    }, { $descendants ->
        [one] один потомок
        [few] { $descendants } потомка
        [many] { $descendants } потомков
       *[other] { $descendants } потомков
    } и { $spouses ->
        [one] один партнёр
        [few] { $spouses } партнёра
        [many] { $spouses } партнёров
       *[other] { $spouses } партнёров
    }, по { $depth } поколений в каждую сторону.
tree-filter-label = Отфильтровать видимые карточки
tree-filter-placeholder = Введите имя…
tree-centre-on = Центрировать на
tree-depth = Поколений в каждую сторону
tree-show = Показать
tree-hidden-notice = { $n ->
        [one] Один человек показан без подробностей
        [few] { $n } человека показаны без подробностей
        [many] { $n } человек показаны без подробностей
       *[other] { $n } человек показаны без подробностей
    }
tree-hidden-because-role = , потому что их видимость выше того, что позволено читать вашей учётной записи.
tree-hidden-because-anonymous = , потому что они не общедоступны.
tree-hidden-sign-in = Войдите, если у вас есть учётная запись.
tree-restricted-card = Эта запись вам не видна
tree-empty = Пока некого рисовать.
tree-unplaced = Ни в одной записанной семье

## Запись о человеке

record-identity = Личность
record-life-events = События жизни
record-family = Семья
record-other-relationships = Другие связи
record-occupations = Род занятий
record-places = Места
record-sources-documents = Источники и документы
record-notes = Заметки
record-history = История правок
record-raw = Исходные данные
record-raw-summary-note = JSON, из которого построена эта страница

record-identity-help = Каждое записанное имя со своим типом, временем употребления и источником, с родной графикой рядом с латинской транслитерацией там, где они различаются, а также пол, признак живущего и видимость.
record-life-events-help = Рождение, смерть и каждое событие, в котором человек участвовал, по датам, с указанием его роли — так что свадьба, где он был лишь свидетелем, стоит рядом с его собственной. Факт без даты уходит в конец, а не притворяется первым.
record-family-help = Родители и братья с сёстрами, затем каждый союз со своим типом, датами, местом, тем, как он завершился, и детьми по порядку рождения.
record-other-relationships-help = Каждая связь, где этот человек стоит на одном из концов, прочитанная с его стороны: одна и та же запись читается как «восприемник» с одного конца и «крестник» с другого.
record-occupations-help = Занятия как отрезки на одной общей оси, чтобы две должности можно было сравнить глазом; полоса остаётся открытой там, где границы нет.
record-places-help = Каждое место, которого касается эта запись, с тем, что там произошло, и с историей границ, которая и делает место осмысленным во времени.
record-sources-documents-help = Каждый источник перечисляет факты этой страницы, которые на нём держатся, по силе свидетельства.
record-notes-help = Заметки к этой записи, включая текст, который преобразователь не смог разобрать и сохранил дословно, вместо того чтобы выбросить.
record-history-help = Каждое сохранённое изменение этой записи, новые сверху. Кто что исправил — факт о людях, ведущих древо, а не о семье внутри него, поэтому он остаётся вне выгружаемого архива и виден только вошедшим родственникам.
record-raw-help = Здесь ничто не создано ради показа: это запись ровно в том виде, в каком она хранится, вплоть до названий полей. Если вам когда-нибудь придётся читать архив без этого сайта, вы увидите именно это.
record-help-toggle = Что показывает этот раздел

record-gender = Пол
record-living = Жив
record-visibility = Видимость
record-yes = да
record-no = нет
record-name-type = Тип имени
record-name-used = Употреблялось
record-name-evidence = Основание
record-transliteration = Латинская транслитерация
record-born = Родился(ась)
record-died = Умер(ла)
record-parents = Родители
record-siblings = Братья и сёстры
record-children = Дети
record-unknown-person = [Неизвестно]
record-restricted-person = Закрыто
record-restricted-title = Эта запись вам не видна
record-absent-person-title = Упомянут в древе, но своей записи не имеет
record-confidence = Уверенность
record-source = Источник
record-download = Скачать

## Доступ

access-restricted-title = Вам не видно
access-restricted-signed-in = Видимость этой записи выше того, что позволено читать вашей учётной записи. Управляющий может изменить либо видимость записи, либо вашу роль.
access-restricted-anonymous = Эта запись не общедоступна. Войдите, чтобы проверить, может ли ваша учётная запись её читать.
access-role-title = Не для вашей роли
access-role-admin = Это страница управляющего. Ваша учётная запись может создавать и править записи, но не управлять учётными записями, не удалять записи и не выгружать архив.
access-role-write = Ваша учётная запись может читать это древо, но не менять его. Управляющий может повысить вашу роль до соавтора.
access-scope-title = Вне вашей ветви
access-scope-named = Ваша учётная запись ограничена одной ветвью древа, а эта запись касается человека вне её. Каждый человек, названный в записи, должен быть внутри вашей ветви — иначе семья с одним партнёром со стороны стала бы способом переписать происхождение этого человека.
access-scope-unnamed = Ваша учётная запись ограничена одной ветвью древа, а эта запись не называет никого, с кем её можно было бы сверить. Источники и места правят учётные записи с доступом ко всему древу.

## Ошибки

error-not-found-title = Не найдено
error-not-found-detail = Такой страницы здесь нет.
error-no-such-person-title = Нет такого человека
error-no-such-person-detail = Здесь нет человека с таким идентификатором.
error-no-such-entity-title = Нет такого объекта
error-no-such-entity-detail = Здесь нет записи с таким идентификатором.
error-deleted-while-editing = Здесь нет записи с таким идентификатором. Возможно, её удалили, пока вы её правили.
error-no-such-file-title = Нет такого файла
error-no-such-file-detail = Здесь нет документа с таким идентификатором, либо документ записан без файла — упомянутый документ указывает на то, что хранится в другом месте.
error-not-an-image-title = Это не изображение
error-not-an-image-detail = Для этого документа нет уменьшенной копии, потому что это не изображение, которое эта сборка умеет разбирать.
error-back = Назад

## Вход

login-title = Вход
login-lede = Учётные записи создаёт управляющий.
login-username = Имя пользователя
login-password = Пароль
login-submit = Войти
login-wrong = Такое имя пользователя и пароль не совпадают.
login-token-wrong = Этот токен неверен.
login-throttled = Слишком много неудачных попыток. Подождите несколько минут и попробуйте снова.
login-no-accounts-title = В этой установке ещё нет ни одной учётной записи.
login-no-accounts-detail = Страницы первоначальной настройки здесь намеренно нет: промежуток между развёртыванием и первым входом — это ровно тот момент, когда установка беззащитна, поэтому первый управляющий создаётся из командной строки.
login-no-accounts-note = Она один раз печатает созданный пароль в stderr и больше никогда. До этого единственный вход — аварийный токен ниже.
login-emergency-summary = Аварийный доступ
login-emergency-detail = Общий токен по-прежнему открывает сеанс управляющего и существует ради одного: вернуться внутрь, когда файл .acl потерян или все управляющие заблокированы. Это не учётная запись — у неё нет своих настроек, и журнал правок записывает её как emergency-token, а не как человека. Её применение заносится в журнал как предупреждение.
login-emergency-label = Аварийный токен
login-emergency-submit = Войти по аварийному токену
login-sign-in-prompt = Войдите, чтобы попасть в панель управления.

## Управление

admin-title = Управление
admin-lede = Правится { $path } — { $total } объектов, { $files ->
        [one] один вложенный файл
        [few] { $files } вложенных файла
        [many] { $files } вложенных файлов
       *[other] { $files } вложенных файлов
    }, { $size } на диске. Каждое изменение записывается целиком; отклонённое изменение оставляет файл нетронутым.
admin-entities = Объекты
admin-create = Создать
admin-new-kind = Новый объект: { $kind }
admin-operations = Операции
admin-validate = Проверить
admin-deduplicate = Убрать дубликаты
admin-export = Выгрузить архив
admin-accounts = Учётные записи
admin-roles-note = Проверка, устранение дубликатов, выгрузка, удаление и управление учётными записями доступны только управляющему. Соавтор попадает на любую другую страницу здесь.
admin-dedup-confirm = Устранение дубликатов сливает записи и переписывает архив. Продолжить?
admin-recent-changes = Недавние изменения
admin-recent-note = Последние { $shown } из { $total ->
        [one] одного записанного изменения
        [few] { $total } записанных изменений
        [many] { $total } записанных изменений
       *[other] { $total } записанных изменений
    }, из { $path }.
admin-sessions-open = { $n ->
        [one] Сейчас открыт один сеанс.
        [few] Сейчас открыто { $n } сеанса.
        [many] Сейчас открыто { $n } сеансов.
       *[other] Сейчас открыто { $n } сеансов.
    }
admin-no-changes-yet = Через это приложение пока ничего не менялось. Каждое сохранение дальше заносится в { $path }.
admin-last-validation = Последняя проверка
admin-bundle-heavy = Этот архив весит { $size }. Он целиком читается при запуске и держится в памяти, поэтому после примерно { $warn } сайт начинает стоить настоящей памяти, а перезапуски становятся долгими. Это подходит семейному архиву, а не медиатеке — если вложения растут без предела, держите их в файловом хранилище, а архив пусть на них ссылается.

admin-fields = Поля
admin-raw-json = Исходный JSON
admin-raw-json-help = Объект целиком, так что неизменяемого нет ничего — списки вроде партнёров и детей семьи или истории границ места живут именно здесь. Это исходный документ; поля выше затем записываются поверх принадлежащих им путей, поэтому правьте значение либо в одном месте, либо в другом, но не в обоих. Он должен разбираться как JSON, иначе не сохранится ничего.
admin-save = Сохранить
admin-cancel = Отмена
place-editor-title = Редактировать место
place-add-detail = Дополнить это место
place-names = Названия
place-name-primary = Основное
place-name-lang = Язык
place-name-value = Название
place-names-hint = One row per recorded name. A place administered by three empires carries three names; the primary is the one shown everywhere else.
place-where = Расположение
place-type = Тип
place-region = Регион
place-country-current = Страна сегодня
place-country-hint = ISO 3166-1 alpha-2, e.g. PL, FR, DE.
place-country-history = История границ
place-history-country = Государство
place-history-from = С
place-history-until = По
place-country-history-hint = Which state held this place over which period. Genealogically significant: a record written in Russian in 1880 and one written in Polish in 1930 can name the same village.
place-coordinates = Координаты
place-lat = Широта
place-lon = Долгота
place-precision = Точность
place-identifiers = Идентификаторы
place-wikidata = Wikidata
place-geonames = GeoNames
place-used-by = { $name } используется в { $n ->
        [one] ещё одной записи
        [few] ещё { $n } записях
        [many] ещё { $n } записях
       *[other] ещё { $n } записях
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

place-coordinates-hint = Обычно их вводят вручную. Многие места, записанные при прежней администрации, современный поиск не находит вовсе.
place-geocode-search = Найти это название
place-geocode-hint = Отправляет название, регион и страну службе геокодирования, по одному месту за раз. Ничего не сохраняется, пока вы не сохраните.
place-geocode-off = Поиск названий выключен. Ему нужен контактный адрес, по которому служба опознает эту установку; запустите сервер с --geocoder-contact, чтобы включить его.
place-geocode-query = Искали: { $q }
place-geocode-error = Служба поиска недоступна. Поля координат выше по-прежнему работают.
place-geocode-none = Ничего не найдено. Для деревни, записанной при российской, прусской или австрийской администрации это обычный исход; введите положение вручную.
place-geocode-not-a-place = не населённый пункт
place-geocode-use = Взять это
place-geocode-attribution = Результаты из OpenStreetMap через Nominatim, по лицензии Open Database.

place-paste = Вставьте положение
place-paste-placeholder = ссылка на карту или 52.0782795, 21.2508068
place-paste-read = Разобрать
place-paste-hint = Ссылка Google Maps или OpenStreetMap, адрес geo:, простая пара чисел либо градусы-минуты-секунды, например 52°04'41.8"N 21°15'02.9"E.
place-paste-read-ok = Разобрано в поля выше. Проверьте и сохраните.
place-paste-unreadable = Это положение здесь прочитать не удалось. Поля выше по-прежнему принимают простую пару чисел.

place-map-hint = Щёлкните по карте, чтобы поставить точку, или перетащите булавку. Записью считаются поля выше.
place-map-clear = Убрать точку
place-open-in-map = Найти это место в OpenStreetMap и вставить ссылку обратно

person-tab-record = Запись
person-tab-life = Жизнь
person-tab-media = Материалы
person-tab-tree = Древо
person-tree-depth = По { $n } поколения в каждую сторону. Всё древо — ниже.
person-tree-alone = В этой записи не названы ни родители, ни супруги, ни дети, так что вокруг неё нечего рисовать.

record-no-evidence = К этой записи ничего не приложено — ни источника, ни документа. Для сконвертированного файла это обычное состояние, а не изъян: GEDCOM переносит факты и оставляет позади то, чем они подтверждались.
record-no-evidence-signed-out = Войдите, чтобы что-нибудь приложить.
admin-delete = Удалить
admin-not-set = — не задано —
admin-edit = Править
admin-page-of = Страница { $page } из { $pages }
admin-previous = Назад
admin-next = Вперёд
admin-saved = Сохранено как версия { $version } — { $summary }
admin-not-saved = Не сохранено
admin-created = Создано
admin-not-created = Не создано
admin-deleted = Удалено
admin-not-deleted = Не удалено — ничего не изменилось
admin-what-changed = что изменилось
admin-field = Поле
admin-from = Было
admin-to = Стало
admin-version = версия { $version }

## Учётные записи

accounts-title = Учётные записи
accounts-lede = Хранятся в { $path }, с правами 600, рядом с архивом и никогда внутри него. Архив копируют, пересылают и публикуют; хеши паролей внутри него превратили бы каждую копию семейного древа в копию учётных данных.
accounts-existing = Существующие
accounts-username = Имя пользователя
accounts-role = Роль
accounts-status = Состояние
accounts-branch = Ветвь
accounts-last-seen = Последний вход
accounts-change = Изменить
accounts-you = (это вы)
accounts-active = действует
accounts-disabled = отключена
accounts-never = никогда
accounts-whole-tree = всё древо
accounts-roots = { $n ->
        [one] один корень
        [few] { $n } корня
        [many] { $n } корней
       *[other] { $n } корней
    }
accounts-add = Добавить учётную запись
accounts-no-registration = Самостоятельной регистрации и приглашений здесь нет намеренно. Для семейного архива достаточно управляющего, который знает всех, и это убирает поле для злоупотреблений целиком, вместо того чтобы его оборонять.
accounts-password-hint = Оставьте пустым — пароль будет создан и показан один раз. Не менее { $min } знаков, если задаёте сами.
accounts-new-password-placeholder = новый пароль (пусто = оставить прежний)
accounts-email = Электронная почта
accounts-optional = (необязательно)
accounts-create = Создать учётную запись
accounts-role-viewer = читатель — читает общедоступные и семейные записи
accounts-role-contributor = соавтор — также создаёт, правит и загружает файлы
accounts-role-admin = управляющий — также ведёт учётные записи, удаляет и выгружает
accounts-branch-hint = Ограничивает то, что эта учётная запись может править, названными людьми, их потомками и супругами.
accounts-branch-reading = Это не ограничивает то, что она может читать — этим ведает видимость каждой записи, и эти две вещи намеренно разделены.
accounts-branch-placeholder = по одному идентификатору человека в строке
accounts-ids-in-bundle = Идентификаторы людей в этом древе
accounts-emergency-warning = Вы вошли по аварийному токену. Он даёт права управляющего на этот сеанс, но не является учётной записью: у него нет своих настроек, и журнал правок запишет ваши изменения как emergency-token, а не как человека. Создайте себе ниже настоящую учётную запись и войдите под ней.
accounts-created-with-password = Создана запись { $username }. Пароль — { $password }; он показывается один раз и хранится только как хеш Argon2id, так что передайте его сейчас.
accounts-created = Создана запись { $username }.
accounts-updated = Обновлена запись { $username }. Все её открытые сеансы завершены.
accounts-username-taken = Такое имя пользователя занято.
accounts-pick-role = Выберите роль.
accounts-no-such = Такой учётной записи нет.
accounts-last-admin = Это единственный действующий управляющий. Сначала повысьте кого-то ещё — установку без управляющего можно вернуть только правкой файла .acl или аварийным токеном.
accounts-not-saved = Не сохранено: { $error }

## Расхождения

conflict-title = Кто-то изменил это раньше вас
conflict-lede = { $who } сохранил(а) изменение этого объекта ({ $kind }) в { $when }, после того как вы его открыли. Ваша правка не сохранена, и ничего не перезаписано.
conflict-no-merge = Здесь ничего не сливается само. Слияние правок двух человек даёт запись, которую не выбирал никто из них, а в генеалогии спор двух редакторов о дате обычно значит, что они читают разные источники — и это вопрос к человеку, а не к программе. Сравните оба варианта ниже и решите.
conflict-versions = Вы начали с версии { $expected }; сейчас запись имеет версию { $current }.
conflict-both-changed = Вы оба изменили это
conflict-both-changed-detail = Эти поля правили вы оба. Что бы вы ни сохранили, оно заменит то, что вписал(а) { $who }:
conflict-different-fields = Вы правили разные поля, так что ничто из работы { $who } не оспаривается — но повторное применение всё равно запишет ваш объект целиком поверх их объекта. Проверьте оба столбца перед сохранением.
conflict-field-by-field = Поле за полем
conflict-theirs = На что изменил(а) { $who }
conflict-yours = На что изменили вы
conflict-unchanged-by-you = вами не менялось
conflict-unchanged-by-them = ими не менялось
conflict-nothing-differs = Ни одна из версий не отличается от той, с которой вы начали, ни в одном поле из показанных на этой странице. Номер версии сдвинулся, значит кто-то сохранил запись, ничего в ней не изменив.
conflict-what-now = Что дальше
conflict-reapply = Применить вашу версию поверх их версии
conflict-reapply-hint = Это ваша правка, перенесённая на версию { $version }. Отредактируйте её здесь, чтобы сохранить из работы { $who } то, что хотите, и сохраните. Их версия показана ниже, чтобы копировать из неё.
conflict-save-over = Сохранить это поверх их версии
conflict-discard = Отказаться от своей и начать заново
conflict-their-version = Версия { $who }, в нынешнем виде
conflict-history-of = История этого объекта ({ $kind })

## Импорт

convert-title = Импортировать семейный файл
convert-submit = Импортировать
convert-result-title = Отчёт об импорте
convert-download = Скачать архив

## Даты
#
# Слова даты — текст интерфейса; её ЗНАЧЕНИЕ и ТОЧНОСТЬ — данные, их не
# трогают. Дата, записанная с точностью до года, годовой и остаётся.

date-unknown = Дата неизвестна
date-not-recorded = Не записана
date-circa = около { $date }
date-between = между { $from } и { $to }
date-before = до { $date }
date-after = после { $date }
date-preserved = записано как «{ $text }»
date-day-month-year = { $day } { $month ->
        [1] января
        [2] февраля
        [3] марта
        [4] апреля
        [5] мая
        [6] июня
        [7] июля
        [8] августа
        [9] сентября
        [10] октября
        [11] ноября
        [12] декабря
        *[other] { $month }
    } { $year }
date-month-year = { $month ->
        [1] январь
        [2] февраль
        [3] март
        [4] апрель
        [5] май
        [6] июнь
        [7] июль
        [8] август
        [9] сентябрь
        [10] октябрь
        [11] ноябрь
        [12] декабрь
        *[other] { $month }
    } { $year }
date-decade = { $decade }-е годы

date-century = { $century ->
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
        *[other] { $century }-й
    } век
date-quarter-century = { $quarter ->
        [1] первая
        [2] вторая
        [3] третья
       *[other] четвёртая
    } четверть { $century ->
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
        *[other] { $century }-го
    } века

## Другие страницы ошибок

error-back-to-start = Вернуться к началу
error-payload-missing-title = Нет такого файла
error-payload-missing-detail = Содержимого этого документа нет в кэше.
error-payload-unopenable-detail = Содержимое этого документа не удалось открыть.
error-no-such-document-detail = Здесь нет документа с таким идентификатором.
error-bad-preference-title = Это не один из вариантов
error-bad-preference-detail = Это не язык и не оформление, которые предлагает сайт. Ничего не изменилось.
error-unknown-kind-title = Неизвестный вид
error-unknown-kind-detail = «{ $kind }» — не вид записи. Этот архив содержит: { $kinds }.
error-io-title = Не удалось сохранить
error-io-detail = { $error }. На диске ничего не изменилось.
error-upload-too-large = Этот файл больше предела в { $mb } МБ. Ничего не сохранено, архив не изменился.
error-upload-refused = Документ отклонён: { $reason }. Архив не изменился.
error-back-to-person = Вернуться к записи
error-no-such-person-to-attach = Здесь нет человека с таким идентификатором, значит и прикреплять документ не к чему.
error-upload-title = Эта загрузка не сохранена
error-download-expired-title = Срок этой загрузки истёк
error-download-expired-detail = Импорт хранится пятнадцать минут, затем удаляется. Импортируйте файл заново.
error-upload-none = Файл не загружен. Сначала выберите файл.
error-upload-unsupported = Такой тип файла архив не хранит. Принимаются изображения, PDF, простой текст, звук и видео; тип читается из самих байтов файла, так что переименование исполняемого файла не поможет. SVG отклоняется сразу, потому что SVG может нести сценарий.
error-export-unreadable-title = Не удалось прочитать выгруженный архив
error-export-unreadable-detail = { $error }

## Страница древа, продолжение

tree-title-suffix = древо
tree-back-to-focused = Вернуться к обзору вокруг одного человека
tree-show-all = Показать все: { $n }
tree-width-notice = Этот вид шириной { $width } пикселей; на экране в 1500 пикселей это { $screens ->
        [one] один экран
        [few] { $screens } экрана
        [many] { $screens } экранов
       *[other] { $screens } экранов
    } прокрутки по горизонтали.
tree-confidence-label = Уверенность:
tree-band-certain = достоверно
tree-band-high = высокая
tree-band-medium = средняя
tree-band-low = предположительно
tree-counts = { $drawn } из { $total } человек · { $generations ->
        [one] одно поколение
        [few] { $generations } поколения
        [many] { $generations } поколений
       *[other] { $generations } поколений
    }
tree-unplaced-count = { $n } без места
tree-contradicts-title = Это древо само себе противоречит.
tree-contradicts-detail = Никакая расстановка рядов этого не удовлетворит, поэтому связь ниже исключена из нумерации поколений и часть рядов может быть нарисована не на своём месте. Исправьте ту из двух записей, которая неверна.
tree-contradicts-pair = Записаны и как пара, и как родитель с ребёнком:
tree-contradicts-more = { $n ->
        [one] Ещё одно противоречие не показано.
        [few] Ещё { $n } противоречия не показаны.
        [many] Ещё { $n } противоречий не показаны.
       *[other] Ещё { $n } противоречий не показаны.
    }
tree-no-people = В этом древе пока никого нет.
tree-no-people-cta = Импортируйте семейный файл или добавьте первого человека.
tree-nobody-selected = Для этого выбора рисовать некого.
tree-nobody-selected-cta = Начните с вида по умолчанию.
tree-edge-union = Записанный союз
tree-edge-parentage = Записанное происхождение

## Главная страница

home-empty = Пока ничего не записано. Импортируйте семейный файл, чтобы перенести существующее древо, или добавьте первого человека вручную.
home-count = { $total ->
        [one] Одна запись
        [few] { $total } записи
        [many] { $total } записей
       *[other] { $total } записей
    }, в одном файле, который принадлежит семье.
home-browse = Посмотреть древо
home-convert = Импортировать семейный файл
home-unnamed-family = Это семейное древо
home-in-this-tree = Что семья записала на сегодня
home-showcase-title = Там, где это древо уже говорит больше, чем имена и даты
home-showcase-example = Посмотреть пример →
home-nothing-title = Показывать пока нечего.
home-nothing-detail = Импортируйте семейный файл, чтобы перенести существующее древо, или начните с пустого места и добавьте первого человека сами.

## Карточки обзора

showcase-links-title = { $n ->
        [one] Одна связь вне семьи
        [few] { $n } связи вне семьи
        [many] { $n } связей вне семьи
       *[other] { $n } связей вне семьи
    }
showcase-links-detail = Восприемники, работодатели, свидетели и наставники — у каждой связи свои даты, свой источник и ваша степень уверенности.
showcase-occupations-title = { $n ->
        [one] Одно занятие с началом и концом
        [few] { $n } занятия с началом и концом
        [many] { $n } занятий с началом и концом
       *[other] { $n } занятий с началом и концом
    }
showcase-occupations-detail = «Учитель, 1948–1978» сохраняет свою длительность и рисуется полосой через годы, а не одной датированной строкой.
showcase-uncertain-title = { $n ->
        [one] Одна дата оставлена ровно настолько неточной, насколько её дали
        [few] { $n } даты оставлены ровно настолько неточными, насколько их дали
        [many] { $n } дат оставлены ровно настолько неточными, насколько их дали
       *[other] { $n } дат оставлены ровно настолько неточными, насколько их дали
    }
showcase-uncertain-detail = Около, до, после и между остаются четырьмя разными утверждениями. Дата, которую источник не смог установить, никогда не показывается так, будто смог.
showcase-preserved-title = { $n ->
        [one] Одна дата сохранена в тех словах, какими её записали
        [few] { $n } даты сохранены в тех словах, какими их записали
        [many] { $n } дат сохранены в тех словах, какими их записали
       *[other] { $n } дат сохранены в тех словах, какими их записали
    }
showcase-preserved-detail = Формулировка, которую никто не смог прочитать как дату, остаётся ровно такой, как написана, а не отбрасывается тихо.
showcase-sources-title = { $n ->
        [one] Один источник с записанной достоверностью
        [few] { $n } источника с записанной достоверностью
        [many] { $n } источников с записанной достоверностью
       *[other] { $n } источников с записанной достоверностью
    }
showcase-sources-detail = { $primary ->
        [one] Один первичный источник.
        [few] { $primary } первичных источника.
        [many] { $primary } первичных источников.
       *[other] { $primary } первичных источников.
    } Каждый факт показывает, на каком свидетельстве он держится и насколько оно сильно.
showcase-places-title = { $n ->
        [one] Одно место, чьи границы сдвигались
        [few] { $n } места, чьи границы сдвигались
        [many] { $n } мест, чьи границы сдвигались
       *[other] { $n } мест, чьи границы сдвигались
    }
showcase-places-detail = Город может в разное время принадлежать разным государствам, и запись говорит, какое действовало когда.

## Подробности записи

record-also-recorded-as = записан также как
record-borders-moved = Границы сдвигались:
record-display-name = отображаемое имя
record-read-as = читается как
record-note = Заметка
record-living-yes = жив
record-deceased = умер(ла)
record-centre-tree-here = Центрировать древо здесь
record-centre-tree-title = Передвинуть древо, чтобы центрировать его на этом человеке
record-open-full-page = Открыть отдельной страницей ↗
record-open-full-title = Открыть самостоятельную страницу, которой можно поделиться
record-edit = Править
panel-empty = Выберите карточку, чтобы увидеть здесь полную запись о человеке.
person-see-in-tree = Посмотреть этого человека в древе
person-visibility-inline = видимость:
person-age-at-death = умер в { $n } лет
person-age-now = { $n } лет
person-born-in = родился в { $place }
person-died-in = умер в { $place }
person-children-count = { $n ->
        [one] один ребёнок
        [few] { $n } ребёнка
        [many] { $n } детей
       *[other] { $n } детей
    }
person-generations-below = { $n ->
        [one] одно поколение ниже
        [few] { $n } поколения ниже
        [many] { $n } поколений ниже
       *[other] { $n } поколений ниже
    }
person-portrait-of = Фотография: { $name }
person-no-portrait = Фотография не записана

## Итоги операций

result-diagnostics = Диагностика
result-diagnostics-note = Каждое сообщение, которое вернула библиотека, включая предупреждения, не остановившие операцию. Ничего не отфильтровано.
result-no-diagnostics = Библиотека не вернула ни одного сообщения.
result-continue = Дальше
result-dashboard = Панель
person-sections-label = Разделы этой страницы

## Vocabulary the structured editors offer

name-part-nasab = насаб (родословная)
name-part-laqab = лакаб (прозвание)
name-part-kunya = кунья (текноним)
name-part-nisbah = нисба (происхождение)
name-part-alias = псевдоним
name-part-religious_name = церковное имя
name-part-pen_name = литературный псевдоним
name-type-pen_name = литературный псевдоним
gender-U = Не записано

## Разделы записи, подробности

record-notes-title = Что стоит отметить об этой записи:
record-name = Имя
record-type = Тип
record-cause = Причина:
record-as = как
record-partner-not-recorded = Партнёр не записан
record-union-from = С
record-union-at = в
record-union-until = по
record-occupation-from = с
record-occupation-until = по
record-source-reliability = Достоверность
record-source-supports = Подтверждает
record-photographs = Фотографии
record-documents = Документы
record-file = Файл
record-status = Состояние
record-size = Размер
record-absent-document = Назван этим человеком, но здесь не хранится.
record-no-file = файла нет
record-attach-document = Прикрепить документ
record-doc-photo = фотография
record-doc-certificate = свидетельство
record-doc-letter = письмо
record-doc-record = запись
record-doc-newspaper = газета
record-doc-other = другое
record-upload = Загрузить
record-upload-help = До { $mb } МБ на файл. Вложения лежат рядом с древом и записываются обратно в архив при выгрузке, так что фотография путешествует вместе с семьёй, которой принадлежит. Вид файла читается из его собственного содержимого, а не из имени: принимаются изображения, PDF, простой текст, звук и видео. SVG отклоняется, потому что SVG может нести сценарий.
record-upload-help-short = До { $mb } МБ. SVG отклоняется.
record-verbatim-note = Сохранено ровно так, как значилось в записи, потому что ни один преобразователь не смог это истолковать.
record-file-to-attach = Файл для прикрепления
record-document-type = Тип документа
record-caption = Подпись
record-caption-placeholder = Подпись (необязательно)
record-history-entry-meta = — { $at }
record-history-entry-version = , { $version }

## Виды объектов

kind-person = человек
kind-family = семья
kind-event = событие
kind-link = связь
kind-occupation = род занятий
kind-source = источник
kind-place = место
kind-document = документ

kind-person-plural = { $n ->
        [one] человек
        [few] человека
        [many] человек
       *[other] человек
    }
kind-family-plural = { $n ->
        [one] семья
        [few] семьи
        [many] семей
       *[other] семей
    }
kind-event-plural = { $n ->
        [one] событие
        [few] события
        [many] событий
       *[other] событий
    }
kind-link-plural = { $n ->
        [one] связь
        [few] связи
        [many] связей
       *[other] связей
    }
kind-occupation-plural = { $n ->
        [one] занятие
        [few] занятия
        [many] занятий
       *[other] занятий
    }
kind-source-plural = { $n ->
        [one] источник
        [few] источника
        [many] источников
       *[other] источников
    }
kind-place-plural = { $n ->
        [one] место
        [few] места
        [many] мест
       *[other] мест
    }
kind-document-plural = { $n ->
        [one] документ
        [few] документа
        [many] документов
       *[other] документов
    }

## Списки

list-matching = { $total ->
        [one] Одно совпадение
        [few] { $total } совпадения
        [many] { $total } совпадений
       *[other] { $total } совпадений
    }, по { $per_page } на страницу.
list-filter-placeholder = Фильтр по имени или идентификатору
list-filter = Фильтр
list-clear = Очистить
list-summary = Описание
list-id = Идентификатор
list-actions = Действия
list-nothing = Здесь ничего нет.
list-nothing-matching = Здесь нет ничего, что подходит под «{ $q }».
list-delete-confirm = Удалить этот объект ({ $kind })? Выберите, что станет с объектами, которые на него ссылаются:
list-policy-reject = Отказать
list-policy-reject-detail = — отказать, если на него ещё что-то ссылается. Ничего не теряется.
list-policy-cascade = Каскадом
list-policy-cascade-detail = — удалить его и физически убрать каждую ссылку на него.
list-policy-orphan = Осиротить
list-policy-orphan-detail = — удалить его, но сохранить ссылающиеся записи с обнулённой ссылкой.

## Полнота

completeness-dates-title = Даты по той форме, которую они на самом деле имеют
completeness-no-dates = Дат пока не записано.
completeness-dates-note = Дата, которую кто-то установил до дня, и дата, которую смогли отнести лишь к десятилетию, — это два разных утверждения, и оба сохраняются такими, как их дали. Текст, который вообще не удалось прочитать как дату, сохраняется слово в слово, а не выбрасывается.
completeness-shape-exact = точная
completeness-shape-exact-note = полный календарный день
completeness-shape-approximate = приблизительная
completeness-shape-approximate-note = около, либо только год или десятилетие
completeness-shape-ranged = с границами
completeness-shape-ranged-note = до, после или между
completeness-shape-preserved = дословная
completeness-shape-preserved-note = нечитаемый текст, сохранён без изменений
completeness-shape-unknown = неизвестная
completeness-shape-unknown-note = записана как неизвестная

## Страница импорта

convert-page-title = Импортировать семейный файл
convert-lede = Перенесите существующее древо из файла GEDCOM — это выгрузка, которую делает большинство генеалогических программ. Здесь ничего не хранится, а древо, которое сайт уже показывает, остаётся ровно таким, каким было.
convert-file-label = Семейный файл (.ged)
convert-file-hint = До { $mb } МБ. Древо из 767 человек весит около 320 КБ.
convert-confidence-label = Насколько эти факты достоверны для начала
convert-confidence-hint = Импортируемый файл не говорит, насколько кто-либо был уверен, поэтому каждому факту нужна точка отсчёта. Поставьте ниже для древа, собранного наспех, выше — для проработанного по документам. Честное прочтение этого числа: «импортировано и с тех пор никем не проверено» — любой факт вы сможете потом поднять или опустить, по одному.
convert-lang-label = Язык названий мест
convert-lang-hint = Метка вроде en, fr или ru.

## Отчёт об импорте

convert-failed = Импорт не прошёл
convert-try-another = Попробовать другой файл
convert-converted = Импортирован { $filename }
convert-result-lede = { $total ->
        [one] Одна запись
        [few] { $total } записи
        [many] { $total } записей
       *[other] { $total } записей
    }, { $size } КБ. Всё вошло со степенью уверенности { $confidence }, названия мест прочитаны как { $lang }. Древо, которое показывает этот сайт, не тронуто.
convert-produced = Что перешло
convert-skipped-title = { $n ->
        [one] Одна запись, которую не удалось прочитать
        [few] { $n } записи, которые не удалось прочитать
        [many] { $n } записей, которые не удалось прочитать
       *[other] { $n } записей, которые не удалось прочитать
    }
convert-skipped-note = В этих записях не было ничего, что можно перенести.
convert-other-diagnostics = { $n ->
        [one] Ещё одна вещь, которую стоит знать
        [few] Ещё { $n } вещи, которые стоит знать
        [many] Ещё { $n } вещей, которые стоит знать
       *[other] Ещё { $n } вещей, которые стоит знать
    }
convert-clean = Позади ничего не осталось — каждая запись файла перешла.
convert-download-title = Скачивание
convert-download-named = Скачать { $name }
convert-download-note = Хранится здесь пятнадцать минут, затем удаляется, поэтому скачайте сейчас.
convert-another = Импортировать ещё файл
admin-history-on = в
admin-history-meta = — { $kind }, { $at }
admin-validation-counts = { $errors ->
        [one] Одна ошибка
        [few] { $errors } ошибки
        [many] { $errors } ошибок
       *[other] { $errors } ошибок
    }, { $warnings ->
        [one] одно предупреждение
        [few] { $warnings } предупреждения
        [many] { $warnings } предупреждений
       *[other] { $warnings } предупреждений
    }, { $infos ->
        [one] одно замечание
        [few] { $infos } замечания
        [many] { $infos } замечаний
       *[other] { $infos } замечаний
    }.
admin-warnings-never-block = Предупреждения никогда не останавливают — это сведения, а не преграда.
admin-validator-clean = Проверка не сообщила ничего.
record-occupations-help-undated = Занятие записывается с началом и концом, чтобы несколько можно было сравнить на одной оси времени. В этом архиве есть названия занятий, но нет дат к ним — обычное дело после импорта, ведь большинству семейных файлов негде их держать, — поэтому шкалу пока не по чему построить.
record-occupations-help-axis = Занятие — это состояние с длительностью, а не событие на одной дате. Все отрезки делят одну ось, { $from }–{ $to }.
admin-value-not-set = не задано
admin-validation-report = Отчёт о проверке
admin-dedup-complete = Устранение дубликатов завершено
admin-dedup-refused = Устранение дубликатов отклонено
record-birth-order = порядок рождения
record-start-not-recorded = начало не записано
record-end-not-recorded = конец не записан
record-document-no-file = Документ здесь записан, но самого файла нет
panel-selected-person = Выбранный человек

## Полосы поколений

tree-band-generation = Поколение { $g }
tree-band-people = { $n ->
        [one] один человек
        [few] { $n } человека
        [many] { $n } человек
       *[other] { $n } человек
    }
tree-band-unplaced = Без места
tree-band-unplaced-note = { $n ->
        [one] один человек вне семьи — показан, а не пропущен
        [few] { $n } человека вне семьи — показаны, а не пропущены
        [many] { $n } человек вне семьи — показаны, а не пропущены
       *[other] { $n } человек вне семьи — показаны, а не пропущены
    }

## Контролируемый словарь

gender-M = Мужской
gender-F = Женский
gender-NB = Небинарный
gender-unrecorded = Не записан

name-part-given_name = имя
name-part-family_name = фамилия
name-part-patronymic = отчество
name-part-matronymic = матроним
name-part-middle_name = второе имя
name-part-nickname = прозвище
name-part-prefix = приставка
name-part-suffix = окончание
name-part-particle = частица
name-part-part = часть

name-type-primary = основное
name-type-other = другое
name-type-alias = употребительное
name-type-birth = девичье
name-type-married = по мужу
name-type-religious = церковное
name-type-transliteration = транслитерация
name-type-nickname = прозвище

## Что отмечено в записи

note-links = { $n ->
        [one] связь вне семьи, со своими датами и источниками
        [few] { $n } связи вне семьи, со своими датами и источниками
        [many] { $n } связей вне семьи, со своими датами и источниками
       *[other] { $n } связей вне семьи, со своими датами и источниками
    }
note-occupations = { $n ->
        [one] занятие, записанное с началом и концом
        [few] { $n } занятия, записанные с началом и концом
        [many] { $n } занятий, записанных с началом и концом
       *[other] { $n } занятий, записанных с началом и концом
    }
note-birth-imprecise = дата рождения, которую источник не смог установить, показана так, как записана
note-death-imprecise = дата смерти, которую источник не смог установить, показана так, как записана
note-names = { $n ->
        [one] одно записанное имя
        [few] { $n } записанных имени
        [many] { $n } записанных имён
       *[other] { $n } записанных имён
    }
note-transliteration = имя в своей графике рядом с латинской транслитерацией
note-witnessed = { $n ->
        [one] событие, где он был свидетелем, а не участником
        [few] { $n } события, где он был свидетелем, а не участником
        [many] { $n } событий, где он был свидетелем, а не участником
       *[other] { $n } событий, где он был свидетелем, а не участником
    }

visibility-public = общедоступно
visibility-members = члены семьи
visibility-contributors = соавторы
visibility-private = закрыто

## Описания строк в списках управления

family-label-couple = { $children ->
        [0] { $a } и { $b }
        [one] { $a } и { $b } — один ребёнок
        [few] { $a } и { $b } — { $children } ребёнка
        [many] { $a } и { $b } — { $children } детей
       *[other] { $a } и { $b } — { $children } детей
    }
family-label-half = { $children ->
        [0] { $a } и { $unknown }
        [one] { $a } и { $unknown } — один ребёнок
        [few] { $a } и { $unknown } — { $children } ребёнка
        [many] { $a } и { $unknown } — { $children } детей
       *[other] { $a } и { $unknown } — { $children } детей
    }
family-label-children = { $others ->
        [0] { $first } — родители не записаны
        [one] { $first } и ещё один ребёнок — родители не записаны
        [few] { $first } и ещё { $others } ребёнка — родители не записаны
        [many] { $first } и ещё { $others } детей — родители не записаны
       *[other] { $first } и ещё { $others } детей — родители не записаны
    }
family-label-empty = Семья без записанных людей

event-label = { $category } — { $who }, { $date }
event-label-nobody = { $category } — { $date }
event-two-people = { $a } и { $b }
event-more-people = { $a } и { $b } и ещё { $others ->
        [one] один человек
        [few] { $others } человека
        [many] { $others } человек
       *[other] { $others } человек
    }

link-label = { $label }: { $from } → { $to }
occupation-label = { $who } — { $title }
source-label = { $title } — { $reliability }
source-label-plain = { $title }
document-label = { $filename } — { $type }
document-label-untitled = { $type } без названия
list-unnamed = { $kind } без названия

## Словари спецификации в списках

event-category-birth = Рождение
event-category-death = Смерть
event-category-marriage = Брак
event-category-divorce = Развод
event-category-baptism = Крещение
event-category-burial = Погребение
event-category-immigration = Иммиграция
event-category-emigration = Эмиграция
event-category-census = Перепись
event-category-residence = Место жительства
event-category-military = Военная служба
event-category-education = Образование
event-category-other = Событие

reliability-primary = первичный источник
reliability-secondary = вторичный источник
reliability-tertiary = обобщающая работа
reliability-recollection = устное свидетельство
reliability-derivative = производная работа
reliability-authored = авторская работа
reliability-oral = устное предание
reliability-unknown = достоверность неизвестна

document-type-photo = фотография
document-type-certificate = свидетельство
document-type-letter = письмо
document-type-record = архивная запись
document-type-newspaper = газетная вырезка
document-type-other = документ

## Где эта запись могла бы сказать больше

completeness-title = Где это древо могло бы сказать больше
completeness-intro = Что записано, а что пока пусто.
completeness-import-title = Что принёс импорт
completeness-import-intro = Подсчитано по файлу, который вы только что загрузили. Пустая строка — это то, чего исходный файл не записывал, а не то, что импорт потерял.

completeness-headline-full = Каждый вид подробностей ниже где-нибудь в этом древе записан.
completeness-headline-empty = { $total ->
        [one] Единственный вид подробностей ниже пока нигде не записан.
        [few] Ни один из { $total } видов подробностей ниже пока не записан.
        [many] Ни один из { $total } видов подробностей ниже пока не записан.
       *[other] Ни один из { $total } видов подробностей ниже пока не записан.
    } Каждый из них — место, где запись могла бы сказать больше.
completeness-headline-partial = { $carried ->
        [one] Один вид подробностей ниже записан
        [few] { $carried } вида подробностей ниже записаны
        [many] { $carried } видов подробностей ниже записаны
       *[other] { $carried } видов подробностей ниже записаны
    }; { $empty ->
        [one] один пока пуст
        [few] { $empty } пока пусты
        [many] { $empty } пока пусты
       *[other] { $empty } пока пусты
    }.

completeness-metric-confidence = Насколько достоверен каждый факт
completeness-metric-confidence-none = Ни один из { $slots } фактов здесь не говорит, насколько он достоверен. Дата, списанная со свидетельства, и дата, которую угадали, выглядят одинаково — пока не перестанут.
completeness-metric-confidence-uniform = { $with } из { $slots } фактов несут оценку, и все они — одно и то же число ({ $modal }). Именно это оставляет после себя массовый импорт: значение по умолчанию, к которому никто не возвращался. Ни один пока не оценён отдельно.
completeness-metric-confidence-some = { $with } из { $slots } фактов несут оценку. { $modal_count } делят одно значение ({ $modal }); { $assessed } отличаются от него, а значит их просмотрели по одному.
completeness-metric-confidence-many = { $with } из { $slots } фактов несут оценку, из них { $assessed } отличаются от самого частого значения ({ $modal }), по { $distinct } различным уровням. Это древо записывает настоящую, разную неуверенность.

completeness-metric-parentage = Насколько достоверна каждая связь родитель — ребёнок
completeness-metric-parentage-none = Ни одно происхождение здесь не говорит, насколько оно достоверно. Усыновления, спорные линии и восстановления по одному упоминанию — это ровно те места, где семье нужно записать сомнение, и древо рисует менее достоверную связь бледнее.
completeness-metric-parentage-some = { $n ->
        [one] Одно происхождение несёт свою оценку
        [few] { $n } происхождения несут свою оценку
        [many] { $n } происхождений несут свою оценку
       *[other] { $n } происхождений несут свою оценку
    }, поэтому предположительная линия заметно слабее документированной.

completeness-metric-links = Связи помимо крови и брака
completeness-metric-links-none = Восприемники, работодатели, свидетели, наставники, опекуны. Пока не записано ни одной. У каждой могут быть свои даты, свой источник и ваша степень уверенности.
completeness-metric-links-some = { $n ->
        [one] Одна записана, со своими датами, источником и вашей степенью уверенности.
        [few] { $n } записаны, у каждой свои даты, источник и ваша степень уверенности.
        [many] { $n } записано, у каждой свои даты, источник и ваша степень уверенности.
       *[other] { $n } записано, у каждой свои даты, источник и ваша степень уверенности.
    }

completeness-metric-occupations = Занятия, записанные с началом и концом
completeness-metric-occupations-none = Занятий не записано. Ремесло, которым занимались тридцать лет, говорит о жизни больше, чем одна датированная строка.
completeness-metric-occupations-undated = { $total ->
        [one] Записано одно занятие, без дат
        [few] Записаны { $total } занятия, без дат
        [many] Записано { $total } занятий, без дат
       *[other] Записано { $total } занятий, без дат
    }. Добавьте начало и конец — и их можно будет сравнить бок о бок на одной оси времени.
completeness-metric-occupations-some = У { $span } из { $total } есть начало или конец, поэтому их можно сравнить бок о бок на одной оси времени.

completeness-metric-sources = Источники с оценкой достоверности
completeness-metric-sources-none = Источников не записано. Указание, откуда взялся факт, — это то, что позволяет родственнику проверить его позже или не согласиться и объяснить почему.
completeness-metric-sources-some = { $graded } из { $total } говорят, насколько они сильны, поэтому утверждение, опирающееся на свидетельство о рождении, заметно не то же, что опирающееся на воспоминание.

completeness-what-is-recorded = Что запись может сказать
completeness-in-this-tree = В этом древе
completeness-not-yet = пока не записано

## Роли участника события

role-spouse = супруг
role-spouse_1 = первый супруг
role-spouse_2 = второй супруг
role-subject = лицо, о котором запись
role-participant = участник
role-witness = свидетель
role-officiant = совершающий обряд
role-informant = заявитель
role-godparent = восприемник

phys-no-source = без источника
phys-col-date = Когда
phys-col-source = Источник
phys-col-confidence = Достоверность
phys-col-note = Примечание
phys-field-height-cm = Рост
phys-field-weight-kg = Вес
phys-field-eye-colour = Цвет глаз
phys-field-hair-colour = Цвет волос
phys-field-build = Телосложение
phys-field-handedness = Ведущая рука
phys-field-features = Особые приметы
phys-field-military = Военная служба
phys-field-languages = Языки
phys-field-blood-group = Группа крови
phys-field-conditions = Известные заболевания
phys-field-operations = Операции и травмы
phys-field-cause-of-death = Причина смерти
phys-field-religion = Вероисповедание или принадлежность
phys-field-health-notes = Примечания
admin-export-health-note = Обычный экспорт исключает все чувствительные категории — здоровье и убеждения, биометрию, геномные данные и судимости — а также поведенческий профиль любого живого человека, так что файл, отправленный родственнику, не содержит ни одной из них. Отметьте, что должен содержать конкретный файл; сам архив записывает, какие категории исключены.
avatar-picker-title = Выбрать изображение
avatar-choose-link = Выбрать изображение
avatar-choose = Какое изображение представляет этого человека
avatar-mode-auto = Пусть выберет программа
avatar-mode-auto-note = Первый портрет, а если его нет — первое изображение, связанное с этой записью.
avatar-mode-none = Показывать инициалы
avatar-mode-none-note = Для записи, где изображения — документы, а не лица.
avatar-focal-hint = Щёлкните изображение, чтобы выбрать его, и щёлкните ещё раз по той части, которая должна остаться в кадре: аватар квадратный, а большинство сканов — нет.
avatar-no-images = С этой записью пока не связано ни одного изображения.
avatar-upload-title = Загрузить изображение и использовать его
avatar-upload-button = Загрузить и сделать изображением
avatar-not-available-title = Это изображение недоступно
avatar-not-available-detail = Выбранный файл не связан с этим человеком либо вам не разрешено его читать.

record-history-withheld = не показано вам

## A death nobody wrote down
#
# GEDCOM cannot say "died, date unknown", so a converter marks those
# people living. These three say that the page worked it out rather than
# read it, which is a distinction this product does not blur.

record-life-status = Статус
record-presumed-deceased = смерть предполагается
record-presumed-short = предпол.
record-presumed-why = Смерть не записана, а рождение было более { $years } лет назад, поэтому запись не может быть верной. Архив не изменён: это вывод страницы, а не то, что говорит источник.

## Фигура, которая рисуется рядом с записью
#
# Это не портрет. Все сообщения ниже существуют для того, чтобы это оставалось
# однозначным.

silhouette-label = Записанные возраст и рост, а не внешность
silhouette-not-a-likeness = Это не портрет: ничто в нём не взято с фотографии.
silhouette-proportions-infant = Пропорции младенца, около четырёх высот головы.
silhouette-proportions-child = Пропорции маленького ребёнка, около шести высот головы.
silhouette-proportions-adolescent = Пропорции подростка, около семи высот головы.
silhouette-proportions-adult = Пропорции взрослого, около семи с половиной высот головы.
silhouette-proportions-elderly = Пропорции взрослого, которые перестают меняться примерно к двадцати годам: фигура не отличает пожилого человека от более молодого взрослого, а даты выше отличают.
silhouette-to-scale = Нарисовано в масштабе относительно опорной линии на { $ref } см.
silhouette-no-height = Рост не записан, поэтому фигура нарисована в условном размере для своей возрастной группы, а не в масштабе.
silhouette-several-heights = Записано больше одного роста; фигура показывает последний по времени.

## The identity editor

identity-editor-title = Имена и личность
identity-primary-name = Имя, показываемое везде
identity-primary-help = То, что используют карточка дерева, заголовок и все списки. Другие имена ниже — это те, которыми источник называл человека в другое время.
identity-display = Имя
identity-display-latin = Латиницей
identity-culture = Язык
identity-direction = Направление письма
identity-direction-ltr = слева направо
identity-direction-rtl = справа налево
identity-direction-auto = по тексту
identity-components = Части имени
identity-components-help = Какая часть — имя, а какая — фамилия, в порядке записи. Запись без частей всё равно отображается: части нужны для поиска.
identity-part = Часть
identity-value = Текст
identity-other-names = Другие имена
identity-other-help = Фамилия по браку, церковное имя, имя из более позднего документа. У каждого — время употребления и источник.
identity-name-type = Вид имени
identity-valid-from = Употреблялось с
identity-valid-until = Употреблялось до
identity-about = О человеке
identity-living-help = Это признак, поставленный источником. Страница отдельно предполагает смерть, если рождение слишком давнее, и это предположение никогда не меняет ни это поле, ни архив.
identity-error-no-display = Записи нужно имя, под которым она отображается. Ничего не сохранено.
editor-blank-to-remove = Очистите имя, чтобы удалить эту запись.
# A BCP 47 tag, shown as an example in the language field.
identity-culture-placeholder = ru
identity-edit-link = Изменить имена и личность

## Union types, statuses and date precision, said out loud

union-type-marriage = брак
union-type-civil_union = гражданский союз
union-type-cohabitation = сожительство
union-type-religious_only = церковный брак
union-type-polygamous = полигамный
union-type-unknown = не записано
union-status-active = продолжается
union-status-ended_by_death = прекращён смертью
union-status-ended_by_divorce = прекращён разводом
union-status-ended_by_separation = прекращён раздельным проживанием
union-status-annulled = аннулирован
union-status-unknown = не записано
union-status-ended = прекращён
union-status-ended-by = прекращён: { $reason }
union-reason-death_of_spouse = смерть супруга
precision-exact = до дня
precision-year = до года
precision-month = до месяца
precision-decade = до десятилетия
precision-century = до века
precision-unknown = неизвестна
record-precision = Точность
record-approximate = Приблизительно
record-place = Место

## The relationships editor

family-editor-title = Семья и связи
family-unions = Союзы
family-no-unions = Для этого человека не записано ни одного союза.
family-union-legend = Союз { $n }
family-writes-family = Сохранение изменит запись семьи #{ $id }, общую для обоих. Страница другого человека изменится вместе с ней.
family-partners = Партнёры
family-partner = Партнёр
family-role = Роль
family-children = Дети
family-children-help = Порядок рождения — это утверждение самой записи. Оставленный пустым, он не утверждает ничего: номер из позиции строки был бы фактом, которого никто не записывал.
family-child = Ребёнок
family-birth-order = Порядок рождения
family-the-union = Сам союз
family-type = Вид союза
family-status = Состояние
family-started = Начало
family-ended = Окончание
family-leave = Убрать этого человека из союза
family-open-entity = Открыть запись семьи
family-new-union = Новый союз
family-new-union-help = Это создаст новую запись семьи с этим человеком. Партнёр необязателен: родитель, названный без партнёра, — это союз из одного.
family-create-union = Создать союз
family-parents = Родители
family-no-parents = Этот человек не записан ребёнком ни в одной семье.
family-child-of = Ребёнок в этой семье
family-detach-child = Убрать этого человека из этой семьи
family-attach-parents = Привязать к родителям
family-attach-help = Выберите семью, в которой этот человек — ребёнок. Он будет добавлен в запись этой семьи, и это появится на страницах родителей.
family-the-family = Семья
family-attach = Привязать
family-error-last-partner = В союзе должен быть хотя бы один человек. Удалите вместо этого запись семьи — тогда будет задан вопрос обо всём, что на неё ссылается.
family-error-no-family = Семья не выбрана. Ничего не сохранено.
family-error-already-child = Этот человек уже записан ребёнком в этой семье.
pick-error-empty = Человек не указан. Ничего не сохранено.
pick-error-not-found = В этом архиве нет человека с таким именем. Ничего не сохранено.
pick-error-ambiguous = Этому соответствует несколько человек. Выберите одного из списка, чтобы запись указывала, кого именно. Ничего не сохранено.

## Links and occupations

links-editor-title = Связи
links-editor-help = Отношения, которые не являются семейными: крёстный, работодатель, свидетель, полк. Каждое — отдельная запись, называющая двоих, поэтому правка здесь меняет и другую запись.
links-none = Для этого человека не записано ни одной связи.
links-new = Новая связь
links-create = Создать связь
links-remove = Удалить связь
links-other-end = Другой конец
links-label = Что это
links-label-reverse = В обратную сторону
links-category = Категория
links-bidirectional = Читается одинаково в обе стороны
links-from = С
links-until = До
links-reversed = Эта связь создана из другой записи. Правка здесь меняет ту же сущность.
link-error-no-label = Связь должна говорить, что она такое. Ничего не сохранено.
occupations-editor-title = Занятия
occupations-editor-help = Занятие — это период с началом и концом, а не название должности. У каждого свои даты и свой источник.
occupations-none = Для этого человека не записано ни одного занятия.
occupations-new = Новое занятие
occupations-create = Создать занятие
occupations-remove = Удалить занятие
occupations-title = Чем занимался
occupations-employer = У кого
occupations-employer-place = Где находился
occupations-from = С
occupations-until = До
occupation-error-no-title = Занятие должно говорить, чем человек занимался. Ничего не сохранено.
link-category-spiritual = духовная
link-category-professional = профессиональная
link-category-social = общественная
link-category-legal = юридическая
link-category-medical = медицинская
link-category-educational = образовательная
link-category-conflict = конфликт
link-category-other = другая
links-edit-link = Изменить связи
occupations-edit-link = Изменить занятия
family-edit-link = Изменить семью и связи

## Events and documents

events-editor-title = События
events-editor-help = Событие называет сразу нескольких человек — венчание, крещение, перепись, — поэтому каждое является отдельной записью и появляется на каждой названной странице.
events-none = Ни одно событие не называет этого человека.
events-new = Новое событие
events-new-help = Этот человек будет добавлен как субъект, если вы не назовёте никого другого. Событие без людей — это просто дата.
events-create = Создать событие
events-remove = Удалить событие
events-category = Что произошло
events-subcategory = Точнее
events-description = Описание
events-participants = Кто участвовал
events-participants-help = Сохранение изменит запись события, которую показывает и каждый другой названный человек.
events-who = Кто
event-error-no-category = Событие должно говорить, что произошло. Ничего не сохранено.
documents-editor-title = Документы
documents-editor-help = На какие файлы указывает эта запись и чем каждый для неё является. Очистка строки отсоединяет файл: документ и его содержимое остаются в архиве.
documents-attached = Прикреплено к этой записи
documents-upload = Загрузить файл
documents-upload-help = До { $mb } МБ. Файл сохраняется в архиве и прикрепляется к этой записи.
documents-caption = Подпись
documents-edit-link = Прикрепить и открепить документы
events-edit-link = Изменить события

## Presentation styles: density, never colour

prefs-style = Плотность
prefs-style-help = Сколько места занимает страница. Независимо от темы, которая отвечает только за цвет, так что любое сочетание возможно.
style-comfortable = Просторная
style-comfortable-note = по умолчанию, с местом для чтения
style-compact = Плотная
style-compact-note = больше записи на экране, чтобы просматривать несколько
style-paper = Бумага
style-paper-note = шрифт с засечками и линейки вместо карточек, для чтения не спеша или печати

## Sensitive classes

admin-export-choose = Включить в этот экспорт
scope-health = Здоровье и убеждения
scope-biometrics = Биометрия
scope-genomics = Геномные данные
scope-legal = Судимости
scope-behaviour = Поведенческие профили живых людей
admin-export-with-chosen = Экспортировать с отмеченным

## Profile

pg-identity = Личность и гражданское состояние
pg-identity-intro = Кем, по записям, был человек и что внесено в акты гражданского состояния.
pg-morphology = Морфология
pg-morphology-intro = Тело, каким его измерили и описали.
pg-biometrics = Биометрия
pg-biometrics-intro = Голос, руки и органы чувств, а также шаблоны, по которым человека можно опознать.
pg-health = Здоровье
pg-health-intro = Болезни, лечение, измерения и результаты анализов.
pg-genomics = Геномика
pg-genomics-intro = ДНК-тесты, гаплогруппы, варианты и другие молекулярные результаты.
pg-death = Смерть
pg-death-intro = Как, когда и где оборвалась жизнь и что стало с телом.
pg-residence = Место жительства и гражданство
pg-residence-intro = Где человек жил, какие государства считали его своим гражданином и на каких языках он говорил.
pg-education = Образование и работа
pg-education-intro = Учёба, квалификация, доходы и имущество.
pg-military = Военная служба и награды
pg-military-intro = Служба, звания, части и награды.
pg-legal = Уголовные дела
pg-legal-intro = Уголовные преследования и их исход.
pg-belief = Вера и принадлежность
pg-belief-intro = Религия, обряды, убеждения и членство.
pg-personality = Личность и поведение
pg-personality-intro = Темперамент, привычки и увлечения — так, как их описывают источники.
pg-relationships = Связи
pg-relationships-intro = Родители, супруги, дети и другие люди в жизни человека.
pg-digital-legacy = Цифровое наследие
pg-digital-legacy-intro = Сканы, модели, записи и архивы, представляющие человека.
pa-identity-titles = Титулы
pa-identity-sex-at-birth = Пол при рождении
pa-identity-gender-identity = Гендерная идентичность
pa-birth-time = Время рождения
pa-birth-coordinates = Место рождения в координатах
pa-civil-status-birth-certificate-number = Номер свидетельства о рождении
pa-civil-status-register-entries = Актовые записи
pa-civil-status-marginal-annotations = Отметки на полях
pa-morphology-height = Рост
pa-morphology-weight = Вес
pa-morphology-bmi = Индекс массы тела
pa-morphology-body-composition = Состав тела
pa-morphology-build = Телосложение
pa-morphology-eye-colour = Цвет глаз
pa-morphology-eye-shape = Форма глаз
pa-morphology-eye-spacing = Посадка глаз
pa-morphology-hair-colour = Натуральный цвет волос
pa-morphology-hair-texture = Тип волос
pa-morphology-hairline = Линия роста волос
pa-morphology-facial-hair = Растительность на лице
pa-morphology-body-hair = Волосяной покров тела
pa-morphology-skin-tone = Фототип (по Фицпатрику)
pa-morphology-skin-undertone = Подтон кожи
pa-morphology-freckles = Веснушки
pa-morphology-pigmentation = Пигментные пятна
pa-morphology-scars = Шрамы
pa-morphology-tattoos = Татуировки
pa-morphology-moles = Родинки
pa-morphology-facial-asymmetries = Асимметрия лица
pa-morphology-face-shape = Форма лица
pa-morphology-nose-shape = Форма носа
pa-morphology-ear-shape = Форма ушей
pa-morphology-lip-shape = Форма губ
pa-morphology-dentition = Зубы
pa-morphology-malocclusion = Аномалия прикуса (класс по Энглю)
pa-morphology-posture = Осанка
pa-morphology-gait = Походка
pa-morphology-distinguishing-features = Особые приметы
pa-biometrics-fingerprints = Отпечатки пальцев
pa-biometrics-retinal-print = Рисунок сетчатки
pa-biometrics-voice-signature = Голосовой отпечаток
pa-biometrics-voice-frequency = Основная частота голоса
pa-biometrics-vocal-timbre = Тембр голоса
pa-biometrics-spoken-accent = Акцент
pa-biometrics-speech-rate = Темп речи
pa-biometrics-verbal-tics = Слова-паразиты
pa-biometrics-frequent-vocabulary = Частая лексика
pa-biometrics-speech-register = Регистр речи
pa-biometrics-motor-tics = Двигательные тики
pa-biometrics-handedness = Ведущая рука
pa-biometrics-hearing = Слух
pa-biometrics-visual-acuity = Острота зрения
pa-biometrics-optical-correction = Коррекция зрения
pa-health-blood-group = Группа крови (AB0)
pa-health-rhesus = Резус-фактор (RhD)
pa-health-blood-pressure = Артериальное давление
pa-health-resting-heart-rate = Пульс в покое
pa-health-respiratory-capacity = Функция внешнего дыхания
pa-health-conditions = Заболевания
pa-health-surgeries = Перенесённые операции
pa-health-injuries = Травмы
pa-health-deformities = Деформации
pa-health-amputations = Ампутации
pa-health-prostheses = Протезы
pa-health-implants = Импланты
pa-health-devices = Имплантированные устройства
pa-health-medications = Лекарства
pa-health-allergies = Аллергии
pa-health-vaccinations = Прививки
pa-health-serology = Серология
pa-health-lab-results = Лабораторные анализы
pa-health-deficiencies = Дефициты
pa-health-sleep-disorders = Нарушения сна
pa-health-mental-health-assessments = Оценки психического здоровья
pa-genomics-autosomal-mapping = Аутосомный ДНК-тест
pa-genomics-y-haplogroup = Гаплогруппа Y-ДНК
pa-genomics-mt-haplogroup = Митохондриальная гаплогруппа
pa-genomics-whole-genome-sequencing = Полногеномное секвенирование
pa-genomics-risk-variants = Варианты риска
pa-genomics-hereditary-conditions = Наследственные заболевания
pa-genomics-predispositions = Предрасположенности
pa-genomics-epigenetic-markers = Эпигенетические маркеры
pa-genomics-epigenetic-age = Эпигенетический возраст
pa-genomics-gut-microbiome = Микробиом кишечника
pa-genomics-skin-microbiome = Микробиом кожи
pa-genomics-toxicological-sensitivities = Чувствительность к лекарствам и токсинам
pa-death-time = Время смерти
pa-death-coordinates = Место смерти в координатах
pa-death-causes = Причины смерти
pa-death-contributing-factors = Сопутствующие факторы
pa-death-autopsy = Вскрытие
pa-death-disposition = Погребение или иное обращение с телом
pa-death-grave = Могила
pa-residence-addresses = Адреса
pa-residence-nationality-of-origin = Гражданство по происхождению
pa-residence-acquired-nationalities = Приобретённые гражданства
pa-residence-mother-tongue = Родной язык
pa-residence-spoken-languages = Языки, которыми владел
pa-education-level = Уровень образования
pa-education-diplomas = Дипломы и степени
pa-education-institutions = Школы и учебные заведения
pa-education-income = Доход
pa-education-real-estate = Недвижимость
pa-military-distinctions = Награды
pa-military-citations = Благодарности в приказах
pa-military-ranks = Звания
pa-military-units = Воинские части
pa-military-service-numbers = Личные номера
pa-legal-criminal-record = Судимости
pa-belief-religions = Вероисповедание
pa-belief-sacraments = Таинства и обряды
pa-belief-beliefs = Убеждения
pa-belief-political-leanings = Политические взгляды
pa-belief-memberships = Членство в организациях
pa-personality-big-five = Баллы «Большой пятёрки»
pa-personality-mbti = Тип MBTI
pa-personality-introversion-extraversion = Интроверсия и экстраверсия
pa-personality-stress-tolerance = Стрессоустойчивость
pa-personality-decision-style = Стиль принятия решений
pa-personality-interests = Интересы
pa-personality-hobbies = Хобби
pa-personality-sports = Спорт
pa-personality-dietary-habits = Питание
pa-personality-dependencies = Зависимости
pa-digital-legacy-body-models = Модели тела
pa-digital-legacy-skin-textures = Текстуры кожи
pa-digital-legacy-rigs = Скелетные риги
pa-digital-legacy-voice-corpora = Записи голоса для синтеза
pa-digital-legacy-text-corpora = Тексты для языковой модели
pa-digital-legacy-digital-traces = Цифровые следы
pa-digital-legacy-carbon-footprint = Углеродный след
pa-digital-legacy-behaviour-models = Модели поведения
pf-identity-titles-text = Титул как в источнике
pf-identity-titles-kind = Вид титула
pf-civil-status-marginal-annotations-text = Отметка
pf-morphology-pigmentation-kind = Вид пятна
pf-biometrics-spoken-accent-description = Как описан
pf-biometrics-optical-correction-kind = Коррекция
pf-health-amputations-level = Уровень ампутации
pf-health-prostheses-kind = Протез
pf-health-implants-kind = Имплант
pf-health-devices-kind = Устройство
pf-health-allergies-type = Вид аллергии
pf-health-vaccinations-status = Статус вакцинации
pf-health-sleep-disorders-category = Категория нарушения
pf-death-autopsy-kind = Вскрытие
pf-education-institutions-name = Название заведения
pf-military-distinctions-name = Название награды
pf-military-distinctions-kind = Вид награды
pf-military-citations-text = Текст благодарности
pf-military-ranks-category = Категория звания
pf-belief-political-leanings-position = Положение на оси «левые — правые»
pf-belief-memberships-kind = Вид организации
pf-digital-legacy-carbon-footprint-method = Способ оценки
pf-age-years = Возраст в годах
pf-agreeableness = Доброжелательность
pf-allergen = Аллерген
pf-amount = Сумма
pf-analyte = Показатель
pf-artefact-type = Вид артефакта
pf-autoimmune = Аутоиммунное
pf-body-region = Область тела
pf-bone-percent = Кости
pf-carrier-status = Носительство
pf-cause = Причина
pf-chronic = Хроническое
pf-clock = Часы
pf-condition = Заболевание
pf-conferred-by = Кем вручено
pf-congenital = Врождённое
pf-conscientiousness = Добросовестность
pf-consent = Согласие
pf-coordinates = Координаты
pf-corrected = С коррекцией
pf-country = Страна
pf-court = Суд
pf-coverage = Покрытие
pf-currency = Валюта
pf-decimal = Острота (десятичная)
pf-denomination = Конфессия
pf-derived-from-id = Получено из
pf-description = Описание
pf-details = Подробности
pf-diagnosis = Диагноз
pf-diameter-mm = Диаметр
pf-diastolic = Диастолическое
pf-diet = Тип питания
pf-document-id = Документ
pf-dose = Доза
pf-ear = Ухо
pf-entry-number = Номер записи
pf-extraversion = Экстраверсия
pf-eye = Глаз
pf-fat-percent = Жир
pf-fev1-fvc-ratio = Индекс ОФВ1/ФЖЕЛ
pf-fev1-litres = ОФВ1
pf-file-format = Формат файла
pf-findings = Заключение
pf-flag = Отметка
pf-format = Формат
pf-fracture = Перелом
pf-fvc-litres = ФЖЕЛ
pf-gene = Ген
pf-generator = Создано в
pf-grade = Степень
pf-iccs-section = Раздел преступлений (ICCS)
pf-icd10-chapter = Класс МКБ-10
pf-indication = Показание
pf-inheritance = Тип наследования
pf-inscription = Надпись
pf-institution = Учебное заведение
pf-instrument = Методика
pf-isced-level = Уровень МСКО
pf-jurisdiction = Юрисдикция
pf-language = Язык
pf-lat = Широта
pf-level = Уровень
pf-lines = Адрес
pf-location = Расположение
pf-lon = Долгота
pf-major = Основная гаплогруппа
pf-marker = Маркер
pf-metaboliser-status = Тип метаболизма
pf-method = Способ
pf-mode = Способ приобретения
pf-muscle-percent = Мышцы
pf-neuroticism = Нейротизм
pf-number = Номер
pf-nutrient = Нутриент
pf-offence = Преступление
pf-office = Орган записи
pf-openness = Открытость опыту
pf-organisation = Организация
pf-outcome = Исход
pf-pace = Темп старения
pf-page = Страница
pf-panel = Панель
pf-party = Партия
pf-pathogen = Возбудитель
pf-pattern = Характер употребления
pf-percentile = Процентиль
pf-period = Период выплаты
pf-place-id = Место
pf-plot = Участок
pf-polygenic-score = Полигенный балл
pf-postal-code = Почтовый индекс
pf-precision = Точность
pf-prescription = Рецепт
pf-proficiency = Уровень владения
pf-provider = Лаборатория
pf-quintile = Квинтиль дохода
pf-rank = Звание
pf-rank-text = Звание как в источнике
pf-reaction = Реакция
pf-reference-build = Референсный геном
pf-reference-high = Верхняя граница нормы
pf-reference-low = Нижняя граница нормы
pf-register-type = Вид записи
pf-result = Результат
pf-role = Роль
pf-sacrament = Таинство или обряд
pf-score = Балл
pf-sentence = Наказание
pf-sequence = Место в цепи причин
pf-service = Вид войск
pf-severity = Тяжесть
pf-shannon-diversity = Индекс Шеннона
pf-shape = Форма
pf-significance = Клиническая значимость
pf-snp-count = Число SNP
pf-sport = Вид спорта
pf-subclade = Субклад
pf-substance = Вещество
pf-summary = Резюме
pf-systolic = Систолическое
pf-tenure = Право владения
pf-test = Тест
pf-threshold-db = Порог слышимости
pf-title = Название
pf-tonnes-co2e-per-year = Выбросы
pf-tradition = Традиция
pf-tree-version = Версия дерева
pf-unit = Единица
pf-use = Назначение
pf-variant = Вариант
pf-volume = Том
pf-zygosity = Зиготность
pu-cm = { $n } см
pu-kg = { $n } кг
pu-kg-m2 = { $n } кг/м²
pu-percent = { $n } %
pu-mm = { $n } мм
pu-hz = { $n } Гц
pu-words-min = { $n } слов/мин
pu-db-hl = { $n } дБ HL
pu-mmhg = { $n } мм рт. ст.
pu-bpm = { $n } уд./мин
pu-litres = { $n } л
pu-coverage = { $n }×
pu-years = { $n } лет
pu-t-co2e-yr = { $n } т CO₂-экв. в год
pv-sensitive-class-health = Здоровье и убеждения
pv-sensitive-class-biometrics = Биометрия
pv-sensitive-class-genomics = Геномные данные
pv-sensitive-class-legal = Судимости
pv-laterality-left = Левое
pv-laterality-right = Правое
pv-laterality-both = С обеих сторон
pv-body-region-head = Голова
pv-body-region-face = Лицо
pv-body-region-neck = Шея
pv-body-region-left-shoulder = Левое плечо
pv-body-region-right-shoulder = Правое плечо
pv-body-region-left-arm = Левая рука
pv-body-region-right-arm = Правая рука
pv-body-region-left-hand = Левая кисть
pv-body-region-right-hand = Правая кисть
pv-body-region-chest = Грудь
pv-body-region-abdomen = Живот
pv-body-region-upper-back = Верхняя часть спины
pv-body-region-lower-back = Нижняя часть спины
pv-body-region-pelvis = Таз и бёдра
pv-body-region-left-leg = Левая нога
pv-body-region-right-leg = Правая нога
pv-body-region-left-foot = Левая стопа
pv-body-region-right-foot = Правая стопа
pv-body-region-internal = Внутри тела
pv-body-region-whole-body = Всё тело
pv-body-region-other = Другая область
pv-artefact-type-mesh = Полигональная сетка
pv-artefact-type-point-cloud = Облако точек
pv-artefact-type-skin-texture-map = Карта текстуры кожи
pv-artefact-type-skeletal-rig = Скелетный риг
pv-artefact-type-voice-corpus = Корпус записей голоса
pv-artefact-type-text-corpus = Корпус текстов
pv-artefact-type-trace-archive = Архив активности в сети
pv-artefact-type-behaviour-model = Модель поведения
pv-artefact-type-fingerprint-card = Дактилокарта
pv-artefact-type-fingerprint-template = Шаблон отпечатка
pv-artefact-type-retinal-image = Снимок сетчатки
pv-artefact-type-voiceprint = Голосовой отпечаток
pv-consent-given = Дано
pv-consent-given-by-estate = Дано наследниками
pv-consent-refused = Отказано
pv-consent-withdrawn = Отозвано
pv-consent-not-asked = Не запрашивалось
pv-consent-unknown = Неизвестно
pv-sex-at-birth-female = Женский
pv-sex-at-birth-male = Мужской
pv-sex-at-birth-intersex = Интерсекс
pv-sex-at-birth-undetermined = Не определён
pv-sex-at-birth-unknown = Неизвестен
pv-gender-identity-woman = Женщина
pv-gender-identity-man = Мужчина
pv-gender-identity-non-binary = Небинарная персона
pv-gender-identity-other = Иная
pv-gender-identity-undisclosed = Не раскрыта
pv-gender-identity-unknown = Неизвестна
pv-title-kind-nobility = Дворянский
pv-title-kind-academic = Учёный
pv-title-kind-professional = Профессиональный
pv-title-kind-religious = Церковный
pv-title-kind-military = Военный
pv-title-kind-civic = Почётный
pv-title-kind-courtesy = Учтивый
pv-title-kind-other = Иной
pv-register-type-birth = Рождение
pv-register-type-baptism = Крещение
pv-register-type-marriage = Брак
pv-register-type-death = Смерть
pv-register-type-burial = Погребение
pv-register-type-divorce = Развод
pv-register-type-recognition = Установление отцовства
pv-register-type-legitimation = Узаконение
pv-register-type-adoption = Усыновление
pv-register-type-name-change = Перемена имени
pv-register-type-other = Иная
pv-build-slight = Хрупкое
pv-build-slim = Худощавое
pv-build-average = Среднее
pv-build-sturdy = Коренастое
pv-build-stout = Плотное
pv-build-heavy = Крупное
pv-eye-colour-light-blue = Голубой
pv-eye-colour-blue = Синий
pv-eye-colour-dark-blue = Тёмно-синий
pv-eye-colour-grey = Серый
pv-eye-colour-blue-grey = Серо-голубой
pv-eye-colour-green = Зелёный
pv-eye-colour-grey-green = Серо-зелёный
pv-eye-colour-hazel = Орехово-карий
pv-eye-colour-amber = Янтарный
pv-eye-colour-light-brown = Светло-карий
pv-eye-colour-brown = Карий
pv-eye-colour-dark-brown = Тёмно-карий
pv-eye-colour-black = Чёрный
pv-eye-colour-mixed = Смешанный
pv-eye-colour-other = Иной
pv-eye-shape-almond = Миндалевидные
pv-eye-shape-round = Круглые
pv-eye-shape-hooded = С нависающим веком
pv-eye-shape-monolid = Без складки века
pv-eye-shape-deep-set = Глубоко посаженные
pv-eye-shape-protruding = Выпуклые
pv-eye-shape-upturned = Приподнятые
pv-eye-shape-downturned = Опущенные
pv-eye-shape-other = Иная
pv-eye-spacing-close-set = Близко посаженные
pv-eye-spacing-average = Обычная
pv-eye-spacing-wide-set = Широко расставленные
pv-hair-colour-black = Чёрный
pv-hair-colour-dark-brown = Тёмно-каштановый
pv-hair-colour-brown = Каштановый
pv-hair-colour-light-brown = Светло-каштановый
pv-hair-colour-auburn = Медно-каштановый
pv-hair-colour-red = Рыжий
pv-hair-colour-strawberry-blond = Рыжеватый блонд
pv-hair-colour-dark-blond = Тёмно-русый
pv-hair-colour-blond = Русый
pv-hair-colour-light-blond = Светло-русый
pv-hair-colour-grey = Седой
pv-hair-colour-white = Белый
pv-hair-colour-none = Нет волос
pv-hair-colour-other = Иной
pv-hair-texture-straight = Прямые
pv-hair-texture-wavy = Волнистые
pv-hair-texture-curly = Кудрявые
pv-hair-texture-coily = Мелкие кольца
pv-hair-texture-other = Иные
pv-hairline-straight = Прямая
pv-hairline-rounded = Округлая
pv-hairline-widows-peak = Мыском
pv-hairline-m-shaped = М-образная
pv-hairline-bell-shaped = Колоколом
pv-hairline-uneven = Неровная
pv-hairline-receding = Залысины
pv-hairline-bald = Лысина
pv-facial-hair-none = Нет
pv-facial-hair-stubble = Щетина
pv-facial-hair-moustache = Усы
pv-facial-hair-goatee = Эспаньолка
pv-facial-hair-full-beard = Борода
pv-facial-hair-sideburns = Бакенбарды
pv-facial-hair-other = Иная
pv-body-hair-none = Нет
pv-body-hair-sparse = Редкий
pv-body-hair-moderate = Умеренный
pv-body-hair-dense = Густой
pv-skin-tone-type-i = Тип I — всегда обгорает, не загорает
pv-skin-tone-type-ii = Тип II — обычно обгорает, загорает слабо
pv-skin-tone-type-iii = Тип III — иногда обгорает, загорает ровно
pv-skin-tone-type-iv = Тип IV — редко обгорает, хорошо загорает
pv-skin-tone-type-v = Тип V — обгорает очень редко
pv-skin-tone-type-vi = Тип VI — никогда не обгорает
pv-skin-undertone-cool = Холодный
pv-skin-undertone-neutral = Нейтральный
pv-skin-undertone-warm = Тёплый
pv-skin-undertone-olive = Оливковый
pv-freckles-none = Нет
pv-freckles-few = Немного
pv-freckles-moderate = Умеренно
pv-freckles-many = Много
pv-pigmentation-mark-birthmark = Родимое пятно
pv-pigmentation-mark-port-wine-stain = Винное пятно
pv-pigmentation-mark-cafe-au-lait-spot = Пятно цвета кофе с молоком
pv-pigmentation-mark-depigmented-patch = Депигментированное пятно
pv-pigmentation-mark-hyperpigmented-patch = Гиперпигментированное пятно
pv-pigmentation-mark-other = Иное
pv-mole-shape-round = Круглая
pv-mole-shape-oval = Овальная
pv-mole-shape-irregular = Неправильная
pv-mole-shape-other = Иная
pv-face-shape-oval = Овальное
pv-face-shape-round = Круглое
pv-face-shape-square = Квадратное
pv-face-shape-oblong = Вытянутое
pv-face-shape-heart = Сердцевидное
pv-face-shape-diamond = Ромбовидное
pv-face-shape-triangular = Треугольное
pv-nose-shape-straight = Прямой
pv-nose-shape-aquiline = С горбинкой
pv-nose-shape-snub = Курносый
pv-nose-shape-upturned = Вздёрнутый
pv-nose-shape-flat = Приплюснутый
pv-nose-shape-broad = Широкий
pv-nose-shape-bulbous = Картофелиной
pv-nose-shape-crooked = Искривлённый
pv-nose-shape-other = Иной
pv-ear-shape-free-lobe = Свободные мочки
pv-ear-shape-attached-lobe = Приросшие мочки
pv-ear-shape-protruding = Оттопыренные
pv-ear-shape-close-set = Прижатые
pv-ear-shape-pointed = Заострённые
pv-ear-shape-other = Иные
pv-lip-shape-thin = Тонкие
pv-lip-shape-medium = Средние
pv-lip-shape-full = Пухлые
pv-lip-shape-bow-shaped = Луком
pv-lip-shape-wide = Широкие
pv-lip-shape-downturned = С опущенными уголками
pv-lip-shape-other = Иные
pv-dentition-primary = Молочные
pv-dentition-mixed = Сменные
pv-dentition-permanent-complete = Постоянные, полный набор
pv-dentition-permanent-partial-loss = Постоянные, с потерями
pv-dentition-edentulous = Беззубость
pv-dentition-partial-denture = Частичный протез
pv-dentition-full-denture = Полный протез
pv-dentition-implants = Зубные импланты
pv-malocclusion-normal = Нормальный прикус
pv-malocclusion-class-i = Класс I
pv-malocclusion-class-ii-division-1 = Класс II, подкласс 1
pv-malocclusion-class-ii-division-2 = Класс II, подкласс 2
pv-malocclusion-class-iii = Класс III
pv-posture-ideal = Правильная
pv-posture-kyphotic-lordotic = Кифолордотическая
pv-posture-flat-back = Плоская спина
pv-posture-sway-back = Плоско-вогнутая спина
pv-posture-stooped = Сутулая
pv-posture-scoliotic = Сколиотическая
pv-posture-other = Иная
pv-gait-brisk = Быстрая
pv-gait-average = Обычная
pv-gait-slow = Медленная
pv-gait-shuffling = Шаркающая
pv-gait-limping = Хромающая
pv-gait-waddling = Утиная
pv-gait-unsteady = Неустойчивая
pv-gait-stiff = Скованная
pv-gait-other = Иная
pv-vocal-timbre-bright = Звонкий
pv-vocal-timbre-dark = Глухой
pv-vocal-timbre-warm = Тёплый
pv-vocal-timbre-breathy = С придыханием
pv-vocal-timbre-nasal = Гнусавый
pv-vocal-timbre-hoarse = Хриплый
pv-vocal-timbre-resonant = Звучный
pv-vocal-timbre-thin = Тонкий
pv-vocal-timbre-other = Иной
pv-speech-register-frozen = Ритуальный
pv-speech-register-formal = Официальный
pv-speech-register-consultative = Нейтральный
pv-speech-register-casual = Разговорный
pv-speech-register-intimate = Интимный
pv-handedness-left = Левша
pv-handedness-right = Правша
pv-handedness-ambidextrous = Амбидекстр
pv-handedness-mixed = Смешанная
pv-handedness-unknown = Неизвестно
pv-hearing-grade-normal = Нормальный
pv-hearing-grade-mild = Лёгкая тугоухость
pv-hearing-grade-moderate = Умеренная тугоухость
pv-hearing-grade-moderately-severe = Умеренно тяжёлая тугоухость
pv-hearing-grade-severe = Тяжёлая тугоухость
pv-hearing-grade-profound = Глубокая тугоухость
pv-hearing-grade-complete = Полная глухота
pv-optical-correction-none = Нет
pv-optical-correction-glasses = Очки
pv-optical-correction-contact-lenses = Контактные линзы
pv-optical-correction-glasses-and-contact-lenses = Очки и линзы
pv-optical-correction-refractive-surgery = Рефракционная хирургия
pv-optical-correction-intraocular-lens = Интраокулярная линза
pv-optical-correction-other = Иная
pv-rhesus-positive = RhD положительный
pv-rhesus-negative = RhD отрицательный
pv-rhesus-weak-d = Слабый D
pv-rhesus-unknown = Неизвестен
pv-icd10-chapter-infectious-parasitic = I Инфекционные и паразитарные болезни
pv-icd10-chapter-neoplasms = II Новообразования
pv-icd10-chapter-blood-immune = III Болезни крови и иммунной системы
pv-icd10-chapter-endocrine-metabolic = IV Эндокринные, пищевые и обменные болезни
pv-icd10-chapter-mental-behavioural = V Психические расстройства и расстройства поведения
pv-icd10-chapter-nervous-system = VI Болезни нервной системы
pv-icd10-chapter-eye-adnexa = VII Болезни глаза и придатков
pv-icd10-chapter-ear-mastoid = VIII Болезни уха и сосцевидного отростка
pv-icd10-chapter-circulatory = IX Болезни системы кровообращения
pv-icd10-chapter-respiratory = X Болезни органов дыхания
pv-icd10-chapter-digestive = XI Болезни органов пищеварения
pv-icd10-chapter-skin = XII Болезни кожи и подкожной клетчатки
pv-icd10-chapter-musculoskeletal = XIII Болезни костно-мышечной системы
pv-icd10-chapter-genitourinary = XIV Болезни мочеполовой системы
pv-icd10-chapter-pregnancy-childbirth = XV Беременность и роды
pv-icd10-chapter-perinatal = XVI Перинатальные состояния
pv-icd10-chapter-congenital = XVII Врождённые аномалии
pv-icd10-chapter-ill-defined = XVIII Симптомы и неуточнённые причины
pv-icd10-chapter-injury-poisoning = XIX Травмы и отравления
pv-icd10-chapter-external-causes = XX Внешние причины
pv-icd10-chapter-health-factors = XXI Факторы, влияющие на здоровье
pv-icd10-chapter-special-purposes = XXII Коды для особых целей
pv-diagnosis-status-diagnosed = Установлен врачом
pv-diagnosis-status-suspected = Предполагаемый
pv-diagnosis-status-self-reported = Со слов семьи
pv-diagnosis-status-unknown = Неизвестно
pv-prosthesis-kind-limb = Конечность
pv-prosthesis-kind-joint = Эндопротез сустава
pv-prosthesis-kind-ocular = Глазной
pv-prosthesis-kind-dental = Зубной
pv-prosthesis-kind-auditory = Слуховой
pv-prosthesis-kind-breast = Грудной
pv-prosthesis-kind-other = Иной
pv-implant-kind-orthopaedic = Ортопедический
pv-implant-kind-dental = Зубной
pv-implant-kind-cochlear = Кохлеарный
pv-implant-kind-breast = Грудной
pv-implant-kind-intraocular-lens = Интраокулярная линза
pv-implant-kind-contraceptive = Контрацептивный
pv-implant-kind-cosmetic = Косметический
pv-implant-kind-other = Иной
pv-device-kind-pacemaker = Кардиостимулятор
pv-device-kind-implantable-defibrillator = Имплантируемый дефибриллятор
pv-device-kind-cardiac-resynchronisation = Ресинхронизирующее устройство
pv-device-kind-ventricular-assist = Устройство поддержки желудочков
pv-device-kind-neurostimulator = Нейростимулятор
pv-device-kind-insulin-pump = Инсулиновая помпа
pv-device-kind-drug-port = Порт-система
pv-device-kind-shunt = Шунт
pv-device-kind-stent = Стент
pv-device-kind-other = Иное
pv-allergy-type-drug = Лекарство
pv-allergy-type-food = Пища
pv-allergy-type-environmental = Окружающая среда
pv-allergy-type-insect-venom = Яд насекомых
pv-allergy-type-latex = Латекс
pv-allergy-type-other = Иное
pv-allergy-severity-mild = Лёгкая
pv-allergy-severity-moderate = Средняя
pv-allergy-severity-severe = Тяжёлая
pv-allergy-severity-anaphylactic = Анафилактическая
pv-allergy-severity-unknown = Неизвестна
pv-pathogen-diphtheria = Дифтерия
pv-pathogen-tetanus = Столбняк
pv-pathogen-pertussis = Коклюш
pv-pathogen-poliomyelitis = Полиомиелит
pv-pathogen-measles = Корь
pv-pathogen-mumps = Эпидемический паротит
pv-pathogen-rubella = Краснуха
pv-pathogen-varicella = Ветряная оспа
pv-pathogen-smallpox = Натуральная оспа
pv-pathogen-tuberculosis = Туберкулёз
pv-pathogen-hepatitis-a = Гепатит A
pv-pathogen-hepatitis-b = Гепатит B
pv-pathogen-hepatitis-c = Гепатит C
pv-pathogen-haemophilus-influenzae-b = Гемофильная инфекция типа b
pv-pathogen-pneumococcal = Пневмококковая инфекция
pv-pathogen-meningococcal = Менингококковая инфекция
pv-pathogen-human-papillomavirus = Вирус папилломы человека
pv-pathogen-influenza = Грипп
pv-pathogen-covid-19 = COVID-19
pv-pathogen-rotavirus = Ротавирус
pv-pathogen-yellow-fever = Жёлтая лихорадка
pv-pathogen-typhoid = Брюшной тиф
pv-pathogen-cholera = Холера
pv-pathogen-rabies = Бешенство
pv-pathogen-japanese-encephalitis = Японский энцефалит
pv-pathogen-tick-borne-encephalitis = Клещевой энцефалит
pv-pathogen-hiv = ВИЧ
pv-pathogen-syphilis = Сифилис
pv-pathogen-toxoplasmosis = Токсоплазмоз
pv-pathogen-cytomegalovirus = Цитомегаловирус
pv-pathogen-epstein-barr = Вирус Эпштейна — Барр
pv-pathogen-other = Иной
pv-vaccination-status-vaccinated = Привит
pv-vaccination-status-partially-vaccinated = Привит частично
pv-vaccination-status-unvaccinated = Не привит
pv-vaccination-status-contraindicated = Противопоказано
pv-vaccination-status-unknown = Неизвестно
pv-serology-result-positive = Положительный
pv-serology-result-negative = Отрицательный
pv-serology-result-equivocal = Сомнительный
pv-serology-result-unknown = Неизвестен
pv-lab-panel-basic-metabolic = Базовый метаболический профиль
pv-lab-panel-lipid = Липидный профиль
pv-lab-panel-liver = Печёночные пробы
pv-lab-panel-renal = Почечный профиль
pv-lab-panel-glycated-haemoglobin = Гликированный гемоглобин
pv-lab-panel-iron = Обмен железа
pv-lab-analyte-sodium = Натрий
pv-lab-analyte-potassium = Калий
pv-lab-analyte-chloride = Хлориды
pv-lab-analyte-bicarbonate = Бикарбонат
pv-lab-analyte-urea = Мочевина
pv-lab-analyte-creatinine = Креатинин
pv-lab-analyte-glucose = Глюкоза
pv-lab-analyte-calcium = Кальций
pv-lab-analyte-total-cholesterol = Общий холестерин
pv-lab-analyte-ldl-cholesterol = Холестерин ЛПНП
pv-lab-analyte-hdl-cholesterol = Холестерин ЛПВП
pv-lab-analyte-triglycerides = Триглицериды
pv-lab-analyte-non-hdl-cholesterol = Холестерин не-ЛПВП
pv-lab-analyte-alt = Аланинаминотрансфераза (АЛТ)
pv-lab-analyte-ast = Аспартатаминотрансфераза (АСТ)
pv-lab-analyte-alp = Щелочная фосфатаза (ЩФ)
pv-lab-analyte-ggt = Гамма-глутамилтрансфераза (ГГТ)
pv-lab-analyte-total-bilirubin = Общий билирубин
pv-lab-analyte-direct-bilirubin = Прямой билирубин
pv-lab-analyte-albumin = Альбумин
pv-lab-analyte-total-protein = Общий белок
pv-lab-analyte-egfr = Расчётная СКФ
pv-lab-analyte-uric-acid = Мочевая кислота
pv-lab-analyte-phosphate = Фосфат
pv-lab-analyte-urine-albumin-creatinine-ratio = Отношение альбумин/креатинин в моче
pv-lab-analyte-hba1c = HbA1c
pv-lab-analyte-serum-iron = Сывороточное железо
pv-lab-analyte-ferritin = Ферритин
pv-lab-analyte-transferrin = Трансферрин
pv-lab-analyte-transferrin-saturation = Насыщение трансферрина
pv-lab-analyte-tibc = Общая железосвязывающая способность
pv-lab-flag-low = Ниже нормы
pv-lab-flag-normal = В норме
pv-lab-flag-high = Выше нормы
pv-lab-flag-critical-low = Критически низкий
pv-lab-flag-critical-high = Критически высокий
pv-nutrient-vitamin-a = Витамин A
pv-nutrient-thiamine = Тиамин (B1)
pv-nutrient-riboflavin = Рибофлавин (B2)
pv-nutrient-niacin = Ниацин (B3)
pv-nutrient-vitamin-b6 = Витамин B6
pv-nutrient-folate = Фолиевая кислота (B9)
pv-nutrient-vitamin-b12 = Витамин B12
pv-nutrient-vitamin-c = Витамин C
pv-nutrient-vitamin-d = Витамин D
pv-nutrient-vitamin-e = Витамин E
pv-nutrient-vitamin-k = Витамин K
pv-nutrient-iron = Железо
pv-nutrient-zinc = Цинк
pv-nutrient-magnesium = Магний
pv-nutrient-calcium = Кальций
pv-nutrient-iodine = Йод
pv-nutrient-selenium = Селен
pv-nutrient-copper = Медь
pv-nutrient-potassium = Калий
pv-nutrient-phosphorus = Фосфор
pv-nutrient-other = Иное
pv-sleep-disorder-insomnia = Бессонница
pv-sleep-disorder-sleep-related-breathing = Нарушения дыхания во сне
pv-sleep-disorder-central-hypersomnolence = Центральная гиперсомния
pv-sleep-disorder-circadian-rhythm = Нарушения циркадного ритма
pv-sleep-disorder-parasomnia = Парасомнии
pv-sleep-disorder-sleep-related-movement = Двигательные расстройства сна
pv-sleep-disorder-other = Иное
pv-assessment-instrument-phq-9 = PHQ-9
pv-assessment-instrument-gad-7 = GAD-7
pv-assessment-instrument-bdi-ii = BDI-II
pv-assessment-instrument-hads = HADS
pv-assessment-instrument-k10 = K10
pv-assessment-instrument-gds-15 = GDS-15
pv-assessment-instrument-mmse = MMSE
pv-assessment-instrument-moca = MoCA
pv-assessment-instrument-audit = AUDIT
pv-assessment-instrument-clinical-interview = Клиническое интервью
pv-assessment-instrument-other = Иная
pv-assessment-severity-none-minimal = Нет или минимальная
pv-assessment-severity-mild = Лёгкая
pv-assessment-severity-moderate = Умеренная
pv-assessment-severity-moderately-severe = Умеренно тяжёлая
pv-assessment-severity-severe = Тяжёлая
pv-reference-build-grch36 = GRCh36 (hg18)
pv-reference-build-grch37 = GRCh37 (hg19)
pv-reference-build-grch38 = GRCh38 (hg38)
pv-reference-build-t2t-chm13 = T2T-CHM13
pv-genomic-file-format-raw-microarray = Сырые данные микрочипа
pv-genomic-file-format-fastq = FASTQ
pv-genomic-file-format-bam = BAM
pv-genomic-file-format-cram = CRAM
pv-genomic-file-format-vcf = VCF
pv-genomic-file-format-gvcf = gVCF
pv-genomic-file-format-other = Иной
pv-zygosity-heterozygous = Гетерозигота
pv-zygosity-homozygous = Гомозигота
pv-zygosity-hemizygous = Гемизигота
pv-zygosity-compound-heterozygous = Компаунд-гетерозигота
pv-clinical-significance-pathogenic = Патогенный
pv-clinical-significance-likely-pathogenic = Вероятно патогенный
pv-clinical-significance-uncertain-significance = Неопределённого значения
pv-clinical-significance-likely-benign = Вероятно доброкачественный
pv-clinical-significance-benign = Доброкачественный
pv-inheritance-pattern-autosomal-dominant = Аутосомно-доминантный
pv-inheritance-pattern-autosomal-recessive = Аутосомно-рецессивный
pv-inheritance-pattern-x-linked-dominant = Х-сцепленный доминантный
pv-inheritance-pattern-x-linked-recessive = Х-сцепленный рецессивный
pv-inheritance-pattern-y-linked = Y-сцепленный
pv-inheritance-pattern-mitochondrial = Митохондриальный
pv-inheritance-pattern-multifactorial = Мультифакториальный
pv-inheritance-pattern-unknown = Неизвестен
pv-carrier-status-affected = Болен
pv-carrier-status-carrier = Носитель
pv-carrier-status-not-carrier = Не носитель
pv-carrier-status-unknown = Неизвестно
pv-epigenetic-clock-horvath = Horvath
pv-epigenetic-clock-hannum = Hannum
pv-epigenetic-clock-phenoage = PhenoAge
pv-epigenetic-clock-grimage = GrimAge
pv-epigenetic-clock-dunedinpace = DunedinPACE
pv-epigenetic-clock-other = Иные
pv-metaboliser-status-poor = Медленный метаболизатор
pv-metaboliser-status-intermediate = Промежуточный метаболизатор
pv-metaboliser-status-normal = Нормальный метаболизатор
pv-metaboliser-status-rapid = Быстрый метаболизатор
pv-metaboliser-status-ultrarapid = Сверхбыстрый метаболизатор
pv-autopsy-not-performed = Не проводилось
pv-autopsy-clinical = Патологоанатомическое
pv-autopsy-forensic = Судебно-медицинское
pv-autopsy-external-examination = Только наружный осмотр
pv-autopsy-unknown = Неизвестно
pv-disposition-burial = Погребение в землю
pv-disposition-cremation = Кремация
pv-disposition-entombment = Погребение в склепе
pv-disposition-burial-at-sea = Погребение в море
pv-disposition-natural-burial = Естественное погребение
pv-disposition-body-donation = Тело передано науке
pv-disposition-other = Иное
pv-disposition-unknown = Неизвестно
pv-address-use-principal = Основное место жительства
pv-address-use-secondary = Дополнительное жильё
pv-address-use-temporary = Временное проживание
pv-address-use-postal = Почтовый адрес
pv-address-use-other = Иное
pv-nationality-mode-descent = По происхождению
pv-nationality-mode-birth-in-territory = По рождению на территории
pv-nationality-mode-naturalisation = В порядке натурализации
pv-nationality-mode-marriage = Через брак
pv-nationality-mode-registration = В порядке регистрации
pv-nationality-mode-restoration = Восстановление
pv-nationality-mode-state-succession = При смене государства
pv-nationality-mode-other = Иной способ
pv-language-proficiency-a1 = A1 Начальный
pv-language-proficiency-a2 = A2 Элементарный
pv-language-proficiency-b1 = B1 Средний
pv-language-proficiency-b2 = B2 Выше среднего
pv-language-proficiency-c1 = C1 Продвинутый
pv-language-proficiency-c2 = C2 Свободный
pv-language-proficiency-native = Родной
pv-isced-level-isced-0 = 0 Дошкольное
pv-isced-level-isced-1 = 1 Начальное
pv-isced-level-isced-2 = 2 Основное общее
pv-isced-level-isced-3 = 3 Среднее общее
pv-isced-level-isced-4 = 4 Послесреднее нетретичное
pv-isced-level-isced-5 = 5 Краткосрочное третичное
pv-isced-level-isced-6 = 6 Бакалавриат или эквивалент
pv-isced-level-isced-7 = 7 Магистратура или эквивалент
pv-isced-level-isced-8 = 8 Докторантура или эквивалент
pv-income-quintile-q1 = Нижняя пятая часть
pv-income-quintile-q2 = Вторая пятая часть
pv-income-quintile-q3 = Средняя пятая часть
pv-income-quintile-q4 = Четвёртая пятая часть
pv-income-quintile-q5 = Верхняя пятая часть
pv-pay-period-hourly = В час
pv-pay-period-daily = В день
pv-pay-period-weekly = В неделю
pv-pay-period-monthly = В месяц
pv-pay-period-annual = В год
pv-tenure-owned = Собственность
pv-tenure-co-owned = Совместная собственность
pv-tenure-leasehold = Долгосрочная аренда
pv-tenure-rented = Наём
pv-tenure-usufruct = Узуфрукт
pv-tenure-other = Иное
pv-distinction-kind-order = Орден
pv-distinction-kind-decoration = Знак отличия
pv-distinction-kind-medal = Медаль
pv-distinction-kind-title = Почётное звание
pv-distinction-kind-other = Иная
pv-military-service-army = Сухопутные войска
pv-military-service-navy = Военно-морской флот
pv-military-service-air-force = Военно-воздушные силы
pv-military-service-marines = Морская пехота
pv-military-service-gendarmerie = Жандармерия
pv-military-service-border-guard = Пограничная служба
pv-military-service-national-guard = Национальная гвардия
pv-military-service-other = Иное
pv-rank-category-enlisted = Рядовой состав
pv-rank-category-non-commissioned = Сержантский состав
pv-rank-category-warrant = Прапорщики
pv-rank-category-officer-cadet = Курсанты
pv-rank-category-junior-officer = Младшие офицеры
pv-rank-category-senior-officer = Старшие офицеры
pv-rank-category-general-officer = Высшие офицеры
pv-iccs-section-acts-leading-to-death = 01 Деяния, повлёкшие смерть
pv-iccs-section-acts-causing-harm = 02 Деяния, причиняющие вред
pv-iccs-section-sexual-acts = 03 Деяния сексуального характера
pv-iccs-section-property-with-violence = 04 Против собственности с насилием
pv-iccs-section-property-only = 05 Против собственности
pv-iccs-section-controlled-substances = 06 Контролируемые вещества
pv-iccs-section-fraud-deception-corruption = 07 Мошенничество и коррупция
pv-iccs-section-public-order-and-state = 08 Против общественного порядка и государства
pv-iccs-section-public-safety-and-security = 09 Против общественной безопасности
pv-iccs-section-natural-environment = 10 Против окружающей среды
pv-iccs-section-other-criminal-acts = 11 Иные преступления
pv-case-outcome-convicted = Осуждён
pv-case-outcome-acquitted = Оправдан
pv-case-outcome-dismissed = Дело прекращено
pv-case-outcome-conviction-quashed = Приговор отменён
pv-case-outcome-pardoned = Помилован
pv-case-outcome-amnestied = Амнистирован
pv-case-outcome-expunged = Судимость снята
pv-case-outcome-pending = Не завершено
pv-case-outcome-unknown = Неизвестно
pv-religion-buddhism = Буддизм
pv-religion-christianity-catholic = Христианство: католицизм
pv-religion-christianity-orthodox = Христианство: православие
pv-religion-christianity-protestant = Христианство: протестантизм
pv-religion-christianity-other = Христианство: другое
pv-religion-hinduism = Индуизм
pv-religion-islam-sunni = Ислам: суннизм
pv-religion-islam-shia = Ислам: шиизм
pv-religion-islam-other = Ислам: другое
pv-religion-jainism = Джайнизм
pv-religion-judaism = Иудаизм
pv-religion-sikhism = Сикхизм
pv-religion-bahai = Бахаизм
pv-religion-shinto = Синтоизм
pv-religion-taoism = Даосизм
pv-religion-zoroastrianism = Зороастризм
pv-religion-traditional = Народная или традиционная религия
pv-religion-other = Иная
pv-religion-none = Без религии
pv-religion-unknown = Неизвестно
pv-sacrament-baptism = Крещение
pv-sacrament-confirmation = Конфирмация или миропомазание
pv-sacrament-first-communion = Первое причастие
pv-sacrament-reconciliation = Исповедь
pv-sacrament-anointing-of-the-sick = Соборование
pv-sacrament-holy-orders = Рукоположение
pv-sacrament-matrimony = Венчание
pv-sacrament-other-rite = Иной обряд
pv-political-position-far-left = Крайне левые
pv-political-position-left = Левые
pv-political-position-centre-left = Левоцентристы
pv-political-position-centre = Центристы
pv-political-position-centre-right = Правоцентристы
pv-political-position-right = Правые
pv-political-position-far-right = Крайне правые
pv-political-position-apolitical = Аполитичен
pv-political-position-other = Вне этой оси
pv-political-position-unknown = Неизвестно
pv-membership-kind-trade-union = Профсоюз
pv-membership-kind-political-party = Политическая партия
pv-membership-kind-professional-body = Профессиональное объединение
pv-membership-kind-religious-order = Монашеский орден
pv-membership-kind-religious-association = Религиозное общество
pv-membership-kind-fraternal-order = Братство
pv-membership-kind-veterans-association = Ветеранская организация
pv-membership-kind-sports-club = Спортивный клуб
pv-membership-kind-cultural-association = Культурное общество
pv-membership-kind-charitable-association = Благотворительное общество
pv-membership-kind-other = Иная
pv-personality-instrument-neo-pi-3 = NEO-PI-3
pv-personality-instrument-neo-ffi-3 = NEO-FFI-3
pv-personality-instrument-bfi-2 = BFI-2
pv-personality-instrument-ipip-neo-120 = IPIP-NEO-120
pv-personality-instrument-tipi = TIPI
pv-personality-instrument-hexaco-pi-r = HEXACO-PI-R
pv-personality-instrument-observer-rating = Оценка знавшего человека
pv-personality-instrument-inferred = Выведено из документов
pv-personality-instrument-other = Иная
pv-introversion-extraversion-strongly-introverted = Выраженный интроверт
pv-introversion-extraversion-introverted = Интроверт
pv-introversion-extraversion-ambiverted = Амбиверт
pv-introversion-extraversion-extraverted = Экстраверт
pv-introversion-extraversion-strongly-extraverted = Выраженный экстраверт
pv-stress-tolerance-very-low = Очень низкая
pv-stress-tolerance-low = Низкая
pv-stress-tolerance-moderate = Умеренная
pv-stress-tolerance-high = Высокая
pv-stress-tolerance-very-high = Очень высокая
pv-decision-style-rational = Рациональный
pv-decision-style-intuitive = Интуитивный
pv-decision-style-dependent = Зависимый
pv-decision-style-avoidant = Избегающий
pv-decision-style-spontaneous = Спонтанный
pv-sport-level-recreational = Любительский
pv-sport-level-amateur-competitive = Любительский соревновательный
pv-sport-level-semi-professional = Полупрофессиональный
pv-sport-level-professional = Профессиональный
pv-diet-omnivore = Всеядность
pv-diet-flexitarian = Флекситарианство
pv-diet-pescatarian = Пескетарианство
pv-diet-vegetarian = Вегетарианство
pv-diet-vegan = Веганство
pv-diet-other = Иное
pv-substance-tobacco = Табак и никотин
pv-substance-alcohol = Алкоголь
pv-substance-cannabis = Каннабис
pv-substance-opioids = Опиоиды
pv-substance-stimulants = Стимуляторы
pv-substance-sedatives-hypnotics = Седативные и снотворные
pv-substance-hallucinogens = Галлюциногены
pv-substance-inhalants = Ингалянты
pv-substance-gambling = Азартные игры
pv-substance-gaming = Видеоигры
pv-substance-other = Иное
pv-use-pattern-occasional-use = Эпизодическое употребление
pv-use-pattern-regular-use = Регулярное употребление
pv-use-pattern-harmful-use = Пагубное употребление
pv-use-pattern-dependence = Зависимость
pv-use-pattern-in-remission = Ремиссия
pv-lineage-biological = Биологическое
pv-lineage-adoptive = Приёмное
pv-lineage-foster = Опека в семье
pv-lineage-step = Неродное
pv-lineage-guardianship = Опекунство
pv-lineage-unknown = Неизвестно
pv-link-relation-godparent = Крёстный
pv-link-relation-godchild = Крестник
pv-link-relation-witness = Свидетель
pv-link-relation-officiant = Совершавший обряд
pv-link-relation-business-partner = Деловой партнёр
pv-link-relation-employer = Работодатель
pv-link-relation-employee = Работник
pv-link-relation-mentor = Наставник
pv-link-relation-apprentice = Ученик
pv-link-relation-close-friend = Близкий друг
pv-link-relation-neighbour = Сосед
pv-link-relation-guardian = Опекун
pv-link-relation-ward = Подопечный
pv-link-relation-other = Иная
pv-country-AD = Андорра
pv-country-AE = ОАЭ
pv-country-AF = Афганистан
pv-country-AG = Антигуа и Барбуда
pv-country-AI = Ангилья
pv-country-AL = Албания
pv-country-AM = Армения
pv-country-AO = Ангола
pv-country-AQ = Антарктида
pv-country-AR = Аргентина
pv-country-AS = Американское Самоа
pv-country-AT = Австрия
pv-country-AU = Австралия
pv-country-AW = Аруба
pv-country-AX = Аландские о-ва
pv-country-AZ = Азербайджан
pv-country-BA = Босния и Герцеговина
pv-country-BB = Барбадос
pv-country-BD = Бангладеш
pv-country-BE = Бельгия
pv-country-BF = Буркина-Фасо
pv-country-BG = Болгария
pv-country-BH = Бахрейн
pv-country-BI = Бурунди
pv-country-BJ = Бенин
pv-country-BL = Сен-Бартелеми
pv-country-BM = Бермудские о-ва
pv-country-BN = Бруней-Даруссалам
pv-country-BO = Боливия
pv-country-BQ = Бонэйр, Синт-Эстатиус и Саба
pv-country-BR = Бразилия
pv-country-BS = Багамы
pv-country-BT = Бутан
pv-country-BV = о-в Буве
pv-country-BW = Ботсвана
pv-country-BY = Беларусь
pv-country-BZ = Белиз
pv-country-CA = Канада
pv-country-CC = Кокосовые о-ва
pv-country-CD = Конго - Киншаса
pv-country-CF = Центрально-Африканская Республика
pv-country-CG = Конго - Браззавиль
pv-country-CH = Швейцария
pv-country-CI = Кот-д’Ивуар
pv-country-CK = Острова Кука
pv-country-CL = Чили
pv-country-CM = Камерун
pv-country-CN = Китай
pv-country-CO = Колумбия
pv-country-CR = Коста-Рика
pv-country-CU = Куба
pv-country-CV = Кабо-Верде
pv-country-CW = Кюрасао
pv-country-CX = о-в Рождества
pv-country-CY = Кипр
pv-country-CZ = Чехия
pv-country-DE = Германия
pv-country-DJ = Джибути
pv-country-DK = Дания
pv-country-DM = Доминика
pv-country-DO = Доминиканская Республика
pv-country-DZ = Алжир
pv-country-EC = Эквадор
pv-country-EE = Эстония
pv-country-EG = Египет
pv-country-EH = Западная Сахара
pv-country-ER = Эритрея
pv-country-ES = Испания
pv-country-ET = Эфиопия
pv-country-FI = Финляндия
pv-country-FJ = Фиджи
pv-country-FK = Фолклендские о-ва
pv-country-FM = Федеративные Штаты Микронезии
pv-country-FO = Фарерские о-ва
pv-country-FR = Франция
pv-country-GA = Габон
pv-country-GB = Великобритания
pv-country-GD = Гренада
pv-country-GE = Грузия
pv-country-GF = Французская Гвиана
pv-country-GG = Гернси
pv-country-GH = Гана
pv-country-GI = Гибралтар
pv-country-GL = Гренландия
pv-country-GM = Гамбия
pv-country-GN = Гвинея
pv-country-GP = Гваделупа
pv-country-GQ = Экваториальная Гвинея
pv-country-GR = Греция
pv-country-GS = Южная Георгия и Южные Сандвичевы о-ва
pv-country-GT = Гватемала
pv-country-GU = Гуам
pv-country-GW = Гвинея-Бисау
pv-country-GY = Гайана
pv-country-HK = Гонконг (САР)
pv-country-HM = о-ва Херд и Макдональд
pv-country-HN = Гондурас
pv-country-HR = Хорватия
pv-country-HT = Гаити
pv-country-HU = Венгрия
pv-country-ID = Индонезия
pv-country-IE = Ирландия
pv-country-IL = Израиль
pv-country-IM = о-в Мэн
pv-country-IN = Индия
pv-country-IO = Британская территория в Индийском океане
pv-country-IQ = Ирак
pv-country-IR = Иран
pv-country-IS = Исландия
pv-country-IT = Италия
pv-country-JE = Джерси
pv-country-JM = Ямайка
pv-country-JO = Иордания
pv-country-JP = Япония
pv-country-KE = Кения
pv-country-KG = Киргизия
pv-country-KH = Камбоджа
pv-country-KI = Кирибати
pv-country-KM = Коморы
pv-country-KN = Сент-Китс и Невис
pv-country-KP = КНДР
pv-country-KR = Республика Корея
pv-country-KW = Кувейт
pv-country-KY = Острова Кайман
pv-country-KZ = Казахстан
pv-country-LA = Лаос
pv-country-LB = Ливан
pv-country-LC = Сент-Люсия
pv-country-LI = Лихтенштейн
pv-country-LK = Шри-Ланка
pv-country-LR = Либерия
pv-country-LS = Лесото
pv-country-LT = Литва
pv-country-LU = Люксембург
pv-country-LV = Латвия
pv-country-LY = Ливия
pv-country-MA = Марокко
pv-country-MC = Монако
pv-country-MD = Молдова
pv-country-ME = Черногория
pv-country-MF = Сен-Мартен
pv-country-MG = Мадагаскар
pv-country-MH = Маршалловы Острова
pv-country-MK = Северная Македония
pv-country-ML = Мали
pv-country-MM = Мьянма (Бирма)
pv-country-MN = Монголия
pv-country-MO = Макао (САР)
pv-country-MP = Северные Марианские о-ва
pv-country-MQ = Мартиника
pv-country-MR = Мавритания
pv-country-MS = Монтсеррат
pv-country-MT = Мальта
pv-country-MU = Маврикий
pv-country-MV = Мальдивы
pv-country-MW = Малави
pv-country-MX = Мексика
pv-country-MY = Малайзия
pv-country-MZ = Мозамбик
pv-country-NA = Намибия
pv-country-NC = Новая Каледония
pv-country-NE = Нигер
pv-country-NF = о-в Норфолк
pv-country-NG = Нигерия
pv-country-NI = Никарагуа
pv-country-NL = Нидерланды
pv-country-NO = Норвегия
pv-country-NP = Непал
pv-country-NR = Науру
pv-country-NU = Ниуэ
pv-country-NZ = Новая Зеландия
pv-country-OM = Оман
pv-country-PA = Панама
pv-country-PE = Перу
pv-country-PF = Французская Полинезия
pv-country-PG = Папуа — Новая Гвинея
pv-country-PH = Филиппины
pv-country-PK = Пакистан
pv-country-PL = Польша
pv-country-PM = Сен-Пьер и Микелон
pv-country-PN = о-ва Питкэрн
pv-country-PR = Пуэрто-Рико
pv-country-PS = Палестинские территории
pv-country-PT = Португалия
pv-country-PW = Палау
pv-country-PY = Парагвай
pv-country-QA = Катар
pv-country-RE = Реюньон
pv-country-RO = Румыния
pv-country-RS = Сербия
pv-country-RU = Россия
pv-country-RW = Руанда
pv-country-SA = Саудовская Аравия
pv-country-SB = Соломоновы Острова
pv-country-SC = Сейшельские Острова
pv-country-SD = Судан
pv-country-SE = Швеция
pv-country-SG = Сингапур
pv-country-SH = о-в Св. Елены
pv-country-SI = Словения
pv-country-SJ = Шпицберген и Ян-Майен
pv-country-SK = Словакия
pv-country-SL = Сьерра-Леоне
pv-country-SM = Сан-Марино
pv-country-SN = Сенегал
pv-country-SO = Сомали
pv-country-SR = Суринам
pv-country-SS = Южный Судан
pv-country-ST = Сан-Томе и Принсипи
pv-country-SV = Сальвадор
pv-country-SX = Синт-Мартен
pv-country-SY = Сирия
pv-country-SZ = Эсватини
pv-country-TC = о-ва Тёркс и Кайкос
pv-country-TD = Чад
pv-country-TF = Французские Южные территории
pv-country-TG = Того
pv-country-TH = Таиланд
pv-country-TJ = Таджикистан
pv-country-TK = Токелау
pv-country-TL = Восточный Тимор
pv-country-TM = Туркменистан
pv-country-TN = Тунис
pv-country-TO = Тонга
pv-country-TR = Турция
pv-country-TT = Тринидад и Тобаго
pv-country-TV = Тувалу
pv-country-TW = Тайвань
pv-country-TZ = Танзания
pv-country-UA = Украина
pv-country-UG = Уганда
pv-country-UM = Внешние малые о-ва (США)
pv-country-US = Соединенные Штаты
pv-country-UY = Уругвай
pv-country-UZ = Узбекистан
pv-country-VA = Ватикан
pv-country-VC = Сент-Винсент и Гренадины
pv-country-VE = Венесуэла
pv-country-VG = Виргинские о-ва (Великобритания)
pv-country-VI = Виргинские о-ва (США)
pv-country-VN = Вьетнам
pv-country-VU = Вануату
pv-country-WF = Уоллис и Футуна
pv-country-WS = Самоа
pv-country-YE = Йемен
pv-country-YT = Майотта
pv-country-ZA = Южно-Африканская Республика
pv-country-ZM = Замбия
pv-country-ZW = Зимбабве
pv-country-SU = Советский Союз
pv-country-DD = Германская Демократическая Республика
pv-country-YU = Югославия
pv-country-CS = Чехословакия
pv-country-OT = Османская империя
lang-aa = Афарский
lang-ab = Абхазский
lang-ae = Авестийский
lang-af = Африкаанс
lang-ak = Акан
lang-am = Амхарский
lang-an = Арагонский
lang-ar = Арабский
lang-as = Ассамский
lang-av = Аварский
lang-ay = Аймара
lang-az = Азербайджанский
lang-ba = Башкирский
lang-be = Белорусский
lang-bg = Болгарский
lang-bi = Бислама
lang-bm = Бамбара
lang-bn = Бенгальский
lang-bo = Тибетский
lang-br = Бретонский
lang-bs = Боснийский
lang-ca = Каталанский
lang-ce = Чеченский
lang-ch = Чаморро
lang-co = Корсиканский
lang-cr = Кри
lang-cs = Чешский
lang-cu = Церковнославянский
lang-cv = Чувашский
lang-cy = Валлийский
lang-da = Датский
lang-de = Немецкий
lang-dv = Мальдивский
lang-dz = Дзонг-кэ
lang-ee = Эве
lang-el = Греческий
lang-en = Английский
lang-eo = Эсперанто
lang-es = Испанский
lang-et = Эстонский
lang-eu = Баскский
lang-fa = Персидский
lang-ff = Фулах
lang-fi = Финский
lang-fj = Фиджи
lang-fo = Фарерский
lang-fr = Французский
lang-fy = Западнофризский
lang-ga = Ирландский
lang-gd = Гэльский
lang-gl = Галисийский
lang-gn = Гуарани
lang-gu = Гуджарати
lang-gv = Мэнский
lang-ha = Хауса
lang-he = Иврит
lang-hi = Хинди
lang-ho = Хиримоту
lang-hr = Хорватский
lang-ht = Гаитянский
lang-hu = Венгерский
lang-hy = Армянский
lang-hz = Гереро
lang-ia = Интерлингва
lang-id = Индонезийский
lang-ie = Интерлингве
lang-ig = Игбо
lang-ii = Носу
lang-ik = Инупиак
lang-io = Идо
lang-is = Исландский
lang-it = Итальянский
lang-iu = Инуктитут
lang-ja = Японский
lang-jv = Яванский
lang-ka = Грузинский
lang-kg = Конго
lang-ki = Кикуйю
lang-kj = Кунама
lang-kk = Казахский
lang-kl = Гренландский
lang-km = Кхмерский
lang-kn = Каннада
lang-ko = Корейский
lang-kr = Канури
lang-ks = Кашмири
lang-ku = Курдский
lang-kv = Коми
lang-kw = Корнский
lang-ky = Киргизский
lang-la = Латинский
lang-lb = Люксембургский
lang-lg = Ганда
lang-li = Лимбургский
lang-ln = Лингала
lang-lo = Лаосский
lang-lt = Литовский
lang-lu = Луба-катанга
lang-lv = Латышский
lang-mg = Малагасийский
lang-mh = Маршалльский
lang-mi = Маори
lang-mk = Македонский
lang-ml = Малаялам
lang-mn = Монгольский
lang-mr = Маратхи
lang-ms = Малайский
lang-mt = Мальтийский
lang-my = Бирманский
lang-na = Науру
lang-nb = Норвежский букмол
lang-nd = Северный ндебеле
lang-ne = Непальский
lang-ng = Ндонга
lang-nl = Нидерландский
lang-nn = Нюнорск
lang-no = Норвежский
lang-nr = Южный ндебеле
lang-nv = Навахо
lang-ny = Ньянджа
lang-oc = Окситанский
lang-oj = Оджибва
lang-om = Оромо
lang-or = Ория
lang-os = Осетинский
lang-pa = Панджаби
lang-pi = Пали
lang-pl = Польский
lang-ps = Пушту
lang-pt = Португальский
lang-qu = Кечуа
lang-rm = Романшский
lang-rn = Рунди
lang-ro = Румынский
lang-ru = Русский
lang-rw = Киньяруанда
lang-sa = Санскрит
lang-sc = Сардинский
lang-sd = Синдхи
lang-se = Северносаамский
lang-sg = Санго
lang-sh = Сербскохорватский
lang-si = Сингальский
lang-sk = Словацкий
lang-sl = Словенский
lang-sm = Самоанский
lang-sn = Шона
lang-so = Сомали
lang-sq = Албанский
lang-sr = Сербский
lang-ss = Свази
lang-st = Южный сото
lang-su = Сунданский
lang-sv = Шведский
lang-sw = Суахили
lang-ta = Тамильский
lang-te = Телугу
lang-tg = Таджикский
lang-th = Тайский
lang-ti = Тигринья
lang-tk = Туркменский
lang-tl = Тагалог
lang-tn = Тсвана
lang-to = Тонганский
lang-tr = Турецкий
lang-ts = Тсонга
lang-tt = Татарский
lang-tw = Тви
lang-ty = Таитянский
lang-ug = Уйгурский
lang-uk = Украинский
lang-ur = Урду
lang-uz = Узбекский
lang-ve = Венда
lang-vi = Вьетнамский
lang-vo = Волапюк
lang-wa = Валлонский
lang-wo = Волоф
lang-xh = Коса
lang-yi = Идиш
lang-yo = Йоруба
lang-za = Чжуань
lang-zh = Китайский
lang-zu = Зулу
person-tab-profile = Профиль
profile-groups-label = Разделы профиля
profile-group-withheld = Часть этого раздела от вас скрыта
profile-withheld = Записано об этом человеке, но скрыто от вас: { $classes }.
profile-empty = В этом разделе пока ничего не записано.
profile-earlier = прежняя форма
profile-earlier-title = Записано прежней версией этого приложения в поле, для которого в AXGF 1.1 нет места. Сохранено в том виде, в каком было внесено.
profile-other-names = { $n ->
        [one] и ещё { $n } имя
        [few] и ещё { $n } имени
        [many] и ещё { $n } имён
       *[other] и ещё { $n } имени
    }
profile-edit-group = Изменить раздел «{ $group }»
profile-summary-link = { $n ->
        [one] { $n } факт в профиле
        [few] { $n } факта в профиле
        [many] { $n } фактов в профиле
       *[other] { $n } факта в профиле
    }
profile-from = с
profile-until = по
profile-yes = Да
profile-no = Нет
profile-value = Значение
profile-editor-title = Профиль
profile-problems = Часть введённого не удалось сохранить. Каждая проблема указана рядом с её полем, и ничего не записано.
profile-editor-withheld = В этом разделе у этого человека есть также данные категории { $classes }, к которым у вас нет доступа. Они здесь не показаны, и сохранение формы оставит их без изменений.
profile-living-class-note = Этот человек записан как живой. То, что вы внесёте здесь в чувствительную категорию, увидят только администраторы.
profile-relationships-elsewhere = Родители, супруги, дети, крёстные и свидетели не хранятся в записи этого человека. Это семьи, связи и события, в которых он упомянут, — поэтому любое изменение здесь меняет и записи всех остальных участников.
profile-documents-first = Артефакт ссылается на документ, прикреплённый к этому человеку. Сначала прикрепите файл.
profile-editor-nothing = В этом разделе нет ничего, что вы можете изменить.
profile-new-entry = Новая запись
profile-provenance = Дата, источник и достоверность
profile-from-date = Верно с
profile-until-date = Верно по
profile-remove-entry = Удалить эту запись
profile-add-entry = Добавить ещё запись
profile-no-such-group-title = Нет такого раздела
profile-no-such-group-detail = В профиле нет раздела с таким названием.
profile-error-number = Значение в этом поле должно быть числом.
profile-error-integer = Значение в этом поле должно быть целым числом.
profile-error-range = Число выходит за допустимые для этого атрибута пределы.
profile-error-term = Значение не входит в список вариантов.
profile-error-required = В записи не хватает обязательного поля.
profile-error-one-of = В записи должно быть заполнено хотя бы одно из основных полей.
profile-error-confidence = Достоверность задаётся числом от 0 до 1, например 0,8.
profile-error-time = Время записывается в часах и минутах, например 05:40.
profile-error-currency = Валюта записывается трёхбуквенным кодом, например RUB.
profile-error-language = Язык записывается кодом, например ru или zh-Hans.
profile-error-coordinates = Для координат нужны широта от −90 до 90 и долгота от −180 до 180.
profile-error-rank-country = Звание относится к другой стране, чем выбранная.
record-unknown-place = [Неизвестное место]
record-missing-document = [Документ отсутствует]
