# axgf-cms — מחרוזות הממשק, עברית.
#
# איכות מכונה — לא נבדק עדיין על ידי דוברת או דובר עברית כשפת אם. אוצר המילים
# הגנאלוגי בפרט עשוי להיות שגוי כאן. תיקונים יתקבלו בברכה — ראו CONTRIBUTING.md.
#
# המונחים שנבחרו (פתוחים לדיון):
#   union → קשר זוגי · link → קשר · confidence → ודאות
#   reliability → אמינות · source → מקור · record → רשומה · archive → ארכיון
#   godparent → סנדק · witness → עד · speculative → השערה
#
# הפנייה לקורא היא בלשון רבים (התחברו, בחרו) כדי לא לקבוע מגדר.
# לעברית ארבע צורות ריבוי בכללי CLDR שבהם הקוד משתמש: one, two, many ו-other.
# כולן כתובות בכל בורר למטה.
#
# כלל: הקובץ הזה מתרגם רק את הממשק. שמות, מקומות, הערות ועיסוקים מגיעים
# מקובץ ה-.axgf ונשארים בשפתם ובכתבם המקוריים.

app-name = ax-genealogy

nav-tree = עץ
nav-convert = ייבוא
nav-admin = ניהול
nav-sign-in = התחברות
nav-sign-out = התנתקות
account-emergency = אסימון חירום
footer-credit = © 2026 · עוצב ונבנה על ידי { $author }
footer-format-link = פורמט AXGF ב-GitHub

settings-title = הגדרות
edit-mode-label = מצב עריכה — מה שתשמרו משנה את רשומות המשפחה
display-menu-label = שפה ותצוגה
display-settings = ערכת נושא ותצוגה…
settings-tabs-label = חלקי ההגדרות
settings-tab-theme = ערכת נושא
settings-tab-language = שפה
settings-tab-appearance = מראה
settings-done = סיום
prefs-language = שפה
prefs-theme = ערכת נושא
prefs-background = רקע
prefs-background-on = גוון צבע רך מאחורי הדף
prefs-apply = החלה
busy-saving = שומר…
busy-working = עובד…
prefs-reviewed = נבדק
prefs-machine = מכונה, { $coverage }%
prefs-machine-complete = מלא, עדיין לא נבדק
prefs-machine-title = תורגם ללא בדיקה של דובר שפת אם. אוצר המילים הגנאלוגי במיוחד עלול להיות שגוי — המילים לקשר זוגי, לסנדק או למקור ראשוני משתנות לפי מסורת הרישום של כל מדינה. תיקונים יתקבלו בברכה, והקובץ CONTRIBUTING.md מסביר מאיפה להתחיל.

theme-canopee = Canopée
theme-canopee-note = כהה, ירוק יער · ברירת המחדל
theme-fusion = Fusion
theme-fusion-note = כהה, ניאון · עץ חי
theme-galaxy = Galaxy
theme-galaxy-note = חלל עמוק · מערכת שמש
theme-summit = Summit
theme-summit-note = כהה, שחר זהוב · טיפוס אל הפסגה

tree-title-around = סביב { $name }
tree-title-whole = העץ כולו
tree-lede-focused = { $ancestors ->
        [one] אב קדמון אחד
        [two] שני אבות קדמונים
        [many] { $ancestors } אבות קדמונים
       *[other] { $ancestors } אבות קדמונים
    }, { $descendants ->
        [one] צאצא אחד
        [two] שני צאצאים
        [many] { $descendants } צאצאים
       *[other] { $descendants } צאצאים
    } ו{ $spouses ->
        [one] בן זוג אחד
        [two] שני בני זוג
        [many] { $spouses } בני זוג
       *[other] { $spouses } בני זוג
    }, { $depth } דורות לכל כיוון.
tree-filter-label = סינון הכרטיסים הנראים
tree-centre-on = מרכוז על
tree-depth = דורות לכל כיוון
tree-show = הצגה
tree-hidden-notice = { $n ->
        [one] אדם אחד מוצג בלי פרטיו
        [two] שני אנשים מוצגים בלי פרטיהם
        [many] { $n } אנשים מוצגים בלי פרטיהם
       *[other] { $n } אנשים מוצגים בלי פרטיהם
    }
tree-hidden-because-role = , כי רמת הנראות שלהם גבוהה ממה שהחשבון שלכם רשאי לקרוא.
tree-hidden-because-anonymous = , כי הם אינם ציבוריים.
tree-hidden-sign-in = התחברו אם יש לכם חשבון.
tree-restricted-card = הרשומה של אדם זה אינה גלויה לכם
tree-empty = עדיין אין את מי לצייר.
tree-unplaced = לא שייך לאף משפחה רשומה

record-identity = זהות
record-life-events = אירועי חיים
record-family = משפחה
record-other-relationships = קשרים אחרים
record-occupations = עיסוקים
record-places = מקומות
record-sources-documents = מקורות ומסמכים
record-notes = הערות
record-history = היסטוריה
record-raw = ישות גולמית
record-raw-summary-note = ה-JSON שממנו נבנה הדף הזה

record-identity-help = כל שם רשום עם סוגו, התקופה שבה שימש והמקור שמאחוריו, עם הכתב המקורי והתעתיק הלטיני שלו זה לצד זה כשהם שונים, וגם מגדר, מצב חיים ונראות.
record-life-events-help = לידה, פטירה וכל אירוע שאדם זה השתתף בו, לפי סדר התאריכים, כל אחד עם תפקידו — כך שחתונה שבה רק היה עד מופיעה לצד חתונתו שלו. עובדה ללא תאריך מופיעה אחרונה במקום להעמיד פנים שהיא ראשונה.
life-nothing-recorded = לא נרשם לאדם זה שום אירוע, עיסוק או מקום. בקובץ שהומר זה רגיל: GEDCOM נושא את מה שמישהו רשם, ורוב הרשומות הן שם ותאריך.
life-add-first = רישום האירוע הראשון
record-family-help = הורים ואחים, ואחר כך כל קשר זוגי עם סוגו, תאריכיו, מקומו, האופן שבו הסתיים וילדיו לפי סדר הלידה.
record-other-relationships-help = כל קשר שאדם זה נמצא באחד מקצותיו, כפי שהוא נקרא מצדו, כך שאותה רשומה מופיעה כ"סנדק של" מצד אחד וכ"בן הסנדקות של" מהצד השני.
record-occupations-help = עיסוקים כטווחי זמן על ציר משותף אחד, כדי שאפשר יהיה להשוות שני תפקידים במבט, עם פסים פתוחים כשאחד הגבולות חסר.
record-places-help = כל מקום שהרשומה הזאת נוגעת בו, עם מה שקרה בכל אחד וההיסטוריה של הגבולות שנותנת למקום משמעות לאורך הזמן.
record-sources-documents-help = כל מקור מציין את העובדות בדף הזה שנשענות עליו, לפי חוזק הראיה.
record-notes-help = הערות על הרשומה הזאת, כולל טקסט שממיר לא הצליח לפענח ושנשמר כלשונו במקום להימחק.
record-history-help = כל שינוי שנשמר ברשומה הזאת, החדש ביותר ראשון. מי תיקן מה הוא עובדה על האנשים שמתחזקים את העץ ולא על המשפחה שבו, ולכן הוא אינו נכלל בארכיון המיוצא ומוצג רק לבני משפחה מחוברים.
record-raw-help = שום דבר כאן אינו נוצר לתצוגה: זו הרשומה בדיוק כפי שהיא שמורה, עד לשמות השדות. אם אי פעם תצטרכו לקרוא את הארכיון בלי האתר הזה, זה מה שתראו.
record-help-toggle = מה החלק הזה מציג

record-gender = מגדר
record-living = בחיים
record-visibility = נראות
record-yes = כן
record-no = לא
record-name-type = סוג השם
record-name-used = בשימוש
record-name-evidence = ראיה
record-transliteration = תעתיק לטיני
record-born = נולד/ה
record-died = נפטר/ה
record-parents = הורים
record-siblings = אחים ואחיות
record-children = ילדים
record-unknown-person = [לא ידוע]
record-restricted-person = פרטי
record-restricted-title = הרשומה של אדם זה אינה גלויה לכם
record-absent-person-title = נזכר בעץ הזה אך אינו רשום בו
record-confidence = רמת ודאות
record-source = מקור
record-download = הורדה

access-restricted-title = לא גלוי לכם
access-restricted-signed-in = רמת הנראות של הרשומה הזאת גבוהה ממה שהחשבון שלכם רשאי לקרוא. מנהל יכול לשנות את נראות הרשומה או את התפקיד שלכם.
access-restricted-anonymous = הרשומה הזאת אינה ציבורית. התחברו כדי לבדוק אם החשבון שלכם רשאי לקרוא אותה.
access-role-title = לא עבור התפקיד שלכם
access-role-admin = זה דף של מנהל. החשבון שלכם יכול ליצור ולערוך רשומות, אבל לא לנהל חשבונות, למחוק רשומות או לייצא את הארכיון.
access-role-write = החשבון שלכם יכול לקרוא את העץ הזה אבל לא לשנות אותו. מנהל יכול להעלות את התפקיד שלכם לתורם.
access-scope-title = מחוץ לענף שלכם
access-scope-named = החשבון שלכם מוגבל לענף אחד של העץ, והרשומה הזאת נוגעת במישהו מחוצה לו. כל אדם שרשומה מזכירה חייב להיות בתוך הענף שלכם — אחרת משפחה עם בן זוג אחד מבחוץ הייתה דרך לשכתב את ההורות של אותו אדם.
access-scope-unnamed = החשבון שלכם מוגבל לענף אחד של העץ, והרשומה הזאת אינה מזכירה אף אחד שאפשר למדוד אותה מולו. מקורות ומקומות נערכים בידי חשבונות עם גישה לעץ כולו.

error-not-found-title = לא נמצא
error-not-found-detail = הדף הזה לא קיים כאן.
error-no-such-person-title = אין אדם כזה
error-no-such-person-detail = אין כאן אדם עם המזהה הזה.
error-merged-title = הרשומה הזאת כבר לא קיימת.
error-merged-detail = היא מוזגה אל { $name } ב-{ $date }. קישורים לכתובת הישנה מובילים כעת אל הרשומה הממוזגת.
error-merged-open = פתיחת הרשומה הממוזגת
error-back-to-river = חזרה אל הנהר
view-tree-back = חזרה אל העץ
view-galaxy-back = חזרה אל הגלקסיה
view-summit-back = חזרה אל ההר
error-no-such-entity-title = אין ישות כזאת
error-no-such-entity-detail = אין כאן רשומה עם המזהה הזה.
error-deleted-while-editing = אין כאן רשומה עם המזהה הזה. ייתכן שהיא נמחקה בזמן שערכתם אותה.
error-no-such-file-title = אין קובץ כזה
error-no-such-file-detail = אין כאן מסמך עם המזהה הזה, או שהמסמך רשום בלי קובץ — מסמך מוזכר מצביע על משהו שנשמר במקום אחר.
error-not-an-image-title = לא תמונה
error-not-an-image-detail = אין תמונה ממוזערת למסמך הזה, כי הוא אינו תמונה שהגרסה הזאת יודעת לפענח.
error-back = חזרה

login-title = התחברות
login-lede = חשבונות נוצרים בידי מנהל.
login-username = שם משתמש
login-password = סיסמה
login-submit = התחברות
login-wrong = שם המשתמש והסיסמה אינם תואמים.
login-token-wrong = האסימון הזה שגוי.
login-throttled = יותר מדי ניסיונות שנכשלו. המתינו כמה דקות ונסו שוב.
login-no-accounts-title = להתקנה הזאת עדיין אין חשבונות.
login-no-accounts-detail = בכוונה אין כאן דף הגדרה — חלון הזמן בין הפריסה להתחברות הראשונה הוא בדיוק הרגע שבו התקנה אינה מוגנת, ולכן המנהל הראשון נוצר משורת הפקודה.
login-no-accounts-note = היא מדפיסה סיסמה שנוצרה ל-stderr פעם אחת ולא יותר. עד אז הדרך היחידה להיכנס היא אסימון החירום שלמטה.
login-emergency-summary = גישת חירום
login-emergency-detail = האסימון המשותף עדיין פותח סשן של מנהל, והוא קיים למטרה אחת: לחזור פנימה כשקובץ ה-.acl אבד או שכל המנהלים ננעלו בחוץ. הוא אינו חשבון — אין לו העדפות, ויומן העריכה רושם אותו כ-emergency-token ולא כאדם. השימוש בו נרשם כאזהרה.
login-emergency-label = אסימון חירום
login-emergency-submit = שימוש באסימון החירום
login-sign-in-prompt = התחברו כדי להגיע ללוח הניהול.

admin-title = ניהול
setup-title = הקמת הארכיון הזה
setup-lede = ההתקנה הזאת עדיין אינה מכילה משפחה. בחרו איך היא תתחיל. שום דבר כאן לא נכתב לעולם מעל משפחה: הדף נסגר ברגע שיש אחת.
setup-door-archive-title = מארכיון AXGF
setup-door-archive-detail = קובץ ‎.axgf מהתקנה אחרת או מגיבוי. האנשים, המקורות והתמונות שבו מגיעים כפי שהם.
setup-door-archive-file = ארכיון (‎.axgf)
setup-door-archive-go = הקמה מהארכיון הזה
setup-limit-gb = עד { $n } GB, נכתב ישירות לדיסק.
setup-door-scratch-title = מאפס
setup-door-scratch-detail = ארכיון ריק. הוסיפו את האדם הראשון, ואחר כך את משפחתו, רשומה אחר רשומה.
setup-door-scratch-go = הוספת האדם הראשון
setup-door-gedcom-title = מקובץ GEDCOM
setup-door-gedcom-detail = הקובץ שכל תוכנת גנאלוגיה מייצאת: GEDCOM 5.5, 5.5.1, 5.5.5 או 7.0, כ-‎.ged או ‎.gdz, בכל קידוד שבו נכתב. תראו איך הוא נקרא ומה עובר לפני שמשהו נשמר.
setup-door-gedcom-file = קובץ GEDCOM (‎.ged, ‎.gdz)
setup-door-gedcom-go = קריאת הקובץ הזה
setup-never-over = כדי להחליף משפחה שכבר נמצאת כאן, שחזרו גיבוי משורת הפקודה (axgf-cms restore); אף דף לא כותב מעליה.
setup-adopt-title = הפיכת זו למשפחת הארכיון
setup-adopt-detail = רשומות שיהפכו למשפחה של ההתקנה הזאת: { $total }. שום דבר לא נשמר עד שתלחצו על הכפתור.
setup-adopt-go = הקמת הארכיון עם המשפחה הזאת
setup-adopt-back = בחירת דרך כניסה אחרת
setup-done-title = הארכיון הוקם
setup-done-detail = רשומות במשפחה של ההתקנה הזאת כעת: { $total }.
setup-done-photos = תמונות ומסמכים שהגיעו איתה: { $n }.
setup-done-mismatches = קבצים שהבתים שלהם לא תאמו את טביעת האצבע שנרשמה להם: { $n }. הם נשמרו; בדקו אותם.
setup-done-river = פתיחת הנהר
view-tree-open = פתיחת העץ
view-galaxy-open = פתיחת הגלקסיה
view-summit-open = פתיחת ההר
setup-done-admin = ניהול
setup-refused-title = זה לא יכול היה להפוך לארכיון
setup-refused-detail = שום דבר לא נכתב. מה שעצר את זה:
setup-closed-title = הארכיון הזה כבר הוקם
setup-closed-detail = הוא מכיל משפחה, ואף דף לא כותב מעליה. כדי להחליף אותה, שחזרו גיבוי משורת הפקודה (axgf-cms restore).
setup-not-archive = זה אינו ארכיון ‎.axgf.
setup-archive-too-large = הארכיון הזה גדול ממה שהעלאה יכולה להיות.
admin-lede = עריכת { $path } — { $total } ישויות, { $files ->
        [one] קובץ מצורף אחד
        [two] שני קבצים מצורפים
        [many] { $files } קבצים מצורפים
       *[other] { $files } קבצים מצורפים
    }, { $size } בדיסק. כל שינוי נכתב באופן אטומי; שינוי שנדחה משאיר את הקובץ ללא שינוי.
admin-entities = ישויות
admin-create = יצירה
admin-new-kind = { $kind } חדש
admin-operations = פעולות
admin-validate = אימות
admin-deduplicate = איחוד כפילויות
admin-export = ייצוא הארכיון
admin-accounts = חשבונות
admin-roles-note = אימות, איחוד כפילויות, ייצוא, מחיקה וניהול חשבונות שמורים למנהלים. תורם מגיע לכל דף אחר כאן.
admin-dedup-confirm = איחוד כפילויות ממזג רשומות וכותב מחדש את הארכיון. להמשיך?
admin-recent-changes = שינויים אחרונים
admin-recent-note = { $shown } האחרונים מתוך { $total ->
        [one] שינוי רשום אחד
        [two] שני שינויים רשומים
        [many] { $total } שינויים רשומים
       *[other] { $total } שינויים רשומים
    }, מתוך { $path }.
admin-sessions-open = { $n ->
        [one] סשן אחד פתוח כעת.
        [two] שני סשנים פתוחים כעת.
        [many] { $n } סשנים פתוחים כעת.
       *[other] { $n } סשנים פתוחים כעת.
    }
admin-no-changes-yet = עדיין לא שונה דבר דרך היישום הזה. כל שמירה מעכשיו נרשמת ב-{ $path }.
admin-last-validation = אימות אחרון
admin-bundle-heavy = הארכיון הזה הוא { $size }. כולו נטען בהפעלה ומוחזק בזיכרון, ולכן מעבר לכ-{ $warn } האתר מתחיל לעלות בזיכרון של ממש וההפעלות מחדש מתארכות. זה מתאים לארכיון משפחתי ולא לספריית מדיה — אם הקבצים המצורפים גדלים בלי גבול, שמרו אותם במאגר קבצים ותנו לארכיון להצביע עליהם.

admin-fields = שדות
admin-raw-json = JSON גולמי
form-more-fields = { $n ->
        [one] שדה נוסף אחד
        [two] שני שדות נוספים
        [many] { $n } שדות נוספים
       *[other] { $n } שדות נוספים
    }
admin-raw-json-help = הישות כולה, כך ששום דבר אינו בלתי ניתן לעריכה — רשימות כמו בני הזוג והילדים של משפחה, או היסטוריית הגבולות של מקום, נמצאות כאן. זה מסמך המוצא; השדות שלמעלה נכתבים אחר כך מעל הנתיבים שבאחריותם, לכן ערכו ערך במקום אחד או באחר, לא בשניהם. הוא חייב להיות JSON תקין, אחרת שום דבר לא נשמר.
admin-save = שמירה
admin-cancel = ביטול
place-editor-title = עריכת מקום
place-add-detail = הוספת פרטים למקום הזה
place-names = שמות
place-name-primary = ראשי
place-name-lang = שפה
place-name-value = שם
place-names-hint = שורה אחת לכל שם רשום. מקום שנוהל בידי שלוש אימפריות נושא שלושה שמות; הראשי הוא זה שמוצג בכל מקום אחר.
place-where = היכן הוא נמצא
place-type = סוג
place-region = אזור
place-country-current = המדינה כיום
place-country-hint = ISO 3166-1 alpha-2, לדוגמה PL, FR, DE.
place-country-history = היסטוריית גבולות
place-history-country = מדינה
place-history-from = מ-
place-history-until = עד
place-country-history-hint = איזו מדינה החזיקה במקום הזה ובאיזו תקופה. משמעותי גנאלוגית: רשומה שנכתבה ברוסית ב-1880 ואחרת שנכתבה בפולנית ב-1930 יכולות לנקוב באותו כפר.
place-coordinates = קואורדינטות
place-lat = קו רוחב
place-lon = קו אורך
place-precision = דיוק
place-identifiers = מזהים
place-wikidata = Wikidata
place-geonames = GeoNames
place-used-by = { $name } משמש { $n ->
        [one] רשומה אחת נוספת
        [two] שתי רשומות נוספות
        [many] { $n } רשומות נוספות
       *[other] { $n } רשומות נוספות
    }.
place-error-no-name = למקום נדרש שם אחד לפחות.
place-error-coords-pair = קו רוחב וקו אורך באים יחד: תנו את שניהם, או אף אחד מהם.
place-error-coords-number = קו רוחב וקו אורך חייבים להיות מספרים.
place-error-coords-range = קו רוחב נע בין ‎-90 ל-90 וקו אורך בין ‎-180 ל-180.
place-type-continent = יבשת
place-type-country = מדינה
place-type-region = אזור
place-type-department = מחוז
place-type-city = עיר

place-type-village = כפר
place-type-district = רובע
place-type-street = רחוב
place-type-building = בניין
place-type-farm = חווה
place-type-island = אי
place-type-historical = היסטורי
place-type-unknown = לא ידוע
place-precision-exact = מדויק
place-precision-building = בניין
place-precision-street = רחוב
place-precision-city_center = מרכז העיר
place-precision-region_center = מרכז האזור
place-precision-country_center = מרכז המדינה
place-precision-approximate = משוער

place-coordinates-hint = הקלדה ידנית היא הדרך הרגילה. מקומות רבים שנרשמו תחת מינהל קודם אינם נמצאים כלל בחיפוש מודרני.
place-geocode-search = חיפוש השם הזה
place-geocode-hint = שולח את השם, האזור והמדינה לשירות הגיאוקוד, מקום אחד בכל פעם. שום דבר לא נשמר עד שתשמרו.
place-geocode-off = חיפוש שמות כבוי. הוא צריך כתובת קשר שבה השירות יזהה את ההתקנה הזאת; הפעילו את השרת עם --geocoder-contact כדי להדליק אותו.
place-geocode-query = חיפוש עבור: { $q }
place-geocode-error = לא ניתן היה להגיע לשירות החיפוש. שדות הקואורדינטות שלמעלה עדיין עובדים.
place-geocode-none = לא נמצא דבר. עבור כפר שנרשם תחת מינהל רוסי, פרוסי או אוסטרי זו התוצאה הרגילה; הזינו את המיקום ידנית.
place-geocode-not-a-place = אינו יישוב
place-geocode-use = שימוש בזה
place-geocode-attribution = תוצאות מ-OpenStreetMap דרך Nominatim, ברישיון Open Database Licence.

place-paste = הדבקת מיקום
place-paste-placeholder = קישור למפה, או 52.0782795, 21.2508068
place-paste-read = קריאה
place-paste-hint = קישור של Google Maps או OpenStreetMap, כתובת geo:, זוג מספרים פשוט, או מעלות-דקות-שניות כמו 52°04'41.8"N 21°15'02.9"E.
place-paste-read-ok = נקרא אל השדות שלמעלה. בדקו, ואז שמרו.
place-paste-unreadable = זה אינו מיקום שאפשר לקרוא. השדות שלמעלה עדיין מקבלים זוג מספרים פשוט.

place-map-hint = לחצו על המפה כדי למקם את הנקודה, או גררו את הסיכה. השדות שלמעלה הם הרשומה.
place-map-clear = ניקוי הנקודה
place-open-in-map = מצאו את המקום הזה ב-OpenStreetMap, ואז הדביקו את הקישור כאן

person-tab-record = רשומה
person-tab-life = אירועים
person-tab-media = מדיה
person-tab-tree = עץ
person-tab-history = היסטוריה
person-tree-depth = { $n } דורות לכל כיוון. העץ כולו למטה.
person-tree-alone = הרשומה הזאת אינה מזכירה הורים, בני זוג או ילדים, ולכן אין צורה לצייר סביבה.

record-no-evidence = שום דבר לא מצורף לרשומה הזאת — לא מקור ולא מסמך. זה המצב הרגיל של קובץ שהומר ולא פגם בו: GEDCOM נושא את העובדות ומשאיר מאחור את מה שהוכיח אותן.
record-no-evidence-signed-out = התחברו כדי לצרף אחד.
admin-delete = מחיקה
admin-not-set = — לא הוגדר —
admin-edit = עריכה
admin-page-of = עמוד { $page } מתוך { $pages }
list-select-row = בחירת { $name }
list-kind-label = סוג
list-select-page = בחירת כל השורות בעמוד הזה
list-selected = { $n ->
        [one] { $n } נבחר
        [two] { $n } נבחרו
        [many] { $n } נבחרו
       *[other] { $n } נבחרו
    }
list-export = ייצוא
list-escape = Esc לניקוי הבחירה
bulk-confirm-title = { $n ->
        [one] למחוק רשומה אחת?
        [two] למחוק שתי רשומות?
        [many] למחוק { $n } רשומות?
       *[other] למחוק { $n } רשומות?
    }
bulk-confirm-detail = רשומה שרשומה אחרת עדיין מפנה אליה נדחית, ואז שום דבר לא נמחק.
bulk-confirm-go = מחיקתן
bulk-deleted = { $n ->
        [one] רשומה אחת נמחקה.
        [two] שתי רשומות נמחקו.
        [many] { $n } רשומות נמחקו.
       *[other] { $n } רשומות נמחקו.
    }
bulk-refused = שום דבר לא נמחק: עדיין מפנים אל { $name } במקום אחר. הסירו קודם את ההפניה הזאת, ואז נסו שוב.
bulk-changed = { $n ->
        [one] רשומה אחת השתנתה מאז שהרשימה הוצגה, ולכן שום דבר לא נמחק. טענו את הרשימה מחדש ונסו שוב.
        [two] שתי רשומות השתנו מאז שהרשימה הוצגה, ולכן שום דבר לא נמחק. טענו את הרשימה מחדש ונסו שוב.
        [many] { $n } רשומות השתנו מאז שהרשימה הוצגה, ולכן שום דבר לא נמחק. טענו את הרשימה מחדש ונסו שוב.
       *[other] { $n } רשומות השתנו מאז שהרשימה הוצגה, ולכן שום דבר לא נמחק. טענו את הרשימה מחדש ונסו שוב.
    }
admin-range = { $from }–{ $to } מתוך { $total }
list-filiation = שושלת
admin-previous = הקודם
admin-next = הבא
admin-saved = נשמר כגרסה { $version } — { $summary }
admin-not-saved = לא נשמר
admin-created = נוצר
admin-not-created = לא נוצר
admin-deleted = נמחק
admin-not-deleted = לא נמחק — שום דבר לא השתנה
admin-what-changed = מה השתנה
admin-field = שדה
admin-from = מ-
admin-to = אל
admin-version = גרסה { $version }

accounts-title = חשבונות
accounts-lede = נשמרים ב-{ $path }, בהרשאות 600, ליד הארכיון ולעולם לא בתוכו. ארכיון מועתק, נשלח ומתפרסם; גיבובי סיסמאות שנוסעים בתוכו היו הופכים כל עותק של עץ המשפחה לעותק של פרטי ההתחברות.
accounts-existing = קיימים
accounts-username = שם משתמש
accounts-role = תפקיד
accounts-status = מצב
accounts-branch = ענף
accounts-last-seen = נראה לאחרונה
accounts-change = שינוי
accounts-you = (אתם)
accounts-active = פעיל
accounts-disabled = מושבת
accounts-never = אף פעם
accounts-whole-tree = העץ כולו
accounts-roots = { $n ->
        [one] שורש אחד
        [two] שני שורשים
        [many] { $n } שורשים
       *[other] { $n } שורשים
    }
accounts-add = הוספת חשבון
accounts-no-registration = מבקרים אינם יכולים לפתוח חשבונות בעצמם; ההרשמה מוגדרת למעלה. לרוב המשפחות מספיק מנהל שמכיר את כולם.
accounts-password-hint = השאירו ריק ותיווצר סיסמה שתוצג פעם אחת. לפחות { $min } תווים אם אתם קובעים אותה בעצמכם.
accounts-new-password-placeholder = סיסמה חדשה (ריק = ללא שינוי)
accounts-email = דוא״ל
accounts-optional = (רשות)
accounts-create = יצירת חשבון
accounts-role-viewer = צופה — קורא רשומות ציבוריות ורשומות של חברים
accounts-role-contributor = תורם — גם יוצר, עורך ומעלה קבצים
accounts-role-admin = מנהל — גם מנהל חשבונות, מוחק ומייצא
accounts-branch-hint = מגביל את מה שהחשבון הזה רשאי לערוך לאנשים האלה, לצאצאיהם ולבני זוגם.
accounts-branch-reading = הוא אינו מגביל את מה שהם רשאים לקרוא — זה נקבע לפי הנראות של כל רשומה, והשניים נשמרים נפרדים בכוונה.
accounts-branch-placeholder = מזהה אדם אחד בכל שורה
accounts-ids-in-bundle = מזהי האנשים בעץ הזה
accounts-emergency-warning = התחברתם עם אסימון החירום. הוא מעניק הרשאות מנהל לסשן הזה אך אינו חשבון: אין לו העדפות, ויומן העריכה ירשום את השינויים שלכם כ-emergency-token ולא כאדם. צרו לעצמכם חשבון אמיתי למטה והתחברו איתו.
accounts-created-with-password = { $username } נוצר. הסיסמה היא { $password } — היא מוצגת פעם אחת ונשמרת רק כגיבוב Argon2id, לכן העבירו אותה עכשיו.
accounts-created = { $username } נוצר.
accounts-updated = { $username } עודכן. כל סשן שהיה פתוח לו נותק.
accounts-username-taken = שם המשתמש הזה תפוס.
accounts-pick-role = בחרו תפקיד.
accounts-no-such = אין חשבון כזה.
accounts-last-admin = זה המנהל הפעיל היחיד. קדמו קודם מישהו אחר — התקנה בלי מנהל ניתנת לשחזור רק בעריכת קובץ ה-.acl או בשימוש באסימון החירום.
accounts-not-saved = לא נשמר: { $error }

conflict-title = מישהו אחר שינה את זה קודם
conflict-lede = { $who } שמר שינוי ל{ $kind } הזה ב-{ $when }, אחרי שפתחתם אותו. העריכה שלכם לא נשמרה, ושום דבר לא נדרס.
conflict-no-merge = שום דבר לא ממוזג כאן אוטומטית. מיזוג של עריכות של שני אנשים יוצר רשומה שאף אחד מהם לא בחר, ובגנאלוגיה שני עורכים שחלוקים על תאריך בדרך כלל קוראים מקורות שונים — וזו שאלה לאדם ולא לתוכנה. השוו בין השניים שלמטה והחליטו.
conflict-versions = התחלתם מגרסה { $expected }; הרשומה מחזיקה כעת בגרסה { $current }.
conflict-both-changed = שניכם שיניתם את אלה
conflict-both-changed-detail = השדות האלה נערכו בידי שניכם. מה שתשמרו יחליף את מה ש-{ $who } הכניס שם:
conflict-different-fields = שיניתם שדות שונים, כך ששום דבר מהעבודה של { $who } אינו שנוי במחלוקת — אבל החלה מחדש עדיין כותבת את כל הישות שלכם מעל שלו. בדקו את שתי העמודות לפני שתשמרו.
conflict-field-by-field = שדה אחר שדה
conflict-theirs = מה ש-{ $who } שינה אליו
conflict-yours = מה שאתם שיניתם אליו
conflict-unchanged-by-you = לא שונה על ידכם
conflict-unchanged-by-them = לא שונה על ידם
conflict-nothing-differs = אף גרסה אינה שונה מזו שממנה התחלתם באף שדה שהדף הזה מציג. מספר הגרסה זז, כך שמישהו שמר את הרשומה בלי לשנות שום דבר שהיא רושמת.
conflict-what-now = מה עכשיו
conflict-reapply = החלה מחדש של הגרסה שלכם מעל שלהם
conflict-reapply-hint = זו העריכה שלכם, מועברת אל מול גרסה { $version }. ערכו אותה כאן כדי לשמור כל חלק מהעבודה של { $who } שתרצו, ואז שמרו. הגרסה שלהם מוצגת למטה כדי להעתיק ממנה.
conflict-save-over = שמירה מעל שלהם
conflict-discard = ביטול שלי והתחלה מחדש
conflict-their-version = הגרסה של { $who }, כפי שהיא עכשיו
conflict-history-of = ההיסטוריה של ה{ $kind } הזה

