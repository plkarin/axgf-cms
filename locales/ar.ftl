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
account-emergency = رمز الطوارئ

settings-title = الإعدادات
edit-mode-label = وضع التحرير — ما تحفظه يغيّر سجلات العائلة
display-menu-label = اللغة والعرض
display-settings = السمة والعرض…
settings-tab-theme = السمة
settings-tab-language = اللغة
settings-tab-appearance = المظهر
settings-done = تم
prefs-language = اللغة
prefs-theme = السمة
prefs-background-on = تدرّج لوني خفيف خلف الصفحة
prefs-apply = تطبيق
busy-saving = جارٍ الحفظ…
busy-working = جارٍ التنفيذ…
prefs-machine = آلية، { $coverage }٪

theme-canopee = ظُلّة الغابة
theme-canopee-note = داكن، أخضر الغابة · الافتراضي
theme-fusion = فيوجن
theme-fusion-note = داكن، نيون · شجرة حيّة
theme-galaxy = مجرّة
theme-galaxy-note = الفضاء العميق · نظام شمسي
theme-summit = القمّة
theme-summit-note = داكن، فجر ذهبي · صعود

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

record-identity = الهوية
record-life-events = أحداث الحياة
record-family = الأسرة
record-other-relationships = علاقات أخرى
record-occupations = المهن
record-places = الأماكن
record-sources-documents = المصادر والوثائق
record-notes = ملاحظات
record-history = سجل التعديلات
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
conf-percent = { $percent }٪
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
error-merged-title = هذا السجل لم يعد موجودًا.
error-merged-detail = دُمج مع { $name } في { $date }. الروابط إلى العنوان القديم تقود الآن إلى السجل المدمج.
error-merged-open = فتح السجل المدمج
error-back-to-river = العودة إلى النهر
view-tree-back = العودة إلى الشجرة
view-galaxy-back = العودة إلى المجرّة
view-summit-back = العودة إلى الجبل
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
setup-title = تهيئة هذا الأرشيف
setup-lede = لا يحتوي هذا التثبيت على أي عائلة بعد. اختر كيف يبدأ.
setup-door-archive-title = من أرشيف AXGF
setup-door-archive-detail = ملف .axgf من تثبيت آخر أو من نسخة احتياطية. ينتقل أشخاصه ومصادره وصوره كما هي.
setup-door-archive-file = أرشيف (.axgf)
setup-door-archive-go = التهيئة من هذا الأرشيف
setup-limit-gb = حتى { $n } غيغابايت، تُكتب مباشرة على القرص.
setup-door-scratch-title = من لا شيء
setup-door-scratch-detail = أرشيف فارغ. أضف الشخص الأول، ثم عائلته، سجلًّا بعد سجل.
setup-door-scratch-go = إضافة الشخص الأول
setup-door-gedcom-title = من ملف GEDCOM
setup-door-gedcom-detail = الملف الذي يصدّره كل برنامج أنساب: GEDCOM 5.5 أو 5.5.1 أو 5.5.5 أو 7.0، بصيغة .ged أو .gdz، بأي ترميز كان. ترى كيف قُرئ وما الذي ينتقل قبل أن يُحفظ أي شيء.
setup-door-gedcom-file = ملف GEDCOM (.ged، .gdz)
setup-door-gedcom-go = قراءة هذا الملف
setup-never-over = حين توجد عائلة، لا تستبدلها هذه الصفحة إلا بعد نسخة احتياطية متحقَّق منها وكتابة الكلمات التي تؤكد ذلك.
setup-adopt-title = جعل هذه عائلة الأرشيف
setup-adopt-detail = السجلات التي ستصبح عائلة هذا التثبيت: { $total }. لا يُحفظ شيء قبل أن تضغط الزر.
setup-adopt-go = تهيئة الأرشيف بهذه العائلة
setup-adopt-back = اختيار مدخل آخر
setup-done-title = تمت تهيئة الأرشيف
setup-done-detail = السجلات في عائلة هذا التثبيت الآن: { $total }.
setup-done-photos = الصور والوثائق التي جاءت معها: { $n }.
setup-done-mismatches = ملفات لا يطابق محتواها البصمة المسجلة لها: { $n }. احتُفظ بها؛ تحقق منها.
setup-done-river = فتح النهر
view-tree-open = فتح الشجرة
view-galaxy-open = فتح المجرّة
view-summit-open = فتح الجبل
setup-done-admin = الإدارة
setup-refused-title = تعذّر أن يصبح هذا الأرشيف
setup-refused-detail = لم يُكتب شيء. ما الذي منعه:
setup-closed-title = هذا الأرشيف مهيأ بالفعل
setup-closed-detail = وصلت عائلة في الأثناء، ولم يُكتب شيء. افتح صفحة الاستيراد من جديد: عند وجود عائلة تطلب تأكيدًا وتنسخها احتياطيًا أولًا.
setup-not-archive = هذا ليس أرشيف .axgf.
setup-archive-too-large = هذا الأرشيف أكبر مما يُسمح برفعه.
admin-entities = السجلات
admin-create = إنشاء
admin-new-kind = جديد: { $kind }
admin-operations = العمليات
admin-validate = تحقّق
admin-deduplicate = دمج المكرّرات
admin-export = تصدير الملف
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
admin-no-changes-yet = لا تعديل بعد. يُسجَّل كل حفظ في { $path }.
admin-last-validation = آخر تحقّق
admin-fields = الحقول
admin-raw-json = JSON الخام
form-more-fields = { $n ->
        [zero] لا حقول أخرى
        [one] حقل آخر
        [two] حقلان آخران
        [few] { $n } حقول أخرى
        [many] { $n } حقلًا آخر
       *[other] { $n } حقل آخر
    }
admin-save = حفظ
admin-cancel = إلغاء
place-editor-title = تحرير مكان
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
person-tab-life = الأحداث
person-tab-media = الوسائط
person-tab-tree = الشجرة
person-tab-history = السجل
person-tab-afg = النسّابة الذكي
person-tab-status = الحالة
status-check = للتحقق
status-know = للعلم
status-fix = صحّح
status-open = افتح
status-dismissed = عُلّمت «عادية عندنا»: { $n }
status-panel = للتحقق: { $n }
person-afg-title = اسأل النسّابة الذكي عن { $name }
person-afg-title-short = محادثات مع النسّابة الذكي
person-afg-lede = اسأل عن حياة { $name } وعائلته وعصره. محادثاتك هنا لك وحدك.
person-afg-reads = يُقرأ مع كل رسالة: السجل، والصور ({ $photos })، وملفات PDF ({ $pdfs })، والملاحظات ({ $notes })، والأقارب المقرّبون ({ $relatives })، والعائلة ({ $persons }) · نحو { $tokens } رمزًا.
person-afg-ask = سؤالك عن { $name }
person-afg-example = مثلًا: «في أي فوج كان { $name } في المعركة التي تذكرها الملاحظات؟»
person-afg-conversations = محادثاتك
person-afg-none = لا محادثة بعد: اطرح سؤالًا أول.
person-afg-all = كل محادثاتك
person-afg-back = العودة إلى محادثاتك
person-afg-gone = هذه المحادثة غير موجودة، أو ليست لك.
person-afg-delete-all = احذف كل محادثاتك عن { $name }
person-afg-delete-all-confirm = أريد حذفها كلها.
person-afg-deleted-all = حُذفت محادثاتك عن هذا الشخص.

admin-delete = حذف
admin-not-set = — غير محدَّد —
admin-edit = تحرير
list-select-row = تحديد { $name }
list-kind-label = النوع
list-select-page = تحديد كل صفوف هذه الصفحة
list-selected = { $n ->
        [zero] لا شيء محدد
        [one] سجل واحد محدد
        [two] سجلان محددان
        [few] { $n } سجلات محددة
        [many] { $n } سجلًا محددًا
       *[other] { $n } سجل محدد
    }
list-export = تصدير
list-escape = Esc لإلغاء التحديد
bulk-confirm-title = { $n ->
        [zero] حذف لا شيء؟
        [one] حذف سجل واحد؟
        [two] حذف سجلين؟
        [few] حذف { $n } سجلات؟
        [many] حذف { $n } سجلًا؟
       *[other] حذف { $n } سجل؟
    }
bulk-confirm-detail = يُرفض السجل الذي لا يزال سجل آخر يشير إليه، وعندها لا يُحذف شيء.
bulk-confirm-go = احذفها
bulk-deleted = { $n ->
        [zero] لم يُحذف شيء.
        [one] حُذف سجل واحد.
        [two] حُذف سجلان.
        [few] حُذفت { $n } سجلات.
        [many] حُذف { $n } سجلًا.
       *[other] حُذف { $n } سجل.
    }
bulk-refused = لم يُحذف شيء: لا يزال { $name } مُشارًا إليه في مكان آخر. أزل تلك الإشارة أولًا ثم أعد المحاولة.
bulk-changed = { $n ->
        [zero] لم يتغيّر شيء.
        [one] تغيّر سجل واحد منذ عرض القائمة، لذلك لم يُحذف شيء. أعد تحميل القائمة وحاول مجددًا.
        [two] تغيّر سجلان منذ عرض القائمة، لذلك لم يُحذف شيء. أعد تحميل القائمة وحاول مجددًا.
        [few] تغيّرت { $n } سجلات منذ عرض القائمة، لذلك لم يُحذف شيء. أعد تحميل القائمة وحاول مجددًا.
        [many] تغيّر { $n } سجلًا منذ عرض القائمة، لذلك لم يُحذف شيء. أعد تحميل القائمة وحاول مجددًا.
       *[other] تغيّر { $n } سجل منذ عرض القائمة، لذلك لم يُحذف شيء. أعد تحميل القائمة وحاول مجددًا.
    }
admin-range = { $from }–{ $to } من { $total }
list-filiation = النسب
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
accounts-password-hint = فارغ: تُولَّد كلمة مرور وتُعرض مرة واحدة. وإلا فعلى الأقل { $min } محارف.
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
tree-level-self = الجيل نفسه
tree-level-up = { $n ->
        [1] الأبناء
        [2] الأحفاد
        [3] أبناء الأحفاد
       *[other] الذرية، الجيل { $n }
    }
tree-level-down = { $n ->
        [1] الوالدان
        [2] الأجداد
        [3] آباء الأجداد
       *[other] الأسلاف، الجيل { $n }
    }
tree-card-centre = توسيط الشجرة هنا
tree-card-same = الشخص نفسه، مرسوم مرة ثانية
# The tree page's legend: what a line to a child says (2026-10-28).
tree-key-lines = الخطوط:
tree-key-son = ابن
tree-key-daughter = ابنة
tree-key-adoptive = تبنٍّ
tree-key-foster = رعاية أو وصاية
tree-key-step = ربيب
tree-key-union = ارتباط
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

record-open-full-page = افتح الصفحة الكاملة ↗
record-open-full-title = افتح الصفحة المستقلة القابلة للمشاركة
record-edit = تحرير
record-see-in-river = عرض في النهر
view-tree-see = عرض في الشجرة
view-galaxy-see = عرض في المجرّة
view-summit-see = عرض على الجبل
record-living-yes = على قيد الحياة
record-deceased = { $gender ->
        [F] متوفّاة
       *[other] متوفّى
    }
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
record-clips = الأفلام والتسجيلات الصوتية
record-documents = الوثائق
record-file = الملف
record-status = الحالة
record-size = الحجم
panel-empty = اختر بطاقة لترى هنا السجل الكامل لذلك الشخص.
panel-selected-person = الشخص المختار
person-visibility-inline = الظهور:
person-age-at-death = { $gender ->
        [F] { $n ->
            [zero] تُوفّيت عن { $n } عام
            [one] تُوفّيت عن عام واحد
            [two] تُوفّيت عن عامين
            [few] تُوفّيت عن { $n } أعوام
            [many] تُوفّيت عن { $n } عامًا
           *[other] تُوفّيت عن { $n } عام
        }
       *[other] { $n ->
            [zero] تُوفّي عن { $n } عام
            [one] تُوفّي عن عام واحد
            [two] تُوفّي عن عامين
            [few] تُوفّي عن { $n } أعوام
            [many] تُوفّي عن { $n } عامًا
           *[other] تُوفّي عن { $n } عام
        }
    }
person-age-now = { $n ->
        [zero] { $n } عام
        [one] عام واحد
        [two] عامان
        [few] { $n } أعوام
        [many] { $n } عامًا
       *[other] { $n } عام
    }
person-born-in = { $gender ->
        [F] وُلدت في { $place }
       *[other] وُلد في { $place }
    }
person-died-in = { $gender ->
        [F] تُوفّيت في { $place }
       *[other] تُوفّي في { $place }
    }
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

note-birth-imprecise = تاريخ الميلاد غير دقيق في المصدر
note-death-imprecise = تاريخ الوفاة غير دقيق في المصدر
note-names = أسماء مسجلة: { $n }
note-transliteration = اسم بخطه الأصلي، مع نقحرته
note-witnessed = أحداث بصفة شاهد: { $n }

visibility-public = علني
visibility-members = الأعضاء
visibility-contributors = المساهمون
visibility-private = خاص

## الواجهة العامة

nav-convert = استيراد
footer-credit = © 2026 · تصميم وتطوير: { $author }
footer-format-link = صيغة AXGF على GitHub
prefs-machine-complete = كاملة، لم تُراجَع بعد

## شروح أقسام السجل

record-identity-help = كل اسم مسجَّل مع نوعه والمدة التي استُعمل فيها والمصدر الذي يسنده، مع الخط الأصلي إلى جانب النقل الحرفي اللاتيني حيث يختلفان، إضافة إلى النوع وحال الحياة ومدى الظهور.
record-life-events-help = الميلاد والوفاة وكل واقعة شارك فيها هذا الشخص، مرتّبة بالتاريخ، كلٌّ مع دوره فيها — فزواجٌ شهده فحسب يقف إلى جانب زواجه هو. والواقعة بلا تاريخ تأتي في الآخر بدل أن تتظاهر بالأسبقية.
record-family-help = الوالدان والإخوة، ثم كل اقتران بنوعه وتواريخه ومكانه وكيف انتهى وأبنائه بترتيب الميلاد.
record-other-relationships-help = كل صلة يقف هذا الشخص في أحد طرفيها، مقروءة من جهته: السجل نفسه يظهر «عرّابًا لـ» من طرف و«ابنًا بالعمادة لـ» من الطرف الآخر.
record-places-help = كل مكان يمسّه هذا السجل، مع ما جرى فيه ومع تاريخ الحدود الذي يجعل للمكان معنى عبر الزمن.
record-history-help = كل تغيير محفوظ على هذا السجل، الأحدث أولًا. ومن صحّح ماذا حقيقةٌ عن القائمين على الشجرة لا عن العائلة التي فيها، ولذلك يبقى خارج الأرشيف المُصدَّر ولا يُعرض إلا لمن سجّل دخوله من الأقارب.

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
upload-file-too-large = هذا الملف أكبر من حد { $mb } ميغابايت ولن يُرسل. اختر ملفًا أصغر.
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
admin-roles-note = التحقق ودمج المكرر والتصدير والحذف والحسابات للمديرين وحدهم.
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
accounts-lede = محفوظ في { $path } (الوضع 600)، بجانب الأرشيف لا داخله.
accounts-no-registration = لا يمكن للزوار فتح حسابات بأنفسهم؛ يُضبط التسجيل أعلاه.
accounts-branch-hint = يقصر ما يستطيع هذا الحساب تحريره على أولئك الأشخاص وذريتهم وأزواجهم.
accounts-branch-reading = ولا يقصر ما يستطيع قراءته — فذاك يحكمه مدى ظهور كل سجل، والأمران مفصولان عن قصد.
accounts-emergency-warning = دخلت برمز الطوارئ لا بحساب: أنشئ حسابًا حقيقيًا أدناه واستعمله.
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
convert-page-title = استيراد ملف عائلي
convert-lede = استورد شجرة من ملف GEDCOM. لا يُحفظ شيء، وتبقى شجرة الموقع كما هي.
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
tree-contradicts-detail = استُبعدت هذه الروابط من ترقيم الأجيال، وقد تكون بعض الصفوف في غير موضعها. صحّح السجل الخاطئ.
tree-contradicts-pair = زوجان ووالد وولد في آن:
tree-contradicts-more = { $n ->
        [zero] لا تناقضات أخرى غير مذكورة.
        [one] تناقض آخر غير مذكور.
        [two] تناقضان آخران غير مذكورين.
        [few] { $n } تناقضات أخرى غير مذكورة.
        [many] { $n } تناقضًا آخر غير مذكورة.
       *[other] { $n } تناقض آخر غير مذكور.
    }
tree-no-people-cta = استورد ملفًا عائليًا، أو أضف أول شخص.
home-empty = لا شيء مسجل بعد.
home-count = { $total ->
        [zero] لا سجلات
        [one] سجل واحد
        [two] سجلان
        [few] { $total } سجلات
        [many] { $total } سجلًا
       *[other] { $total } سجل
    }، في ملف واحد تملكه العائلة.
home-unnamed-family = شجرة العائلة هذه
home-in-this-tree = ما سجّلته العائلة حتى الآن
home-showcase-title = حيث تقول هذه الشجرة أكثر من الأسماء والتواريخ
home-showcase-example = انظر مثالًا ←
home-nothing-title = لا شيء لعرضه بعد.
home-nothing-admin = لا يحتوي هذا التثبيت على أي عائلة بعد: هيّئه من أرشيف، أو من ملف GEDCOM، أو من لا شيء.
home-nothing-visitor = يجري إعداد أرشيف العائلة هذا.
home-eyebrow = أرشيف العائلة
home-hero-1 = كل حياة نبع.
home-hero-2 = وكل عائلة نهر.
view-tree-hero-1 = كل حياة ورقة.
view-tree-hero-2 = وكل عائلة شجرة.
view-galaxy-hero-1 = كل حياة نجم.
view-galaxy-hero-2 = وكل عائلة مجرّة.
view-summit-hero-1 = كل حياة خطوة.
view-summit-hero-2 = كل عائلة صعود.
home-cta-river = استكشف النهر
view-tree-cta = استكشف الشجرة
view-galaxy-cta = استكشف المجرّة
view-summit-cta = اصعد الجبل
home-cta-tree = اعرض الشجرة
home-scroll = مرّر للاستكشاف
home-trust-title = درجة الثقة في صلات النسب
home-trust-certain = شبه مؤكدة
home-trust-high = مدعومة جيدًا
home-trust-medium = معقولة
home-trust-low = افتراضية
home-trust-unrated = لم تُقيَّم بعد
home-showcase-eyebrow = أبعد من الأسماء والتواريخ
home-cta-title = معًا نحفظ ذاكرة العائلة.
home-cta-detail = كل اسم يُضاف وكل مصدر يُتحقَّق منه يجعل النهر أصدق لمن سيأتون بعدنا.
view-tree-cta-detail = كل اسم يُضاف وكل مصدر يُتحقَّق منه يجعل الشجرة أصدق لمن سيأتون بعدنا.
view-galaxy-cta-detail = كل اسم يُضاف وكل مصدر يُتحقَّق منه يجعل المجرّة أصدق لمن سيأتون بعدنا.
view-summit-cta-detail = كل اسم يُضاف وكل مصدر يُتحقق منه يجعل الجبل أصدق لمن يأتي بعدنا.
home-cta-add = أضف شخصًا

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

record-absent-document = ذكره هذا الشخص لكنه غير محفوظ هنا.
record-upload-help-short = حتى { $mb } ميغابايت. وSVG مرفوض.
record-verbatim-note = محفوظ كما جاء في السجل تمامًا، لأن أي محوِّل لم يستطع تأويله.
record-file-to-attach = الملف المراد إرفاقه
record-document-type = نوع المستند
record-caption = التعليق
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
completeness-dates-note = تُحفظ التواريخ كما أُعطيت، باليوم أو بالعقد؛ والنص الذي ليس تاريخًا يُحفظ حرفيًا.
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

## ملاحظات على السجل

note-links = علاقات خارج العائلة: { $n }
note-occupations = مهن مؤرخة: { $n }

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
completeness-metric-confidence-many = { $with } من { $slots } حقيقة تحمل درجة: { $assessed } منها تخالف { $modal }، على { $distinct } مستويات.

completeness-metric-parentage = ما مقدار الثقة بكل صلة والدية
completeness-metric-parentage-none = ليس في صلات النسب هنا واحدة تقول ما مقدار الثقة بها. والتبنّي والخطوط المتنازع عليها وما أُعيد بناؤه من ذكرٍ واحد هي بعينها المواضع التي تحتاج فيها عائلة إلى تسجيل الشك — والشجرة ترسم الصلة الأقل يقينًا خطًا أبهت.
completeness-metric-parentage-some = أنساب بدرجتها الخاصة: { $n }

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
completeness-metric-occupations-some = { $span } من { $total } لها بداية أو نهاية.

completeness-metric-sources = مصادر مقدَّرة الموثوقية
completeness-metric-sources-none = لا مصادر مسجَّلة. وتسميةُ ما جاءت منه الواقعة هي ما يتيح لقريب أن يتحقق منها لاحقًا — أو أن يخالفها ويقول لماذا.
completeness-metric-sources-some = { $graded } من { $total } تذكر مدى متانتها.

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
admin-export-health-note = يستبعد التصدير البسيط كل فئة حساسة وسلوك الأحياء. حدّد ما يجب أن يحمله هذا الملف.
avatar-picker-title = اختر صورة
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
ai-badge = AI
ai-portraits-title = صور شخصية عدّلها الذكاء الاصطناعي
ai-portraits-band = تعديلات
ai-portraits-upload = رفع صور معدّلة بالذكاء الاصطناعي
ai-portraits-manage = إدارة صور الذكاء الاصطناعي
ai-portraits-intro = صور صنعها الذكاء الاصطناعي من الصور الفوتوغرافية لهذا الشخص: كيف ربما كان يبدو. تظهر بجانب الوجه في السجل، موسومة بـ«AI»، ولا يراها إلا الأعضاء المسجّل دخولهم. لا تكون أبدًا صورة الشخص، ولا تُدرج بين ملفاته، ولا تُعدّ دليلًا.
ai-portraits-current = في السجل
ai-portraits-add = إضافة صور — يتسع لـ { $n }
ai-portraits-files = صور (PNG أو JPEG أو WebP أو GIF)، يمكن اختيار عدة صور معًا
ai-portraits-generator = صُنعت بواسطة
ai-portraits-from = انطلاقًا من
ai-portraits-from-none = غير محدد
ai-portraits-remove = إزالة
ai-portraits-alt = صورة { $name } معدّلة بالذكاء الاصطناعي
ai-portraits-not-evidence = صنعها الذكاء الاصطناعي من الصور الفوتوغرافية: ليست صورًا من ذلك الزمن.
ai-portraits-full = في السجل ثلاث صور معدّلة بالذكاء الاصطناعي. أزل واحدة لإضافة أخرى.
ai-portraits-too-many = لا يتسع السجل لأكثر من ثلاث صور معدّلة بالذكاء الاصطناعي. أزل واحدة قبل إضافة أخرى.
ai-portraits-images-only = يجب أن تكون الصورة المعدّلة بالذكاء الاصطناعي صورةً: PNG أو JPEG أو WebP أو GIF.
ai-portraits-stale = تغيّر السجل أثناء فتح هذه الصفحة. لم يُحفظ شيء: افتحها من جديد وأعد المحاولة.
ai-portraits-back = العودة إلى صور الذكاء الاصطناعي

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
events-edit-link = حرّر الأحداث

