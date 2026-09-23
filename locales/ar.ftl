# axgf-cms — نصوص الواجهة، العربية.
#
# جودة آلية — لم تراجعها متحدثة أو متحدث بالعربية لغةً أُولى. ومفردات علم
# الأنساب على وجه الخصوص لها مقابلات مستقرة تختلف باختلاف تقاليد حفظ
# السجلات، وقد تكون الترجمة هنا خاطئة. التصويبات مرحّب بها — انظر
# CONTRIBUTING.md.
#
# المقابلات المعتمدة (وهي موضع نقاش لمن العربية لغته الأولى):
#   union → اقتران · link → صلة · confidence → درجة الثقة
#   reliability → موثوقية · source → مصدر · primary source → مصدر أوّلي
#   occupation → مهنة · record → سجل · archive → أرشيف
#   godparent → عرّاب · witness → شاهد · speculative → مُفترَض
#
# للعربية ست صيغ عددية في قواعد CLDR: zero وone وtwo وfew وmany وother.
# وهي مذكورة كاملة أدناه؛ فرضُ منطق «واحد أو أكثر» الإنجليزي عليها خطأ.
#
# الأرقام: تُستعمل الأرقام العربية الغربية (0–9) في كل النصوص، لا الأرقام
# العربية المشرقية (٠–٩). والسبب أن الأعداد التي تأتي من الأرشيف — السنوات
# والمعرّفات والنسب المئوية — تُعرض بالأرقام الغربية، فاختيار المشرقية هنا
# كان سيضع نظامَي ترقيم في السطر الواحد.
#
# التواريخ: «12 أبريل 1923». وجدول الشهور موضوع داخل نمط التاريخ نفسه.
#
# القاعدة: هذا الملف يترجم الواجهة وحدها. الأسماء والأماكن والملاحظات والمهن
# تأتي من الأرشيف وتبقى بلغتها وخطّها.

app-name = ax-genealogy

nav-tree = الشجرة
nav-admin = الإدارة
nav-sign-in = تسجيل الدخول
nav-sign-out = تسجيل الخروج

settings-title = الإعدادات
settings-tabs-label = أقسام الإعدادات
settings-tab-theme = السمة
settings-tab-language = اللغة
settings-tab-appearance = المظهر
settings-done = تم
prefs-language = اللغة
prefs-theme = السمة
prefs-background = الخلفية
prefs-background-on = تدرّج لوني خفيف خلف الصفحة
prefs-apply = تطبيق
prefs-reviewed = مُراجَعة
prefs-machine = آلية، { $coverage }٪

theme-light = فاتح
theme-dark = داكن
theme-system = حسب إعداد النظام
theme-high-contrast = تباين عالٍ
theme-sepia = بنّي داكن
theme-deuteranopia = عمى اللون الأخضر
theme-protanopia = عمى اللون الأحمر
theme-tritanopia = عمى اللون الأزرق
theme-colour-blind-note = ملائم لعمى الألوان
theme-contrast-note = أقصى تباين

tree-title-around = حول { $name }
tree-title-whole = الشجرة كاملة
tree-lede-focused = { $ancestors ->
        [zero] لا أسلاف
        [one] سلف واحد
        [two] سلفان
        [few] { $ancestors } أسلاف
        [many] { $ancestors } سلفًا
       *[other] { $ancestors } سلف
    }، و{ $descendants ->
        [zero] لا أحفاد
        [one] حفيد واحد
        [two] حفيدان
        [few] { $descendants } أحفاد
        [many] { $descendants } حفيدًا
       *[other] { $descendants } حفيد
    }، و{ $spouses ->
        [zero] لا أزواج
        [one] زوج واحد
        [two] زوجان
        [few] { $spouses } أزواج
        [many] { $spouses } زوجًا
       *[other] { $spouses } زوج
    }، { $depth } أجيال في كل اتجاه.
tree-filter-label = تصفية البطاقات الظاهرة
tree-filter-placeholder = اكتب اسمًا…
tree-centre-on = التمركز حول
tree-depth = أجيال في كل اتجاه
tree-show = إظهار
tree-hidden-notice = { $n ->
        [zero] لا أحد مخفي
        [one] شخص واحد يظهر دون تفاصيله
        [two] شخصان يظهران دون تفاصيلهما
        [few] { $n } أشخاص يظهرون دون تفاصيلهم
        [many] { $n } شخصًا يظهرون دون تفاصيلهم
       *[other] { $n } شخص يظهرون دون تفاصيلهم
    }
tree-hidden-because-role = ، لأن مستوى ظهورهم يفوق ما يسمح حسابك بقراءته.
tree-hidden-because-anonymous = ، لأنهم ليسوا علنيين.
tree-hidden-sign-in = سجّل الدخول إن كان لديك حساب.
tree-restricted-card = سجل هذا الشخص غير مرئي لك
tree-empty = لا يحتوي هذا الملف على أحد لرسمه.
tree-unplaced = ليس ضمن أي أسرة مسجّلة

record-identity = الهوية
record-life-events = أحداث الحياة
record-family = الأسرة
record-other-relationships = علاقات أخرى
record-occupations = المهن
record-places = الأماكن
record-sources-documents = المصادر والوثائق
record-notes = ملاحظات
record-history = سجل التعديلات
record-raw = السجل الخام
record-raw-summary-note = ملف JSON الذي بُنيت منه هذه الصفحة
record-sources-documents-help = كل مصدر يسمّي ما في هذه الصفحة من وقائع تستند إليه، مرتّبة بحسب قوة الدليل.
record-notes-help = ملاحظات على هذا السجل، ومنها نص عجز كل محوّل عن تفسيره فحُفظ بحرفه بدلًا من إسقاطه.
record-help-toggle = ما يعرضه هذا القسم

record-gender = النوع
record-living = على قيد الحياة
record-visibility = الظهور
record-yes = نعم
record-no = لا
record-name-type = نوع الاسم
record-name-used = فترة الاستعمال
record-name-evidence = الدليل
record-transliteration = النقحرة اللاتينية
record-born = وُلد
record-died = تُوفّي
record-parents = الوالدان
record-siblings = الإخوة والأخوات
record-children = الأبناء
record-unknown-person = [غير معروف]
record-restricted-person = خاص
record-restricted-title = سجل هذا الشخص غير مرئي لك
record-absent-person-title = مذكور في هذا الملف لكنه غير موجود فيه
record-confidence = درجة اليقين
record-source = المصدر
record-download = تنزيل

access-restricted-title = غير مرئي لك
access-restricted-anonymous = هذا السجل ليس علنيًا. سجّل الدخول لتعرف إن كان حسابك يستطيع قراءته.
access-role-title = ليس لدورك
access-role-write = يستطيع حسابك قراءة هذا الملف لا تغييره. بإمكان المدير رفع دورك إلى مساهم.
access-scope-title = خارج فرعك

error-not-found-title = غير موجود
error-not-found-detail = هذه الصفحة غير موجودة في هذا الملف.
error-no-such-person-title = لا يوجد هذا الشخص
error-no-such-person-detail = لا يحتوي هذا الملف على شخص بهذا المعرّف.
error-no-such-entity-title = لا يوجد هذا السجل
error-no-such-entity-detail = لا يحتوي هذا الملف على سجل بهذا المعرّف.
error-deleted-while-editing = لا يحتوي هذا الملف على سجل بهذا المعرّف. ربما حُذف أثناء تحريرك له.
error-no-such-file-title = لا يوجد هذا الملف
error-not-an-image-title = ليست صورة
error-not-an-image-detail = لا توجد مصغّرة لهذه الوثيقة، لأنها ليست صورة يستطيع هذا الإصدار فكّ ترميزها.
error-back = رجوع

login-title = تسجيل الدخول
login-lede = ينشئ الحسابات مديرٌ.
login-username = اسم المستخدم
login-password = كلمة المرور
login-submit = تسجيل الدخول
login-wrong = اسم المستخدم وكلمة المرور غير متطابقين.
login-token-wrong = هذا الرمز غير صحيح.
login-throttled = محاولات فاشلة كثيرة. انتظر بضع دقائق ثم أعد المحاولة.
login-no-accounts-title = لا حسابات في هذا التنصيب بعد.
login-emergency-summary = وصول الطوارئ
login-emergency-label = رمز الطوارئ
login-emergency-submit = استخدم رمز الطوارئ
login-sign-in-prompt = سجّل الدخول للوصول إلى لوحة الإدارة.

admin-title = الإدارة
admin-entities = السجلات
admin-create = إنشاء
admin-new-kind = جديد: { $kind }
admin-operations = العمليات
admin-validate = تحقّق
admin-deduplicate = دمج المكرّرات
admin-export = تصدير الملف
admin-accounts = الحسابات
admin-dedup-confirm = دمج المكرّرات يدمج السجلات ويعيد كتابة الملف. أتريد المتابعة؟
admin-recent-changes = أحدث التغييرات
admin-sessions-open = { $n ->
        [zero] لا جلسات مفتوحة الآن.
        [one] جلسة واحدة مفتوحة الآن.
        [two] جلستان مفتوحتان الآن.
        [few] { $n } جلسات مفتوحة الآن.
        [many] { $n } جلسة مفتوحة الآن.
       *[other] { $n } جلسة مفتوحة الآن.
    }
admin-no-changes-yet = لم يُغيَّر شيء بعد عبر هذا التطبيق. كل حفظ من الآن يُسجَّل في { $path }.
admin-last-validation = آخر تحقّق
admin-fields = الحقول
admin-raw-json = JSON الخام
admin-save = حفظ
admin-cancel = إلغاء
place-editor-title = تحرير مكان
place-add-detail = أضف تفاصيل لهذا المكان
place-names = الأسماء
place-name-primary = الاسم الأساسي
place-name-lang = اللغة
place-name-value = الاسم
place-names-hint = سطر لكل اسم مسجَّل. المكان الذي حكمته ثلاث إمبراطوريات يحمل ثلاثة أسماء؛ والرئيسي هو الظاهر في كل مكان آخر.
place-where = الموقع
place-type = النوع
place-region = المنطقة
place-country-current = الدولة اليوم
place-country-hint = ISO 3166-1 alpha-2، مثل PL وFR وDE.
place-country-history = تاريخ الحدود
place-history-country = الدولة
place-history-from = من
place-history-until = حتى
place-country-history-hint = أي دولة ملكت هذا المكان وفي أي مدة. وهذا مهم في الأنساب: سجل كُتب بالروسية سنة 1880 وآخر كُتب بالبولندية سنة 1930 قد يسمّيان القرية نفسها.
place-coordinates = الإحداثيات
place-lat = خط العرض
place-lon = خط الطول
place-precision = الدقة
place-identifiers = المعرّفات
place-wikidata = ويكي بيانات
place-geonames = GeoNames
place-used-by = يُستخدم { $name } في { $n ->
        [zero] لا سجلات أخرى
        [one] سجل آخر واحد
        [two] سجلين آخرين
        [few] { $n } سجلات أخرى
        [many] { $n } سجلًا آخر
       *[other] { $n } سجل آخر
    }.
place-error-no-name = يحتاج المكان إلى اسم واحد على الأقل.
place-error-coords-pair = خط العرض وخط الطول متلازمان: اذكرهما معًا أو لا تذكر أيًّا منهما.
place-error-coords-number = يجب أن يكون خط العرض وخط الطول رقمين.
place-error-coords-range = خط العرض من ‎-90 إلى 90، وخط الطول من ‎-180 إلى 180.
place-type-continent = قارة
place-type-country = دولة
place-type-region = منطقة
place-type-department = مقاطعة
place-type-city = مدينة
place-type-village = قرية
place-type-district = حيّ
place-type-street = شارع
place-type-building = مبنى
place-type-farm = مزرعة
place-type-island = جزيرة
place-type-historical = تاريخي
place-type-unknown = غير معروف
place-precision-exact = دقيق
place-precision-building = مبنى
place-precision-street = شارع
place-precision-city_center = وسط المدينة
place-precision-region_center = وسط المنطقة
place-precision-country_center = وسط البلد
place-precision-approximate = تقريبي

place-coordinates-hint = الإدخال اليدوي هو الطريق المعتاد. كثير من الأماكن المسجَّلة تحت إدارة سابقة لا يعثر عليها البحث الحديث أصلًا.
place-geocode-search = ابحث عن هذا الاسم
place-geocode-hint = يرسل الاسم والمنطقة والبلد إلى خدمة الترميز الجغرافي، مكانًا واحدًا في كل مرة. لا يُحفظ شيء حتى تحفظ أنت.
place-geocode-off = البحث عن الأسماء متوقف. يحتاج إلى عنوان اتصال تتعرف به الخدمة على هذا التنصيب؛ شغِّل الخادم مع ‎--geocoder-contact‎ لتشغيله.
place-geocode-query = بُحث عن: { $q }
place-geocode-error = تعذّر الوصول إلى خدمة البحث. حقول الإحداثيات أعلاه ما زالت تعمل.
place-geocode-none = لم يُعثر على شيء. أما القرية المسجَّلة تحت الإدارة الروسية أو البروسية أو النمساوية فهذه نتيجتها المعتادة؛ أدخل الموضع يدويًا.
place-geocode-not-a-place = ليس مكانًا مأهولًا
place-geocode-use = استخدم هذا
place-geocode-attribution = نتائج من OpenStreetMap عبر Nominatim، بترخيص Open Database.

place-paste = ألصق موضعًا
place-paste-placeholder = رابط خريطة، أو 52.0782795, 21.2508068
place-paste-read = اقرأه
place-paste-hint = رابط من Google Maps أو OpenStreetMap، أو معرّف geo:، أو زوج أرقام بسيط، أو درجات ودقائق وثوانٍ مثل 52°04'41.8"N 21°15'02.9"E.
place-paste-read-ok = قُرئ إلى الحقول أعلاه. تحقق منه ثم احفظ.
place-paste-unreadable = ليس هذا موضعًا يمكن قراءته هنا. ما زالت الحقول أعلاه تقبل زوجًا بسيطًا من الأرقام.

place-map-hint = انقر على الخريطة لوضع النقطة، أو اسحب الدبوس. الحقول أعلاه هي السجل.
place-map-clear = أزل النقطة
place-open-in-map = ابحث عن هذا المكان في OpenStreetMap ثم ألصق الرابط هنا

person-tab-record = السجل
person-tab-life = الحياة
person-tab-media = المواد
person-tab-tree = الشجرة
person-tab-history = السجل
person-tree-depth = { $n } أجيال في كل اتجاه. الشجرة كاملةً في الأسفل.
person-tree-alone = لا يسمّي هذا السجل والدين ولا أزواجًا ولا أبناء، فليس حوله شكل يُرسم.

record-no-evidence = لا شيء مرفق بهذا السجل — لا مصدر ولا وثيقة. هذه هي الحال المعتادة لملف محوَّل، لا عيب فيه: يحمل GEDCOM الوقائع ويترك خلفه ما كان يثبتها.
record-no-evidence-signed-out = سجّل الدخول لإرفاق شيء.
admin-delete = حذف
admin-not-set = — غير محدَّد —
admin-edit = تحرير
admin-page-of = صفحة { $page } من { $pages }
admin-previous = السابق
admin-next = التالي
admin-saved = حُفظ بوصفه الإصدار { $version } — { $summary }
admin-not-saved = لم يُحفظ
admin-created = أُنشئ
admin-not-created = لم يُنشأ
admin-deleted = حُذف
admin-not-deleted = لم يُحذف — الملف كما هو
admin-what-changed = ما الذي تغيّر
admin-field = الحقل
admin-from = من
admin-to = إلى
admin-version = الإصدار { $version }

accounts-title = الحسابات
accounts-existing = الموجودة
accounts-username = اسم المستخدم
accounts-role = الدور
accounts-status = الحالة
accounts-branch = الفرع
accounts-last-seen = آخر ظهور
accounts-change = تغيير
accounts-you = (أنت)
accounts-active = نشط
accounts-disabled = معطّل
accounts-never = أبدًا
accounts-whole-tree = الشجرة كاملة
accounts-roots = { $n ->
        [zero] لا جذور
        [one] جذر واحد
        [two] جذران
        [few] { $n } جذور
        [many] { $n } جذرًا
       *[other] { $n } جذر
    }
accounts-add = إضافة حساب
accounts-password-hint = اتركها فارغة فتُولَّد وتُعرض مرة واحدة. وإن حدّدتها بنفسك فلا تقل عن { $min } محرفًا.
accounts-new-password-placeholder = كلمة مرور جديدة (فارغ = دون تغيير)
accounts-email = البريد الإلكتروني
accounts-optional = (اختياري)
accounts-create = أنشئ الحساب
accounts-role-viewer = قارئ — يقرأ السجلات العلنية وسجلات الأعضاء
accounts-role-contributor = مساهم — وينشئ ويحرّر ويرفع كذلك
accounts-role-admin = مدير — ويدير الحسابات ويحذف ويصدّر كذلك
accounts-branch-placeholder = معرّف شخص واحد في كل سطر
accounts-ids-in-bundle = معرّفات الأشخاص في هذا الملف
accounts-created = أُنشئ { $username }.
accounts-updated = حُدِّث { $username }. وأُنهيت كل جلسة كانت مفتوحة له.
accounts-username-taken = اسم المستخدم هذا مأخوذ.
accounts-pick-role = اختر دورًا.
accounts-no-such = لا يوجد هذا الحساب.
accounts-not-saved = لم يُحفظ: { $error }

conflict-title = غيّره شخص آخر قبلك
conflict-versions = بدأت من الإصدار { $expected }، والملف الآن عند الإصدار { $current }.
conflict-both-changed = غيّرتما كلاكما هذه الحقول
conflict-both-changed-detail = هذه الحقول حرّرتماها كلاكما. وما تحفظه سيحل محل ما وضعه { $who }:
conflict-field-by-field = حقلًا حقلًا
conflict-theirs = ما غيّره إليه { $who }
conflict-yours = ما غيّرته إليه أنت
conflict-unchanged-by-you = لم تغيّره أنت
conflict-unchanged-by-them = لم يغيّروه هم
conflict-what-now = ماذا الآن
conflict-reapply = أعِد تطبيق نسختك فوق نسختهم
conflict-save-over = احفظ هذه بدل نسختهم
conflict-discard = اطرح نسختي وابدأ من جديد
conflict-their-version = نسخة { $who } كما يحملها الملف الآن
conflict-history-of = سجل تعديلات هذا السجل ({ $kind })

## Dates

date-unknown = التاريخ غير معروف
date-not-recorded = غير مسجَّل
date-circa = نحو { $date }
date-between = بين { $from } و{ $to }
date-before = قبل { $date }
date-after = بعد { $date }
date-preserved = مسجَّل هكذا: «{ $text }»
date-day-month-year = { $day } { $month ->
        [1] يناير
        [2] فبراير
        [3] مارس
        [4] أبريل
        [5] مايو
        [6] يونيو
        [7] يوليو
        [8] أغسطس
        [9] سبتمبر
        [10] أكتوبر
        [11] نوفمبر
        [12] ديسمبر
        *[other] { $month }
    } { $year }
date-month-year = { $month ->
        [1] يناير
        [2] فبراير
        [3] مارس
        [4] أبريل
        [5] مايو
        [6] يونيو
        [7] يوليو
        [8] أغسطس
        [9] سبتمبر
        [10] أكتوبر
        [11] نوفمبر
        [12] ديسمبر
        *[other] { $month }
    } { $year }
date-decade = عقد { $decade }
date-century = القرن { $century }

## بطاقات الشجرة

tree-band-generation = الجيل { $g }
tree-band-people = { $n ->
        [zero] لا أحد
        [one] شخص واحد
        [two] شخصان
        [few] { $n } أشخاص
        [many] { $n } شخصًا
       *[other] { $n } شخص
    }
tree-band-unplaced = بلا موضع
tree-band-unplaced-note = { $n ->
        [zero] لا أحد خارج الأسر
        [one] شخص واحد لا ينتمي إلى أسرة — معروض بدل أن يُحذف
        [two] شخصان لا ينتميان إلى أسرة — معروضان بدل أن يُحذفا
        [few] { $n } أشخاص لا ينتمون إلى أسرة — معروضون بدل أن يُحذفوا
        [many] { $n } شخصًا لا ينتمون إلى أسرة — معروضون بدل أن يُحذفوا
       *[other] { $n } شخص لا ينتمون إلى أسرة — معروضون بدل أن يُحذفوا
    }

## بقية نصوص الشجرة

tree-show-all = عرض الكل ({ $n })
tree-back-to-focused = العودة إلى العرض المركّز
tree-confidence-label = درجة اليقين:
tree-band-certain = مؤكَّد
tree-band-high = عالٍ
tree-band-medium = متوسط
tree-band-low = ظنّي
tree-counts = { $drawn } من أصل { $total } شخصًا · { $generations ->
        [zero] لا أجيال
        [one] جيل واحد
        [two] جيلان
        [few] { $generations } أجيال
        [many] { $generations } جيلًا
       *[other] { $generations } جيل
    }
tree-unplaced-count = { $n } بلا موضع
tree-no-people = لا أشخاص في هذا الملف بعد.
tree-nobody-selected = لا أحد لرسمه بهذا الاختيار.
tree-nobody-selected-cta = ابدأ من العرض الافتراضي.
tree-edge-union = اقتران مسجَّل
tree-edge-parentage = بنوّة مسجَّلة
tree-title-suffix = الشجرة

## Vocabulary the structured editors offer

name-part-nasab = نسب
name-part-laqab = لقب
name-part-kunya = كنية
name-part-nisbah = نسبة
name-part-alias = كنية بديلة
name-part-religious_name = اسم ديني
name-part-pen_name = اسم مستعار أدبي
name-type-pen_name = اسم مستعار أدبي
gender-U = غير مسجّل

## بقية نصوص السجل