convert-title = ייבוא קובץ משפחה
convert-submit = ייבוא
convert-result-title = דוח ייבוא
convert-download = הורדת הארכיון

date-unknown = תאריך לא ידוע
date-not-recorded = לא נרשם
date-circa = בערך { $date }
date-between = בין { $from } ל-{ $to }
date-before = לפני { $date }
date-after = אחרי { $date }
date-preserved = נרשם כ„{ $text }”
date-day-month-year = { $day } ב{ $month ->
        [1] ינואר
        [2] פברואר
        [3] מרץ
        [4] אפריל
        [5] מאי
        [6] יוני
        [7] יולי
        [8] אוגוסט
        [9] ספטמבר
        [10] אוקטובר
        [11] נובמבר
        [12] דצמבר
        *[other] { $month }
    } { $year }
date-month-year = { $month ->
        [1] ינואר
        [2] פברואר
        [3] מרץ
        [4] אפריל
        [5] מאי
        [6] יוני
        [7] יולי
        [8] אוגוסט
        [9] ספטמבר
        [10] אוקטובר
        [11] נובמבר
        [12] דצמבר
        *[other] { $month }
    } { $year }
date-decade = שנות ה-{ $decade }
date-century = המאה ה-{ $century }
date-quarter-century = הרבע { $quarter ->
        [1] הראשון
        [2] השני
        [3] השלישי
       *[other] הרביעי
    } של המאה ה-{ $century }

error-back-to-start = חזרה להתחלה
error-payload-missing-title = אין קובץ כזה
error-payload-missing-detail = התוכן של המסמך הזה אינו נמצא במטמון.
error-payload-unopenable-detail = לא ניתן היה לפתוח את התוכן של המסמך הזה.
error-no-such-document-detail = אין כאן מסמך עם המזהה הזה.
error-bad-preference-title = לא אחת מהאפשרויות
error-bad-preference-detail = זו אינה שפה או ערכת נושא שהאתר הזה מציע. שום דבר לא השתנה.
error-unknown-kind-title = סוג לא ידוע
error-unknown-kind-detail = „{ $kind }” אינו סוג של רשומה. הארכיון הזה מכיל: { $kinds }.
error-io-title = לא ניתן היה לשמור
error-io-detail = { $error }. שום דבר בדיסק לא השתנה.
error-upload-too-large = הקובץ הזה גדול ממגבלת ה-{ $mb } MB. שום דבר לא נשמר, והארכיון לא השתנה.
error-upload-refused = המסמך נדחה: { $reason }. הארכיון לא השתנה.
error-back-to-person = חזרה אל הרשומה
error-no-such-person-to-attach = אין כאן אדם עם המזהה הזה, ולכן אין למה לצרף מסמך.
error-upload-title = ההעלאה הזאת לא נשמרה
error-download-expired-title = ההורדה הזאת פגה
error-download-expired-detail = ייבוא נשמר חמש עשרה דקות ואז נמחק. ייבאו את הקובץ שוב.
error-upload-none = לא הועלה קובץ. בחרו קודם קובץ.
error-upload-unsupported = הקובץ הזה אינו מסוג שהארכיון הזה שומר. תמונות, PDF, טקסט פשוט, שמע ווידאו מתקבלים; הסוג נקרא מהבתים של הקובץ עצמו, כך ששינוי שם של קובץ הרצה לא יכניס אותו. SVG נדחה לחלוטין, כי SVG יכול לשאת סקריפט.
error-export-unreadable-title = לא ניתן היה לקרוא את הארכיון המיוצא
error-export-unreadable-detail = { $error }

tree-title-suffix = עץ
tree-back-to-focused = חזרה לתצוגה הממוקדת
tree-show-all = הצגת כל ה-{ $n }
tree-width-notice = התצוגה הזאת ברוחב { $width } פיקסלים — במסך של 1500 פיקסלים, { $screens ->
        [one] מסך אחד
        [two] שני מסכים
        [many] { $screens } מסכים
       *[other] { $screens } מסכים
    } של גלילה אופקית.
tree-confidence-label = רמת ודאות:
tree-band-certain = ודאי
tree-band-high = גבוהה
tree-band-medium = בינונית
tree-band-low = משוער
tree-counts = { $drawn } מתוך { $total } אנשים · { $generations ->
        [one] דור אחד
        [two] שני דורות
        [many] { $generations } דורות
       *[other] { $generations } דורות
    }
tree-unplaced-count = { $n } ללא מקום
tree-contradicts-title = העץ הזה סותר את עצמו.
tree-contradicts-detail = שום סידור של שורות אינו יכול לקיים זאת, ולכן הקשר שלמטה הושמט ממספור הדורות וייתכן שחלק מהשורות מצוירות במקום הלא נכון. תקנו את הרשומה השגויה מבין השתיים.
tree-contradicts-pair = נרשמו גם כזוג וגם כהורה וילד:
tree-contradicts-more = { $n ->
        [one] סתירה נוספת אחת אינה מופיעה ברשימה.
        [two] שתי סתירות נוספות אינן מופיעות ברשימה.
        [many] { $n } סתירות נוספות אינן מופיעות ברשימה.
       *[other] { $n } סתירות נוספות אינן מופיעות ברשימה.
    }
tree-no-people = עדיין אין אף אחד בעץ הזה.
tree-no-people-cta = ייבאו קובץ משפחה, או הוסיפו את האדם הראשון.
tree-nobody-selected = אין את מי לצייר עבור הבחירה הזאת.
tree-nobody-selected-cta = התחילו מהתצוגה ברירת המחדל.
tree-edge-union = קשר זוגי רשום
tree-edge-parentage = הורות רשומה

home-empty = עדיין לא נרשם דבר.
home-count = { $total ->
        [one] רשומה אחת
        [two] שתי רשומות
        [many] { $total } רשומות
       *[other] { $total } רשומות
    }, שמורות בקובץ אחד שהמשפחה מחזיקה בו.
home-browse = עיון בעץ
home-convert = ייבוא קובץ משפחה
home-unnamed-family = עץ המשפחה הזה
home-in-this-tree = מה שהמשפחה רשמה עד כה
home-showcase-title = היכן העץ הזה כבר אומר יותר משמות ותאריכים

home-showcase-example = לדוגמה ←
home-nothing-title = עדיין אין מה להציג.
home-nothing-admin = ההתקנה הזאת עדיין אינה מכילה משפחה: הקימו אותה מארכיון, מקובץ GEDCOM או מאפס.
home-nothing-visitor = ארכיון המשפחה הזה נמצא בהקמה.
home-eyebrow = ארכיון משפחתי
home-hero-1 = כל חיים הם מקור.
home-hero-2 = כל משפחה, נהר.
view-tree-hero-1 = כל חיים הם עלה.
view-tree-hero-2 = כל משפחה, עץ.
view-galaxy-hero-1 = כל חיים הם כוכב.
view-galaxy-hero-2 = כל משפחה, גלקסיה.
view-summit-hero-1 = כל חיים הם צעד.
view-summit-hero-2 = כל משפחה, טיפוס.
home-cta-river = לחקור את הנהר
view-tree-cta = לחקור את העץ
view-galaxy-cta = לחקור את הגלקסיה
view-summit-cta = לטפס על ההר
home-cta-tree = צפייה בעץ
home-scroll = גללו כדי לחקור
home-trust-title = רמת הוודאות בכל הורות
home-trust-certain = כמעט ודאי
home-trust-high = מבוסס היטב
home-trust-medium = סביר
home-trust-low = השערה
home-trust-unrated = טרם דורג
home-showcase-eyebrow = מעבר לשמות ולתאריכים
home-cta-title = יחד, נשמור את זיכרון המשפחה.
home-cta-detail = כל שם שנוסף וכל מקור שנבדק הופכים את הנהר לנאמן יותר עבור הבאים אחרינו.
view-tree-cta-detail = כל שם שנוסף וכל מקור שנבדק הופכים את העץ לנאמן יותר עבור הבאים אחרינו.
view-galaxy-cta-detail = כל שם שנוסף וכל מקור שנבדק הופכים את הגלקסיה לנאמנה יותר עבור הבאים אחרינו.
view-summit-cta-detail = כל שם שנוסף וכל מקור שנבדק הופכים את ההר לנאמן יותר עבור הבאים אחרינו.
home-cta-add = הוספת אדם

showcase-links-title = { $n ->
        [one] קשר אחד מעבר למשפחה
        [two] שני קשרים מעבר למשפחה
        [many] { $n } קשרים מעבר למשפחה
       *[other] { $n } קשרים מעבר למשפחה
    }
showcase-links-detail = סנדקים, מעסיקים, עדים וחונכים, כל אחד עם התאריכים שלו, המקור שלו ומידת הוודאות שלכם בו.
showcase-occupations-title = { $n ->
        [one] עבודה אחת עם התחלה וסוף
        [two] שתי עבודות עם התחלה וסוף
        [many] { $n } עבודות עם התחלה וסוף
       *[other] { $n } עבודות עם התחלה וסוף
    }
showcase-occupations-detail = „מורה, 1948–1978” שומר על משכו, ומצויר כפס לאורך השנים ולא כשורה בעלת תאריך אחד.
showcase-uncertain-title = { $n ->
        [one] תאריך אחד שנשאר לא ודאי כפי שנמסר
        [two] שני תאריכים שנשארו לא ודאיים כפי שנמסרו
        [many] { $n } תאריכים שנשארו לא ודאיים כפי שנמסרו
       *[other] { $n } תאריכים שנשארו לא ודאיים כפי שנמסרו
    }
showcase-uncertain-detail = בערך, לפני, אחרי ובין נשארים ארבע אמירות שונות. תאריך שהמקור לא הצליח לקבוע לעולם אינו מוצג כאילו נקבע.
showcase-preserved-title = { $n ->
        [one] תאריך אחד שנשמר במילים שבהן נכתב
        [two] שני תאריכים שנשמרו במילים שבהן נכתבו
        [many] { $n } תאריכים שנשמרו במילים שבהן נכתבו
       *[other] { $n } תאריכים שנשמרו במילים שבהן נכתבו
    }
showcase-preserved-detail = ניסוח שאיש לא הצליח לקרוא כתאריך נשמר בדיוק כפי שנכתב, במקום להיזרק בשקט.
showcase-sources-title = { $n ->
        [one] מקור אחד שמהימנותו רשומה
        [two] שני מקורות שמהימנותם רשומה
        [many] { $n } מקורות שמהימנותם רשומה
       *[other] { $n } מקורות שמהימנותם רשומה
    }
showcase-sources-detail = { $primary ->
        [one] מקור ראשוני אחד.
        [two] שני מקורות ראשוניים.
        [many] { $primary } ראשוניים.
       *[other] { $primary } ראשוניים.
    } כל עובדה מראה על איזו ראיה היא נשענת, ועד כמה הראיה הזאת חזקה.
showcase-places-title = { $n ->
        [one] מקום אחד שגבולותיו זזו
        [two] שני מקומות שגבולותיהם זזו
        [many] { $n } מקומות שגבולותיהם זזו
       *[other] { $n } מקומות שגבולותיהם זזו
    }
showcase-places-detail = עיירה יכולה להשתייך למדינות שונות בזמנים שונים, והרשומה אומרת איזו מהן חלה מתי.

record-also-recorded-as = נרשם גם כ-
record-borders-moved = גבולות זזו:
record-display-name = שם תצוגה
record-read-as = נקרא כ-
record-note = הערה
record-living-yes = בחיים
record-deceased = נפטר/ה
record-centre-tree-here = מרכוז העץ כאן
record-centre-tree-title = הזזת העץ כך שיתמרכז באדם הזה
record-see-in-tree = צפייה בעץ
record-see-in-tree-title = פתיחת רשת הכרטיסים סביב האדם הזה
record-open-full-page = פתיחת הדף המלא ↗
record-open-full-title = פתיחת הדף העצמאי שאפשר לשתף
record-edit = עריכה
record-see-in-river = צפייה בנהר
view-tree-see = צפייה בעץ
view-galaxy-see = צפייה בגלקסיה
view-summit-see = צפייה בהר
panel-empty = בחרו כרטיס כדי לראות כאן את הרשומה המלאה של אותו אדם.
person-see-in-tree = צפייה באדם הזה בעץ
person-visibility-inline = נראות:
person-age-at-death = נפטר/ה בגיל { $n }
person-age-now = בן/בת { $n }
person-born-in = נולד/ה ב{ $place }
person-died-in = נפטר/ה ב{ $place }
person-children-count = { $n ->
        [one] ילד אחד
        [two] שני ילדים
        [many] { $n } ילדים
       *[other] { $n } ילדים
    }
person-generations-below = { $n ->
        [one] דור אחד מתחת
        [two] שני דורות מתחת
        [many] { $n } דורות מתחת
       *[other] { $n } דורות מתחת
    }
person-portrait-of = תצלום של { $name }
person-no-portrait = לא נרשם תצלום

result-diagnostics = אבחונים
result-diagnostics-note = כל אבחון שהספרייה החזירה, כולל אזהרות שלא חסמו את הפעולה. אף אחד מהם אינו מסונן.
result-no-diagnostics = הספרייה לא החזירה אבחונים.
result-continue = המשך
result-dashboard = לוח בקרה
person-sections-label = חלקים בדף הזה

name-part-nasab = נסב (שושלת)
name-part-laqab = לקב (כינוי)
name-part-kunya = כוניה (שם על שם הילד)
name-part-nisbah = ניסבה (מוצא)
name-part-alias = כינוי
name-part-religious_name = שם דתי
name-part-pen_name = שם עט
name-type-pen_name = שם עט
gender-U = לא נרשם

record-notes-title = הערות על הרשומה הזאת:
record-name = שם
record-type = סוג
record-cause = סיבה:
record-as = כ-
record-partner-not-recorded = בן/בת הזוג לא נרשמו
record-union-from = מ-
record-union-at = ב-
record-union-until = עד
record-occupation-from = מ-
record-occupation-until = עד
record-source-reliability = מהימנות
record-source-supports = תומך ב
record-photographs = תצלומים
record-documents = מסמכים
record-file = קובץ
record-status = מצב
record-size = גודל
record-absent-document = מוזכר בידי אדם זה אך אינו שמור כאן.
record-no-file = אין קובץ
record-attach-document = צירוף מסמך
record-upload = העלאה
record-upload-help = עד { $mb } MB לקובץ. קבצים מצורפים נשמרים לצד העץ ונכתבים חזרה לארכיון בעת ייצוא, כך שתצלום נוסע עם המשפחה שאליה הוא שייך. סוג הקובץ נקרא מתוכנו ולא משמו: תמונות, PDF, טקסט פשוט, שמע ווידאו מתקבלים. SVG נדחה, כי SVG יכול לשאת סקריפט.
record-upload-help-short = עד { $mb } MB. ‏SVG נדחה.
record-verbatim-note = נשמר בדיוק כפי שהרשומה ציינה, כי אף ממיר לא הצליח לפרש אותו.
record-file-to-attach = קובץ לצירוף
record-document-type = סוג המסמך
record-caption = כיתוב
record-history-entry-meta = — { $at }
record-history-entry-version = , { $version }
kind-person = אדם
kind-family = משפחה
kind-event = אירוע
kind-link = קשר
kind-occupation = עיסוק
kind-source = מקור
kind-place = מקום
kind-document = מסמך

kind-person-plural = { $n ->
        [one] אדם
        [two] אנשים
        [many] אנשים
       *[other] אנשים
    }
kind-family-plural = { $n ->
        [one] משפחה
        [two] משפחות
        [many] משפחות
       *[other] משפחות
    }
kind-event-plural = { $n ->
        [one] אירוע
        [two] אירועים
        [many] אירועים
       *[other] אירועים
    }
kind-link-plural = { $n ->
        [one] קשר
        [two] קשרים
        [many] קשרים
       *[other] קשרים
    }
kind-occupation-plural = { $n ->
        [one] עיסוק
        [two] עיסוקים
        [many] עיסוקים
       *[other] עיסוקים
    }
kind-source-plural = { $n ->
        [one] מקור
        [two] מקורות
        [many] מקורות
       *[other] מקורות
    }
kind-place-plural = { $n ->
        [one] מקום
        [two] מקומות
        [many] מקומות
       *[other] מקומות
    }
kind-document-plural = { $n ->
        [one] מסמך
        [two] מסמכים
        [many] מסמכים
       *[other] מסמכים
    }

list-matching = { $total ->
        [one] התאמה אחת
        [two] שתי התאמות
        [many] { $total } התאמות
       *[other] { $total } התאמות
    }, { $per_page } בכל עמוד.
list-filter-placeholder = סינון לפי שם או מזהה
list-filter = סינון
list-clear = ניקוי
list-summary = תקציר
list-id = מזהה
list-actions = פעולות
list-nothing = אין כאן דבר.
list-nothing-matching = אין כאן דבר שמתאים ל„{ $q }”.
list-delete-confirm = למחוק את ה{ $kind } הזה? בחרו מה יקרה לישויות שמפנות אליו:
list-policy-reject = דחייה
list-policy-reject-detail = — לסרב אם משהו עדיין מפנה אליו. שום דבר לא אובד.
list-policy-cascade = מחיקה מדורגת
list-policy-cascade-detail = — למחוק אותו ולהסיר בפועל כל הפניה אליו.
list-policy-orphan = יתום
list-policy-orphan-detail = — למחוק אותו אך לשמור את הרשומות המפנות, כשהקישור מאופס.

completeness-dates-title = תאריכים, לפי הצורה שיש להם בפועל
completeness-no-dates = עדיין לא נרשמו תאריכים.
completeness-dates-note = תאריך שמישהו קבע ליום מסוים ותאריך שמישהו יכול היה למקם רק בעשור הם טענות שונות, ושניהם נשמרים כפי שנמסרו. טקסט שלא ניתן היה לקרוא כתאריך כלל נשמר מילה במילה במקום להימחק.
completeness-shape-exact = מדויק
completeness-shape-exact-note = יום מלא בלוח השנה
completeness-shape-approximate = משוער
completeness-shape-approximate-note = בערך, או שנה/עשור בלבד
completeness-shape-ranged = טווח
completeness-shape-ranged-note = לפני, אחרי או בין
completeness-shape-preserved = שמור
completeness-shape-preserved-note = טקסט שלא פוענח, נשמר כלשונו
completeness-shape-unknown = לא ידוע
completeness-shape-unknown-note = נרשם כלא ידוע

convert-page-title = ייבוא קובץ משפחה
convert-lede = הביאו עץ קיים מקובץ GEDCOM — הייצוא שרוב תוכנות הגנאלוגיה מפיקות. שום דבר לא נשמר כאן, והעץ שהאתר הזה כבר מציג נשאר בדיוק כפי שהיה.
convert-file-label = קובץ משפחה (‎.ged)
convert-file-hint = עד { $mb } MB. עץ של 767 אנשים הוא כ-320 KB.
convert-confidence-label = מידת הוודאות של העובדות האלה, להתחלה
convert-confidence-hint = הקובץ המיובא אינו אומר עד כמה מישהו היה בטוח, ולכן כל עובדה צריכה נקודת מוצא. קבעו ערך נמוך לעץ שנבנה במהירות, וגבוה יותר לעץ שנבנה ממסמכים. הקריאה הכנה של המספר הזה היא „יובא, ואיש לא בדק מאז” — אפשר להעלות או להוריד כל עובדה אחר כך, אחת אחת.
convert-lang-label = שפת שמות המקומות
convert-lang-hint = תגית כמו en,‏ fr או pl.

convert-failed = הייבוא לא הצליח
convert-try-another = נסו קובץ אחר
convert-converted = { $filename } יובא
convert-result-lede = { $total ->
        [one] רשומה אחת
        [two] שתי רשומות
        [many] { $total } רשומות
       *[other] { $total } רשומות
    }, { $size } KB. הכול נכנס ברמת ודאות של { $confidence }, ושמות המקומות נקראו כ-{ $lang }. העץ שהאתר הזה מציג לא נגע.
convert-produced = מה עבר
convert-skipped-title = { $n ->
        [one] רשומה אחת שלא ניתן היה לקרוא
        [two] שתי רשומות שלא ניתן היה לקרוא
        [many] { $n } רשומות שלא ניתן היה לקרוא
       *[other] { $n } רשומות שלא ניתן היה לקרוא
    }
convert-skipped-note = הרשומות האלה לא הכילו דבר שאפשר היה להעביר.
convert-other-diagnostics = { $n ->
        [one] עוד דבר אחד שכדאי לדעת
        [two] עוד שני דברים שכדאי לדעת
        [many] עוד { $n } דברים שכדאי לדעת
       *[other] עוד { $n } דברים שכדאי לדעת
    }
convert-clean = שום דבר לא נשאר מאחור — כל רשומה בקובץ עברה.
convert-download-title = הורדה
convert-download-named = הורדת { $name }
convert-download-note = נשמר כאן חמש עשרה דקות ואז נמחק, לכן הורידו אותו עכשיו.
convert-another = ייבוא קובץ אחר
admin-history-on = על
admin-history-meta = — { $kind }, { $at }
admin-validation-counts = { $errors ->
        [one] שגיאה אחת
        [two] שתי שגיאות
        [many] { $errors } שגיאות
       *[other] { $errors } שגיאות
    }, { $warnings ->
        [one] אזהרה אחת
        [two] שתי אזהרות
        [many] { $warnings } אזהרות
       *[other] { $warnings } אזהרות
    }, { $infos ->
        [one] הערה אחת
        [two] שתי הערות
        [many] { $infos } הערות
       *[other] { $infos } הערות
    }.
admin-warnings-never-block = אזהרות לעולם אינן חוסמות — הן מידע, לא מחסום.
admin-validator-clean = המאמת לא דיווח על דבר.
record-occupations-help-undated = עבודה נרשמת עם התחלה וסוף, כך שאפשר להשוות כמה מהן לאורך ציר זמן אחד. בארכיון הזה יש את שמות העבודות אבל לא תאריכים עבורן — רגיל אחרי ייבוא, כי לרוב קובצי המשפחה אין מקום לשמור אותם — ולכן עדיין אין קנה מידה לצייר.
record-occupations-help-axis = עיסוק הוא מצב בעל משך, לא אירוע בתאריך אחד. כל הטווחים חולקים ציר אחד, { $from }–{ $to }.
admin-value-not-set = לא הוגדר
admin-validation-report = דוח אימות
admin-dedup-complete = איחוד הכפילויות הושלם

admin-dedup-refused = איחוד הכפילויות נדחה
record-birth-order = סדר לידה
record-start-not-recorded = ההתחלה לא נרשמה
record-end-not-recorded = הסוף לא נרשם
record-document-no-file = המסמך רשום כאן, אך הקובץ עצמו אינו שמור
panel-selected-person = האדם שנבחר

tree-band-generation = דור { $g }
tree-band-people = { $n ->
        [one] אדם אחד
        [two] שני אנשים
        [many] { $n } אנשים
       *[other] { $n } אנשים
    }
tree-band-unplaced = ללא מקום
tree-band-unplaced-note = { $n ->
        [one] אדם אחד שאינו בשום משפחה — מוצג ולא מושמט
        [two] שני אנשים שאינם בשום משפחה — מוצגים ולא מושמטים
        [many] { $n } אנשים שאינם בשום משפחה — מוצגים ולא מושמטים
       *[other] { $n } אנשים שאינם בשום משפחה — מוצגים ולא מושמטים
    }

gender-M = זכר
gender-F = נקבה
gender-NB = א-בינארי
gender-unrecorded = לא נרשם

name-part-given_name = שם פרטי
name-part-family_name = שם משפחה
name-part-patronymic = שם האב
name-part-matronymic = שם האם
name-part-middle_name = שם אמצעי
name-part-nickname = כינוי
name-part-prefix = תחילית
name-part-suffix = סיומת
name-part-particle = מילית
name-part-part = חלק

name-type-primary = ראשי
name-type-other = אחר
name-type-alias = כינוי
name-type-birth = שם לידה
name-type-married = שם נישואין
name-type-religious = דתי
name-type-transliteration = תעתיק
name-type-nickname = כינוי

note-links = { $n ->
        [one] קשר מחוץ למשפחה, עם תאריכים ומקורות משלו
        [two] שני קשרים מחוץ למשפחה, עם תאריכים ומקורות משלהם
        [many] { $n } קשרים מחוץ למשפחה, עם תאריכים ומקורות משלהם
       *[other] { $n } קשרים מחוץ למשפחה, עם תאריכים ומקורות משלהם
    }
note-occupations = { $n ->
        [one] עבודה שנרשמה עם התחלה וסוף
        [two] שתי עבודות שנרשמו עם התחלה וסוף
        [many] { $n } עבודות שנרשמו עם התחלה וסוף
       *[other] { $n } עבודות שנרשמו עם התחלה וסוף
    }
note-birth-imprecise = תאריך לידה שהמקור לא הצליח לקבוע, מוצג כפי שנרשם
note-death-imprecise = תאריך פטירה שהמקור לא הצליח לקבוע, מוצג כפי שנרשם
note-names = { $n ->
        [one] שם רשום אחד
        [two] שני שמות רשומים
        [many] { $n } שמות רשומים
       *[other] { $n } שמות רשומים
    }
note-transliteration = שם בכתב המקורי שלו לצד התעתיק הלטיני
note-witnessed = { $n ->
        [one] אירוע שבו היו עדים ולא בעלי האירוע
        [two] שני אירועים שבהם היו עדים ולא בעלי האירוע
        [many] { $n } אירועים שבהם היו עדים ולא בעלי האירוע
       *[other] { $n } אירועים שבהם היו עדים ולא בעלי האירוע
    }

visibility-public = ציבורי
visibility-members = חברים
visibility-contributors = תורמים
visibility-private = פרטי

family-label-couple = { $children ->
        [0] { $a } ו{ $b }
        [one] { $a } ו{ $b } — ילד אחד
        [two] { $a } ו{ $b } — שני ילדים
        [many] { $a } ו{ $b } — { $children } ילדים
       *[other] { $a } ו{ $b } — { $children } ילדים
    }
family-label-half = { $children ->
        [0] { $a } ו{ $unknown }
        [one] { $a } ו{ $unknown } — ילד אחד
        [two] { $a } ו{ $unknown } — שני ילדים
        [many] { $a } ו{ $unknown } — { $children } ילדים
       *[other] { $a } ו{ $unknown } — { $children } ילדים
    }
family-label-children = { $others ->
        [0] { $first } — ההורים לא נרשמו
        [one] { $first } ואח אחד — ההורים לא נרשמו
        [two] { $first } ושני אחים — ההורים לא נרשמו
        [many] { $first } ו-{ $others } אחים — ההורים לא נרשמו
       *[other] { $first } ו-{ $others } אחים — ההורים לא נרשמו
    }
family-label-empty = משפחה שאיש בה לא נרשם

event-label = { $category } — { $who }, { $date }
event-label-nobody = { $category } — { $date }
event-two-people = { $a } ו{ $b }
event-more-people = { $a }, { $b } ו{ $others ->
        [one] עוד אחד
        [two] עוד שניים
        [many] עוד { $others }
       *[other] עוד { $others }
    }

link-label = { $label }: { $from } → { $to }
occupation-label = { $who } — { $title }
source-label = { $title } — { $reliability }
source-label-plain = { $title }
document-label = { $filename } — { $type }
document-label-untitled = { $type } ללא כותרת
list-unnamed = { $kind } ללא שם

event-category-birth = לידה
event-category-death = פטירה
event-category-marriage = נישואין
event-category-divorce = גירושין
event-category-baptism = טבילה
event-category-burial = קבורה
event-category-immigration = הגירה פנימה
event-category-emigration = הגירה החוצה
event-category-census = מפקד אוכלוסין
event-category-residence = מגורים
event-category-military = שירות צבאי
event-category-education = השכלה
event-category-other = אירוע

reliability-primary = מקור ראשוני
reliability-secondary = מקור משני
reliability-tertiary = מקור שלישוני
reliability-recollection = זיכרון
reliability-derivative = עבודה נגזרת
reliability-authored = חיבור
reliability-oral = מסורת בעל פה
reliability-unknown = מהימנות לא ידועה

document-type-photo = תצלום
document-type-certificate = תעודה
document-type-letter = מכתב
document-type-record = רשומה
document-type-newspaper = קטע עיתון
document-type-other = מסמך

completeness-title = היכן העץ הזה יכול לומר יותר
completeness-intro = מה שנרשם ומה שעדיין ריק.
completeness-import-title = מה שהייבוא הביא
completeness-import-intro = נספר מהקובץ שהעליתם זה עתה. שורה ריקה היא משהו שהקובץ המקורי לא רשם — לא משהו שהייבוא איבד.

completeness-headline-full = כל סוג פרטים שלמטה נרשם איפשהו בעץ הזה.
completeness-headline-empty = { $total ->
        [one] סוג הפרטים היחיד שלמטה עדיין לא נרשם בשום מקום.
        [two] שני סוגי הפרטים שלמטה עדיין לא נרשמו.
        [many] אף אחד מ-{ $total } סוגי הפרטים שלמטה עדיין לא נרשם.
       *[other] אף אחד מ-{ $total } סוגי הפרטים שלמטה עדיין לא נרשם.
    } כל אחד מהם הוא מקום שבו הרשומה יכולה לומר יותר.
completeness-headline-partial = { $carried ->
        [one] סוג פרטים אחד שלמטה נרשם
        [two] שני סוגי פרטים שלמטה נרשמו
        [many] { $carried } סוגי פרטים שלמטה נרשמו
       *[other] { $carried } סוגי פרטים שלמטה נרשמו
    }; { $empty ->
        [one] אחד עדיין ריק
        [two] שניים עדיין ריקים
        [many] { $empty } עדיין ריקים
       *[other] { $empty } עדיין ריקים
    }.

completeness-metric-confidence = עד כמה כל עובדה ודאית
completeness-metric-confidence-none = אף אחת מ-{ $slots } העובדות כאן אינה אומרת עד כמה היא ודאית. תאריך שמישהו קרא מתעודה ותאריך שמישהו ניחש נראים אותו דבר — עד שכבר לא.
completeness-metric-confidence-uniform = { $with } מתוך { $slots } עובדות נושאות ציון וכולן באותו מספר ({ $modal }). זה מה שייבוא גורף משאיר אחריו: ערך זמני שאיש לא חזר אליו. אף אחת עדיין לא נשפטה בנפרד.
completeness-metric-confidence-some = { $with } מתוך { $slots } עובדות נושאות ציון. { $modal_count } חולקות ערך אחד ({ $modal }); { $assessed } שונות ממנו ולכן נבחנו אחת אחת.
completeness-metric-confidence-many = { $with } מתוך { $slots } עובדות נושאות ציון, { $assessed } מהן שונות מהערך הנפוץ ביותר ({ $modal }) על פני { $distinct } רמות שונות. העץ הזה רושם אי-ודאות אמיתית ומשתנה.

completeness-metric-parentage = עד כמה כל קשר הורה–ילד ודאי
completeness-metric-parentage-none = אף הורות כאן אינה אומרת עד כמה היא ודאית. אימוצים, שושלות שנויות במחלוקת ושחזורים מאזכור יחיד הם בדיוק המקום שבו משפחה צריכה לרשום ספק — והעץ מצייר קשר פחות ודאי בקו דהוי יותר.
completeness-metric-parentage-some = { $n ->
        [one] הורות אחת נושאת ציון משלה
        [two] שתי הורויות נושאות ציון משלהן
        [many] { $n } הורויות נושאות ציון משלהן
       *[other] { $n } הורויות נושאות ציון משלהן
    }, כך שקו משוער נראה חלש יותר מקו מתועד.

completeness-metric-links = קשרים מעבר לדם ולנישואין
completeness-metric-links-none = סנדקים, מעסיקים, עדים, חונכים, אפוטרופסים. אף אחד עדיין לא נרשם. כל אחד יכול לשאת תאריכים משלו, את המקור שלו ואת מידת הוודאות שלכם.
completeness-metric-links-some = { $n ->
        [one] אחד רשום, עם תאריכים, מקור ומידת ודאות משלו.
        [two] שניים רשומים, כל אחד עם תאריכים, מקור ומידת ודאות משלו.
        [many] { $n } רשומים, כל אחד עם תאריכים, מקור ומידת ודאות משלו.
       *[other] { $n } רשומים, כל אחד עם תאריכים, מקור ומידת ודאות משלו.
    }