## Presentation styles: density, never colour

prefs-style = الكثافة
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
profile-group-filled = { $n ->
        [zero] لا شيء
        [one] حقل واحد معبّأ
        [two] حقلان معبّآن
        [few] { $n } حقول معبّأة
        [many] { $n } حقلًا معبّأً
       *[other] { $n } حقل معبّأ
    }
profile-problems = تعذّر حفظ جزء مما أُدخل. كل مشكلة مذكورة بجوار حقلها، ولم يُكتب شيء.
profile-editor-withheld = يضم هذا القسم أيضًا لهذا الشخص بيانات من فئة { $classes } لا يحق لك قراءتها. لا تظهر هنا، وحفظ هذا النموذج يتركها كما هي.
profile-living-class-note = هذا الشخص مسجّل على قيد الحياة. ما تُدخله هنا في فئة حساسة لا يراه إلا المشرفون.
profile-relationships-elsewhere = الأقارب والعرّابون والشهود مسجلون في العائلات والروابط والأحداث: تغييرهم هنا يغيّر تلك السجلات أيضًا.
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
diff-section-other = حقلًا آخر
diff-section-avatar = صورة شخصية
diff-section-partners = الشريكان
diff-section-ai = مساعدة الذكاء الاصطناعي
diff-section-birth = الولادة
diff-section-civil-status = الحالة المدنية
diff-section-documents = المستندات المرفقة
diff-section-extensions = بيانات التطبيق
diff-section-children = الأبناء
diff-section-union = الارتباط
diff-section-date = التاريخ
diff-section-participants = المشاركون
diff-section-from = مرتبط من
diff-section-to = مرتبط بـ
diff-section-valid-from = ساري من
diff-section-valid-until = ساري حتى
diff-section-visibility = الظهور
diff-section-employer = جهة العمل
diff-section-title-normalized = المسمى الموحّد
diff-section-conflicts = مصادر متعارضة
diff-section-dna = الحمض النووي
diff-section-document = المستند
diff-section-language = اللغة
diff-section-place = المكان
diff-section-repository = مكان الحفظ
diff-section-script = الخط
diff-section-coordinates = الإحداثيات
diff-section-country-history = تاريخ الدول
diff-section-identifiers = المعرّفات
diff-section-names = الأسماء
diff-section-file = الملف
diff-section-linked-to = مرفق بـ
diff-section-ocr = النص المتعرَّف عليه
diff-section-format-version = إصدار الصيغة
history-created = أنشأ
history-deleted = حذف
history-merged = دمج مع سجل آخر
history-setup = هيّأ الأرشيف
history-attached = أرفق ملفًا
admin-raw-json-unparsed = تعذّرت قراءة JSON الخام ({ $error }). لم يُحفظ شيء.
admin-raw-json-dangling = يشير JSON الخام في { $field } إلى { $target }، وهو غير موجود في هذا الأرشيف. اختره من حقل أعلاه أو صحّح المعرّف. لم يُحفظ شيء.
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
gedcom-read-title = كيف قُرئ الملف
gedcom-read-as = قُرئ بوصفه GEDCOM { $version }، بترميز { $encoding }.
gedcom-read-as-unversioned = قُرئ بوصفه GEDCOM (لا يذكر رأسه إصدارًا)، بترميز { $encoding }.
gedcom-read-declared = يذكر رأسه { $declared }؛ لكن بايتاته تقول غير ذلك، فقُرئت كما هي.
gedcom-read-gedzip = أرشيف GEDCOM 7 (.gdz): قُرئت مجموعة بياناته. ملفات أخرى فيه (صور، مسح ضوئي) لم تُنقل: { $n }.
gedcom-read-crs = أسطر تنتهي على طريقة ماكنتوش القديمة، أُصلحت: { $n }.
gedcom-read-voids = إشارات إلى @VOID@ — «لا سجل» في GEDCOM 7 — نُحّيت بدل قراءتها أشخاصًا: { $n }.
gedcom-read-notes = ملاحظات مشتركة (SNOTE) قُرئت ملاحظات: { $n }.
gedcom-read-stray = بايتات لم تكن UTF-8 في هذا الملف المكتوب بـ UTF-8، قُرئ كلٌّ منها بترميز Windows-1252: { $n }.
gedcom-read-tidied = أسطر مكتوبة بطريقة لا يقرؤها المحوِّل (مُزاحة، أو بفواصل جدولة أو مسافات زائدة بين الحقول، أو بوسم بأحرف صغيرة، أو بمسافة قبل وصلة CONC، أو بحرف تحكّم، أو بعلامة التقويم الغريغوري)، أُصلحت: { $n }.
gedcom-read-too-deep = أسطر متداخلة أعمق مما تبلغه أي بنية في GEDCOM، وُضعت جانبًا مع ما تحمله: { $n }.
convert-error-no-dataset = لا يحتوي ملف zip هذا على gedcom.ged، ولا على ملف .ged واحد يُقرأ بدلًا منه.
convert-error-refused = رفض المحوّل هذا الملف. التشخيصات أدناه تبيّن السبب. لم يُحفظ شيء.
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
diag-merge_refused = لا يمكن دمج السجلين على حالهما.
diag-merge_unresolved = لم يُحسم خلاف بين السجلين.
diag-merge_unsupported = لا يمكن دمج سجلات من هذا النوع.
diag-stale_plan = تغيّر أحد السجلين منذ إعداد الخطة: أعد التخطيط.
field-person-display-name = الاسم المعروض
field-person-display-name-hint = الاسم الظاهر في كل مكان من الموقع.
field-person-gender = الجنس
field-person-living = على قيد الحياة
field-person-birth-date = تاريخ الميلاد
field-date-value-hint = 1923 أو 1923-04 أو 1923-04-12؛ فارغ إن كان مجهولًا.
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
health-standing-token = ما زال رمز الطوارئ مُعدًّا ويتجاوز كل حساب. احذفه من ملف بيئة الخدمة ثم أعد التشغيل.
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

## River

nav-river = النهر
nav-home = العائلة
river-aria = النهر حول { $name }: الأسلاف في الأسفل، والذرية في الأعلى.
river-band-living = الأحياء
river-circa = نحو{" "}
river-trail = المسار
river-back = → رجوع
river-range = المدى
river-motion = الحركة
river-travel = انتقال
river-cut = قطع
river-legend-attested = موثّق
river-legend-inferred = مستنتج
river-legend-speculative = مفترض
river-legend-lost = ينتهي السجل
river-legend-repeat = ×n تصل إليه n من السلالات، ويُرسم تحت كل منها
river-legend-again = ↔ يُرسم مرة أخرى بجانب شريك لاحق
river-legend-discharge = الذرية
river-legend-direction = ↓ نحو المنبع · ↑ نحو الأحياء
view-tree-direction = ↓ الجذور: الأسلاف · ↑ التاج: الأحفاد
view-galaxy-direction = ↓ الأسلاف تحت الشمس · ↑ الأحفاد فوقها
view-summit-direction = ↓ الأسلاف في الوادي · ↑ الأحفاد نحو القمّة
river-legend-generation = جيل واحد ≈ 29 سنة
river-zoom = تكبير
river-still = إيقاف حركة المنظر
view-tree-still = إيقاف حركة الشجرة
view-galaxy-still = إيقاف حركة السماء
view-summit-still = إيقاف حركة الجبل
river-whole = العائلة كلها
river-names-all = كل الأسماء
# A tap on a person in a view, on a touch screen: the drawing moves onto them.
river-pick-centre = اجعله في المركز
river-zoom-hint = التكبير: Ctrl مع العجلة، أو القرص بإصبعين، أو + / − · بعد التكبير، اسحب للتنقل
river-tab-sources = المصادر
river-legend-title = كيف يُقرأ النهر
view-tree-name = الشجرة الحيّة
view-tree-legend = كيف تُقرأ الشجرة
view-galaxy-name = المجرّة
view-galaxy-legend = كيف تُقرأ المجرّة
view-summit-name = الصعود
view-summit-legend = كيف تقرأ الجبل
hud-ancestors = الأسلاف
hud-descendants = الأحفاد
hud-generations = الأجيال
hud-living = الأحياء
hud-attested = الروابط الموثّقة

# Masthead search
search-label = ابحث عن شخص
search-short = شخص
date-qualifier = المُحدِّد
date-qualifier-circa = نحو
search-title = البحث
search-heading = الأشخاص المطابقون لـ «{ $q }»
search-none = لا أحد ممن يمكنك رؤيتهم يحمل هذا الاسم.
search-open-record = افتح السجل
search-hint = يأخذ Enter النهر إليه، ويفتح Shift+Enter سجله.
view-tree-search-hint = يُنمّي Enter الشجرة حوله، ويفتح Shift+Enter سجله.
view-galaxy-search-hint = يجعله Enter الشمس، ويفتح Shift+Enter سجله.
view-summit-search-hint = يأخذك Enter إليه، ويفتح Shift+Enter سجله.
search-summary = { $n ->
        [zero] لا أحد
        [one] شخص واحد
        [two] شخصان
        [few] { $n } أشخاص
        [many] { $n } شخصًا
       *[other] { $n } شخص
    } · ↵ انتقال · ⇧↵ فتح السجل
search-empty-query = اكتب اسمًا للبحث.

# Data quality (/quality)
nav-quality = جودة البيانات
quality-title = أمور تستحق النظر
quality-none = لا شيء يستحق النظر: اجتازت كل الفحوص.
quality-intro = نقاط تستحق التحقق؛ وكثيرًا ما يكون لها سبب وجيه. افتح السجل أو علّم النقطة كمُتحقَّق منها.
quality-groups-label = أنواع الملاحظات
quality-all = الكل
quality-show-label = عرض
quality-show-open-option = للتحقق
quality-download = تنزيل
quality-group-names = الأسماء
quality-group-relations = صلات القرابة
quality-group-dates = التواريخ
quality-group-places = الأماكن
quality-group-evidence = الأدلة
quality-export-csv = تنزيل بصيغة CSV
quality-export-json = تنزيل بصيغة JSON
quality-open = افتح
quality-none-file = لا شيء يستحق النظر في هذا الملف.
quality-show-list = اعرض القائمة
quality-after-import = لم يُغيَّر شيء في الملف. حين يصبح أرشيف الموقع، تعرض صفحة جودة البيانات الأمور نفسها مع رابط إلى كل سجل.
quality-file-list-cut = لا يُعرض هنا إلا الأوائل حتى { $shown }. وحين يصبح الملف أرشيف الموقع، تعرضها صفحة جودة البيانات كلها.
quality-rule-name-sentence = جملة في حقل الاسم
quality-name-sentence = «{ $value }» يبدو جملة لا اسمًا.
quality-rule-union-parent-child = زوجان هما أيضًا والد وولد
quality-union-parent-child = { $parent } و{ $child }: زوجان ووالد وولد في آن.
quality-rule-child-listed-twice = طفل مذكور مرتين
quality-child-listed-twice = { $child } مذكور مرتين بين الأبناء.
quality-rule-duplicate-union = الزوجان نفسهما مسجلان مرتين
quality-duplicate-union = { $a } و{ $b }: عدد الزيجات المسجلة: { $count }.
quality-rule-partner-twice = شخص واحد في طرفي الزواج
quality-partner-twice = { $person } على طرفي الزواج.
quality-rule-foreign-surnames = ابن لا يطابق لقبه أيًّا من الوالدين
quality-foreign-surnames = لقب لا يطابق لقب أيّ من الوالدين: { $who }.
quality-rule-death-before-birth = وفاة قبل الولادة
quality-death-before-birth = { $person }: الوفاة ({ $death }) قبل الميلاد ({ $birth }).
quality-rule-child-before-parent = ولد مولود قبل أحد والديه
quality-child-before-parent = ميلاد { $child } ({ $childborn }) قبل ميلاد أحد والديه { $parent } ({ $parentborn }).
quality-rule-parent-too-young = والد دون الثانية عشرة
quality-parent-too-young = كان عمر { $parent } { $age } عند ميلاد { $child }.
quality-rule-parent-too-old = والد تجاوز الخامسة والستين
quality-parent-too-old = كان عمر { $parent } { $age } عند ميلاد { $child }.
quality-rule-marriage-before-birth = زواج قبل ولادة أحد الزوجين
quality-marriage-before-birth = زواج ({ $married }) قبل ميلاد { $person } ({ $born }).
quality-rule-isolated-record = سجل بلا تاريخ ولا صلات
quality-isolated-record = { $person }: لا تاريخ ولا صلة.
quality-rule-place-no-region = مكان بلا منطقة
quality-place-no-region = { $place }: لا منطقة.
quality-rule-region-free-text = منطقة تبدو كملاحظة
quality-region-free-text = منطقة { $place }: «{ $value }» تبدو ملاحظة.
quality-rule-place-near-duplicate = أماكن قد تكون واحدًا
quality-place-near-duplicate = ربما مكان واحد: { $names }.
quality-rule-not-a-place = إدخال قد لا يكون مكانًا
quality-not-a-place = «{ $place }» قد لا يكون مكانًا.
quality-rule-certain-without-source = مؤكد بلا مصدر
quality-certain-without-source = { $record }: مؤكَّد ({ $claims }) بلا مصدر.
quality-dismiss = تم التحقق — طبيعي لدينا
quality-restore = أظهره مجددًا
quality-note-label = السبب (اختياري)
quality-show-dismissed = ما تم التحقق منه وإبقاؤه
quality-show-open = العودة إلى القائمة المفتوحة
quality-dismissed-by = تحقق منه { $by } في { $at }
quality-gone-title = تلك الملاحظة لم تعد موجودة
quality-gone-detail = تغيّر السجل منذ فتح القائمة، فلم تعد الملاحظة كما كانت. أعد تحميل القائمة وانظر مجددًا.
quality-summary = { $n ->
    [zero] لا شيء يستحق النظر
    [one] أمر واحد يستحق النظر
    [two] أمران يستحقان النظر
    [few] { $n } أمور تستحق النظر
    [many] { $n } أمرًا يستحق النظر
   *[other] { $n } أمر يستحق النظر
}
quality-summary-file = { $n ->
    [zero] لا شيء يستحق النظر في هذا الملف
    [one] أمر واحد يستحق النظر في هذا الملف
    [two] أمران يستحقان النظر في هذا الملف
    [few] { $n } أمور تستحق النظر في هذا الملف
    [many] { $n } أمرًا يستحق النظر في هذا الملف
   *[other] { $n } أمر يستحق النظر في هذا الملف
}
quality-dismissed-summary = { $n ->
    [zero] لا ملاحظات تم التحقق منها
    [one] ملاحظة واحدة تم التحقق منها وإبقاؤها
    [two] ملاحظتان تم التحقق منهما وإبقاؤهما
    [few] { $n } ملاحظات تم التحقق منها وإبقاؤها
    [many] { $n } ملاحظةً تم التحقق منها وإبقاؤها
   *[other] { $n } ملاحظة تم التحقق منها وإبقاؤها
}
quality-dismiss-all = علِّم الكل بأنه تم التحقق منه ({ $n })
quality-restore-all = أظهر الكل مجددًا ({ $n })
quality-bulk-conflict = سجلات تغيّرت منذ فتح القائمة: { $n }. لم يُعلَّم شيء؛ أعد تحميل القائمة وحاول مجددًا.

## The record's editor (v0.4.2)

edit-tab-changed = معدَّل، لم يُحفظ بعد
edit-remove = إزالة
edit-add-partner = إضافة زوج
edit-add-child = إضافة ابن
edit-union-with = اقتران مع { $names }
edit-union-alone = اقتران بلا زوج مسجَّل
edit-child-of = ابن { $names }
edit-new-family = إنشاء عائلة
edit-cite-title = ما يستند إلى مصدر
edit-attach-to = استشهد به لـ
edit-add-source = إضافة مصدر
edit-error-source-title = يحتاج المصدر الجديد إلى عنوان. لم يُحفظ شيء.
edit-detach-file = فصل عن هذه البطاقة
edit-no-sources = لا تستشهد أي واقعة خاصة بهذا الشخص بمصدر بعد.
edit-no-files = لم يُرفق أي ملف بهذه البطاقة بعد.
edit-saved = { $n ->
        [zero] لم يُكتب شيء.
        [one] حُفظ: بطاقة واحدة.
        [two] حُفظ: بطاقتان في حفظ واحد.
        [few] حُفظ: { $n } بطاقات في حفظ واحد.
        [many] حُفظ: { $n } بطاقة في حفظ واحد.
       *[other] حُفظ: { $n } بطاقة في حفظ واحد.
    }
edit-saved-none = لم يتغير شيء، فلم يُحفظ شيء.
edit-back-to-record = العودة إلى السجل
edit-avatar = صورة الملف الشخصي
edit-avatar-frame = ضبط موضع الوجه
edit-avatar-upload = استخدامها صورةً للملف الشخصي
edit-stale = عدَّل شخص آخر { $what } منذ فتح هذه الصفحة. لم يُحفظ شيء؛ كل ما أدخلته ما زال هنا.
edit-refused = لم يُحفظ شيء: { $reason }

# Administration console (v0.5.0, docs/adr/0011)
error-cross-site-title = مرفوض: أُرسل من موقع آخر
error-cross-site-detail = جاء هذا النموذج من صفحة في موقع آخر، لذا لم يُنفَّذ. إن كنت أرسلته من هذا الموقع فأعد تحميل الصفحة وأرسله مجددًا.
admin-nav-label = أقسام الإدارة
admin-nav-overview = نظرة عامة
admin-nav-bundle = أرشيف AXGF
admin-nav-infrastructure = البنية التحتية
admin-nav-accounts = الحسابات
admin-nav-connectivity = الاتصال

# Administration console: the AXGF bundle (v0.5.0, docs/adr/0011)
bundle-title = أرشيف AXGF
bundle-lede = أرشيف العائلة: { $path }. مطابقته، وما يبدو خاطئًا، وكيف تُؤخذ نسخة منه.
bundle-unset-title = لا توجد عائلة بعد.
bundle-unset-detail = تحتوي هذه التثبيتة على أرشيف فارغ، فلا شيء للتحقق منه بعد. هيّئها من أرشيف أو من ملف GEDCOM أو من الصفر.
bundle-kpi-conformity = المطابقة
bundle-verdict-ok = مطابق
bundle-verdict-warn = يحتاج مراقبة
bundle-verdict-fail = غير مطابق
findings-tally = سليمة: { $ok } · تحذيرات: { $warn } · حرجة: { $fail }
bundle-kpi-entities = الكيانات
bundle-kpi-entities-note = { $persons ->
    [zero] لا أشخاص
    [one] شخص واحد
    [two] شخصان
    [few] { $persons } أشخاص
    [many] { $persons } شخصًا
    *[other] { $persons } شخص
    } · { $files ->
    [zero] لا ملفات مرفقة
    [one] ملف مرفق واحد
    [two] ملفان مرفقان
    [few] { $files } ملفات مرفقة
    [many] { $files } ملفًا مرفقًا
    *[other] { $files } ملف مرفق
    }
bundle-kpi-size = الحجم على القرص
bundle-kpi-members = { $n ->
    [zero] لا ملفات في الأرشيف
    [one] ملف واحد في الأرشيف
    [two] ملفان في الأرشيف
    [few] { $n } ملفات في الأرشيف
    [many] { $n } ملفًا في الأرشيف
    *[other] { $n } ملف في الأرشيف
    }
bundle-kpi-format = الصيغة
bundle-kpi-format-note = هذا الإصدار يقرأ ويكتب AXGF { $current }.
bundle-conformity = المطابقة
bundle-conformity-intro = تسعة فحوص على الأرشيف كما هو الآن، ولا يغيّره أيٌّ منها. افتح سطرًا لترى ما يجب فعله.
bundle-area-format = الصيغة
bundle-area-integrity = السلامة
bundle-area-privacy = الخصوصية
bundle-area-content = المحتوى
level-ok = سليم
level-warn = تحذير
level-fail = حرج
findings-what-to-do = ما الذي يجب فعله
findings-go = اذهب إلى هناك ←
bundle-anomalies = الحالات الشاذة
bundle-anomalies-intro = ما تجده صفحة جودة البيانات، قاعدة بقاعدة: ما زال مفتوحًا، وما عُلِّم بأنه رُوجع.
bundle-anomalies-rule = القاعدة
bundle-anomalies-open = مفتوحة
bundle-anomalies-checked = رُوجعت
bundle-anomalies-none = لا تجد صفحة جودة البيانات ما يستدعي النظر.
bundle-diagnostics = تشخيصات المدقّق
bundle-facts = التفاصيل
bundle-fact-path = الملف
bundle-fact-size = الحجم
bundle-fact-spec = الصيغة
bundle-fact-schema = المخطط المرفق
bundle-fact-none = لا يوجد
bundle-fact-created = أُنشئ
bundle-fact-updated = آخر كتابة
bundle-fact-generator = كتبه
bundle-fact-gedcom = حُوِّل من
bundle-export = التصدير
bundle-export-raw = الأرشيف كما هو (.axgf)
bundle-export-raw-go = تنزيل الأرشيف
bundle-export-sealed = مُعمّى لشخص واحد (.axgf.gpg)
bundle-export-sealed-intro = الأرشيف نفسه في صورة رسالة OpenPGP لا يفتحها إلا صاحب المفتاح الخاص. الصق مفتاحه العام: النص الذي يبدأ بـ «-----BEGIN PGP PUBLIC KEY BLOCK-----».
bundle-export-sealed-go = تنزيل مُعمّى
seal-recipient = إلى
seal-fingerprint = البصمة
seal-algorithm = الخوارزميات
seal-fips-yes = ضمن مجموعة FIPS المعتمدة
seal-fips-no = خارج مجموعة FIPS المعتمدة
seal-expires = انتهاء صلاحية المفتاح
seal-never = أبدًا
seal-key-label = المفتاح العام للمستلم
seal-key-hint = مفتاح RSA لا يقل عن 2048 بت، أو مفتاح بمنحنى إهليلجي. تحقّق من البصمة مع صاحبها قبل إرسال أي شيء.
seal-remember = احتفظ بهذا المفتاح للمرة القادمة
seal-check = اقرأ المفتاح فقط
seal-fips-note = يُعمّى الأرشيف بـ AES-256 إلى مفتاح المستلم. الخوارزميات المعتمدة في FIPS هي RSA بطول 3072 بت فأكثر ومنحنيات NIST ‏P-256 وP-384 وP-521: اطلب P-384. المكتبة المستخدمة هنا ليست بحد ذاتها وحدة معتمدة وفق FIPS 140-3.
seal-decrypt-note = يفتحه المستلم بواسطة GnuPG:
seal-forget = انسَ المفتاح المحفوظ
seal-key-kept = حُفظ المفتاح { $fingerprint } للتصدير المُعمّى التالي.
seal-error-no-key = الصق أولًا المفتاح العام للمستلم.
seal-error-not-a-key = هذا ليس مفتاحًا عامًا بصيغة OpenPGP. الصق الكتلة كاملة، من سطر BEGIN إلى سطر END، ولا تلصق مفتاحًا خاصًا أبدًا.
seal-error-bad-signatures = لا تتحقق تواقيع هذا المفتاح الذاتية: ربما عُدِّل. اطلب من صاحبه نسخة جديدة.
seal-error-expired = انتهت صلاحية هذا المفتاح، أو كل مفتاح فيه قادر على التعمية. اطلب من صاحبه تمديده أو إرسال مفتاح جديد.
seal-error-revoked = ألغى صاحب هذا المفتاح صلاحيته. اطلب منه مفتاحه الجديد.
seal-error-no-encryption-key = يستطيع هذا المفتاح التوقيع لا التعمية: لا يحوي مفتاحًا فرعيًا للتعمية. اطلب من صاحبه إضافة واحد (gpg --quick-add-key … encr).
seal-error-too-weak = طول مفتاح RSA هذا { $bits } بت؛ والتصدير يحتاج { $min } على الأقل.
seal-error-unsupported = خوارزمية هذا المفتاح ليست مما يستخدمه التصدير: مفاتيح RSA أو المنحنى الإهليلجي فقط.
seal-error-io = تعذّر إنشاء النسخة المُعمّاة: { $error }
date-time-utc = { $date }، { $time } UTC