record-centre-tree-here = اجعل الشجرة تتمركز هنا
record-centre-tree-title = حرّك الشجرة لتتمركز على هذا الشخص
record-open-full-page = افتح الصفحة الكاملة ↗
record-open-full-title = افتح الصفحة المستقلة القابلة للمشاركة
record-edit = تحرير
record-living-yes = على قيد الحياة
record-deceased = متوفّى
record-display-name = الاسم المعروض
record-read-as = يُقرأ
record-also-recorded-as = مسجَّل أيضًا بوصفه
record-borders-moved = تغيّرت الحدود:
record-note = ملاحظة
record-name = الاسم
record-type = النوع
record-cause = السبب:
record-as = بصفة
record-partner-not-recorded = الزوج غير مسجَّل
record-union-from = من
record-union-at = في
record-union-until = حتى
record-occupation-from = من
record-occupation-until = إلى
record-source-reliability = الموثوقية
record-source-supports = يدعم
record-photographs = الصور
record-documents = الوثائق
record-file = الملف
record-status = الحالة
record-size = الحجم
record-no-file = لا ملف
record-attach-document = إرفاق وثيقة
record-upload = رفع
panel-empty = اختر بطاقة لترى هنا السجل الكامل لذلك الشخص.
panel-selected-person = الشخص المختار
person-see-in-tree = انظر هذا الشخص في الشجرة
person-visibility-inline = الظهور:
person-age-at-death = تُوفّي عن { $n } عامًا
person-age-now = { $n } عامًا
person-born-in = وُلد في { $place }
person-died-in = تُوفّي في { $place }
person-children-count = { $n ->
        [zero] لا أطفال
        [one] طفل واحد
        [two] طفلان
        [few] { $n } أطفال
        [many] { $n } طفلًا
       *[other] { $n } طفل
    }
person-generations-below = { $n ->
        [zero] لا أجيال تحته
        [one] جيل واحد تحته
        [two] جيلان تحته
        [few] { $n } أجيال تحته
        [many] { $n } جيلًا تحته
       *[other] { $n } جيل تحته
    }
person-portrait-of = صورة { $name }
person-no-portrait = لا صورة مسجَّلة
person-sections-label = أقسام هذه الصفحة

## المفردات المضبوطة

gender-M = ذكر
gender-F = أنثى
gender-NB = غير ثنائي
gender-unrecorded = غير مسجَّل

name-part-given_name = الاسم الأول
name-part-family_name = اسم العائلة
name-part-patronymic = اسم الأب
name-part-matronymic = اسم الأم
name-part-middle_name = الاسم الأوسط
name-part-nickname = اللقب
name-part-prefix = سابقة
name-part-suffix = لاحقة
name-part-particle = أداة
name-part-part = جزء

name-type-primary = أساسي
name-type-other = آخر
name-type-alias = كنية
name-type-birth = عند الولادة
name-type-married = بعد الزواج
name-type-religious = ديني
name-type-transliteration = نقحرة
name-type-nickname = لقب

## ملاحظات العرض

note-birth-imprecise = تاريخ ميلاد لم تستطع المصادر تحديده، معروض كما سُجّل
note-death-imprecise = تاريخ وفاة لم تستطع المصادر تحديده، معروض كما سُجّل
note-names = { $n ->
        [zero] لا أسماء مسجَّلة
        [one] اسم واحد مسجَّل
        [two] اسمان مسجَّلان
        [few] { $n } أسماء مسجَّلة
        [many] { $n } اسمًا مسجَّلًا
       *[other] { $n } اسم مسجَّل
    }
note-transliteration = اسم بخطّه الأصلي إلى جانب نقحرته اللاتينية
note-witnessed = { $n ->
        [zero] لا أحداث شهدها
        [one] حدث واحد شهده دون أن يكون صاحبه
        [two] حدثان شهدهما دون أن يكون صاحبهما
        [few] { $n } أحداث شهدها دون أن يكون صاحبها
        [many] { $n } حدثًا شهدها دون أن يكون صاحبها
       *[other] { $n } حدث شهدها دون أن يكون صاحبها
    }

visibility-public = علني
visibility-members = الأعضاء
visibility-contributors = المساهمون
visibility-private = خاص

## الواجهة العامة

nav-convert = استيراد
footer-open-format = أرشيف عائلتك ملف واحد يبقى عندك، مكتوب بصيغة مفتوحة: سيظل يُفتح بعد زوال هذا الموقع بزمن طويل.
footer-open-format-link = عن الصيغة
prefs-machine-complete = كاملة، لم تُراجَع بعد
prefs-machine-title = تُرجمت دون مراجعة ممن العربية لغته الأولى. ومفردات علم الأنساب على وجه الخصوص قد تكون خاطئة: فألفاظ الاقتران والعرّاب والمصدر الأوّلي تختلف باختلاف تقاليد حفظ السجلات في كل بلد. التصويبات مرحّب بها، وملف CONTRIBUTING.md يبيّن من أين تبدأ.

## شروح أقسام السجل

record-identity-help = كل اسم مسجَّل مع نوعه والمدة التي استُعمل فيها والمصدر الذي يسنده، مع الخط الأصلي إلى جانب النقل الحرفي اللاتيني حيث يختلفان، إضافة إلى النوع وحال الحياة ومدى الظهور.
record-life-events-help = الميلاد والوفاة وكل واقعة شارك فيها هذا الشخص، مرتّبة بالتاريخ، كلٌّ مع دوره فيها — فزواجٌ شهده فحسب يقف إلى جانب زواجه هو. والواقعة بلا تاريخ تأتي في الآخر بدل أن تتظاهر بالأسبقية.
life-nothing-recorded = لا حدث ولا مهنة ولا مكان مسجَّل لهذا الشخص. هذا هو المعتاد في ملف محوَّل: يحمل GEDCOM ما دوَّنه أحدهم، ومعظم المدخلات اسم وتاريخ.
life-add-first = سجِّل الحدث الأول
record-family-help = الوالدان والإخوة، ثم كل اقتران بنوعه وتواريخه ومكانه وكيف انتهى وأبنائه بترتيب الميلاد.
record-other-relationships-help = كل صلة يقف هذا الشخص في أحد طرفيها، مقروءة من جهته: السجل نفسه يظهر «عرّابًا لـ» من طرف و«ابنًا بالعمادة لـ» من الطرف الآخر.
record-occupations-help = المهن بوصفها مددًا على محور واحد مشترك، فتُقارَن وظيفتان بالعين؛ وحيث ينقص أحد الطرفين يبقى الشريط مفتوحًا.
record-places-help = كل مكان يمسّه هذا السجل، مع ما جرى فيه ومع تاريخ الحدود الذي يجعل للمكان معنى عبر الزمن.
record-history-help = كل تغيير محفوظ على هذا السجل، الأحدث أولًا. ومن صحّح ماذا حقيقةٌ عن القائمين على الشجرة لا عن العائلة التي فيها، ولذلك يبقى خارج الأرشيف المُصدَّر ولا يُعرض إلا لمن سجّل دخوله من الأقارب.
record-raw-help = ليس هنا شيء مُولَّد للعرض: هذا هو السجل كما هو محفوظ تمامًا، حتى أسماء الحقول. ولو احتجت يومًا إلى قراءة الأرشيف من دون هذا الموقع، فهذا ما سترى.

## الوصول

access-restricted-signed-in = مدى ظهور هذا السجل أعلى مما يسمح حسابك بقراءته. ويستطيع مدير أن يغيّر إما مدى ظهور السجل وإما دورك.
access-role-admin = هذه صفحة إدارة. يستطيع حسابك إنشاء السجلات وتحريرها، لكنه لا يدير الحسابات ولا يحذف السجلات ولا يُصدّر الأرشيف.
access-scope-named = حسابك مقصور على فرع واحد من الشجرة، وهذا السجل يخصّ شخصًا خارجه. وكل شخص يُذكر في سجل يجب أن يكون داخل فرعك — وإلا صارت عائلةٌ فيها شريك من الخارج بابًا لإعادة كتابة نسب ذلك الشخص.
access-scope-unnamed = حسابك مقصور على فرع واحد من الشجرة، وهذا السجل لا يذكر أحدًا يمكن قياسه عليه. أما المصادر والأماكن فتحرّرها حسابات لها وصول إلى الشجرة كلها.

## رسائل الأخطاء

error-no-such-file-detail = لا يوجد هنا مستند بهذا المعرّف، أو أن المستند مسجَّل بلا ملف — فالمستند المُحال إليه يسمّي شيئًا محفوظًا في مكان آخر.
error-back-to-start = العودة إلى البداية
error-payload-missing-title = لا يوجد ملف كهذا
error-payload-missing-detail = محتوى ذلك المستند ليس في الذاكرة المؤقتة.
error-payload-unopenable-detail = تعذّر فتح محتوى ذلك المستند.
error-no-such-document-detail = لا يوجد هنا مستند بهذا المعرّف.
error-bad-preference-title = ليس من الخيارات
error-bad-preference-detail = هذه ليست لغة ولا مظهرًا يقدّمه هذا الموقع. ولم يتغيّر شيء.
error-unknown-kind-title = نوع مجهول
error-unknown-kind-detail = «{ $kind }» ليس نوعًا من السجلات. هذا الأرشيف يضم: { $kinds }.
error-io-title = تعذّر الحفظ
error-io-detail = { $error }. ولم يتغيّر شيء على القرص.
error-upload-too-large = هذا الملف أكبر من حد { $mb } ميغابايت. لم يُحفظ شيء والأرشيف على حاله.
error-upload-refused = رُفض المستند: { $reason }. والأرشيف على حاله.
error-back-to-person = العودة إلى السجل
error-no-such-person-to-attach = لا يوجد هنا شخص بهذا المعرّف، فليس ثمة ما يُرفق به مستند.
error-upload-title = لم يُحفظ ذلك الرفع
error-download-expired-title = انتهت صلاحية هذا التنزيل
error-download-expired-detail = يُحفظ الاستيراد خمس عشرة دقيقة ثم يُتلف. استورد الملف من جديد.
error-upload-none = لم يُرفع أي ملف. اختر ملفًا أولًا.
error-upload-unsupported = هذا النوع من الملفات لا يحفظه الأرشيف. تُقبل الصور وPDF والنص الصِّرف والصوت والفيديو؛ ويُقرأ النوع من بايتات الملف نفسه، فإعادة تسمية برنامج تنفيذي لا تُدخله. أما SVG فمرفوض قطعًا، لأن ملف SVG قد يحمل سكربتًا.
error-export-unreadable-title = تعذّرت قراءة الأرشيف المُصدَّر
error-export-unreadable-detail = { $error }

## تسجيل الدخول والإدارة

login-no-accounts-detail = لا توجد هنا صفحة إعداد عن قصد — فالفترة بين النشر وأول تسجيل دخول هي بعينها اللحظة التي يكون فيها التنصيب بلا حماية، ولذلك يُنشأ أول مدير من سطر الأوامر.
login-no-accounts-note = يطبع كلمة سر مولَّدة على stderr مرة واحدة لا غير. وحتى ذلك الحين، المدخل الوحيد هو الرمز الطارئ أدناه.
login-emergency-detail = ما يزال الرمز المشترك يفتح جلسة مدير، وهو موجود لغرض واحد: العودة إلى الداخل حين يُفقد ملف ‎.acl‎ أو يُحجب جميع المديرين. وهو ليس حسابًا — لا تفضيلات له، وسجل التحرير يقيّده بوصفه emergency-token لا بوصفه شخصًا. واستعماله يُدوَّن تحذيرًا.
admin-lede = تحرير { $path } — { $total } سجلًا، { $files ->
        [zero] بلا ملفات مرفقة
        [one] وملف مرفق واحد
        [two] وملفان مرفقان
        [few] و{ $files } ملفات مرفقة
        [many] و{ $files } ملفًا مرفقًا
       *[other] و{ $files } ملف مرفق
    }، { $size } على القرص. كل تغيير يُكتب دفعة واحدة؛ والتغيير المرفوض يترك الملف كما هو.
admin-roles-note = التحقق وإزالة التكرار والتصدير والحذف وإدارة الحسابات للمدير وحده. أما المساهم فيبلغ كل صفحة أخرى هنا.
admin-recent-note = آخر { $shown } من { $total ->
        [zero] لا تغييرات مسجَّلة
        [one] تغيير مسجَّل واحد
        [two] تغييرين مسجَّلين
        [few] { $total } تغييرات مسجَّلة
        [many] { $total } تغييرًا مسجَّلًا
       *[other] { $total } تغيير مسجَّل
    }، من { $path }.
admin-bundle-heavy = حجم هذا الأرشيف { $size }. ويُقرأ كاملًا عند الإقلاع ويبقى في الذاكرة، فبعد { $warn } تقريبًا يبدأ الموقع يكلّف ذاكرة حقيقية وتبطؤ عمليات إعادة التشغيل. وهذا يناسب أرشيفًا عائليًا لا مكتبة وسائط — فإن كانت المرفقات تنمو بلا حد فاحفظها في مخزن ملفات واجعل الأرشيف يشير إليها.
admin-raw-json-help = السجل كاملًا، فلا شيء يتعذّر تحريره — القوائم مثل شركاء العائلة وأبنائها، أو تاريخ حدود مكان، تعيش هنا. وهذا هو المستند الأصل؛ ثم تُكتب الحقول أعلاه فوق المسارات التي تخصّها، فحرّر القيمة في أحد الموضعين لا في كليهما. ولا بد أن يُقرأ بصيغة JSON وإلا لم يُحفظ شيء.
accounts-lede = محفوظة في { $path }، بأذونات 600، بجوار الأرشيف لا داخله أبدًا. فالأرشيف يُنسخ ويُرسل ويُنشر؛ ولو سافرت بصمات كلمات السر في داخله لصارت كل نسخة من شجرة العائلة نسخةً من بيانات الدخول.
accounts-no-registration = لا تسجيل ذاتيًا ولا دعوات، عن قصد. فأرشيف عائلة يكفيه مدير يعرف الجميع، وفي ذلك إزالةٌ لمساحة إساءة الاستعمال بأكملها بدل الدفاع عنها.
accounts-branch-hint = يقصر ما يستطيع هذا الحساب تحريره على أولئك الأشخاص وذريتهم وأزواجهم.
accounts-branch-reading = ولا يقصر ما يستطيع قراءته — فذاك يحكمه مدى ظهور كل سجل، والأمران مفصولان عن قصد.
accounts-emergency-warning = أنت داخل بالرمز الطارئ. يمنحك صلاحيات المدير في هذه الجلسة لكنه ليس حسابًا: لا تفضيلات له، وسجل التحرير سيقيّد تغييراتك بوصفها emergency-token لا بوصفها شخصًا. أنشئ لنفسك حسابًا حقيقيًا أدناه وادخل به.
accounts-created-with-password = أُنشئ { $username }. وكلمة السر هي { $password } — تُعرض مرة واحدة ولا تُحفظ إلا بصمةً بخوارزمية Argon2id، فسلّمها الآن.
accounts-last-admin = هذا هو المدير الفعّال الوحيد. رقِّ شخصًا آخر أولًا — فالتنصيب بلا مدير لا يُستعاد إلا بتحرير ملف ‎.acl‎ أو باستعمال الرمز الطارئ.

## التعارض

conflict-lede = حفظ { $who } تغييرًا على هذا السجل ({ $kind }) في { $when }، بعد أن فتحته أنت. ولم يُحفظ تحريرك، ولم يُطمس شيء.
conflict-no-merge = لا دمج تلقائي هنا. فدمج تحريرَي شخصين يُخرج سجلًا لم يخترْه أيٌّ منهما، وفي علم الأنساب يعني اختلاف محرّرَين على تاريخ أنهما يقرآن مصدرين مختلفين — وذاك سؤال لإنسان لا لبرنامج. قارن بين الاثنين أدناه ثم قرّر.
conflict-different-fields = غيّرتما حقولًا مختلفة، فليس في عمل { $who } ما يُنازَع فيه — غير أن إعادة التطبيق تكتب سجلك كاملًا فوق سجله على أي حال. راجع العمودين قبل الحفظ.
conflict-nothing-differs = لا تختلف أي من النسختين عمّا بدأتَ منه في أي حقل تعرضه هذه الصفحة. تقدّم رقم النسخة فحسب، أي أن أحدهم حفظ السجل دون أن يغيّر شيئًا مما يحويه.
conflict-reapply-hint = هذا تحريرك أنت، منقولًا إلى النسخة { $version }. عدّله هنا لتُبقي ما تشاء من عمل { $who }، ثم احفظ. ونسخته معروضة أدناه لتنقل منها.

## الاستيراد

convert-title = استيراد ملف عائلي
convert-submit = استيراد
convert-result-title = تقرير الاستيراد
convert-download = تنزيل الأرشيف
convert-page-title = استيراد ملف عائلي
convert-lede = انقل شجرة قائمة من ملف GEDCOM — وهو التصدير الذي تنتجه معظم برامج الأنساب. لا يُحفظ هنا شيء، والشجرة التي يعرضها هذا الموقع تبقى كما هي تمامًا.
convert-file-label = ملف عائلي (‎.ged‎)
convert-file-hint = حتى { $mb } ميغابايت. شجرة من 767 شخصًا نحو 320 كيلوبايت.
convert-confidence-label = ما مقدار الثقة بهذه الوقائع في البداية
convert-confidence-hint = الملف الذي تستورده لا يقول كم كان أحدهم واثقًا، فكل واقعة تحتاج نقطة بداية. اجعلها منخفضة لشجرة جُمعت على عجل، وأعلى لشجرة عُملت من الوثائق. والقراءة الأمينة لهذا الرقم هي «مستورَد، ولم يراجعه أحد منذئذ» — ويمكنك رفع أي واقعة أو خفضها لاحقًا، واحدةً واحدة.
convert-lang-label = لغة أسماء الأماكن
convert-lang-hint = وسم مثل en أو fr أو ar.
convert-failed = لم يتم الاستيراد
convert-try-another = جرّب ملفًا آخر
convert-converted = استُورد { $filename }
convert-result-lede = { $total ->
        [zero] لا سجلات
        [one] سجل واحد
        [two] سجلان
        [few] { $total } سجلات
        [many] { $total } سجلًا
       *[other] { $total } سجل
    }، { $size } كيلوبايت. دخل كل شيء بدرجة ثقة { $confidence }، وقُرئت أسماء الأماكن على أنها { $lang }. ولم تُمسّ الشجرة التي يعرضها هذا الموقع.
convert-produced = ما عبر
convert-skipped-title = { $n ->
        [zero] لا مدخلات تعذّرت قراءتها
        [one] مدخل واحد تعذّرت قراءته
        [two] مدخلان تعذّرت قراءتهما
        [few] { $n } مدخلات تعذّرت قراءتها
        [many] { $n } مدخلًا تعذّرت قراءتها
       *[other] { $n } مدخل تعذّرت قراءته
    }
convert-skipped-note = لم يكن في هذه المدخلات ما يمكن نقله.
convert-other-diagnostics = { $n ->
        [zero] لا شيء آخر يستحق العلم به
        [one] أمر آخر يستحق العلم به
        [two] أمران آخران يستحقان العلم بهما
        [few] { $n } أمور أخرى تستحق العلم بها
        [many] { $n } أمرًا آخر تستحق العلم بها
       *[other] { $n } أمر آخر يستحق العلم به
    }
convert-clean = لم يبقَ وراءنا شيء — عبر كل مدخل في الملف.
convert-download-title = التنزيل
convert-download-named = تنزيل { $name }
convert-download-note = يُحفظ هنا خمس عشرة دقيقة ثم يُتلف، فنزّله الآن.
convert-another = استيراد ملف آخر

## الشجرة والصفحة الرئيسة

date-quarter-century = الربع { $quarter ->
        [1] الأول
        [2] الثاني
        [3] الثالث
       *[other] الرابع
    } من القرن { $century }
tree-width-notice = عرض هذا العرض { $width } بكسل — فعلى شاشة عرضها 1500 بكسل يعادل ذلك { $screens ->
        [zero] لا تمرير أفقي
        [one] شاشة واحدة
        [two] شاشتين
        [few] { $screens } شاشات
        [many] { $screens } شاشةً
       *[other] { $screens } شاشة
    } من التمرير الأفقي.
tree-contradicts-title = هذه الشجرة تناقض نفسها.
tree-contradicts-detail = لا ترتيب للصفوف يفي بذلك، فتُركت الصلة أدناه خارج ترقيم الأجيال وقد تكون بعض الصفوف مرسومة في غير موضعها. صحّح أيًّا من السجلين هو الخطأ.
tree-contradicts-pair = مسجَّلان زوجين ووالدًا وولدًا في آن:
tree-contradicts-more = { $n ->
        [zero] لا تناقضات أخرى غير مذكورة.
        [one] تناقض آخر غير مذكور.
        [two] تناقضان آخران غير مذكورين.
        [few] { $n } تناقضات أخرى غير مذكورة.
        [many] { $n } تناقضًا آخر غير مذكورة.
       *[other] { $n } تناقض آخر غير مذكور.
    }
tree-no-people-cta = استورد ملفًا عائليًا، أو أضف أول شخص.
home-empty = لم يُسجَّل شيء بعد. استورد ملفًا عائليًا لنقل شجرة قائمة، أو أضف أول شخص بيدك.
home-count = { $total ->
        [zero] لا سجلات
        [one] سجل واحد
        [two] سجلان
        [few] { $total } سجلات
        [many] { $total } سجلًا
       *[other] { $total } سجل
    }، في ملف واحد تملكه العائلة.
home-browse = تصفّح الشجرة
home-convert = استيراد ملف عائلي
home-unnamed-family = شجرة العائلة هذه
home-in-this-tree = ما سجّلته العائلة حتى الآن
home-showcase-title = حيث تقول هذه الشجرة أكثر من الأسماء والتواريخ
home-showcase-example = انظر مثالًا ←
home-nothing-title = لا شيء لعرضه بعد.
home-nothing-detail = استورد ملفًا عائليًا لنقل شجرة قائمة، أو ابدأ من الصفر وأضف أول شخص بنفسك.

## بطاقات العرض

showcase-links-title = { $n ->
        [zero] لا صلات خارج العائلة
        [one] صلة واحدة خارج العائلة
        [two] صلتان خارج العائلة
        [few] { $n } صلات خارج العائلة
        [many] { $n } صلةً خارج العائلة
       *[other] { $n } صلة خارج العائلة
    }