completeness-metric-occupations = עבודה שנרשמה עם התחלה וסוף
completeness-metric-occupations-none = לא נרשמו עיסוקים. מקצוע שהוחזק שלושים שנה אומר על חיים יותר מרשומה בודדת עם תאריך.
completeness-metric-occupations-undated = { $total ->
        [one] עיסוק אחד רשום, ללא תאריכים
        [two] שני עיסוקים רשומים, ללא תאריכים
        [many] { $total } עיסוקים רשומים, ללא תאריכים
       *[other] { $total } עיסוקים רשומים, ללא תאריכים
    }. הוסיפו התחלה וסוף ואפשר יהיה להשוות ביניהם על ציר זמן אחד.
completeness-metric-occupations-some = ל-{ $span } מתוך { $total } יש התחלה או סוף, כך שאפשר להשוות ביניהם על ציר זמן אחד.

completeness-metric-sources = מקורות שדורגו לפי מהימנותם
completeness-metric-sources-none = לא נרשמו מקורות. ציון מאיפה הגיעה עובדה הוא מה שמאפשר לקרוב משפחה לבדוק אותה אחר כך — או לחלוק עליה ולומר מדוע.
completeness-metric-sources-some = { $graded } מתוך { $total } אומרים עד כמה הם חזקים, כך שטענה הנשענת על תעודת לידה נראית שונה מטענה הנשענת על זיכרון.

completeness-what-is-recorded = מה שהרשומה יכולה לומר
completeness-in-this-tree = בעץ הזה
completeness-not-yet = עדיין לא נרשם

role-spouse = בן/בת זוג
role-spouse_1 = בן/בת זוג ראשון
role-spouse_2 = בן/בת זוג שני
role-subject = נושא
role-participant = משתתף
role-witness = עד
role-officiant = עורך הטקס
role-informant = מוסר המידע
role-godparent = סנדק/סנדקית

phys-no-source = אין מקור
phys-col-date = מתי
phys-col-source = מקור
phys-col-confidence = רמת ודאות
phys-col-note = הערה
phys-field-height-cm = גובה
phys-field-weight-kg = משקל
phys-field-eye-colour = צבע עיניים
phys-field-hair-colour = צבע שיער
phys-field-build = מבנה גוף
phys-field-handedness = יד דומיננטית
phys-field-features = סימנים מזהים
phys-field-military = שירות צבאי
phys-field-languages = שפות מדוברות
phys-field-blood-group = סוג דם
phys-field-conditions = מצבים רפואיים ידועים
phys-field-operations = ניתוחים ופציעות
phys-field-cause-of-death = סיבת המוות
phys-field-religion = דת או השתייכויות
phys-field-health-notes = הערות
admin-export-health-note = הייצוא הפשוט משמיט כל סוג מידע רגיש — בריאות ואמונה, נתונים ביומטריים, נתונים גנומיים ורישום פלילי — ואת הפרופיל ההתנהגותי של כל אדם חי, כך שקובץ שנשלח לקרוב משפחה אינו נושא אף אחד מהם. סמנו מה קובץ מסוים צריך לשאת; הארכיון עצמו רושם אילו סוגים הושמטו.
avatar-picker-title = בחירת תמונה
avatar-choose-link = בחירת תמונה
avatar-choose = איזו תמונה מייצגת את האדם הזה
avatar-mode-auto = לתת לתוכנה לבחור
avatar-mode-auto-note = הדיוקן הראשון, ואם אין — התמונה הראשונה המקושרת לרשומה הזאת.
avatar-mode-none = הצגת ראשי תיבות במקום
avatar-mode-none-note = לרשומה שהתמונות היחידות שלה הן מסמכים ולא פנים.
avatar-focal-hint = לחצו על תמונה כדי לבחור בה, ולחצו שוב על החלק שצריך להישאר במסגרת — אוואטר הוא ריבועי ורוב הסריקות אינן.
avatar-no-images = עדיין אין תמונות מקושרות לרשומה הזאת.
avatar-upload-title = העלאת תמונה ושימוש בה
avatar-upload-button = העלאה ושימוש כתמונה
avatar-not-available-title = התמונה הזאת אינה זמינה
avatar-not-available-detail = הקובץ שנבחר אינו מקושר לאדם הזה, או שאינו מותר לכם לקריאה.
ai-badge = AI
ai-portraits-title = דיוקנאות שעובדו בבינה מלאכותית
ai-portraits-band = משופר
ai-portraits-upload = העלאת תמונות שעובדו בבינה מלאכותית
ai-portraits-manage = ניהול תמונות הבינה המלאכותית
ai-portraits-intro = תמונות שבינה מלאכותית יצרה מתצלומי האדם הזה: איך אולי נראה. הן מופיעות ליד הפנים ברשומה, מסומנות „AI”, ורק חברים מחוברים רואים אותן. הן לעולם אינן התמונה של האדם, לעולם אינן בין הקבצים שלו, לעולם אינן ראיה.
ai-portraits-current = ברשומה
ai-portraits-add = הוספת תמונות — יש מקום ל-{ $n }
ai-portraits-files = תמונות (PNG, JPEG, WebP או GIF), כמה בבת אחת
ai-portraits-generator = נוצר באמצעות
ai-portraits-from = נוצר מתוך
ai-portraits-from-none = לא צוין
ai-portraits-remove = הסרה
ai-portraits-alt = דיוקן של { $name }, שעובד בבינה מלאכותית
ai-portraits-not-evidence = נוצר בידי בינה מלאכותית מתוך התצלומים: אינן תמונות מאותה תקופה.
ai-portraits-full = לרשומה יש את שלוש תמונות הבינה המלאכותית שלה. הסירו אחת כדי להוסיף אחרת.
ai-portraits-too-many = רשומה מכילה לכל היותר שלוש תמונות בינה מלאכותית. הסירו אחת לפני שתוסיפו אחרת.
ai-portraits-images-only = דיוקן בינה מלאכותית חייב להיות תמונה: PNG, JPEG, WebP או GIF.
ai-portraits-stale = הרשומה השתנתה בזמן שהדף הזה היה פתוח. שום דבר לא נשמר: פתחו אותה שוב וחזרו על הפעולה.
ai-portraits-back = חזרה לתמונות הבינה המלאכותית

record-history-withheld = מוסתר מכם

record-life-status = מצב
record-presumed-deceased = נפטר/ה ככל הנראה
record-presumed-short = משוער
record-presumed-why = לא נרשמה פטירה והלידה הייתה לפני יותר מ-{ $years } שנים, ולכן הרשומה הזאת אינה יכולה להיות נכונה. החבילה לא השתנתה: זה מה שהדף מסיק, לא מה שהמקור אומר.

identity-editor-title = שמות וזהות
identity-primary-name = השם שמוצג בכל מקום
identity-primary-help = מה שכרטיס העץ, הכותרת וכל רשימה משתמשים בו. השמות האחרים שלמטה הם מה שמקור כלשהו כינה את האדם הזה בזמן אחר.
identity-display = שם
identity-display-latin = בכתב לטיני
identity-culture = שפה
identity-direction = כיוון
identity-direction-ltr = משמאל לימין
identity-direction-rtl = מימין לשמאל
identity-direction-auto = לפי הטקסט
identity-components = חלקי השם
identity-components-help = איזה חלק הוא השם הפרטי ואיזה שם המשפחה, בסדר שבו הם נכתבים. רשומה ללא חלקים עדיין מוצגת: החלקים הם מה שחיפוש יכול להתאים אליו.
identity-part = חלק
identity-value = טקסט
identity-other-names = שמות אחרים
identity-other-help = שם נישואין, שם דתי, שם שרשומה מאוחרת השתמשה בו. כל אחד נושא את תקופת השימוש בו ואת המקור שאומר זאת.
identity-name-type = סוג השם
identity-valid-from = בשימוש מ-
identity-valid-until = בשימוש עד
identity-about = על האדם
identity-living-help = זה הסימון שהמקור קבע. הדף מניח בנפרד פטירה כשהלידה רחוקה מדי, וההנחה הזאת לעולם אינה משנה את התיבה הזאת או את החבילה.
identity-error-no-display = רשומה צריכה שם שבו היא תוצג. שום דבר לא נשמר.
editor-blank-to-remove = מחקו את השם כדי להסיר את הרשומה הזאת.
identity-edit-link = עריכת שמות וזהות

union-type-marriage = נישואין
union-type-civil_union = ברית זוגיות אזרחית
union-type-cohabitation = חיים משותפים
union-type-religious_only = קשר דתי
union-type-polygamous = פוליגמי
union-type-unknown = לא נרשם
union-role-spouse = בן/בת זוג
union-role-partner = שותף/ה לחיים
union-role-husband = בעל
union-role-wife = אישה
union-status-active = נמשך
union-status-ended_by_death = הסתיים במוות
union-status-ended_by_divorce = הסתיים בגירושין
union-status-ended_by_separation = הסתיים בפרידה
union-status-annulled = בוטל
union-status-unknown = לא נרשם
union-status-ended = הסתיים
union-status-ended-by = הסתיים ב{ $reason }
union-reason-death_of_spouse = מות בן/בת הזוג
precision-exact = עד היום
precision-year = עד השנה
precision-month = עד החודש
precision-decade = עד העשור
precision-century = עד המאה
precision-unknown = לא ידוע
record-precision = דיוק
record-approximate = משוער
record-place = מקום

family-editor-title = משפחה וקשרים
family-unions = קשרים זוגיים
family-no-unions = לא נרשם קשר זוגי לאדם הזה.
family-union-legend = קשר זוגי { $n }
family-writes-family = השמירה משנה את רשומת המשפחה ‎#{ $id }, ששני האנשים חולקים. הדף של האדם האחר משתנה יחד איתה.
family-partners = בני זוג
family-partner = בן/בת זוג
family-role = תפקיד
family-children = ילדים
family-children-help = סדר הלידה הוא טענה של הרשומה עצמה על מי בא ראשון. אם נשאר ריק, דבר אינו נטען: מספר שנלקח ממיקום השורה היה עובדה שאיש לא רשם.
family-child = ילד
family-birth-order = סדר לידה
family-the-union = הקשר הזוגי עצמו
family-type = סוג הקשר
family-status = מצבו
family-started = התחיל
family-ended = הסתיים
family-leave = הוצאת האדם הזה מהקשר הזה
family-open-entity = פתיחת רשומת המשפחה
family-new-union = קשר זוגי חדש
family-new-union-help = פעולה זו יוצרת רשומת משפחה חדשה שהאדם הזה נמצא בה. בן זוג הוא רשות: הורה שהרשומה מזכירה ללא בן זוג כלל הוא קשר של אחד.
family-create-union = יצירת הקשר
family-parents = הורים
family-no-parents = האדם הזה אינו רשום כילד של אף משפחה.
family-child-of = ילד של המשפחה הזאת
family-detach-child = הוצאת האדם הזה מהמשפחה הזאת
family-attach-parents = שיוך להורים
family-attach-help = בחרו את המשפחה שהאדם הזה הוא ילד שלה. זה מוסיף אותו לרשומת המשפחה, כך שהוא מופיע גם בדפי ההורים.
family-the-family = המשפחה
family-attach = שיוך
family-error-last-partner = קשר זוגי צריך לפחות אדם אחד. מחקו במקום זאת את רשומת המשפחה, שתשאל מה לעשות בכל מה שמפנה אליה.
family-error-no-family = לא נבחרה משפחה. שום דבר לא נשמר.
family-error-already-child = האדם הזה כבר ילד של המשפחה הזאת.
pick-error-empty = לא צוין אדם. שום דבר לא נשמר.
pick-error-not-found = אין בארכיון הזה אדם בשם הזה. שום דבר לא נשמר.
pick-error-ambiguous = יותר מאדם אחד עונה לזה. בחרו אחד מהרשימה כדי שהרשומה תאמר מי. שום דבר לא נשמר.

links-editor-title = קשרים
links-editor-help = הקשרים שאינם משפחה: סנדק, מעסיק, עד, גדוד. כל אחד הוא רשומה בפני עצמה שמזכירה שני אנשים, כך שעריכה כאן משנה את מה שהרשומה האחרת מציגה.
links-none = לא נרשם קשר לאדם הזה.
links-new = קשר חדש
links-create = יצירת הקשר
links-remove = הסרת הקשר הזה
links-other-end = הקצה השני
links-label = מהו
links-label-reverse = בקריאה הפוכה
links-category = קטגוריה
links-bidirectional = נקרא אותו דבר בשני הכיוונים
links-from = מ-
links-until = עד
links-reversed = הקשר הזה צויר מהרשומה האחרת. עריכה כאן משנה את אותה ישות.
link-error-no-label = קשר צריך לומר מהו. שום דבר לא נשמר.
occupations-editor-title = עיסוקים
occupations-editor-help = עיסוק הוא תקופה עם התחלה וסוף, לא תואר משרה. כל אחד נושא תאריכים ומקור משלו.
occupations-none = לא נרשם עיסוק לאדם הזה.
occupations-new = עיסוק חדש
occupations-create = יצירת העיסוק
occupations-remove = הסרת העיסוק הזה
occupations-title = מה עשו
occupations-employer = עבור מי
occupations-employer-place = היכן היו
occupations-from = מ-
occupations-until = עד
occupation-error-no-title = עיסוק צריך לומר מה מישהו עשה. שום דבר לא נשמר.
link-category-spiritual = רוחני
link-category-professional = מקצועי
link-category-social = חברתי
link-category-legal = משפטי
link-category-medical = רפואי
link-category-educational = חינוכי
link-category-conflict = סכסוך
link-category-other = אחר
links-edit-link = עריכת קשרים
occupations-edit-link = עריכת עיסוקים
family-edit-link = עריכת משפחה וקשרים

events-editor-title = אירועים
events-editor-help = אירוע מזכיר כמה אנשים בבת אחת — נישואין, טבילה, מפקד — ולכן כל אחד הוא רשומה בפני עצמה ומופיע בכל דף שהוא מזכיר.
events-none = אף אירוע אינו מזכיר את האדם הזה.
events-new = אירוע חדש
events-new-help = האדם הזה מתווסף כנושא האירוע אם לא תציינו איש אחר. אירוע שאין בו איש הוא רק תאריך.
events-create = יצירת האירוע
events-remove = הסרת האירוע הזה
events-category = מה קרה
events-subcategory = ליתר דיוק
events-description = תיאור
events-participants = מי היה שם
events-participants-help = השמירה משנה את רשומת האירוע, שכל אדם אחר שנזכר בה מציג גם הוא.
events-who = מי
event-error-no-category = אירוע צריך לומר מה קרה. שום דבר לא נשמר.
documents-editor-title = מסמכים
documents-editor-help = לאילו קבצים הרשומה הזאת מפנה, ומה כל אחד מהם בשבילה. ניקוי שורה מנתק את הקובץ: המסמך והבתים שלו נשארים בארכיון.
documents-attached = מצורפים לרשומה הזאת
documents-upload = העלאת קובץ
documents-upload-help = עד { $mb } MB. הקובץ נשמר בארכיון ומצורף לרשומה הזאת.
documents-caption = כיתוב
documents-edit-link = צירוף וניתוק מסמכים
events-edit-link = עריכת אירועים

prefs-style = צפיפות
prefs-style-help = כמה מקום הדף תופס. נפרד מערכת הנושא, שעוסקת רק בצבע, כך שאפשר לשלב כל אחת מאלה עם כל אחת מאלה.
style-comfortable = מרווח
style-comfortable-note = ברירת המחדל, עם מקום לקרוא
style-compact = דחוס
style-compact-note = יותר מהרשומה בכל מסך, לעבודה על כמה רשומות
style-paper = נייר
style-paper-note = גופן עם תגים וקווים במקום כרטיסים, לקריאה חד-פעמית או להדפסה

admin-export-choose = לכלול בייצוא הזה
scope-health = בריאות ואמונה
scope-biometrics = ביומטריה
scope-genomics = נתונים גנומיים
scope-legal = רישום פלילי
scope-behaviour = פרופילים התנהגותיים של אנשים חיים
admin-export-with-chosen = ייצוא עם מה שסומן

pg-identity = זהות ומעמד אזרחי
pg-identity-intro = מי היה האדם לפי הרשומה, ומה רשמו מרשמי האוכלוסין.
pg-morphology = מבנה גוף
pg-morphology-intro = הגוף כפי שנמדד ותואר.
pg-biometrics = ביומטריה
pg-biometrics-intro = הקול, הידיים והחושים, והתבניות שמזהות אדם.
pg-health = בריאות
pg-health-intro = מצבים, טיפולים, מדידות ותוצאות.
pg-genomics = גנומיקה
pg-genomics-intro = בדיקות DNA, הפלוגרופים, וריאנטים ותוצאות מולקולריות אחרות.
pg-death = פטירה
pg-death-intro = איך, מתי והיכן הסתיימו החיים, ומה עלה בגורל הגוף.
pg-residence = מגורים ואזרחות
pg-residence-intro = היכן האדם חי, אילו מדינות טענו לו, והשפות שדיבר.
pg-education = השכלה ועבודה
pg-education-intro = לימודים, הסמכות, הכנסה ורכוש.
pg-military = צבא ועיטורים
pg-military-intro = שירות, דרגות, יחידות ואותות.
pg-legal = משפטי
pg-legal-intro = הליכים פליליים ותוצאותיהם.
pg-belief = אמונה והשתייכות
pg-belief-intro = דת, טקסים, השקפות וחברויות.
pg-personality = אישיות והתנהגות
pg-personality-intro = מזג, הרגלים ותחביבים, כפי שהמקורות מתארים אותם.
pg-relationships = קשרים
pg-relationships-intro = הורים, בני זוג, ילדים והאנשים האחרים בחיים.
pg-digital-legacy = מורשת דיגיטלית
pg-digital-legacy-intro = סריקות, מודלים, הקלטות וארכיונים שמייצגים אדם.
pa-identity-titles = תארים
pa-identity-sex-at-birth = מין בלידה
pa-identity-gender-identity = זהות מגדרית
pa-birth-time = שעת הלידה
pa-birth-coordinates = מקום הלידה, כקואורדינטות
pa-civil-status-birth-certificate-number = מספר תעודת הלידה
pa-civil-status-register-entries = רשומות במרשם האוכלוסין
pa-civil-status-marginal-annotations = הערות שוליים
pa-morphology-height = גובה
pa-morphology-weight = משקל
pa-morphology-bmi = מדד מסת גוף
pa-morphology-body-composition = הרכב גוף
pa-morphology-build = מבנה גוף
pa-morphology-eye-colour = צבע עיניים
pa-morphology-eye-shape = צורת העיניים
pa-morphology-eye-spacing = מרחק בין העיניים
pa-morphology-hair-colour = צבע שיער טבעי
pa-morphology-hair-texture = מרקם השיער
pa-morphology-hairline = קו השיער
pa-morphology-facial-hair = שיער פנים
pa-morphology-body-hair = שיער גוף
pa-morphology-skin-tone = גוון עור (פיצפטריק)
pa-morphology-skin-undertone = גוון תת-עורי
pa-morphology-freckles = נמשים
pa-morphology-pigmentation = כתמי פיגמנט
pa-morphology-scars = צלקות
pa-morphology-tattoos = קעקועים
pa-morphology-moles = שומות
pa-morphology-facial-asymmetries = אי-סימטריות בפנים
pa-morphology-face-shape = צורת הפנים
pa-morphology-nose-shape = צורת האף
pa-morphology-ear-shape = צורת האוזניים
pa-morphology-lip-shape = צורת השפתיים
pa-morphology-dentition = שיניים
pa-morphology-malocclusion = סגר לקוי (סיווג Angle)
pa-morphology-posture = יציבה
pa-morphology-gait = הליכה
pa-morphology-distinguishing-features = סימנים מזהים
pa-biometrics-fingerprints = טביעות אצבע
pa-biometrics-retinal-print = טביעת רשתית
pa-biometrics-voice-signature = טביעת קול
pa-biometrics-voice-frequency = תדר הקול הבסיסי
pa-biometrics-vocal-timbre = גוון הקול
pa-biometrics-spoken-accent = מבטא
pa-biometrics-speech-rate = קצב הדיבור
pa-biometrics-verbal-tics = טיקים מילוליים
pa-biometrics-frequent-vocabulary = אוצר מילים שכיח
pa-biometrics-speech-register = משלב הדיבור
pa-biometrics-motor-tics = טיקים מוטוריים
pa-biometrics-handedness = יד דומיננטית
pa-biometrics-hearing = שמיעה
pa-biometrics-visual-acuity = חדות ראייה
pa-biometrics-optical-correction = תיקון אופטי
pa-health-blood-group = סוג דם (ABO)
pa-health-rhesus = גורם Rh ‏(RhD)
pa-health-blood-pressure = לחץ דם
pa-health-resting-heart-rate = דופק במנוחה
pa-health-respiratory-capacity = נפח נשימה
pa-health-conditions = מצבים רפואיים
pa-health-surgeries = היסטוריה ניתוחית
pa-health-injuries = פציעות
pa-health-deformities = עיוותים
pa-health-amputations = קטיעות
pa-health-prostheses = תותבות
pa-health-implants = שתלים
pa-health-devices = מכשירים מושתלים
pa-health-medications = תרופות
pa-health-allergies = אלרגיות
pa-health-vaccinations = חיסונים
pa-health-serology = סרולוגיה
pa-health-lab-results = תוצאות מעבדה
pa-health-deficiencies = חסרים
pa-health-sleep-disorders = הפרעות שינה
pa-health-mental-health-assessments = הערכות בריאות הנפש
pa-genomics-autosomal-mapping = בדיקת DNA אוטוזומלית
pa-genomics-y-haplogroup = הפלוגרופ Y-DNA
pa-genomics-mt-haplogroup = הפלוגרופ מיטוכונדריאלי
pa-genomics-whole-genome-sequencing = ריצוף גנום מלא
pa-genomics-risk-variants = וריאנטים של סיכון
pa-genomics-hereditary-conditions = מצבים תורשתיים

pa-genomics-predispositions = נטיות
pa-genomics-epigenetic-markers = סמנים אפיגנטיים
pa-genomics-epigenetic-age = גיל אפיגנטי
pa-genomics-gut-microbiome = מיקרוביום המעי
pa-genomics-skin-microbiome = מיקרוביום העור
pa-genomics-toxicological-sensitivities = רגישות לתרופות ולרעלים
pa-death-time = שעת הפטירה
pa-death-coordinates = מקום הפטירה, כקואורדינטות
pa-death-causes = סיבות המוות
pa-death-contributing-factors = גורמים תורמים
pa-death-autopsy = נתיחה
pa-death-disposition = מה נעשה בגוף
pa-death-grave = קבר
pa-residence-addresses = כתובות
pa-residence-nationality-of-origin = אזרחות מלידה
pa-residence-acquired-nationalities = אזרחויות שנרכשו
pa-residence-mother-tongue = שפת אם
pa-residence-spoken-languages = שפות מדוברות
pa-education-level = רמת השכלה
pa-education-diplomas = תעודות ותארים
pa-education-institutions = בתי ספר ומוסדות
pa-education-income = הכנסה
pa-education-real-estate = נדל״ן
pa-military-distinctions = עיטורים
pa-military-citations = ציונים לשבח
pa-military-ranks = דרגות
pa-military-units = יחידות
pa-military-service-numbers = מספרים אישיים
pa-legal-criminal-record = רישום פלילי
pa-belief-religions = דת
pa-belief-sacraments = טקסים ומצוות
pa-belief-beliefs = אמונות
pa-belief-political-leanings = נטיות פוליטיות
pa-belief-memberships = חברויות
pa-personality-big-five = ציוני חמשת הגדולים
pa-personality-mbti = טיפוס MBTI
pa-personality-introversion-extraversion = מופנמות ומוחצנות
pa-personality-stress-tolerance = עמידות בלחץ
pa-personality-decision-style = סגנון קבלת החלטות
pa-personality-interests = תחומי עניין
pa-personality-hobbies = תחביבים
pa-personality-sports = ספורט
pa-personality-dietary-habits = תזונה
pa-personality-dependencies = התמכרויות
pa-digital-legacy-body-models = מודלים של הגוף
pa-digital-legacy-skin-textures = מרקמי עור
pa-digital-legacy-rigs = שלדים דיגיטליים
pa-digital-legacy-voice-corpora = הקלטות קול לסינתזה
pa-digital-legacy-text-corpora = כתבים עבור מודל שפה
pa-digital-legacy-digital-traces = עקבות דיגיטליים
pa-digital-legacy-carbon-footprint = טביעת רגל פחמנית
pa-digital-legacy-behaviour-models = מודלים התנהגותיים
pf-identity-titles-text = התואר כפי שנכתב
pf-identity-titles-kind = סוג התואר
pf-civil-status-marginal-annotations-text = הערה
pf-morphology-pigmentation-kind = סוג הכתם
pf-biometrics-spoken-accent-description = איך הוא מתואר
pf-biometrics-optical-correction-kind = תיקון
pf-health-amputations-level = גובה הקטיעה
pf-health-prostheses-kind = תותבת
pf-health-implants-kind = שתל
pf-health-devices-kind = מכשיר
pf-health-allergies-type = סוג האלרגיה
pf-health-vaccinations-status = מצב החיסון
pf-health-sleep-disorders-category = קטגוריית ההפרעה
pf-death-autopsy-kind = נתיחה
pf-education-institutions-name = שם המוסד
pf-military-distinctions-name = שם העיטור
pf-military-distinctions-kind = סוג העיטור
pf-military-citations-text = ציון לשבח
pf-military-ranks-category = קטגוריית הדרגה
pf-belief-political-leanings-position = מיקום על ציר שמאל–ימין
pf-belief-memberships-kind = סוג הארגון
pf-digital-legacy-carbon-footprint-method = איך זה הוערך
pf-age-years = גיל בשנים
pf-agreeableness = נעימות
pf-allergen = אלרגן
pf-amount = סכום
pf-analyte = אנליט
pf-artefact-type = סוג התוצר
pf-autoimmune = אוטואימוני
pf-body-region = אזור בגוף
pf-bone-percent = עצם
pf-carrier-status = מצב נשאות
pf-cause = סיבה
pf-chronic = כרוני
pf-clock = שעון
pf-condition = מצב
pf-conferred-by = הוענק על ידי
pf-congenital = מולד
pf-conscientiousness = מצפוניות
pf-consent = הסכמה
pf-coordinates = קואורדינטות
pf-corrected = עם תיקון
pf-country = מדינה
pf-court = בית משפט
pf-coverage = כיסוי
pf-currency = מטבע
pf-decimal = חדות (עשרונית)
pf-denomination = זרם
pf-derived-from-id = נגזר מ-
pf-description = תיאור
pf-details = פרטים
pf-diagnosis = אבחנה
pf-diameter-mm = קוטר
pf-diastolic = דיאסטולי
pf-diet = תזונה
pf-document-id = מסמך
pf-dose = מינון
pf-ear = אוזן
pf-entry-number = מספר רשומה
pf-extraversion = מוחצנות
pf-eye = עין
pf-fat-percent = שומן
pf-fev1-fvc-ratio = יחס FEV1/FVC
pf-fev1-litres = FEV1
pf-file-format = תבנית קובץ
pf-findings = ממצאים
pf-flag = סימון
pf-format = תבנית
pf-fracture = שבר
pf-fvc-litres = FVC
pf-gene = גן
pf-generator = נוצר באמצעות
pf-grade = דרגה
pf-iccs-section = סעיף העבירה (ICCS)
pf-icd10-chapter = פרק ICD-10
pf-indication = התוויה
pf-inheritance = תורשה
pf-inscription = כתובת
pf-institution = מוסד
pf-instrument = מכשיר
pf-isced-level = רמת ISCED
pf-jurisdiction = סמכות שיפוט
pf-language = שפה
pf-lat = קו רוחב
pf-level = רמה
pf-lines = כתובת
pf-location = מיקום
pf-lon = קו אורך
pf-major = הפלוגרופ ראשי
pf-marker = סמן
pf-metaboliser-status = מצב מטבולי
pf-method = שיטה
pf-mode = איך נרכש
pf-muscle-percent = שריר
pf-neuroticism = נוירוטיות
pf-number = מספר
pf-nutrient = רכיב תזונתי
pf-offence = עבירה
pf-office = תפקיד
pf-openness = פתיחות
pf-organisation = ארגון
pf-outcome = תוצאה
pf-pace = קצב ההזדקנות
pf-page = עמוד
pf-panel = פאנל
pf-party = מפלגה
pf-pathogen = פתוגן
pf-pattern = דפוס שימוש
pf-percentile = אחוזון
pf-period = תקופת תשלום
pf-place-id = מקום
pf-plot = חלקה
pf-polygenic-score = ציון פוליגני
pf-postal-code = מיקוד
pf-precision = דיוק
pf-prescription = מרשם
pf-proficiency = רמת שליטה
pf-provider = ספק
pf-quintile = חמישון הכנסה
pf-rank = דרגה
pf-rank-text = הדרגה כפי שנכתבה
pf-reaction = תגובה
pf-reference-build = גנום ייחוס
pf-reference-high = טווח ייחוס, עליון
pf-reference-low = טווח ייחוס, תחתון
pf-register-type = סוג הרשומה
pf-result = תוצאה
pf-role = תפקיד
pf-sacrament = טקס או מצווה
pf-score = ציון
pf-sentence = עונש
pf-sequence = מקום בשרשרת
pf-service = שירות
pf-severity = חומרה
pf-shannon-diversity = מגוון שנון
pf-shape = צורה
pf-significance = משמעות קלינית
pf-snp-count = SNPs שנבדקו
pf-sport = ענף ספורט
pf-subclade = תת-ענף
pf-substance = חומר
pf-summary = סיכום
pf-systolic = סיסטולי
pf-tenure = חזקה
pf-test = בדיקה
pf-threshold-db = סף שמיעה
pf-title = כותרת
pf-tonnes-co2e-per-year = פליטות
pf-tradition = מסורת
pf-tree-version = גרסת העץ
pf-unit = יחידה
pf-use = שימוש
pf-variant = וריאנט
pf-volume = כרך
pf-zygosity = זיגוטיות
pu-cm = { $n } ס״מ
pu-kg = { $n } ק״ג
pu-kg-m2 = { $n } ק״ג/מ״ר
pu-percent = { $n }%
pu-mm = { $n } מ״מ
pu-hz = { $n } הרץ
pu-words-min = { $n } מילים לדקה
pu-db-hl = { $n } dB HL
pu-mmhg = { $n } מ״מ כספית
pu-bpm = { $n } פעימות לדקה
pu-litres = { $n } ליטר
pu-coverage = { $n }×
pu-years = { $n } שנים
pu-t-co2e-yr = { $n } טון CO₂e בשנה
pv-sensitive-class-health = בריאות ואמונה
pv-sensitive-class-biometrics = ביומטריה
pv-sensitive-class-genomics = נתונים גנומיים
pv-sensitive-class-legal = רישום פלילי
pv-laterality-left = שמאל
pv-laterality-right = ימין
pv-laterality-both = שניהם
pv-body-region-head = ראש
pv-body-region-face = פנים
pv-body-region-neck = צוואר
pv-body-region-left-shoulder = כתף שמאל
pv-body-region-right-shoulder = כתף ימין
pv-body-region-left-arm = זרוע שמאל
pv-body-region-right-arm = זרוע ימין
pv-body-region-left-hand = כף יד שמאל
pv-body-region-right-hand = כף יד ימין
pv-body-region-chest = חזה
pv-body-region-abdomen = בטן
pv-body-region-upper-back = גב עליון
pv-body-region-lower-back = גב תחתון
pv-body-region-pelvis = אגן וירכיים
pv-body-region-left-leg = רגל שמאל
pv-body-region-right-leg = רגל ימין
pv-body-region-left-foot = כף רגל שמאל
pv-body-region-right-foot = כף רגל ימין
pv-body-region-internal = פנימי
pv-body-region-whole-body = כל הגוף
pv-body-region-other = אזור אחר
pv-artefact-type-mesh = רשת תלת-ממדית
pv-artefact-type-point-cloud = ענן נקודות
pv-artefact-type-skin-texture-map = מפת מרקם עור
pv-artefact-type-skeletal-rig = שלד דיגיטלי
pv-artefact-type-voice-corpus = מאגר קול
pv-artefact-type-text-corpus = מאגר טקסט
pv-artefact-type-trace-archive = ארכיון פעילות מקוונת
pv-artefact-type-behaviour-model = מודל התנהגותי
pv-artefact-type-fingerprint-card = כרטיס טביעות אצבע
pv-artefact-type-fingerprint-template = תבנית טביעת אצבע
pv-artefact-type-retinal-image = תמונת רשתית
pv-artefact-type-voiceprint = טביעת קול
pv-consent-given = ניתנה
pv-consent-given-by-estate = ניתנה על ידי העיזבון
pv-consent-refused = סורבה
pv-consent-withdrawn = נמשכה
pv-consent-not-asked = לא התבקשה
pv-consent-unknown = לא ידוע
pv-sex-at-birth-female = נקבה
pv-sex-at-birth-male = זכר
pv-sex-at-birth-intersex = אינטרסקס
pv-sex-at-birth-undetermined = לא נקבע
pv-sex-at-birth-unknown = לא ידוע
pv-gender-identity-woman = אישה
pv-gender-identity-man = גבר
pv-gender-identity-non-binary = א-בינארי
pv-gender-identity-other = אחר
pv-gender-identity-undisclosed = לא נמסר
pv-gender-identity-unknown = לא ידוע
pv-title-kind-nobility = אצולה
pv-title-kind-academic = אקדמי
pv-title-kind-professional = מקצועי
pv-title-kind-religious = דתי
pv-title-kind-military = צבאי
pv-title-kind-civic = אזרחי או של כבוד
pv-title-kind-courtesy = נימוס
pv-title-kind-other = אחר
pv-register-type-birth = לידה
pv-register-type-baptism = טבילה
pv-register-type-marriage = נישואין
pv-register-type-death = פטירה
pv-register-type-burial = קבורה
pv-register-type-divorce = גירושין
pv-register-type-recognition = הכרה בילד
pv-register-type-legitimation = הכשרת ילד
pv-register-type-adoption = אימוץ
pv-register-type-name-change = שינוי שם
pv-register-type-other = אחר
pv-build-slight = עדין
pv-build-slim = רזה
pv-build-average = ממוצע