# Administration console: the bundle's conformity (v0.5.0, docs/adr/0011)
bundle-check-format-title = إصدار الصيغة
bundle-check-format-ok = AXGF { $version }، الإصدار الذي يقرؤه هذا البناء ويكتبه.
bundle-check-format-missing = لا يذكر البيان أي إصدار من AXGF.
bundle-check-format-missing-fix = لم تكتب الأرشيفَ أداةٌ من أدوات AXGF، أو أنه تضرّر. استعد آخر نسخة احتياطية سليمة (axgf-cms restore).
bundle-check-format-older = مكتوب بصيغة AXGF { $version }؛ وسيكتبه الحفظ التالي بصيغة AXGF { $current }.
bundle-check-format-older-fix = لا شيء يجب فعله: سيعيد التعديل التالي كتابته. احتفظ بنسخة من الملف القديم إن كان برنامج آخر لا يقرأ إلا { $version }.
bundle-check-format-newer = مكتوب بصيغة AXGF { $version }، وهي أحدث من { $current } في هذا البناء: قد يُفقد الحفظُ ما لا يفهمه.
bundle-check-format-newer-fix = حدّث axgf-cms إلى إصدار يقرأ AXGF { $version } قبل أن يعدّل أحد (البنية التحتية، الإصدار).
bundle-check-manifest-title = أعداد البيان
bundle-check-manifest-ok = تطابق أعداد البيان ما يحويه الأرشيف.
bundle-check-manifest-drift = يُعلن البيان { $declared } بينما يحوي الأرشيف { $actual } ({ $collection }).
bundle-check-manifest-fix = لا شيء يضيع: تُعاد كتابة الأعداد في الحفظ التالي. وإن جاء الأرشيف من مكان آخر فتحقّق من أنه نُسخ كاملًا.
bundle-check-validation-title = التحقق
bundle-check-validation-ok = لا يجد مدقّق المكتبة أي خطأ.
bundle-check-validation-errors = { $errors ->
    [zero] لا أخطاء
    [one] خطأ واحد
    [two] خطآن
    [few] { $errors } أخطاء
    [many] { $errors } خطأً
    *[other] { $errors } خطأ
    }: الأرشيف لا يجتاز التحقق.
bundle-check-validation-errors-fix = يذكر كل تشخيص أدناه السجل والحقل. صحّح السجل، أو استعد آخر نسخة احتياطية سليمة.
bundle-check-validation-warnings = { $warnings ->
    [zero] لا تحذيرات
    [one] تحذير واحد
    [two] تحذيران
    [few] { $warnings } تحذيرات
    [many] { $warnings } تحذيرًا
    *[other] { $warnings } تحذير
    }. لا تمنع الحفظ أبدًا؛ كلٌّ منها يشير إلى أن شيئًا قد يكون غير سليم.
bundle-check-validation-warnings-fix = اقرأها أدناه. معظمها إحالات إلى سجلات ليست في الأرشيف، وكثيرًا ما يتركها الاستيراد من GEDCOM.
bundle-check-payloads-title = الملفات المرفقة
bundle-check-payloads-none = لا ملفات مرفقة.
bundle-check-payloads-ok = { $declared ->
    [zero] لا ملفات
    [one] الملف الوحيد موجود.
    [two] الملفان كلاهما موجودان.
    [few] الملفات الـ { $declared } كلها موجودة.
    [many] الملفات الـ { $declared } كلها موجودة.
    *[other] الملفات الـ { $declared } كلها موجودة.
    }
bundle-check-payloads-missing = ينقص من الذاكرة المؤقتة { $missing } من أصل { $declared } ملفًا مرفقًا.
bundle-check-payloads-fix = يعيد الحفظ التالي بناء الذاكرة المؤقتة من الأرشيف. وإن كانت ناقصة من الأرشيف نفسه فاستعدها من نسخة احتياطية.
bundle-check-binding-title = الحسابات والأرشيف
bundle-check-binding-ok = أُنشئ ملف الحسابات لهذه العائلة.
bundle-check-binding-mismatch = أُنشئ ملف الحسابات لـ { $expected }؛ وهذا الأرشيف هو { $found }.
bundle-check-binding-fix = إن كان الأرشيف قد استُعيد أو أُعيدت تسميته عمدًا فلا شيء يجب فعله. وإلا فحسابات عائلة تُطبَّق على شجرة عائلة أخرى: ضع ملف ‎.acl الصحيح بجانبه.
bundle-check-living-title = الأحياء، علنًا
bundle-check-living-ok = لا أحد ممن سُجّلوا أحياءً ظاهر للزوار.
bundle-check-living-public = { $n ->
    [zero] لا أحد من المسجّلين أحياءً موسوم بأنه علني.
    [one] شخص واحد مسجّل حيًّا موسوم بأنه علني: أي زائر يقرأ سجله.
    [two] شخصان مسجّلان حيَّين موسومان بأنهما علنيان: أي زائر يقرأ سجلَّيهما.
    [few] { $n } من المسجّلين أحياءً موسومون بأنهم علنيون: أي زائر يقرأ سجلاتهم.
    [many] { $n } من المسجّلين أحياءً موسومون بأنهم علنيون: أي زائر يقرأ سجلاتهم.
    *[other] { $n } من المسجّلين أحياءً موسومون بأنهم علنيون: أي زائر يقرأ سجلاتهم.
    }
bundle-check-living-fix = ما لم يوافقوا على ذلك، اجعل ظهورهم للأعضاء (تعديل، الهوية).
bundle-check-special-title = الفئات الخاصة
bundle-check-special-none = لا يحوي أي سجل بيانات صحية أو بيومترية أو جينومية أو جنائية أو سلوكية.
bundle-check-special-some = { $n ->
    [zero] لا سجلات
    [one] سجل واحد يحوي
    [two] سجلان يحويان
    [few] { $n } سجلات تحوي
    [many] { $n } سجلًا يحوي
    *[other] { $n } سجل يحوي
    } بيانات صحية أو بيومترية أو جينومية أو جنائية أو سلوكية. وبيانات الأحياء منها تبقى للمديرين أيًا كان ما يقوله السجل.
bundle-check-quality-title = جودة البيانات
bundle-check-quality-ok = لا شيء يستدعي النظر.
bundle-check-quality-open = { $n ->
    [zero] لا شيء يستدعي النظر
    [one] أمر واحد يستدعي النظر
    [two] أمران يستدعيان النظر
    [few] { $n } أمور تستدعي النظر
    [many] { $n } أمرًا يستدعي النظر
    *[other] { $n } أمر يستدعي النظر
    }، في { $groups ->
    [zero] لا مجموعات
    [one] مجموعة واحدة
    [two] مجموعتين
    [few] { $groups } مجموعات
    [many] { $groups } مجموعةً
    *[other] { $groups } مجموعة
    }.
bundle-check-quality-fix = افتح صفحة جودة البيانات: يذكر كل سطر السجل وما يبدو خطأً. صحّحه، أو علّمه بأنه رُوجع.
bundle-check-size-title = الحجم
bundle-check-size-ok = { $size }، دون { $warn } التي تبدأ هذه الصفحة بعدها في التحذير.
bundle-check-size-heavy = { $size }، تتجاوز { $warn }.
bundle-check-size-fix = الأرشيف للعائلة وله أن ينمو. راقب القرص والنسخ الاحتياطية (البنية التحتية، الأحجام)، وارفع ‎--size-warn-mb إن كان هذا الحجم متوقعًا.
seal-algorithm-value = { $key } · AES-256
seal-key-placeholder = -----BEGIN PGP PUBLIC KEY BLOCK-----

# Administration console: the infrastructure's checks (v0.5.0, docs/adr/0011 §3)
infra-area-security = الأمان
infra-area-access = الوصول
infra-area-data = البيانات
infra-area-capacity = السعة
infra-area-release = الإصدار
infra-area-platform = المنصة
infra-verdict-ok = سليمة
infra-verdict-warn = تحتاج إلى انتباه
infra-verdict-fail = حرجة
duration-days = { $n ->
    [zero] { $n } يوم
    [one] يوم واحد
    [two] يومان
    [few] { $n } أيام
    [many] { $n } يومًا
    *[other] { $n } يوم
    }
duration-hours = { $n ->
    [zero] { $n } ساعة
    [one] ساعة واحدة
    [two] ساعتان
    [few] { $n } ساعات
    [many] { $n } ساعة
    *[other] { $n } ساعة
    }
duration-minutes = { $n ->
    [zero] { $n } دقيقة
    [one] دقيقة واحدة
    [two] دقيقتان
    [few] { $n } دقائق
    [many] { $n } دقيقة
    *[other] { $n } دقيقة
    }
infra-transport-title = كيف يُوصَل إلى الموقع
infra-transport-ok = وصلت هذه الصفحة عبر HTTPS.
infra-transport-exposed = وصلت هذه الصفحة دون تشفير، والخادم يستمع على { $bind } خارج هذا الجهاز: كلمات المرور وملفات تعريف الجلسة تعبر الشبكة نصًّا واضحًا.
infra-transport-exposed-fix = ضع أمامه وكيلًا عكسيًا يعمل بـ HTTPS (Caddy أو nginx أو HAProxy)، وأعد ربط الخدمة بـ 127.0.0.1: شغّل bootstrap.sh من جديد مع ‎--bind 127.0.0.1:8080، أو احذف ملف drop-in الذي وسّعها.
infra-transport-plain = غير مشفّر، من { $bind }: مقبول على هذا الجهاز وحده.
infra-transport-plain-fix = ادخل إلى الموقع عبر عنوانه بـ HTTPS. وإن كان وكيلٌ ينهي TLS فاجعله يرسل X-Forwarded-Proto: https ليعرف هذا الخادم ذلك.
infra-forwarded-title = العناوين المُمرَّرة
infra-forwarded-ok = لا يستمع الخادم إلا على هذا الجهاز، لذا يمكن الوثوق بعنوان العميل الذي يمرّره الوكيل لإبطاء محاولات الدخول الفاشلة المتكررة.
infra-forwarded-exposed = يمكن الوصول إلى الخادم مباشرة، وهو يثق بالترويسة X-Forwarded-For: فالعميل الذي يضعها بنفسه يفلت من الإبطاء بعد محاولات الدخول الفاشلة.
infra-forwarded-fix = اربط الخدمة بـ 127.0.0.1 خلف الوكيل، أو احجب المنفذ بجدار حماية كي لا يصل إليه إلا الوكيل.
infra-cross-site-title = الإرسال من مواقع أخرى
infra-cross-site-ok = لم يُرسَل إلى هنا أي نموذج من موقع آخر خلال اليوم الأخير.
infra-cross-site-refused = { $n ->
    [zero] لم يُرفض أي نموذج من موقع آخر خلال اليوم الأخير.
    [one] رُفض نموذج واحد أُرسل من موقع آخر خلال اليوم الأخير.
    [two] رُفض نموذجان أُرسلا من مواقع أخرى خلال اليوم الأخير.
    [few] رُفضت { $n } نماذج أُرسلت من مواقع أخرى خلال اليوم الأخير.
    [many] رُفض { $n } نموذجًا أُرسل من مواقع أخرى خلال اليوم الأخير.
    *[other] رُفض { $n } نموذج أُرسل من مواقع أخرى خلال اليوم الأخير.
    }
infra-cross-site-fix = لم يمرّ شيء. وإن كان هذا الرفض لك — الموقع نفسه عبر عنوان ثانٍ — فاضبط العنوان العام في صفحة الاتصال، أو استعمل ذلك العنوان وحده.
infra-sso-title = خدمات تسجيل الدخول
infra-sso-none = لا خدمة تسجيل دخول مفعّلة؛ تدخل الحسابات بكلمة مرور.
infra-sso-ok = { $n ->
    [zero] لا خدمة تسجيل دخول مفعّلة.
    [one] خدمة تسجيل دخول واحدة مفعّلة، والعنوان العام يعمل بـ HTTPS.
    [two] خدمتا تسجيل دخول مفعّلتان، والعنوان العام يعمل بـ HTTPS.
    [few] { $n } خدمات تسجيل دخول مفعّلة، والعنوان العام يعمل بـ HTTPS.
    [many] { $n } خدمة تسجيل دخول مفعّلة، والعنوان العام يعمل بـ HTTPS.
    *[other] { $n } خدمة تسجيل دخول مفعّلة، والعنوان العام يعمل بـ HTTPS.
    }
infra-sso-no-https = خدمة تسجيل دخول مفعّلة، لكن العنوان العام ليس HTTPS: يرفض المزوّدون إرسال ردّهم إلى عنوان غير مشفّر، فيفشل تسجيل الدخول.
infra-sso-fix = في صفحة الاتصال، اجعل العنوان العام هو عنوان https://‎ الذي يستعمله القرّاء.
infra-admins-title = المسؤولون
infra-admins-none = لا يستطيع أي حساب نشط إدارة هذا الموقع؛ وحده رمز الطوارئ يستطيع.
infra-admins-none-fix = ادخل برمز الطوارئ، وأنشئ مديرًا، ثم احذف الرمز.
infra-admins-one = مسؤول واحد فقط. إن غاب هذا الشخص أو فقد كلمة مروره فلن يستطيع أحد إدارة الموقع.
infra-admins-one-fix = امنح دور المسؤول لشخص ثانٍ موثوق في صفحة الحسابات.
infra-admins-ok = { $n ->
    [zero] لا مسؤول يستطيع الدخول.
    [one] مسؤول واحد يستطيع الدخول.
    [two] مسؤولان يستطيعان الدخول.
    [few] { $n } مسؤولين يستطيعون الدخول.
    [many] { $n } مسؤولًا يستطيعون الدخول.
    *[other] { $n } مسؤول يستطيعون الدخول.
    }
infra-token-title = رمز الطوارئ
infra-token-generated = لا رمز طوارئ مضبوط.
infra-token-only-way = رمز الطوارئ هو المدخل الوحيد إلى أن يوجد حساب مدير.
infra-token-standing = ما زال رمز الطوارئ مضبوطًا. إنه يدخل متجاوزًا كل حساب وكل صلاحية.
infra-token-fix = احذف AXGF_CMS_ADMIN_TOKEN من ملف بيئة الخدمة، ثم أعد تشغيل الخدمة.
infra-registration-title = التسجيل الذاتي
infra-registration-closed = لا يستطيع الزوار إنشاء حسابات؛ وحده المسؤول يستطيع.
infra-registration-secret = يستطيع الزوار إنشاء حساب عضو إن عرفوا سرّ التسجيل.
infra-registration-open = يستطيع أي زائر إنشاء حساب عضو.
infra-registration-open-fix = يناسب هذا موقعًا يُراد له أن يكون مفتوحًا. وإلا فاشترط سرّ التسجيل، أو أغلق التسجيل، في صفحة الحسابات.
infra-failures-title = محاولات الدخول الفاشلة
infra-failures-ok = { $n ->
    [zero] لا محاولات دخول فاشلة خلال اليوم الأخير.
    [one] محاولة دخول فاشلة واحدة خلال اليوم الأخير.
    [two] محاولتا دخول فاشلتان خلال اليوم الأخير.
    [few] { $n } محاولات دخول فاشلة خلال اليوم الأخير.
    [many] { $n } محاولة دخول فاشلة خلال اليوم الأخير.
    *[other] { $n } محاولة دخول فاشلة خلال اليوم الأخير.
    }
infra-failures-many = { $n ->
    [zero] لا محاولات دخول فاشلة.
    [one] محاولة دخول فاشلة واحدة خلال اليوم الأخير: ربما يخمّن أحدهم كلمات المرور.
    [two] محاولتا دخول فاشلتان خلال اليوم الأخير: ربما يخمّن أحدهم كلمات المرور.
    [few] { $n } محاولات دخول فاشلة خلال اليوم الأخير: ربما يخمّن أحدهم كلمات المرور.
    [many] { $n } محاولة دخول فاشلة خلال اليوم الأخير: ربما يخمّن أحدهم كلمات المرور.
    *[other] { $n } محاولة دخول فاشلة خلال اليوم الأخير: ربما يخمّن أحدهم كلمات المرور.
    }
infra-failures-fix = كل عنوان يُبطَّأ أصلًا بعد الإخفاقات المتكررة. وإن ظل العدد يرتفع فابحث في سجل الوكيل عن مصدر المحاولات واحجبه هناك، وتأكد أن لكل مسؤول كلمة مرور طويلة أو أنه يدخل عبر خدمة تسجيل دخول.
infra-health-bundle-title = بيانات العائلة
infra-health-bundle-ok = يُفتح الأرشيف ويجتاز التحقق.
infra-health-bundle-fix = افتح صفحة أرشيف AXGF لترى الأخطاء وتصححها، أو استعد آخر نسخة احتياطية سليمة (axgf-cms restore).
infra-health-cache-title = ذاكرة المرفقات المؤقتة
infra-health-cache-ok = كل ملف مرفق موجود في الذاكرة المؤقتة.
infra-health-cache-fix = احفظ أي سجل مرة واحدة: الحفظ يعيد بناء الذاكرة المؤقتة. والملفات التي تبقى مفقودة بعده مفقودة من الأرشيف نفسه.
infra-health-backup-title = النسخ الاحتياطية
infra-health-backup-ok = { $count ->
    [zero] توجد نسخة احتياطية حديثة؛ ولا أرشيف محفوظ.
    [one] توجد نسخة احتياطية حديثة؛ ويُحفظ أرشيف واحد.
    [two] توجد نسخة احتياطية حديثة؛ ويُحفظ أرشيفان.
    [few] توجد نسخة احتياطية حديثة؛ وتُحفظ { $count } أرشيفات.
    [many] توجد نسخة احتياطية حديثة؛ ويُحفظ { $count } أرشيفًا.
    *[other] توجد نسخة احتياطية حديثة؛ ويُحفظ { $count } أرشيف.
    }
infra-health-backup-fix = إن لم يكن هناك مجلد للنسخ فأعد تشغيل bootstrap.sh مع ‎--backup-dir. وإن كان فافحص المؤقّت (systemctl status axgf-cms-backup.timer) وسجلّه (journalctl -u axgf-cms-backup).
infra-disk-title = مساحة القرص
infra-disk-ok = { $free } متاحة من { $total } ({ $percent }٪ متاحة).
infra-disk-fix = حرّر مساحة على القسم الذي يضم مجلد البيانات — نسخ احتياطية قديمة محفوظة في مكان آخر أيضًا، سجلات، ملفات برامج أخرى — أو انقل البيانات إلى قرص أكبر.
infra-growth-title = النمو
infra-growth-ok = { $days ->
    [zero] ينمو نحو { $rate } شهريًا؛ ولا يبقى مكان بهذا المعدل.
    [one] ينمو نحو { $rate } شهريًا؛ ويبقى مكان ليوم واحد بهذا المعدل.
    [two] ينمو نحو { $rate } شهريًا؛ ويبقى مكان ليومين بهذا المعدل.
    [few] ينمو نحو { $rate } شهريًا؛ ويبقى مكان لـ { $days } أيام بهذا المعدل.
    [many] ينمو نحو { $rate } شهريًا؛ ويبقى مكان لـ { $days } يومًا بهذا المعدل.
    *[other] ينمو نحو { $rate } شهريًا؛ ويبقى مكان لـ { $days } يوم بهذا المعدل.
    }
infra-growth-soon = { $days ->
    [zero] ينمو نحو { $rate } شهريًا: وبهذا المعدل يمتلئ القرص الآن.
    [one] ينمو نحو { $rate } شهريًا: وبهذا المعدل يمتلئ القرص خلال يوم واحد.
    [two] ينمو نحو { $rate } شهريًا: وبهذا المعدل يمتلئ القرص خلال يومين.
    [few] ينمو نحو { $rate } شهريًا: وبهذا المعدل يمتلئ القرص خلال { $days } أيام.
    [many] ينمو نحو { $rate } شهريًا: وبهذا المعدل يمتلئ القرص خلال { $days } يومًا.
    *[other] ينمو نحو { $rate } شهريًا: وبهذا المعدل يمتلئ القرص خلال { $days } يوم.
    }
infra-growth-fix = خطّط للمساحة الآن: قرص أكبر، أو نسخ احتياطية أقل على القرص نفسه، أو نقل النسخ إلى قرص آخر.
infra-growth-flat = لم تكبر النسخ الاحتياطية؛ لا شيء يحتاج إلى تخطيط.
infra-growth-unknown = لا توجد بعدُ نسخ احتياطية كافية لقياس النمو: يلزم يوم واحد على الأقل بين الأقدم والأحدث.
infra-memory-title = الذاكرة
infra-memory-ok = مستعمل { $used } من سقف الخدمة البالغ { $max } ({ $percent }٪).
infra-memory-high = مستعمل { $used } من سقف الخدمة البالغ { $max } ({ $percent }٪): قريب من الحد الذي يوقف عنده النظامُ الخدمةَ.
infra-memory-fix = ارفع ‎MemoryMax=‎ في ملف drop-in لـ axgf-cms.service، أو اعثر على ما يحتجز الذاكرة — استيراد ضخم جدًا، مرفق كبير — وأعد تشغيل الخدمة.
infra-memory-unlimited = قيد الاستعمال { $used }، دون سقف مضبوط للخدمة.
infra-version-title = الإصدار
infra-version-ok = v{ $current } هو أحدث إصدار منشور.
infra-version-behind = يعمل v{ $current }؛ وقد نُشر { $latest }.
infra-version-behind-fix = اقرأ ملاحظات الإصدار، ثم حدّث من قسم الإصدار أدناه. تُؤخذ نسخة احتياطية أولًا، ويعود الإصدار السابق تلقائيًا إن لم يبدأ الجديد.
infra-version-unknown = لا يُعرف إن كان هناك إصدار أحدث: لم تُجلب قائمة الإصدارات.
infra-version-unknown-fix = استعمل «البحث عن إصدارات جديدة» أدناه. لا تصل بيئة الإنتاج المعزولة إلى GitHub بنفسها؛ وبعد تثبيت مساعد الترقية يجلب هو القائمة كل يوم.
infra-version-stale = { $days ->
    [zero] جُلبت قائمة الإصدارات اليوم.
    [one] جُلبت قائمة الإصدارات آخر مرة قبل يوم واحد.
    [two] جُلبت قائمة الإصدارات آخر مرة قبل يومين.
    [few] جُلبت قائمة الإصدارات آخر مرة قبل { $days } أيام.
    [many] جُلبت قائمة الإصدارات آخر مرة قبل { $days } يومًا.
    *[other] جُلبت قائمة الإصدارات آخر مرة قبل { $days } يوم.
    }