showcase-links-detail = عرّابون وأرباب عمل وشهود ومعلّمون، لكل صلة تواريخها ومصدرها ودرجة ثقتك بها.
showcase-occupations-title = { $n ->
        [zero] لا مهن لها بداية ونهاية
        [one] مهنة واحدة لها بداية ونهاية
        [two] مهنتان لهما بداية ونهاية
        [few] { $n } مهن لها بداية ونهاية
        [many] { $n } مهنةً لها بداية ونهاية
       *[other] { $n } مهنة لها بداية ونهاية
    }
showcase-occupations-detail = «معلّمة، 1948–1978» تحفظ طولها، وتُرسم شريطًا عبر السنين لا سطرًا واحدًا مؤرَّخًا.
showcase-uncertain-title = { $n ->
        [zero] لا تواريخ تُركت على ما جاءت به من عدم يقين
        [one] تاريخ واحد تُرك على ما جاء به من عدم يقين
        [two] تاريخان تُركا على ما جاءا به من عدم يقين
        [few] { $n } تواريخ تُركت على ما جاءت به من عدم يقين
        [many] { $n } تاريخًا تُركت على ما جاءت به من عدم يقين
       *[other] { $n } تاريخ تُرك على ما جاء به من عدم يقين
    }
showcase-uncertain-detail = نحو، وقبل، وبعد، وبين تبقى أربع عبارات مختلفة. والتاريخ الذي عجز المصدر عن تحديده لا يُعرض قط كأنه حدّده.
showcase-preserved-title = { $n ->
        [zero] لا تواريخ محفوظة بألفاظها التي كُتبت بها
        [one] تاريخ واحد محفوظ بألفاظه التي كُتب بها
        [two] تاريخان محفوظان بألفاظهما التي كُتبا بها
        [few] { $n } تواريخ محفوظة بألفاظها التي كُتبت بها
        [many] { $n } تاريخًا محفوظة بألفاظها التي كُتبت بها
       *[other] { $n } تاريخ محفوظ بألفاظه التي كُتب بها
    }
showcase-preserved-detail = الصياغة التي عجز الجميع عن قراءتها تاريخًا تبقى كما كُتبت تمامًا، بدل أن تُطرح في صمت.
showcase-sources-title = { $n ->
        [zero] لا مصادر مسجَّلة الموثوقية
        [one] مصدر واحد مسجَّل الموثوقية
        [two] مصدران مسجَّلا الموثوقية
        [few] { $n } مصادر مسجَّلة الموثوقية
        [many] { $n } مصدرًا مسجَّلة الموثوقية
       *[other] { $n } مصدر مسجَّل الموثوقية
    }
showcase-sources-detail = { $primary ->
        [zero] لا مصادر أوّلية.
        [one] مصدر أوّلي واحد.
        [two] مصدران أوّليان.
        [few] { $primary } مصادر أوّلية.
        [many] { $primary } مصدرًا أوّليًا.
       *[other] { $primary } مصدر أوّلي.
    } وكل واقعة تبيّن على أي دليل تقوم، وما قوّة ذلك الدليل.
showcase-places-title = { $n ->
        [zero] لا أماكن تحرّكت حدودها
        [one] مكان واحد تحرّكت حدوده
        [two] مكانان تحرّكت حدودهما
        [few] { $n } أماكن تحرّكت حدودها
        [many] { $n } مكانًا تحرّكت حدودها
       *[other] { $n } مكان تحرّكت حدوده
    }
showcase-places-detail = قد تنتمي بلدة إلى دول مختلفة في أزمنة مختلفة، والسجل يقول أيها كان ساريًا ومتى.

## نتائج العمليات

result-diagnostics = التشخيصات
result-diagnostics-note = كل تشخيص أعادته المكتبة، بما في ذلك التحذيرات التي لم توقف العملية. ولا يُرشَّح منها شيء.
result-no-diagnostics = لم تُعِد المكتبة أي تشخيص.
result-continue = متابعة
result-dashboard = اللوحة

## تفاصيل السجل

record-notes-title = ما يُذكر عن هذا السجل:
record-absent-document = ذكره هذا الشخص لكنه غير محفوظ هنا.
record-upload-help = حتى { $mb } ميغابايت للملف. تُحفظ المرفقات بجوار الشجرة وتُكتب في الأرشيف عند التصدير، فتسافر الصورة مع العائلة التي تخصّها. ويُقرأ نوع الملف من محتواه لا من اسمه: تُقبل الصور وPDF والنص الصِّرف والصوت والفيديو. أما SVG فمرفوض، لأنه قد يحمل سكربتًا.
record-upload-help-short = حتى { $mb } ميغابايت. وSVG مرفوض.
record-verbatim-note = محفوظ كما جاء في السجل تمامًا، لأن أي محوِّل لم يستطع تأويله.
record-file-to-attach = الملف المراد إرفاقه
record-document-type = نوع المستند
record-caption = التعليق
record-caption-placeholder = تعليق (اختياري)
record-history-entry-meta = — { $at }
record-history-entry-version = ، { $version }

## أنواع السجلات

kind-person = شخص
kind-family = عائلة
kind-event = واقعة
kind-link = صلة
kind-occupation = مهنة
kind-source = مصدر
kind-place = مكان
kind-document = مستند

kind-person-plural = { $n ->
        [zero] أشخاص
        [one] شخص
        [two] شخصان
        [few] أشخاص
        [many] شخصًا
       *[other] شخص
    }
kind-family-plural = { $n ->
        [zero] عائلات
        [one] عائلة
        [two] عائلتان
        [few] عائلات
        [many] عائلةً
       *[other] عائلة
    }
kind-event-plural = { $n ->
        [zero] وقائع
        [one] واقعة
        [two] واقعتان
        [few] وقائع
        [many] واقعةً
       *[other] واقعة
    }
kind-link-plural = { $n ->
        [zero] صلات
        [one] صلة
        [two] صلتان
        [few] صلات
        [many] صلةً
       *[other] صلة
    }
kind-occupation-plural = { $n ->
        [zero] مهن
        [one] مهنة
        [two] مهنتان
        [few] مهن
        [many] مهنةً
       *[other] مهنة
    }
kind-source-plural = { $n ->
        [zero] مصادر
        [one] مصدر
        [two] مصدران
        [few] مصادر
        [many] مصدرًا
       *[other] مصدر
    }
kind-place-plural = { $n ->
        [zero] أماكن
        [one] مكان
        [two] مكانان
        [few] أماكن
        [many] مكانًا
       *[other] مكان
    }
kind-document-plural = { $n ->
        [zero] مستندات
        [one] مستند
        [two] مستندان
        [few] مستندات
        [many] مستندًا
       *[other] مستند
    }

## القوائم

list-matching = { $total ->
        [zero] لا مطابقات
        [one] مطابقة واحدة
        [two] مطابقتان
        [few] { $total } مطابقات
        [many] { $total } مطابقةً
       *[other] { $total } مطابقة
    }، { $per_page } في الصفحة.
list-filter-placeholder = ترشيح بالاسم أو المعرّف
list-filter = ترشيح
list-clear = مسح
list-summary = الوصف
list-id = المعرّف
list-actions = إجراءات
list-nothing = لا شيء هنا.
list-nothing-matching = لا شيء هنا يطابق «{ $q }».
list-delete-confirm = أتحذف هذا السجل ({ $kind })؟ اختر ما يحلّ بالسجلات التي تشير إليه:
list-policy-reject = رفض
list-policy-reject-detail = — ارفض ما دام شيء يشير إليه. ولا يضيع شيء.
list-policy-cascade = تعاقب
list-policy-cascade-detail = — احذفه وأزل فعليًا كل إشارة إليه.
list-policy-orphan = تفريغ الإشارة
list-policy-orphan-detail = — احذفه وأبقِ السجلات المشيرة إليه مع تفريغ الإشارة.

## اكتمال السجل

completeness-dates-title = التواريخ بحسب الصورة التي هي عليها فعلًا
completeness-no-dates = لا تواريخ مسجَّلة بعد.
completeness-dates-note = تاريخٌ استطاع أحدهم تحديده إلى اليوم، وتاريخٌ لم يستطع وضعه إلا في عقد، عبارتان مختلفتان، وكلتاهما تُحفظ كما جاءت. والنص الذي تعذّرت قراءته تاريخًا يُحفظ كلمةً بكلمة بدل أن يُطرح.
completeness-shape-exact = محدَّد
completeness-shape-exact-note = يوم تقويمي كامل
completeness-shape-approximate = تقريبي
completeness-shape-approximate-note = «نحو»، أو سنة أو عقد فحسب
completeness-shape-ranged = محصور
completeness-shape-ranged-note = قبل أو بعد أو بين
completeness-shape-preserved = محفوظ بنصّه
completeness-shape-preserved-note = نص غير قابل للتأويل، محفوظ كما هو
completeness-shape-unknown = مجهول
completeness-shape-unknown-note = مسجَّل بوصفه غير معروف

## الإدارة، تتمة

admin-history-on = على
admin-history-meta = — { $kind }، { $at }
admin-validation-counts = { $errors ->
        [zero] لا أخطاء
        [one] خطأ واحد
        [two] خطآن
        [few] { $errors } أخطاء
        [many] { $errors } خطأً
       *[other] { $errors } خطأ
    }، { $warnings ->
        [zero] ولا تحذيرات
        [one] وتحذير واحد
        [two] وتحذيران
        [few] و{ $warnings } تحذيرات
        [many] و{ $warnings } تحذيرًا
       *[other] و{ $warnings } تحذير
    }، { $infos ->
        [zero] ولا ملاحظات
        [one] وملاحظة واحدة
        [two] وملاحظتان
        [few] و{ $infos } ملاحظات
        [many] و{ $infos } ملاحظةً
       *[other] و{ $infos } ملاحظة
    }.
admin-warnings-never-block = التحذيرات لا توقف شيئًا أبدًا — فهي معلومات لا بوابات.
admin-validator-clean = لم يبلّغ التحقق عن شيء.
record-occupations-help-undated = تُسجَّل المهنة ببداية ونهاية، فيمكن مقارنة عدة مهن على محور زمني واحد. وفي هذا الأرشيف مسمّيات المهن دون تواريخ لها — وهو مألوف بعد الاستيراد، إذ لا موضع لها في أكثر الملفات العائلية — فليس ثمة مقياس يُرسم بعد.
record-occupations-help-axis = المهنة حالٌ لها امتداد، لا واقعةٌ في تاريخ واحد. وكل المدد تتقاسم محورًا واحدًا، { $from }–{ $to }.
admin-value-not-set = غير محدَّد
admin-validation-report = تقرير التحقق
admin-dedup-complete = اكتملت إزالة التكرار
admin-dedup-refused = رُفضت إزالة التكرار
record-birth-order = ترتيب الميلاد
record-start-not-recorded = البداية غير مسجَّلة
record-end-not-recorded = النهاية غير مسجَّلة
record-document-no-file = المستند مسجَّل هنا، لكن الملف نفسه غير محفوظ

## ملاحظات على السجل

note-links = { $n ->
        [zero] لا صلات خارج العائلة، لها تواريخها ومصادرها
        [one] صلة واحدة خارج العائلة، لها تواريخها ومصادرها
        [two] صلتان خارج العائلة، لهما تواريخهما ومصادرهما
        [few] { $n } صلات خارج العائلة، لها تواريخها ومصادرها
        [many] { $n } صلةً خارج العائلة، لها تواريخها ومصادرها
       *[other] { $n } صلة خارج العائلة، لها تواريخها ومصادرها
    }
note-occupations = { $n ->
        [zero] لا مهن مسجَّلة ببداية ونهاية
        [one] مهنة واحدة مسجَّلة ببداية ونهاية
        [two] مهنتان مسجَّلتان ببداية ونهاية
        [few] { $n } مهن مسجَّلة ببداية ونهاية
        [many] { $n } مهنةً مسجَّلة ببداية ونهاية
       *[other] { $n } مهنة مسجَّلة ببداية ونهاية
    }

## عناوين صفوف قوائم الإدارة

family-label-couple = { $children ->
        [0] { $a } و{ $b }
        [one] { $a } و{ $b } — ابن واحد
        [two] { $a } و{ $b } — ابنان
        [few] { $a } و{ $b } — { $children } أبناء
        [many] { $a } و{ $b } — { $children } ابنًا
       *[other] { $a } و{ $b } — { $children } ابن
    }
family-label-half = { $children ->
        [0] { $a } و{ $unknown }
        [one] { $a } و{ $unknown } — ابن واحد
        [two] { $a } و{ $unknown } — ابنان
        [few] { $a } و{ $unknown } — { $children } أبناء
        [many] { $a } و{ $unknown } — { $children } ابنًا
       *[other] { $a } و{ $unknown } — { $children } ابن
    }
family-label-children = { $others ->
        [0] { $first } — الوالدان غير مسجَّلين
        [one] { $first } وأخ واحد — الوالدان غير مسجَّلين
        [two] { $first } وأخوان — الوالدان غير مسجَّلين
        [few] { $first } و{ $others } إخوة — الوالدان غير مسجَّلين
        [many] { $first } و{ $others } أخًا — الوالدان غير مسجَّلين
       *[other] { $first } و{ $others } أخ — الوالدان غير مسجَّلين
    }
family-label-empty = عائلة لا أحد مسجَّل فيها

event-label = { $category } — { $who }، { $date }
event-label-nobody = { $category } — { $date }
event-two-people = { $a } و{ $b }
event-more-people = { $a } و{ $b } و{ $others ->
        [zero] لا أحد غيرهما
        [one] واحد آخر
        [two] اثنان آخران
        [few] { $others } آخرون
        [many] { $others } آخرين
       *[other] { $others } آخر
    }

link-label = { $label }: { $from } ← { $to }
occupation-label = { $who } — { $title }
source-label = { $title } — { $reliability }
source-label-plain = { $title }
document-label = { $filename } — { $type }
document-label-untitled = { $type } بلا عنوان
list-unnamed = { $kind } بلا اسم

## مفردات المواصفة في القوائم

event-category-birth = ميلاد
event-category-death = وفاة
event-category-marriage = زواج
event-category-divorce = طلاق
event-category-baptism = عماد
event-category-burial = دفن
event-category-immigration = هجرة وافدة
event-category-emigration = هجرة مغادرة
event-category-census = تعداد
event-category-residence = إقامة
event-category-military = خدمة عسكرية
event-category-education = تعليم
event-category-other = واقعة

reliability-primary = مصدر أوّلي
reliability-secondary = مصدر ثانوي
reliability-tertiary = مصدر ثالثي
reliability-recollection = رواية شفوية
reliability-derivative = عمل مشتق
reliability-authored = عمل مؤلَّف
reliability-oral = رواية شفوية متوارثة
reliability-unknown = الموثوقية مجهولة

document-type-photo = صورة فوتوغرافية
document-type-certificate = وثيقة رسمية
document-type-letter = رسالة
document-type-record = قيد أرشيفي
document-type-newspaper = قصاصة صحفية
document-type-other = مستند

## أين يمكن لهذا السجل أن يقول أكثر

completeness-title = أين يمكن لهذه الشجرة أن تقول أكثر
completeness-intro = ما هو مسجَّل وما لا يزال فارغًا.
completeness-import-title = ما جاء به الاستيراد
completeness-import-intro = محسوبًا من الملف الذي رفعته للتو. والسطر الفارغ شيء لم يسجّله الملف الأصلي، لا شيء أضاعه الاستيراد.

completeness-headline-full = كل ضرب من التفاصيل أدناه مسجَّل في مكان ما من هذه الشجرة.
completeness-headline-empty = { $total ->
        [zero] لا ضروب من التفاصيل أدناه.
        [one] الضرب الوحيد من التفاصيل أدناه لم يُسجَّل في أي موضع بعد.
        [two] لم يُسجَّل بعدُ أيٌّ من ضربَي التفاصيل أدناه.
        [few] لم يُسجَّل بعدُ أيٌّ من ضروب التفاصيل الـ{ $total } أدناه.
        [many] لم يُسجَّل بعدُ أيٌّ من ضروب التفاصيل الـ{ $total } أدناه.
       *[other] لم يُسجَّل بعدُ أيٌّ من ضروب التفاصيل الـ{ $total } أدناه.
    } وكلٌّ منها موضعٌ يمكن للسجل أن يقول فيه أكثر.
completeness-headline-partial = { $carried ->
        [zero] لا ضرب من التفاصيل أدناه مسجَّل
        [one] ضرب واحد من التفاصيل أدناه مسجَّل
        [two] ضربان من التفاصيل أدناه مسجَّلان
        [few] { $carried } ضروب من التفاصيل أدناه مسجَّلة
        [many] { $carried } ضربًا من التفاصيل أدناه مسجَّلة
       *[other] { $carried } ضرب من التفاصيل أدناه مسجَّل
    }؛ { $empty ->
        [zero] ولا شيء منها فارغ
        [one] وواحد لا يزال فارغًا
        [two] واثنان لا يزالان فارغين
        [few] و{ $empty } لا تزال فارغة
        [many] و{ $empty } لا تزال فارغة
       *[other] و{ $empty } لا يزال فارغًا
    }.

completeness-metric-confidence = ما مقدار الثقة بكل واقعة
completeness-metric-confidence-none = ليس في الوقائع الـ{ $slots } هنا واحدة تقول ما مقدار الثقة بها. فتاريخٌ نُقل عن وثيقة وتاريخٌ خُمّن يبدوان سواءً، إلى أن يكفّا عن ذلك.
completeness-metric-confidence-uniform = { $with } من { $slots } واقعة تحمل درجة، وكلها الرقم نفسه ({ $modal }). وهذا ما يخلّفه استيراد بالجملة: قيمة افتراضية لم يعد إليها أحد. ولم تُقيَّم واحدة منها على حدة بعد.
completeness-metric-confidence-some = { $with } من { $slots } واقعة تحمل درجة. منها { $modal_count } تشترك في قيمة واحدة ({ $modal })، و{ $assessed } تخالفها فنُظر إليها واحدةً واحدة.
completeness-metric-confidence-many = { $with } من { $slots } واقعة تحمل درجة، منها { $assessed } تخالف القيمة الأشيع ({ $modal })، على { $distinct } مستوى متمايزًا. وهذه الشجرة تسجّل عدم يقين حقيقيًا متفاوتًا.

completeness-metric-parentage = ما مقدار الثقة بكل صلة والدية
completeness-metric-parentage-none = ليس في صلات النسب هنا واحدة تقول ما مقدار الثقة بها. والتبنّي والخطوط المتنازع عليها وما أُعيد بناؤه من ذكرٍ واحد هي بعينها المواضع التي تحتاج فيها عائلة إلى تسجيل الشك — والشجرة ترسم الصلة الأقل يقينًا خطًا أبهت.
completeness-metric-parentage-some = { $n ->
        [zero] لا صلات نسب تحمل درجتها الخاصة
        [one] صلة نسب واحدة تحمل درجتها الخاصة
        [two] صلتا نسب تحملان درجتيهما الخاصة
        [few] { $n } صلات نسب تحمل درجاتها الخاصة
        [many] { $n } صلةَ نسب تحمل درجاتها الخاصة
       *[other] { $n } صلة نسب تحمل درجتها الخاصة
    }، فالخط المفترَض أضعف في العين من الخط الموثَّق.

completeness-metric-links = صلات وراء الدم والزواج
completeness-metric-links-none = عرّابون وأرباب عمل وشهود ومعلّمون وأوصياء. ولم تُسجَّل واحدة بعد. ولكلٍّ منها أن تحمل تواريخها ومصدرها ودرجة ثقتك بها.
completeness-metric-links-some = { $n ->
        [zero] لا شيء مسجَّل بعد.
        [one] واحدة مسجَّلة، لها تواريخها ومصدرها ودرجة ثقتك بها.
        [two] اثنتان مسجَّلتان، لكلٍّ تواريخها ومصدرها ودرجة ثقتك بها.
        [few] { $n } مسجَّلة، لكلٍّ تواريخها ومصدرها ودرجة ثقتك بها.
        [many] { $n } مسجَّلة، لكلٍّ تواريخها ومصدرها ودرجة ثقتك بها.
       *[other] { $n } مسجَّلة، لكلٍّ تواريخها ومصدرها ودرجة ثقتك بها.
    }

completeness-metric-occupations = مهن مسجَّلة ببداية ونهاية
completeness-metric-occupations-none = لا مهن مسجَّلة. وحرفةٌ زاولها المرء ثلاثين سنة تقول عن حياة أكثر مما يقوله سطر واحد مؤرَّخ.
completeness-metric-occupations-undated = { $total ->
        [zero] لا مهن مسجَّلة، فلا تواريخ
        [one] مهنة واحدة مسجَّلة، بلا تواريخ
        [two] مهنتان مسجَّلتان، بلا تواريخ
        [few] { $total } مهن مسجَّلة، بلا تواريخ
        [many] { $total } مهنةً مسجَّلة، بلا تواريخ
       *[other] { $total } مهنة مسجَّلة، بلا تواريخ
    }. أضف بداية ونهاية فتُقارَن جنبًا إلى جنب على محور زمني واحد.
completeness-metric-occupations-some = { $span } من { $total } لها بداية أو نهاية، فيمكن مقارنتها جنبًا إلى جنب على محور زمني واحد.

completeness-metric-sources = مصادر مقدَّرة الموثوقية
completeness-metric-sources-none = لا مصادر مسجَّلة. وتسميةُ ما جاءت منه الواقعة هي ما يتيح لقريب أن يتحقق منها لاحقًا — أو أن يخالفها ويقول لماذا.
completeness-metric-sources-some = { $graded } من { $total } تقول ما قوّتها، فالدعوى القائمة على شهادة ميلاد ليست في العين كالدعوى القائمة على ذكرى.

completeness-what-is-recorded = ما يستطيع السجل قوله
completeness-in-this-tree = في هذه الشجرة
completeness-not-yet = لم يُسجَّل بعد