pv-build-sturdy = חסון
pv-build-stout = מלא
pv-build-heavy = כבד
pv-eye-colour-light-blue = כחול בהיר
pv-eye-colour-blue = כחול
pv-eye-colour-dark-blue = כחול כהה
pv-eye-colour-grey = אפור
pv-eye-colour-blue-grey = כחול-אפור
pv-eye-colour-green = ירוק
pv-eye-colour-grey-green = אפור-ירוק
pv-eye-colour-hazel = ערמוני-ירקרק
pv-eye-colour-amber = ענבר
pv-eye-colour-light-brown = חום בהיר
pv-eye-colour-brown = חום
pv-eye-colour-dark-brown = חום כהה
pv-eye-colour-black = שחור
pv-eye-colour-mixed = מעורב
pv-eye-colour-other = אחר
pv-eye-shape-almond = שקד
pv-eye-shape-round = עגול
pv-eye-shape-hooded = עפעף נפול
pv-eye-shape-monolid = עפעף יחיד
pv-eye-shape-deep-set = שקועות
pv-eye-shape-protruding = בולטות
pv-eye-shape-upturned = מורמות בקצה
pv-eye-shape-downturned = מורדות בקצה
pv-eye-shape-other = אחר
pv-eye-spacing-close-set = קרובות
pv-eye-spacing-average = ממוצע
pv-eye-spacing-wide-set = מרוחקות
pv-hair-colour-black = שחור
pv-hair-colour-dark-brown = חום כהה
pv-hair-colour-brown = חום
pv-hair-colour-light-brown = חום בהיר
pv-hair-colour-auburn = ערמוני-אדמדם
pv-hair-colour-red = ג׳ינג׳י
pv-hair-colour-strawberry-blond = בלונד תותי
pv-hair-colour-dark-blond = בלונד כהה
pv-hair-colour-blond = בלונד
pv-hair-colour-light-blond = בלונד בהיר
pv-hair-colour-grey = אפור
pv-hair-colour-white = לבן
pv-hair-colour-none = ללא שיער
pv-hair-colour-other = אחר
pv-hair-texture-straight = חלק
pv-hair-texture-wavy = גלי
pv-hair-texture-curly = מתולתל
pv-hair-texture-coily = מסולסל מאוד
pv-hair-texture-other = אחר
pv-hairline-straight = ישר
pv-hairline-rounded = מעוגל
pv-hairline-widows-peak = שפיץ במרכז המצח
pv-hairline-m-shaped = בצורת M
pv-hairline-bell-shaped = בצורת פעמון
pv-hairline-uneven = לא אחיד
pv-hairline-receding = נסוג
pv-hairline-bald = קירח
pv-facial-hair-none = אין
pv-facial-hair-stubble = זיפים
pv-facial-hair-moustache = שפם
pv-facial-hair-goatee = זקנקן
pv-facial-hair-full-beard = זקן מלא
pv-facial-hair-sideburns = פאות לחיים
pv-facial-hair-other = אחר
pv-body-hair-none = אין
pv-body-hair-sparse = דליל
pv-body-hair-moderate = בינוני
pv-body-hair-dense = צפוף
pv-skin-tone-type-i = סוג I — תמיד נכווה, לעולם אינו משתזף
pv-skin-tone-type-ii = סוג II — בדרך כלל נכווה, משתזף מעט
pv-skin-tone-type-iii = סוג III — לפעמים נכווה, משתזף באופן אחיד
pv-skin-tone-type-iv = סוג IV — לעיתים רחוקות נכווה, משתזף היטב
pv-skin-tone-type-v = סוג V — לעיתים רחוקות מאוד נכווה
pv-skin-tone-type-vi = סוג VI — לעולם אינו נכווה
pv-skin-undertone-cool = קר
pv-skin-undertone-neutral = ניטרלי
pv-skin-undertone-warm = חם
pv-skin-undertone-olive = זית
pv-freckles-none = אין
pv-freckles-few = מעט
pv-freckles-moderate = בינוני
pv-freckles-many = הרבה
pv-pigmentation-mark-birthmark = כתם לידה
pv-pigmentation-mark-port-wine-stain = כתם יין
pv-pigmentation-mark-cafe-au-lait-spot = כתם קפה בחלב
pv-pigmentation-mark-depigmented-patch = כתם חסר פיגמנט
pv-pigmentation-mark-hyperpigmented-patch = כתם עודף פיגמנט
pv-pigmentation-mark-other = אחר
pv-mole-shape-round = עגול
pv-mole-shape-oval = סגלגל
pv-mole-shape-irregular = לא סדיר
pv-mole-shape-other = אחר
pv-face-shape-oval = סגלגל
pv-face-shape-round = עגול
pv-face-shape-square = מרובע
pv-face-shape-oblong = מוארך
pv-face-shape-heart = בצורת לב
pv-face-shape-diamond = יהלום
pv-face-shape-triangular = משולש
pv-nose-shape-straight = ישר
pv-nose-shape-aquiline = נשרי
pv-nose-shape-snub = סולד
pv-nose-shape-upturned = מורם
pv-nose-shape-flat = שטוח
pv-nose-shape-broad = רחב
pv-nose-shape-bulbous = בולבוסי
pv-nose-shape-crooked = עקום
pv-nose-shape-other = אחר
pv-ear-shape-free-lobe = תנוכים חופשיים
pv-ear-shape-attached-lobe = תנוכים מחוברים
pv-ear-shape-protruding = בולטות
pv-ear-shape-close-set = צמודות לראש
pv-ear-shape-pointed = מחודדות
pv-ear-shape-other = אחר
pv-lip-shape-thin = דקות
pv-lip-shape-medium = בינוניות
pv-lip-shape-full = מלאות
pv-lip-shape-bow-shaped = בצורת קשת
pv-lip-shape-wide = רחבות
pv-lip-shape-downturned = מורדות בקצוות
pv-lip-shape-other = אחר
pv-dentition-primary = שיני חלב
pv-dentition-mixed = מעורבות
pv-dentition-permanent-complete = קבועות, מלאות
pv-dentition-permanent-partial-loss = קבועות, חלקן חסרות
pv-dentition-edentulous = ללא שיניים טבעיות
pv-dentition-partial-denture = תותבת חלקית
pv-dentition-full-denture = תותבת מלאה
pv-dentition-implants = שתלים דנטליים
pv-malocclusion-normal = סגר תקין
pv-malocclusion-class-i = מחלקה I
pv-malocclusion-class-ii-division-1 = מחלקה II, חלוקה 1
pv-malocclusion-class-ii-division-2 = מחלקה II, חלוקה 2
pv-malocclusion-class-iii = מחלקה III
pv-posture-ideal = אידיאלית
pv-posture-kyphotic-lordotic = קיפוזיס-לורדוזיס
pv-posture-flat-back = גב שטוח
pv-posture-sway-back = גב שקוע
pv-posture-stooped = כפופה
pv-posture-scoliotic = עקמת
pv-posture-other = אחר
pv-gait-brisk = נמרצת
pv-gait-average = ממוצעת
pv-gait-slow = איטית
pv-gait-shuffling = גוררת רגליים
pv-gait-limping = צולעת
pv-gait-waddling = מתנדנדת
pv-gait-unsteady = לא יציבה
pv-gait-stiff = נוקשה
pv-gait-other = אחר
pv-vocal-timbre-bright = בהיר
pv-vocal-timbre-dark = כהה
pv-vocal-timbre-warm = חם
pv-vocal-timbre-breathy = נושף
pv-vocal-timbre-nasal = אנפופי
pv-vocal-timbre-hoarse = צרוד
pv-vocal-timbre-resonant = מהדהד
pv-vocal-timbre-thin = דק
pv-vocal-timbre-other = אחר
pv-speech-register-frozen = טקסי
pv-speech-register-formal = רשמי
pv-speech-register-consultative = מקצועי
pv-speech-register-casual = יומיומי
pv-speech-register-intimate = אינטימי
pv-handedness-left = שמאלי
pv-handedness-right = ימני
pv-handedness-ambidextrous = דו-ידי
pv-handedness-mixed = מעורב
pv-handedness-unknown = לא ידוע
pv-hearing-grade-normal = תקינה
pv-hearing-grade-mild = קלה
pv-hearing-grade-moderate = בינונית
pv-hearing-grade-moderately-severe = בינונית-קשה
pv-hearing-grade-severe = קשה
pv-hearing-grade-profound = עמוקה
pv-hearing-grade-complete = מלאה
pv-optical-correction-none = אין
pv-optical-correction-glasses = משקפיים
pv-optical-correction-contact-lenses = עדשות מגע
pv-optical-correction-glasses-and-contact-lenses = משקפיים ועדשות מגע
pv-optical-correction-refractive-surgery = ניתוח להסרת משקפיים
pv-optical-correction-intraocular-lens = עדשה תוך-עינית
pv-optical-correction-other = אחר
pv-rhesus-positive = RhD חיובי
pv-rhesus-negative = RhD שלילי
pv-rhesus-weak-d = D חלש
pv-rhesus-unknown = לא ידוע
pv-icd10-chapter-infectious-parasitic = I מחלות זיהומיות וטפיליות
pv-icd10-chapter-neoplasms = II גידולים
pv-icd10-chapter-blood-immune = III מחלות דם ומערכת החיסון
pv-icd10-chapter-endocrine-metabolic = IV אנדוקריני, תזונתי ומטבולי
pv-icd10-chapter-mental-behavioural = V הפרעות נפשיות והתנהגותיות
pv-icd10-chapter-nervous-system = VI מערכת העצבים
pv-icd10-chapter-eye-adnexa = VII העין ונספחיה
pv-icd10-chapter-ear-mastoid = VIII האוזן ותהליך המסטואיד
pv-icd10-chapter-circulatory = IX מערכת הדם
pv-icd10-chapter-respiratory = X מערכת הנשימה
pv-icd10-chapter-digestive = XI מערכת העיכול
pv-icd10-chapter-skin = XII העור והרקמה התת-עורית
pv-icd10-chapter-musculoskeletal = XIII מערכת השלד והשרירים
pv-icd10-chapter-genitourinary = XIV מערכת השתן והמין
pv-icd10-chapter-pregnancy-childbirth = XV היריון ולידה
pv-icd10-chapter-perinatal = XVI מצבים סביב הלידה
pv-icd10-chapter-congenital = XVII מומים מולדים
pv-icd10-chapter-ill-defined = XVIII תסמינים וסיבות לא מוגדרות
pv-icd10-chapter-injury-poisoning = XIX פציעות והרעלות
pv-icd10-chapter-external-causes = XX סיבות חיצוניות
pv-icd10-chapter-health-factors = XXI גורמים המשפיעים על הבריאות
pv-icd10-chapter-special-purposes = XXII קודים למטרות מיוחדות
pv-diagnosis-status-diagnosed = אובחן
pv-diagnosis-status-suspected = חשד
pv-diagnosis-status-self-reported = דיווח עצמי
pv-diagnosis-status-unknown = לא ידוע
pv-prosthesis-kind-limb = גפה
pv-prosthesis-kind-joint = החלפת מפרק
pv-prosthesis-kind-ocular = עין
pv-prosthesis-kind-dental = שיניים
pv-prosthesis-kind-auditory = שמיעה
pv-prosthesis-kind-breast = שד
pv-prosthesis-kind-other = אחר
pv-implant-kind-orthopaedic = אורתופדי
pv-implant-kind-dental = דנטלי
pv-implant-kind-cochlear = שתל שבלול
pv-implant-kind-breast = שד
pv-implant-kind-intraocular-lens = עדשה תוך-עינית
pv-implant-kind-contraceptive = אמצעי מניעה
pv-implant-kind-cosmetic = קוסמטי
pv-implant-kind-other = אחר
pv-device-kind-pacemaker = קוצב לב
pv-device-kind-implantable-defibrillator = דפיברילטור מושתל
pv-device-kind-cardiac-resynchronisation = מכשיר לסנכרון הלב
pv-device-kind-ventricular-assist = מכשיר עזר חדרי
pv-device-kind-neurostimulator = ממריץ עצבי
pv-device-kind-insulin-pump = משאבת אינסולין
pv-device-kind-drug-port = פורט למתן תרופות
pv-device-kind-shunt = שאנט
pv-device-kind-stent = סטנט
pv-device-kind-other = אחר
pv-allergy-type-drug = תרופה
pv-allergy-type-food = מזון
pv-allergy-type-environmental = סביבתית
pv-allergy-type-insect-venom = ארס חרקים
pv-allergy-type-latex = לטקס
pv-allergy-type-other = אחר
pv-allergy-severity-mild = קלה
pv-allergy-severity-moderate = בינונית
pv-allergy-severity-severe = חמורה
pv-allergy-severity-anaphylactic = אנפילקטית
pv-allergy-severity-unknown = לא ידוע
pv-pathogen-diphtheria = אסכרה
pv-pathogen-tetanus = טטנוס
pv-pathogen-pertussis = שעלת
pv-pathogen-poliomyelitis = פוליו
pv-pathogen-measles = חצבת
pv-pathogen-mumps = חזרת
pv-pathogen-rubella = אדמת
pv-pathogen-varicella = אבעבועות רוח
pv-pathogen-smallpox = אבעבועות שחורות
pv-pathogen-tuberculosis = שחפת
pv-pathogen-hepatitis-a = דלקת כבד A
pv-pathogen-hepatitis-b = דלקת כבד B
pv-pathogen-hepatitis-c = דלקת כבד C
pv-pathogen-haemophilus-influenzae-b = המופילוס אינפלואנזה סוג b
pv-pathogen-pneumococcal = מחלה פנאומוקוקית
pv-pathogen-meningococcal = מחלה מנינגוקוקית
pv-pathogen-human-papillomavirus = נגיף הפפילומה האנושי
pv-pathogen-influenza = שפעת
pv-pathogen-covid-19 = COVID-19
pv-pathogen-rotavirus = נגיף רוטה
pv-pathogen-yellow-fever = קדחת צהובה
pv-pathogen-typhoid = טיפוס הבטן
pv-pathogen-cholera = כולרה
pv-pathogen-rabies = כלבת
pv-pathogen-japanese-encephalitis = דלקת מוח יפנית
pv-pathogen-tick-borne-encephalitis = דלקת מוח מקרציות
pv-pathogen-hiv = HIV
pv-pathogen-syphilis = עגבת
pv-pathogen-toxoplasmosis = טוקסופלזמוזיס
pv-pathogen-cytomegalovirus = ציטומגלווירוס
pv-pathogen-epstein-barr = נגיף אפשטיין–בר
pv-pathogen-other = אחר
pv-vaccination-status-vaccinated = מחוסן
pv-vaccination-status-partially-vaccinated = מחוסן חלקית
pv-vaccination-status-unvaccinated = לא מחוסן
pv-vaccination-status-contraindicated = התווית נגד
pv-vaccination-status-unknown = לא ידוע
pv-serology-result-positive = חיובי
pv-serology-result-negative = שלילי
pv-serology-result-equivocal = לא חד-משמעי
pv-serology-result-unknown = לא ידוע
pv-lab-panel-basic-metabolic = פאנל מטבולי בסיסי
pv-lab-panel-lipid = פרופיל שומנים
pv-lab-panel-liver = תפקודי כבד
pv-lab-panel-renal = תפקודי כליה
pv-lab-panel-glycated-haemoglobin = המוגלובין מסוכרר
pv-lab-panel-iron = בדיקות ברזל
pv-lab-analyte-sodium = נתרן
pv-lab-analyte-potassium = אשלגן
pv-lab-analyte-chloride = כלוריד
pv-lab-analyte-bicarbonate = ביקרבונט

pv-lab-analyte-urea = אוריאה
pv-lab-analyte-creatinine = קריאטינין
pv-lab-analyte-glucose = גלוקוז
pv-lab-analyte-calcium = סידן
pv-lab-analyte-total-cholesterol = כולסטרול כללי
pv-lab-analyte-ldl-cholesterol = כולסטרול LDL
pv-lab-analyte-hdl-cholesterol = כולסטרול HDL
pv-lab-analyte-triglycerides = טריגליצרידים
pv-lab-analyte-non-hdl-cholesterol = כולסטרול שאינו HDL
pv-lab-analyte-alt = אלנין אמינוטרנספראז (ALT)
pv-lab-analyte-ast = אספרטט אמינוטרנספראז (AST)
pv-lab-analyte-alp = פוספטאז בסיסי (ALP)
pv-lab-analyte-ggt = גמא-גלוטמיל טרנספראז (GGT)
pv-lab-analyte-total-bilirubin = בילירובין כללי
pv-lab-analyte-direct-bilirubin = בילירובין ישיר
pv-lab-analyte-albumin = אלבומין
pv-lab-analyte-total-protein = חלבון כללי
pv-lab-analyte-egfr = GFR משוער
pv-lab-analyte-uric-acid = חומצת שתן
pv-lab-analyte-phosphate = זרחה
pv-lab-analyte-urine-albumin-creatinine-ratio = יחס אלבומין לקריאטינין בשתן
pv-lab-analyte-hba1c = HbA1c
pv-lab-analyte-serum-iron = ברזל בסרום
pv-lab-analyte-ferritin = פריטין
pv-lab-analyte-transferrin = טרנספרין
pv-lab-analyte-transferrin-saturation = רוויית טרנספרין
pv-lab-analyte-tibc = יכולת קשירת ברזל כוללת
pv-lab-flag-low = נמוך
pv-lab-flag-normal = תקין
pv-lab-flag-high = גבוה
pv-lab-flag-critical-low = נמוך באופן קריטי
pv-lab-flag-critical-high = גבוה באופן קריטי
pv-nutrient-vitamin-a = ויטמין A
pv-nutrient-thiamine = תיאמין (B1)
pv-nutrient-riboflavin = ריבופלבין (B2)
pv-nutrient-niacin = ניאצין (B3)
pv-nutrient-vitamin-b6 = ויטמין B6
pv-nutrient-folate = חומצה פולית (B9)
pv-nutrient-vitamin-b12 = ויטמין B12
pv-nutrient-vitamin-c = ויטמין C
pv-nutrient-vitamin-d = ויטמין D
pv-nutrient-vitamin-e = ויטמין E
pv-nutrient-vitamin-k = ויטמין K
pv-nutrient-iron = ברזל
pv-nutrient-zinc = אבץ
pv-nutrient-magnesium = מגנזיום
pv-nutrient-calcium = סידן
pv-nutrient-iodine = יוד
pv-nutrient-selenium = סלניום
pv-nutrient-copper = נחושת
pv-nutrient-potassium = אשלגן
pv-nutrient-phosphorus = זרחן
pv-nutrient-other = אחר
pv-sleep-disorder-insomnia = נדודי שינה
pv-sleep-disorder-sleep-related-breathing = הפרעת נשימה בשינה
pv-sleep-disorder-central-hypersomnolence = ישנוניות-יתר מרכזית
pv-sleep-disorder-circadian-rhythm = הפרעת קצב יממה
pv-sleep-disorder-parasomnia = פאראסומניה
pv-sleep-disorder-sleep-related-movement = הפרעת תנועה בשינה
pv-sleep-disorder-other = אחר
pv-assessment-instrument-phq-9 = PHQ-9
pv-assessment-instrument-gad-7 = GAD-7
pv-assessment-instrument-bdi-ii = BDI-II
pv-assessment-instrument-hads = HADS
pv-assessment-instrument-k10 = K10
pv-assessment-instrument-gds-15 = GDS-15
pv-assessment-instrument-mmse = MMSE
pv-assessment-instrument-moca = MoCA
pv-assessment-instrument-audit = AUDIT
pv-assessment-instrument-clinical-interview = ראיון קליני
pv-assessment-instrument-other = אחר
pv-assessment-severity-none-minimal = אין או מזערי
pv-assessment-severity-mild = קל
pv-assessment-severity-moderate = בינוני
pv-assessment-severity-moderately-severe = בינוני-חמור
pv-assessment-severity-severe = חמור
pv-reference-build-grch36 = GRCh36 (hg18)
pv-reference-build-grch37 = GRCh37 (hg19)
pv-reference-build-grch38 = GRCh38 (hg38)
pv-reference-build-t2t-chm13 = T2T-CHM13
pv-genomic-file-format-raw-microarray = נתוני מיקרו-מערך גולמיים
pv-genomic-file-format-fastq = FASTQ
pv-genomic-file-format-bam = BAM
pv-genomic-file-format-cram = CRAM
pv-genomic-file-format-vcf = VCF
pv-genomic-file-format-gvcf = gVCF
pv-genomic-file-format-other = אחר
pv-zygosity-heterozygous = הטרוזיגוטי
pv-zygosity-homozygous = הומוזיגוטי
pv-zygosity-hemizygous = המיזיגוטי
pv-zygosity-compound-heterozygous = הטרוזיגוטי מורכב
pv-clinical-significance-pathogenic = פתוגני
pv-clinical-significance-likely-pathogenic = פתוגני ככל הנראה
pv-clinical-significance-uncertain-significance = משמעות לא ודאית
pv-clinical-significance-likely-benign = שפיר ככל הנראה
pv-clinical-significance-benign = שפיר
pv-inheritance-pattern-autosomal-dominant = אוטוזומלי דומיננטי
pv-inheritance-pattern-autosomal-recessive = אוטוזומלי רצסיבי
pv-inheritance-pattern-x-linked-dominant = דומיננטי תלוי X
pv-inheritance-pattern-x-linked-recessive = רצסיבי תלוי X
pv-inheritance-pattern-y-linked = תלוי Y
pv-inheritance-pattern-mitochondrial = מיטוכונדריאלי
pv-inheritance-pattern-multifactorial = רב-גורמי
pv-inheritance-pattern-unknown = לא ידוע
pv-carrier-status-affected = חולה
pv-carrier-status-carrier = נשא
pv-carrier-status-not-carrier = אינו נשא
pv-carrier-status-unknown = לא ידוע
pv-epigenetic-clock-horvath = Horvath
pv-epigenetic-clock-hannum = Hannum
pv-epigenetic-clock-phenoage = PhenoAge
pv-epigenetic-clock-grimage = GrimAge
pv-epigenetic-clock-dunedinpace = DunedinPACE
pv-epigenetic-clock-other = אחר
pv-metaboliser-status-poor = מפרק איטי
pv-metaboliser-status-intermediate = מפרק בינוני
pv-metaboliser-status-normal = מפרק תקין
pv-metaboliser-status-rapid = מפרק מהיר
pv-metaboliser-status-ultrarapid = מפרק מהיר מאוד
pv-autopsy-not-performed = לא בוצעה
pv-autopsy-clinical = קלינית
pv-autopsy-forensic = משפטית
pv-autopsy-external-examination = בדיקה חיצונית בלבד
pv-autopsy-unknown = לא ידוע
pv-disposition-burial = קבורה
pv-disposition-cremation = שריפה
pv-disposition-entombment = קבורה במערה או במאוזוליאום
pv-disposition-burial-at-sea = קבורה בים
pv-disposition-natural-burial = קבורה טבעית
pv-disposition-body-donation = נתרם למדע
pv-disposition-other = אחר
pv-disposition-unknown = לא ידוע
pv-address-use-principal = מגורים עיקריים
pv-address-use-secondary = מגורים משניים
pv-address-use-temporary = מגורים זמניים
pv-address-use-postal = כתובת למשלוח דואר
pv-address-use-other = אחר
pv-nationality-mode-descent = מכוח מוצא
pv-nationality-mode-birth-in-territory = מכוח לידה בשטח המדינה
pv-nationality-mode-naturalisation = מכוח התאזרחות
pv-nationality-mode-marriage = מכוח נישואין
pv-nationality-mode-registration = מכוח רישום
pv-nationality-mode-restoration = מכוח השבה
pv-nationality-mode-state-succession = מכוח שינוי ריבונות
pv-nationality-mode-other = אחר
pv-language-proficiency-a1 = A1 מתחיל
pv-language-proficiency-a2 = A2 בסיסי
pv-language-proficiency-b1 = B1 בינוני
pv-language-proficiency-b2 = B2 בינוני-גבוה
pv-language-proficiency-c1 = C1 מתקדם
pv-language-proficiency-c2 = C2 שליטה מלאה
pv-language-proficiency-native = שפה ראשונה
pv-isced-level-isced-0 = 0 גיל הרך
pv-isced-level-isced-1 = 1 יסודי
pv-isced-level-isced-2 = 2 חטיבת ביניים
pv-isced-level-isced-3 = 3 חטיבה עליונה
pv-isced-level-isced-4 = 4 על-תיכוני לא אקדמי
pv-isced-level-isced-5 = 5 על-תיכוני קצר
pv-isced-level-isced-6 = 6 תואר ראשון או מקביל
pv-isced-level-isced-7 = 7 תואר שני או מקביל
pv-isced-level-isced-8 = 8 דוקטורט או מקביל
pv-income-quintile-q1 = החמישון התחתון
pv-income-quintile-q2 = החמישון השני
pv-income-quintile-q3 = החמישון האמצעי
pv-income-quintile-q4 = החמישון הרביעי
pv-income-quintile-q5 = החמישון העליון
pv-pay-period-hourly = לשעה
pv-pay-period-daily = ליום
pv-pay-period-weekly = לשבוע
pv-pay-period-monthly = לחודש
pv-pay-period-annual = לשנה
pv-tenure-owned = בבעלות
pv-tenure-co-owned = בבעלות משותפת
pv-tenure-leasehold = חכירה
pv-tenure-rented = בשכירות
pv-tenure-usufruct = זכות שימוש
pv-tenure-other = אחר
pv-distinction-kind-order = מסדר
pv-distinction-kind-decoration = עיטור
pv-distinction-kind-medal = מדליה
pv-distinction-kind-title = תואר כבוד
pv-distinction-kind-other = אחר
pv-military-service-army = יבשה
pv-military-service-navy = ים
pv-military-service-air-force = אוויר
pv-military-service-marines = נחתים
pv-military-service-gendarmerie = ז׳נדרמריה
pv-military-service-border-guard = משמר הגבול
pv-military-service-national-guard = משמר לאומי
pv-military-service-other = אחר
pv-rank-category-enlisted = חוגר
pv-rank-category-non-commissioned = נגד
pv-rank-category-warrant = נגד בכיר
pv-rank-category-officer-cadet = צוער
pv-rank-category-junior-officer = קצין זוטר
pv-rank-category-senior-officer = קצין בכיר
pv-rank-category-general-officer = קצין בדרגת אלוף
pv-iccs-section-acts-leading-to-death = 01 מעשים שגרמו למוות
pv-iccs-section-acts-causing-harm = 02 מעשים שגרמו לפגיעה
pv-iccs-section-sexual-acts = 03 מעשים פוגעניים בעלי אופי מיני
pv-iccs-section-property-with-violence = 04 נגד רכוש, באלימות
pv-iccs-section-property-only = 05 נגד רכוש בלבד
pv-iccs-section-controlled-substances = 06 חומרים מפוקחים
pv-iccs-section-fraud-deception-corruption = 07 מרמה, הונאה או שחיתות
pv-iccs-section-public-order-and-state = 08 נגד הסדר הציבורי והמדינה
pv-iccs-section-public-safety-and-security = 09 נגד שלום הציבור וביטחונו
pv-iccs-section-natural-environment = 10 נגד הסביבה הטבעית
pv-iccs-section-other-criminal-acts = 11 מעשים פליליים אחרים
pv-case-outcome-convicted = הורשע
pv-case-outcome-acquitted = זוכה
pv-case-outcome-dismissed = התיק נסגר
pv-case-outcome-conviction-quashed = ההרשעה בוטלה
pv-case-outcome-pardoned = קיבל חנינה
pv-case-outcome-amnestied = נכלל בחנינה כללית
pv-case-outcome-expunged = נמחק מהרישום
pv-case-outcome-pending = תלוי ועומד
pv-case-outcome-unknown = לא ידוע
pv-religion-buddhism = בודהיזם
pv-religion-christianity-catholic = נצרות: קתולית
pv-religion-christianity-orthodox = נצרות: אורתודוקסית
pv-religion-christianity-protestant = נצרות: פרוטסטנטית
pv-religion-christianity-other = נצרות: אחר
pv-religion-hinduism = הינדואיזם
pv-religion-islam-sunni = אסלאם: סוני
pv-religion-islam-shia = אסלאם: שיעי
pv-religion-islam-other = אסלאם: אחר
pv-religion-jainism = ג׳ייניזם
pv-religion-judaism = יהדות
pv-religion-sikhism = סיקיזם
pv-religion-bahai = האמונה הבהאית
pv-religion-shinto = שינטו
pv-religion-taoism = טאואיזם
pv-religion-zoroastrianism = זורואסטריות
pv-religion-traditional = דת מסורתית או עממית
pv-religion-other = אחר
pv-religion-none = ללא דת
pv-religion-unknown = לא ידוע
pv-sacrament-baptism = טבילה
pv-sacrament-confirmation = קונפירמציה
pv-sacrament-first-communion = קומוניון ראשון
pv-sacrament-reconciliation = וידוי
pv-sacrament-anointing-of-the-sick = משיחת חולים
pv-sacrament-holy-orders = הסמכה לכהונה
pv-sacrament-matrimony = נישואין
pv-sacrament-other-rite = טקס אחר
pv-political-position-far-left = שמאל קיצוני
pv-political-position-left = שמאל
pv-political-position-centre-left = מרכז-שמאל
pv-political-position-centre = מרכז
pv-political-position-centre-right = מרכז-ימין
pv-political-position-right = ימין
pv-political-position-far-right = ימין קיצוני
pv-political-position-apolitical = א-פוליטי
pv-political-position-other = מחוץ לציר הזה
pv-political-position-unknown = לא ידוע
pv-membership-kind-trade-union = איגוד מקצועי
pv-membership-kind-political-party = מפלגה פוליטית
pv-membership-kind-professional-body = ארגון מקצועי
pv-membership-kind-religious-order = מסדר דתי
pv-membership-kind-religious-association = אגודה דתית
pv-membership-kind-fraternal-order = מסדר אחווה
pv-membership-kind-veterans-association = ארגון יוצאי צבא
pv-membership-kind-sports-club = מועדון ספורט
pv-membership-kind-cultural-association = אגודה תרבותית
pv-membership-kind-charitable-association = אגודת צדקה
pv-membership-kind-other = אחר
pv-personality-instrument-neo-pi-3 = NEO-PI-3
pv-personality-instrument-neo-ffi-3 = NEO-FFI-3
pv-personality-instrument-bfi-2 = BFI-2
pv-personality-instrument-ipip-neo-120 = IPIP-NEO-120
pv-personality-instrument-tipi = TIPI
pv-personality-instrument-hexaco-pi-r = HEXACO-PI-R
pv-personality-instrument-observer-rating = דורג על ידי מי שהכיר אותו
pv-personality-instrument-inferred = הוסק מתוך הרשומות
pv-personality-instrument-other = אחר
pv-introversion-extraversion-strongly-introverted = מופנם מאוד
pv-introversion-extraversion-introverted = מופנם
pv-introversion-extraversion-ambiverted = מאוזן
pv-introversion-extraversion-extraverted = מוחצן
pv-introversion-extraversion-strongly-extraverted = מוחצן מאוד
pv-stress-tolerance-very-low = נמוכה מאוד
pv-stress-tolerance-low = נמוכה
pv-stress-tolerance-moderate = בינונית
pv-stress-tolerance-high = גבוהה
pv-stress-tolerance-very-high = גבוהה מאוד
pv-decision-style-rational = רציונלי
pv-decision-style-intuitive = אינטואיטיבי
pv-decision-style-dependent = תלותי
pv-decision-style-avoidant = נמנע
pv-decision-style-spontaneous = ספונטני
pv-sport-level-recreational = פנאי
pv-sport-level-amateur-competitive = חובב תחרותי
pv-sport-level-semi-professional = חצי-מקצועי
pv-sport-level-professional = מקצועי
pv-diet-omnivore = אוכל-כל
pv-diet-flexitarian = פלקסיטריאני
pv-diet-pescatarian = פסקטריאני
pv-diet-vegetarian = צמחוני
pv-diet-vegan = טבעוני
pv-diet-other = אחר