infra-upgrades-title = الترقيات
infra-upgrades-ok = مساعد الترقية مثبّت: يستطيع الزر أدناه تغيير الإصدار.
infra-upgrades-manual = مساعد الترقية غير مثبّت، لذا يُغيَّر الإصدار يدويًا على الخادم.
infra-upgrades-fix = شغّل bootstrap.sh الحالي مرة واحدة بصلاحيات root: فهو يثبّت المساعد مع الخدمة.
infra-sandbox-title = العزل
infra-sandbox-ok = تعمل العملية مع no-new-privileges ومرشّح لاستدعاءات النظام.
infra-sandbox-unknown = لا يُبلغ هذا النظام عن عزل العملية.
infra-sandbox-off = تعمل العملية دون جزء من عزلها: no-new-privileges أو مرشّح استدعاءات النظام معطّل.
infra-sandbox-fix = شغّل الخدمة من الوحدة التي يكتبها bootstrap.sh، دون ملفات drop-in تخفّف قيودها. وعلى جهاز تطوير هذا متوقَّع.
infra-clock-title = الساعة
infra-clock-ok = تشير ساعة النظام إلى عام { $year }، وهذا معقول.
infra-clock-wrong = تشير ساعة النظام إلى عام { $year }: التواريخ وأعمار النسخ الاحتياطية والجلسات كلها خاطئة.
infra-clock-fix = فعّل مزامنة الوقت: timedatectl set-ntp true.

# Administration console: the Infrastructure page (v0.5.0, docs/adr/0011)
infra-title = البنية التحتية
infra-lede = جهاز هذا الموقع: الوصول والمفاتيح والبيانات والإصدار والعزل، تُفحص عند كل فتح للصفحة.
infra-kpi-health = الحالة
infra-kpi-version = الإصدار
infra-kpi-version-newer = { $version } متاح
infra-kpi-version-current = محدّث
infra-kpi-version-unknown = لم يُفحص بعد
infra-kpi-disk = المساحة المتاحة
infra-kpi-disk-note = مستعمل { $percent }٪ من { $total }
infra-kpi-compute = الجهاز
infra-kpi-cpus = { $n ->
    [zero] { $n } نواة
    [one] نواة واحدة
    [two] نواتان
    [few] { $n } أنوية
    [many] { $n } نواة
    *[other] { $n } نواة
    }
infra-kpi-load = الحِمل { $load }
infra-kpi-rss = قيد الاستعمال { $size }
infra-unknown = غير معروف
infra-health = الحالة
infra-health-intro = الأخضر سليم، والتحذير يستحق نظرة، والحرج يحتاج إجراءً فوريًا.
infra-json = الفحوص نفسها بصيغة JSON، لأداة مراقبة
infra-version = الإصدار
infra-version-running = قيد التشغيل
infra-version-latest = أحدث إصدار منشور
infra-version-up-to-date = محدّث
infra-version-checked = جُلبت القائمة
infra-version-never = أبدًا
infra-source-direct = بواسطة هذا الخادم
infra-source-helper = بواسطة مساعد الترقية
infra-version-token = مستودع خاص
infra-token-env-set = رمز GitHub مضبوط لسرد الإصدارات وتنزيلها.
infra-token-env-unset = لا رمز. إن كان المستودع خاصًا فضع رمزًا في { $name } لا يقرؤه إلا root: فمساعد الترقية يقرؤه من هناك.
infra-releases-error = فشلت آخر محاولة لجلب القائمة:
infra-releases-refresh = البحث عن إصدارات جديدة
infra-releases-asked = طُلب من مساعد الترقية جلب قائمة جديدة. أعد تحميل هذه الصفحة بعد دقيقة.
infra-releases-not-asked = تعذّرت كتابة الطلب: { $error }
infra-upgrade-status-running = جارٍ التغيير من { $from } إلى { $to }…
infra-upgrade-status-succeeded = تم التغيير من { $from } إلى { $to }.
infra-upgrade-status-failed = فشل التغيير من { $from } إلى { $to }؛ وما زال { $from } هو الإصدار العامل.
infra-upgrade-status-refused = رفض مساعد الترقية التغيير إلى { $to }.
infra-upgrade-log = مخرجات المساعد
infra-upgrade-pending = طلب { $by } الإصدار { $version } في { $at }؛ ولم يتسلّمه مساعد الترقية بعد.
infra-releases = الإصدارات المنشورة
infra-releases-tag = الإصدار
infra-releases-published = تاريخ النشر
infra-releases-relation = مقارنةً بهذا
infra-releases-notes = ملاحظات الإصدار
infra-releases-open = اقرأ
infra-relation-newer = أحدث
infra-relation-current = هذا
infra-relation-older = أقدم
infra-prerelease = إصدار مرشّح
infra-releases-none = لا توجد قائمة إصدارات بعد.
infra-upgrade = تغيير الإصدار
infra-upgrade-intro = يأخذ مساعد التحديث نسخة احتياطية، ويثبّت الإصدار المُتحقَّق منه، ويعيد التشغيل ويفحص؛ وإن فشل عاد الإصدار السابق.
infra-upgrade-version = الإصدار المراد تثبيته
infra-upgrade-confirm = للعودة إلى إصدار أقدم، اكتب وسمه مرة أخرى
infra-upgrade-confirm-hint = لا حاجة إليه لإصدار أحدث. قد لا يقرأ الإصدار الأقدم بيانات كتبها إصدار أحدث: اقرأ ملاحظاته أولًا.
infra-upgrade-go = تثبيت هذا الإصدار
infra-upgrade-safety = يتعذّر الوصول إلى الموقع بضع ثوانٍ أثناء إعادة تشغيل الخدمة.
infra-upgrade-manual = مساعد الترقية غير مثبّت على هذا الخادم، لذا لا تستطيع هذه الصفحة تغيير الإصدار. بصلاحيات root على الخادم:
infra-upgrade-manual-helper = شغّل bootstrap.sh الحالي مرة واحدة لتثبيت المساعد.
infra-upgrade-asked = طُلب الإصدار { $version }. يتسلّمه مساعد الترقية خلال ثوانٍ؛ وتُظهر هذه الصفحة كيف جرى ذلك.
infra-upgrade-no-helper = مساعد الترقية غير مثبّت، لذا لا يمكن تثبيت { $version } من هنا.
infra-upgrade-unknown-version = { $version } ليس في قائمة الإصدارات المنشورة.
infra-upgrade-same-version = { $version } هو الإصدار العامل حاليًا.
infra-upgrade-confirm-older = { $version } أقدم من الإصدار العامل. اكتب وسمه في حقل التأكيد للعودة إليه.
infra-volumes = التخزين
infra-volumes-intro = القسم الذي يضم مجلد البيانات ({ $path })، وما يحفظه هذا الموقع عليه.
infra-volumes-partition = القسم
infra-of = { $used } من { $total }
infra-volumes-ours = هذا الموقع
infra-volumes-breakdown = ما يحفظه هذا الموقع
infra-part-bundle = أرشيف العائلة
infra-part-cache = ذاكرة المرفقات المؤقتة
infra-part-backups = النسخ الاحتياطية
infra-part-binary = البرنامج
infra-part-journal = سجل التغييرات
infra-part-accounts = الحسابات والإعدادات
infra-files = { $n ->
    [zero] لا ملفات
    [one] ملف واحد
    [two] ملفان
    [few] { $n } ملفات
    [many] { $n } ملفًا
    *[other] { $n } ملف
    }
infra-growth = النمو
infra-growth-month = شهريًا
infra-growth-not-enough = لا توجد نسخ كافية للحكم
infra-growth-left = المساحة الباقية بهذا المعدل
infra-backups = النسخ الاحتياطية
infra-backups-dir = المجلد
infra-backups-count = الأرشيفات
infra-backups-newest = الأحدث
infra-backups-oldest = الأقدم
infra-backups-room = المساحة حيث تُحفظ
infra-free-of = { $free } متاحة من { $total }
infra-paths = المسارات
infra-path-data = مجلد البيانات
infra-path-binary = البرنامج
infra-compute = الحوسبة
infra-compute-hidden = يُخفي عزلُ الخدمة عنها أرقام الجهاز نفسه؛ وما يظهر هنا هو ما تستطيع العملية رؤيته.
infra-compute-memory = ذاكرة الجهاز
infra-available-of = { $available } متاحة من { $total }
infra-compute-ceiling = ذاكرة الخدمة
infra-compute-no-ceiling = { $used }، دون سقف
infra-compute-os = النظام
infra-compute-kernel = النواة
infra-compute-arch = المعمارية
infra-compute-cpu = المعالج
infra-compute-cpus = الأنوية
infra-compute-load = الحِمل (1، 5، 15 دقيقة)
infra-compute-rss = هذه العملية
infra-compute-threads = الخيوط
infra-compute-files = الملفات المفتوحة
infra-compute-uptime = الخدمة تعمل منذ
infra-compute-sys-uptime = الجهاز يعمل منذ
infra-compute-sandbox = العزل
infra-compute-nnp-on = دون صلاحيات جديدة
infra-compute-nnp-off = يمكن اكتساب صلاحيات
infra-compute-seccomp-on = استدعاءات النظام مُرشَّحة
infra-compute-seccomp-off = استدعاءات النظام غير مُرشَّحة
infra-activity = النشاط
infra-activity-intro = على مدى { $span }؛ يبدأ من الصفر عند إعادة تشغيل الخدمة.
infra-activity-sessions = الجلسات المفتوحة
infra-activity-signed-in = عمليات الدخول
infra-activity-failed = محاولات الدخول الفاشلة
infra-activity-registered = الحسابات المُنشأة
infra-activity-cross-site = الإرسالات المرفوضة من مواقع أخرى

# Sign-in services: what the reader is told when one fails (v0.5.0, docs/adr/0012)
sso-error-not-configured = خدمة تسجيل الدخول هذه غير مُعدّة على هذا الموقع.
sso-error-no-public-url = يتطلب الدخول عبر خدمةٍ العنوانَ العام لهذا الموقع، ولم يُضبط بعد. يضبطه المسؤول في صفحة الاتصال.
sso-error-unreachable = تعذّر الوصول إلى خدمة تسجيل الدخول. أعد المحاولة بعد قليل، أو ادخل بطريقة أخرى.
sso-error-refused = لم تسمح لك خدمة تسجيل الدخول بالدخول، أو أُلغي الدخول هناك.
sso-error-stale = انتهت صلاحية عملية الدخول هذه، أو بدأت في متصفح آخر. ابدأ من جديد.
sso-error-invalid = لم يجتز ردّ خدمة تسجيل الدخول فحوص هذا الموقع، فلم يُستعمل. ابدأ من جديد؛ وإن تكرر ذلك فأخبر المسؤول.
infra-upgrade-already-pending = { $version } ينتظر مساعد الترقية بالفعل. انتظر حتى ينتهي قبل طلب إصدار آخر.

# Administration console: Connectivity, sign-in services and registration (v0.5.0, docs/adr/0012)
connectivity-title = الاتصال
connectivity-lede = كيف يصل القرّاء إلى هذا الموقع ويدخلون: العنوان العام وخدمات الدخول والقواعد.
connectivity-kpi-address = العنوان العام
connectivity-address-unset = غير مضبوط
connectivity-address-https = HTTPS
connectivity-address-plain = غير مشفّر
connectivity-address-unset-note = تحتاج إليه كل خدمة تسجيل دخول
connectivity-kpi-services = خدمات تسجيل الدخول
connectivity-kpi-services-note = { $n ->
    [zero] لا خدمة جاهزة
    [one] جاهزة للعرض
    [two] جاهزتان للعرض
    [few] جاهزة للعرض
    [many] جاهزة للعرض
    *[other] جاهزة للعرض
    }
connectivity-kpi-passwords = كلمات المرور
connectivity-policy-short-everyone = الجميع
connectivity-policy-short-non-admins = عدا المسؤولين
connectivity-policy-short-nobody = لا أحد
connectivity-policy-note-everyone = يستطيع كل حساب الدخول بكلمة مروره، وبخدمة مرتبطة أيضًا.
connectivity-policy-note-non-admins = يدخل المسؤولون عبر خدمة فقط؛ ويبقى للآخرين استعمال كلمة المرور. فلا تفتح كلمة مرور مسؤول مسروقة شيئًا.
connectivity-policy-note-nobody = يدخل الجميع عبر خدمة؛ والحساب بلا خدمة مربوطة لا يمكنه الدخول.
connectivity-kpi-admins-linked = { $n ->
    [zero] لا مسؤول لديه خدمة مرتبطة
    [one] مسؤول واحد لديه خدمة مرتبطة
    [two] مسؤولان لديهما خدمة مرتبطة
    [few] { $n } مسؤولين لديهم خدمة مرتبطة
    [many] { $n } مسؤولًا لديهم خدمة مرتبطة
    *[other] { $n } مسؤول لديهم خدمة مرتبطة
    }
connectivity-kpi-registration = التسجيل
connectivity-kpi-registration-go = يُضبط في صفحة الحسابات ←
registration-short-closed = المسؤولون فقط
registration-short-secret = بسرّ العائلة
registration-short-open = مفتوح للزوار
connectivity-address = العنوان العام
connectivity-address-intro = العنوان العام الذي يكتبه القرّاء، مثل https://family.example؛ وتعيدهم خدمات الدخول إليه.
connectivity-address-label = العنوان العام
connectivity-address-hint = اتركه فارغًا ما دام لا أحد يدخل عبر خدمة.
connectivity-address-invalid = هذا ليس عنوان http أو https فيه اسم مضيف.
connectivity-address-saved = حُفظ العنوان العام.
connectivity-policy = قواعد تسجيل الدخول
connectivity-policy-intro = تسري على كل الحسابات؛ وتُرفض أي قاعدة لا تترك مديرًا قادرًا على الدخول.
connectivity-policy-passwords = من يستطيع الدخول بكلمة مرور
connectivity-policy-without-way = { $n ->
    [zero] لا حساب نشطًا بلا خدمة مرتبطة ولا عنوان.
    [one] حساب نشط واحد بلا خدمة مرتبطة ولا عنوان: مع «لا أحد» لن يستطيع الدخول.
    [two] حسابان نشطان بلا خدمة مرتبطة ولا عنوان: مع «لا أحد» لن يستطيعا الدخول.
    [few] { $n } حسابات نشطة بلا خدمة مرتبطة ولا عنوان: مع «لا أحد» لن تستطيع الدخول.
    [many] { $n } حسابًا نشطًا بلا خدمة مرتبطة ولا عنوان: مع «لا أحد» لن تستطيع الدخول.
    *[other] { $n } حساب نشط بلا خدمة مرتبطة ولا عنوان: مع «لا أحد» لن تستطيع الدخول.
    }
connectivity-link-by-email = ربط الخدمة بالحساب الذي يحمل العنوان المؤكَّد نفسه
connectivity-link-by-email-note = أنشئ حساب القريب بعنوانه: أول دخول له عبر خدمة تتحقق من العنوان يربطه.
connectivity-domains = النطاقات المسموح لها بفتح حساب
connectivity-domains-hint = واحد في كل سطر (family.example). فارغ: الكل. وإلا فلا يفتح حسابًا إلا عنوان مُتحقَّق منه في هذه النطاقات.
connectivity-domain-invalid = «{ $domain }» ليس اسم نطاق.
connectivity-policy-unknown = اختر من يستطيع الدخول بكلمة مرور.
connectivity-policy-lockout = مرفوض: مع هذا التغيير لن يستطيع أي مسؤول نشط الدخول. اربط أولًا خدمة تسجيل دخول بحساب مسؤول بالدخول عبرها مرة، أو أبقِ كلمات المرور للمسؤولين.
connectivity-policy-saved = حُفظت قواعد تسجيل الدخول.
connectivity-not-saved = لم يُحفظ: { $error }
connectivity-services = خدمات تسجيل الدخول
connectivity-services-intro = سجّل هذا الموقع في لوحة الخدمة بعنوان إعادة التوجيه الظاهر، ثم الصق هنا معرّف العميل والسر.
connectivity-outbound-note = يتطلب الدخول عبر خدمة اتصالات صادرة: نفّذ sudo systemctl edit axgf-cms، وأضف IPAddressAllow=any تحت [Service]، ثم أعد التشغيل.
connectivity-service-ready = جاهزة
connectivity-service-incomplete = غير مكتملة
connectivity-service-off = معطّلة
connectivity-service-enable = عرض هذه الخدمة في صفحتي الدخول والتسجيل
connectivity-service-issuer = المُصدِر
connectivity-service-issuer-hint = عنوان المزوّد، مثل https://auth.family.example/realms/family؛ وتُقرأ إعداداته من ‎/.well-known/openid-configuration تحته.
connectivity-service-label = نص الزر
connectivity-service-tenant = الدليل (المستأجر)
connectivity-service-tenant-hint = common لأي حساب Microsoft، وorganizations لحسابات العمل والدراسة، وconsumers للحسابات الشخصية، أو معرّف دليلك.
connectivity-service-client-id = معرّف العميل
connectivity-service-secret = سرّ العميل
connectivity-service-secret-set = السرّ مضبوط. اترك هذا فارغًا للإبقاء عليه.
connectivity-service-secret-unset = لم يُضبط سرّ بعد.
connectivity-service-secret-clear = إزالة السرّ
connectivity-service-redirect = عنوان إعادة التوجيه المراد تسجيله
connectivity-service-redirect-needs-address = اضبط العنوان العام أولًا
connectivity-service-console = أين تُسجّله
connectivity-service-console-open = لوحة { $name }
connectivity-service-test = اختبار الاتصال
connectivity-issuer-invalid = يجب أن يكون المُصدِر عنوان https://‎.
connectivity-tenant-invalid = الدليل اسم أو معرّف: حروف وأرقام ونقاط وشرطات.
connectivity-provider-incomplete = { $name } غير مكتمل: يلزمه معرّف عميل وسرّ عميل، ومُصدِر إن كان مزوّدًا عامًا.
connectivity-provider-saved = حُفظ { $name }.
connectivity-test-ok = أجاب { $name }: { $answer }.
connectivity-test-failed = تعذّر الوصول إلى { $name } من هذا الخادم: { $detail }
registration-title = التسجيل
registration-intro = هل يمكن للزائر فتح حساب عضو بنفسه؛ ويمكن للمدير منحه دورًا آخر أدناه.
registration-who = من يستطيع فتح حساب
registration-note-closed = لا يُنشئ الحسابات إلا المسؤول، في هذه الصفحة.
registration-note-secret = من يعرف سرّ العائلة يمكنه فتح حساب عضو. شاركه كما تشارك رمز باب.
registration-note-open = يستطيع أي زائر فتح حساب عضو. ويقرأ الأعضاء السجلات المخصّصة للأعضاء.
registration-rotate = توليد سرّ جديد
registration-secret = سرّ العائلة
registration-secret-note = يستطيع كل من يملكه فتح حساب ما دام هذا الوضع مفعّلًا. وتوليد سرّ جديد يُبطل القديم؛ وتبقى الحسابات المفتوحة.
registration-link = صفحة التسجيل
registration-quick = طرق الدخول السريعة
registration-quick-none = لا خدمة تسجيل دخول جاهزة: أعِدّ واحدة في صفحة الاتصال
registration-changed = آخر تغيير بواسطة { $by }، { $at }.
registration-unknown = اختر من يستطيع فتح حساب.
accounts-kpi-total = الحسابات النشطة
accounts-kpi-total-note = { $disabled ->
    [zero] لا حسابات معطّلة
    [one] حساب واحد معطّل
    [two] حسابان معطّلان
    [few] { $disabled } حسابات معطّلة
    [many] { $disabled } حسابًا معطّلًا
    *[other] { $disabled } حساب معطّل
    }
accounts-kpi-admins = المسؤولون
accounts-kpi-admins-few = أضف مسؤولًا ثانيًا
accounts-kpi-admins-ok = أكثر من واحد، كما ينبغي
accounts-kpi-linked = مرتبطة بخدمة
accounts-kpi-linked-note = { $n ->
    [zero] لا خدمة جاهزة
    [one] خدمة واحدة جاهزة
    [two] خدمتان جاهزتان
    [few] { $n } خدمات جاهزة
    [many] { $n } خدمة جاهزة
    *[other] { $n } خدمة جاهزة
    }
accounts-kpi-registered = { $n ->
    [zero] لم يفتح الزوار أي حساب
    [one] حساب واحد فتحه زائر
    [two] حسابان فتحهما زوار
    [few] { $n } حسابات فتحها زوار
    [many] { $n } حسابًا فتحه زوار
    *[other] { $n } حساب فتحه زوار
    }
accounts-roles = الأدوار
accounts-sign-in = يدخل بواسطة
accounts-sign-in-password = كلمة مرور
accounts-sign-in-google = Google
accounts-sign-in-microsoft = Microsoft
accounts-sign-in-github = GitHub
accounts-sign-in-oidc = الدخول الموحّد
accounts-sign-in-none = لا طريقة للدخول بعد
accounts-created-via-register = فتحه الزائر
accounts-created-via-sso = فُتح عبر خدمة
accounts-add-also-register = يستطيع الزوار أيضًا فتح حسابات أعضاء بأنفسهم، كما ضُبط أعلاه. ويمكن للحساب المُنشأ هنا أن يحمل أي دور.
login-password-not-allowed = لا تسمح قواعد الدخول في هذا الموقع لهذا الحساب باستعمال كلمة مرور: ادخل عبر خدمة.
login-register = ليس لديك حساب بعد؟ أنشئ واحدًا
register-title = إنشاء حساب
register-lede = يقرأ حساب العضو السجلات المخصّصة لأعضاء العائلة. وأسرع طريقة حسابٌ تملكه أصلًا.
register-closed = يُنشئ المسؤول الحسابات هنا. اطلب منه حسابك.
register-have-account = لديك حساب؟ سجّل الدخول
register-or = أو
register-secret = سرّ العائلة
register-secret-hint = أعطاك إياه مسؤول، أو أحد أفراد العائلة.
register-secret-hint-services = اكتبه قبل اختيار خدمة، وإلا طلبته الخطوة التالية.
register-domains = لا تفتح حسابًا إلا العناوين في { $domains }.
register-username-hint = من 2 إلى 64 من الحروف أو الأرقام أو النقاط أو الشرطات أو الشرطات السفلية.
register-submit = إنشاء الحساب
register-member-note = لا يُطلب أي عنوان: لا يُرسل إليك شيء، ولا يستطيع أحد ادّعاء عنوانك.
register-form-closed = لا يفتح هذا الموقع حسابات بهذا النموذج.
register-throttled = محاولات كثيرة جدًا من هنا. انتظر ربع ساعة وأعد المحاولة.
register-secret-wrong = هذا ليس سرّ العائلة.
register-username-invalid = يتكوّن اسم المستخدم من 2 إلى 64 من الحروف أو الأرقام أو النقاط أو الشرطات أو الشرطات السفلية.
register-password-short = تحتاج كلمة المرور إلى { $min } محرفًا على الأقل.
register-username-taken = اسم المستخدم هذا محجوز. اختر غيره.
register-not-saved = تعذّر حفظ الحساب. أعد المحاولة بعد قليل.
register-finish-title = خطوة أخرى
register-finish-lede = أكّدت الخدمة هويتك. لفتح حساب هنا، اكتب سرّ العائلة.
register-finish-lede-email = أكّدتك الخدمة بصفتك { $email }. لفتح حساب هنا، اكتب سرّ العائلة.
sso-continue = متابعة
sso-continue-with = المتابعة باستخدام { $name }
sso-error-title = لم يكتمل تسجيل الدخول
sso-account-disabled = الحساب المرتبط بهذا الدخول معطّل. راجع المسؤول.
sso-no-account = لا حساب هنا يطابق هذا الدخول، ولا يفتح هذا الموقع حسابات من تلقاء نفسه. اطلب من مسؤول إنشاء حسابك بهذا العنوان: وسيرتبط به دخولك التالي.
sso-domain-refused = لا تُفتح الحسابات هنا إلا بعنوان من نطاقات العائلة تؤكّده الخدمة.