## أدوار المشارك في واقعة

role-spouse = الزوج
role-spouse_1 = الزوج الأول
role-spouse_2 = الزوج الثاني
role-subject = صاحب السجل
role-participant = مشارك
role-witness = شاهد
role-officiant = عاقد
role-informant = مُبلِّغ
role-godparent = عرّاب

phys-no-source = بلا مصدر
phys-col-date = متى
phys-col-source = المصدر
phys-col-confidence = الثقة
phys-col-note = ملاحظة
phys-field-height-cm = الطول
phys-field-weight-kg = الوزن
phys-field-eye-colour = لون العينين
phys-field-hair-colour = لون الشعر
phys-field-build = البنية
phys-field-handedness = اليد الغالبة
phys-field-features = العلامات الفارقة
phys-field-military = الخدمة العسكرية
phys-field-languages = اللغات
phys-field-blood-group = فصيلة الدم
phys-field-conditions = الأمراض المعروفة
phys-field-operations = العمليات والإصابات
phys-field-cause-of-death = سبب الوفاة
phys-field-religion = الدين أو الانتماء
phys-field-health-notes = ملاحظات
admin-export-health-note = يستبعد التصدير العادي كل فئة حساسة — الصحة والمعتقد، والبيانات البيومترية، والبيانات الجينومية، والسجل الجنائي — وكذلك الملف السلوكي لأي شخص على قيد الحياة، فلا يحمل ملفٌّ يُرسَل إلى قريبٍ أيًّا منها. حدِّد ما ينبغي أن يحمله ملف بعينه؛ ويسجّل الملف نفسه الفئات التي استُبعدت.
avatar-picker-title = اختر صورة
avatar-choose-link = اختر صورة
avatar-choose = أي صورة تمثّل هذا الشخص
avatar-mode-auto = دع البرنامج يختار
avatar-mode-auto-note = أول صورة شخصية، وإلا فأول صورة مرتبطة بهذا السجل.
avatar-mode-none = أظهر الحروف الأولى بدلًا منها
avatar-mode-none-note = لسجل صوره وثائق لا وجوه.
avatar-focal-hint = انقر صورة لاختيارها، ثم انقر ثانيةً على الجزء الذي ينبغي بقاؤه في الإطار — الصورة الرمزية مربّعة ومعظم المسوحات ليست كذلك.
avatar-no-images = لا صور مرتبطة بهذا السجل بعد.
avatar-upload-title = ارفع صورة واستعملها
avatar-upload-button = ارفع واجعلها الصورة
avatar-not-available-title = هذه الصورة غير متاحة
avatar-not-available-detail = الملف المختار غير مرتبط بهذا الشخص، أو ليس لك أن تقرأه.

record-history-withheld = محجوب عنك

## A death nobody wrote down
#
# GEDCOM cannot say "died, date unknown", so a converter marks those
# people living. These three say that the page worked it out rather than
# read it, which is a distinction this product does not blur.

record-life-status = الحالة
record-presumed-deceased = وفاة مفترضة
record-presumed-short = مفترضة
record-presumed-why = لا وفاة مسجّلة والولادة قبل أكثر من { $years } سنة، فلا يمكن أن يكون هذا السجل صحيحًا. الأرشيف لم يُغيَّر: هذا استنتاج الصفحة لا ما يقوله المصدر.

## The identity editor

identity-editor-title = الأسماء والهوية
identity-primary-name = الاسم المعروض في كل مكان
identity-primary-help = ما تستعمله بطاقة الشجرة والعنوان وكل قائمة. الأسماء الأخرى أدناه هي ما سمّى به مصدرٌ هذا الشخص في وقت آخر.
identity-display = الاسم
identity-display-latin = بالحروف اللاتينية
identity-culture = اللغة
identity-direction = اتجاه الكتابة
identity-direction-ltr = من اليسار إلى اليمين
identity-direction-rtl = من اليمين إلى اليسار
identity-direction-auto = من النص
identity-components = أجزاء الاسم
identity-components-help = أي جزء هو الاسم الشخصي وأيّها اسم العائلة، بترتيب كتابتهما. السجل بلا أجزاء يُعرض على أي حال: الأجزاء هي ما يمكن للبحث مطابقته.
identity-part = الجزء
identity-value = النص
identity-other-names = أسماء أخرى
identity-other-help = اسم بعد الزواج، اسم ديني، اسم استعمله سجل لاحق. لكل واحد مدة استعماله والمصدر الذي يذكره.
identity-name-type = نوع الاسم
identity-valid-from = مستعمل منذ
identity-valid-until = مستعمل حتى
identity-about = عن الشخص
identity-living-help = هذه العلامة التي وضعها المصدر. تفترض الصفحة الوفاة على حدة حين تكون الولادة قديمة جدًا، ولا يغيّر ذلك الافتراض هذا المربع ولا الأرشيف.
identity-error-no-display = يحتاج السجل إلى اسم يُعرض به. لم يُحفظ شيء.
editor-blank-to-remove = امسح الاسم لحذف هذا المُدخَل.
# A BCP 47 tag, shown as an example in the language field.
identity-culture-placeholder = ar
identity-edit-link = حرّر الأسماء والهوية

## Union types, statuses and date precision, said out loud

union-type-marriage = زواج
union-type-civil_union = اقتران مدني
union-type-cohabitation = معاشرة
union-type-religious_only = اقتران ديني
union-type-polygamous = تعدد
union-type-unknown = غير مسجّل
union-role-spouse = زوج
union-role-partner = شريك
union-role-husband = زوج
union-role-wife = زوجة
union-status-active = قائم
union-status-ended_by_death = انتهى بوفاة
union-status-ended_by_divorce = انتهى بطلاق
union-status-ended_by_separation = انتهى بانفصال
union-status-annulled = مُبطَل
union-status-unknown = غير مسجّل
union-status-ended = انتهى
union-status-ended-by = انتهى بـ { $reason }
union-reason-death_of_spouse = وفاة الزوج
precision-exact = إلى اليوم
precision-year = إلى السنة
precision-month = إلى الشهر
precision-decade = إلى العقد
precision-century = إلى القرن
precision-unknown = غير معروفة
record-precision = الدقة
record-approximate = تقريبي
record-place = المكان

## The relationships editor

family-editor-title = الأسرة والعلاقات
family-unions = الاقترانات
family-no-unions = لا اقتران مسجّل لهذا الشخص.
family-union-legend = اقتران { $n }
family-writes-family = الحفظ يغيّر سجل الأسرة #{ $id } الذي يتشاركه الشخصان، وتتغيّر معه صفحة الشخص الآخر.
family-partners = الشريكان
family-partner = الشريك
family-role = الدور
family-children = الأبناء
family-children-help = ترتيب الولادة هو ما يقوله السجل نفسه. إن تُرك فارغًا فلا يقول شيئًا: رقم مأخوذ من موضع الصف سيكون واقعة لم يكتبها أحد.
family-child = الابن
family-birth-order = ترتيب الولادة
family-the-union = الاقتران نفسه
family-type = نوع الاقتران
family-status = الحالة
family-started = البداية
family-ended = النهاية
family-leave = أخرج هذا الشخص من هذا الاقتران
family-open-entity = افتح سجل الأسرة
family-new-union = اقتران جديد
family-new-union-help = ينشئ هذا سجل أسرة جديدًا يضم هذا الشخص. الشريك اختياري: والدٌ يسمّيه السجل بلا شريك هو اقتران من واحد.
family-create-union = أنشئ الاقتران
family-parents = الوالدان
family-no-parents = هذا الشخص غير مسجّل ابنًا لأي أسرة.
family-child-of = ابن هذه الأسرة
family-detach-child = أخرج هذا الشخص من هذه الأسرة
family-attach-parents = اربط بالوالدين
family-attach-help = اختر الأسرة التي هذا الشخص ابنٌ فيها. يُضاف إلى سجل تلك الأسرة، فيظهر في صفحتي الوالدين أيضًا.
family-the-family = الأسرة
family-attach = اربط
family-error-last-partner = الاقتران يحتاج إلى شخص واحد على الأقل. احذف سجل الأسرة بدلًا من ذلك، فيسأل عمّا يشير إليه.
family-error-no-family = لم تُختَر أسرة. لم يُحفظ شيء.
family-error-already-child = هذا الشخص ابنٌ في تلك الأسرة أصلًا.
pick-error-empty = لم يُسمَّ أي شخص. لم يُحفظ شيء.
pick-error-not-found = لا شخص بهذا الاسم في هذا الأرشيف. لم يُحفظ شيء.
pick-error-ambiguous = أكثر من شخص ينطبق عليه ذلك. اختر واحدًا من القائمة ليقول السجل أيّهم. لم يُحفظ شيء.

## Links and occupations

links-editor-title = الروابط
links-editor-help = العلاقات غير الأسرية: عرّاب، ربّ عمل، شاهد، فوج. كلٌّ منها سجل قائم بذاته يسمّي شخصين، فتعديله هنا يغيّر ما يعرضه السجل الآخر.
links-none = لا رابط مسجّل لهذا الشخص.
links-new = رابط جديد
links-create = أنشئ الرابط
links-remove = احذف هذا الرابط
links-other-end = الطرف الآخر
links-label = ما هو
links-label-reverse = بالاتجاه المعاكس
links-category = الفئة
links-bidirectional = يُقرأ نفسه في الاتجاهين
links-from = من
links-until = حتى
links-reversed = أُنشئ هذا الرابط من السجل الآخر. تعديله هنا يغيّر الكيان نفسه.
link-error-no-label = يجب أن يقول الرابط ما هو. لم يُحفظ شيء.
occupations-editor-title = المِهَن
occupations-editor-help = المِهنة مدة لها بداية ونهاية، لا مسمّى وظيفي. لكلٍّ تواريخها ومصدرها.
occupations-none = لا مِهنة مسجّلة لهذا الشخص.
occupations-new = مِهنة جديدة
occupations-create = أنشئ المِهنة
occupations-remove = احذف هذه المِهنة
occupations-title = ما كان يعمله
occupations-employer = لمن
occupations-employer-place = أين كانوا
occupations-from = من
occupations-until = حتى
occupation-error-no-title = يجب أن تقول المِهنة ما كان يفعله المرء. لم يُحفظ شيء.
link-category-spiritual = روحية
link-category-professional = مهنية
link-category-social = اجتماعية
link-category-legal = قانونية
link-category-medical = طبية
link-category-educational = تعليمية
link-category-conflict = نزاع
link-category-other = أخرى
links-edit-link = حرّر الروابط
occupations-edit-link = حرّر المِهَن
family-edit-link = حرّر الأسرة والعلاقات

## Events and documents

events-editor-title = الأحداث
events-editor-help = الحدث يسمّي عدة أشخاص دفعة واحدة — زواج، عمادة، إحصاء — فكلٌّ منها سجل قائم بذاته يظهر في كل صفحة يسمّيها.
events-none = لا حدث يسمّي هذا الشخص.
events-new = حدث جديد
events-new-help = يُضاف هذا الشخص موضوعًا للحدث إن لم تسمِّ غيره. حدثٌ بلا أحد ليس إلا تاريخًا.
events-create = أنشئ الحدث
events-remove = احذف هذا الحدث
events-category = ماذا حدث
events-subcategory = بدقة أكبر
events-description = الوصف
events-participants = من كان حاضرًا
events-participants-help = الحفظ يغيّر سجل الحدث، وهو ما يعرضه كل شخص آخر مذكور فيه.
events-who = من
event-error-no-category = يجب أن يقول الحدث ماذا جرى. لم يُحفظ شيء.
documents-editor-title = الوثائق
documents-editor-help = إلى أي ملفات يشير هذا السجل وما كلٌّ منها بالنسبة إليه. مسح صف يفصل الملف: تبقى الوثيقة وبياناتها في الأرشيف.
documents-attached = مرفقة بهذا السجل
documents-upload = ارفع ملفًا
documents-upload-help = حتى { $mb } ميغابايت. يُحفظ الملف في الأرشيف ويُرفق بهذا السجل.
documents-caption = التعليق
documents-edit-link = أرفق الوثائق وافصلها
events-edit-link = حرّر الأحداث

## Presentation styles: density, never colour

prefs-style = الكثافة
prefs-style-help = كم تأخذ الصفحة من مساحة. مستقلة عن السمة التي تخص اللون وحده، فأي مزيج ممكن.
style-comfortable = مريحة
style-comfortable-note = الافتراضية، بمساحة للقراءة
style-compact = مضغوطة
style-compact-note = سجل أكثر في الشاشة، لمن يمرّ على عدة سجلات
style-paper = ورق
style-paper-note = خط بذيول وخطوط بدل البطاقات، للقراءة على مهل أو للطباعة

## Sensitive classes

admin-export-choose = تضمين في هذا التصدير
scope-health = الصحة والمعتقد
scope-biometrics = البيانات البيومترية
scope-genomics = البيانات الجينومية
scope-legal = السجل الجنائي
scope-behaviour = الملفات السلوكية للأحياء
admin-export-with-chosen = التصدير مع ما حُدِّد

## Profile