pv-substance-tobacco = טבק וניקוטין
pv-substance-alcohol = אלכוהול
pv-substance-cannabis = קנאביס
pv-substance-opioids = אופיואידים
pv-substance-stimulants = ממריצים
pv-substance-sedatives-hypnotics = תרופות הרגעה ושינה
pv-substance-hallucinogens = חומרים הזייתיים
pv-substance-inhalants = חומרים נשאפים
pv-substance-gambling = הימורים
pv-substance-gaming = משחקי מחשב
pv-substance-other = אחר
pv-use-pattern-occasional-use = שימוש מזדמן
pv-use-pattern-regular-use = שימוש קבוע
pv-use-pattern-harmful-use = שימוש מזיק
pv-use-pattern-dependence = תלות
pv-use-pattern-in-remission = בהפוגה
pv-lineage-biological = ביולוגי
pv-lineage-adoptive = מאמץ
pv-lineage-foster = אומנה
pv-lineage-step = חורג
pv-lineage-guardianship = אפוטרופסות
pv-lineage-unknown = לא ידוע
pv-link-relation-godparent = סנדק
pv-link-relation-godchild = בן סנדקות
pv-link-relation-witness = עד
pv-link-relation-officiant = עורך הטקס
pv-link-relation-business-partner = שותף עסקי
pv-link-relation-employer = מעסיק
pv-link-relation-employee = עובד
pv-link-relation-mentor = מנטור
pv-link-relation-apprentice = שוליה
pv-link-relation-close-friend = חבר קרוב
pv-link-relation-neighbour = שכן
pv-link-relation-guardian = אפוטרופוס
pv-link-relation-ward = חסוי
pv-link-relation-other = אחר
pv-country-AD = אנדורה
pv-country-AE = איחוד האמירויות הערביות
pv-country-AF = אפגניסטן
pv-country-AG = אנטיגואה וברבודה
pv-country-AI = אנגווילה
pv-country-AL = אלבניה
pv-country-AM = ארמניה
pv-country-AO = אנגולה
pv-country-AQ = אנטארקטיקה
pv-country-AR = ארגנטינה
pv-country-AS = סמואה האמריקנית
pv-country-AT = אוסטריה
pv-country-AU = אוסטרליה
pv-country-AW = ארובה
pv-country-AX = איי אולנד
pv-country-AZ = אזרבייג׳ן
pv-country-BA = בוסניה והרצגובינה
pv-country-BB = ברבדוס
pv-country-BD = בנגלדש
pv-country-BE = בלגיה
pv-country-BF = בורקינה פאסו
pv-country-BG = בולגריה
pv-country-BH = בחריין
pv-country-BI = בורונדי
pv-country-BJ = בנין
pv-country-BL = סנט ברתולומיאו
pv-country-BM = ברמודה
pv-country-BN = ברוניי
pv-country-BO = בוליביה
pv-country-BQ = האיים הקריביים ההולנדיים
pv-country-BR = ברזיל
pv-country-BS = איי בהאמה
pv-country-BT = בהוטן
pv-country-BV = האי בובה
pv-country-BW = בוצוואנה
pv-country-BY = בלארוס
pv-country-BZ = בליז
pv-country-CA = קנדה
pv-country-CC = איי קוקוס (קילינג)
pv-country-CD = קונגו - קינשאסה
pv-country-CF = הרפובליקה המרכז-אפריקאית
pv-country-CG = קונגו - ברזאויל
pv-country-CH = שווייץ
pv-country-CI = חוף השנהב
pv-country-CK = איי קוק
pv-country-CL = צ׳ילה
pv-country-CM = קמרון
pv-country-CN = סין
pv-country-CO = קולומביה
pv-country-CR = קוסטה ריקה
pv-country-CU = קובה
pv-country-CV = כף ורדה
pv-country-CW = קוראסאו
pv-country-CX = אי חג המולד
pv-country-CY = קפריסין
pv-country-CZ = צ׳כיה
pv-country-DE = גרמניה
pv-country-DJ = ג׳יבוטי
pv-country-DK = דנמרק
pv-country-DM = דומיניקה
pv-country-DO = הרפובליקה הדומיניקנית
pv-country-DZ = אלג׳יריה
pv-country-EC = אקוודור
pv-country-EE = אסטוניה
pv-country-EG = מצרים
pv-country-EH = סהרה המערבית
pv-country-ER = אריתריאה
pv-country-ES = ספרד
pv-country-ET = אתיופיה
pv-country-FI = פינלנד
pv-country-FJ = פיג׳י
pv-country-FK = איי פוקלנד
pv-country-FM = מיקרונזיה
pv-country-FO = איי פארו
pv-country-FR = צרפת
pv-country-GA = גבון
pv-country-GB = בריטניה
pv-country-GD = גרנדה
pv-country-GE = גאורגיה
pv-country-GF = גיאנה הצרפתית
pv-country-GG = גרנזי
pv-country-GH = גאנה
pv-country-GI = גיברלטר
pv-country-GL = גרינלנד
pv-country-GM = גמביה
pv-country-GN = גינאה
pv-country-GP = גוואדלופ
pv-country-GQ = גינאה המשוונית
pv-country-GR = יוון
pv-country-GS = ג׳ורג׳יה הדרומית ואיי סנדוויץ׳ הדרומיים
pv-country-GT = גואטמלה
pv-country-GU = גואם
pv-country-GW = גינאה-ביסאו
pv-country-GY = גיאנה
pv-country-HK = הונג קונג (אזור מנהלי מיוחד של סין)
pv-country-HM = איי הרד ומקדונלד
pv-country-HN = הונדורס
pv-country-HR = קרואטיה
pv-country-HT = האיטי
pv-country-HU = הונגריה
pv-country-ID = אינדונזיה
pv-country-IE = אירלנד
pv-country-IL = ישראל
pv-country-IM = האי מאן
pv-country-IN = הודו
pv-country-IO = הטריטוריה הבריטית באוקיינוס ההודי
pv-country-IQ = עיראק
pv-country-IR = איראן
pv-country-IS = איסלנד
pv-country-IT = איטליה
pv-country-JE = ג׳רזי
pv-country-JM = ג׳מייקה
pv-country-JO = ירדן
pv-country-JP = יפן
pv-country-KE = קניה
pv-country-KG = קירגיזסטן
pv-country-KH = קמבודיה
pv-country-KI = קיריבאטי
pv-country-KM = קומורו
pv-country-KN = סנט קיטס ונוויס
pv-country-KP = קוריאה הצפונית
pv-country-KR = קוריאה הדרומית
pv-country-KW = כווית
pv-country-KY = איי קיימן
pv-country-KZ = קזחסטן
pv-country-LA = לאוס
pv-country-LB = לבנון
pv-country-LC = סנט לוסיה
pv-country-LI = ליכטנשטיין
pv-country-LK = סרי לנקה
pv-country-LR = ליבריה
pv-country-LS = לסוטו
pv-country-LT = ליטא
pv-country-LU = לוקסמבורג
pv-country-LV = לטביה
pv-country-LY = לוב
pv-country-MA = מרוקו
pv-country-MC = מונקו
pv-country-MD = מולדובה
pv-country-ME = מונטנגרו
pv-country-MF = סן מרטן
pv-country-MG = מדגסקר
pv-country-MH = איי מרשל
pv-country-MK = מקדוניה הצפונית
pv-country-ML = מאלי
pv-country-MM = מיאנמר (בורמה)
pv-country-MN = מונגוליה
pv-country-MO = מקאו (אזור מנהלי מיוחד של סין)
pv-country-MP = איי מריאנה הצפוניים
pv-country-MQ = מרטיניק
pv-country-MR = מאוריטניה
pv-country-MS = מונסראט
pv-country-MT = מלטה
pv-country-MU = מאוריציוס
pv-country-MV = האיים המלדיביים
pv-country-MW = מלאווי
pv-country-MX = מקסיקו
pv-country-MY = מלזיה
pv-country-MZ = מוזמביק
pv-country-NA = נמיביה
pv-country-NC = קלדוניה החדשה
pv-country-NE = ניז׳ר
pv-country-NF = האי נורפוק
pv-country-NG = ניגריה
pv-country-NI = ניקרגואה
pv-country-NL = הולנד
pv-country-NO = נורווגיה
pv-country-NP = נפאל
pv-country-NR = נאורו
pv-country-NU = ניווה
pv-country-NZ = ניו זילנד
pv-country-OM = עומאן
pv-country-PA = פנמה
pv-country-PE = פרו
pv-country-PF = פולינזיה הצרפתית
pv-country-PG = פפואה גינאה החדשה
pv-country-PH = הפיליפינים
pv-country-PK = פקיסטן
pv-country-PL = פולין
pv-country-PM = סנט פייר ומיקלון
pv-country-PN = איי פיטקרן
pv-country-PR = פוארטו ריקו
pv-country-PS = השטחים הפלסטיניים
pv-country-PT = פורטוגל
pv-country-PW = פלאו
pv-country-PY = פרגוואי
pv-country-QA = קטאר
pv-country-RE = ראוניון
pv-country-RO = רומניה
pv-country-RS = סרביה
pv-country-RU = רוסיה
pv-country-RW = רואנדה
pv-country-SA = ערב הסעודית
pv-country-SB = איי שלמה
pv-country-SC = איי סיישל
pv-country-SD = סודן
pv-country-SE = שוודיה
pv-country-SG = סינגפור
pv-country-SH = סנט הלנה
pv-country-SI = סלובניה
pv-country-SJ = סבאלברד ויאן מאיין
pv-country-SK = סלובקיה
pv-country-SL = סיירה לאונה
pv-country-SM = סן מרינו
pv-country-SN = סנגל
pv-country-SO = סומליה
pv-country-SR = סורינאם
pv-country-SS = דרום סודן
pv-country-ST = סאו טומה ופרינסיפה
pv-country-SV = אל סלבדור
pv-country-SX = סנט מארטן
pv-country-SY = סוריה
pv-country-SZ = אסוואטיני
pv-country-TC = איי טרקס וקייקוס
pv-country-TD = צ׳אד
pv-country-TF = הטריטוריות הדרומיות של צרפת
pv-country-TG = טוגו
pv-country-TH = תאילנד
pv-country-TJ = טג׳יקיסטן
pv-country-TK = טוקלאו
pv-country-TL = טימור-לסטה
pv-country-TM = טורקמניסטן
pv-country-TN = תוניסיה
pv-country-TO = טונגה
pv-country-TR = טורקיה
pv-country-TT = טרינידד וטובגו
pv-country-TV = טובאלו
pv-country-TW = טייוואן
pv-country-TZ = טנזניה
pv-country-UA = אוקראינה
pv-country-UG = אוגנדה
pv-country-UM = האיים המרוחקים הקטנים של ארה״ב
pv-country-US = ארצות הברית
pv-country-UY = אורוגוואי
pv-country-UZ = אוזבקיסטן
pv-country-VA = הוותיקן
pv-country-VC = סנט וינסנט והגרנדינים
pv-country-VE = ונצואלה
pv-country-VG = איי הבתולה הבריטיים
pv-country-VI = איי הבתולה של ארצות הברית
pv-country-VN = וייטנאם
pv-country-VU = ונואטו
pv-country-WF = איי ווליס ופוטונה
pv-country-WS = סמואה
pv-country-YE = תימן
pv-country-YT = מאיוט
pv-country-ZA = דרום אפריקה
pv-country-ZM = זמביה
pv-country-ZW = זימבבואה
pv-country-SU = ברית המועצות
pv-country-DD = מזרח גרמניה
pv-country-YU = יוגוסלביה
pv-country-CS = צ׳כוסלובקיה
pv-country-OT = האימפריה העות׳מאנית
lang-aa = אפארית
lang-ab = אבחזית
lang-ae = אבסטן
lang-af = אפריקאנס
lang-ak = אקאן
lang-am = אמהרית
lang-an = אראגונית
lang-ar = ערבית
lang-as = אסאמית
lang-av = אווארית

lang-ay = איימארית
lang-az = אזרית
lang-ba = בשקירית
lang-be = בלארוסית
lang-bg = בולגרית
lang-bi = ביסלמה
lang-bm = במבארה
lang-bn = בנגלית
lang-bo = טיבטית
lang-br = ברטונית
lang-bs = בוסנית
lang-ca = קטלאנית
lang-ce = צ׳צ׳נית
lang-ch = צ׳מורו
lang-co = קורסיקנית
lang-cr = קרי
lang-cs = צ׳כית
lang-cu = סלאבית כנסייתית עתיקה
lang-cv = צ׳ובאש
lang-cy = וולשית
lang-da = דנית
lang-de = גרמנית
lang-dv = דיבהי
lang-dz = דזונקה
lang-ee = אווה
lang-el = יוונית
lang-en = אנגלית
lang-eo = אספרנטו
lang-es = ספרדית
lang-et = אסטונית
lang-eu = בסקית
lang-fa = פרסית
lang-ff = פולה
lang-fi = פינית
lang-fj = פיג׳ית
lang-fo = פארואזית
lang-fr = צרפתית
lang-fy = פריזית מערבית
lang-ga = אירית
lang-gd = גאלית סקוטית
lang-gl = גליציאנית
lang-gn = גוארני
lang-gu = גוג׳ארטי
lang-gv = מאנית
lang-ha = האוסה
lang-he = עברית
lang-hi = הינדי
lang-ho = הירי מוטו
lang-hr = קרואטית
lang-ht = קריאולית (האיטי)
lang-hu = הונגרית
lang-hy = ארמנית
lang-hz = הררו
lang-ia = ‏אינטרלינגואה
lang-id = אינדונזית
lang-ie = אינטרלינגה
lang-ig = איגבו
lang-ii = סצ׳ואן יי
lang-ik = אינופיאק
lang-io = אידו
lang-is = איסלנדית
lang-it = איטלקית
lang-iu = אינוקטיטוט
lang-ja = יפנית
lang-jv = יאוואית
lang-ka = גאורגית
lang-kg = קונגו
lang-ki = קיקויו
lang-kj = קואניאמה
lang-kk = קזחית
lang-kl = גרינלנדית
lang-km = חמרית
lang-kn = קנאדה
lang-ko = קוריאנית
lang-kr = קאנורי
lang-ks = קשמירית
lang-ku = כורדית
lang-kv = קומי
lang-kw = קורנית
lang-ky = קירגיזית
lang-la = לטינית
lang-lb = לוקסמבורגית
lang-lg = גאנדה
lang-li = לימבורגית
lang-ln = לינגלה
lang-lo = לאו
lang-lt = ליטאית
lang-lu = לובה-קטנגה
lang-lv = לטבית
lang-mg = מלגשית
lang-mh = מרשלית
lang-mi = מאורית
lang-mk = מקדונית
lang-ml = מליאלאם
lang-mn = מונגולית
lang-mr = מראטהי
lang-ms = מלאית
lang-mt = מלטית
lang-my = בורמזית
lang-na = נאורית
lang-nb = נורווגית ספרותית
lang-nd = נדבלה צפונית
lang-ne = נפאלית
lang-ng = נדונגה
lang-nl = הולנדית
lang-nn = נורווגית חדשה
lang-no = נורווגית
lang-nr = נדבלה דרומית
lang-nv = נאוואחו
lang-ny = ניאנג׳ה
lang-oc = אוקסיטנית
lang-oj = אוג׳יבווה
lang-om = אורומו
lang-or = אורייה
lang-os = אוסטית
lang-pa = פנג׳אבי
lang-pi = פאלי
lang-pl = פולנית
lang-ps = פאשטו
lang-pt = פורטוגזית
lang-qu = קצ׳ואה
lang-rm = רומאנש
lang-rn = קירונדי
lang-ro = רומנית
lang-ru = רוסית
lang-rw = קנירואנדית
lang-sa = סנסקריט
lang-sc = סרדינית
lang-sd = סינדהית
lang-se = סמי צפונית
lang-sg = סנגו
lang-sh = סרבו-קרואטית
lang-si = סינהלה
lang-sk = סלובקית
lang-sl = סלובנית
lang-sm = סמואית
lang-sn = שונה
lang-so = סומלית
lang-sq = אלבנית
lang-sr = סרבית
lang-ss = סאווזי
lang-st = סותו דרומית
lang-su = סונדנזית
lang-sv = שוודית
lang-sw = סווהילי
lang-ta = טמילית
lang-te = טלוגו
lang-tg = טג׳יקית
lang-th = תאית
lang-ti = תיגרינית
lang-tk = טורקמנית
lang-tl = טאגאלוג
lang-tn = סוואנה
lang-to = טונגאית
lang-tr = טורקית
lang-ts = טסונגה
lang-tt = טטרית
lang-tw = טווי
lang-ty = טהיטית
lang-ug = אויגורית
lang-uk = אוקראינית
lang-ur = אורדו
lang-uz = אוזבקית
lang-ve = וונדה
lang-vi = וייטנאמית
lang-vo = ‏וולאפיק
lang-wa = ולונית
lang-wo = וולוף
lang-xh = קוסה
lang-yi = יידיש
lang-yo = יורובה
lang-za = זואנג
lang-zh = סינית
lang-zu = זולו
person-tab-profile = פרופיל
profile-groups-label = קבוצות הפרופיל
profile-group-withheld = חלק מהקבוצה הזאת מוסתר מכם
profile-withheld = נרשם לאדם הזה ומוסתר מכם: { $classes }.
profile-empty = עדיין לא נרשם דבר בקבוצה הזאת.
profile-earlier = צורה קודמת
profile-earlier-title = נרשם על ידי גרסה קודמת של היישום הזה, בשדה שאין לו מקום ב-AXGF 1.1. הוא נשמר כפי שנכתב.
profile-other-names = { $n ->
        [one] ועוד שם אחד
        [two] ועוד שני שמות
        [many] ועוד { $n } שמות
       *[other] ועוד { $n } שמות
    }
profile-edit-group = עריכת { $group }
profile-summary-link = { $n ->
        [one] עובדה אחת בפרופיל
        [two] שתי עובדות בפרופיל
        [many] { $n } עובדות בפרופיל
       *[other] { $n } עובדות בפרופיל
    }
profile-from = מ-
profile-until = עד
profile-yes = כן
profile-no = לא
profile-value = ערך
profile-editor-title = פרופיל
profile-group-filled = { $n } מולאו
profile-problems = חלק ממה שהוזן לא נשמר. כל בעיה מצוינת ליד השדה שלה, ושום דבר לא נכתב.
profile-editor-withheld = הקבוצה הזאת מכילה לאדם הזה גם { $classes }, שאינכם רשאים לקרוא. זה לא מוצג כאן, ושמירת הטופס משאירה זאת בדיוק כפי שהוא.
profile-living-class-note = האדם הזה רשום כחי. מה שתזינו כאן בקטגוריה רגישה מוצג למנהלים בלבד.
profile-relationships-elsewhere = הורים, בני זוג, ילדים, סנדקים ועדים אינם נשמרים על האדם הזה. הם משפחות, קשרים ואירועים שמזכירים אותו — כך שכל שינוי כאן משנה גם את הרשומה של כל מי שנמצא בהם.
profile-documents-first = תוצר מפנה למסמך שמצורף לאדם הזה. צרפו קודם את הקובץ.
profile-editor-nothing = אין בקבוצה הזאת דבר שאתם רשאים לערוך.
profile-new-entry = רשומה חדשה
profile-provenance = תאריך, מקור ורמת ודאות
profile-from-date = נכון מ-
profile-until-date = נכון עד
profile-remove-entry = הסרת הרשומה הזאת
profile-add-entry = הוספת רשומה נוספת
profile-no-such-group-title = אין קבוצה כזאת
profile-no-such-group-detail = לפרופיל אין קבוצה בשם הזה.
profile-error-number = ערך כאן הוא מספר.
profile-error-integer = ערך כאן הוא מספר שלם.
profile-error-range = מספר נמצא מחוץ לטווח שהמאפיין הזה מתיר.
profile-error-term = ערך אינו אחת מהאפשרויות שברשימה.
profile-error-required = לרשומה חסר שדה שהיא צריכה.
profile-error-one-of = רשומה צריכה שלפחות אחד מהשדות העיקריים שלה יהיה מלא.
profile-error-confidence = רמת הוודאות נעה בין 0 ל-1, למשל 0.8.
profile-error-time = שעה נכתבת כשעות ודקות, למשל 05:40.
profile-error-currency = מטבע הוא הקוד בן שלוש האותיות שלו, למשל ILS.
profile-error-language = שפה היא הקוד שלה, למשל he או zh-Hans.
profile-error-coordinates = קואורדינטות צריכות גם קו רוחב בין ‎−90 ל-90 וגם קו אורך בין ‎−180 ל-180.
profile-error-rank-country = הדרגה שייכת למדינה אחרת מזו שנבחרה.
record-unknown-place = [מקום לא ידוע]
record-missing-document = [מסמך חסר]
confidence-certain = ודאות { $percent }% — כמעט בטוח
confidence-high = ודאות { $percent }% — מבוסס היטב
confidence-medium = ודאות { $percent }% — סביר אך לא מאומת
confidence-low = ודאות { $percent }% — השערה
tree-edge-union-between = { $from } ו{ $to } — { $confidence }
tree-edge-parentage-of = { $from }, הורה של { $to } — { $confidence }
record-note-biography = ביוגרפיה
record-note-birth-date-as-recorded = תאריך הלידה, כפי שנרשם
record-note-death-date-as-recorded = תאריך הפטירה, כפי שנרשם
record-note-event-date-as-recorded = תאריך { $event }, כפי שנרשם
record-unknown-source = [מקור לא ידוע]
record-untitled-source = [מקור ללא כותרת]
record-unnamed = [ללא שם]
record-untitled = [ללא כותרת]
record-period-from = מ-{ $date }
record-period-until = עד { $date }
record-dates-unrecorded = תאריכים לא נרשמו
record-link-unlabelled = קשור אל
record-link-reverse = { $label } (של)
record-place-worked-as = עבד/ה כ{ $title }
record-place-married-to = נישא/ה ל{ $name }
record-place-married = נישאו
record-source-use-name = השם „{ $name }”
record-source-use-working-as = עבודה כ{ $title }
record-source-use-union-with = הקשר הזוגי עם { $name }
record-source-use-union = הקשר הזוגי
record-lifespan-born = נ. { $year }
record-lifespan-died = נפ׳ { $year }
size-bytes = { $n ->
        [one] בית אחד
        [two] { $n } בתים
        [many] { $n } בתים
       *[other] { $n } בתים
    }
size-kb = { $n } KB
size-mb = { $n } MB
size-gb = { $n } GB
calendar-gregorian = גרגוריאני
calendar-julian = יוליאני
calendar-hebrew = עברי
calendar-hijri = הג׳רי
calendar-persian = פרסי
calendar-chinese = סיני
calendar-ethiopian = אתיופי
calendar-japanese_era = תקופות יפניות
calendar-republican_french = הרפובליקני הצרפתי
calendar-roman = רומי
diff-summary-none = אף שדה לא השתנה
diff-summary-one = שונה { $a }
diff-summary-two = שונו { $a } ו{ $b }
diff-summary-many = שונו { $a }, { $b } ו{ $n ->
        [one] עוד אחד
        [two] עוד שניים
        [many] עוד { $n }
       *[other] עוד { $n }
    }
diff-saved-none = אף שדה לא השתנה
diff-saved-one = שונה { $a }
diff-saved-two = שונו { $a } ו{ $b }
diff-saved-many = שונו { $a }, { $b } ו{ $n ->
        [one] עוד אחד
        [two] עוד שניים
        [many] עוד { $n }
       *[other] עוד { $n }
    }
diff-section-other = שדה אחר
diff-section-avatar = תמונת דיוקן
diff-section-partners = בני זוג
diff-section-ai = סיוע בינה מלאכותית
diff-section-birth = לידה
diff-section-civil-status = מעמד אזרחי
diff-section-documents = מסמכים מצורפים
diff-section-extensions = נתוני היישום
diff-section-children = ילדים
diff-section-union = קשר זוגי
diff-section-date = תאריך
diff-section-participants = משתתפים
diff-section-from = מקושר מ-

diff-section-to = מקושר אל
diff-section-valid-from = בתוקף מ-
diff-section-valid-until = בתוקף עד
diff-section-visibility = נראות
diff-section-employer = מעסיק
diff-section-title-normalized = תואר מתוקנן
diff-section-conflicts = מקורות סותרים
diff-section-dna = DNA
diff-section-document = מסמך
diff-section-language = שפה
diff-section-place = מקום
diff-section-repository = מאגר
diff-section-script = כתב
diff-section-coordinates = קואורדינטות
diff-section-country-history = היסטוריית המדינה
diff-section-identifiers = מזהים
diff-section-names = שמות
diff-section-file = קובץ
diff-section-linked-to = מצורף אל
diff-section-ocr = טקסט מזוהה
diff-section-format-version = גרסת התבנית
history-created = נוצר
history-deleted = נמחק
history-merged = מוזג לרשומה אחרת
history-setup = הקים את הארכיון
history-attached = צירף קובץ
admin-raw-json-unparsed = ה-JSON הגולמי לא נותח ({ $error }). שום דבר לא נשמר.
admin-raw-json-dangling = ה-JSON הגולמי מפנה את { $field } אל { $target }, שאינו בארכיון הזה. בחרו אותו בשדה שלמעלה, או תקנו את המזהה. שום דבר לא נשמר.
conflict-someone = מישהו
conflict-unrecorded-time = בזמן שאיש לא רשם
dedup-merged-persons = { $n ->
        [one] אדם אחד מוזג
        [two] שני אנשים מוזגו
        [many] { $n } אנשים מוזגו
       *[other] { $n } אנשים מוזגו
    }
dedup-merged-families = { $n ->
        [one] משפחה אחת מוזגה
        [two] שתי משפחות מוזגו
        [many] { $n } משפחות מוזגו
       *[other] { $n } משפחות מוזגו
    }
dedup-manual-review = { $n ->
        [one] מקרה אחד נותר לבדיקה של אדם
        [two] שני מקרים נותרו לבדיקה של אדם
        [many] { $n } מקרים נותרו לבדיקה של אדם
       *[other] { $n } מקרים נותרו לבדיקה של אדם
    }
dedup-nothing = אין מה לדווח.
record-union-duplicate = זוג אחד, יותר מרשומה אחת.
record-union-duplicate-detail = החבילה מחזיקה רשומות משפחה נפרדות לשני האנשים האלה. זה פגם בנתונים, לא קשר זוגי שני.
record-union-duplicate-action = מיזוג כפילויות
record-union-duplicate-confirm = להסיר כפילויות בכל החבילה? כל זוג שהספרייה יכולה למזג ימוזג; השאר ידווחו.
dedup-pair-merged = הזוג ששאלתם עליו הוא עכשיו רשומה אחת.
dedup-pair-refused = הזוג ששאלתם עליו לא מוזג: הספרייה סירבה והשאירה אותו לבדיקה של אדם.
validate-errors = { $n ->
        [one] שגיאה אחת
        [two] שתי שגיאות
        [many] { $n } שגיאות
       *[other] { $n } שגיאות
    }
validate-warnings = { $n ->
        [one] אזהרה אחת
        [two] שתי אזהרות
        [many] { $n } אזהרות
       *[other] { $n } אזהרות
    }
validate-notes = { $n ->
        [one] הערה אחת
        [two] שתי הערות
        [many] { $n } הערות
       *[other] { $n } הערות
    }