# Administration console: the overview's tiles (v0.5.0, docs/adr/0011)
overview-accounts-note = { $admins ->
    [zero] لا مسؤولين
    [one] مسؤول واحد
    [two] مسؤولان
    [few] { $admins } مسؤولين
    [many] { $admins } مسؤولًا
    *[other] { $admins } مسؤول
    }
register-password-hint = { $min } محرفًا على الأقل. وتُحفظ عبارة من عدة كلمات أسهل من خليط من الرموز.

# Clarity: the light theme and its funnel (v0.5.0, docs/adr/0013)
theme-clarity = الصفاء
theme-clarity-note = فاتح، أبيض وبرتقالي · قمع
view-funnel-back = العودة إلى القمع
view-funnel-open = افتح القمع
view-funnel-hero-1 = كل جيل حلقة.
view-funnel-hero-2 = وكل عائلة قمع.
view-funnel-cta = استكشف القمع
view-funnel-cta-detail = كل اسم يُضاف وكل مصدر يُتحقق منه يجعلان القمع أصدق لمن يأتي بعدنا.
view-funnel-see = عرض في القمع
view-funnel-direction = ↓ الأقدم عند الطرف الضيق · ↑ الأحدث عند الفوهة
view-funnel-still = أوقف حركة القمع
view-funnel-name = القمع
view-funnel-legend = كيف تقرأ القمع
view-funnel-search-hint = يضعه Enter في المقدّمة؛ ويفتح Shift+Enter سجلّه.
theme-genealogy = علم الأنساب
theme-genealogy-note = أخضر داكن وكريمي · شجرة للتسلّق
view-scroll-back = العودة إلى شجرة النسب
view-scroll-open = فتح شجرة النسب
view-scroll-hero-1 = كل حياة صورة.
view-scroll-hero-2 = وكل عائلة شجرة نتسلّقها.
view-scroll-cta = استكشف شجرة النسب
view-scroll-cta-detail = كل اسم يُضاف وكل مصدر يُتحقَّق منه يجعل شجرة النسب أصدق لمن سيأتون بعدنا.
view-scroll-see = عرض في شجرة النسب
view-scroll-direction = ↓ نحو الجذور: الأسلاف · ↑ نحو التاج: الأحفاد
view-scroll-still = إيقاف حركة الأوراق
view-scroll-name = شجرة النسب
view-scroll-legend = كيف تُقرأ شجرة النسب
view-scroll-search-hint = يضعه Enter على الجذع، ويفتح Shift+Enter سجله.
bundle-binding-unnamed = أرشيف بلا اسم
bundle-binding-created = أرشيف أُنشئ في { $date }
bundle-binding-named = «{ $family }»
bundle-binding-named-created = «{ $family }»، أُنشئ في { $date }

# Notes and contributions (docs/adr/0014)
notes-section-title = ملاحظات ومساهمات
notes-section-intro = ما دوّنه أفراد العائلة: ذكريات وأبحاث ورسائل ووثائق. ابحث فيها، أو تصفّحها حسب الحرف، أو افتح واحدة لتقرأها وتحفظها بصيغة PDF.
notes-search-label = البحث في الملاحظات
notes-search-go = بحث
notes-all = الكل
notes-letters-label = الملاحظات حسب الحرف الأول من عنوانها
notes-shown = المعروض: { $shown } من { $total }
notes-none = لم تُكتب أي ملاحظة بعد.
notes-none-found = لا توجد ملاحظة تطابق هذا البحث.
notes-write = كتابة ملاحظة
notes-family-wide = العائلة كلها
notes-by = بقلم { $author }
notes-about = بشأن: { $names }
notes-about-prefix = بشأن
notes-about-family = بشأن العائلة كلها
notes-pdf = تنزيل بصيغة PDF
notes-edit = تعديل
notes-close = إغلاق
notes-back = العودة إلى الملاحظات
notes-add-about = كتابة ملاحظة عن هذا الشخص
notes-sign-in = ملاحظات العائلة ومساهماتها مخصّصة لأفرادها. سجّل الدخول لقراءتها.
notes-admin-title = ملاحظات ومساهمات
notes-admin-intro = اكتب ما لا تتّسع له السجلات: ذكرى أو بحثًا أو رسالة، مع صور. الملاحظة المرتبطة بأشخاص تظهر في سجلاتهم؛ والملاحظة غير المرتبطة بأحد تخص العائلة كلها. يقرأ كل أفراد العائلة جميع الملاحظات في صفحة العائلة.
notes-deleted = حُذفت الملاحظة.
notes-col-title = العنوان
notes-col-category = الفئة
notes-col-date = التاريخ
notes-col-about = بشأن
notes-col-author = الكاتب
notes-col-actions = إجراءات
notes-new-title = ملاحظة جديدة
notes-edit-title = تعديل الملاحظة
notes-section-about = الملاحظة
notes-field-title = العنوان
notes-field-category = الفئة
notes-field-category-hint = نص حر؛ تُقترح الفئات المستخدمة من قبل.
notes-field-date = التاريخ
notes-field-date-hint = 1952 أو 1952-06 أو 1952-06-14؛ فارغ إن لم يوجد.
notes-field-people = الأشخاص المعنيّون
notes-field-people-hint = اكتب اسمًا واختره. إن لم يُذكر أحد، فالملاحظة للعائلة كلها.
notes-person-add = إضافة
notes-person-remove = إزالة هذا الشخص
notes-field-text = النص
notes-text-hint = Markdown أو الأزرار أعلاه. الصق الصور أو أفلتها، حتى { $mb } م.ب لكل منها.
notes-tool-bold = عريض
notes-tool-italic = مائل
notes-tool-heading = عنوان فرعي
notes-tool-list = قائمة
notes-tool-quote = اقتباس
notes-tool-link = رابط
notes-tool-image = إضافة صورة
notes-tool-preview = معاينة
notes-tool-write = العودة إلى الكتابة
notes-uploading = جارٍ إضافة الصورة…
notes-save = حفظ الملاحظة
notes-cancel = إلغاء
notes-delete = حذف هذه الملاحظة
notes-delete-confirm = ستُزال الملاحظة وصورها من الأرشيف.
notes-category-suggestions = ذكرى, بحث, رسالة, حكاية, وصفة, وثيقة
notes-error-title = تحتاج الملاحظة إلى عنوان. لم يُحفظ شيء.
notes-error-title-long = العنوان طويل جدًا: 200 حرف على الأكثر. لم يُحفظ شيء.
notes-error-text-long = النص طويل جدًا. لم يُحفظ شيء.
notes-error-date = هذا التاريخ غير مفهوم. اكتب سنة أو شهرًا أو يومًا: 1952 أو 1952-06 أو 1952-06-14. لم يُحفظ شيء.
notes-error-person = الأشخاص المعنيّون: «{ $name }».
notes-error-image-gone = لم تعد صورة ملصقة تنتظر الحفظ؛ فالصور تنتظر ست ساعات. أضفها من جديد. لم يُحفظ شيء.
notes-error-too-many-images = تتسع الملاحظة لـ 40 صورة على الأكثر. لم يُحفظ شيء.
notes-error-stale = عدّل أحدهم هذه الملاحظة بينما كنت تعمل عليها. افتحها من جديد لترى نسخته، ثم أجرِ تعديلك. لم يُحفظ شيء.
notes-error-refused = رفض الأرشيف هذا التعديل. لم يُحفظ شيء.
notes-error-image = هذا الملف ليس صورة يستطيع هذا الموقع عرضها: PNG أو JPEG أو GIF أو WebP.
notes-error-image-large = هذه الصورة كبيرة جدًا.
notes-stale-title = لم تُعدَّل الملاحظة
notes-missing-title = الملاحظة غير موجودة
notes-missing = لا توجد ملاحظة هنا، أو أنها غير متاحة لك.
notes-not-yours = يعدّل المساهم الملاحظات التي كتبها فقط. يستطيع المسؤول تعديل هذه.
notes-pdf-failed-title = تعذّر إنشاء ملف PDF
notes-pdf-failed = حدث خطأ أثناء إنشاء ملف PDF لهذه الملاحظة. اقرأها في صفحتها إلى حين ذلك، وأبلغ أحد المسؤولين.
notes-dashboard-count = الملاحظات المكتوبة: { $n }
notes-dashboard-open = كتابة الملاحظات وإدارتها

# The family the archive describes: the bundle room's form (2026-10-23)
kind-bundle = الأرشيف
manifest-title = العائلة
manifest-intro = ما يقوله الأرشيف عن عائلته. يظهر اسمه في صفحة العائلة وعناوين الصفحات وملفات PDF للملاحظات.
manifest-name = اسم العائلة
manifest-name-hint = يظهر في صفحة العائلة وفي العناوين. تتبع الحسابات الاسم الجديد.
manifest-description = الوصف
manifest-culture = الثقافة الرئيسية
manifest-culture-hint = وسم لغة: fr أو pl أو pt-BR…
manifest-place = المكان الرئيسي
manifest-place-hint = القارة التي عاشت فيها العائلة في الغالب.
manifest-span = الفترة المشمولة
manifest-span-hint = من أقدم ميلاد إلى أحدثه؛ يُحسب عند الحفظ.
manifest-license = الترخيص
manifest-license-type = الترخيص
manifest-save = حفظ بيانات العائلة
manifest-saved = حُفظت بيانات العائلة. تتبع الحسابات اسمها.
manifest-error-long = أحد الحقول طويل جدًا: 120 حرفًا للاسم، و2000 للوصف. لم يُحفظ شيء.
manifest-error-culture = الثقافة وسم لغة مثل fr أو pl أو pt-BR. لم يُحفظ شيء.
manifest-error-stale = عدّل أحدهم بيانات العائلة بينما كنت تعدّلها. نسختك أدناه: حفظها مجددًا يستبدل نسخته. لم يُحفظ شيء.
manifest-error-io = تعذّرت كتابة الأرشيف؛ سجل الخادم يذكر السبب. لم يُحفظ شيء.

# The family form: places, licences and the period (2026-10-23)
manifest-none = غير محدد
manifest-span-value = { $earliest } – { $latest }
manifest-span-none = لا يوجد تاريخ ولادة مسجّل بعد.
manifest-license-pick = اختر ترخيصًا لترى ما يتيحه للآخرين أن يفعلوه بالأرشيف.
manifest-license-author = المؤلف
manifest-license-author-value = { $who }، { $at }
manifest-license-author-none = يُسجَّل عند اختيار ترخيص.
manifest-error-place = اختر المكان الرئيسي من القائمة. لم يُحفظ شيء.
manifest-error-licence = اختر الترخيص من القائمة. لم يُحفظ شيء.
continent-africa = أفريقيا
continent-antarctica = القارة القطبية الجنوبية
continent-asia = آسيا
continent-europe = أوروبا
continent-north-america = أمريكا الشمالية
continent-south-america = أمريكا الجنوبية
continent-oceania = أوقيانوسيا
continent-several = عدة قارات
licence-all-rights-reserved-name = جميع الحقوق محفوظة
licence-all-rights-reserved-about = لا يجوز لأحد نسخ الأرشيف أو مشاركته أو إعادة استخدامه دون إذن العائلة. وهذا ما يسري حين لا يُذكر شيء.
licence-family-private-name = خاص بالعائلة
licence-family-private-about = مخصّص لأفراد العائلة وحدهم: لا يُنشر ولا يُنقل خارجها.
licence-cc0-name = CC0 1.0 — ملكية عامة
licence-cc0-about = يحق لأي شخص نسخ الأرشيف وتعديله واستخدامه لأي غرض، حتى التجاري، دون استئذان أو ذكر أحد.
licence-cc-by-name = CC BY 4.0 — نسب المصنف
licence-cc-by-about = يحق لأي شخص مشاركة الأرشيف وتعديله، حتى تجاريًا، شرط أن ينسبه إلى مؤلفيه ويذكر ما غيّره.
licence-cc-by-sa-name = CC BY-SA 4.0 — نسب المصنف، الترخيص بالمثل
licence-cc-by-sa-about = يحق لأي شخص مشاركته وتعديله، حتى تجاريًا، مع نسبه إلى مؤلفيه؛ وما يُصنع منه يجب أن يُشارك بالترخيص نفسه.
licence-cc-by-nc-name = CC BY-NC 4.0 — نسب المصنف، غير تجاري
licence-cc-by-nc-about = يحق لأي شخص مشاركته وتعديله مع نسبه إلى مؤلفيه، لكن ليس لأغراض تجارية.
licence-cc-by-nc-sa-name = CC BY-NC-SA 4.0 — نسب المصنف، غير تجاري، الترخيص بالمثل
licence-cc-by-nc-sa-about = يحق لأي شخص مشاركته وتعديله مع نسبه إلى مؤلفيه، دون غرض تجاري وبالترخيص نفسه فقط.
licence-cc-by-nd-name = CC BY-ND 4.0 — نسب المصنف، منع الاشتقاق
licence-cc-by-nd-about = يحق لأي شخص مشاركته كما هو، حتى تجاريًا، مع نسبه إلى مؤلفيه، لكن لا يجوز نشر نسخة معدّلة.
licence-cc-by-nc-nd-name = CC BY-NC-ND 4.0 — نسب المصنف، غير تجاري، منع الاشتقاق
licence-cc-by-nc-nd-about = يحق لأي شخص مشاركته كما هو مع نسبه إلى مؤلفيه، لكن لا لأغراض تجارية ولا معدّلًا: أشد تراخيص المشاع الإبداعي تقييدًا.
licence-odbl-name = ODbL 1.0 — رخصة قواعد البيانات المفتوحة
licence-odbl-about = مصمَّمة لقواعد البيانات: يحق لأي شخص استخدام البيانات ومشاركتها وتعديلها مع ذكر مصدرها، ويجب أن تبقى أي قاعدة مستمدة منها مفتوحة بالشروط نفسها.

# An account's own page and its record in the tree (2026-10-23)
nav-account = حسابي
account-title = حسابي
account-who = الحساب
account-email = عنوان البريد الإلكتروني
account-since = منذ
account-person-title = سجلّي في الشجرة
account-person-intro = من أنت في الشجرة؟ اختيارك، بلا تحقق؛ يمكنك تغييره متى شئت.
account-person-is = يقول هذا الحساب إنه
account-person-unlink = إزالة الربط
account-person-pick = شخص في الشجرة
account-person-link = ربط حسابي بهذا السجل
account-person-change = الربط بهذا السجل بدلًا من ذلك
account-person-linked = حسابك مرتبط بسجلّه.
account-person-unlinked = لم يعد حسابك مرتبطًا بأي سجل.
account-person-error = تعذّر ربط هذا السجل:
account-person-io = تعذّر حفظ الحساب؛ سجل الخادم يذكر السبب. لم يتغيّر شيء.
account-emergency-page = رمز الطوارئ ليس حسابًا: ليست له صفحة خاصة ولا سجل في الشجرة.
accounts-person = السجل:

## The merge screen (src/routes/merge.rs): two records a person says are one.
merge-action = دمج
merge-needs-two = اختر سجلين بالضبط لدمجهما.
merge-title = دمج سجلين
merge-lede = لا يتغير شيء قبل أن تؤكد. يُحتفظ بالسجل الأول؛ ويُدمج الثاني فيه ويُحفظ كما كان في سجل التغييرات.
merge-keep = يُحتفظ به
merge-absorb = يُدمج فيه
merge-swap = الاحتفاظ بالآخر
merge-refused = هذا الدمج مرفوض:
merge-refusal-ancestor = أحدهما سلفٌ للآخر
merge-refusal-spouse-and-child = في العائلة نفسها، أحدهما زوج والآخر ابن
merge-refusal-spouses = هما الزوجان في الارتباط نفسه
merge-refusal-siblings = مسجّلان ولدَين للوالدين أنفسهما
merge-refusal-different-couples = العائلتان زوجان مختلفان
merge-refusal-child-is-spouse = ابن في إحدى العائلتين زوج في الأخرى
merge-refusal-same-record = إنه السجل نفسه مرتين
merge-conflicts = فيمَ يختلفان
merge-nothing = لا شيء للاختيار: لا يتناقضان.
merge-choice-keep = السجل المحتفظ به
merge-choice-absorb = السجل الآخر
merge-choice-both = كلاهما
merge-because-higher-confidence = المقترح: القيمة ذات درجة اليقين الأعلى.
merge-because-sourced = المقترح: القيمة التي تستشهد بمصدر.
merge-because-kept-record = المقترح: قيمة السجل المحتفظ به.
merge-because-more-restrictive = المقترح: الظهور الأكثر تقييدًا.
merge-because-protective = المقترح: على قيد الحياة، القراءة التي تحمي.
merge-because-both-kept = المقترح: الاحتفاظ بالنصين.
merge-becomes-alias = يُحفظ الاسم غير المختار اسمًا آخر.
merge-becomes-event = تُحفظ القيمة غير المختارة حدثًا.
merge-agreements = فيمَ يتفقان
merge-from-keep = من السجل المحتفظ به
merge-from-absorb = من السجل الآخر
merge-from-both = متطابق في الاثنين
merge-from-joined = قوائم مضمومة
merge-from-combined = مجمّعة
merge-effects = ما الذي يتغير أيضًا
merge-effect-references = مراجع أُعيدت كتابتها
merge-effect-self-links = روابط إلى نفسه أُزيلت
merge-effect-listed-twice = قوائم كانت ستذكره مرتين
merge-effect-same-couple = عائلات للزوجين أنفسهما
merge-confirm-label = قد يكونان مع ذلك شخصًا واحدًا. للدمج رغم ذلك، اكتب الاسم بدقة: { $name }
merge-go = دمج
merge-missing = أحد السجلين لم يعد موجودًا.
merge-done-title = تم الدمج
merge-done-lede = أصبح { $absorbed } جزءًا من { $kept }.
merge-report-references = مراجع أُعيدت كتابتها
merge-report-events = أحداث أُنشئت من قيمة غير مختارة
merge-report-source-conflicts = اختلافات سُجّلت على مصدر
merge-report-links = روابط أُزيلت
merge-report-tidied = مدخلات رُتّبت
merge-absorbed-record = السجل المدموج كما كان
merge-open-kept = فتح السجل المحتفظ به

## القرابة (docs/adr/0020)

kin-tab = القرابة
kin-parent = { $adoptive ->
        [yes] { $sex ->
            [M] أب بالتبني
            [F] أم بالتبني
           *[other] والد بالتبني
        }
       *[no] { $sex ->
            [M] أب
            [F] أم
           *[other] والد
        }
    }
kin-ancestor = { $n ->
        [2] { $sex ->
            [M] { $side ->
                [M] جد لأب
                [F] جد لأم
               *[other] جد
            }
            [F] { $side ->
                [M] جدة لأب
                [F] جدة لأم
               *[other] جدة
            }
           *[other] جد أو جدة
        }
        [3] { $sex ->
            [M] جد الأب أو الأم
            [F] جدة الأب أو الأم
           *[other] جد أكبر
        }
        [4] { $sex ->
            [M] جد أعلى
            [F] جدة عليا
           *[other] جد أعلى
        }
       *[other] سلف من الجيل { $n }
    }
kin-child = { $adoptive ->
        [yes] { $sex ->
            [M] ابن بالتبني
            [F] ابنة بالتبني
           *[other] ولد بالتبني
        }
       *[no] { $sex ->
            [M] ابن
            [F] ابنة
           *[other] ولد
        }
    }
kin-descendant = { $n ->
        [2] { $sex ->
            [M] حفيد
            [F] حفيدة
           *[other] حفيد أو حفيدة
        }
        [3] { $sex ->
            [M] ابن الحفيد
            [F] ابنة الحفيد
           *[other] ولد الحفيد
        }
        [4] { $sex ->
            [M] حفيد الحفيد
            [F] حفيدة الحفيد
           *[other] حفيد الحفيد
        }
       *[other] خلف من الجيل { $n }
    }
kin-sibling = { $half ->
        [yes] { $sex ->
            [M] { $side ->
                [M] أخ لأب
                [F] أخ لأم
               *[other] أخ غير شقيق
            }
            [F] { $side ->
                [M] أخت لأب
                [F] أخت لأم
               *[other] أخت غير شقيقة
            }
           *[other] أخ أو أخت غير شقيقين
        }
       *[no] { $adoptive ->
            [yes] { $sex ->
                [M] أخ بالتبني
                [F] أخت بالتبني
               *[other] أخ أو أخت بالتبني
            }
           *[no] { $sex ->
                [M] أخ
                [F] أخت
               *[other] أخ أو أخت
            }
        }
    }
kin-uncle = { $n ->
        [1] { $side ->
            [M] { $sex ->
                [M] عم
                [F] عمة
               *[other] عم أو عمة
            }
            [F] { $sex ->
                [M] خال
                [F] خالة
               *[other] خال أو خالة
            }
           *[other] { $sex ->
                [M] عم أو خال
                [F] عمة أو خالة
               *[other] عم أو خال
            }
        }
        [2] { $sex ->
            [M] أخو
            [F] أخت
           *[other] أخو أو أخت
        } { $turna ->
            [M] الجد
            [F] الجدة
           *[other] الجد أو الجدة
        }
        [3] { $sex ->
            [M] أخو الجد الأكبر
            [F] أخت الجد الأكبر
           *[other] أخو الجد الأكبر أو أخته
        }
       *[other] { $sex ->
            [M] أخو سلف
            [F] أخت سلف
           *[other] أخو سلف أو أخته
        }
    }
kin-nephew = { $n ->
        [1] { $sex ->
            [M] ابن
            [F] ابنة
           *[other] ولد
        } { $via ->
            [M] الأخ
            [F] الأخت
           *[other] الأخ أو الأخت
        }
        [2] { $sex ->
            [M] حفيد
            [F] حفيدة
           *[other] حفيد
        } { $via ->
            [M] الأخ
            [F] الأخت
           *[other] الأخ أو الأخت
        }
       *[other] خلف الأخ أو الأخت
    }
kin-cousin = { $k ->
        [1] { $sex ->
            [M] { $side ->
                [M] { $via ->
                    [M] ابن العم
                    [F] ابن العمة
                   *[other] ابن العم أو العمة
                }
                [F] { $via ->
                    [M] ابن الخال
                    [F] ابن الخالة
                   *[other] ابن الخال أو الخالة
                }
               *[other] ابن عمومة أو خؤولة
            }
            [F] { $side ->
                [M] { $via ->
                    [M] بنت العم
                    [F] بنت العمة
                   *[other] بنت العم أو العمة
                }
                [F] { $via ->
                    [M] بنت الخال
                    [F] بنت الخالة
                   *[other] بنت الخال أو الخالة
                }
               *[other] بنت عمومة أو خؤولة
            }
           *[other] ابن عمومة أو خؤولة
        }
       *[other] { $sex ->
            [M] ابن عمومة
            [F] ابنة عمومة
           *[other] قريب
        } من الدرجة { $k }
    }