pg-identity = الهوية والأحوال المدنية
pg-identity-intro = من كان الشخص بحسب السجلات، وما دوّنته سجلات الأحوال المدنية.
pg-morphology = الصفات الجسدية
pg-morphology-intro = الجسد كما قيس ووُصف.
pg-biometrics = القياسات الحيوية
pg-biometrics-intro = الصوت واليدان والحواس، والأنماط التي يُتعرّف بها على الشخص.
pg-health = الصحة
pg-health-intro = الأمراض والعلاجات والقياسات والنتائج.
pg-genomics = علم الجينوم
pg-genomics-intro = فحوص الحمض النووي والمجموعات الفردانية والمتغيرات وسائر النتائج الجزيئية.
pg-death = الوفاة
pg-death-intro = كيف انتهت الحياة ومتى وأين، وما جرى للجثمان.
pg-residence = الإقامة والجنسية
pg-residence-intro = أين عاش الشخص، وأي الدول عدّته من مواطنيها، وما اللغات التي تكلّمها.
pg-education = التعليم والعمل
pg-education-intro = الدراسة والمؤهلات والدخل والممتلكات.
pg-military = الخدمة العسكرية والأوسمة
pg-military-intro = الخدمة والرتب والوحدات والأوسمة.
pg-legal = القضايا الجنائية
pg-legal-intro = الإجراءات الجنائية ونتائجها.
pg-belief = المعتقد والانتماء
pg-belief-intro = الدين والشعائر والقناعات والعضويات.
pg-personality = الشخصية والسلوك
pg-personality-intro = الطبع والعادات والهوايات كما تصفها المصادر.
pg-relationships = العلاقات
pg-relationships-intro = الوالدان والأزواج والأبناء وسائر الناس في حياة الشخص.
pg-digital-legacy = الإرث الرقمي
pg-digital-legacy-intro = المسوح والنماذج والتسجيلات والأرشيفات التي تمثّل الشخص.
pa-identity-titles = الألقاب
pa-identity-sex-at-birth = الجنس عند الولادة
pa-identity-gender-identity = الهوية الجندرية
pa-birth-time = ساعة الولادة
pa-birth-coordinates = مكان الولادة بالإحداثيات
pa-civil-status-birth-certificate-number = رقم شهادة الميلاد
pa-civil-status-register-entries = قيود السجل المدني
pa-civil-status-marginal-annotations = الحواشي
pa-morphology-height = الطول
pa-morphology-weight = الوزن
pa-morphology-bmi = مؤشر كتلة الجسم
pa-morphology-body-composition = تركيب الجسم
pa-morphology-build = البنية
pa-morphology-eye-colour = لون العينين
pa-morphology-eye-shape = شكل العينين
pa-morphology-eye-spacing = المسافة بين العينين
pa-morphology-hair-colour = لون الشعر الطبيعي
pa-morphology-hair-texture = نوع الشعر
pa-morphology-hairline = خط الشعر
pa-morphology-facial-hair = شعر الوجه
pa-morphology-body-hair = شعر الجسم
pa-morphology-skin-tone = نمط البشرة (فيتزباتريك)
pa-morphology-skin-undertone = درجة لون البشرة الخفية
pa-morphology-freckles = النمش
pa-morphology-pigmentation = علامات التصبّغ
pa-morphology-scars = الندوب
pa-morphology-tattoos = الوشوم
pa-morphology-moles = الشامات
pa-morphology-facial-asymmetries = عدم تناظر الوجه
pa-morphology-face-shape = شكل الوجه
pa-morphology-nose-shape = شكل الأنف
pa-morphology-ear-shape = شكل الأذنين
pa-morphology-lip-shape = شكل الشفتين
pa-morphology-dentition = الأسنان
pa-morphology-malocclusion = سوء الإطباق (تصنيف أنجل)
pa-morphology-posture = القوام
pa-morphology-gait = المشية
pa-morphology-distinguishing-features = العلامات المميزة
pa-biometrics-fingerprints = بصمات الأصابع
pa-biometrics-retinal-print = بصمة الشبكية
pa-biometrics-voice-signature = البصمة الصوتية
pa-biometrics-voice-frequency = التردد الأساسي للصوت
pa-biometrics-vocal-timbre = نبرة الصوت
pa-biometrics-spoken-accent = اللكنة
pa-biometrics-speech-rate = سرعة الكلام
pa-biometrics-verbal-tics = اللوازم اللفظية
pa-biometrics-frequent-vocabulary = المفردات الشائعة
pa-biometrics-speech-register = مستوى الكلام
pa-biometrics-motor-tics = العرّات الحركية
pa-biometrics-handedness = اليد المفضّلة
pa-biometrics-hearing = السمع
pa-biometrics-visual-acuity = حدّة البصر
pa-biometrics-optical-correction = تصحيح البصر
pa-health-blood-group = فصيلة الدم (ABO)
pa-health-rhesus = العامل الريسوسي (RhD)
pa-health-blood-pressure = ضغط الدم
pa-health-resting-heart-rate = معدل النبض في الراحة
pa-health-respiratory-capacity = وظائف التنفّس
pa-health-conditions = الأمراض
pa-health-surgeries = العمليات الجراحية
pa-health-injuries = الإصابات
pa-health-deformities = التشوّهات
pa-health-amputations = البتر
pa-health-prostheses = الأطراف والأعضاء الاصطناعية
pa-health-implants = الغرسات
pa-health-devices = الأجهزة المزروعة
pa-health-medications = الأدوية
pa-health-allergies = الحساسيات
pa-health-vaccinations = اللقاحات
pa-health-serology = الفحوص المصلية
pa-health-lab-results = نتائج المختبر
pa-health-deficiencies = حالات النقص
pa-health-sleep-disorders = اضطرابات النوم
pa-health-mental-health-assessments = تقييمات الصحة النفسية
pa-genomics-autosomal-mapping = فحص الحمض النووي الجسدي
pa-genomics-y-haplogroup = المجموعة الفردانية للكروموسوم Y
pa-genomics-mt-haplogroup = المجموعة الفردانية الميتوكوندرية
pa-genomics-whole-genome-sequencing = تسلسل الجينوم الكامل
pa-genomics-risk-variants = متغيرات الخطر
pa-genomics-hereditary-conditions = الأمراض الوراثية
pa-genomics-predispositions = الاستعدادات
pa-genomics-epigenetic-markers = الواسمات فوق الجينية
pa-genomics-epigenetic-age = العمر فوق الجيني
pa-genomics-gut-microbiome = ميكروبيوم الأمعاء
pa-genomics-skin-microbiome = ميكروبيوم الجلد
pa-genomics-toxicological-sensitivities = الحساسية للأدوية والسموم
pa-death-time = ساعة الوفاة
pa-death-coordinates = مكان الوفاة بالإحداثيات
pa-death-causes = أسباب الوفاة
pa-death-contributing-factors = العوامل المساهمة
pa-death-autopsy = تشريح الجثة
pa-death-disposition = التصرف في الجثمان
pa-death-grave = القبر
pa-residence-addresses = العناوين
pa-residence-nationality-of-origin = الجنسية الأصلية
pa-residence-acquired-nationalities = الجنسيات المكتسبة
pa-residence-mother-tongue = اللغة الأم
pa-residence-spoken-languages = اللغات المحكية
pa-education-level = المستوى التعليمي
pa-education-diplomas = الشهادات والدرجات
pa-education-institutions = المدارس والمؤسسات
pa-education-income = الدخل
pa-education-real-estate = العقارات
pa-military-distinctions = الأوسمة
pa-military-citations = التنويهات
pa-military-ranks = الرتب
pa-military-units = الوحدات
pa-military-service-numbers = الأرقام العسكرية
pa-legal-criminal-record = السجل الجنائي
pa-belief-religions = الدين
pa-belief-sacraments = الأسرار والشعائر
pa-belief-beliefs = القناعات
pa-belief-political-leanings = الميول السياسية
pa-belief-memberships = العضويات
pa-personality-big-five = درجات السمات الخمس الكبرى
pa-personality-mbti = نمط MBTI
pa-personality-introversion-extraversion = الانطواء والانبساط
pa-personality-stress-tolerance = تحمّل الضغط
pa-personality-decision-style = أسلوب اتخاذ القرار
pa-personality-interests = الاهتمامات
pa-personality-hobbies = الهوايات
pa-personality-sports = الرياضة
pa-personality-dietary-habits = النظام الغذائي
pa-personality-dependencies = الإدمان
pa-digital-legacy-body-models = نماذج الجسم
pa-digital-legacy-skin-textures = خامات البشرة
pa-digital-legacy-rigs = هياكل التحريك
pa-digital-legacy-voice-corpora = تسجيلات لتوليد الصوت
pa-digital-legacy-text-corpora = كتابات لنموذج لغوي
pa-digital-legacy-digital-traces = الآثار الرقمية
pa-digital-legacy-carbon-footprint = البصمة الكربونية
pa-digital-legacy-behaviour-models = نماذج السلوك
pf-identity-titles-text = اللقب كما ورد
pf-identity-titles-kind = نوع اللقب
pf-civil-status-marginal-annotations-text = الحاشية
pf-morphology-pigmentation-kind = نوع العلامة
pf-biometrics-spoken-accent-description = كيف وُصفت
pf-biometrics-optical-correction-kind = التصحيح
pf-health-amputations-level = مستوى البتر
pf-health-prostheses-kind = البديل الاصطناعي
pf-health-implants-kind = الغرسة
pf-health-devices-kind = الجهاز
pf-health-allergies-type = نوع الحساسية
pf-health-vaccinations-status = حالة التلقيح
pf-health-sleep-disorders-category = فئة الاضطراب
pf-death-autopsy-kind = التشريح
pf-education-institutions-name = اسم المؤسسة
pf-military-distinctions-name = اسم الوسام
pf-military-distinctions-kind = نوع الوسام
pf-military-citations-text = نص التنويه
pf-military-ranks-category = فئة الرتبة
pf-belief-political-leanings-position = الموقع على محور اليسار واليمين
pf-belief-memberships-kind = نوع المنظمة
pf-digital-legacy-carbon-footprint-method = طريقة التقدير
pf-age-years = العمر بالسنوات
pf-agreeableness = الوداعة
pf-allergen = مسبّب الحساسية
pf-amount = المبلغ
pf-analyte = المادة المقيسة
pf-artefact-type = نوع الأثر
pf-autoimmune = مناعي ذاتي
pf-body-region = منطقة الجسم
pf-bone-percent = العظام
pf-carrier-status = حالة الحمل الوراثي
pf-cause = السبب
pf-chronic = مزمن
pf-clock = الساعة
pf-condition = المرض
pf-conferred-by = مَنح من
pf-congenital = خلقي
pf-conscientiousness = يقظة الضمير
pf-consent = الموافقة
pf-coordinates = الإحداثيات
pf-corrected = مع التصحيح
pf-country = البلد
pf-court = المحكمة
pf-coverage = التغطية
pf-currency = العملة
pf-decimal = الحدّة (عشرية)
pf-denomination = المذهب
pf-derived-from-id = مشتق من
pf-description = الوصف
pf-details = التفاصيل
pf-diagnosis = التشخيص
pf-diameter-mm = القطر
pf-diastolic = الانبساطي
pf-diet = النظام الغذائي
pf-document-id = الوثيقة
pf-dose = الجرعة
pf-ear = الأذن
pf-entry-number = رقم القيد
pf-extraversion = الانبساط
pf-eye = العين
pf-fat-percent = الدهون
pf-fev1-fvc-ratio = نسبة FEV1/FVC
pf-fev1-litres = FEV1
pf-file-format = صيغة الملف
pf-findings = النتائج
pf-flag = العلامة
pf-format = الصيغة
pf-fracture = كسر
pf-fvc-litres = FVC
pf-gene = الجين
pf-generator = أُنجز بواسطة
pf-grade = الدرجة
pf-iccs-section = قسم الجريمة (ICCS)
pf-icd10-chapter = فصل التصنيف الدولي للأمراض
pf-indication = دواعي الاستعمال
pf-inheritance = نمط الوراثة
pf-inscription = النقش
pf-institution = المؤسسة
pf-instrument = الأداة
pf-isced-level = مستوى ISCED
pf-jurisdiction = الولاية القضائية
pf-language = اللغة
pf-lat = خط العرض
pf-level = المستوى
pf-lines = العنوان
pf-location = الموضع
pf-lon = خط الطول
pf-major = المجموعة الرئيسية
pf-marker = الواسم
pf-metaboliser-status = نمط الاستقلاب
pf-method = الطريقة
pf-mode = طريقة الاكتساب
pf-muscle-percent = العضلات
pf-neuroticism = العصابية
pf-number = الرقم
pf-nutrient = المغذّي
pf-offence = الجريمة
pf-office = المكتب
pf-openness = الانفتاح
pf-organisation = المنظمة
pf-outcome = النتيجة
pf-pace = وتيرة الشيخوخة
pf-page = الصفحة
pf-panel = مجموعة الفحوص
pf-party = الحزب
pf-pathogen = العامل الممرض
pf-pattern = نمط التعاطي
pf-percentile = الشريحة المئوية
pf-period = دورية الدفع
pf-place-id = المكان
pf-plot = القطعة
pf-polygenic-score = الدرجة متعددة الجينات
pf-postal-code = الرمز البريدي
pf-precision = الدقة
pf-prescription = الوصفة
pf-proficiency = مستوى الإتقان
pf-provider = الجهة المزوّدة
pf-quintile = خُمس الدخل
pf-rank = الرتبة
pf-rank-text = الرتبة كما وردت
pf-reaction = رد الفعل
pf-reference-build = الجينوم المرجعي
pf-reference-high = الحد الأعلى المرجعي
pf-reference-low = الحد الأدنى المرجعي
pf-register-type = نوع القيد
pf-result = النتيجة
pf-role = الدور
pf-sacrament = السرّ أو الشعيرة
pf-score = الدرجة
pf-sentence = العقوبة
pf-sequence = الترتيب في سلسلة الأسباب
pf-service = الفرع العسكري
pf-severity = الشدة
pf-shannon-diversity = تنوّع شانون
pf-shape = الشكل
pf-significance = الأهمية السريرية
pf-snp-count = عدد النيوكليوتيدات المفحوصة
pf-sport = الرياضة
pf-subclade = الفرع
pf-substance = المادة
pf-summary = الملخّص
pf-systolic = الانقباضي
pf-tenure = نوع الحيازة
pf-test = الفحص
pf-threshold-db = عتبة السمع
pf-title = العنوان
pf-tonnes-co2e-per-year = الانبعاثات
pf-tradition = التقليد الديني
pf-tree-version = إصدار الشجرة
pf-unit = الوحدة
pf-use = الاستخدام
pf-variant = المتغيّر
pf-volume = المجلد
pf-zygosity = اللاقحية
pu-cm = { $n } سم
pu-kg = { $n } كغ
pu-kg-m2 = { $n } كغ/م²
pu-percent = { $n }٪
pu-mm = { $n } مم
pu-hz = { $n } هرتز
pu-words-min = { $n } كلمة/دقيقة
pu-db-hl = { $n } dB HL
pu-mmhg = { $n } مم زئبق
pu-bpm = { $n } نبضة/دقيقة
pu-litres = { $n } لتر
pu-coverage = { $n }×
pu-years = { $n } سنة
pu-t-co2e-yr = { $n } طن مكافئ CO₂ سنويًا
pv-sensitive-class-health = الصحة والمعتقد
pv-sensitive-class-biometrics = البيانات البيومترية
pv-sensitive-class-genomics = البيانات الجينومية
pv-sensitive-class-legal = السجل الجنائي
pv-laterality-left = الأيسر
pv-laterality-right = الأيمن
pv-laterality-both = كلاهما
pv-body-region-head = الرأس
pv-body-region-face = الوجه
pv-body-region-neck = العنق
pv-body-region-left-shoulder = الكتف الأيسر
pv-body-region-right-shoulder = الكتف الأيمن
pv-body-region-left-arm = الذراع اليسرى
pv-body-region-right-arm = الذراع اليمنى
pv-body-region-left-hand = اليد اليسرى
pv-body-region-right-hand = اليد اليمنى
pv-body-region-chest = الصدر
pv-body-region-abdomen = البطن
pv-body-region-upper-back = أعلى الظهر
pv-body-region-lower-back = أسفل الظهر
pv-body-region-pelvis = الحوض والوركان
pv-body-region-left-leg = الساق اليسرى
pv-body-region-right-leg = الساق اليمنى
pv-body-region-left-foot = القدم اليسرى
pv-body-region-right-foot = القدم اليمنى
pv-body-region-internal = داخلي
pv-body-region-whole-body = الجسم كله
pv-body-region-other = منطقة أخرى
pv-artefact-type-mesh = شبكة مضلّعات
pv-artefact-type-point-cloud = سحابة نقاط
pv-artefact-type-skin-texture-map = خريطة خامة البشرة
pv-artefact-type-skeletal-rig = هيكل تحريك
pv-artefact-type-voice-corpus = مدوّنة صوتية
pv-artefact-type-text-corpus = مدوّنة نصوص
pv-artefact-type-trace-archive = أرشيف النشاط على الإنترنت
pv-artefact-type-behaviour-model = نموذج سلوك
pv-artefact-type-fingerprint-card = بطاقة بصمات
pv-artefact-type-fingerprint-template = قالب بصمة
pv-artefact-type-retinal-image = صورة الشبكية
pv-artefact-type-voiceprint = بصمة صوتية
pv-consent-given = ممنوحة
pv-consent-given-by-estate = ممنوحة من الورثة
pv-consent-refused = مرفوضة
pv-consent-withdrawn = مسحوبة
pv-consent-not-asked = لم تُطلب
pv-consent-unknown = غير معروفة
pv-sex-at-birth-female = أنثى
pv-sex-at-birth-male = ذكر
pv-sex-at-birth-intersex = ثنائي الجنس
pv-sex-at-birth-undetermined = غير محدد
pv-sex-at-birth-unknown = غير معروف
pv-gender-identity-woman = امرأة
pv-gender-identity-man = رجل
pv-gender-identity-non-binary = غير ثنائي
pv-gender-identity-other = أخرى
pv-gender-identity-undisclosed = غير مُفصح عنها
pv-gender-identity-unknown = غير معروفة
pv-title-kind-nobility = لقب نبالة
pv-title-kind-academic = لقب علمي
pv-title-kind-professional = لقب مهني
pv-title-kind-religious = لقب ديني
pv-title-kind-military = لقب عسكري
pv-title-kind-civic = لقب فخري
pv-title-kind-courtesy = لقب مجاملة
pv-title-kind-other = آخر
pv-register-type-birth = ولادة
pv-register-type-baptism = معمودية
pv-register-type-marriage = زواج
pv-register-type-death = وفاة
pv-register-type-burial = دفن
pv-register-type-divorce = طلاق
pv-register-type-recognition = إقرار بالنسب
pv-register-type-legitimation = تصحيح النسب
pv-register-type-adoption = تبنٍّ
pv-register-type-name-change = تغيير الاسم
pv-register-type-other = آخر
pv-build-slight = نحيلة جدًا
pv-build-slim = نحيلة
pv-build-average = متوسطة
pv-build-sturdy = متينة
pv-build-stout = ممتلئة
pv-build-heavy = ضخمة
pv-eye-colour-light-blue = أزرق فاتح
pv-eye-colour-blue = أزرق
pv-eye-colour-dark-blue = أزرق داكن
pv-eye-colour-grey = رمادي
pv-eye-colour-blue-grey = رمادي مزرق
pv-eye-colour-green = أخضر
pv-eye-colour-grey-green = رمادي مخضر
pv-eye-colour-hazel = عسلي
pv-eye-colour-amber = كهرماني
pv-eye-colour-light-brown = بني فاتح
pv-eye-colour-brown = بني
pv-eye-colour-dark-brown = بني داكن
pv-eye-colour-black = أسود
pv-eye-colour-mixed = مختلط
pv-eye-colour-other = آخر
pv-eye-shape-almond = لوزيتان
pv-eye-shape-round = مستديرتان
pv-eye-shape-hooded = جفن متدلٍّ
pv-eye-shape-monolid = جفن بلا ثنية
pv-eye-shape-deep-set = غائرتان
pv-eye-shape-protruding = بارزتان
pv-eye-shape-upturned = مرفوعتا الطرفين
pv-eye-shape-downturned = منخفضتا الطرفين
pv-eye-shape-other = آخر
pv-eye-spacing-close-set = متقاربتان
pv-eye-spacing-average = متوسطة
pv-eye-spacing-wide-set = متباعدتان
pv-hair-colour-black = أسود
pv-hair-colour-dark-brown = بني داكن
pv-hair-colour-brown = بني
pv-hair-colour-light-brown = بني فاتح
pv-hair-colour-auburn = كستنائي محمر
pv-hair-colour-red = أحمر
pv-hair-colour-strawberry-blond = أشقر محمر
pv-hair-colour-dark-blond = أشقر داكن
pv-hair-colour-blond = أشقر
pv-hair-colour-light-blond = أشقر فاتح
pv-hair-colour-grey = رمادي
pv-hair-colour-white = أبيض
pv-hair-colour-none = بلا شعر
pv-hair-colour-other = آخر
pv-hair-texture-straight = ناعم مستقيم
pv-hair-texture-wavy = مموّج
pv-hair-texture-curly = مجعّد
pv-hair-texture-coily = كثيف التجعيد
pv-hair-texture-other = آخر
pv-hairline-straight = مستقيم
pv-hairline-rounded = مستدير
pv-hairline-widows-peak = مدبّب في المنتصف
pv-hairline-m-shaped = على شكل M
pv-hairline-bell-shaped = على شكل جرس
pv-hairline-uneven = غير منتظم
pv-hairline-receding = منحسر
pv-hairline-bald = أصلع
pv-facial-hair-none = لا يوجد
pv-facial-hair-stubble = لحية خفيفة نامية
pv-facial-hair-moustache = شارب
pv-facial-hair-goatee = سكسوكة
pv-facial-hair-full-beard = لحية كاملة
pv-facial-hair-sideburns = سوالف
pv-facial-hair-other = آخر
pv-body-hair-none = لا يوجد
pv-body-hair-sparse = خفيف
pv-body-hair-moderate = متوسط
pv-body-hair-dense = كثيف
pv-skin-tone-type-i = النمط I — يحترق دائمًا ولا يسمرّ
pv-skin-tone-type-ii = النمط II — يحترق غالبًا ويسمرّ قليلًا
pv-skin-tone-type-iii = النمط III — يحترق أحيانًا ويسمرّ بانتظام
pv-skin-tone-type-iv = النمط IV — نادرًا ما يحترق ويسمرّ جيدًا
pv-skin-tone-type-v = النمط V — نادرًا جدًا ما يحترق
pv-skin-tone-type-vi = النمط VI — لا يحترق أبدًا
pv-skin-undertone-cool = بارد
pv-skin-undertone-neutral = محايد
pv-skin-undertone-warm = دافئ
pv-skin-undertone-olive = زيتوني
pv-freckles-none = لا يوجد
pv-freckles-few = قليل
pv-freckles-moderate = متوسط
pv-freckles-many = كثير
pv-pigmentation-mark-birthmark = وحمة
pv-pigmentation-mark-port-wine-stain = وحمة خمرية
pv-pigmentation-mark-cafe-au-lait-spot = بقعة بلون القهوة بالحليب
pv-pigmentation-mark-depigmented-patch = بقعة فاقدة للصبغة
pv-pigmentation-mark-hyperpigmented-patch = بقعة مفرطة التصبغ
pv-pigmentation-mark-other = أخرى
pv-mole-shape-round = مستديرة
pv-mole-shape-oval = بيضاوية
pv-mole-shape-irregular = غير منتظمة
pv-mole-shape-other = أخرى
pv-face-shape-oval = بيضاوي
pv-face-shape-round = مستدير
pv-face-shape-square = مربع
pv-face-shape-oblong = مستطيل
pv-face-shape-heart = على شكل قلب
pv-face-shape-diamond = ماسي
pv-face-shape-triangular = مثلث
pv-nose-shape-straight = مستقيم
pv-nose-shape-aquiline = معقوف
pv-nose-shape-snub = أفطس قصير
pv-nose-shape-upturned = مرفوع الطرف
pv-nose-shape-flat = مسطّح
pv-nose-shape-broad = عريض
pv-nose-shape-bulbous = منتفخ
pv-nose-shape-crooked = معوج
pv-nose-shape-other = آخر
pv-ear-shape-free-lobe = شحمتان حرتان
pv-ear-shape-attached-lobe = شحمتان ملتصقتان
pv-ear-shape-protruding = بارزتان
pv-ear-shape-close-set = ملتصقتان بالرأس
pv-ear-shape-pointed = مدبّبتان
pv-ear-shape-other = آخر
pv-lip-shape-thin = رفيعتان
pv-lip-shape-medium = متوسطتان
pv-lip-shape-full = ممتلئتان
pv-lip-shape-bow-shaped = قوسيتان
pv-lip-shape-wide = عريضتان
pv-lip-shape-downturned = منخفضتا الزاويتين
pv-lip-shape-other = آخر
pv-dentition-primary = أسنان لبنية
pv-dentition-mixed = مختلطة
pv-dentition-permanent-complete = دائمة كاملة
pv-dentition-permanent-partial-loss = دائمة ناقصة
pv-dentition-edentulous = بلا أسنان
pv-dentition-partial-denture = طقم جزئي
pv-dentition-full-denture = طقم كامل
pv-dentition-implants = زراعة أسنان
pv-malocclusion-normal = إطباق طبيعي
pv-malocclusion-class-i = الصنف الأول
pv-malocclusion-class-ii-division-1 = الصنف الثاني، القسم 1
pv-malocclusion-class-ii-division-2 = الصنف الثاني، القسم 2
pv-malocclusion-class-iii = الصنف الثالث
pv-posture-ideal = سليم
pv-posture-kyphotic-lordotic = حدب وقعس
pv-posture-flat-back = ظهر مسطّح
pv-posture-sway-back = ظهر متقوّس
pv-posture-stooped = منحنٍ
pv-posture-scoliotic = جنفي
pv-posture-other = آخر
pv-gait-brisk = سريعة
pv-gait-average = عادية
pv-gait-slow = بطيئة
pv-gait-shuffling = جارّة للقدمين
pv-gait-limping = عرجاء
pv-gait-waddling = متمايلة
pv-gait-unsteady = غير ثابتة
pv-gait-stiff = متيبّسة
pv-gait-other = أخرى
pv-vocal-timbre-bright = صافٍ
pv-vocal-timbre-dark = قاتم
pv-vocal-timbre-warm = دافئ
pv-vocal-timbre-breathy = مهموس
pv-vocal-timbre-nasal = أغنّ
pv-vocal-timbre-hoarse = أجشّ
pv-vocal-timbre-resonant = رنّان
pv-vocal-timbre-thin = رفيع
pv-vocal-timbre-other = آخر
pv-speech-register-frozen = طقسي جامد
pv-speech-register-formal = رسمي
pv-speech-register-consultative = محايد
pv-speech-register-casual = عامّي
pv-speech-register-intimate = حميمي
pv-handedness-left = أعسر
pv-handedness-right = أيمن
pv-handedness-ambidextrous = أضبط
pv-handedness-mixed = مختلطة
pv-handedness-unknown = غير معروفة
pv-hearing-grade-normal = طبيعي
pv-hearing-grade-mild = ضعف خفيف
pv-hearing-grade-moderate = ضعف متوسط
pv-hearing-grade-moderately-severe = ضعف متوسط إلى شديد
pv-hearing-grade-severe = ضعف شديد
pv-hearing-grade-profound = ضعف عميق
pv-hearing-grade-complete = صمم تام
pv-optical-correction-none = لا يوجد
pv-optical-correction-glasses = نظارات
pv-optical-correction-contact-lenses = عدسات لاصقة
pv-optical-correction-glasses-and-contact-lenses = نظارات وعدسات لاصقة
pv-optical-correction-refractive-surgery = جراحة انكسارية
pv-optical-correction-intraocular-lens = عدسة داخل العين
pv-optical-correction-other = آخر
pv-rhesus-positive = RhD موجب
pv-rhesus-negative = RhD سالب
pv-rhesus-weak-d = D ضعيف
pv-rhesus-unknown = غير معروف
pv-icd10-chapter-infectious-parasitic = I أمراض معدية وطفيلية
pv-icd10-chapter-neoplasms = II أورام
pv-icd10-chapter-blood-immune = III أمراض الدم والجهاز المناعي
pv-icd10-chapter-endocrine-metabolic = IV أمراض الغدد الصم والتغذية والاستقلاب
pv-icd10-chapter-mental-behavioural = V اضطرابات نفسية وسلوكية
pv-icd10-chapter-nervous-system = VI أمراض الجهاز العصبي
pv-icd10-chapter-eye-adnexa = VII أمراض العين وملحقاتها
pv-icd10-chapter-ear-mastoid = VIII أمراض الأذن والناتئ الخشائي
pv-icd10-chapter-circulatory = IX أمراض جهاز الدوران
pv-icd10-chapter-respiratory = X أمراض الجهاز التنفسي
pv-icd10-chapter-digestive = XI أمراض الجهاز الهضمي
pv-icd10-chapter-skin = XII أمراض الجلد والنسيج تحت الجلد
pv-icd10-chapter-musculoskeletal = XIII أمراض الجهاز العضلي الهيكلي
pv-icd10-chapter-genitourinary = XIV أمراض الجهاز البولي التناسلي
pv-icd10-chapter-pregnancy-childbirth = XV الحمل والولادة
pv-icd10-chapter-perinatal = XVI حالات الفترة المحيطة بالولادة
pv-icd10-chapter-congenital = XVII التشوهات الخلقية
pv-icd10-chapter-ill-defined = XVIII أعراض وأسباب غير محددة
pv-icd10-chapter-injury-poisoning = XIX الإصابات والتسممات
pv-icd10-chapter-external-causes = XX أسباب خارجية
pv-icd10-chapter-health-factors = XXI عوامل مؤثرة في الحالة الصحية
pv-icd10-chapter-special-purposes = XXII رموز لأغراض خاصة
pv-diagnosis-status-diagnosed = مشخّص طبيًا
pv-diagnosis-status-suspected = مشتبه به
pv-diagnosis-status-self-reported = بإفادة الشخص أو العائلة
pv-diagnosis-status-unknown = غير معروف
pv-prosthesis-kind-limb = طرف
pv-prosthesis-kind-joint = مفصل اصطناعي
pv-prosthesis-kind-ocular = عين اصطناعية
pv-prosthesis-kind-dental = تعويض سني
pv-prosthesis-kind-auditory = تعويض سمعي
pv-prosthesis-kind-breast = تعويض ثدي
pv-prosthesis-kind-other = آخر
pv-implant-kind-orthopaedic = تقويمي عظمي
pv-implant-kind-dental = سنّي
pv-implant-kind-cochlear = قوقعي
pv-implant-kind-breast = ثدي
pv-implant-kind-intraocular-lens = عدسة داخل العين
pv-implant-kind-contraceptive = لمنع الحمل
pv-implant-kind-cosmetic = تجميلي
pv-implant-kind-other = آخر
pv-device-kind-pacemaker = ناظم خطى القلب
pv-device-kind-implantable-defibrillator = مزيل رجفان مزروع
pv-device-kind-cardiac-resynchronisation = جهاز إعادة تزامن القلب
pv-device-kind-ventricular-assist = جهاز مساعدة البطين
pv-device-kind-neurostimulator = منبّه عصبي
pv-device-kind-insulin-pump = مضخة إنسولين
pv-device-kind-drug-port = منفذ دوائي مزروع
pv-device-kind-shunt = تحويلة
pv-device-kind-stent = دعامة
pv-device-kind-other = آخر
pv-allergy-type-drug = دواء
pv-allergy-type-food = طعام
pv-allergy-type-environmental = بيئية
pv-allergy-type-insect-venom = سم الحشرات
pv-allergy-type-latex = اللاتكس
pv-allergy-type-other = أخرى
pv-allergy-severity-mild = خفيفة
pv-allergy-severity-moderate = متوسطة
pv-allergy-severity-severe = شديدة
pv-allergy-severity-anaphylactic = تأقية
pv-allergy-severity-unknown = غير معروفة
pv-pathogen-diphtheria = الخناق
pv-pathogen-tetanus = الكزاز
pv-pathogen-pertussis = السعال الديكي
pv-pathogen-poliomyelitis = شلل الأطفال
pv-pathogen-measles = الحصبة
pv-pathogen-mumps = النكاف
pv-pathogen-rubella = الحصبة الألمانية
pv-pathogen-varicella = الجديري المائي
pv-pathogen-smallpox = الجدري
pv-pathogen-tuberculosis = السل
pv-pathogen-hepatitis-a = التهاب الكبد A
pv-pathogen-hepatitis-b = التهاب الكبد B
pv-pathogen-hepatitis-c = التهاب الكبد C
pv-pathogen-haemophilus-influenzae-b = المستدمية النزلية من النوع b
pv-pathogen-pneumococcal = المكورات الرئوية
pv-pathogen-meningococcal = المكورات السحائية
pv-pathogen-human-papillomavirus = فيروس الورم الحليمي البشري
pv-pathogen-influenza = الإنفلونزا
pv-pathogen-covid-19 = كوفيد-19
pv-pathogen-rotavirus = الفيروس العجلي
pv-pathogen-yellow-fever = الحمى الصفراء
pv-pathogen-typhoid = التيفوئيد
pv-pathogen-cholera = الكوليرا
pv-pathogen-rabies = داء الكلب
pv-pathogen-japanese-encephalitis = التهاب الدماغ الياباني
pv-pathogen-tick-borne-encephalitis = التهاب الدماغ المنقول بالقراد
pv-pathogen-hiv = فيروس نقص المناعة البشرية
pv-pathogen-syphilis = الزهري
pv-pathogen-toxoplasmosis = داء المقوسات
pv-pathogen-cytomegalovirus = الفيروس المضخم للخلايا
pv-pathogen-epstein-barr = فيروس إبشتاين-بار
pv-pathogen-other = آخر
pv-vaccination-status-vaccinated = ملقّح
pv-vaccination-status-partially-vaccinated = ملقّح جزئيًا
pv-vaccination-status-unvaccinated = غير ملقّح
pv-vaccination-status-contraindicated = موانع طبية
pv-vaccination-status-unknown = غير معروف
pv-serology-result-positive = إيجابي
pv-serology-result-negative = سلبي
pv-serology-result-equivocal = ملتبس
pv-serology-result-unknown = غير معروف
pv-lab-panel-basic-metabolic = لوحة الاستقلاب الأساسية
pv-lab-panel-lipid = لوحة الدهون
pv-lab-panel-liver = وظائف الكبد
pv-lab-panel-renal = وظائف الكلى
pv-lab-panel-glycated-haemoglobin = الهيموغلوبين السكري
pv-lab-panel-iron = استقلاب الحديد
pv-lab-analyte-sodium = الصوديوم
pv-lab-analyte-potassium = البوتاسيوم
pv-lab-analyte-chloride = الكلوريد
pv-lab-analyte-bicarbonate = البيكربونات
pv-lab-analyte-urea = اليوريا
pv-lab-analyte-creatinine = الكرياتينين
pv-lab-analyte-glucose = الغلوكوز
pv-lab-analyte-calcium = الكالسيوم
pv-lab-analyte-total-cholesterol = الكوليسترول الكلي
pv-lab-analyte-ldl-cholesterol = كوليسترول LDL
pv-lab-analyte-hdl-cholesterol = كوليسترول HDL
pv-lab-analyte-triglycerides = الدهون الثلاثية
pv-lab-analyte-non-hdl-cholesterol = الكوليسترول غير HDL
pv-lab-analyte-alt = ناقلة أمين الألانين (ALT)
pv-lab-analyte-ast = ناقلة أمين الأسبارتات (AST)
pv-lab-analyte-alp = الفوسفاتاز القلوي (ALP)
pv-lab-analyte-ggt = ناقلة غاما غلوتاميل (GGT)
pv-lab-analyte-total-bilirubin = البيليروبين الكلي
pv-lab-analyte-direct-bilirubin = البيليروبين المباشر
pv-lab-analyte-albumin = الألبومين
pv-lab-analyte-total-protein = البروتين الكلي
pv-lab-analyte-egfr = معدل الترشيح الكبيبي المقدّر
pv-lab-analyte-uric-acid = حمض اليوريك
pv-lab-analyte-phosphate = الفوسفات
pv-lab-analyte-urine-albumin-creatinine-ratio = نسبة الألبومين إلى الكرياتينين في البول
pv-lab-analyte-hba1c = HbA1c
pv-lab-analyte-serum-iron = حديد المصل
pv-lab-analyte-ferritin = الفيريتين
pv-lab-analyte-transferrin = الترانسفرين
pv-lab-analyte-transferrin-saturation = تشبع الترانسفرين
pv-lab-analyte-tibc = السعة الكلية لربط الحديد
pv-lab-flag-low = منخفض
pv-lab-flag-normal = طبيعي
pv-lab-flag-high = مرتفع
pv-lab-flag-critical-low = منخفض بدرجة حرجة
pv-lab-flag-critical-high = مرتفع بدرجة حرجة
pv-nutrient-vitamin-a = فيتامين A
pv-nutrient-thiamine = الثيامين (B1)
pv-nutrient-riboflavin = الريبوفلافين (B2)
pv-nutrient-niacin = النياسين (B3)
pv-nutrient-vitamin-b6 = فيتامين B6
pv-nutrient-folate = الفولات (B9)
pv-nutrient-vitamin-b12 = فيتامين B12
pv-nutrient-vitamin-c = فيتامين C
pv-nutrient-vitamin-d = فيتامين D
pv-nutrient-vitamin-e = فيتامين E
pv-nutrient-vitamin-k = فيتامين K
pv-nutrient-iron = الحديد
pv-nutrient-zinc = الزنك
pv-nutrient-magnesium = المغنيسيوم
pv-nutrient-calcium = الكالسيوم
pv-nutrient-iodine = اليود
pv-nutrient-selenium = السيلينيوم
pv-nutrient-copper = النحاس
pv-nutrient-potassium = البوتاسيوم
pv-nutrient-phosphorus = الفوسفور
pv-nutrient-other = آخر
pv-sleep-disorder-insomnia = الأرق
pv-sleep-disorder-sleep-related-breathing = اضطرابات التنفس أثناء النوم
pv-sleep-disorder-central-hypersomnolence = فرط النعاس المركزي
pv-sleep-disorder-circadian-rhythm = اضطرابات إيقاع النوم واليقظة
pv-sleep-disorder-parasomnia = الخطل النومي
pv-sleep-disorder-sleep-related-movement = اضطرابات الحركة أثناء النوم
pv-sleep-disorder-other = أخرى
pv-assessment-instrument-phq-9 = PHQ-9
pv-assessment-instrument-gad-7 = GAD-7
pv-assessment-instrument-bdi-ii = BDI-II
pv-assessment-instrument-hads = HADS
pv-assessment-instrument-k10 = K10
pv-assessment-instrument-gds-15 = GDS-15
pv-assessment-instrument-mmse = MMSE
pv-assessment-instrument-moca = MoCA
pv-assessment-instrument-audit = AUDIT
pv-assessment-instrument-clinical-interview = مقابلة سريرية
pv-assessment-instrument-other = أخرى
pv-assessment-severity-none-minimal = لا شيء أو طفيف
pv-assessment-severity-mild = خفيف
pv-assessment-severity-moderate = متوسط
pv-assessment-severity-moderately-severe = متوسط إلى شديد
pv-assessment-severity-severe = شديد
pv-reference-build-grch36 = GRCh36 (hg18)
pv-reference-build-grch37 = GRCh37 (hg19)
pv-reference-build-grch38 = GRCh38 (hg38)
pv-reference-build-t2t-chm13 = T2T-CHM13
pv-genomic-file-format-raw-microarray = بيانات خام من رقاقة التنميط
pv-genomic-file-format-fastq = FASTQ
pv-genomic-file-format-bam = BAM
pv-genomic-file-format-cram = CRAM
pv-genomic-file-format-vcf = VCF
pv-genomic-file-format-gvcf = gVCF
pv-genomic-file-format-other = أخرى
pv-zygosity-heterozygous = متخالف اللواقح
pv-zygosity-homozygous = متماثل اللواقح
pv-zygosity-hemizygous = نصف لاقحي
pv-zygosity-compound-heterozygous = متخالف اللواقح المركّب
pv-clinical-significance-pathogenic = ممرض
pv-clinical-significance-likely-pathogenic = ممرض على الأرجح
pv-clinical-significance-uncertain-significance = غير مؤكد الدلالة
pv-clinical-significance-likely-benign = حميد على الأرجح
pv-clinical-significance-benign = حميد
pv-inheritance-pattern-autosomal-dominant = جسدي سائد
pv-inheritance-pattern-autosomal-recessive = جسدي متنحٍّ
pv-inheritance-pattern-x-linked-dominant = سائد مرتبط بالكروموسوم X
pv-inheritance-pattern-x-linked-recessive = متنحٍّ مرتبط بالكروموسوم X
pv-inheritance-pattern-y-linked = مرتبط بالكروموسوم Y
pv-inheritance-pattern-mitochondrial = ميتوكوندري
pv-inheritance-pattern-multifactorial = متعدد العوامل
pv-inheritance-pattern-unknown = غير معروف
pv-carrier-status-affected = مصاب
pv-carrier-status-carrier = حامل
pv-carrier-status-not-carrier = غير حامل
pv-carrier-status-unknown = غير معروف
pv-epigenetic-clock-horvath = Horvath
pv-epigenetic-clock-hannum = Hannum
pv-epigenetic-clock-phenoage = PhenoAge
pv-epigenetic-clock-grimage = GrimAge
pv-epigenetic-clock-dunedinpace = DunedinPACE
pv-epigenetic-clock-other = أخرى
pv-metaboliser-status-poor = مستقلِب بطيء
pv-metaboliser-status-intermediate = مستقلِب متوسط
pv-metaboliser-status-normal = مستقلِب طبيعي
pv-metaboliser-status-rapid = مستقلِب سريع
pv-metaboliser-status-ultrarapid = مستقلِب فائق السرعة
pv-autopsy-not-performed = لم يُجرَ
pv-autopsy-clinical = تشريح سريري
pv-autopsy-forensic = تشريح شرعي
pv-autopsy-external-examination = فحص ظاهري فقط
pv-autopsy-unknown = غير معروف
pv-disposition-burial = دفن
pv-disposition-cremation = حرق
pv-disposition-entombment = دفن في ضريح
pv-disposition-burial-at-sea = دفن في البحر
pv-disposition-natural-burial = دفن طبيعي
pv-disposition-body-donation = التبرع بالجثمان للعلم
pv-disposition-other = آخر
pv-disposition-unknown = غير معروف
pv-address-use-principal = محل الإقامة الرئيسي
pv-address-use-secondary = مسكن ثانوي
pv-address-use-temporary = إقامة مؤقتة
pv-address-use-postal = عنوان بريدي
pv-address-use-other = آخر
pv-nationality-mode-descent = بالنسب
pv-nationality-mode-birth-in-territory = بالولادة على الإقليم
pv-nationality-mode-naturalisation = بالتجنس
pv-nationality-mode-marriage = بالزواج
pv-nationality-mode-registration = بالتسجيل
pv-nationality-mode-restoration = بالاسترداد
pv-nationality-mode-state-succession = بتغير السيادة
pv-nationality-mode-other = أخرى
pv-language-proficiency-a1 = A1 مبتدئ
pv-language-proficiency-a2 = A2 أساسي
pv-language-proficiency-b1 = B1 متوسط
pv-language-proficiency-b2 = B2 فوق المتوسط
pv-language-proficiency-c1 = C1 متقدم
pv-language-proficiency-c2 = C2 متمكن
pv-language-proficiency-native = اللغة الأولى
pv-isced-level-isced-0 = 0 التعليم في الطفولة المبكرة
pv-isced-level-isced-1 = 1 التعليم الابتدائي
pv-isced-level-isced-2 = 2 التعليم الإعدادي
pv-isced-level-isced-3 = 3 التعليم الثانوي
pv-isced-level-isced-4 = 4 ما بعد الثانوي غير العالي
pv-isced-level-isced-5 = 5 التعليم العالي قصير الأمد
pv-isced-level-isced-6 = 6 البكالوريوس أو ما يعادله
pv-isced-level-isced-7 = 7 الماجستير أو ما يعادله
pv-isced-level-isced-8 = 8 الدكتوراه أو ما يعادلها
pv-income-quintile-q1 = الخُمس الأدنى
pv-income-quintile-q2 = الخُمس الثاني
pv-income-quintile-q3 = الخُمس الأوسط
pv-income-quintile-q4 = الخُمس الرابع
pv-income-quintile-q5 = الخُمس الأعلى
pv-pay-period-hourly = بالساعة
pv-pay-period-daily = باليوم
pv-pay-period-weekly = بالأسبوع
pv-pay-period-monthly = بالشهر
pv-pay-period-annual = بالسنة
pv-tenure-owned = ملكية
pv-tenure-co-owned = ملكية مشتركة
pv-tenure-leasehold = إيجار طويل الأمد
pv-tenure-rented = إيجار
pv-tenure-usufruct = حق انتفاع
pv-tenure-other = أخرى
pv-distinction-kind-order = وسام رتبة
pv-distinction-kind-decoration = وسام
pv-distinction-kind-medal = ميدالية
pv-distinction-kind-title = لقب فخري
pv-distinction-kind-other = أخرى
pv-military-service-army = القوات البرية
pv-military-service-navy = القوات البحرية
pv-military-service-air-force = القوات الجوية
pv-military-service-marines = مشاة البحرية
pv-military-service-gendarmerie = الدرك
pv-military-service-border-guard = حرس الحدود
pv-military-service-national-guard = الحرس الوطني
pv-military-service-other = أخرى
pv-rank-category-enlisted = الأفراد
pv-rank-category-non-commissioned = ضباط الصف
pv-rank-category-warrant = المساعدون
pv-rank-category-officer-cadet = طلاب الكليات العسكرية
pv-rank-category-junior-officer = الضباط الأصاغر
pv-rank-category-senior-officer = الضباط الأقدمون
pv-rank-category-general-officer = الضباط العامّون
pv-iccs-section-acts-leading-to-death = 01 أفعال مفضية إلى الموت
pv-iccs-section-acts-causing-harm = 02 أفعال مسببة للأذى
pv-iccs-section-sexual-acts = 03 أفعال ضارة ذات طابع جنسي
pv-iccs-section-property-with-violence = 04 ضد الممتلكات مع العنف
pv-iccs-section-property-only = 05 ضد الممتلكات فقط
pv-iccs-section-controlled-substances = 06 المواد الخاضعة للرقابة
pv-iccs-section-fraud-deception-corruption = 07 الاحتيال والخداع والفساد
pv-iccs-section-public-order-and-state = 08 ضد النظام العام والدولة
pv-iccs-section-public-safety-and-security = 09 ضد السلامة العامة
pv-iccs-section-natural-environment = 10 ضد البيئة الطبيعية
pv-iccs-section-other-criminal-acts = 11 جرائم أخرى
pv-case-outcome-convicted = مُدان
pv-case-outcome-acquitted = مُبرّأ
pv-case-outcome-dismissed = حُفظت الدعوى
pv-case-outcome-conviction-quashed = أُلغي الحكم
pv-case-outcome-pardoned = عُفي عنه
pv-case-outcome-amnestied = شمله العفو العام
pv-case-outcome-expunged = مُحي من السجل
pv-case-outcome-pending = قيد النظر
pv-case-outcome-unknown = غير معروف
pv-religion-buddhism = البوذية
pv-religion-christianity-catholic = المسيحية: الكاثوليكية
pv-religion-christianity-orthodox = المسيحية: الأرثوذكسية
pv-religion-christianity-protestant = المسيحية: البروتستانتية
pv-religion-christianity-other = المسيحية: أخرى
pv-religion-hinduism = الهندوسية
pv-religion-islam-sunni = الإسلام: السنة
pv-religion-islam-shia = الإسلام: الشيعة
pv-religion-islam-other = الإسلام: أخرى
pv-religion-jainism = الجاينية
pv-religion-judaism = اليهودية
pv-religion-sikhism = السيخية
pv-religion-bahai = البهائية
pv-religion-shinto = الشنتو
pv-religion-taoism = الطاوية
pv-religion-zoroastrianism = الزرادشتية
pv-religion-traditional = ديانة تقليدية أو شعبية
pv-religion-other = أخرى
pv-religion-none = بلا دين
pv-religion-unknown = غير معروف
pv-sacrament-baptism = المعمودية
pv-sacrament-confirmation = التثبيت
pv-sacrament-first-communion = المناولة الأولى
pv-sacrament-reconciliation = الاعتراف
pv-sacrament-anointing-of-the-sick = مسحة المرضى
pv-sacrament-holy-orders = الكهنوت
pv-sacrament-matrimony = الزواج
pv-sacrament-other-rite = شعيرة أخرى
pv-political-position-far-left = أقصى اليسار
pv-political-position-left = اليسار
pv-political-position-centre-left = يسار الوسط
pv-political-position-centre = الوسط
pv-political-position-centre-right = يمين الوسط
pv-political-position-right = اليمين
pv-political-position-far-right = أقصى اليمين
pv-political-position-apolitical = غير مسيّس
pv-political-position-other = خارج هذا المحور
pv-political-position-unknown = غير معروف
pv-membership-kind-trade-union = نقابة عمالية
pv-membership-kind-political-party = حزب سياسي
pv-membership-kind-professional-body = هيئة مهنية
pv-membership-kind-religious-order = رهبنة
pv-membership-kind-religious-association = جمعية دينية
pv-membership-kind-fraternal-order = أخوية
pv-membership-kind-veterans-association = جمعية قدامى المحاربين
pv-membership-kind-sports-club = نادٍ رياضي
pv-membership-kind-cultural-association = جمعية ثقافية
pv-membership-kind-charitable-association = جمعية خيرية
pv-membership-kind-other = أخرى
pv-personality-instrument-neo-pi-3 = NEO-PI-3
pv-personality-instrument-neo-ffi-3 = NEO-FFI-3
pv-personality-instrument-bfi-2 = BFI-2
pv-personality-instrument-ipip-neo-120 = IPIP-NEO-120
pv-personality-instrument-tipi = TIPI
pv-personality-instrument-hexaco-pi-r = HEXACO-PI-R
pv-personality-instrument-observer-rating = تقييم من شخص عرفه
pv-personality-instrument-inferred = مستنتج من السجلات
pv-personality-instrument-other = أخرى
pv-introversion-extraversion-strongly-introverted = منطوٍ جدًا
pv-introversion-extraversion-introverted = منطوٍ
pv-introversion-extraversion-ambiverted = متوازن
pv-introversion-extraversion-extraverted = منبسط
pv-introversion-extraversion-strongly-extraverted = منبسط جدًا
pv-stress-tolerance-very-low = منخفض جدًا
pv-stress-tolerance-low = منخفض
pv-stress-tolerance-moderate = متوسط
pv-stress-tolerance-high = مرتفع
pv-stress-tolerance-very-high = مرتفع جدًا
pv-decision-style-rational = عقلاني
pv-decision-style-intuitive = حدسي
pv-decision-style-dependent = اتكالي
pv-decision-style-avoidant = تجنّبي
pv-decision-style-spontaneous = عفوي
pv-sport-level-recreational = ترفيهي
pv-sport-level-amateur-competitive = تنافسي للهواة
pv-sport-level-semi-professional = شبه احترافي
pv-sport-level-professional = احترافي
pv-diet-omnivore = نظام متنوع
pv-diet-flexitarian = نباتي مرن
pv-diet-pescatarian = نباتي يأكل السمك
pv-diet-vegetarian = نباتي
pv-diet-vegan = نباتي صرف
pv-diet-other = آخر
pv-substance-tobacco = التبغ والنيكوتين
pv-substance-alcohol = الكحول
pv-substance-cannabis = القنّب
pv-substance-opioids = الأفيونيات
pv-substance-stimulants = المنشطات
pv-substance-sedatives-hypnotics = المهدئات والمنوّمات
pv-substance-hallucinogens = المهلوسات
pv-substance-inhalants = المستنشقات
pv-substance-gambling = القمار
pv-substance-gaming = ألعاب الفيديو
pv-substance-other = أخرى
pv-use-pattern-occasional-use = تعاطٍ عرضي
pv-use-pattern-regular-use = تعاطٍ منتظم
pv-use-pattern-harmful-use = تعاطٍ ضار
pv-use-pattern-dependence = اعتماد
pv-use-pattern-in-remission = في طور التعافي
pv-lineage-biological = بيولوجية
pv-lineage-adoptive = بالتبنّي
pv-lineage-foster = بالرعاية
pv-lineage-step = من زوج أحد الوالدين
pv-lineage-guardianship = بالوصاية
pv-lineage-unknown = غير معروفة
pv-link-relation-godparent = عرّاب
pv-link-relation-godchild = فليون
pv-link-relation-witness = شاهد
pv-link-relation-officiant = قائم بالمراسم
pv-link-relation-business-partner = شريك تجاري
pv-link-relation-employer = صاحب عمل
pv-link-relation-employee = موظف
pv-link-relation-mentor = مرشد
pv-link-relation-apprentice = متدرّب
pv-link-relation-close-friend = صديق مقرّب
pv-link-relation-neighbour = جار
pv-link-relation-guardian = وصي
pv-link-relation-ward = قاصر تحت الوصاية
pv-link-relation-other = أخرى
pv-country-AD = أندورا
pv-country-AE = الإمارات العربية المتحدة
pv-country-AF = أفغانستان
pv-country-AG = أنتيغوا وبربودا
pv-country-AI = أنغويلا
pv-country-AL = ألبانيا
pv-country-AM = أرمينيا
pv-country-AO = أنغولا
pv-country-AQ = أنتاركتيكا
pv-country-AR = الأرجنتين
pv-country-AS = ساموا الأمريكية
pv-country-AT = النمسا
pv-country-AU = أستراليا
pv-country-AW = أروبا
pv-country-AX = جزر آلاند
pv-country-AZ = أذربيجان
pv-country-BA = البوسنة والهرسك
pv-country-BB = بربادوس
pv-country-BD = بنغلاديش
pv-country-BE = بلجيكا
pv-country-BF = بوركينا فاسو
pv-country-BG = بلغاريا
pv-country-BH = البحرين
pv-country-BI = بوروندي
pv-country-BJ = بنين
pv-country-BL = سان بارتليمي
pv-country-BM = برمودا
pv-country-BN = بروناي
pv-country-BO = بوليفيا
pv-country-BQ = هولندا الكاريبية
pv-country-BR = البرازيل
pv-country-BS = جزر البهاما
pv-country-BT = بوتان
pv-country-BV = جزيرة بوفيه
pv-country-BW = بوتسوانا
pv-country-BY = بيلاروس
pv-country-BZ = بليز
pv-country-CA = كندا
pv-country-CC = جزر كوكوس (كيلينغ)
pv-country-CD = الكونغو - كينشاسا
pv-country-CF = جمهورية أفريقيا الوسطى
pv-country-CG = الكونغو - برازافيل
pv-country-CH = سويسرا
pv-country-CI = ساحل العاج
pv-country-CK = جزر كوك
pv-country-CL = تشيلي
pv-country-CM = الكاميرون
pv-country-CN = الصين
pv-country-CO = كولومبيا
pv-country-CR = كوستاريكا
pv-country-CU = كوبا
pv-country-CV = الرأس الأخضر
pv-country-CW = كوراساو
pv-country-CX = جزيرة كريسماس
pv-country-CY = قبرص
pv-country-CZ = التشيك
pv-country-DE = ألمانيا
pv-country-DJ = جيبوتي
pv-country-DK = الدانمرك
pv-country-DM = دومينيكا
pv-country-DO = جمهورية الدومينيكان
pv-country-DZ = الجزائر
pv-country-EC = الإكوادور
pv-country-EE = إستونيا
pv-country-EG = مصر
pv-country-EH = الصحراء الغربية
pv-country-ER = إريتريا
pv-country-ES = إسبانيا
pv-country-ET = إثيوبيا
pv-country-FI = فنلندا
pv-country-FJ = فيجي
pv-country-FK = جزر فوكلاند
pv-country-FM = ميكرونيزيا
pv-country-FO = جزر فارو
pv-country-FR = فرنسا
pv-country-GA = الغابون
pv-country-GB = المملكة المتحدة
pv-country-GD = غرينادا
pv-country-GE = جورجيا
pv-country-GF = غويانا الفرنسية
pv-country-GG = غيرنزي
pv-country-GH = غانا
pv-country-GI = جبل طارق
pv-country-GL = غرينلاند
pv-country-GM = غامبيا
pv-country-GN = غينيا
pv-country-GP = غوادلوب
pv-country-GQ = غينيا الاستوائية
pv-country-GR = اليونان
pv-country-GS = جورجيا الجنوبية وجزر ساندويتش الجنوبية
pv-country-GT = غواتيمالا
pv-country-GU = غوام
pv-country-GW = غينيا بيساو
pv-country-GY = غيانا
pv-country-HK = هونغ كونغ الصينية (منطقة إدارية خاصة)
pv-country-HM = جزيرة هيرد وجزر ماكدونالد
pv-country-HN = هندوراس
pv-country-HR = كرواتيا
pv-country-HT = هايتي
pv-country-HU = هنغاريا
pv-country-ID = إندونيسيا
pv-country-IE = أيرلندا
pv-country-IL = إسرائيل
pv-country-IM = جزيرة مان
pv-country-IN = الهند
pv-country-IO = الإقليم البريطاني في المحيط الهندي
pv-country-IQ = العراق
pv-country-IR = إيران
pv-country-IS = آيسلندا
pv-country-IT = إيطاليا
pv-country-JE = جيرسي
pv-country-JM = جامايكا
pv-country-JO = الأردن
pv-country-JP = اليابان
pv-country-KE = كينيا
pv-country-KG = قيرغيزستان
pv-country-KH = كمبوديا
pv-country-KI = كيريباتي
pv-country-KM = جزر القمر
pv-country-KN = سانت كيتس ونيفيس
pv-country-KP = كوريا الشمالية
pv-country-KR = كوريا الجنوبية
pv-country-KW = الكويت
pv-country-KY = جزر كايمان
pv-country-KZ = كازاخستان
pv-country-LA = لاوس
pv-country-LB = لبنان
pv-country-LC = سانت لوسيا
pv-country-LI = ليختنشتاين
pv-country-LK = سريلانكا
pv-country-LR = ليبيريا
pv-country-LS = ليسوتو
pv-country-LT = ليتوانيا
pv-country-LU = لوكسمبورغ
pv-country-LV = لاتفيا
pv-country-LY = ليبيا
pv-country-MA = المغرب
pv-country-MC = موناكو
pv-country-MD = مولدوفا
pv-country-ME = الجبل الأسود
pv-country-MF = سان مارتن
pv-country-MG = مدغشقر
pv-country-MH = جزر مارشال
pv-country-MK = مقدونيا الشمالية
pv-country-ML = مالي
pv-country-MM = ميانمار (بورما)
pv-country-MN = منغوليا
pv-country-MO = منطقة ماكاو الإدارية الخاصة
pv-country-MP = جزر ماريانا الشمالية
pv-country-MQ = جزر المارتينيك
pv-country-MR = موريتانيا
pv-country-MS = مونتسرات
pv-country-MT = مالطا
pv-country-MU = موريشيوس
pv-country-MV = جزر المالديف
pv-country-MW = ملاوي
pv-country-MX = المكسيك
pv-country-MY = ماليزيا
pv-country-MZ = موزمبيق
pv-country-NA = ناميبيا
pv-country-NC = كاليدونيا الجديدة
pv-country-NE = النيجر
pv-country-NF = جزيرة نورفولك
pv-country-NG = نيجيريا
pv-country-NI = نيكاراغوا
pv-country-NL = هولندا
pv-country-NO = النرويج
pv-country-NP = نيبال
pv-country-NR = ناورو
pv-country-NU = نيوي
pv-country-NZ = نيوزيلندا
pv-country-OM = عُمان
pv-country-PA = بنما
pv-country-PE = بيرو
pv-country-PF = بولينيزيا الفرنسية
pv-country-PG = بابوا غينيا الجديدة
pv-country-PH = الفلبين
pv-country-PK = باكستان
pv-country-PL = بولندا
pv-country-PM = سان بيير ومكويلون
pv-country-PN = جزر بيتكيرن
pv-country-PR = بورتوريكو
pv-country-PS = الأراضي الفلسطينية
pv-country-PT = البرتغال
pv-country-PW = بالاو
pv-country-PY = باراغواي
pv-country-QA = قطر
pv-country-RE = روينيون
pv-country-RO = رومانيا
pv-country-RS = صربيا
pv-country-RU = روسيا
pv-country-RW = رواندا
pv-country-SA = المملكة العربية السعودية
pv-country-SB = جزر سليمان
pv-country-SC = سيشل
pv-country-SD = السودان
pv-country-SE = السويد
pv-country-SG = سنغافورة
pv-country-SH = سانت هيلينا
pv-country-SI = سلوفينيا
pv-country-SJ = سفالبارد وجان ماين
pv-country-SK = سلوفاكيا
pv-country-SL = سيراليون
pv-country-SM = سان مارينو
pv-country-SN = السنغال
pv-country-SO = الصومال
pv-country-SR = سورينام
pv-country-SS = جنوب السودان
pv-country-ST = ساو تومي وبرينسيبي
pv-country-SV = السلفادور
pv-country-SX = سانت مارتن
pv-country-SY = سوريا
pv-country-SZ = إسواتيني
pv-country-TC = جزر توركس وكايكوس
pv-country-TD = تشاد
pv-country-TF = الأقاليم الجنوبية الفرنسية
pv-country-TG = توغو
pv-country-TH = تايلاند
pv-country-TJ = طاجيكستان
pv-country-TK = توكيلو
pv-country-TL = تيمور - ليشتي
pv-country-TM = تركمانستان
pv-country-TN = تونس
pv-country-TO = تونغا
pv-country-TR = تركيا
pv-country-TT = ترينيداد وتوباغو
pv-country-TV = توفالو
pv-country-TW = تايوان
pv-country-TZ = تنزانيا
pv-country-UA = أوكرانيا
pv-country-UG = أوغندا
pv-country-UM = جزر الولايات المتحدة النائية
pv-country-US = الولايات المتحدة
pv-country-UY = أورغواي
pv-country-UZ = أوزبكستان
pv-country-VA = الفاتيكان
pv-country-VC = سانت فنسنت وجزر غرينادين
pv-country-VE = فنزويلا
pv-country-VG = جزر فيرجن البريطانية
pv-country-VI = جزر فيرجن التابعة للولايات المتحدة
pv-country-VN = فيتنام
pv-country-VU = فانواتو
pv-country-WF = جزر والس وفوتونا
pv-country-WS = ساموا
pv-country-YE = اليمن
pv-country-YT = مايوت
pv-country-ZA = جنوب أفريقيا
pv-country-ZM = زامبيا
pv-country-ZW = زيمبابوي
pv-country-SU = الاتحاد السوفيتي
pv-country-DD = ألمانيا الشرقية
pv-country-YU = يوغوسلافيا
pv-country-CS = تشيكوسلوفاكيا
pv-country-OT = الدولة العثمانية
lang-aa = الأفارية
lang-ab = الأبخازية
lang-ae = الأفستية
lang-af = الأفريقانية
lang-ak = الأكانية
lang-am = الأمهرية
lang-an = الأراغونية
lang-ar = العربية
lang-as = الأسامية
lang-av = الأوارية
lang-ay = الأيمارا
lang-az = الأذربيجانية
lang-ba = الباشكيرية
lang-be = البيلاروسية
lang-bg = البلغارية
lang-bi = البيسلامية
lang-bm = البامبارا
lang-bn = البنغالية
lang-bo = التبتية
lang-br = البريتونية
lang-bs = البوسنية
lang-ca = الكتالانية
lang-ce = الشيشانية
lang-ch = التشامورو
lang-co = الكورسيكية
lang-cr = الكرى
lang-cs = التشيكية
lang-cu = سلافية كنسية
lang-cv = التشوفاشي
lang-cy = الويلزية
lang-da = الدانمركية
lang-de = الألمانية
lang-dv = المالديفية
lang-dz = الزونخاية
lang-ee = الإيوي
lang-el = اليونانية
lang-en = الإنجليزية
lang-eo = الإسبرانتو
lang-es = الإسبانية
lang-et = الإستونية
lang-eu = الباسكية
lang-fa = الفارسية
lang-ff = الفولانية
lang-fi = الفنلندية
lang-fj = الفيجية
lang-fo = الفاروية
lang-fr = الفرنسية
lang-fy = الفريزيان
lang-ga = الأيرلندية
lang-gd = الغيلية الأسكتلندية
lang-gl = الجاليكية
lang-gn = الغوارانية
lang-gu = الغوجاراتية
lang-gv = المنكية
lang-ha = الهوسا
lang-he = العبرية
lang-hi = الهندية
lang-ho = الهيري موتو
lang-hr = الكرواتية
lang-ht = الكريولية الهايتية
lang-hu = الهنغارية
lang-hy = الأرمنية
lang-hz = الهيريرو
lang-ia = اللّغة الوسيطة
lang-id = الإندونيسية
lang-ie = الإنترلينج
lang-ig = الإيجبو
lang-ii = السيتشيون يي
lang-ik = الإينبياك
lang-io = الإيدو
lang-is = الأيسلندية
lang-it = الإيطالية
lang-iu = الإينكتيتت
lang-ja = اليابانية
lang-jv = الجاوية
lang-ka = الجورجية
lang-kg = الكونغو
lang-ki = الكيكيو
lang-kj = الكيونياما
lang-kk = الكازاخستانية
lang-kl = الكالاليست
lang-km = الخميرية
lang-kn = الكانادا
lang-ko = الكورية
lang-kr = الكانوري
lang-ks = الكشميرية
lang-ku = الكردية
lang-kv = الكومي
lang-kw = الكورنية
lang-ky = القيرغيزية
lang-la = اللاتينية
lang-lb = اللكسمبورغية
lang-lg = الغاندا
lang-li = الليمبورغية
lang-ln = اللينجالا
lang-lo = اللاوية
lang-lt = الليتوانية
lang-lu = اللوبا كاتانغا
lang-lv = اللاتفية
lang-mg = الملغاشي
lang-mh = المارشالية
lang-mi = الماورية
lang-mk = المقدونية
lang-ml = المالايالامية
lang-mn = المنغولية
lang-mr = الماراثية
lang-ms = الماليزية
lang-mt = المالطية
lang-my = البورمية
lang-na = النورو
lang-nb = النرويجية بوكمال
lang-nd = النديبيل الشمالية
lang-ne = النيبالية
lang-ng = الندونجا
lang-nl = الهولندية
lang-nn = النرويجية نينورسك
lang-no = النرويجية
lang-nr = النديبيل الجنوبي
lang-nv = النافاجو
lang-ny = النيانجا
lang-oc = الأوكسيتانية
lang-oj = الأوجيبوا
lang-om = الأورومية
lang-or = الأورية
lang-os = الأوسيتيك
lang-pa = البنجابية
lang-pi = البالية
lang-pl = البولندية
lang-ps = البشتو
lang-pt = البرتغالية
lang-qu = الكويتشوا
lang-rm = الرومانشية
lang-rn = الرندي
lang-ro = الرومانية
lang-ru = الروسية
lang-rw = الكينيارواندا
lang-sa = السنسكريتية
lang-sc = السردينية
lang-sd = السندية
lang-se = سامي الشمالية
lang-sg = السانجو
lang-sh = صربية-كرواتية
lang-si = السنهالية
lang-sk = السلوفاكية
lang-sl = السلوفانية
lang-sm = الساموائية
lang-sn = الشونا
lang-so = الصومالية
lang-sq = الألبانية
lang-sr = الصربية
lang-ss = السواتي
lang-st = السوتو الجنوبية
lang-su = السوندانية
lang-sv = السويدية
lang-sw = السواحلية
lang-ta = التاميلية
lang-te = التيلوغوية
lang-tg = الطاجيكية
lang-th = التايلاندية
lang-ti = التغرينية
lang-tk = التركمانية
lang-tl = التاغالوغية
lang-tn = التسوانية
lang-to = التونغية
lang-tr = التركية
lang-ts = السونجا
lang-tt = التترية
lang-tw = التوي
lang-ty = التاهيتية
lang-ug = الأويغورية
lang-uk = الأوكرانية
lang-ur = الأوردية
lang-uz = الأوزبكية
lang-ve = الفيندا
lang-vi = الفيتنامية
lang-vo = لغة الفولابوك
lang-wa = الولونية
lang-wo = الولوفية
lang-xh = الخوسا
lang-yi = اليديشية
lang-yo = اليوروبا
lang-za = الزهيونج
lang-zh = الصينية
lang-zu = الزولو
person-tab-profile = الملف الشخصي
profile-groups-label = أقسام الملف الشخصي
profile-group-withheld = جزء من هذا القسم محجوب عنك
profile-withheld = مسجّل لهذا الشخص ومحجوب عنك: { $classes }.
profile-empty = لم يُسجَّل شيء في هذا القسم بعد.
profile-earlier = نموذج سابق
profile-earlier-title = سجّلته نسخة سابقة من هذا التطبيق في حقل ليس له مكان في AXGF 1.1، وقد حُفظ كما كُتب.
profile-other-names = { $n ->
        [zero] دون أسماء أخرى
        [one] واسم آخر
        [two] واسمان آخران
        [few] و{ $n } أسماء أخرى
        [many] و{ $n } اسمًا آخر
       *[other] و{ $n } اسم آخر
    }