validate-nothing = אין מה לדווח.
list-separator = { ", " }
result-written = הארכיון נכתב לדיסק.
result-refused = הספרייה סירבה לפעולה הזאת. הארכיון שבדיסק לא השתנה.
convert-error-no-file = לא הועלה קובץ. בחרו קודם קובץ ‎.ged.
convert-error-file-too-large = גודל הקובץ { $size } MB והמגבלה היא { $limit } MB. שום דבר לא הומר.
convert-error-too-large = ההעלאה גדולה ממגבלת { $limit } MB. שום דבר לא הומר.
convert-error-unreadable = לא ניתן היה לקרוא את ההעלאה ({ $error }). שום דבר לא הומר.
convert-error-not-gedcom = זה לא נראה כמו קובץ GEDCOM: קובץ GEDCOM 5.5.1 מתחיל בשורה „0 HEAD”. שום דבר לא הומר.
gedcom-read-title = איך הקובץ נקרא
gedcom-read-as = נקרא כ-GEDCOM { $version }, בקידוד { $encoding }.
gedcom-read-as-unversioned = נקרא כ-GEDCOM (הכותרת שלו אינה מציינת גרסה), בקידוד { $encoding }.
gedcom-read-declared = הכותרת שלו אומרת { $declared }; הבתים שלו אומרים אחרת, והם נקראו כפי שהם.
gedcom-read-gedzip = ארכיון GEDCOM 7 ‏(.gdz): מערך הנתונים שלו נקרא. קבצים אחרים שבו (תצלומים, סריקות) לא הועברו: { $n }.
gedcom-read-crs = שורות שמסתיימות בדרך של מקינטוש הישן, תוקנו: { $n }.
gedcom-read-voids = הפניות ל-@VOID@ — „אין רשומה” של GEDCOM 7 — הונחו בצד במקום להיקרא כאנשים: { $n }.
gedcom-read-notes = הערות משותפות (SNOTE) נקראו כהערות: { $n }.
gedcom-read-stray = בתים שלא היו UTF-8 בקובץ ה-UTF-8 הזה, כל אחד נקרא כ-Windows-1252: { $n }.
gedcom-read-tidied = שורות שסודרו בדרך שהממיר אינו קורא (מוזחות, טאבים או רווחים מיותרים בין שדות, תג באותיות קטנות, רווח לפני חיבור CONC, תו בקרה, סימון של לוח גרגוריאני), תוקנו: { $n }.
gedcom-read-too-deep = שורות מקוננות עמוק יותר מכל מבנה GEDCOM, הונחו בצד עם מה שהכילו: { $n }.
convert-error-no-dataset = בקובץ ה-zip הזה אין gedcom.ged, וגם לא קובץ ‎.ged יחיד לקרוא במקומו.
convert-error-refused = הממיר סירב לקובץ הזה. האבחונים שלמטה מסבירים למה. שום דבר לא נשמר.
convert-error-packaging = הקובץ הומר אך לא ניתן היה לארוז אותו ({ $error }).
completeness-fraction = { $part } מתוך { $whole }
event-category-adoption = אימוץ
event-category-migration = הגירה
event-category-naturalization = התאזרחות
event-category-incarceration = מאסר
event-category-name_change = שינוי שם
event-category-legal = עניין משפטי
event-category-religious = אירוע דתי
event-category-social = אירוע חברתי
event-category-historical = אירוע היסטורי
precision-quarter_century = עד רבע המאה
source-type-birth_certificate = תעודת לידה
source-type-death_certificate = תעודת פטירה
source-type-marriage_certificate = תעודת נישואין
source-type-census = מפקד אוכלוסין
source-type-baptism_record = רישום טבילה
source-type-burial_record = רישום קבורה
source-type-will = צוואה
source-type-land_record = רישום מקרקעין
source-type-military_record = רשומה צבאית
source-type-immigration_record = רישום הגירה
source-type-naturalization = רישום התאזרחות
source-type-passport = דרכון
source-type-photograph = תצלום
source-type-letter = מכתב
source-type-diary = יומן
source-type-newspaper = עיתון
source-type-oral_tradition = מסורת בעל פה
source-type-dna = בדיקת DNA
source-type-family_bible = תנ״ך משפחתי
source-type-gravestone = מצבה
source-type-published_genealogy = גנאלוגיה שפורסמה
source-type-other = מקור אחר
source-status-verified = נבדק מול המקור
source-status-unverified = עדיין לא נבדק
source-status-lost = אבד
source-status-known_missing = ידוע כחסר
document-type-birth_certificate = תעודת לידה
document-type-death_certificate = תעודת פטירה
document-type-marriage_certificate = תעודת נישואין
document-type-census_page = עמוד ממפקד אוכלוסין
document-type-baptism_record = רישום טבילה
document-type-military_record = רשומה צבאית
document-type-will = צוואה
document-type-land_record = רישום מקרקעין
document-type-diary = יומן
document-type-newspaper_clipping = גזיר עיתון
document-type-gravestone_photo = תצלום מצבה
document-type-family_tree_drawing = עץ משפחה מצויר
document-type-audio = הקלטת קול
document-type-video = הקלטת וידאו
document-status-present = מוחזק כאן
document-status-referenced = מוזכר, מוחזק במקום אחר
document-status-known_missing = ידוע כחסר
document-status-lost = אבד
document-status-unknown = מקומו לא ידוע
diag-unsupported_spec_version = הארכיון מצהיר על גרסת AXGF שהבנייה הזאת אינה יכולה לקרוא.
diag-invalid_json = משהו שאמור להיות JSON אינו מתפרש.
diag-invalid_bundle_structure = הארכיון אינו מסודר כפי ש-AXGF דורש.
diag-schema_validation_failed = רשומה אינה תואמת את סכמת AXGF.
diag-dangling_reference = רשומה מפנה לרשומה אחרת שאינה בארכיון.
diag-duplicate_entity_id = לשתי רשומות יש אותו מזהה.
diag-duplicate_unique_ref = שתי רשומות טוענות לאותה הפניה שאמורה להיות ייחודית.
diag-cycle_detected = קשרי המשפחה סובבים במעגל: מישהו היה מתברר כאב קדמון של עצמו.
diag-chronology_conflict = תאריכים סותרים זה את זה, למשל ילד שנולד לפני הורה.
diag-out_of_vocabulary = ערך אינו אחד מהמונחים שהרשימה שלו מתירה.
diag-claim_inconsistent = טענה סותרת את עצמה או טענה אחרת על אותו דבר.
diag-spec_version_mismatch = גרסת ה-AXGF שרשומה מצהירה עליה אינה מתאימה לתוכן שלה.
diag-unknown_attribute = רשומה נושאת מאפיין ש-AXGF אינו מגדיר.
diag-entity_not_found = הרשומה שיש לשנות אינה בארכיון.
diag-entity_already_exists = כבר קיימת רשומה עם המזהה הזה.
diag-unknown_entity_kind = זה אינו סוג רשומה שקיים ב-AXGF.
diag-delete_blocked_by_reference = אי אפשר למחוק את הרשומה כל עוד רשומות אחרות עדיין מפנות אליה.
diag-manual_review_required = אדם צריך לבדוק את זה; זה לא שונה אוטומטית.
diag-zip_read_error = לא ניתן היה לקרוא את קובץ הארכיון.
diag-zip_write_error = לא ניתן היה לכתוב את קובץ הארכיון.
diag-payloads_external = הקבצים המצורפים נשמרים מחוץ לנתונים של הארכיון עצמו.
diag-payload_source_failed = לא ניתן היה לקרוא קובץ מצורף.
diag-payload_sink_failed = לא ניתן היה לכתוב קובץ מצורף.
diag-gedcom_parse_error = שורה בקובץ ה-GEDCOM לא הובנה.
diag-gedcom_unrecognized_tag = קובץ ה-GEDCOM משתמש בתג שהמייבא אינו מכיר, ולכן הרשומה הזאת לא הועברה.
diag-internal = משהו השתבש בתוך הספרייה.
diag-merge_refused = לא ניתן לאחד את שתי הרשומות כפי שהן.
diag-merge_unresolved = אי־הסכמה בין שתי הרשומות לא הוכרעה.
diag-merge_unsupported = לא ניתן לאחד רשומות מסוג זה.
diag-stale_plan = אחת הרשומות השתנתה מאז שהתוכנית הוכנה: תכננו מחדש.
field-person-display-name = שם תצוגה
field-person-display-name-hint = השם שמוצג בכל מקום באתר.
field-person-gender = מגדר
field-person-living = בחיים
field-person-birth-date = תאריך לידה
field-date-value-hint = שנה, שנה וחודש, או תאריך מלא: 1923, ‏1923-04 או 1923-04-12. השאירו ריק כשאיש אינו יודע.
field-person-birth-precision = דיוק הלידה
field-precision-hint = עד כמה המקור מדייק בזה.
field-person-birth-circa = הלידה משוערת
field-circa-hint = מוצג כ„בערך 1923” ולא כטענה מדויקת.
field-person-birth-place = מזהה מקום הלידה
field-person-birth-confidence = ודאות הלידה
field-person-confidence-hint = עד כמה אתם בטוחים. זה מה שהאתר מצייר.
field-person-death-date = תאריך פטירה
field-person-death-precision = דיוק הפטירה
field-person-death-circa = הפטירה משוערת
field-person-death-place = מזהה מקום הפטירה
field-person-death-confidence = ודאות הפטירה
field-person-death-cause = סיבת המוות
field-person-bio = ביוגרפיה
field-notes = הערות
field-family-name = שם המשפחה
field-description = תיאור
field-family-union-type = סוג הקשר
field-family-union-status = מצב הקשר
field-family-union-confidence = ודאות הקשר
field-family-union-confidence-hint = קובע עד כמה חזק מצויר הקו בין בני הזוג בעץ.
field-family-union-start = תחילת הקשר
field-family-union-end = סוף הקשר
field-family-notes-hint = בני זוג וילדים הם רשימות — ערכו אותם ב-JSON הגולמי שלמטה, או בדף הקשרים של האדם.
field-category = קטגוריה
field-required-hint = חובה.
field-event-subcategory = תת-קטגוריה
field-date = תאריך
field-event-date-hint = נדרש לפי הסכמה.
field-precision = דיוק
field-circa = משוער
field-place-id = מזהה מקום
field-confidence = ודאות
field-source-id = מזהה מקור
field-link-from-type = סוג המקור של הקשר
field-link-from-id = מזהה המקור של הקשר
field-link-to-type = סוג היעד של הקשר
field-link-to-id = מזהה היעד של הקשר
field-link-label = תווית
field-link-label-hint = נקראת קדימה: „סנדק”, „מעסיק”, „עד”. חובה.
field-link-label-reverse = תווית הפוכה
field-link-label-reverse-hint = איך זה נקרא מהקצה השני: „בן סנדקות”, „עובד”.
field-link-bidirectional = נקרא אותו דבר בשני הכיוונים
field-valid-from = בתוקף מ-
field-link-valid-from-hint = מתי הקשר התחיל.
field-valid-until = בתוקף עד
field-link-confidence-hint = „85% בטוח, לפי מכתב משפחתי” — הדבר ש-GEDCOM אינו יכול לומר.
field-note = הערה
field-occupation-person-id = מזהה האדם
field-occupation-title = עיסוק
field-occupation-title-hint = חובה, למשל מורה בבית ספר.
field-occupation-title-latin = עיסוק (בכתב לטיני)
field-occupation-employer = מעסיק
field-occupation-from = מ-
field-occupation-from-hint = עיסוק הוא תקופה. מתן שני הקצוות הוא מה שמצייר אותו כפס.
field-occupation-until = עד
field-source-title = כותרת
field-source-type = סוג המקור
field-source-reliability = אמינות
field-source-reliability-hint = חובה. מוצג כתג ליד כל עובדה שנשענת על המקור הזה.
field-source-status = מצב המקור
field-source-repository = מאגר
field-source-repository-reference = סימול במאגר
field-source-transcription = תעתיק
field-place-name = שם עיקרי
field-place-name-lang = שפת השם
field-place-name-lang-hint = תג שפה, למשל he, en או pl.
field-place-type = סוג המקום
field-place-region = אזור
field-place-country-current = המדינה כיום
field-place-country-current-hint = היסטוריית הגבולות שלו היא רשימה — ערכו אותה ב-JSON הגולמי שלמטה.
field-document-filename = שם הקובץ
field-document-mime-type = סוג המדיה
field-document-mime-type-hint = חובה, למשל image/jpeg.
field-document-type = סוג המסמך
field-document-status = מצב הקובץ
field-document-url = כתובת אינטרנט
field-document-caption = כיתוב
lang-zh-Hans = סינית פשוטה
family-lineage = הורות
links-relation = סוג הקשר
occupations-position = התפקיד
field-link-relation = סוג הקשר
field-link-relation-hint = אחד מהקשרים ש-AXGF 1.1 מגדיר. התווית שלמעלה שומרת את המילים של הרשומה עצמה.
field-occupation-position = התפקיד
field-occupation-position-hint = התפקיד בתוך העיסוק: מנהלת, כשהעיסוק הוא הוראה.
error-delete-changed-title = השתנה מאז שהסתכלתם
error-delete-changed-detail = הרשומה הזאת נשמרה שוב אחרי שהדף שממנו מחקתם אותה צויר, והיא עכשיו בגרסה { $version }. שום דבר לא נמחק. הסתכלו עליה כפי שהיא לפני שתחליטו שוב.
error-delete-changed-look = להסתכל עליה שוב
documents-files = קבצים שמצורפים כאן
documents-files-help = ערכו את פרטי הקובץ, או מחקו את הקובץ עצמו. מחיקה מסירה את המסמך ואת הבתים שלו מהארכיון, לכל מי שהוא מצורף אליו; כדי להסיר אותו מהאדם הזה בלבד, נקו את השורה שלו למעלה.
documents-edit-details = עריכת פרטים
documents-delete = מחיקת הקובץ הזה
radar-section = תרשימים מתוך הרשומה
radar-section-help = שלוש קריאות של מה שהרשומה הזאת מחזיקה, כל ציר מ-0 עד 100. כל מספר מחושב מהעובדות שמפורטות לצדו, לפי כללים שכתובים בתיעוד של היישום; שום דבר לא נשמר, שום דבר לא מנוחש, וציר שאין בו מה לקרוא נשאר ריק במקום לקבל ציון בינוני.
radar-physique = מבנה גופני
radar-mind = מזג ונפש
radar-vitality = בריאות וחיוניות
radar-axis-stature = קומה
radar-axis-build = מבנה
radar-axis-lean-mass = מסת גוף רזה
radar-axis-posture = יציבה
radar-axis-gait = הליכה
radar-axis-dentition = שיניים
radar-axis-openness = פתיחות
radar-axis-conscientiousness = מצפוניות
radar-axis-extraversion = מוחצנות
radar-axis-agreeableness = נעימות
radar-axis-stability = יציבות רגשית
radar-axis-cognition = קוגניציה
radar-axis-circulation = מחזור הדם
radar-axis-breathing = נשימה
radar-axis-metabolism = חילוף חומרים
radar-axis-illness = חופש ממחלות
radar-axis-senses = חושים
radar-axis-rest = שינה ומצב רוח
radar-folded-open = הצגת התרשים הזה
radar-folded-why = האדם הזה רשום כחי. דיוקן של מזגו של אדם חי נשאר מקופל עד שמישהו שרשאי לקרוא אותו מבקש לראות.
radar-empty = עדיין אין ברשומה הזאת דבר שאפשר לקרוא לתוך התרשים הזה.
radar-table-caption = { $chart }: כל ציר, הציון שלו וממה הוא נקרא
radar-col-axis = ציר
radar-col-score = ציון
radar-col-from = נקרא מ-
radar-no-score = אין ציון
radar-from-none = כלום
record-link-outgoing = מהאדם הזה
record-link-incoming = אל האדם הזה

health-attention = דורש תשומת לב:
health-standing-token = עדיין מוגדר אסימון מנהל לשעת חירום. הוא מתחבר מעבר לכל חשבון ולכל הרשאה — זה בדיוק תפקידו ביום שבו איש אינו יכול להתחבר, ולא משהו להשאיר במקומו עכשיו כשמישהו כן יכול. הסירו אותו מקובץ הסביבה של השירות והפעילו מחדש.
health-bundle-invalid = { $errors ->
        [one] נתוני המשפחה כבר לא עוברים אימות: שגיאה אחת, שמפורטת תחת „אימות” למטה.
        [two] נתוני המשפחה כבר לא עוברים אימות: שתי שגיאות, שמפורטות תחת „אימות” למטה.
        [many] נתוני המשפחה כבר לא עוברים אימות: { $errors } שגיאות, שמפורטות תחת „אימות” למטה.
       *[other] נתוני המשפחה כבר לא עוברים אימות: { $errors } שגיאות, שמפורטות תחת „אימות” למטה.
    }
health-disk-unknown = לא ניתן היה לקרוא את השטח הפנוי במכונה הזאת, ולכן שום דבר כאן אינו יכול להבטיח שהשמירה הבאה תיכנס.
health-disk-no-room-to-save = לא ניתן לשמור: פנויים { $free }, ובנייה מחדש של החבילה הזאת דורשת { $need }. שום דבר לא אבד, ושום דבר אינו יכול להשתנות עד שיהיה מקום.
health-disk-critical = הדיסק פנוי ב-{ $percent }% — נותרו { $free }. השמירה תפסיק לעבוד בקרוב.
health-disk-low = הדיסק פנוי ב-{ $percent }% — נותרו { $free }. כדאי לטפל בזה לפני שזה יהפוך לדחוף.
health-backup-unconfigured = שום דבר אינו מגובה. דיסק אחד שיתקלקל ייקח איתו כל רשומה.
health-backup-never = הוגדרה תיקיית גיבוי, אבל אף גיבוי עוד לא נכתב אליה.
health-backup-stale = { $days ->
        [one] הגיבוי החדש ביותר בן יום. צריך לכתוב גיבוי בכל יום.
        [two] הגיבוי החדש ביותר בן יומיים. צריך לכתוב גיבוי בכל יום.
        [many] הגיבוי החדש ביותר בן { $days } ימים. צריך לכתוב גיבוי בכל יום.
       *[other] הגיבוי החדש ביותר בן { $days } ימים. צריך לכתוב גיבוי בכל יום.
    }
health-cache-missing = { $missing ->
        [one] אחד מתוך { $declared } הקבצים המצורפים חסר במטמון ולא ניתן להוריד אותו עד שהשמירה הבאה תבנה אותו מחדש.
        [two] { $missing } מתוך { $declared } הקבצים המצורפים חסרים במטמון ולא ניתן להוריד אותם עד שהשמירה הבאה תבנה אותם מחדש.
        [many] { $missing } מתוך { $declared } הקבצים המצורפים חסרים במטמון ולא ניתן להוריד אותם עד שהשמירה הבאה תבנה אותם מחדש.
       *[other] { $missing } מתוך { $declared } הקבצים המצורפים חסרים במטמון ולא ניתן להוריד אותם עד שהשמירה הבאה תבנה אותם מחדש.
    }

nav-river = נהר
nav-home = משפחה
river-aria = הנהר סביב { $name }: אבות למטה, צאצאים למעלה.
river-band-living = בחיים
river-circa = בערך
river-trail = מסלול
river-back = חזרה →
river-range = טווח
river-motion = תנועה
river-travel = מעבר
river-cut = חיתוך
river-legend-attested = מתועד
river-legend-inferred = מוסק
river-legend-speculative = השערה
river-legend-lost = הרשומה נגמרת
river-legend-repeat = ×n מגיעים אליו ב-n קווים, מצויר מתחת לכל אחד
river-legend-again = ↔ מצויר שוב ליד בן זוג מאוחר יותר
river-legend-discharge = צאצאים
river-legend-direction = ↓ אל המקור · ↑ אל החיים
view-tree-direction = ↓ השורשים: אבות · ↑ הצמרת: צאצאים
view-galaxy-direction = ↓ אבות מתחת לשמש · ↑ צאצאים מעליה
view-summit-direction = ↓ אבות בעמק · ↑ צאצאים במעלה אל הפסגה
river-legend-generation = דור אחד ≈ 29 שנים
river-zoom = זום
river-still = עצירת הנוף
view-tree-still = עצירת העץ
view-galaxy-still = עצירת השמיים
view-summit-still = עצירת ההר
river-whole = כל המשפחה
river-names-all = כל השמות
river-zoom-hint = זום: Ctrl + גלגלת, צביטה, או + / − · כשמוגדל, גררו כדי לזוז
river-grid-link = תצוגת רשת
river-tab-sources = מקורות
river-legend-title = איך לקרוא את הנהר
view-tree-name = עץ חי
view-tree-legend = איך לקרוא את העץ
view-galaxy-name = גלקסיה
view-galaxy-legend = איך לקרוא את הגלקסיה
view-summit-name = עלייה
view-summit-legend = איך לקרוא את ההר
hud-ancestors = אבות
hud-descendants = צאצאים
hud-generations = דורות
hud-living = בחיים
hud-attested = קשרים מתועדים

search-label = חיפוש אדם
search-short = אדם
date-qualifier = מגדיר
date-qualifier-circa = בערך
search-title = חיפוש
search-heading = אנשים שתואמים ל„{ $q }”
search-none = אין מישהו שאתם יכולים לראות בשם הזה.
search-open-record = פתיחת הרשומה
search-hint = Enter עובר אליהם בנהר; Shift+Enter פותח את הרשומה שלהם.
view-tree-search-hint = Enter מגדל את העץ סביבם; Shift+Enter פותח את הרשומה שלהם.
view-galaxy-search-hint = Enter הופך אותם לשמש; Shift+Enter פותח את הרשומה שלהם.
view-summit-search-hint = Enter לוקח אתכם אליהם; Shift+Enter פותח את הרשומה שלהם.
search-summary = { $n ->
        [one] אדם אחד
        [two] { $n } אנשים
        [many] { $n } אנשים
       *[other] { $n } אנשים
    } · ↵ מעבר · ⇧↵ פתיחת הרשומה
search-empty-query = הקלידו שם כדי לחפש.

nav-quality = איכות הנתונים
quality-title = דברים ששווה לבדוק
quality-none = אין מה לבדוק: כל הבדיקות עברו.
quality-intro = שום דבר כאן לא משתנה בשבילכם. כל שורה היא דפוס ששווה לבדוק — לעיתים קרובות יש לו סיבה טובה. פתחו את הרשומה והחליטו, או סמנו שנבדק.
quality-groups-label = סוגי ממצאים
quality-all = הכול
quality-show-label = הצגה
quality-show-open-option = לבדיקה
quality-download = הורדה
quality-group-names = שמות
quality-group-relations = קשרים
quality-group-dates = תאריכים
quality-group-places = מקומות
quality-group-evidence = ראיות
quality-export-csv = הורדה כ-CSV
quality-export-json = הורדה כ-JSON
quality-open = פתיחה
quality-none-file = אין בקובץ הזה דבר ששווה לבדוק.
quality-show-list = הצגת הרשימה
quality-after-import = שום דבר בקובץ לא שונה. כשהוא יהיה הארכיון של האתר, דף איכות הנתונים יציג את אותם דברים עם קישור לכל רשומה.
quality-file-list-cut = רק { $shown } הראשונים מפורטים כאן. כשהקובץ יהיה הארכיון של האתר, דף איכות הנתונים יפרט את כולם.
quality-rule-name-sentence = משפט בשדה של שם
quality-name-sentence = „{ $value }” נראה יותר כמו משפט מאשר שם: אם זה הסבר, השאירו את השם בשדה והעבירו את השאר להערה.
quality-rule-union-parent-child = זוג שהם גם הורה וילד
quality-union-parent-child = { $parent } ו{ $child } מופיעים גם כזוג וגם כהורה וילד: כדאי לבדוק איזה קשר המקורות תומכים בו.
quality-rule-child-listed-twice = ילד שרשום פעמיים
quality-child-listed-twice = { $child } מופיע פעמיים בין ילדי המשפחה הזאת: אם זה אותו ילד, אפשר להסיר רשומה אחת.
quality-rule-duplicate-union = אותו זוג נרשם פעמיים
quality-duplicate-union = ל{ $a } ול{ $b } רשומים { $count } קשרים זוגיים: אם הם נישאו רק פעם אחת, ייתכן שזה קשר אחד שהוזן פעמיים.
quality-rule-partner-twice = אדם אחד בשני הצדדים של קשר זוגי
quality-partner-twice = { $person } מופיע בשני הצדדים של הקשר הזה: כדאי לבדוק מי היה בן הזוג האחר, או להשאיר את הצד הזה ריק אם איש אינו יודע.
quality-rule-foreign-surnames = שם משפחה של בן שאינו תואם אף הורה
quality-foreign-surnames = שם משפחה שאינו תואם את זה של אף הורה: { $who }. זה יכול להיות רגיל — שינוי שם, ילד חורג, איות אחר — אבל שווה לבדוק. (בנות אינן נכללות, כי שם נישואין הוא הסיבה הרגילה.)
quality-rule-death-before-birth = פטירה לפני לידה
quality-death-before-birth = ל{ $person } רשומה פטירה ב-{ $death }, לפני הלידה ב-{ $birth }: ייתכן שאחד משני התאריכים הוקלד בטעות; כדאי לבדוק מול המקורות.
quality-rule-child-before-parent = ילד שנולד לפני הורה
quality-child-before-parent = { $child } (נולד/ה { $childborn }) נראה שנולד לפני ההורה { $parent } (נולד/ה { $parentborn }): כדאי לבדוק את התאריכים או את ההורות.
quality-rule-parent-too-young = הורה צעיר מגיל שתים-עשרה
quality-parent-too-young = { $parent } היה בן/בת { $age } כש{ $child } נולד/ה, וזה חריג: כדאי לבדוק את התאריכים או את ההורות.
quality-rule-parent-too-old = הורה מבוגר מגיל שישים וחמש
quality-parent-too-old = { $parent } היה בן/בת { $age } כש{ $child } נולד/ה, וזה חריג: כדאי לבדוק את התאריכים או את ההורות.
quality-rule-marriage-before-birth = נישואין לפני לידת אחד מבני הזוג
quality-marriage-before-birth = הנישואין האלה מתוארכים ל-{ $married }, לפני ש{ $person } נולד/ה ב-{ $born }: כדאי לבדוק את תאריך הנישואין.
quality-rule-isolated-record = רשומה בלי תאריך ובלי קשרים
quality-isolated-record = ל{ $person } אין תאריך ואין קשר לאף אחד במשפחה: כדאי להוסיף איך הוא שייך, או להסיר את הרשומה אם הוזנה בטעות.
quality-rule-place-no-region = מקום בלי אזור
quality-place-no-region = ל{ $place } אין אזור: הוספת אזור עוזרת להבחין בין מקומות באותו שם.
quality-rule-region-free-text = אזור שנראה כמו הערה
quality-region-free-text = האזור של { $place } הוא „{ $value }”, וזה נראה יותר כמו הערה מאשר אזור: כדאי להעביר את ההערה לרשומה שהיא מתארת.
quality-rule-place-near-duplicate = מקומות שאולי הם אותו מקום
quality-place-near-duplicate = { $names } אולי הם אותו מקום שנכתב באופן שונה: אם כן, אפשר למזג אותם לאחד.
quality-rule-not-a-place = רשומה שאולי אינה מקום
quality-not-a-place = „{ $place }” נמצא ברשימת המקומות אבל אולי אינו מקום — הוא מתחיל באות קטנה, ושמות מקומות בדרך כלל לא: כדאי לבדוק אם זו הערה על מקום.
quality-rule-certain-without-source = ודאי, בלי מקור
quality-certain-without-source = ל{ $record } יש טענה שמסומנת כוודאית לחלוטין ({ $claims }) בלי מקור מצורף: כדאי לצרף את המקור, או להוריד את רמת הוודאות.
quality-dismiss = נבדק — רגיל אצלנו
quality-restore = להציג שוב
quality-note-label = למה (רשות)
quality-show-dismissed = נבדקו ונשמרו
quality-show-open = חזרה לרשימה הפתוחה
quality-dismissed-by = נבדק על ידי { $by } ב-{ $at }
quality-gone-title = הממצא הזה כבר לא שם
quality-gone-detail = הרשומה השתנתה מאז שהרשימה נפתחה, ולכן הממצא כבר לא נקרא אותו דבר. טענו את הרשימה מחדש והסתכלו שוב.
quality-summary = { $n ->
    [one] דבר אחד ששווה לבדוק
    [two] שני דברים ששווה לבדוק
    [many] { $n } דברים ששווה לבדוק
   *[other] { $n } דברים ששווה לבדוק
    }
quality-summary-file = { $n ->
    [one] דבר אחד ששווה לבדוק בקובץ הזה
    [two] שני דברים ששווה לבדוק בקובץ הזה
    [many] { $n } דברים ששווה לבדוק בקובץ הזה
   *[other] { $n } דברים ששווה לבדוק בקובץ הזה
    }
quality-dismissed-summary = { $n ->
    [one] ממצא אחד נבדק ונשמר
    [two] שני ממצאים נבדקו ונשמרו
    [many] { $n } ממצאים נבדקו ונשמרו
   *[other] { $n } ממצאים נבדקו ונשמרו
    }
quality-dismiss-all = סימון הכול כנבדק ({ $n })
quality-restore-all = הצגת הכול שוב ({ $n })
quality-bulk-conflict = רשומות שהשתנו מאז שהרשימה נפתחה: { $n }. שום דבר לא סומן; טענו את הרשימה מחדש ונסו שוב.

edit-tab-changed = שונה, עדיין לא נשמר
edit-remove = הסרה
edit-add-partner = הוספת בן/בת זוג
edit-add-child = הוספת ילד
edit-union-with = קשר זוגי עם { $names }
edit-union-alone = קשר זוגי בלי בן זוג רשום
edit-child-of = ילד של { $names }
edit-new-family = הקמת משפחה
edit-cite-title = מה נשען על מקור
edit-attach-to = לצטט עבור
edit-add-source = הוספת מקור
edit-error-source-title = מקור חדש צריך כותרת. שום דבר לא נשמר.
edit-detach-file = ניתוק מהרשומה הזאת
edit-no-sources = אף אחת מהעובדות של האדם הזה עדיין אינה מצטטת מקור.
edit-no-files = עדיין לא מצורף קובץ לרשומה הזאת.
edit-saved = { $n ->
        [one] נשמר: רשומה אחת נכתבה.
        [two] נשמר: שתי רשומות נכתבו בשמירה אחת.
        [many] נשמר: { $n } רשומות נכתבו בשמירה אחת.
       *[other] נשמר: { $n } רשומות נכתבו בשמירה אחת.
    }
edit-saved-none = שום דבר לא השתנה, ולכן שום דבר לא נשמר.
edit-back-to-record = חזרה לרשומה
edit-avatar = תמונת פרופיל
edit-avatar-frame = מסגור הפנים
edit-avatar-upload = שימוש כתמונת הפרופיל
edit-stale = מישהו אחר שינה את { $what } מאז שהדף הזה נפתח. שום דבר לא נשמר; כל מה שהזנתם עדיין כאן.
edit-refused = שום דבר לא נשמר: { $reason }

error-cross-site-title = נדחה: נשלח מאתר אחר
error-cross-site-detail = הטופס הזה הגיע מדף באתר אחר, ולכן לא בוצע. אם שלחתם אותו מהאתר הזה, טענו את הדף מחדש ושלחו שוב.
admin-nav-label = אזורי הניהול
admin-nav-overview = סקירה
admin-nav-bundle = חבילת AXGF
admin-nav-infrastructure = תשתית
admin-nav-accounts = חשבונות
admin-nav-connectivity = קישוריות

bundle-title = חבילת AXGF
bundle-lede = הארכיון של המשפחה כפי שהאתר הזה מחזיק אותו: { $path }. למה הוא תואם, מה נראה שגוי, ושתי הדרכים לקחת עותק.
bundle-unset-title = עדיין אין משפחה.
bundle-unset-detail = ההתקנה הזאת מחזיקה חבילה ריקה, ולכן עדיין אין מה לבדוק. הקימו אותה מארכיון, מקובץ GEDCOM או מכלום.
bundle-kpi-conformity = תאימות
bundle-verdict-ok = תואם
bundle-verdict-warn = לשים לב
bundle-verdict-fail = אינו תואם
findings-tally = ירוק: { $ok } · אזהרות: { $warn } · קריטי: { $fail }
bundle-kpi-entities = ישויות
bundle-kpi-entities-note = { $persons ->
    [one] אדם אחד
    [two] { $persons } אנשים
    [many] { $persons } אנשים
    *[other] { $persons } אנשים
    } · { $files ->
    [one] קובץ מצורף אחד
    [two] { $files } קבצים מצורפים
    [many] { $files } קבצים מצורפים
    *[other] { $files } קבצים מצורפים
    }
bundle-kpi-size = גודל בדיסק
bundle-kpi-members = { $n ->
    [one] קובץ אחד בארכיון
    [two] { $n } קבצים בארכיון
    [many] { $n } קבצים בארכיון
    *[other] { $n } קבצים בארכיון
    }