kin-cousin-removed = { $older ->
        [yes] { $k ->
            [1] { $sex ->
                [M] ابن عمومة
                [F] ابنة عمومة
               *[other] قريب
            } { $forebear }
           *[other] { $sex ->
                [M] ابن عمومة
                [F] ابنة عمومة
               *[other] قريب
            } من الدرجة { $k }، من جيل { $r ->
                [1] الوالدين
                [2] الأجداد
               *[other] أقدم
            }
        }
       *[no] { $k ->
            [1] { $descendant } { $ofcousin }
           *[other] { $sex ->
                [M] ابن عمومة
                [F] ابنة عمومة
               *[other] قريب
            } من الدرجة { $k }، من جيل { $r ->
                [1] الأبناء
                [2] الأحفاد
               *[other] أحدث
            }
        }
    }
kin-of-forebear = { $n ->
        [1] { $sex ->
            [M] الأب
            [F] الأم
           *[other] أحد الوالدين
        }
        [2] { $sex ->
            [M] الجد
            [F] الجدة
           *[other] أحد الأجداد
        }
       *[other] أحد الأسلاف
    }
kin-descendant-word = { $n ->
        [1] { $sex ->
            [M] ابن
            [F] ابنة
           *[other] ولد
        }
        [2] { $sex ->
            [M] حفيد
            [F] حفيدة
           *[other] حفيد
        }
       *[other] خلف
    }
kin-of-cousin = { $sex ->
        [F] ابنة العمومة
       *[other] ابن العمومة
    }
kin-partner = { $ended ->
        [yes] { $married ->
            [yes] { $sex ->
                [M] زوج سابق
                [F] زوجة سابقة
               *[other] زوج سابق
            }
           *[no] { $sex ->
                [M] شريك سابق
                [F] شريكة سابقة
               *[other] شريك سابق
            }
        }
       *[no] { $married ->
            [yes] { $sex ->
                [M] زوج
                [F] زوجة
               *[other] زوج أو زوجة
            }
           *[no] { $sex ->
                [M] شريك
                [F] شريكة
               *[other] شريك
            }
        }
    }
kin-parent-in-law = { $sex ->
        [M] حمو
        [F] حماة
       *[other] أحد الأصهار
    }
kin-child-in-law = { $sex ->
        [M] صهر
        [F] كنّة
       *[other] صهر أو كنّة
    }
kin-sibling-in-law = { $kind ->
        [partner-sibling] { $via ->
            [F] { $sex ->
                [M] أخو الزوجة
                [F] أخت الزوجة
               *[other] أخو الزوجة أو أختها
            }
            [M] { $sex ->
                [M] أخو الزوج
                [F] أخت الزوج
               *[other] أخو الزوج أو أخته
            }
           *[other] { $sex ->
                [M] أخو الشريك
                [F] أخت الشريك
               *[other] أخو الشريك أو أخته
            }
        }
       *[other] { $sex ->
            [M] زوج الأخت
            [F] زوجة الأخ
           *[other] زوج الأخ أو الأخت
        }
    }
kin-step-parent = { $sex ->
        [M] زوج الأم
        [F] زوجة الأب
       *[other] زوج أحد الوالدين
    }
kin-step-child = { $sex ->
        [M] ربيب
        [F] ربيبة
       *[other] ربيب أو ربيبة
    }
kin-note-step-parent = { $sex ->
        [F] زوجة
       *[other] زوج
    } { $next ->
        [M] الأب
        [F] الأم
       *[other] أحد الوالدين
    }
kin-note-step-child = ولد { $via ->
        [M] الزوج
        [F] الزوجة
       *[other] الشريك
    }
kin-foster-parent = { $sex ->
        [M] أب حاضن
        [F] أم حاضنة
       *[other] حاضن
    }
kin-foster-child = { $sex ->
        [M] طفل محتضن
        [F] طفلة محتضنة
       *[other] طفل محتضن
    }
kin-guardian = { $sex ->
        [F] وصية
       *[other] وصي
    }
kin-ward = { $sex ->
        [F] قاصرة تحت الوصاية
       *[other] قاصر تحت الوصاية
    }
kin-relative-of-partner = { $relative } من جهة { $via ->
        [M] الزوج
        [F] الزوجة
       *[other] الشريك
    }
kin-partner-of-relative = { $sex ->
        [M] زوج
        [F] زوجة
       *[other] شريك
    } { $relative }
kin-affine = { $sex ->
        [F] قريبة بالمصاهرة
       *[other] قريب بالمصاهرة
    }
kin-related = { $sex ->
        [F] مرتبطة عبر سجلات العائلة
       *[other] مرتبط عبر سجلات العائلة
    }
kin-note-adoptive = بالتبني
kin-of = بالنسبة إلى { $name }
kin-hop-parent = { $line ->
        [adoption] { $sex ->
            [M] أب بالتبني
            [F] أم بالتبني
           *[other] والد بالتبني
        }
        [foster] { $sex ->
            [M] أب حاضن
            [F] أم حاضنة
           *[other] حاضن
        }
        [guardianship] { $sex ->
            [F] وصية
           *[other] وصي
        }
        [step] { $sex ->
            [M] زوج الأم
            [F] زوجة الأب
           *[other] زوج أحد الوالدين
        }
       *[other] { $sex ->
            [M] أب
            [F] أم
           *[other] والد
        }
    }
kin-hop-child = { $line ->
        [adoption] { $sex ->
            [M] ابن بالتبني
            [F] ابنة بالتبني
           *[other] ولد بالتبني
        }
        [foster] { $sex ->
            [M] طفل محتضن
            [F] طفلة محتضنة
           *[other] طفل محتضن
        }
        [guardianship] { $sex ->
            [F] قاصرة تحت الوصاية
           *[other] قاصر تحت الوصاية
        }
        [step] { $sex ->
            [M] ربيب
            [F] ربيبة
           *[other] ربيب أو ربيبة
        }
       *[other] { $sex ->
            [M] ابن
            [F] ابنة
           *[other] ولد
        }
    }
kin-group-partners = الأزواج
kin-group-ancestors = الأصول
kin-group-descendants = الفروع
kin-group-siblings = الإخوة والأخوات
kin-group-uncles = الأعمام والأخوال
kin-group-nephews = أبناء الإخوة والأخوات
kin-group-cousins = أبناء العمومة والخؤولة
kin-group-in-laws = الأصهار
kin-group-step = الأسرة المركبة
kin-group-care = الحضانة والوصاية
kin-group-rites = الشعائر والأعراف
kin-group-others = روابط أخرى
kin-find-title = ما صلة القرابة؟
kin-find-lede = اختر شخصًا: صلته بـ{ $name } خطوةً خطوة.
kin-find-label = الشخص
kin-find-choose = اختر شخصًا…
kin-find-go = اعرض الصلة
kin-find-none = لا تربط سجلات العائلة بين { $a } و{ $b }.
kin-path = الصلة خطوة بخطوة
kin-degree = قرابة من الدرجة { $n }
kin-all-title = كل أقارب { $name }
kin-all-count = { $n ->
        [zero] لا أحد
        [one] شخص واحد
        [two] شخصان
        [few] { $n } أشخاص
        [many] { $n } شخصًا
       *[other] { $n } شخص
    }
kin-common-list = الأسلاف المشتركون: { $names }
kin-also-list = وأيضًا { $list }
kin-rites-help = العرّابون وروابط أخرى تنشئها التقاليد؛ ترسمها الشجرة بخط منقّط.
kin-rite-recorded = يقول السجل: «{ $label }».
kin-hidden = شخص لا يمكنك رؤيته
kin-native = بكلمات التقليد
kin-legend-rite = عرّاب أو شعيرة أو عرف
kin-tree-root = { $term } { $of }
kin-editor-rite = شعيرة أو عرف
kin-editor-rite-none = لا شيء: علاقة بسيطة
kin-editor-rite-hint = ما يكونه هذا الشخص للآخر. يملأ الاختيار العلاقة والكلمات، ويحتفظ السجل بالشعيرة.
kin-editor-rite-option = { $rite }: { $word }
kin-tradition-christian = مسيحية
kin-tradition-orthodox = مسيحية أرثوذكسية
kin-tradition-hispanic = إسبانية وفلبينية
kin-tradition-jewish = يهودية
kin-tradition-islamic = إسلامية
kin-tradition-south-asian = من جنوب آسيا
kin-tradition-east-asian = من شرق آسيا
kin-tradition-african = أفريقية
kin-tradition-indigenous = الشعوب الأصلية والمحيط الهادئ
kin-tradition-civil = مدنية وعلمانية
kin-rite-godparent = { $role ->
        [sponsored] { $sex ->
            [M] فليون
            [F] فليونة
           *[other] فليون أو فليونة
        }
       *[other] { $sex ->
            [M] عرّاب
            [F] عرّابة
           *[other] عرّاب أو عرّابة
        }
    }
kin-rite-godparent-name = العرابة
kin-rite-godparent-note = عرّاب أو عرّابة كما يذكر السجل.
kin-rite-baptism = { $role ->
        [sponsored] { $sex ->
            [M] فليون
            [F] فليونة
           *[other] فليون أو فليونة
        }
       *[other] { $sex ->
            [M] عرّاب
            [F] عرّابة
           *[other] عرّاب أو عرّابة
        }
    }
kin-rite-baptism-name = المعمودية
kin-rite-baptism-note = عرّاب في المعمودية.
kin-rite-confirmation = { $role ->
        [sponsored] { $sex ->
            [M] مثبَّت
            [F] مثبَّتة
           *[other] مثبَّت
        }
       *[other] { $sex ->
            [M] عرّاب التثبيت
            [F] عرّابة التثبيت
           *[other] عرّاب التثبيت
        }
    }
kin-rite-confirmation-name = التثبيت
kin-rite-confirmation-note = عرّاب في التثبيت أو الميرون.
kin-rite-wedding-sponsor = { $role ->
        [sponsored] { $sex ->
            [M] عريس
            [F] عروس
           *[other] عريس أو عروس
        }
       *[other] { $sex ->
            [M] إشبين
            [F] إشبينة
           *[other] إشبين
        }
    }
kin-rite-wedding-sponsor-name = الإشبينية
kin-rite-wedding-sponsor-note = إشبين الزواج: padrino de boda، ninong، koumbaros.
kin-rite-kum = { $role ->
        [sponsored] { $sex ->
            [M] فليون
            [F] فليونة
           *[other] فليون أو فليونة
        }
       *[other] { $sex ->
            [M] عرّاب
            [F] عرّابة
           *[other] عرّاب أو عرّابة
        }
    }
kin-rite-kum-name = كومستفو
kin-rite-kum-note = عرّاب أو شاهد زواج في التقليد الأرثوذكسي السلافي؛ الكومستفو يربط عائلتين.
kin-rite-koumbaros = { $role ->
        [sponsored] { $sex ->
            [M] فليون
            [F] فليونة
           *[other] فليون أو فليونة
        }
       *[other] { $sex ->
            [M] عرّاب
            [F] عرّابة
           *[other] عرّاب أو عرّابة
        }
    }
kin-rite-koumbaros-name = كومباروس
kin-rite-koumbaros-note = شاهد زواج أو عرّاب في التقليد الأرثوذكسي اليوناني.
kin-rite-padrino = { $role ->
        [sponsored] { $sex ->
            [M] فليون
            [F] فليونة
           *[other] فليون أو فليونة
        }
       *[other] { $sex ->
            [M] عرّاب
            [F] عرّابة
           *[other] عرّاب أو عرّابة
        }
    }
kin-rite-padrino-name = بادرينو
kin-rite-padrino-note = بادرينو أو مادرينا: في المعمودية والمناولة الأولى والكينسينييرا.
kin-rite-compadre = { $sex ->
        [M] شريك في الأبوة الروحية
        [F] شريكة في الأمومة الروحية
       *[other] شريك في الأبوة الروحية
    }
kin-rite-compadre-name = كومبادرازغو
kin-rite-compadre-note = الرابطة بين والدي الطفل وعرّابيه.
kin-rite-ninong = { $role ->
        [sponsored] { $sex ->
            [M] فليون
            [F] فليونة
           *[other] فليون أو فليونة
        }
       *[other] { $sex ->
            [M] عرّاب
            [F] عرّابة
           *[other] عرّاب أو عرّابة
        }
    }
kin-rite-ninong-name = نينونغ
kin-rite-ninong-note = نينونغ أو نينانغ: عرّاب في المعمودية أو التثبيت أو الزواج في الفلبين.
kin-rite-sandek = { $role ->
        [sponsored] { $sex ->
            [M] فليون
            [F] فليونة
           *[other] فليون
        }
       *[other] ساندك
    }
kin-rite-sandek-name = بريت ميلا: ساندك
kin-rite-sandek-note = يحمل الطفل أثناء البريت ميلا.
kin-rite-kvatter = { $role ->
        [sponsored] { $sex ->
            [M] فليون
            [F] فليونة
           *[other] فليون
        }
       *[other] { $sex ->
            [F] كفاترين
           *[other] كفاتر
        }
    }
kin-rite-kvatter-name = بريت ميلا: كفاتر
kin-rite-kvatter-note = يحمل الطفل إلى البريت ميلا.
kin-rite-milk-parent = { $role ->
        [sponsored] { $sex ->
            [M] ابن من الرضاعة
            [F] ابنة من الرضاعة
           *[other] ولد من الرضاعة
        }
       *[other] { $sex ->
            [M] أب من الرضاعة
            [F] أم من الرضاعة
           *[other] والد من الرضاعة
        }
    }
kin-rite-milk-parent-name = القرابة من الرضاعة
kin-rite-milk-parent-note = أرضعت الطفل: في الشريعة الإسلامية تنشئ الرضاعة قرابة وتحرّم الزواج كما تحرّمه الولادة.
kin-rite-milk-sibling = { $sex ->
        [M] أخ من الرضاعة
        [F] أخت من الرضاعة
       *[other] أخ أو أخت من الرضاعة
    }
kin-rite-milk-sibling-name = الإخوة من الرضاعة
kin-rite-milk-sibling-note = أرضعتهما المرأة نفسها.
kin-rite-kafala = { $role ->
        [sponsored] { $sex ->
            [F] مكفولة
           *[other] مكفول
        }
       *[other] { $sex ->
            [F] كافلة
           *[other] كافل
        }
    }
kin-rite-kafala-name = الكفالة
kin-rite-kafala-note = يكفل طفلًا ويربيه دون نسب التبني، وفق الشريعة الإسلامية.
kin-rite-rakhi = { $sex ->
        [M] أخ الراخي
        [F] أخت الراخي
       *[other] أخ أو أخت الراخي
    }
kin-rite-rakhi-name = راخي
kin-rite-rakhi-note = في راكشا بندهن تربط الأخت الراخي على معصم أخيها، فيتعهد بحمايتها.
kin-rite-guru = { $role ->
        [sponsored] { $sex ->
            [M] تلميذ
            [F] تلميذة
           *[other] تلميذ
        }
       *[other] غورو
    }
kin-rite-guru-name = غورو
kin-rite-guru-note = معلّم روحي وتلميذه.
kin-rite-gan-parent = { $role ->
        [sponsored] { $sex ->
            [M] فليون
            [F] فليونة
           *[other] فليون أو فليونة
        }
       *[other] { $sex ->
            [M] عرّاب
            [F] عرّابة
           *[other] عرّاب أو عرّابة
        }
    }
kin-rite-gan-parent-name = غان تشين
kin-rite-gan-parent-note = عرّابون يُختارون في الصين والعائلات الصينية، غالبًا لحماية الطفل.
kin-rite-sworn-sibling = { $sex ->
        [M] أخ بالقسم
        [F] أخت بالقسم
       *[other] أخ أو أخت بالقسم
    }
kin-rite-sworn-sibling-name = الأخوّة بالقسم
kin-rite-sworn-sibling-note = إخوة أو أخوات بالقسم (جيباي).
kin-rite-name-giver = { $role ->
        [sponsored] { $sex ->
            [M] الطفل المُسمّى
            [F] الطفلة المُسمّاة
           *[other] الطفل المُسمّى
        }
       *[other] { $sex ->
            [F] مُسمّية
           *[other] مُسمّي
        }
    }
kin-rite-name-giver-name = التسمية
kin-rite-name-giver-note = أعطى الطفل اسمه (ناتسوكي-أويا في اليابان).
kin-rite-nakodo = { $role ->
        [sponsored] { $sex ->
            [M] عريس
            [F] عروس
           *[other] عريس أو عروس
        }
       *[other] { $sex ->
            [F] خاطبة
           *[other] خاطب
        }
    }
kin-rite-nakodo-name = ناكودو
kin-rite-nakodo-note = رتّب الزواج وشهد عليه (ناكودو في اليابان).
kin-rite-namesake = { $sex ->
        [F] سميّة
       *[other] سميّ
    }
kin-rite-namesake-name = السميّ
kin-rite-namesake-note = يحمل الاسم نفسه: في وسط أفريقيا وشرقها رابطة هدايا ورعاية (ndoyi، somo).
kin-rite-initiation = { $role ->
        [sponsored] { $sex ->
            [F] مُكرَّسة
           *[other] مُكرَّس
        }
       *[other] { $sex ->
            [F] مرشدة التكريس
           *[other] مرشد التكريس
        }
    }
kin-rite-initiation-name = التكريس
kin-rite-initiation-note = رافق الشخص في طقس تكريس.
kin-rite-blood-pact = { $sex ->
        [M] أخ بالدم
        [F] أخت بالدم
       *[other] أخ أو أخت بالدم
    }
kin-rite-blood-pact-name = ميثاق الدم
kin-rite-blood-pact-note = يجمعهما ميثاق دم.
kin-rite-hanai = { $role ->
        [sponsored] { $sex ->
            [M] ابن هاناي
            [F] ابنة هاناي
           *[other] ولد هاناي
        }
       *[other] { $sex ->
            [M] أب هاناي
            [F] أم هاناي
           *[other] والد هاناي
        }
    }
kin-rite-hanai-name = هاناي
kin-rite-hanai-note = رُبّي كابن لهم بحسب العرف الهاوايي، دون تبنٍّ قانوني.
kin-rite-hunka = { $role ->
        [sponsored] { $sex ->
            [M] ابن هونكا
            [F] ابنة هونكا
           *[other] ولد هونكا
        }
       *[other] { $sex ->
            [M] أب هونكا
            [F] أم هونكا
           *[other] والد هونكا
        }
    }
kin-rite-hunka-name = هونكا
kin-rite-hunka-note = صار قريبًا في طقس الهونكا لدى اللاكوتا.
kin-rite-civil-sponsor = { $role ->
        [sponsored] { $sex ->
            [M] فليون مدني
            [F] فليونة مدنية
           *[other] فليون مدني
        }
       *[other] { $sex ->
            [M] عرّاب مدني
            [F] عرّابة مدنية
           *[other] عرّاب مدني
        }
    }
kin-rite-civil-sponsor-name = العرابة المدنية
kin-rite-civil-sponsor-note = عرّاب في مراسم تسمية مدنية.
kin-rite-guardian = { $role ->
        [sponsored] { $sex ->
            [F] قاصرة تحت الوصاية
           *[other] قاصر تحت الوصاية
        }
       *[other] { $sex ->
            [F] وصية
           *[other] وصي
        }
    }
kin-rite-guardian-name = الوصاية
kin-rite-guardian-note = وصي كما يذكر السجل.

## Polygamous unions (docs/adr/0021): one marriage per spouse, the
## co-spouses, the editor's two fields.
kin-co-spouse = { $sex ->
        [M] زوج آخر
        [F] ضَرّة
       *[other] شريك آخر في الزواج
    }
union-co-spouses = { $sex ->
        [F] { $n ->
            [zero] الضرائر
            [one] الضَّرّة
            [two] الضَّرّتان
            [few] الضرائر
            [many] الضرائر
           *[other] الضرائر
        }
        [M] { $n ->
            [zero] الأزواج الآخرون
            [one] الزوج الآخر
            [two] الزوجان الآخران
            [few] الأزواج الآخرون
            [many] الأزواج الآخرون
           *[other] الأزواج الآخرون
        }
       *[other] { $n ->
            [zero] الشركاء الآخرون في الزواج
            [one] الشريك الآخر في الزواج
            [two] الشريكان الآخران في الزواج
            [few] الشركاء الآخرون في الزواج
            [many] الشركاء الآخرون في الزواج
           *[other] الشركاء الآخرون في الزواج
        }
    }
tree-key-polygamous = زواج متعدد
family-primary = في مركز الزواج
family-primary-hint = في الزواج المتعدد، الشخص المتزوج من كل واحد من الآخرين. والآخرون ليسوا متزوجين فيما بينهم.
family-child-parent = الوالد الآخر
family-child-parent-hint = في الزواج المتعدد، الزوج الذي هو الوالد الآخر لهذا الطفل. إن لم يُختر، فالسجل لا يذكره.

## Emblems (docs/adr/0022): the picture that stands for a family, and for
## the archive's.
emblem-family-edit = شعار أو شارة نبالة
emblem-family-title = شعار العائلة
emblem-archive-title = شعار عائلة الأرشيف
emblem-archive-edit = شعار العائلة أو شارة نبالتها
emblem-saved = حُفظ الشعار.
emblem-choose = الصورة التي تمثل العائلة
emblem-help = شارة نبالة أو علامة أو صورة: ما يمثل هذه العائلة. تُعرض كاملة بجانب الزواج في كل سجل وعلى عقدة الزوجين في الشجرة.
emblem-archive-help = شارة نبالة أو علامة أو صورة: ما يمثل العائلة التي يتحدث عنها هذا الأرشيف. تُعرض كاملة أعلى كل صفحة وفي النظرة العامة، لكل الزوار.
emblem-mode-auto-note = الصورة التي تربطها هذه العائلة شعارًا لها، إن وُجدت.
emblem-mode-none = بلا شعار
emblem-no-images = لا صورة بعد: أضف واحدة أدناه.
emblem-upload-title = إضافة صورة
emblem-upload-button = إضافتها واستخدامها
emblem-not-an-image = الشعار صورة: PNG أو JPEG أو GIF أو WebP أو BMP.
emblem-back = العودة إلى الشعار
emblem-no-family-title = لا توجد هذه العائلة
emblem-no-family-detail = هذه العائلة ليست في الأرشيف، أو لا يُسمح لك برؤيتها.