profile-edit-group = تعديل «{ $group }»
profile-summary-link = { $n ->
        [zero] لا معلومات في الملف الشخصي
        [one] معلومة واحدة في الملف الشخصي
        [two] معلومتان في الملف الشخصي
        [few] { $n } معلومات في الملف الشخصي
        [many] { $n } معلومةً في الملف الشخصي
       *[other] { $n } معلومة في الملف الشخصي
    }
profile-from = من
profile-until = حتى
profile-yes = نعم
profile-no = لا
profile-value = القيمة
profile-editor-title = الملف الشخصي
profile-problems = تعذّر حفظ جزء مما أُدخل. كل مشكلة مذكورة بجوار حقلها، ولم يُكتب شيء.
profile-editor-withheld = يضم هذا القسم أيضًا لهذا الشخص بيانات من فئة { $classes } لا يحق لك قراءتها. لا تظهر هنا، وحفظ هذا النموذج يتركها كما هي.
profile-living-class-note = هذا الشخص مسجّل على قيد الحياة. ما تُدخله هنا في فئة حساسة لا يراه إلا المشرفون.
profile-relationships-elsewhere = الوالدان والأزواج والأبناء والعرّابون والشهود ليسوا مخزّنين في سجل هذا الشخص، بل هم عائلات وروابط وأحداث تذكره — ولذلك فإن كل تعديل هنا يغيّر أيضًا سجل كل من يشاركه فيها.
profile-documents-first = يشير الأثر إلى وثيقة مرفقة بهذا الشخص. أرفق الملف أولًا.
profile-editor-nothing = لا شيء في هذا القسم يمكنك تعديله.
profile-new-entry = إدخال جديد
profile-provenance = التاريخ والمصدر ودرجة الثقة
profile-from-date = صحيح ابتداءً من
profile-until-date = صحيح حتى
profile-remove-entry = إزالة هذا الإدخال
profile-add-entry = إضافة إدخال آخر
profile-no-such-group-title = لا يوجد قسم بهذا الاسم
profile-no-such-group-detail = ليس في الملف الشخصي قسم بهذا الاسم.
profile-error-number = يجب أن تكون القيمة في هذا الحقل رقمًا.
profile-error-integer = يجب أن تكون القيمة في هذا الحقل عددًا صحيحًا.
profile-error-range = رقم خارج النطاق المسموح به لهذه الصفة.
profile-error-term = قيمة ليست من الخيارات المتاحة.
profile-error-required = ينقص أحد الإدخالات حقلٌ لازم.
profile-error-one-of = يحتاج الإدخال إلى واحد على الأقل من حقوله الرئيسية.
profile-error-confidence = تتراوح درجة الثقة بين 0 و1، مثل 0.8.
profile-error-time = يُكتب الوقت بالساعات والدقائق، مثل 05:40.
profile-error-currency = تُكتب العملة برمزها المكوّن من ثلاثة أحرف، مثل EGP.
profile-error-language = تُكتب اللغة برمزها، مثل ar أو zh-Hans.
profile-error-coordinates = تحتاج الإحداثيات إلى خط عرض بين −90 و90 وخط طول بين −180 و180.
profile-error-rank-country = الرتبة تابعة لبلد غير البلد المختار.
record-unknown-place = [مكان غير معروف]
record-missing-document = [وثيقة مفقودة]