bundle-kpi-format = תבנית
bundle-kpi-format-note = הבנייה הזאת קוראת וכותבת AXGF { $current }.
bundle-conformity = תאימות
bundle-conformity-intro = תשע בדיקות, שנקראות מהחבילה כפי שהיא עכשיו; אף אחת מהן אינה משנה אותה. פתחו שורה כדי לראות מה לעשות.
bundle-area-format = תבנית
bundle-area-integrity = שלמות
bundle-area-privacy = פרטיות
bundle-area-content = תוכן
level-ok = ירוק
level-warn = אזהרה
level-fail = קריטי
findings-what-to-do = מה לעשות
findings-go = לשם ←
bundle-anomalies = חריגות
bundle-anomalies-intro = מה שדף איכות הנתונים מוצא, כלל אחר כלל: עדיין פתוח, וכבר סומן כנבדק.
bundle-anomalies-rule = כלל
bundle-anomalies-open = פתוח
bundle-anomalies-checked = נבדק
bundle-anomalies-none = דף איכות הנתונים לא מוצא דבר לבדוק.
bundle-diagnostics = אבחוני המאמת
bundle-facts = פרטים
bundle-fact-path = קובץ
bundle-fact-size = גודל
bundle-fact-spec = תבנית
bundle-fact-schema = סכמה מצורפת
bundle-fact-none = אין
bundle-fact-created = נוצר
bundle-fact-updated = נכתב לאחרונה
bundle-fact-generator = נכתב על ידי
bundle-fact-gedcom = הומר מ-
bundle-export = ייצוא
bundle-export-raw = החבילה כפי שהיא (.axgf)
bundle-export-raw-go = הורדת החבילה
bundle-export-sealed = מוצפנת לאדם אחד (.axgf.gpg)
bundle-export-sealed-intro = אותה חבילה, כהודעת OpenPGP שרק מי שמחזיק את המפתח הפרטי יכול לפתוח. הדביקו את המפתח הציבורי שלו: הטקסט שמתחיל ב„-----BEGIN PGP PUBLIC KEY BLOCK-----”.
bundle-export-sealed-go = הורדה מוצפנת
seal-recipient = עבור
seal-fingerprint = טביעת אצבע
seal-algorithm = אלגוריתמים
seal-fips-yes = בתוך הקבוצה המאושרת לפי FIPS
seal-fips-no = מחוץ לקבוצה המאושרת לפי FIPS
seal-expires = המפתח פג
seal-never = לעולם לא
seal-key-label = המפתח הציבורי של הנמען
seal-key-hint = מפתח RSA של 2048 סיביות לפחות, או מפתח עקום אליפטי. בדקו את טביעת האצבע מול הבעלים שלו לפני שאתם שולחים משהו.
seal-remember = לשמור את המפתח הזה לפעם הבאה
seal-check = קריאת המפתח בלבד
seal-fips-note = החבילה מוצפנת ב-AES-256 למפתח של הנמען. האלגוריתמים המאושרים לפי FIPS הם RSA של 3072 סיביות ומעלה והעקומים של NIST ‏P-256, ‏P-384 ו-P-521: בקשו P-384. הספרייה שבשימוש כאן אינה בעצמה מודול מאומת לפי FIPS 140-3.
seal-decrypt-note = הנמען פותח אותה עם GnuPG:
seal-forget = שכחת המפתח השמור
seal-key-kept = המפתח { $fingerprint } נשמר לייצוא המוצפן הבא.
seal-error-no-key = הדביקו קודם את המפתח הציבורי של הנמען.
seal-error-not-a-key = זה אינו מפתח ציבורי של OpenPGP. הדביקו את כל הבלוק, משורת ה-BEGIN ועד שורת ה-END, ולעולם לא מפתח פרטי.
seal-error-bad-signatures = החתימות של המפתח הזה על עצמו אינן מאומתות: ייתכן ששונה. בקשו מהבעלים שלו עותק חדש.
seal-error-expired = תוקף המפתח הזה, או של כל מפתח בו שיכול להצפין, פג. בקשו מהבעלים שלו להאריך אותו או לשלוח חדש.
seal-error-revoked = הבעלים ביטל את המפתח הזה. בקשו ממנו את המפתח החדש.
seal-error-no-encryption-key = המפתח הזה יכול לחתום אבל לא להצפין: אין לו תת-מפתח הצפנה. בקשו מהבעלים שלו אחד (gpg --quick-add-key … encr).
seal-error-too-weak = למפתח ה-RSA הזה יש { $bits } סיביות; ייצוא דורש לפחות { $min }.
seal-error-unsupported = האלגוריתם של המפתח הזה אינו אחד שייצוא משתמש בו: רק מפתחות RSA או עקום אליפטי.
seal-error-io = לא ניתן היה ליצור את העותק המוצפן: { $error }
date-time-utc = { $date }, { $time } UTC

bundle-check-format-title = גרסת התבנית
bundle-check-format-ok = AXGF { $version }, הגרסה שהבנייה הזאת קוראת וכותבת.
bundle-check-format-missing = המניפסט אינו מציין גרסת AXGF.
bundle-check-format-missing-fix = החבילה לא נכתבה בכלי AXGF, או שנפגמה. שחזרו את הגיבוי האחרון שעבר אימות (axgf-cms restore).

bundle-check-format-older = נכתב כ-AXGF { $version }; השמירה הבאה תכתוב אותו כ-AXGF { $current }.
bundle-check-format-older-fix = אין מה לעשות: העריכה הבאה תכתוב אותו מחדש. שמרו עותק של הקובץ הישן אם תוכנה אחרת עדיין קוראת רק { $version }.
bundle-check-format-newer = נכתב כ-AXGF { $version }, חדש יותר מ-{ $current } של הבנייה הזאת: שמירה עלולה לאבד את מה שהבנייה הזאת אינה מבינה.
bundle-check-format-newer-fix = שדרגו את axgf-cms לגרסה שקוראת AXGF { $version } לפני שמישהו עורך (תשתית, גרסה).
bundle-check-manifest-title = הספירות במניפסט
bundle-check-manifest-ok = הספירות במניפסט תואמות את מה שהחבילה מחזיקה.
bundle-check-manifest-drift = המניפסט מצהיר על { $declared } במקום שבו החבילה מחזיקה { $actual } ({ $collection }).
bundle-check-manifest-fix = שום דבר לא אבד: הספירות ייכתבו מחדש בשמירה הבאה. אם הארכיון הגיע ממקום אחר, בדקו שהוא הועתק בשלמותו.
bundle-check-validation-title = אימות
bundle-check-validation-ok = המאמת של הספרייה לא מוצא שום בעיה.
bundle-check-validation-errors = { $errors ->
        [one] שגיאה אחת
        [two] שתי שגיאות
        [many] { $errors } שגיאות
       *[other] { $errors } שגיאות
    }: החבילה אינה עוברת אימות.
bundle-check-validation-errors-fix = כל אבחון שלמטה מציין את הרשומה ואת השדה. תקנו את הרשומה, או שחזרו את הגיבוי האחרון שעבר אימות.
bundle-check-validation-warnings = { $warnings ->
        [one] אזהרה אחת
        [two] שתי אזהרות
        [many] { $warnings } אזהרות
       *[other] { $warnings } אזהרות
    }. הן לעולם אינן חוסמות שמירה; כל אחת אומרת שמשהו אולי לא בסדר.
bundle-check-validation-warnings-fix = קראו אותן למטה. רובן הפניות לרשומות שאינן בחבילה הזאת, דבר שייבוא GEDCOM משאיר לעיתים קרובות.
bundle-check-payloads-title = קבצים מצורפים
bundle-check-payloads-none = אין קבצים מצורפים.
bundle-check-payloads-ok = { $declared ->
        [one] הקובץ היחיד קיים.
        [two] שני הקבצים קיימים.
        [many] כל { $declared } הקבצים קיימים.
       *[other] כל { $declared } הקבצים קיימים.
    }
bundle-check-payloads-missing = { $missing } מתוך { $declared } הקבצים המצורפים חסרים במטמון.
bundle-check-payloads-fix = השמירה הבאה בונה מחדש את המטמון מתוך החבילה. אם הם חסרים בחבילה עצמה, שחזרו אותם מגיבוי.
bundle-check-binding-title = חשבונות וחבילה
bundle-check-binding-ok = קובץ החשבונות נוצר עבור המשפחה הזאת.
bundle-check-binding-mismatch = קובץ החשבונות נוצר עבור { $expected }; החבילה הזאת היא { $found }.
bundle-check-binding-fix = אם החבילה שוחזרה או שונה שמה בכוונה, אין מה לעשות. אחרת, החשבונות של משפחה אחת חלים על העץ של משפחה אחרת: החזירו את קובץ ה-‎.acl הנכון לצדה.
bundle-check-living-title = החיים, בפומבי
bundle-check-living-ok = אף אחד שרשום כחי אינו גלוי למבקרים.
bundle-check-living-public = { $n ->
        [one] אדם אחד שרשום כחי מסומן
        [two] { $n } אנשים שרשומים כחיים מסומנים
        [many] { $n } אנשים שרשומים כחיים מסומנים
       *[other] { $n } אנשים שרשומים כחיים מסומנים
    } כציבוריים: כל מבקר קורא את הרשומה שלהם.
bundle-check-living-fix = אלא אם הם הסכימו לכך, הגדירו את הנראות שלהם לחברים (עריכה, זהות).
bundle-check-special-title = קטגוריות מיוחדות
bundle-check-special-none = אף רשומה אינה מחזיקה נתוני בריאות, ביומטריה, גנומיקה, רישום פלילי או התנהגות.
bundle-check-special-some = { $n ->
        [one] רשומה אחת מחזיקה
        [two] { $n } רשומות מחזיקות
        [many] { $n } רשומות מחזיקות
       *[other] { $n } רשומות מחזיקות
    } נתוני בריאות, ביומטריה, גנומיקה, רישום פלילי או התנהגות. עבור החיים הם נשארים אצל המנהלים, מה שלא תאמר הרשומה.
bundle-check-quality-title = איכות הנתונים
bundle-check-quality-ok = אין מה לבדוק.
bundle-check-quality-open = { $n ->
        [one] דבר אחד לבדוק
        [two] שני דברים לבדוק
        [many] { $n } דברים לבדוק
       *[other] { $n } דברים לבדוק
    }, ב{ $groups ->
        [one] קבוצה אחת
        [two] שתי קבוצות
        [many] { $groups } קבוצות
       *[other] { $groups } קבוצות
    }.
bundle-check-quality-fix = פתחו את דף איכות הנתונים: כל שורה מציינת את הרשומה ומה נראה שגוי. תקנו, או סמנו שנבדק.
bundle-check-size-title = גודל
bundle-check-size-ok = { $size }, מתחת ל-{ $warn } שמעבר להם הדף הזה מזהיר.
bundle-check-size-heavy = { $size }, מעבר ל-{ $warn }.
bundle-check-size-fix = הארכיון שייך למשפחה, והיא רשאית להגדיל אותו. עקבו אחרי הדיסק והגיבויים (תשתית, אחסון), והעלו את ‎--size-warn-mb אם הגודל הזה צפוי.
seal-algorithm-value = { $key } · AES-256
seal-key-placeholder = -----BEGIN PGP PUBLIC KEY BLOCK-----

infra-area-security = אבטחה
infra-area-access = גישה
infra-area-data = נתונים
infra-area-capacity = קיבולת
infra-area-release = גרסה
infra-area-platform = פלטפורמה
infra-verdict-ok = תקין
infra-verdict-warn = דורש תשומת לב
infra-verdict-fail = קריטי
duration-days = { $n ->
        [one] יום אחד
        [two] יומיים
        [many] { $n } ימים
       *[other] { $n } ימים
    }
duration-hours = { $n ->
        [one] שעה אחת
        [two] שעתיים
        [many] { $n } שעות
       *[other] { $n } שעות
    }
duration-minutes = { $n ->
        [one] דקה אחת
        [two] שתי דקות
        [many] { $n } דקות
       *[other] { $n } דקות
    }
infra-transport-title = איך מגיעים לאתר
infra-transport-ok = הדף הזה הגיע דרך HTTPS.
infra-transport-exposed = הדף הזה הגיע לא מוצפן, והשרת מאזין ב-{ $bind }, מעבר למכונה הזאת: סיסמאות ועוגיות הפעלה עוברות ברשת בגלוי.
infra-transport-exposed-fix = הציבו לפניו פרוקסי הפוך עם HTTPS ‏(Caddy, ‏nginx, ‏HAProxy), וקשרו שוב את השירות ל-127.0.0.1: הריצו שוב את bootstrap.sh עם ‎--bind 127.0.0.1:8080, או הסירו את קובץ ה-drop-in שהרחיב אותו.
infra-transport-plain = הדף הזה הגיע לא מוצפן, משרת שמאזין ב-{ $bind }. זה בסדר על המכונה הזאת בלבד, ורק שם.
infra-transport-plain-fix = היכנסו לאתר דרך כתובת ה-HTTPS שלו. אם פרוקסי אכן מסיים TLS, הגדירו אותו לשלוח X-Forwarded-Proto: https כדי שהשרת הזה יידע.
infra-forwarded-title = כתובות מועברות
infra-forwarded-ok = השרת מאזין רק על המכונה הזאת, ולכן אפשר לסמוך על כתובת הלקוח שפרוקסי מעביר כדי להאט ניסיונות התחברות כושלים חוזרים.
infra-forwarded-exposed = אפשר להגיע לשרת ישירות, והוא סומך על הכותרת X-Forwarded-For: לקוח שמגדיר את הכותרת הזאת בעצמו יכול לחמוק מההאטה אחרי התחברויות כושלות.
infra-forwarded-fix = קשרו את השירות ל-127.0.0.1 מאחורי הפרוקסי, או חסמו את הפורט בחומת אש כך שרק הפרוקסי יגיע אליו.
infra-cross-site-title = שליחות מאתרים אחרים
infra-cross-site-ok = ביממה האחרונה לא נשלח לכאן טופס מאתר אחר.
infra-cross-site-refused = { $n ->
        [one] טופס אחד שנשלח מאתר אחר נדחה ביממה האחרונה.
        [two] שני טפסים שנשלחו מאתרים אחרים נדחו ביממה האחרונה.
        [many] { $n } טפסים שנשלחו מאתרים אחרים נדחו ביממה האחרונה.
       *[other] { $n } טפסים שנשלחו מאתרים אחרים נדחו ביממה האחרונה.
    }
infra-cross-site-fix = שום דבר לא עבר. אם הדחיות הן שלכם — אותו אתר שהגיעו אליו דרך כתובת שנייה — הגדירו את הכתובת הציבורית תחת „קישוריות”, או השתמשו בכתובת האחת הזאת.
infra-sso-title = שירותי התחברות
infra-sso-none = אף שירות התחברות אינו מופעל; החשבונות מתחברים עם סיסמה.
infra-sso-ok = { $n ->
        [one] שירות התחברות אחד מופעל, והכתובת הציבורית היא HTTPS.
        [two] שני שירותי התחברות מופעלים, והכתובת הציבורית היא HTTPS.
        [many] { $n } שירותי התחברות מופעלים, והכתובת הציבורית היא HTTPS.
       *[other] { $n } שירותי התחברות מופעלים, והכתובת הציבורית היא HTTPS.
    }
infra-sso-no-https = שירות התחברות מופעל, אבל הכתובת הציבורית אינה HTTPS: הספקים מסרבים לשלוח את תשובתם לכתובת לא מוצפנת, וההתחברות נכשלת.
infra-sso-fix = תחת „קישוריות”, הגדירו את הכתובת הציבורית לכתובת ה-https:// שהקוראים משתמשים בה.
infra-admins-title = מנהלים
infra-admins-none = אף חשבון פעיל אינו יכול לנהל את האתר הזה. רק אסימון החירום יכול, ורק כל עוד הוא מוגדר.
infra-admins-none-fix = התחברו עם אסימון החירום, צרו מנהל תחת „חשבונות”, ואז הסירו את האסימון.
infra-admins-one = מנהל אחד. אם האדם הזה אינו זמין או מאבד את הסיסמה, איש אינו יכול לנהל את האתר.
infra-admins-one-fix = תנו לאדם שני, מהימן, את תפקיד המנהל תחת „חשבונות”.
infra-admins-ok = { $n ->
        [one] מנהל אחד יכול להתחבר.
        [two] שני מנהלים יכולים להתחבר.
        [many] { $n } מנהלים יכולים להתחבר.
       *[other] { $n } מנהלים יכולים להתחבר.
    }
infra-token-title = אסימון חירום
infra-token-generated = לא מוגדר אסימון חירום.
infra-token-only-way = מוגדר אסימון חירום, ובלי חשבון מנהל הוא הדרך היחידה להיכנס. השאירו אותו עד שיהיה מנהל.
infra-token-standing = עדיין מוגדר אסימון חירום. הוא מתחבר מעבר לכל חשבון ולכל הרשאה.
infra-token-fix = הסירו את AXGF_CMS_ADMIN_TOKEN מקובץ הסביבה של השירות, ואז הפעילו מחדש את השירות.
infra-registration-title = הרשמה עצמית
infra-registration-closed = מבקרים אינם יכולים ליצור חשבונות; רק מנהל יכול.
infra-registration-secret = מבקרים יכולים ליצור חשבון חבר אם הם יודעים את סוד ההרשמה.
infra-registration-open = כל מבקר יכול ליצור חשבון חבר.
infra-registration-open-fix = זה מתאים לאתר שנועד להיות פתוח. אחרת דרשו את סוד ההרשמה, או סגרו את ההרשמה, תחת „חשבונות”.
infra-failures-title = התחברויות כושלות
infra-failures-ok = { $n ->
        [one] התחברות כושלת אחת ביממה האחרונה.
        [two] שתי התחברויות כושלות ביממה האחרונה.
        [many] { $n } התחברויות כושלות ביממה האחרונה.
       *[other] { $n } התחברויות כושלות ביממה האחרונה.
    }
infra-failures-many = { $n ->
        [one] התחברות כושלת אחת ביממה האחרונה: ייתכן שמישהו מנחש סיסמאות.
        [two] שתי התחברויות כושלות ביממה האחרונה: ייתכן שמישהו מנחש סיסמאות.
        [many] { $n } התחברויות כושלות ביממה האחרונה: ייתכן שמישהו מנחש סיסמאות.
       *[other] { $n } התחברויות כושלות ביממה האחרונה: ייתכן שמישהו מנחש סיסמאות.
    }
infra-failures-fix = כל כתובת כבר מואטת אחרי כישלונות חוזרים. אם המספר ממשיך לטפס, מצאו ביומן של הפרוקסי מאיפה מגיעים הניסיונות וחסמו שם, וודאו שלכל מנהל יש סיסמה ארוכה או שהוא מתחבר דרך שירות התחברות.
infra-health-bundle-title = נתוני המשפחה
infra-health-bundle-ok = החבילה נפתחת ועוברת אימות.
infra-health-bundle-fix = פתחו את דף חבילת ה-AXGF כדי לראות את השגיאות ולתקן אותן, או שחזרו את הגיבוי האחרון שעבר אימות (axgf-cms restore).
infra-health-cache-title = מטמון הקבצים המצורפים
infra-health-cache-ok = כל קובץ מצורף נמצא במטמון.
infra-health-cache-fix = שמרו רשומה כלשהי פעם אחת: השמירה בונה מחדש את המטמון. קבצים שעדיין חסרים אחרי זה חסרים בחבילה עצמה.
infra-health-backup-title = גיבויים
infra-health-backup-ok = { $count ->
        [one] קיים גיבוי עדכני; ארכיון אחד נשמר.
        [two] קיים גיבוי עדכני; שני ארכיונים נשמרים.
        [many] קיים גיבוי עדכני; { $count } ארכיונים נשמרים.
       *[other] קיים גיבוי עדכני; { $count } ארכיונים נשמרים.
    }
infra-health-backup-fix = בלי תיקיית גיבוי, הריצו שוב את bootstrap.sh עם ‎--backup-dir. אם יש אחת, בדקו את הטיימר (systemctl status axgf-cms-backup.timer) ואת היומן שלו (journalctl -u axgf-cms-backup).
infra-disk-title = שטח דיסק
infra-disk-ok = { $free } פנויים מתוך { $total } ({ $percent }% פנוי).
infra-disk-fix = פנו מקום במחיצה שמחזיקה את תיקיית הנתונים — גיבויים ישנים שנשמרים גם במקום אחר, יומנים, קבצים של תוכנות אחרות — או העבירו את הנתונים לדיסק גדול יותר.
infra-growth-title = גדילה
infra-growth-ok = { $days ->
        [one] גדל בכ-{ $rate } בחודש; יש מקום ליום אחד נוסף בקצב הזה.
        [two] גדל בכ-{ $rate } בחודש; יש מקום ליומיים נוספים בקצב הזה.
        [many] גדל בכ-{ $rate } בחודש; יש מקום ל-{ $days } ימים נוספים בקצב הזה.
       *[other] גדל בכ-{ $rate } בחודש; יש מקום ל-{ $days } ימים נוספים בקצב הזה.
    }
infra-growth-soon = { $days ->
        [one] גדל בכ-{ $rate } בחודש: בקצב הזה הדיסק יתמלא בעוד יום אחד.
        [two] גדל בכ-{ $rate } בחודש: בקצב הזה הדיסק יתמלא בעוד יומיים.
        [many] גדל בכ-{ $rate } בחודש: בקצב הזה הדיסק יתמלא בעוד { $days } ימים.
       *[other] גדל בכ-{ $rate } בחודש: בקצב הזה הדיסק יתמלא בעוד { $days } ימים.
    }
infra-growth-fix = תכננו את המקום עכשיו: דיסק גדול יותר, פחות גיבויים שנשמרים על אותו דיסק, או העברת הגיבויים לדיסק אחר.
infra-growth-flat = הגיבויים לא גדלו; אין מה לתכנן.
infra-growth-unknown = עדיין אין מספיק גיבויים כדי למדוד גדילה: נדרש לפחות יום בין הישן ביותר לחדש ביותר.
infra-memory-title = זיכרון
infra-memory-ok = בשימוש { $used } מתוך תקרה של { $max } לשירות ({ $percent }%).
infra-memory-high = בשימוש { $used } מתוך תקרה של { $max } לשירות ({ $percent }%): קרוב לנקודה שבה המערכת עוצרת את השירות.
infra-memory-fix = העלו את MemoryMax=‎ בקובץ drop-in עבור axgf-cms.service, או מצאו מה מחזיק את הזיכרון — ייבוא גדול מאוד, קובץ מצורף גדול — והפעילו מחדש את השירות.
infra-memory-unlimited = { $used } בשימוש, בלי תקרה מוגדרת לשירות.
infra-version-title = גרסה
infra-version-ok = v{ $current } היא הגרסה החדשה ביותר שפורסמה.
infra-version-behind = רצה v{ $current }; פורסמה { $latest }.
infra-version-behind-fix = קראו את הערות הגרסה, ואז שדרגו מאזור „גרסה” שלמטה. קודם נלקח גיבוי, והגרסה הקודמת חוזרת מעצמה אם החדשה לא עולה.
infra-version-unknown = לא ידוע אם קיימת גרסה חדשה יותר: רשימת הגרסאות לא נמשכה.
infra-version-unknown-fix = השתמשו ב„בדיקת גרסאות חדשות” שלמטה. ארגז החול של סביבת הייצור אינו יכול להגיע ל-GitHub בעצמו; ברגע שעוזר השדרוג מותקן, הוא מושך את הרשימה מדי יום.
infra-version-stale = { $days ->
        [one] רשימת הגרסאות נמשכה לאחרונה לפני יום אחד.
        [two] רשימת הגרסאות נמשכה לאחרונה לפני יומיים.
        [many] רשימת הגרסאות נמשכה לאחרונה לפני { $days } ימים.
       *[other] רשימת הגרסאות נמשכה לאחרונה לפני { $days } ימים.
    }
infra-upgrades-title = שדרוגים
infra-upgrades-ok = עוזר השדרוג מותקן: הכפתור שלמטה יכול להחליף את הגרסה.
infra-upgrades-manual = עוזר השדרוג אינו מותקן, ולכן הגרסה מוחלפת ידנית על השרת.
infra-upgrades-fix = הריצו את bootstrap.sh הנוכחי פעם אחת כ-root: הוא מתקין את העוזר יחד עם השירות.
infra-sandbox-title = ארגז חול
infra-sandbox-ok = התהליך רץ עם no-new-privileges ועם מסנן קריאות מערכת.
infra-sandbox-unknown = המערכת הזאת אינה מדווחת על ארגז החול של התהליך.
infra-sandbox-off = התהליך רץ בלי חלק מארגז החול שלו: no-new-privileges או מסנן קריאות המערכת כבויים.
infra-sandbox-fix = הריצו את השירות מקובץ היחידה ש-bootstrap.sh כותב, בלי קובצי drop-in שמרפים אותו. במכונת פיתוח זה צפוי.
infra-clock-title = שעון
infra-clock-ok = שעון המערכת מראה { $year }, וזה סביר.
infra-clock-wrong = שעון המערכת מראה { $year }: התאריכים, גילאי הגיבויים וההפעלות כולם שגויים.
infra-clock-fix = הפעילו סנכרון זמן: timedatectl set-ntp true.

infra-title = תשתית
infra-lede = המכונה שהאתר הזה רץ עליה, נבדקת בכל פעם שהדף הזה נפתח: איך מגיעים אליה, מי מחזיק במפתחות, אם הנתונים בטוחים ועדיין נכנסים, איזו גרסה רצה, וארגז החול שסביבה.
infra-kpi-health = תקינות
infra-kpi-version = גרסה
infra-kpi-version-newer = { $version } זמינה
infra-kpi-version-current = מעודכן
infra-kpi-version-unknown = עדיין לא נבדק
infra-kpi-disk = שטח פנוי
infra-kpi-disk-note = { $percent }% מתוך { $total } בשימוש
infra-kpi-compute = מכונה
infra-kpi-cpus = { $n ->
        [one] ליבה אחת
        [two] שתי ליבות
        [many] { $n } ליבות
       *[other] { $n } ליבות
    }
infra-kpi-load = עומס { $load }
infra-kpi-rss = { $size } בשימוש
infra-unknown = לא ידוע
infra-health = תקינות
infra-health-intro = ירוק זה בסדר, אזהרה שווה מבט, קריטי דורש טיפול עכשיו. כל ממצא נפתח על מה לעשות בעניינו.
infra-json = אותן בדיקות כ-JSON, עבור מערכת ניטור
infra-version = גרסה
infra-version-running = רצה
infra-version-latest = האחרונה שפורסמה
infra-version-up-to-date = מעודכן
infra-version-checked = הרשימה נמשכה
infra-version-never = אף פעם
infra-source-direct = על ידי השרת הזה
infra-source-helper = על ידי עוזר השדרוג
infra-version-token = מאגר פרטי
infra-token-env-set = מוגדר אסימון GitHub לרישום ולהורדה של גרסאות.
infra-token-env-unset = אין אסימון. אם המאגר פרטי, שימו אחד ב-{ $name }, קריא ל-root בלבד: עוזר השדרוג קורא אותו משם.
infra-releases-error = הניסיון האחרון למשוך את הרשימה נכשל:
infra-releases-refresh = בדיקת גרסאות חדשות
infra-releases-asked = עוזר השדרוג התבקש לרשימה עדכנית. טענו את הדף הזה מחדש בעוד דקה.
infra-releases-not-asked = לא ניתן היה לכתוב את הבקשה: { $error }
infra-upgrade-status-running = מחליף מ-{ $from } ל-{ $to }…
infra-upgrade-status-succeeded = הוחלף מ-{ $from } ל-{ $to }.
infra-upgrade-status-failed = ההחלפה מ-{ $from } ל-{ $to } נכשלה; { $from } היא עדיין הגרסה שרצה.
infra-upgrade-status-refused = עוזר השדרוג סירב להחלפה ל-{ $to }.
infra-upgrade-log = פלט העוזר
infra-upgrade-pending = { $by } ביקש את { $version } ב-{ $at }; עוזר השדרוג עדיין לא קלט את הבקשה.
infra-releases = גרסאות שפורסמו
infra-releases-tag = גרסה
infra-releases-published = פורסמה
infra-releases-relation = בהשוואה לזו
infra-releases-notes = הערות גרסה
infra-releases-open = קריאה
infra-relation-newer = חדשה יותר
infra-relation-current = זו
infra-relation-older = ישנה יותר
infra-prerelease = גרסה מועמדת
infra-releases-none = עדיין אין רשימת גרסאות.
infra-upgrade = החלפת גרסה
infra-upgrade-intro = הכפתור פונה לעוזר השדרוג, שרץ כ-root מחוץ לארגז החול של האתר. הוא לוקח גיבוי, מתקין את הגרסה אחרי שסכום הביקורת שלה אומת, מפעיל מחדש את השירות ובודק אותו; אם הגרסה החדשה לא עונה, הקודמת מוחזרת.
infra-upgrade-version = גרסה להתקנה
infra-upgrade-confirm = כדי לחזור לגרסה ישנה יותר, הקלידו שוב את התג שלה
infra-upgrade-confirm-hint = לא נדרש לגרסה חדשה יותר. גרסה ישנה יותר אולי לא תקרא נתונים שגרסה חדשה כתבה: קראו קודם את ההערות שלה.
infra-upgrade-go = התקנת הגרסה הזאת
infra-upgrade-safety = האתר אינו זמין לכמה שניות בזמן שהשירות מופעל מחדש.
infra-upgrade-manual = עוזר השדרוג אינו מותקן על השרת הזה, ולכן הדף הזה אינו יכול להחליף את הגרסה. כ-root, על השרת:
infra-upgrade-manual-helper = הרצה אחת של bootstrap.sh הנוכחי מתקינה את העוזר, ומאז הכפתור כאן עובד.
infra-upgrade-asked = התבקשה { $version }. עוזר השדרוג קולט את הבקשה תוך כמה שניות; הדף הזה מראה איך זה הלך.
infra-upgrade-no-helper = עוזר השדרוג אינו מותקן, ולכן אי אפשר להתקין את { $version } מכאן.
infra-upgrade-unknown-version = { $version } אינה ברשימת הגרסאות שפורסמו.
infra-upgrade-same-version = { $version } היא הגרסה שכבר רצה.
infra-upgrade-confirm-older = { $version } ישנה יותר מהגרסה שרצה. הקלידו את התג שלה בשדה האישור כדי לחזור אליה.
infra-volumes = אחסון
infra-volumes-intro = המחיצה שמחזיקה את תיקיית הנתונים ({ $path }), ומה שהאתר הזה שומר עליה.
infra-volumes-partition = מחיצה
infra-of = { $used } מתוך { $total }
infra-volumes-ours = האתר הזה
infra-volumes-breakdown = מה שהאתר הזה שומר
infra-part-bundle = חבילת המשפחה
infra-part-cache = מטמון הקבצים המצורפים
infra-part-backups = גיבויים
infra-part-binary = תוכנה
infra-part-journal = יומן השינויים
infra-part-accounts = חשבונות והגדרות
infra-files = { $n ->
        [one] קובץ אחד
        [two] שני קבצים
        [many] { $n } קבצים
       *[other] { $n } קבצים
    }
infra-growth = גדילה
infra-growth-month = לחודש
infra-growth-not-enough = אין מספיק גיבויים כדי לדעת
infra-growth-left = המקום שנותר בקצב הזה
infra-backups = גיבויים
infra-backups-dir = תיקייה
infra-backups-count = ארכיונים
infra-backups-newest = החדש ביותר
infra-backups-oldest = הישן ביותר
infra-backups-room = מקום היכן שהם נשמרים
infra-free-of = { $free } פנויים מתוך { $total }
infra-paths = נתיבים
infra-path-data = תיקיית הנתונים
infra-path-binary = תוכנה
infra-compute = מחשוב
infra-compute-hidden = ארגז החול של השירות מסתיר ממנו את הנתונים של המכונה עצמה; מה שמוצג כאן הוא מה שהתהליך יכול לראות.
infra-compute-memory = זיכרון המכונה
infra-available-of = { $available } פנויים מתוך { $total }
infra-compute-ceiling = זיכרון השירות
infra-compute-no-ceiling = { $used }, ללא תקרה
infra-compute-os = מערכת
infra-compute-kernel = ליבה
infra-compute-arch = ארכיטקטורה

infra-compute-cpu = מעבד
infra-compute-cpus = ליבות
infra-compute-load = עומס (1, ‏5, ‏15 דקות)
infra-compute-rss = התהליך הזה
infra-compute-threads = תהליכונים
infra-compute-files = קבצים פתוחים
infra-compute-uptime = השירות רץ כבר
infra-compute-sys-uptime = המכונה רצה כבר
infra-compute-sandbox = ארגז חול
infra-compute-nnp-on = ללא הרשאות חדשות
infra-compute-nnp-off = אפשר לקבל הרשאות
infra-compute-seccomp-on = קריאות מערכת מסוננות
infra-compute-seccomp-off = קריאות מערכת לא מסוננות
infra-activity = פעילות
infra-activity-intro = נספר לאורך { $span }: השירות שומר יממה אחת, ומתחיל שוב מאפס כשהוא מופעל מחדש.
infra-activity-sessions = הפעלות פתוחות
infra-activity-signed-in = התחברויות
infra-activity-failed = התחברויות כושלות
infra-activity-registered = חשבונות שנוצרו
infra-activity-cross-site = שליחות מאתרים אחרים שנדחו