## The Artificial family genealogist, AFG (docs/adr/0023): its room, where
## its computing comes from, and a test.
admin-nav-afg = النسّابة الذكي
afg-title = النسّابة الاصطناعي للعائلة
afg-lede = من أين يأتي حساب النسّابة الاصطناعي، واختبار.
afg-kpi-source = الحوسبة
afg-ready = جاهز
afg-off-note = اختر مصدر الحوسبة.
afg-not-ready = غير مكتمل
afg-kpi-sent = الملفات المرسلة
afg-kpi-sent-note = { $kb } كيلوبايت، ونحو { $tokens } من الرموز
afg-kpi-last = آخر اختبار
afg-never = لا شيء بعد
afg-run-open = فتحه
afg-source = من أين تأتي الحوسبة
afg-source-intro = عنوان؛ أو عنوان بمفتاح ونموذج؛ أو وكيل مسجّل الدخول على هذا الجهاز.
afg-kind = طريقة الحوسبة
afg-kind-hint = تُبلغ الخدمة عبر الشبكة؛ أما الوكيل فيعمل على هذا الجهاز ويجب أن يكون مسجّل الدخول فيه.
afg-kind-off = لا شيء
afg-kind-anthropic = Claude (واجهة Anthropic البرمجية)
afg-kind-ollama = Ollama
afg-kind-openai = OpenAI
afg-kind-mistral = Mistral
afg-kind-vllm = vLLM
afg-kind-gemini = Gemini
afg-kind-claude-code = Claude Code
afg-kind-codex = Codex
afg-kind-opencode = OpenCode
afg-group-address = عنوان
afg-group-service = عنوان ومفتاح API ونموذج
afg-group-agent = وكيل برمجة على هذا الجهاز
afg-url = العنوان
afg-url-hint = إذا تُرك فارغًا: العنوان المعتاد للخدمة. الاستدعاء:
afg-key = مفتاح API
afg-key-set = مفتاح محفوظ ينتهي بـ { $end }.
afg-key-unset = لا يوجد مفتاح محفوظ.
afg-key-hint = الفراغ يُبقي المفتاح المحفوظ، ولن يُعرض مجددًا.
afg-key-clear = نسيان المفتاح
afg-model = النموذج
afg-model-hint = اسم النموذج كما تكتبه الخدمة. يُرسل أي اسم كما هو؛ والفراغ يعني الافتراضي.
afg-model-unlisted = ليس من الاقتراحات: يُرسَل كما كُتب.
afg-effort = الجهد
afg-effort-hint = مدى عمق تفكير النموذج. تُرسل أي كلمة كما هي؛ والفراغ يعني إعداد النموذج.
afg-effort-unlisted = ليس من جهود هذا النموذج: يُرسَل كما كُتب.
afg-effort-default = افتراضي
afg-program = البرنامج
afg-program-found = وُجد:
afg-program-missing = غير موجود على هذا الجهاز.
afg-program-hint = فارغ: يُبحث عنه حيث يضعه مثبّته. وإلا فالمسار الكامل للبرنامج.
afg-sensitive = إرسال الفئات الحساسة أيضًا
afg-sensitive-hint = دون تحديد، تبقى الصحة والقياسات الحيوية والوراثة والسجلات القضائية والسلوك خارج الإرسال، كما في تصدير مشترك.
afg-timeout = الوقت المسموح (بالثواني)
afg-timeout-hint = المدة التي قد يستغرقها الجواب قبل التخلي عنه.
afg-test = اختبار
afg-test-intro = يرسل الأرشيف (JSON، دون صور) وذاكرتك مع سؤال واحد؛ ويُفتح الجواب في نافذة.
afg-sent-files = الملفات
afg-sent-files-value = { $n ->
        [zero] لا ملفات بصيغة JSON
        [one] ملف واحد بصيغة JSON
        [two] ملفان بصيغة JSON
        [few] { $n } ملفات بصيغة JSON
        [many] { $n } ملفًا بصيغة JSON
       *[other] { $n } ملف بصيغة JSON
    }، { $kb } كيلوبايت
afg-sent-tokens = الرموز، تقريبًا
afg-sent-records = السجلات
afg-sent-records-value = { $persons ->
        [zero] لا أشخاص
        [one] شخص واحد
        [two] شخصان
        [few] { $persons } أشخاص
        [many] { $persons } شخصًا
       *[other] { $persons } شخص
    }، { $families ->
        [zero] لا أسر
        [one] أسرة واحدة
        [two] أسرتان
        [few] { $families } أسر
        [many] { $families } أسرة
       *[other] { $families } أسرة
    }
afg-sent-classes = الفئات الحساسة
afg-sent-classes-in = تُرسَل
afg-sent-classes-out = مستبعدة
afg-sent-question = السؤال
afg-privacy = يرسل الاختبار سجلات العائلة، بما فيها الأحياء، إلى الحساب المختار أعلاه.
afg-test-button = إرسال الاختبار
afg-test-needs = احفظ أولًا طريقة حوسبة كاملة.
afg-test-question = أعطني أكثر ثلاث عائلات من حيث عدد الأفراد المسجَّلين.
afg-test-empty = الحزمة فارغة
afg-saved = تم الحفظ.
afg-error-not-saved = لم يُحفظ: { $error }
afg-error-off = لم تُختر أي طريقة حوسبة.
afg-error-archive = تعذّرت قراءة الأرشيف: { $error }
afg-error-url = يجب أن يبدأ العنوان بـ http:// أو https://.
afg-error-key = تحتاج هذه الخدمة إلى مفتاح API.
afg-error-model = يلزم نموذج هنا، مكتوب بحروف وأرقام و . _ : / @ [ ] + -
afg-error-effort = يُكتب الجهد بحروف وأرقام و . _ : / @ [ ] + -
afg-error-program = يجب أن يكون البرنامج برنامج الوكيل نفسه (claude أو codex أو opencode)، بمساره الكامل.
afg-run-state-running = بانتظار الجواب
afg-run-state-done = أُجيب
afg-run-state-failed = فشل
afg-run-seconds = { $s } ث
afg-run-usage = الرموز المقروءة
afg-run-usage-out = { $n } مكتوبة
afg-run-cost = { $usd } دولار أمريكي
afg-run-waiting = أُرسل الأرشيف؛ وسيظهر الجواب هنا. تعيد هذه الصفحة السؤال كل ثلاث ثوانٍ.
afg-run-model = النموذج:
afg-run-failed-hint = تحقّق من طريقة الحوسبة، ومن الشبكة التي يمكن لهذا الموقع بلوغها، وبالنسبة إلى الوكيل، من أنه مثبَّت ومسجَّل الدخول تحت الحساب الذي يعمل به هذا الموقع.
afg-run-gone-detail = تُحفظ الاختبارات طوال عمر العملية، آخر عشرة منها. أرسل اختبارًا جديدًا.

## The AFG's brains, its window and who may use it (docs/adr/0024).
afg-kpi-brain = ذاكرتك
afg-brain-exchanges = { $n ->
        [zero] لا تبادلات
        [one] تبادل واحد
        [two] تبادلان
        [few] { $n } تبادلات
        [many] { $n } تبادلًا
       *[other] { $n } تبادل
    }
afg-brain-note = { $facts ->
        [zero] لا حقائق محفوظة
        [one] حقيقة واحدة محفوظة
        [two] حقيقتان محفوظتان
        [few] { $facts } حقائق محفوظة
        [many] { $facts } حقيقةً محفوظة
       *[other] { $facts } حقيقة محفوظة
    } · { $brains ->
        [zero] لا حسابات بذاكرة
        [one] حساب واحد بذاكرة
        [two] حسابان بذاكرة
        [few] { $brains } حسابات بذاكرة
        [many] { $brains } حسابًا بذاكرة
       *[other] { $brains } حساب بذاكرة
    }
afg-brain-no-account = لا حساب لهذه الجلسة، فلا ذاكرة لها.
afg-access-count = الحسابات التي يمكنها محادثة النسّابة الاصطناعي: { $n }
afg-access-set = يُضبط في صفحة الحسابات
afg-result-ok = نجاح
afg-result-error = خطأ
afg-result-close = إغلاق
afg-result-remembered = حُفظ في ذاكرة { $name }:
afg-result-nothing-kept = لم يُحفظ شيء جديد في ذاكرة { $name }.
accounts-afg = النسّابة الذكي
accounts-afg-allowed = يمكنه التحدث إلى النسّابة الذكي
accounts-afg-yes = نعم
accounts-afg-no = لا
accounts-afg-hint = معطّل ما لم يُحدَّد. تُحفظ ذاكرته منفصلة، بجانب الأرشيف.

## An account's AFG brain on its own page: seen and forgotten (docs/adr/0024 §6).
account-brain-title = ذاكرة النسّابة الذكي الخاصة بك
account-brain-intro = ما يتذكره النسّابة الاصطناعي عنك، ويُعاد إليه مع كل سؤال. احذف ما تشاء؛ لا يراه غيرك.
account-brain-no-access = النسّابة الذكي غير متاح لحسابك: يقرر ذلك مسؤول في صفحة الحسابات.
account-brain-none = لا يتذكر النسّابة الذكي شيئًا عنك بعد.
account-brain-facts = ما يتذكره
account-brain-facts-none = لم يحتفظ بأي حقيقة عنك.
account-brain-exchanges = محادثاتك معه
account-brain-exchanges-none = لا توجد محادثات محفوظة.
account-brain-older = { $n ->
        [zero] لا محادثات أقدم محفوظة.
        [one] محادثة أقدم واحدة محفوظة وغير مدرجة هنا.
        [two] محادثتان أقدم محفوظتان وغير مدرجتين هنا.
        [few] { $n } محادثات أقدم محفوظة وغير مدرجة هنا.
        [many] { $n } محادثةً أقدم محفوظة وغير مدرجة هنا.
       *[other] { $n } محادثة أقدم محفوظة وغير مدرجة هنا.
    }
account-brain-context-test = اختبار في قاعة النسّابة الذكي
account-brain-answer = جوابه
account-brain-forget = نسيان
account-brain-forget-all = نسيان كل شيء…
account-brain-forget-all-confirm = نعم، انسَ كل ما يتذكره النسّابة الذكي عني
account-brain-forget-all-button = نسيان كل شيء
account-brain-forgotten = نُسي: لن يُخبَر النسّابة الذكي به بعد الآن.
account-brain-forgotten-all = لم يعد النسّابة الذكي يتذكر شيئًا عنك.
account-brain-already = كان ذلك منسيًّا من قبل.
account-brain-confirm-needed = حدّد المربع لنسيان كل شيء.
account-brain-error = لم يُنسَ شيء: { $error }
afg-brain-see = عرض ونسيان

## My personal assistant: projects, conversations, code of conduct, second brain (docs/adr/0026).
assistant-title = مساعدي الشخصي
assistant-entry = مشاريعك ومحادثاتك مع النسّابة الاصطناعي.
assistant-lede = مشاريعك مع النسّابة الاصطناعي، لكل منها مدونة سلوك وذاكرة ومحادثات.
assistant-shut-no-account = لا حساب لهذه الجلسة، فلا مساعد شخصي لها. سجّل الدخول بحسابك الخاص.
assistant-shut-not-open = النسّابة الاصطناعي غير متاح لحسابك: يقرر ذلك مسؤول في صفحة الحسابات.
assistant-shut-broken = تعذّر فتح مساحتك. أعد المحاولة بعد قليل؛ وإن استمر ذلك فأخبر مسؤولًا.
assistant-shut-gone = لا شيء هنا: هذا المشروع أو هذه المحادثة غير موجودة، أو ليست لك.
assistant-back-account = العودة إلى حسابي
assistant-new-project = مشروع جديد
assistant-not-ready = حساب النسّابة الاصطناعي لم يُضبط بعد: القراءة فقط حاليًا.
assistant-conversations-count = { $n ->
        [zero] لا محادثات
        [one] محادثة واحدة
        [two] محادثتان
        [few] { $n } محادثات
        [many] { $n } محادثة
       *[other] { $n } محادثة
    }
assistant-conduct-set = مدونة السلوك محددة
assistant-empty = لا مشروع بعد. يجمع المشروع المحادثات حول موضوع واحد.
assistant-project-name = اسم المشروع
assistant-create = إنشاء المشروع
assistant-facts-count = { $n ->
        [zero] لا حقائق محفوظة
        [one] حقيقة واحدة محفوظة
        [two] حقيقتان محفوظتان
        [few] { $n } حقائق محفوظة
        [many] { $n } حقيقة محفوظة
       *[other] { $n } حقيقة محفوظة
    }
assistant-archive-on = مع أرشيف العائلة
assistant-start = بدء محادثة
assistant-conversations = المحادثات
assistant-no-conversation = لا محادثات بعد: سؤالك الأول يبدأ واحدة.
assistant-conduct = مدونة السلوك
assistant-conduct-label = كيف ينبغي أن يتصرف النسّابة الاصطناعي في هذا المشروع
assistant-conduct-hint = تعليماتك لكل محادثات هذا المشروع: النبرة، واللغة، وما يُنظر إليه، وما يُترك جانبًا. يتبعها النسّابة الاصطناعي ما لم تطلب منه خداعك أو إلحاق الأذى.
assistant-save = حفظ
assistant-memory = ذاكرة المشروع
assistant-memory-intro = ما اختار النسّابة الاصطناعي حفظه من محادثات هذا المشروع: دماغه الثاني. تُعاد إليه مع كل رسالة في المشروع. اجعله ينسى ما لم تعد تريده أن يعرفه.
assistant-forget = نسيان
assistant-forget-all = نسيان كل شيء…
assistant-forget-all-confirm = نعم، انسَ كل ما يتذكره النسّابة الاصطناعي عن هذا المشروع
assistant-forget-all-button = نسيان كل شيء
assistant-memory-none = لا يتذكر النسّابة الاصطناعي شيئًا عن هذا المشروع بعد.
assistant-settings = الإعدادات
assistant-archive = إعطاء النسّابة الاصطناعي أرشيف العائلة في هذا المشروع
assistant-archive-hint = عندها يذهب كل سجل في الأرشيف، الأحياء منهم أيضًا، مع كل رسالة إلى حيث يحسب النسّابة الاصطناعي. تبقى الفئات الحساسة خارجًا ما لم يقرر مسؤول غير ذلك.
assistant-archive-size = ملفات JSON: { $files }؛ الرموز لكل رسالة: نحو { $tokens }.
assistant-avatar-upload = صورة خاصة بك صورةً رمزية
assistant-avatar-send = استخدام هذه الصورة
assistant-delete-project = حذف المشروع
assistant-delete-project-confirm = نعم، احذف هذا المشروع ومحادثاته وملفاتها وذاكرته
assistant-delete = حذف
assistant-conversation-settings = إعدادات المحادثة
assistant-conversation-title = عنوان المحادثة
assistant-delete-conversation = حذف المحادثة
assistant-delete-conversation-confirm = نعم، احذف هذه المحادثة ورسائلها
assistant-thread = المحادثة
assistant-reply = رسالتك
assistant-sending = جارٍ الإرسال…
assistant-attach = إرفاق
assistant-send = إرسال
assistant-composer-hint = ‏Enter يرسل، وShift+Enter يبدأ سطرًا جديدًا. المستندات والصور والصوت والفيديو: أفلتها أو الصقها أو أرفقها؛ عدد الملفات لا يتجاوز: { $n }، وحجم كل ملف حتى { $mb } MB.
assistant-avatar-choose = اختيار الصورة الرمزية
assistant-avatar = الصورة الرمزية
assistant-avatar-auto = مختارة له
assistant-avatar-own = صورتك
assistant-kept = { $n ->
        [zero] لم يُحفظ شيء في ذاكرة المشروع
        [one] حُفظت في ذاكرة المشروع حقيقة واحدة
        [two] حُفظت في ذاكرة المشروع حقيقتان
        [few] حُفظت في ذاكرة المشروع { $n } حقائق
        [many] حُفظت في ذاكرة المشروع { $n } حقيقة
       *[other] حُفظت في ذاكرة المشروع { $n } حقيقة
    }
assistant-error-answer = تعذّر على النسّابة الاصطناعي أن يجيب.
assistant-retry = السؤال مجددًا
assistant-unanswered = بقيت هذه الرسالة بلا جواب.
assistant-writing = النسّابة الاصطناعي يكتب
assistant-saved = تم الحفظ.
assistant-project-deleted = حُذف المشروع.
assistant-conversation-deleted = حُذفت المحادثة.
assistant-memory-forgotten = نُسي: لن يتلقاه النسّابة الاصطناعي بعد الآن.
assistant-memory-forgotten-all = لم يعد النسّابة الاصطناعي يتذكر شيئًا عن هذا المشروع.
assistant-avatar-saved = حُفظت الصورة الرمزية.
assistant-error-name = يلزم اسم.
assistant-error-too-many = هناك بالفعل العدد الذي يمكن لحساب أن يحتفظ به.
assistant-error-not-saved = لم يُحفظ: لم يتمكن الخادم من كتابته. أعد المحاولة بعد قليل.
assistant-error-conduct-long = مدونة السلوك طويلة جدًا.
assistant-error-confirm = ضع علامة في المربع للتأكيد.
assistant-error-too-big = الرسالة كبيرة جدًا.
assistant-error-file-big = ملف كبير جدًا.
assistant-error-too-many-files = ملفات كثيرة جدًا في رسالة واحدة.
assistant-error-text-long = الرسالة طويلة جدًا.
assistant-error-empty = اكتب رسالة أو أرفق ملفًا.
assistant-error-busy = يجيب النسّابة الاصطناعي بالفعل في أكبر عدد يستطيعه من المحادثات. أعد المحاولة بعد قليل.
assistant-error-wait = لا يزال النسّابة الاصطناعي يجيب في هذه المحادثة.
assistant-error-picture = هذا الملف ليس صورة يستطيع هذا الموقع قراءتها.
assistant-domain-genealogy = علم الأنساب
assistant-domain-medicine = الصحة والطب
assistant-domain-work = العمل والمسيرة المهنية
assistant-domain-money = المال
assistant-domain-law = القانون والأوراق
assistant-domain-learning = التعلّم
assistant-domain-home = المنزل
assistant-domain-cooking = الطبخ
assistant-domain-travel = السفر
assistant-domain-nature = الطبيعة والحديقة
assistant-domain-wellbeing = الرفاه
assistant-domain-family = الحياة العائلية
assistant-domain-arts = الفنون والثقافة
assistant-domain-sports = الرياضة
assistant-domain-tech = التقنية
assistant-domain-science = العلوم
assistant-domain-history = التاريخ والذاكرة
assistant-domain-admin = المعاملات
assistant-domain-pets = الحيوانات
assistant-domain-rites = الأعياد والطقوس

## The AFG on a person's record: a face or a photograph restored, a document read, the summary (docs/adr/0029).