## Interface

confidence-certain = درجة اليقين { $percent }٪ — شبه مؤكد
confidence-high = درجة اليقين { $percent }٪ — مدعوم جيدًا
confidence-medium = درجة اليقين { $percent }٪ — مرجَّح لكنه غير مؤكد
confidence-low = درجة اليقين { $percent }٪ — تخمين
tree-edge-union-between = { $from } و{ $to } — { $confidence }
tree-edge-parentage-of = { $from }، والد أو والدة { $to } — { $confidence }
record-note-biography = السيرة
record-note-birth-date-as-recorded = تاريخ الميلاد كما سُجِّل
record-note-death-date-as-recorded = تاريخ الوفاة كما سُجِّل
record-note-event-date-as-recorded = تاريخ «{ $event }» كما سُجِّل
record-unknown-source = [مصدر مجهول]
record-untitled-source = [مصدر بلا عنوان]
record-unnamed = [بلا اسم]
record-untitled = [بلا عنوان]
record-period-from = منذ { $date }
record-period-until = حتى { $date }
record-dates-unrecorded = التواريخ غير مسجلة
record-link-unlabelled = مرتبط بـ
record-link-reverse = { $label } (لـ)
record-place-worked-as = عمل بصفة { $title }
record-place-married-to = الزواج من { $name }
record-place-married = الزواج
record-source-use-name = الاسم «{ $name }»
record-source-use-working-as = العمل بصفة { $title }
record-source-use-union-with = الارتباط بـ{ $name }
record-source-use-union = الارتباط
record-lifespan-born = وُلد { $year }
record-lifespan-died = تُوفّي { $year }
size-bytes = { $n ->
        [zero] { $n } بايت
        [one] بايت واحد
        [two] بايتان
        [few] { $n } بايتات
        [many] { $n } بايتًا
       *[other] { $n } بايت
    }