sso-error-not-configured = שירות ההתחברות הזה אינו מוגדר באתר הזה.
sso-error-no-public-url = התחברות דרך שירות דורשת את הכתובת הציבורית של האתר, שעדיין לא הוגדרה. מנהל מגדיר אותה תחת „קישוריות”.
sso-error-unreachable = לא ניתן היה להגיע לשירות ההתחברות. נסו שוב בעוד רגע, או התחברו בדרך אחרת.
sso-error-refused = שירות ההתחברות לא הכניס אתכם, או שההתחברות בוטלה שם.
sso-error-stale = תוקף ההתחברות הזאת פג, או שהיא התחילה בדפדפן אחר. התחילו מחדש.
sso-error-invalid = התשובה של שירות ההתחברות לא עברה את הבדיקות של האתר הזה, ולכן לא נעשה בה שימוש. התחילו מחדש; אם זה ממשיך לקרות, ספרו למנהל.
infra-upgrade-already-pending = { $version } כבר ממתינה לעוזר השדרוג. חכו שהיא תסתיים לפני שתבקשו גרסה אחרת.

connectivity-title = קישוריות
connectivity-lede = איך הקוראים מגיעים לאתר הזה ואיך הם מתחברים: הכתובת הציבורית, שירותי ההתחברות והכללים שכל התחברות פועלת לפיהם, כולל של מנהלים.
connectivity-kpi-address = כתובת ציבורית
connectivity-address-unset = לא הוגדרה
connectivity-address-https = HTTPS
connectivity-address-plain = לא מוצפנת
connectivity-address-unset-note = כל שירות התחברות צריך אותה
connectivity-kpi-services = שירותי התחברות
connectivity-kpi-services-note = { $n ->
        [one] מוכן להצעה
        [two] מוכנים להצעה
        [many] מוכנים להצעה
       *[other] מוכנים להצעה
    }
connectivity-kpi-passwords = סיסמאות
connectivity-policy-short-everyone = כולם
connectivity-policy-short-non-admins = לא מנהלים
connectivity-policy-short-nobody = אף אחד
connectivity-policy-note-everyone = כל חשבון רשאי להתחבר עם הסיסמה שלו, וגם עם שירות מקושר.
connectivity-policy-note-non-admins = מנהלים מתחברים רק דרך שירות; כל השאר עדיין רשאים להשתמש בסיסמה. סיסמת מנהל גנובה לא פותחת אז דבר.
connectivity-policy-note-nobody = כולם מתחברים דרך שירות. חשבון בלי שירות מקושר ובלי כתובת לקשר לפיה אינו יכול להתחבר.
connectivity-kpi-admins-linked = { $n ->
        [one] מנהל אחד עם שירות מקושר
        [two] { $n } מנהלים עם שירות מקושר
        [many] { $n } מנהלים עם שירות מקושר
       *[other] { $n } מנהלים עם שירות מקושר
    }
connectivity-kpi-registration = הרשמה
connectivity-kpi-registration-go = הגדרה בדף „חשבונות” ←
registration-short-closed = מנהלים בלבד
registration-short-secret = עם הסוד של המשפחה
registration-short-open = פתוח למבקרים
connectivity-address = כתובת ציבורית
connectivity-address-intro = הכתובת שהקוראים מקלידים, כפי שהם מקלידים אותה: https://family.example. שירותי ההתחברות מחזירים אליה את הקוראים, ולכן היא חייבת להיות הכתובת הציבורית, לא זו שהשרת הזה מאזין בה.
connectivity-address-label = כתובת ציבורית
connectivity-address-hint = השאירו ריק כל עוד איש אינו מתחבר דרך שירות.
connectivity-address-invalid = זו אינה כתובת http או https עם שם מארח.
connectivity-address-saved = הכתובת הציבורית נשמרה.
connectivity-policy = כללי התחברות
connectivity-policy-intro = הם חלים על כל חשבון, כולל של מנהלים. כלל שהיה משאיר אף מנהל שאינו יכול להתחבר נדחה.
connectivity-policy-passwords = מי רשאי להתחבר עם סיסמה
connectivity-policy-without-way = { $n ->
        [one] לחשבון פעיל אחד אין שירות מקושר ואין כתובת: תחת „אף אחד” הוא לא יוכל להתחבר.
        [two] ל-{ $n } חשבונות פעילים אין שירות מקושר ואין כתובת: תחת „אף אחד” הם לא יוכלו להתחבר.
        [many] ל-{ $n } חשבונות פעילים אין שירות מקושר ואין כתובת: תחת „אף אחד” הם לא יוכלו להתחבר.
       *[other] ל-{ $n } חשבונות פעילים אין שירות מקושר ואין כתובת: תחת „אף אחד” הם לא יוכלו להתחבר.
    }
connectivity-link-by-email = קישור שירות לחשבון עם אותה כתובת מאומתת
connectivity-link-by-email-note = כך קרוב משפחה מקבל התחברות בלחיצה אחת בלי שום הרשמה: צרו את החשבון שלו עם הכתובת שלו, וההתחברות הראשונה שלו דרך שירות שאימת את הכתובת הזאת מקשרת אותו. רק כתובת שהקליד מנהל או שאימת שירות מותאמת אי פעם.
connectivity-domains = דומיינים שרשאים לפתוח חשבון
connectivity-domains-hint = אחד בכל שורה, למשל family.example. ריק: כל דומיין. כשמוגדר, רק כתובת ששירות אימת באחד מהדומיינים האלה יכולה לפתוח חשבון, וטופס ההרשמה אינו מוצע.
connectivity-domain-invalid = „{ $domain }” אינו שם דומיין.
connectivity-policy-unknown = בחרו מי רשאי להתחבר עם סיסמה.
connectivity-policy-lockout = נדחה: עם השינוי הזה אף מנהל פעיל לא יוכל להתחבר. קשרו קודם שירות התחברות לחשבון של מנהל, על ידי התחברות דרכו פעם אחת, או השאירו סיסמאות למנהלים.
connectivity-policy-saved = כללי ההתחברות נשמרו.
connectivity-not-saved = לא נשמר: { $error }
connectivity-services = שירותי התחברות
connectivity-services-intro = רשמו את האתר הזה במסוף של השירות עם כתובת ההפניה שמוצגת, ואז הדביקו כאן את מזהה הלקוח ואת הסוד. המבקרים יראו אז „המשך עם …” בדפי ההתחברות וההרשמה.
connectivity-outbound-note = התחברות דרך שירות דורשת שהשרת הזה יגיע לשירות. היחידה ש-bootstrap.sh כותב מסרבת לחיבורים יוצאים (IPAddressDeny=any): אפשרו אותם עם sudo systemctl edit axgf-cms, והוסיפו IPAddressAllow=any תחת [Service], ואז הפעילו מחדש. כפתור הבדיקה אומר אם זה אפשרי.
connectivity-service-ready = מוכן
connectivity-service-incomplete = לא שלם
connectivity-service-off = כבוי
connectivity-service-enable = להציע את השירות הזה בדפי ההתחברות וההרשמה
connectivity-service-issuer = מנפיק
connectivity-service-issuer-hint = הכתובת של הספק, למשל https://auth.family.example/realms/family; ההגדרות שלו נקראות מ-‎/.well-known/openid-configuration שמתחתיה.
connectivity-service-label = תווית הכפתור
connectivity-service-tenant = מדריך (tenant)
connectivity-service-tenant-hint = common לכל חשבון Microsoft, ‏organizations לחשבונות עבודה ולימודים, ‏consumers לחשבונות אישיים, או המזהה של המדריך שלכם.
connectivity-service-client-id = מזהה לקוח
connectivity-service-secret = סוד לקוח
connectivity-service-secret-set = מוגדר סוד. השאירו ריק כדי לשמור עליו.
connectivity-service-secret-unset = עדיין לא הוגדר סוד.
connectivity-service-secret-clear = הסרת הסוד
connectivity-service-redirect = כתובת הפניה לרישום
connectivity-service-redirect-needs-address = הגדירו קודם את הכתובת הציבורית
connectivity-service-console = איפה לרשום
connectivity-service-console-open = המסוף של { $name }
connectivity-service-test = בדיקת החיבור
connectivity-issuer-invalid = המנפיק חייב להיות כתובת https://‎.
connectivity-tenant-invalid = מדריך הוא שם או מזהה: אותיות, ספרות, נקודות ומקפים.
connectivity-provider-incomplete = { $name } אינו שלם: הוא צריך מזהה לקוח וסוד לקוח, ומנפיק עבור ספק כללי.
connectivity-provider-saved = { $name } נשמר.
connectivity-test-ok = { $name } ענה: { $answer }.
connectivity-test-failed = לא ניתן היה להגיע אל { $name } מהשרת הזה: { $detail }
registration-title = הרשמה
registration-intro = האם מבקר רשאי לפתוח כאן חשבון בעצמו. חשבון כזה הוא של חבר: הוא קורא את מה שחברים קוראים ואינו משנה דבר. מתן תפקיד אחר הוא החלטה של מנהל, למטה.
registration-who = מי רשאי לפתוח חשבון
registration-note-closed = רק מנהל יוצר חשבונות, בדף הזה.
registration-note-secret = מבקר שיודע את הסוד של המשפחה רשאי לפתוח חשבון חבר. שתפו את הסוד כמו שהייתם משתפים קוד של דלת.
registration-note-open = כל מבקר רשאי לפתוח חשבון חבר. חברים קוראים את הרשומות שנשמרות לחברים.
registration-rotate = הגרלת סוד חדש
registration-secret = הסוד של המשפחה
registration-secret-note = כל מי שמחזיק בו יכול לפתוח חשבון כל עוד המצב הזה פעיל. הגרלת סוד חדש חוסמת את הישן; חשבונות שכבר נפתחו נשארים.
registration-link = דף ההרשמה
registration-quick = דרכי כניסה מהירות
registration-quick-none = אף שירות התחברות אינו מוכן: הגדירו אחד תחת „קישוריות”
registration-changed = שונה לאחרונה על ידי { $by }, { $at }.
registration-unknown = בחרו מי רשאי לפתוח חשבון.
accounts-kpi-total = חשבונות פעילים
accounts-kpi-total-note = { $disabled ->
        [one] אחד מושבת
        [two] { $disabled } מושבתים
        [many] { $disabled } מושבתים
       *[other] { $disabled } מושבתים
    }
accounts-kpi-admins = מנהלים
accounts-kpi-admins-few = הוסיפו מנהל שני
accounts-kpi-admins-ok = יותר מאחד, כמו שצריך
accounts-kpi-linked = מקושרים לשירות
accounts-kpi-linked-note = { $n ->
        [one] שירות אחד מוכן
        [two] { $n } שירותים מוכנים
        [many] { $n } שירותים מוכנים
       *[other] { $n } שירותים מוכנים
    }
accounts-kpi-registered = { $n ->
        [one] אחד נפתח על ידי מבקר
        [two] { $n } נפתחו על ידי מבקרים
        [many] { $n } נפתחו על ידי מבקרים
       *[other] { $n } נפתחו על ידי מבקרים
    }
accounts-roles = תפקידים
accounts-sign-in = מתחבר עם
accounts-sign-in-password = סיסמה
accounts-sign-in-google = Google
accounts-sign-in-microsoft = Microsoft
accounts-sign-in-github = GitHub
accounts-sign-in-oidc = כניסה יחידה
accounts-sign-in-none = עדיין אין דרך להיכנס
accounts-created-via-register = נפתח על ידי המבקר
accounts-created-via-sso = נפתח דרך שירות
accounts-add-also-register = מבקרים יכולים גם לפתוח חשבונות חבר בעצמם, כפי שהוגדר למעלה. לחשבון שנוצר כאן יכול להיות כל תפקיד.
login-password-not-allowed = כללי ההתחברות של האתר הזה אינם מתירים לחשבון הזה להשתמש בסיסמה: התחברו דרך שירות.
login-register = עדיין אין לכם חשבון? צרו אחד
register-title = יצירת חשבון
register-lede = חשבון חבר קורא את הרשומות שנשמרות לחברי המשפחה. הדרך המהירה ביותר היא חשבון שכבר יש לכם.
register-closed = כאן חשבונות נוצרים על ידי מנהל. בקשו מאחד המנהלים את החשבון שלכם.
register-have-account = כבר יש לכם חשבון? התחברו
register-or = או
register-secret = הסוד של המשפחה
register-secret-hint = מנהל נתן לכם אותו, או מישהו מהמשפחה.
register-secret-hint-services = הקלידו אותו לפני שתבחרו שירות, או שהשלב הבא יבקש אותו.
register-domains = רק כתובות ב-{ $domains } יכולות לפתוח חשבון.
register-username-hint = 2 עד 64 אותיות, ספרות, נקודות, מקפים או קווים תחתונים.
register-submit = יצירת החשבון
register-member-note = לא מתבקשת כתובת: שום דבר לא נשלח אליכם, ואיש אינו יכול לנכס את הכתובת שלכם.
register-form-closed = האתר הזה אינו פותח חשבונות בטופס הזה.
register-throttled = יותר מדי ניסיונות מכאן. חכו רבע שעה ונסו שוב.
register-secret-wrong = זה אינו הסוד של המשפחה.
register-username-invalid = שם משתמש מכיל 2 עד 64 אותיות, ספרות, נקודות, מקפים או קווים תחתונים.
register-password-short = הסיסמה צריכה לפחות { $min } תווים.
register-username-taken = שם המשתמש הזה תפוס. בחרו שם אחר.
register-not-saved = לא ניתן היה לשמור את החשבון. נסו שוב בעוד רגע.
register-finish-title = עוד צעד אחד
register-finish-lede = השירות אישר מי אתם. כדי לפתוח כאן חשבון, הקלידו את הסוד של המשפחה.
register-finish-lede-email = השירות אישר אתכם כ-{ $email }. כדי לפתוח כאן חשבון, הקלידו את הסוד של המשפחה.
sso-continue = המשך
sso-continue-with = המשך עם { $name }
sso-error-title = ההתחברות לא הושלמה
sso-account-disabled = החשבון שמקושר להתחברות הזאת מושבת. פנו למנהל.
sso-no-account = אין כאן חשבון שתואם להתחברות הזאת, והאתר הזה אינו פותח חשבונות בעצמו. בקשו ממנהל ליצור את החשבון שלכם עם הכתובת הזאת: ההתחברות הבאה שלכם תתקשר אליו.
sso-domain-refused = כאן אפשר לפתוח חשבונות רק עם כתובת בדומיינים של המשפחה, שאושרה על ידי השירות.

overview-accounts-note = { $admins ->
        [one] מנהל אחד
        [two] { $admins } מנהלים
        [many] { $admins } מנהלים
       *[other] { $admins } מנהלים
    }
register-password-hint = לפחות { $min } תווים. משפט של כמה מילים קל יותר לזכור מאשר ערבוביה.

theme-clarity = Clarity
theme-clarity-note = בהיר, לבן וכתום · משפך
view-funnel-back = חזרה למשפך
view-funnel-open = פתיחת המשפך
view-funnel-hero-1 = כל דור הוא טבעת.
view-funnel-hero-2 = כל משפחה, משפך.
view-funnel-cta = לחקור את המשפך
view-funnel-cta-detail = כל שם שנוסף וכל מקור שנבדק הופכים את המשפך לנאמן יותר עבור מי שיבואו אחרינו.
view-funnel-see = לראות במשפך
view-funnel-direction = ↓ הוותיקים ביותר בקצה הצר · ↑ החדשים ביותר בפתח
view-funnel-still = עצירת המשפך
view-funnel-name = משפך
view-funnel-legend = איך לקרוא את המשפך
view-funnel-search-hint = Enter מביא אותם לחזית; Shift+Enter פותח את הרשומה שלהם.
theme-genealogy = Genealogy
theme-genealogy-note = ירוק כהה וקרם · עץ לטפס עליו
view-scroll-back = חזרה אל עץ המשפחה
view-scroll-open = פתיחת עץ המשפחה
view-scroll-hero-1 = כל חיים הם דיוקן.
view-scroll-hero-2 = כל משפחה, עץ לטפס עליו.
view-scroll-cta = לחקור את עץ המשפחה
view-scroll-cta-detail = כל שם שנוסף וכל מקור שנבדק הופכים את עץ המשפחה לנאמן יותר עבור הבאים אחרינו.
view-scroll-see = צפייה בעץ המשפחה
view-scroll-direction = ↓ אל השורשים: אבות · ↑ אל הצמרת: צאצאים
view-scroll-still = עצירת העלים
view-scroll-name = עץ משפחה
view-scroll-legend = איך לקרוא את עץ המשפחה
view-scroll-search-hint = Enter מציב אותם על הגזע; Shift+Enter פותח את הרשומה שלהם.
bundle-binding-unnamed = ארכיון ללא שם
bundle-binding-created = ארכיון שנוצר ב-{ $date }
bundle-binding-named = „{ $family }”
bundle-binding-named-created = „{ $family }”, שנוצר ב-{ $date }

notes-section-title = הערות ותרומות
notes-section-intro = מה שחברי המשפחה כתבו: זיכרונות, מחקר, מכתבים, מסמכים. חפשו בהם, עיינו בהם לפי אות, או פתחו אחד כדי לקרוא אותו ולשמור אותו כ-PDF.
notes-search-label = חיפוש בהערות
notes-search-go = חיפוש
notes-all = הכול
notes-letters-label = הערות לפי האות הראשונה של הכותרת
notes-shown = מוצגות: { $shown } מתוך { $total }
notes-none = עדיין לא נכתבה הערה.
notes-none-found = אף הערה אינה תואמת את החיפוש הזה.
notes-write = כתיבת הערה
notes-family-wide = כל המשפחה
notes-by = מאת { $author }
notes-about = על: { $names }
notes-about-prefix = על
notes-about-family = על כל המשפחה
notes-pdf = הורדה כ-PDF
notes-edit = עריכה
notes-close = סגירה
notes-back = חזרה להערות
notes-add-about = כתיבת הערה על האדם הזה
notes-sign-in = ההערות והתרומות של המשפחה מיועדות לחבריה. התחברו כדי לקרוא אותן.
notes-admin-title = הערות ותרומות
notes-admin-intro = כתבו את מה שהרשומות אינן יכולות להכיל: זיכרון, מחקר, מכתב, עם תמונות. הערה שמקושרת לאנשים מופיעה ברשומות שלהם; הערה שאינה מקושרת לאיש שייכת לכל המשפחה. כל חבר קורא את כולן בדף „משפחה”.
notes-deleted = ההערה נמחקה.
notes-col-title = כותרת
notes-col-category = קטגוריה
notes-col-date = תאריך
notes-col-about = על
notes-col-author = מחבר
notes-col-actions = פעולות
notes-new-title = הערה חדשה
notes-edit-title = עריכת ההערה
notes-section-about = ההערה
notes-field-title = כותרת
notes-field-category = קטגוריה
notes-field-category-hint = טקסט חופשי; הקטגוריות שכבר בשימוש מוצעות.
notes-field-date = תאריך
notes-field-date-hint = שנה, חודש או יום: 1952, ‏1952-06 או 1952-06-14. השאירו ריק אם להערה אין תאריך.
notes-field-people = אנשים שנוגעים בדבר
notes-field-people-hint = הקלידו שם ובחרו אותו מהרשימה. אם איש אינו מצוין, ההערה שייכת לכל המשפחה.
notes-person-add = הוספה
notes-person-remove = הסרת האדם הזה
notes-field-text = טקסט
notes-text-hint = הדגשה, נטוי, כותרות, רשימות וקישורים נכתבים ב-Markdown; הכפתורים שלמעלה כותבים אותם בשבילכם. הדביקו או גררו תמונה לתוך הטקסט כדי להוסיף אותה, עד { $mb } MB כל אחת.
notes-tool-bold = מודגש
notes-tool-italic = נטוי
notes-tool-heading = כותרת
notes-tool-list = רשימה
notes-tool-quote = ציטוט
notes-tool-link = קישור
notes-tool-image = הוספת תמונה
notes-tool-preview = תצוגה מקדימה
notes-tool-write = חזרה לכתיבה
notes-uploading = מוסיף את התמונה…
notes-save = שמירת ההערה
notes-cancel = ביטול
notes-delete = מחיקת ההערה הזאת
notes-delete-confirm = ההערה והתמונות שלה יוסרו מהארכיון.
notes-category-suggestions = זיכרון, מחקר, מכתב, סיפור, מתכון, מסמך
notes-error-title = להערה צריכה להיות כותרת. שום דבר לא נשמר.
notes-error-title-long = הכותרת ארוכה מדי: 200 תווים לכל היותר. שום דבר לא נשמר.
notes-error-text-long = הטקסט ארוך מדי. שום דבר לא נשמר.
notes-error-date = התאריך הזה לא מובן. כתבו שנה, חודש או יום: 1952, ‏1952-06 או 1952-06-14. שום דבר לא נשמר.
notes-error-person = אנשים שנוגעים בדבר: „{ $name }”.
notes-error-image-gone = תמונה שהודבקה כבר לא המתינה לשמירה; תמונות ממתינות שש שעות. הוסיפו אותה שוב. שום דבר לא נשמר.
notes-error-too-many-images = הערה מכילה 40 תמונות לכל היותר. שום דבר לא נשמר.
notes-error-stale = מישהו שינה את ההערה הזאת בזמן שעבדתם עליה. פתחו אותה שוב כדי לראות את הגרסה שלו, ואז בצעו את השינוי שלכם. שום דבר לא נשמר.
notes-error-refused = הארכיון סירב לשינוי הזה. שום דבר לא נשמר.
notes-error-image = הקובץ הזה אינו תמונה שהאתר הזה יכול להציג: PNG, ‏JPEG, ‏GIF או WebP.
notes-error-image-large = התמונה הזאת גדולה מדי.
notes-stale-title = ההערה לא שונתה
notes-missing-title = ההערה לא נמצאה
notes-missing = אין כאן הערה, או שאינכם רשאים לקרוא אותה.
notes-not-yours = תורם משנה רק את ההערות שהוא כתב. מנהל יכול לשנות את ההערה הזאת.
notes-pdf-failed-title = לא ניתן היה ליצור את ה-PDF
notes-pdf-failed = משהו השתבש ביצירת ה-PDF של ההערה הזאת. בינתיים קראו אותה בדף שלה, וספרו למנהל.
notes-dashboard-count = הערות שנכתבו: { $n }
notes-dashboard-open = כתיבה וניהול של ההערות

kind-bundle = ארכיון
manifest-title = המשפחה
manifest-intro = מה שהארכיון אומר על המשפחה שהוא מחזיק. השם שלו הוא השם שמוצג בדף „משפחה”, בכותרות הדפים וב-PDF של כל הערה; השאר נוסע עם הקובץ. המספרים והתאריכים של הקובץ עצמו נספרים כשהוא נכתב.
manifest-name = שם המשפחה
manifest-name-hint = מוצג בדף „משפחה” ובכותרות. החשבונות עוקבים אחרי השם החדש.
manifest-description = תיאור
manifest-culture = התרבות העיקרית
manifest-culture-hint = תג שפה: he, fr, pt-BR…
manifest-place = המקום העיקרי
manifest-place-hint = היבשת שבה המשפחה חיה בעיקר.

manifest-span = התקופה המכוסה
manifest-span-hint = מהלידה המוקדמת ביותר ועד המאוחרת ביותר שהארכיון רושם; נספרת, ונכתבת לקובץ כשהפרטים נשמרים.
manifest-license = רישיון
manifest-license-type = רישיון
manifest-save = שמירת פרטי המשפחה
manifest-saved = פרטי המשפחה נשמרו. החשבונות עוקבים אחרי שמה.
manifest-error-long = שדה ארוך מדי: 120 תווים לשם, 2,000 לתיאור. שום דבר לא נשמר.
manifest-error-culture = התרבות היא תג שפה, למשל he, fr או pt-BR. שום דבר לא נשמר.
manifest-error-stale = מישהו שינה את פרטי המשפחה בזמן שערכתם אותם. הגרסה שלכם למטה: שמירה שלה שוב מחליפה את שלו. שום דבר לא נשמר.
manifest-error-io = לא ניתן היה לכתוב את הארכיון; היומן של השרת מסביר למה. שום דבר לא נשמר.

manifest-none = לא צוין
manifest-span-value = { $earliest } – { $latest }
manifest-span-none = עדיין לא נרשם תאריך לידה.
manifest-license-pick = בחרו רישיון כדי לראות מה הוא מתיר לאחרים לעשות בארכיון.
manifest-license-author = מחבר
manifest-license-author-value = { $who }, { $at }
manifest-license-author-none = נרשם כשנבחר רישיון.
manifest-error-place = בחרו את המקום העיקרי מהרשימה. שום דבר לא נשמר.
manifest-error-licence = בחרו את הרישיון מהרשימה. שום דבר לא נשמר.
continent-africa = אפריקה
continent-antarctica = אנטארקטיקה
continent-asia = אסיה
continent-europe = אירופה
continent-north-america = אמריקה הצפונית
continent-south-america = אמריקה הדרומית
continent-oceania = אוקיאניה
continent-several = כמה יבשות
licence-all-rights-reserved-name = כל הזכויות שמורות
licence-all-rights-reserved-about = איש אינו רשאי להעתיק, לשתף או לעשות שימוש חוזר בארכיון בלי רשות המשפחה. זה מה שחל כשלא נאמר דבר.
licence-family-private-name = פרטי, למשפחה
licence-family-private-about = מיועד לחברי המשפחה בלבד: לא לפרסום ולא להעברה מחוץ למשפחה.
licence-cc0-name = CC0 1.0 — נחלת הכלל
licence-cc0-about = כל אחד רשאי להעתיק, לשנות ולהשתמש בארכיון לכל מטרה, גם מסחרית, בלי לבקש רשות ובלי לציין איש.
licence-cc-by-name = CC BY 4.0 — ייחוס
licence-cc-by-about = כל אחד רשאי לשתף ולעבד את הארכיון, גם מסחרית, ובלבד שיציין את מחבריו ויאמר מה שינה.
licence-cc-by-sa-name = CC BY-SA 4.0 — ייחוס, שיתוף זהה
licence-cc-by-sa-about = כל אחד רשאי לשתף ולעבד אותו, גם מסחרית, תוך ציון מחבריו; מה שייעשה ממנו חייב להיות משותף באותו רישיון.
licence-cc-by-nc-name = CC BY-NC 4.0 — ייחוס, שימוש לא מסחרי
licence-cc-by-nc-about = כל אחד רשאי לשתף ולעבד אותו, תוך ציון מחבריו, אך לא למטרות מסחריות.
licence-cc-by-nc-sa-name = CC BY-NC-SA 4.0 — ייחוס, שימוש לא מסחרי, שיתוף זהה
licence-cc-by-nc-sa-about = כל אחד רשאי לשתף ולעבד אותו, תוך ציון מחבריו, לא למטרות מסחריות, ורק באותו רישיון.
licence-cc-by-nd-name = CC BY-ND 4.0 — ייחוס, ללא יצירות נגזרות
licence-cc-by-nd-about = כל אחד רשאי לשתף אותו כפי שהוא, גם מסחרית, תוך ציון מחבריו, אך אינו רשאי לשתף גרסה ששונתה.
licence-cc-by-nc-nd-name = CC BY-NC-ND 4.0 — ייחוס, שימוש לא מסחרי, ללא יצירות נגזרות
licence-cc-by-nc-nd-about = כל אחד רשאי לשתף אותו כפי שהוא, תוך ציון מחבריו, אך לא למטרות מסחריות ולא בגרסה ששונתה: רישיון ה-Creative Commons המחמיר ביותר.
licence-odbl-name = ODbL 1.0 — רישיון מסד נתונים פתוח
licence-odbl-about = נועד למסדי נתונים: כל אחד רשאי להשתמש בנתונים, לשתף ולעבד אותם, תוך ציון המקור, ומסד נתונים שנעשה מהם חייב להישאר פתוח באותם תנאים.

nav-account = החשבון שלי
account-title = החשבון שלי
account-who = החשבון
account-email = כתובת דוא״ל
account-since = מאז
account-person-title = הרשומה שלי בעץ
account-person-intro = ציינו מי מהאנשים בעץ אתם. איש אינו בודק זאת: כל חשבון בוחר את הרשומה שלו, באחריותו, ורשאי לשנות או להסיר אותה בכל עת.
account-person-is = החשבון הזה אומר שהוא
account-person-unlink = הסרת הקישור
account-person-pick = אדם בעץ
account-person-link = קישור החשבון שלי לרשומה הזאת
account-person-change = קישור לרשומה הזאת במקום
account-person-linked = החשבון שלכם מקושר לרשומה שלו.
account-person-unlinked = החשבון שלכם כבר אינו מקושר לרשומה.
account-person-error = לא ניתן היה לקשר את הרשומה הזאת:
account-person-io = לא ניתן היה לשמור את החשבון; היומן של השרת מסביר למה. שום דבר לא שונה.
account-emergency-page = אסימון החירום אינו חשבון: אין לו דף משלו ואין לו רשומה בעץ.
accounts-person = רשומה:

## The merge screen (src/routes/merge.rs): two records a person says are one.
merge-action = איחוד
merge-needs-two = סמנו בדיוק שתי רשומות כדי לאחד אותן.
merge-title = איחוד שתי רשומות
merge-lede = שום דבר לא משתנה עד שתאשרו. הרשומה הראשונה נשמרת; השנייה מאוחדת לתוכה ונשמרת כפי שהייתה ביומן השינויים.
merge-keep = נשמרת
merge-absorb = מאוחדת לתוכה
merge-swap = לשמור את האחרת
merge-refused = האיחוד הזה נדחה:
merge-refusal-ancestor = אחת היא אב קדמון של האחרת
merge-refusal-spouse-and-child = באותה משפחה אחת היא בן זוג והאחרת ילד
merge-refusal-spouses = אלה שני בני הזוג של אותו קשר
merge-refusal-siblings = הן רשומות כילדים של אותם הורים
merge-refusal-different-couples = שתי המשפחות הן זוגות שונים
merge-refusal-child-is-spouse = ילד של משפחה אחת הוא בן זוג באחרת
merge-refusal-same-record = זו אותה רשומה פעמיים
merge-conflicts = במה הן שונות
merge-nothing = אין מה לבחור: הן לא סותרות זו את זו.
merge-choice-keep = הרשומה שנשמרת
merge-choice-absorb = הרשומה האחרת
merge-choice-both = שתיהן
merge-because-higher-confidence = הצעה: הערך בדרגת הוודאות הגבוהה יותר.
merge-because-sourced = הצעה: הערך שמפנה למקור.
merge-because-kept-record = הצעה: הערך של הרשומה שנשמרת.
merge-because-more-restrictive = הצעה: הנראות המגבילה יותר.
merge-because-protective = הצעה: בחיים — הקריאה שמגינה.
merge-because-both-kept = הצעה: לשמור את שני הטקסטים.
merge-becomes-alias = השם שלא נבחר נשמר כשם נוסף.
merge-becomes-event = הערך שלא נבחר נשמר כאירוע.
merge-agreements = במה הן מסכימות
merge-from-keep = מהרשומה שנשמרת
merge-from-absorb = מהרשומה האחרת
merge-from-both = זהה בשתיהן
merge-from-joined = רשימות אוחדו
merge-from-combined = שולבו
merge-effects = מה עוד משתנה
merge-effect-references = הפניות שנכתבו מחדש
merge-effect-self-links = קישורים לעצמה שהוסרו
merge-effect-listed-twice = רשימות שהיו מזכירות אותה פעמיים
merge-effect-same-couple = משפחות של אותו זוג
merge-confirm-label = ייתכן שבכל זאת מדובר באדם אחד. כדי לאחד בכל זאת, הקלידו את השם בדיוק: { $name }
merge-go = איחוד
merge-missing = אחת משתי הרשומות כבר לא קיימת.
merge-done-title = אוחדו
merge-done-lede = { $absorbed } הוא כעת חלק מ־{ $kept }.
merge-report-references = הפניות שנכתבו מחדש
merge-report-events = אירועים שנוצרו מערך שלא נבחר
merge-report-source-conflicts = אי־הסכמות שנרשמו על מקור
merge-report-links = קישורים שהוסרו
merge-report-tidied = רשומות שסודרו
merge-absorbed-record = הרשומה שאוחדה, כפי שהייתה
merge-open-kept = פתיחת הרשומה שנשמרה