summary-title = الملخّص
summary-write = كتابة الملخّص
summary-edit = تعديل الملخّص
summary-language = اللغة
summary-by-ai = كتبه الذكاء الاصطناعي ({ $generator }): يُرجى مراجعته.
summary-note-title = الملخّص — { $name }
summary-category = ملخّص
summary-edit-title = ملخّص الشخص
afg-summary = الكتابة بالذكاء الاصطناعي
afg-summary-busy = جارٍ الكتابة…
afg-retouch = تنقيح بالذكاء الاصطناعي
afg-retouch-busy = جارٍ التنقيح…
afg-retouched = رمّمها ذكاء اصطناعي: ليست صورة من ذلك الزمن.
afg-read = قراءة المستند
afg-read-busy = جارٍ القراءة…
afg-read-note-title = نسخ نصّي — { $name }
afg-read-category = نسخ نصّي
afg-rework = إعادة معالجة الصورة بالذكاء الاصطناعي
afg-rework-busy = جارٍ المعالجة…
afg-images-off-admin = لا توجد خدمة صور بعد: اخترها في الإدارة › النسّابة الذكي › الصور.
notes-manage-in-media = عدّلها أو احذفها في تبويب الوسائط
notes-delete-short = حذف
notes-none-yet = لا ملاحظات عن هذا الشخص بعد.
afg-record-title = الذكاء الاصطناعي في السجل
afg-record-back = العودة إلى السجل
afg-record-no-access = لم يُفتح النسّابة الذكي لحسابك. يمكن لمسؤول أن يفتحه من صفحة الحسابات.
afg-record-images-off = لم تُضبط أي خدمة صور. يختارها مسؤول في الإدارة › النسّابة الذكي › الصور.
afg-record-text-off = لم يُضبط مصدر حساب النسّابة الذكي. يختاره مسؤول في الإدارة › النسّابة الذكي.
afg-record-no-face = ليس لهذا الشخص صورة يمكن معالجتها.
afg-record-no-document = هذا المستند ليس بين ملفات هذا الشخص، أو ملفه غير محفوظ هنا، أو هو أصلًا نسخة صنعها ذكاء اصطناعي.
afg-record-not-a-picture = لا يمكن تنقيح سوى صورة.
afg-record-not-readable = لا يمكن قراءة سوى صورة أو ملف PDF.
afg-record-pdf-unread = مصدر حساب النسّابة الذكي الحالي لا يقرأ ملفات PDF. تقرؤها OpenAI وClaude وGemini وClaude Code وOpenCode.
afg-record-failed = لم ينجح الذكاء الاصطناعي: { $reason }
afg-record-bad-picture = لم تُرجع الخدمة صورة يمكن لهذا الموقع حفظها.
afg-record-no-text = لم يجد الذكاء الاصطناعي أي نص في هذا المستند.
afg-record-stale = تغيّر السجل في الأثناء. أعد تحميل الصفحة وحاول مجددًا.
afg-record-refused = رفضه الأرشيف: { $reason }
afg-record-summary-not-yours = كتب هذا الملخّص عضو آخر: لا يستبدله إلا كاتبه أو مسؤول.
afg-images = الصور
afg-images-intro = من أين تأتي صور النسّابة الاصطناعي: خدمة مدفوعة (OpenAI أو Google أو Canva) أو جهاز للعائلة ببطاقة رسوميات.
afg-image-kind = خدمة الصور
afg-image-kind-hint = نماذج OpenAI وGoogle، أو ذكاء Canva الاصطناعي عبر حسابك، أو خادم للعائلة: Ollama، أو أي خادم متوافق مع OpenAI، أو ComfyUI.
afg-image-kind-off = لا شيء
afg-image-kind-openai = OpenAI (GPT Image)
afg-image-kind-gemini = Google Gemini (Nano Banana)
afg-image-url-hint = فارغ: العنوان المعتاد للخدمة. لخادم خاص بك: عنوانه (مثلًا http://192.168.1.20:8000/v1).
afg-image-key-hint = تغيير الخدمة يُنسي المفتاح: أدخل مفتاح الخدمة الجديدة معه.
afg-image-model-hint = فارغ: أول نموذج مقترح؛ ويُرسل أي نموذج آخر كما كُتب.
afg-images-privacy = تُرسل الصورة إلى هذه الخدمة كلما طلب عضو ذلك. اختر خدمة تأتمنها عائلتك على صورها.
afg-images-saved = حُفظت خدمة الصور.
afg-images-ready = الصور جاهزة: يمكن لأزرار السجل أن تطلبها.
afg-images-not-ready = لا توجد خدمة صور: أزرار الصور في السجل معطّلة.
afg-images-choose = اختيار خدمة الصور

## Canva and the family's own servers as the AFG's picture service (docs/adr/0030).

afg-image-kind-canva = Canva (ذكاؤه الاصطناعي، عبر حسابك)
afg-image-kind-ollama = Ollama (تجريبي)
afg-image-kind-openai-compatible = خادم خاص بك (متوافق مع OpenAI)
afg-image-kind-comfyui = ComfyUI (سير عملك)
afg-image-workflow = سير عمل ComfyUI (بصيغة API)
afg-image-workflow-hint = مُصدَّر من ComfyUI بصيغة API («Save (API Format)»). توضع الصورة في كل عقدة LoadImage، وتصبح هذه العلامة في أي نص تعليمةَ الترميم:
afg-image-key-optional-hint = اختياري: فقط إذا طلبه خادمك.
afg-images-at-home = تبقى الصورة داخل شبكتك: تعمل هذه الخدمة على جهاز للعائلة.
afg-images-saved-canva = اختير Canva: صِل الآن حسابًا.
afg-canva-connect = ربط Canva
afg-canva-connected = متصل بحساب Canva { $account }.
afg-canva-connected-unnamed = حساب Canva متصل.
afg-canva-not-connected = لا يوجد حساب Canva متصل.
afg-canva-disconnect = فصل Canva
afg-canva-disconnect-confirm = فصل حساب Canva هذا؟ تبقى الصور المصنوعة سابقًا في الأرشيف.
afg-canva-step-open = 1. افتح Canva ووافق
afg-canva-step-paste = 2. الصق هنا عنوان الصفحة التي أعادك إليها Canva (http://localhost…)
afg-canva-paste-hint = لا تُعرض تلك الصفحة: هذا متوقع. يكفي عنوانها، ولا يصلح إلا مرة واحدة.
afg-canva-finish = إتمام الربط
afg-canva-finish-wait = يستغرق الربط ما يصل إلى دقيقة: فهو يجهّز أيضًا في Canva الصفحة التي تُنقَّح عليها كل صورة.
afg-canva-done-ready = Canva متصل، وصفحة عمله جاهزة في المجلد «ax-genealogy». قد يوقف Canva بعد ذلك أرصدة الذكاء الاصطناعي بضع دقائق: إذا رُفض التنقيح الأول فأعد المحاولة بعد قليل.
afg-canva-cooldown = أوقف Canva أرصدة الذكاء الاصطناعي لهذا الحساب بضع دقائق. أعد المحاولة بعد قليل.
afg-canva-done = Canva متصل.
afg-canva-gone = Canva مفصول.
afg-canva-save-first = اختر Canva أولًا خدمةً للصور، ثم احفظ.
afg-canva-policy = ربط واحد يخدم العائلة كلها: تمر عمليات التنقيح عبر حساب Canva هذا وأرصدته ومجلده «ax-genealogy». تنص سياسة استخدام Canva على أن يربط كل شخص حسابه الخاص؛ وبربط حسابك للنسّابة الذكي تتحمل أنت هذه المسؤولية.
afg-error-workflow = سير عمل ComfyUI مفقود، أو ليس بصيغة API (يحتاج إلى عقدة LoadImage واحدة على الأقل).
afg-error-canva = Canva غير متصل: استخدم «ربط Canva».

## Impact: the Universal Synoptic Timeline (docs/adr/0034)

nav-impact = الأثر
impact-title = الأثر
impact-lede = حياة العائلة على امتداد تاريخ العالم.
impact-span-all = التاريخ كله
impact-span-family = قرون العائلة
impact-placed = الأشخاص الموضوعون: { $n }
impact-undated = بلا تاريخ: { $n }
impact-hidden = غير معروضين: { $n }
impact-empty = لا يُعرف أي تاريخ بعد: لا يمكن وضع أحد.
impact-col-world = العالم
impact-col-continents = القارات
impact-col-family = العائلة
impact-col-life = الحياة
impact-col-country = البلد
impact-continent-europe = أوروبا
impact-continent-africa = أفريقيا
impact-continent-asia = آسيا
impact-continent-americas = الأمريكتان
impact-continent-oceania = أوقيانوسيا
impact-kind-era = حضارات وعصور
impact-kind-war = حروب
impact-kind-epidemic = أوبئة
impact-kind-migration = هجرات
impact-kind-event = محطات
impact-kind-economy = اقتصاد ومجتمع
impact-kind-disaster = مجاعات وكوارث
impact-kind-invention = اختراعات كبرى
impact-kind-discovery = اكتشافات كبرى
impact-person-title = زمن { $name }
impact-back = العائلة كلها
impact-record = فتح السجل
impact-states = الدول التي تبعتها الأماكن
impact-countries-none = لم يُعثر على أي بلد في الأماكن.
impact-unplaced = لا يُعرف أي تاريخ: لا يمكن وضع هذا الشخص على الخط الزمني.
impact-see = رؤية الأثر
impact-year-bc = { $year } ق.م
impact-now = اليوم
impact-years = { $n ->
        [zero] { $n } سنة
        [one] سنة واحدة
        [two] سنتان
        [few] { $n } سنوات
        [many] { $n } سنة
       *[other] { $n } سنة
    }
impact-months = { $n ->
        [zero] { $n } شهر
        [one] شهر واحد
        [two] شهران
        [few] { $n } أشهر
        [many] { $n } شهرًا
       *[other] { $n } شهر
    }
impact-days = { $n ->
        [zero] { $n } يوم
        [one] يوم واحد
        [two] يومان
        [few] { $n } أيام
        [many] { $n } يومًا
       *[other] { $n } يوم
    }
impact-age = في سن { $n }
impact-under-way = كان جاريًا عند الولادة
impact-milestone-birth = الولادة
impact-milestone-death = الوفاة
impact-milestone-marriage = الزواج
impact-milestone-child = ولادة { $name }
impact-context = السياق
impact-stakes = ما كان على المحك
impact-victims = الضحايا
impact-close = إغلاق
impact-hint = مرّر المؤشر على حدث لترى سنواته، وانقر لمعرفة سياقه.

# A person created with what surrounds them (docs/adr/0039), and the
# pickers' refusals by what they pick.
person-new-title = شخص جديد
person-new-lede = الاسم الذي لا يعرفه الأرشيف بعد يصبح سجلًا جديدًا يُحفظ مع هذا السجل.
person-new-section-name = الاسم
person-new-given = الاسم الأول
person-new-family = اسم العائلة
person-new-nicknames = الألقاب
person-new-nickname-add = أضف لقبًا
person-new-nickname-quoted = «{ $nick }»
person-new-remove = أزل
person-new-preview = يظهر هكذا
person-new-name-hint = اسم أول أو اسم عائلة على الأقل.
person-new-section-life = الحياة
person-new-section-texts = السيرة والملاحظة
person-new-birth-place = مكان الولادة
person-new-death-place = مكان الوفاة
person-new-death-hint = اتركه فارغًا ما دام الشخص حيًّا.
person-new-place-hint = يُنشأ المكان الذي لا يعرفه الأرشيف.
person-new-section-portrait = الصورة الشخصية
person-new-portrait-hint = PNG أو JPEG أو GIF أو WebP، حتى { $mb } ميغابايت.
person-new-bio-hint = تنسيق حر، كما في الملاحظة.
person-new-section-note = ملاحظة
person-new-note-hint = ملاحظة مستقلة عن هذا الشخص، بالصور أيضًا. إن لم يكن لها عنوان حملت اسم الشخص.
person-new-section-parents = الوالدان والإخوة
person-new-father = الأب
person-new-mother = الأم
person-new-siblings = الإخوة والأخوات
person-new-sibling-add = أضف أخًا أو أختًا
person-new-who-hint = اختر من القائمة أو اكتب اسمًا: الاسم الذي لا يحمله أحد يصبح سجلًا جديدًا.
person-new-section-unions = الاقترانات والأبناء
person-new-union-n = اقتران { $n }
person-new-partner = الزوج أو الشريك
person-new-union-type = نوع الارتباط
person-new-children = الأبناء
person-new-child-add = أضف ابنًا أو ابنة
person-new-union-add = أضف اقترانًا
person-new-save = أنشئ السجل
person-new-hint-existing = في الأرشيف
person-new-hint-new = سجل جديد
person-new-hint-ambiguous = عدة سجلات تحمل هذا الاسم: اختر واحدًا من القائمة.
person-new-hint-place-existing = مكان في الأرشيف
person-new-hint-place-new = مكان جديد
person-new-field-error = «{ $typed }»: { $message }
person-new-error-name = يلزم اسم أول أو اسم عائلة. لم يُحفظ شيء.
person-new-error-date = يُكتب التاريخ هكذا: 1890 أو 1890-06 أو 1890-06-14. لم يُحفظ شيء.
person-new-error-same-parents = الأب والأم شخص واحد. لم يُحفظ شيء.
person-new-error-siblings-need-parent = لا يكوّن الإخوة عائلة إلا عبر أحد الوالدين: اذكر أحد الوالدين، أو أخًا أو أختًا ينتمي إلى عائلة. لم يُحفظ شيء.
person-new-error-siblings-two-families = هؤلاء الإخوة ينتمون إلى عائلات مختلفة: اذكر الوالدين. لم يُحفظ شيء.
person-new-error-portrait = الصورة الشخصية ليست صورة يستطيع هذا الموقع عرضها: PNG أو JPEG أو GIF أو WebP. لم يُحفظ شيء.
person-new-error-conflict = تغيّر الأرشيف بينما كنت تكتب. لم يُحفظ شيء؛ احفظ مرة أخرى.
person-new-error-refused = رفض الأرشيف الحفظ. لم يُحفظ شيء.
person-new-created = { $others ->
        [0] أُنشئ السجل.
        [one] أُنشئ السجل ومعه سجل آخر.
        [two] أُنشئ السجل ومعه سجلان آخران.
        [few] أُنشئ السجل ومعه { $others } سجلات أخرى.
        [many] أُنشئ السجل ومعه { $others } سجلًا آخر.
       *[other] أُنشئ السجل ومعه { $others } سجل آخر.
    }
pick-error-place-not-found = لا مكان بهذا الاسم في هذا الأرشيف. لم يُحفظ شيء.
pick-error-place-ambiguous = أكثر من مكان ينطبق عليه ذلك. اختر واحدًا من القائمة. لم يُحفظ شيء.
pick-error-source-not-found = لا مصدر بهذا الاسم في هذا الأرشيف. لم يُحفظ شيء.
pick-error-source-ambiguous = أكثر من مصدر ينطبق عليه ذلك. اختر واحدًا من القائمة. لم يُحفظ شيء.
pick-error-thing-empty = لم يُذكر شيء. لم يُحفظ شيء.
pick-error-thing-not-found = لا شيء بهذا الاسم في هذا الأرشيف. لم يُحفظ شيء.
pick-error-thing-ambiguous = أكثر من سجل ينطبق عليه ذلك. اختر واحدًا من القائمة. لم يُحفظ شيء.
search-create = أنشئ «{ $name }»

# What a file sent to the site may weigh, and the kinds refused
# (docs/adr/0041).
uploads-title = إرسال الملفات
uploads-lede = الحجم الذي يمكن أن يبلغه ملف يُرسل إلى الموقع، وأنواع الملفات التي يقبلها الأرشيف.
uploads-sizes = الأحجام القصوى
uploads-limit-document = ملف مرفق بسجل (ميغابايت)
uploads-limit-note-image = صورة ملصقة في ملاحظة (ميغابايت)
uploads-limit-assistant-file = ملف يُرسل إلى المساعد (ميغابايت)
uploads-limit-assistant-message = كل ملفات رسالة واحدة (ميغابايت)
uploads-limit-gedcom = ملف GEDCOM (ميغابايت)
uploads-limit-archive = أرشيف يُستورد كاملًا (ميغابايت)
uploads-limit-range = من { $min } إلى { $max } ميغابايت؛ { $default } افتراضيًا.
uploads-kinds = أنواع الملفات
uploads-kinds-summary = { $accepted } نوعًا مقبولًا من { $total }
uploads-family-image = الصور
uploads-family-document = الوثائق
uploads-family-audio = الأصوات
uploads-family-video = الأفلام
uploads-kinds-hint = النوع غير المحدد يُرفض عند الإرسال؛ والملفات الموجودة في الأرشيف تبقى.
uploads-save = احفظ
uploads-saved = حُفظت إعدادات الإرسال.
uploads-error-range = { $field }: عدد صحيح من { $min } إلى { $max }. لم يُحفظ شيء.
uploads-not-saved = تعذّر حفظ الإعدادات: { $error }
uploads-summary = الملفات المرفقة بالسجلات: { $document } ميغابايت على الأكثر؛ { $accepted } نوعًا مقبولًا من { $total }.
uploads-open = اضبط الإرسال
error-upload-kind-refused = هذا النوع من الملفات ({ $kind }) يرفضه هذا الموقع: الإدارة لا تقبله. لم يُحفظ شيء.

# An import in place of the family, behind a word and a verified backup
# (docs/adr/0042).
setup-replace-title = استبدال العائلة
setup-replace-lede = يحتوي هذا الأرشيف بالفعل على عائلة. استيراد ملف يستبدلها بكاملها.
setup-replace-warning = كل ما ليس في الملف المستورد سيضيع: السجلات والملاحظات والصور والروابط.
setup-replace-backup = قبل ذلك مباشرة، تُكتب نسخة احتياطية متحقَّق منها للعائلة الحالية في { $dir }؛ وتُستعاد بالأمر axgf-cms restore.
setup-replace-keyword = استبدال الأرشيف
setup-replace-type = للتأكيد، اكتب «{ $keyword }»
setup-replace-go = استبدل العائلة
setup-replace-keyword-wrong = كلمات التأكيد غير مطابقة: اكتب «{ $keyword }». لم يُستبدل شيء.
setup-replace-backup-failed = فشل النسخ الاحتياطي للعائلة الحالية، لذا لم يُستبدل شيء: { $error }
setup-replace-changed = تغيّرت العائلة أثناء نسخها احتياطيًا، لذا لم يُستبدل شيء. أعد المحاولة.
setup-replace-done = العائلة المستبدَلة محفوظة في نسخة احتياطية متحقَّق منها: { $file }
setup-replace-room-title = استبدال العائلة باستيراد
setup-replace-room-detail = أرشيف أو GEDCOM مكان العائلة الحالية، بعد نسخة احتياطية متحقَّق منها.
setup-replace-room-go = افتح الاستيراد

# أوامر النسّابة الاصطناعي وMCP ‏(docs/adr/0045).
ops-line-create-person = إنشاء { $name }
ops-line-create-relative = إنشاء { $name } أيضًا
ops-line-create-place = إنشاء المكان { $name }
ops-line-update-person = تعديل { $name }: { $fields }
ops-line-set-birth = ميلاد { $name }: { $fact }
ops-line-set-death = وفاة { $name }: { $fact }
ops-line-add-event = إضافة { $event }
ops-line-update-event = تعديل { $event }
ops-line-delete-event = حذف { $event }
ops-line-add-union = جمع { $partners } ({ $kind })
ops-line-add-child = { $child }، من أبناء { $parents }
ops-line-remove-child = { $child } لم يعد من أبناء { $parents }
ops-line-delete-person = إخراج { $name } من كل العائلات والأحداث، ثم حذف السجل
ops-line-delete-union = فكّ رابطة { $partners }: يبقى الأبناء، من دون هذين الوالدين
ops-field-name = الاسم
ops-field-gender = الجنس
ops-field-living = على قيد الحياة أم لا
ops-field-visibility = الظهور
ops-field-bio = السيرة
ops-refused-at = العملية { $n }: { $why }
ops-refused-empty = لا شيء يُعمل.
ops-refused-no-person = لم يُذكر أي شخص.
ops-refused-not-found = لا شيء في الأرشيف يطابق «{ $what }».
ops-refused-ambiguous = أكثر من سجل يطابق «{ $what }»: سمِّه بمعرّفه.
ops-refused-date = «{ $value }» ليس تاريخًا: اكتب 1890 أو 1890-06 أو 1890-06-14.
ops-refused-gender = «{ $value }» ليس جنسًا: M أو F أو NB أو U.
ops-refused-visibility = «{ $value }» ليس درجة ظهور: public أو members أو contributors أو private.
ops-refused-category = «{ $value }» ليس فئة حدث يعرفها هذا الموقع.
ops-refused-role-name = «{ $value }» ليس دورًا في حدث.
ops-refused-union-kind = «{ $value }» ليس نوعًا من الروابط.
ops-refused-partners = للرابطة شريك واحد أو شريكان.
ops-refused-parents = سمِّ والدًا واحدًا أو اثنين، أو العائلة.
ops-refused-own-parent = لا أحد والدُ نفسه.
ops-refused-already-child = { $child } من أبناء هذه العائلة بالفعل.
ops-refused-not-a-child = { $child } ليس من أبناء هذه العائلة.
ops-refused-nothing = لن يتغيّر شيء.
ops-refused-name = يحتاج الشخص إلى اسم شخصي أو اسم عائلة.
ops-refused-field = { $field } «{ $typed }»: { $why }
ops-refused-role = يستطيع هذا الحساب قراءة الأرشيف لا تعديله.
ops-refused-scope = هذا يمسّ شخصًا خارج الفرع الذي يحق لهذا الحساب تعديله.
ops-refused-admin = لا يحذف شخصًا إلا مدير.
ops-refused-dangling = { $field } يشير إلى { $target }، وهو غير موجود في أي مكان من الأرشيف.
ops-refused-invalid = لا يمكن إجراء هذه العملية هنا: { $detail }.
ops-refused-conflict = تغيّر الأرشيف في الأثناء ولم يُكتب شيء. تحقّق مجددًا ثم طبّق.
ops-refused-library = رفض الأرشيف ذلك ولم يُكتب شيء: { $detail }
ops-refused-io = تعذّرت كتابة الأرشيف، ولم يُكتب شيء: { $detail }
journal-via-afg = النسّابة الاصطناعي
afg-ops-title = التعديلات المقترحة
afg-ops-apply = تطبيق
afg-ops-aside = تنحية
afg-ops-note = لا يتغيّر شيء قبل أن تطبّق. كل ما سبق يُطبَّق معًا، أو لا يُطبَّق شيء.
afg-ops-applied = { $n ->
    [0] طُبِّق: لم يُكتب أي سجل.
    [zero] طُبِّق: لم يُكتب أي سجل.
    [one] طُبِّق: كُتب سجل واحد.
    [two] طُبِّق: كُتب سجلّان.
    [few] طُبِّق: كُتبت { $n } سجلات.
    [many] طُبِّق: كُتب { $n } سجلًا.
   *[other] طُبِّق: كُتب { $n } سجل.
}
afg-ops-aside-done = نُحّي: لم يُعدَّل شيء.
afg-ops-json = العمليات كما كتبها النسّابة الاصطناعي
afg-ops-unreadable = كتب النسّابة الاصطناعي عمليات لا يعرف هذا الموقع قراءتها: { $detail }
afg-ops-applied-text = { $n ->
    [0] طبّقتُ التعديلات المقترحة: لم يُكتب أي سجل.
    [zero] طبّقتُ التعديلات المقترحة: لم يُكتب أي سجل.
    [one] طبّقتُ التعديلات المقترحة: كُتب سجل واحد.
    [two] طبّقتُ التعديلات المقترحة: كُتب سجلّان.
    [few] طبّقتُ التعديلات المقترحة: كُتبت { $n } سجلات.
    [many] طبّقتُ التعديلات المقترحة: كُتب { $n } سجلًا.
   *[other] طبّقتُ التعديلات المقترحة: كُتب { $n } سجل.
}
afg-ops-aside-text = نحّيتُ التعديلات المقترحة: لم يُعدَّل شيء.
afg-ops-preparing = النسّابة الاصطناعي يُعدّ تعديلات لاقتراحها…
account-tokens-title = رموز الوصول لذكاء اصطناعي (MCP)
account-tokens-intro = يتيح الرمز لذكاء اصطناعي خارجي — ChatGPT أو Claude أو أي ذكاء آخر يتحدث MCP — قراءة أرشيف العائلة باسم حسابك، وتعديله إن كان الرمز يملك الكتابة. ولا يفعل أبدًا أكثر مما يحق لحسابك.
account-tokens-closed = بروتوكول MCP في هذا الموقع مغلق: لا يفتح أي رمز شيئًا حتى يفتحه مدير.
account-tokens-open-it = فتحه
account-token-made = أُنشئ الرمز «{ $label }».
account-token-once = انسخه الآن: لا يُعرض إلا هذه المرة، ولا تُحفظ إلا بصمته. من يملكه يتصرف باسم حسابك.
account-token-value = الرمز
account-token-address = عنوان MCP
account-token-address-with = العنوان والرمز في داخله
account-token-address-with-hint = لذكاء اصطناعي لا يقبل إلا عنوانًا، مثل موصل ChatGPT أو Claude: احفظه سرًّا كما تحفظ الرمز.
account-token-writes = يقرأ ويكتب
account-token-reads = يقرأ فقط
account-token-made-on = أُنشئ في { $date }
account-token-used-on = آخر استخدام في { $date }
account-token-never-used = لم يُستخدم قط
account-token-revoke = إبطال
account-token-revoked = أُبطل الرمز: لم يعد يفتح شيئًا.
account-tokens-none = لا رمز بعد.
account-token-label = اسم الرمز
account-token-label-example = ChatGPT، Claude على الحاسوب المحمول…
account-token-write = يمكنه تعديل الأرشيف، كما يحق لهذا الحساب
account-token-make = إنشاء رمز
account-tokens-too-many = يحتفظ الحساب بعشرة رموز على الأكثر: أبطل واحدًا لتنشئ آخر.
account-tokens-how = يشرح دليل المساعد الشخصي كيف توصل ChatGPT وClaude وغيرهما، بالموصل المجتمعي ax-genealogy.
account-tokens-guide = قراءة الدليل
account-tokens-connector = تنزيل الموصل لـ Claude Desktop ‏(.mcpb)
afg-mcp = MCP للذكاء الاصطناعي الخارجي
afg-mcp-intro = يتيح MCP لذكاء اصطناعي خارجي — ChatGPT أو Claude أو أي ذكاء آخر يتحدث Model Context Protocol — قراءة الأرشيف وتعديله باسم حساب واحد، برمز أنشأه ذلك الحساب في صفحته. كل رمز يفعل ما يحق لحسابه، ولا أكثر.
afg-mcp-state = الحالة
afg-mcp-open = مفتوح
afg-mcp-shut = مغلق
afg-mcp-address = العنوان
afg-mcp-address-relative = بعد العنوان العام للموقع، ولم يُضبط بعد: تضبطه وحدة التحكم.
afg-mcp-tokens = رموز الحسابات النشطة
afg-mcp-privacy = كل ما يحق لحساب قراءته يذهب إلى الشركة التي يقرؤه ذكاؤها الاصطناعي، والأحياء من ضمنه. لا تفتح MCP إلا إذا وافقت العائلة.
afg-mcp-switch = فتح MCP لرموز الحسابات
afg-mcp-close = إغلاق MCP
afg-mcp-opened = MCP مفتوح: تُقبل رموز الحسابات.
afg-mcp-closed = MCP مغلق: تُحفظ الرموز ولا تفتح شيئًا.
assistant-guide-title = إدارة نسب العائلة مع النسّابة الاصطناعي
assistant-guide-reader = حسابك يقرأ الأرشيف دون أن يعدّله: يجيب النسّابة الاصطناعي عن أسئلتك لكنه لا يقترح تعديلًا. يستطيع مدير أن يمنح حسابك دور المساهم.

# من يحق له فتح كل جزء من الموقع ‏(docs/adr/0046).
doors-title = من يحق له فتح كل جزء من الموقع
doors-intro = لا يرى الزوار إلا صفحة العائلة ما دمت لم تفتح هنا شيئًا آخر؛ وكل ما سواها للحسابات المسجّلة الدخول. ويحتفظ كل سجل فوق ذلك بدرجة ظهوره.
doors-change = تغيير من يحق له فتح كل جزء
doors-save = حفظ
doors-saved = حُفظ: من يحق له فتح كل جزء من الموقع.
doors-unknown = هذا المستوى غير موجود في هذا الموقع. لم يُحفظ شيء.
doors-part-family = صفحة العائلة
doors-part-river = النهر (أو رسم السمة)
doors-part-tree = الشجرة
doors-part-records = السجلات والبحث والملاحظات
doors-part-impact = الأثر
doors-part-import = الاستيراد (GEDCOM)
doors-level-visitors = الجميع
doors-level-members = الحسابات المسجّلة الدخول
doors-level-contributors = المساهمون
doors-level-admins = المديرون
doors-closed-title = غير مفتوح لحسابك
doors-closed-detail = يخصّص مدير هذا الجزء من الموقع لأدوار أخرى.