size-kb = { $n } ك.ب
size-mb = { $n } م.ب
size-gb = { $n } ج.ب
calendar-gregorian = ميلادي
calendar-julian = يولياني
calendar-hebrew = عبري
calendar-hijri = هجري
calendar-persian = فارسي
calendar-chinese = صيني
calendar-ethiopian = إثيوبي
calendar-japanese_era = العصور اليابانية
calendar-republican_french = الجمهوري الفرنسي
calendar-roman = روماني
diff-summary-none = لم يغيّر أي حقل
diff-summary-one = غيّر { $a }
diff-summary-two = غيّر { $a } و{ $b }
diff-summary-many = غيّر { $a } و{ $b } { $n ->
        [zero] ولا حقل آخر
        [one] وحقلًا آخر
        [two] وحقلين آخرين
        [few] و{ $n } حقول أخرى
        [many] و{ $n } حقلًا آخر
       *[other] و{ $n } حقل آخر
    }
diff-saved-none = لم يتغيّر أي حقل
diff-saved-one = تغيّر { $a }
diff-saved-two = تغيّر { $a } و{ $b }
diff-saved-many = تغيّر { $a } و{ $b } { $n ->
        [zero] ولا حقل آخر
        [one] وحقل آخر
        [two] وحقلان آخران
        [few] و{ $n } حقول أخرى
        [many] و{ $n } حقلًا آخر
       *[other] و{ $n } حقل آخر
    }
history-created = أنشأ
history-deleted = حذف
history-attached = أرفق ملفًا
admin-raw-json-unparsed = تعذّرت قراءة JSON الخام ({ $error }). لم يُحفظ شيء.
conflict-someone = شخص ما
conflict-unrecorded-time = وقت لم يسجّله أحد
dedup-merged-persons = { $n ->
        [zero] لم يُدمج أي شخص
        [one] دُمج شخص واحد
        [two] دُمج شخصان
        [few] دُمج { $n } أشخاص
        [many] دُمج { $n } شخصًا
       *[other] دُمج { $n } شخص
    }
dedup-merged-families = { $n ->
        [zero] لم تُدمج أي عائلة
        [one] دُمجت عائلة واحدة
        [two] دُمجت عائلتان
        [few] دُمجت { $n } عائلات
        [many] دُمجت { $n } عائلةً
       *[other] دُمجت { $n } عائلة
    }
dedup-manual-review = { $n ->
        [zero] لا حالات متروكة لمراجعة بشرية
        [one] حالة واحدة متروكة لمراجعة بشرية
        [two] حالتان متروكتان لمراجعة بشرية
        [few] { $n } حالات متروكة لمراجعة بشرية
        [many] { $n } حالةً متروكة لمراجعة بشرية
       *[other] { $n } حالة متروكة لمراجعة بشرية
    }
dedup-nothing = لا شيء يستحق الذكر.
record-union-duplicate = زوجان واحدان، أكثر من سجل.
record-union-duplicate-detail = تحتفظ الحزمة بسجلَّي عائلة منفصلين لهذين الشخصين. هذا خلل في البيانات، وليس ارتباطًا ثانيًا.
record-union-duplicate-action = دمج المكرَّر
record-union-duplicate-confirm = إزالة التكرار من الحزمة كلها؟ سيُدمج كل زوج تستطيع المكتبة دمجه، ويُبلَّغ عن الباقي.
dedup-pair-merged = صار الزوج المقصود سجلًّا واحدًا.
dedup-pair-refused = لم يُدمج الزوج المقصود: رفضت المكتبة دمجه وتركته لمراجعة إنسان.

validate-errors = { $n ->
        [zero] لا أخطاء
        [one] خطأ واحد
        [two] خطآن
        [few] { $n } أخطاء
        [many] { $n } خطأً
       *[other] { $n } خطأ
    }
validate-warnings = { $n ->
        [zero] لا تحذيرات
        [one] تحذير واحد
        [two] تحذيران
        [few] { $n } تحذيرات
        [many] { $n } تحذيرًا
       *[other] { $n } تحذير
    }
validate-notes = { $n ->
        [zero] لا ملاحظات
        [one] ملاحظة واحدة
        [two] ملاحظتان
        [few] { $n } ملاحظات
        [many] { $n } ملاحظةً
       *[other] { $n } ملاحظة
    }
validate-nothing = لا شيء يستحق الذكر.
list-separator = { "، " }
result-written = كُتب الأرشيف على القرص.
result-refused = رفضت المكتبة هذه العملية. الأرشيف على القرص لم يتغيّر.
convert-error-no-file = لم يُرفع أي ملف. اختر ملف ‎.ged أولًا.
convert-error-file-too-large = حجم هذا الملف { $size } م.ب والحد { $limit } م.ب. لم يُحوَّل شيء.
convert-error-too-large = الملف المرفوع أكبر من حد { $limit } م.ب. لم يُحوَّل شيء.
convert-error-unreadable = تعذّرت قراءة الملف المرفوع ({ $error }). لم يُحوَّل شيء.
convert-error-not-gedcom = لا يبدو هذا ملف GEDCOM: ملف GEDCOM 5.5.1 يبدأ بسطر «0 HEAD». لم يُحوَّل شيء.
convert-error-packaging = حُوِّل الملف لكن تعذّر تحزيمه ({ $error }).
completeness-fraction = { $part } من { $whole }
event-category-adoption = تبنٍّ
event-category-migration = هجرة
event-category-naturalization = تجنُّس
event-category-incarceration = سجن
event-category-name_change = تغيير الاسم
event-category-legal = مسألة قانونية
event-category-religious = واقعة دينية
event-category-social = واقعة اجتماعية
event-category-historical = واقعة تاريخية
precision-quarter_century = إلى ربع القرن
source-type-birth_certificate = شهادة ميلاد
source-type-death_certificate = شهادة وفاة
source-type-marriage_certificate = عقد زواج
source-type-census = تعداد سكاني
source-type-baptism_record = سجل معمودية
source-type-burial_record = سجل دفن
source-type-will = وصية
source-type-land_record = سجل عقاري
source-type-military_record = سجل عسكري
source-type-immigration_record = سجل هجرة
source-type-naturalization = ملف تجنُّس
source-type-passport = جواز سفر
source-type-photograph = صورة فوتوغرافية
source-type-letter = رسالة
source-type-diary = يوميات
source-type-newspaper = صحيفة
source-type-oral_tradition = رواية شفهية
source-type-dna = فحص الحمض النووي
source-type-family_bible = كتاب مقدس عائلي
source-type-gravestone = شاهد قبر
source-type-published_genealogy = شجرة نسب منشورة
source-type-other = مصدر آخر
source-status-verified = رُوجع على الأصل
source-status-unverified = لم يُراجَع بعد
source-status-lost = مفقود
source-status-known_missing = معروف أنه ناقص
document-type-birth_certificate = شهادة ميلاد
document-type-death_certificate = شهادة وفاة
document-type-marriage_certificate = عقد زواج
document-type-census_page = صفحة تعداد سكاني
document-type-baptism_record = سجل معمودية
document-type-military_record = سجل عسكري
document-type-will = وصية
document-type-land_record = سجل عقاري
document-type-diary = يوميات
document-type-newspaper_clipping = قصاصة صحفية
document-type-gravestone_photo = صورة شاهد قبر
document-type-family_tree_drawing = شجرة عائلة مرسومة
document-type-audio = تسجيل صوتي
document-type-video = تسجيل مرئي
document-status-present = محفوظ هنا
document-status-referenced = مذكور، محفوظ في مكان آخر
document-status-known_missing = معروف أنه ناقص
document-status-lost = مفقود
document-status-unknown = مكانه مجهول
diag-unsupported_spec_version = يعلن الأرشيف إصدارًا من AXGF لا يستطيع هذا البرنامج قراءته.
diag-invalid_json = شيء ينبغي أن يكون JSON لا تمكن قراءته.
diag-invalid_bundle_structure = الأرشيف غير مرتّب على النحو الذي يشترطه AXGF.
diag-schema_validation_failed = سجل لا يطابق مخطط AXGF.
diag-dangling_reference = سجل يشير إلى سجل آخر غير موجود في الأرشيف.
diag-duplicate_entity_id = سجلان يشتركان في المعرّف نفسه.
diag-duplicate_unique_ref = سجلان يدّعيان المرجع نفسه الذي ينبغي أن يكون فريدًا.
diag-cycle_detected = روابط العائلة تدور في حلقة: سيكون أحدهم جدًّا لنفسه.
diag-chronology_conflict = التواريخ متناقضة، كطفل وُلد قبل أحد والديه.
diag-out_of_vocabulary = قيمة ليست من المصطلحات التي تسمح بها قائمتها.
diag-claim_inconsistent = ادعاء يناقض نفسه أو يناقض ادعاءً آخر عن الأمر نفسه.
diag-spec_version_mismatch = إصدار AXGF الذي يعلنه سجل لا يوافق ما يحتويه.
diag-unknown_attribute = سجل يحمل سمة لا يعرّفها AXGF.
diag-entity_not_found = السجل المطلوب تغييره غير موجود في الأرشيف.
diag-entity_already_exists = يوجد سجل بهذا المعرّف بالفعل.
diag-unknown_entity_kind = هذا ليس نوع سجل يعرفه AXGF.
diag-delete_blocked_by_reference = لا يمكن حذف السجل ما دامت سجلات أخرى تشير إليه.
diag-manual_review_required = يحتاج هذا إلى نظر إنسان؛ لم يُغيَّر تلقائيًا.
diag-zip_read_error = تعذّرت قراءة ملف الأرشيف.
diag-zip_write_error = تعذّرت كتابة ملف الأرشيف.
diag-payloads_external = الملفات المرفقة محفوظة خارج بيانات الأرشيف.
diag-payload_source_failed = تعذّرت قراءة ملف مرفق.
diag-payload_sink_failed = تعذّرت كتابة ملف مرفق.
diag-gedcom_parse_error = تعذّر فهم سطر من ملف GEDCOM.
diag-gedcom_unrecognized_tag = يستخدم ملف GEDCOM وسمًا لا يعرفه الاستيراد، فلم يُنقل ذلك المُدخل.
diag-internal = حدث خطأ داخل المكتبة.
field-person-display-name = الاسم المعروض
field-person-display-name-hint = الاسم الظاهر في كل مكان من الموقع.
field-person-gender = الجنس
field-person-living = على قيد الحياة
field-person-birth-date = تاريخ الميلاد
field-date-value-hint = سنة، أو سنة وشهر، أو تاريخ كامل: 1923 أو 1923-04 أو 1923-04-12. اتركه فارغًا إن لم يعرفه أحد.
field-person-birth-precision = دقة تاريخ الميلاد
field-precision-hint = مدى الدقة التي يحدد بها المصدر ذلك.
field-person-birth-circa = الميلاد تقريبي
field-circa-hint = يُعرض بصيغة «نحو 1923» لا بوصفه تاريخًا دقيقًا.
field-person-birth-place = معرّف مكان الميلاد
field-person-birth-confidence = درجة اليقين في الميلاد
field-person-confidence-hint = مدى تأكدك. وهذا ما يرسمه الموقع.
field-person-death-date = تاريخ الوفاة
field-person-death-precision = دقة تاريخ الوفاة
field-person-death-circa = الوفاة تقريبية
field-person-death-place = معرّف مكان الوفاة
field-person-death-confidence = درجة اليقين في الوفاة
field-person-death-cause = سبب الوفاة
field-person-bio = السيرة
field-notes = ملاحظات
field-family-name = اسم العائلة
field-description = الوصف
field-family-union-type = نوع الارتباط
field-family-union-status = حالة الارتباط
field-family-union-confidence = درجة اليقين في الارتباط
field-family-union-confidence-hint = تحدد مدى وضوح الخط المرسوم بين الزوجين في الشجرة.
field-family-union-start = بداية الارتباط
field-family-union-end = نهاية الارتباط
field-family-notes-hint = الزوجان والأبناء قوائم: حرّرها في JSON الخام أدناه، أو في صفحة علاقات الشخص.
field-category = الفئة
field-required-hint = إلزامي.
field-event-subcategory = الفئة الفرعية
field-date = التاريخ
field-event-date-hint = يشترطه المخطط.
field-precision = الدقة
field-circa = تقريبي
field-place-id = معرّف المكان
field-confidence = درجة اليقين
field-source-id = معرّف المصدر
field-link-from-type = نوع البداية
field-link-from-id = معرّف البداية
field-link-to-type = نوع النهاية
field-link-to-id = معرّف النهاية
field-link-label = اسم الصلة
field-link-label-hint = يُقرأ في اتجاه الصلة: «العرّاب»، «صاحب العمل»، «الشاهد». إلزامي.
field-link-label-reverse = الاسم في الاتجاه المعاكس
field-link-label-reverse-hint = كيف يُقرأ من الطرف الآخر: «الابن بالمعمودية»، «الموظف».
field-link-bidirectional = يُقرأ نفسه في الاتجاهين
field-valid-from = سارٍ منذ
field-link-valid-from-hint = متى بدأت العلاقة.
field-valid-until = سارٍ حتى
field-link-confidence-hint = «متأكد بنسبة 85٪ بحسب رسالة عائلية» — ما لا يستطيع GEDCOM قوله.
field-note = ملاحظة
field-occupation-person-id = معرّف الشخص
field-occupation-title = العمل
field-occupation-title-hint = إلزامي، مثل: معلّم.
field-occupation-title-latin = العمل (بالحروف اللاتينية)
field-occupation-employer = صاحب العمل
field-occupation-from = من
field-occupation-from-hint = العمل مدة زمنية. ذكر طرفيها هو ما يتيح رسمها شريطًا.
field-occupation-until = إلى
field-source-title = العنوان
field-source-type = نوع المصدر
field-source-reliability = الموثوقية
field-source-reliability-hint = إلزامي. تُعرض شارةً بجانب كل واقعة تستند إلى هذا المصدر.
field-source-status = حالة المصدر
field-source-repository = جهة الحفظ
field-source-repository-reference = رقم الحفظ لدى الجهة
field-source-transcription = النسخ
field-place-name = الاسم الرئيسي
field-place-name-lang = لغة الاسم
field-place-name-lang-hint = رمز لغة، مثل en أو fr أو pl.
field-place-type = نوع المكان
field-place-region = المنطقة
field-place-country-current = الدولة اليوم
field-place-country-current-hint = تاريخ حدوده قائمة: حرّره في JSON الخام أدناه.
field-document-filename = اسم الملف
field-document-mime-type = نوع الوسائط
field-document-mime-type-hint = إلزامي، مثل image/jpeg.
field-document-type = نوع الوثيقة
field-document-status = حالة الملف
field-document-url = عنوان الويب
field-document-caption = التعليق
lang-zh-Hans = الصينية المبسطة
family-lineage = النَّسَب
links-relation = نوع الصلة
occupations-position = المنصب
field-link-relation = نوع الصلة
field-link-relation-hint = إحدى الصلات التي يسمّيها AXGF 1.1. ويحفظ الاسم أعلاه كلمات السجل نفسها.
field-occupation-position = المنصب
field-occupation-position-hint = المنصب داخل العمل: مديرة، حين يكون العمل معلّمة.
error-delete-changed-title = تغيّر منذ أن اطّلعت عليه
error-delete-changed-detail = حُفظ هذا السجل مرة أخرى بعد عرض الصفحة التي تحذفه منها، وهو الآن الإصدار { $version }. لم يُحذف شيء. انظر إليه على حاله الآن قبل أن تقرر من جديد.
error-delete-changed-look = انظر إليه مجددًا
documents-files = الملفات المرفقة هنا
documents-files-help = حرّر تفاصيل ملف، أو احذف الملف نفسه. الحذف يزيل الوثيقة وبايتاتها من الأرشيف لكل من أُرفقت به؛ ولفصلها عن هذا الشخص وحده، أفرغ سطرها أعلاه.
documents-edit-details = تحرير التفاصيل
documents-delete = حذف هذا الملف

## Charts

radar-section = مخططات من السجل
radar-section-help = ثلاث قراءات لما يحتويه هذا السجل، كل محور من 0 إلى 100. كل رقم يُحسب من الوقائع المذكورة بجانبه، وفق قواعد مكتوبة في توثيق التطبيق؛ لا يُحفظ شيء ولا يُخمَّن شيء، والمحور الذي لا يوجد ما يُقرأ منه يُترك فارغًا بدل أن يُعطى درجة وسطى.
radar-physique = البنية الجسدية
radar-mind = المزاج والعقل
radar-vitality = الصحة والحيوية
radar-axis-stature = القامة
radar-axis-build = البنية
radar-axis-lean-mass = الكتلة الخالية من الدهون
radar-axis-posture = الوقفة
radar-axis-gait = المشية
radar-axis-dentition = الأسنان
radar-axis-openness = الانفتاح
radar-axis-conscientiousness = يقظة الضمير
radar-axis-extraversion = الانبساط
radar-axis-agreeableness = الوداعة
radar-axis-stability = الاستقرار العاطفي
radar-axis-cognition = الإدراك
radar-axis-circulation = الدورة الدموية
radar-axis-breathing = التنفس
radar-axis-metabolism = الأيض
radar-axis-illness = الخلو من المرض
radar-axis-senses = الحواس
radar-axis-rest = النوم والمزاج
radar-folded-open = اعرض هذا المخطط
radar-folded-why = هذا الشخص مسجَّل على قيد الحياة. تبقى صورة مزاج الشخص الحي مطوية حتى يطلب رؤيتها من يُسمح له بقراءتها.
radar-empty = لا يوجد في هذا السجل بعدُ ما يمكن قراءته في هذا المخطط.
radar-table-caption = { $chart }: كل محور ودرجته وما قُرئت منه
radar-col-axis = المحور
radar-col-score = الدرجة
radar-col-from = قُرئ من
radar-no-score = لا درجة
radar-from-none = لا شيء
record-link-outgoing = من هذا الشخص
record-link-incoming = إلى هذا الشخص
# تحذير التشغيل في لوحة الإدارة.
health-attention = يحتاج إلى انتباه:
health-standing-token = ما زال رمز إدارة الطوارئ مُعدًّا. إنه يدخل متجاوزًا كل حساب وكل صلاحية — وهذا غرضه في اليوم الذي لا يستطيع فيه أحد الدخول، لا أن يبقى بعد أن صار الدخول ممكنًا. أزِله من ملف بيئة الخدمة وأعد تشغيلها.
health-bundle-invalid = { $errors ->
        [zero] لم تعد بيانات العائلة تجتاز التحقق.
        [one] لم تعد بيانات العائلة تجتاز التحقق: خطأ واحد، مذكور أدناه تحت «تحقق».
        [two] لم تعد بيانات العائلة تجتاز التحقق: خطأان، مذكوران أدناه تحت «تحقق».
        [few] لم تعد بيانات العائلة تجتاز التحقق: { $errors } أخطاء، مذكورة أدناه تحت «تحقق».
        [many] لم تعد بيانات العائلة تجتاز التحقق: { $errors } خطأً، مذكورة أدناه تحت «تحقق».
       *[other] لم تعد بيانات العائلة تجتاز التحقق: { $errors } خطأ، مذكورة أدناه تحت «تحقق».
    }
health-disk-unknown = لم تتمكن الخدمة من قراءة المساحة الحرة على هذا الجهاز، فلا شيء هنا يضمن أن الحفظ القادم سيتسع.
health-disk-no-room-to-save = الحفظ غير ممكن: المتاح { $free }، وإعادة بناء هذه الحزمة تحتاج { $need }. لم يُفقد شيء، ولا يمكن تغيير شيء حتى تتوفر مساحة.
health-disk-critical = القرص حر بنسبة { $percent }٪ — بقي { $free }. سيتوقف الحفظ عن العمل قريبًا.
health-disk-low = القرص حر بنسبة { $percent }٪ — بقي { $free }. يُستحسن معالجته قبل أن يصبح عاجلًا.
health-backup-unconfigured = لا يُنسخ أي شيء احتياطيًا. قرص واحد معطوب كان سيأخذ كل السجلات معه.
health-backup-never = مجلد النسخ الاحتياطي مُعدّ، لكن لم تُكتب فيه أي نسخة قط.
health-backup-stale = { $days ->
        [zero] أحدث نسخة احتياطية قديمة. ينبغي كتابة واحدة كل يوم.
        [one] عمر أحدث نسخة احتياطية يوم واحد. ينبغي كتابة واحدة كل يوم.
        [two] عمر أحدث نسخة احتياطية يومان. ينبغي كتابة واحدة كل يوم.
        [few] عمر أحدث نسخة احتياطية { $days } أيام. ينبغي كتابة واحدة كل يوم.
        [many] عمر أحدث نسخة احتياطية { $days } يومًا. ينبغي كتابة واحدة كل يوم.
       *[other] عمر أحدث نسخة احتياطية { $days } يوم. ينبغي كتابة واحدة كل يوم.
    }
health-cache-missing = { $missing ->
        [zero] لا يوجد ملف مفقود من الذاكرة المؤقتة.
        [one] ملف واحد من { $declared } من الملفات المرفقة مفقود من الذاكرة المؤقتة ولا يمكن تنزيله حتى يعيد الحفظ القادم بناءه.
        [two] ملفان من { $declared } من الملفات المرفقة مفقودان من الذاكرة المؤقتة ولا يمكن تنزيلهما حتى يعيد الحفظ القادم بناءهما.
        [few] { $missing } ملفات من { $declared } من الملفات المرفقة مفقودة من الذاكرة المؤقتة ولا يمكن تنزيلها حتى يعيد الحفظ القادم بناءها.
        [many] { $missing } ملفًا من { $declared } من الملفات المرفقة مفقودة من الذاكرة المؤقتة ولا يمكن تنزيلها حتى يعيد الحفظ القادم بناءها.
       *[other] { $missing } ملف من { $declared } من الملفات المرفقة مفقود من الذاكرة المؤقتة ولا يمكن تنزيله حتى يعيد الحفظ القادم بناءه.
    }
