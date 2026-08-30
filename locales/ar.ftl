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

prefs-title = اللغة والمظهر
prefs-language = اللغة
prefs-language-note = هذا يغيّر الواجهة وحدها. أما الأسماء والأماكن والملاحظات فتُعرض دائمًا بلغتها وخطّها.
prefs-theme = المظهر
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
    }، { $depth } أجيال في كل اتجاه. الأقدم في الأسفل. تعتيم الخط يدل على درجة اليقين في العلاقة — فالخط الباهت ادعاء لا يجزم به السجل.
tree-lede-whole = كل الأشخاص في الملف. الأقدم في الأسفل والأحدث في الأعلى. تعتيم الخط يدل على درجة اليقين في العلاقة.
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
tree-click-hint = انقر أي بطاقة لفتح سجل ذلك الشخص في اللوحة؛ و«اجعل الشجرة تتمركز هنا» في اللوحة يعيد تجذير العرض.
tree-edge-union = اقتران مسجَّل
tree-edge-parentage = بنوّة مسجَّلة
tree-title-suffix = الشجرة

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
    }، من { $path }. ويُحفظ السجل بجوار الأرشيف لا داخله: فالأرشيف يُنسخ ويُرسل ويُنشر، ومن صحّح ماذا حقيقةٌ عن القائمين على الشجرة لا عن العائلة التي فيها.
admin-bundle-heavy = حجم هذا الأرشيف { $size }. ويُقرأ كاملًا عند الإقلاع ويبقى في الذاكرة، فبعد { $warn } تقريبًا يبدأ الموقع يكلّف ذاكرة حقيقية وتبطؤ عمليات إعادة التشغيل. وهذا يناسب أرشيفًا عائليًا لا مكتبة وسائط — فإن كانت المرفقات تنمو بلا حد فاحفظها في مخزن ملفات واجعل الأرشيف يشير إليها.
admin-raw-json-help = السجل كاملًا، فلا شيء يتعذّر تحريره — القوائم مثل شركاء العائلة وأبنائها، أو تاريخ حدود مكان، تعيش هنا. وهذا هو المستند الأصل؛ ثم تُكتب الحقول أعلاه فوق المسارات التي تخصّها، فحرّر القيمة في أحد الموضعين لا في كليهما. ولا بد أن يُقرأ بصيغة JSON وإلا لم يُحفظ شيء.
accounts-lede = محفوظة في { $path }، بأذونات 600، بجوار الأرشيف لا داخله أبدًا. فالأرشيف يُنسخ ويُرسل ويُنشر؛ ولو سافرت بصمات كلمات السر في داخله لصارت كل نسخة من شجرة العائلة نسخةً من بيانات الدخول.
accounts-no-registration = لا تسجيل ذاتيًا ولا دعوات، عن قصد. فأرشيف عائلة يكفيه مدير يعرف الجميع، وفي ذلك إزالةٌ لمساحة إساءة الاستعمال بأكملها بدل الدفاع عنها.
accounts-branch-hint = يقصر ما يستطيع هذا الحساب تحريره على أولئك الأشخاص وذريتهم وأزواجهم. ولا يقصر ما يستطيع قراءته — فذاك يحكمه مدى ظهور كل سجل، والأمران مفصولان عن قصد.
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
convert-lede = انقل شجرة قائمة من ملف GEDCOM — وهو التصدير الذي تنتجه معظم برامج الأنساب. ستستعيد أرشيفًا تحتفظ به. لا يُحفظ هنا شيء، والشجرة التي يعرضها هذا الموقع تبقى كما هي تمامًا.
convert-file-label = ملف عائلي (‎.ged‎)
convert-file-hint = حتى { $mb } ميغابايت. شجرة من 767 شخصًا نحو 320 كيلوبايت.
convert-confidence-label = ما مقدار الثقة بهذه الوقائع في البداية
convert-confidence-hint = الملف الذي تستورده لا يقول كم كان أحدهم واثقًا، فكل واقعة تحتاج نقطة بداية. اجعلها منخفضة لشجرة جُمعت على عجل، وأعلى لشجرة عُملت من الوثائق. والقراءة الأمينة لهذا الرقم هي «مستورَد، ولم يراجعه أحد منذئذ» — ويمكنك رفع أي واقعة أو خفضها لاحقًا، واحدةً واحدة.
convert-lang-label = لغة أسماء الأماكن
convert-lang-hint = وسم مثل en أو fr أو ar. والمكان الواحد قد يحمل اسمه بعدة لغات؛ وهذا يبيّن بأي لغة كُتبت الأسماء في ملفك.
convert-what-you-get = ما يضيفه الاستيراد
convert-what-you-get-1 = كل واقعة تنال درجة ثقة يمكنك تعديلها لاحقًا، فيُكتب الشك بدل أن يُطرح. والتواريخ تحفظ صورتها: نحو 1500، وقبل 1430، وبين 1920 و1925 تبقى ثلاث عبارات مختلفة، وما لم يستطع أحد قراءته تاريخًا يُحفظ كلمةً بكلمة. والمهنة تصير مدة لها بداية ونهاية. وكل مكان يصير مدخلًا قائمًا بذاته، فتحتفظ بلدةٌ غيّرت دولتها بذلك التاريخ.
convert-no-way-back = لا تُقدَّم الكتابة رجوعًا إلى ملف ‎.ged‎. فتلك الصيغة ليس فيها موضع لدرجة الثقة بواقعة، ولا لصلة خارج العائلة، ولا لطول مهنة، ولا لتاريخ لم يستطع أحد تحديده — ورحلة العودة ستُسقطها في صمت. أما أرشيفك فيُصدَّر كاملًا، ملفًا واحدًا.
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
convert-skipped-note = لم يكن في هذه المدخلات ما يمكن نقله. وهي مذكورة لا مبتلعة: فمعرفة ما بقي وراءك على وجه الدقة هي الفرق بين استيراد تثق به وآخر لا تثق به.
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
convert-download-note = يُحفظ هنا خمس عشرة دقيقة ثم يُتلف، فنزّله الآن. هذا الملف الواحد هو الشجرة كلها؛ فاحفظه في مأمن.
convert-another = استيراد ملف آخر

## الشجرة والصفحة الرئيسة

date-quarter-century = الربع { $quarter ->
        [1] الأول
        [2] الثاني
        [3] الثالث
       *[other] الرابع
    } من القرن { $century }
tree-width-notice = عرض هذا العرض { $width } بكسل. وكل جيل صف واحد، وأعرض الصفوف هو الذي يحدّد هذا العرض — فعلى شاشة عرضها 1500 بكسل يعادل ذلك { $screens ->
        [zero] لا تمرير أفقي
        [one] شاشة واحدة
        [two] شاشتين
        [few] { $screens } شاشات
        [many] { $screens } شاشةً
       *[other] { $screens } شاشة
    } من التمرير الأفقي. أما العرض المتمركز حول شخص فيُظهر بضع عشرات حوله، وكل بطاقة تعيد تمركزه.
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
home-what-title = ماذا يقدّم هذا لعائلة
home-what-archive-title = مكان واحد للأرشيف كله
home-what-archive-body = الشجرة والمستندات والصور تجتمع معًا. فنسخةُ عقد زواج ممسوحة تتعلّق بالزواج نفسه لا ببريد أحدهم، والصورة تسمّي من فيها.
home-what-together-title = أقارب عدّة، أدوار مختلفة
home-what-together-body = عمّة لديها ثلاثون سنة من الملاحظات، وابن عم يريد تصحيح هجاء واحد فحسب، لا يحتاجان الصلاحيات نفسها. كل قريب يُدعى بدوره الخاص، وكل تغيير يسجّل من أجراه ومتى.
home-what-privacy-title = الخصوصية تُقرَّر شخصًا شخصًا
home-what-privacy-body = قريبٌ على قيد الحياة يمكن أن يكون ظاهرًا للعائلة محجوبًا عن الزوار، بينما جدّته الكبرى مفتوحة للجميع. والاختيار يُتخذ لكل شخص، لا مرةً واحدة للشجرة كلها.
home-what-languages-title = إحدى عشرة لغة
home-what-languages-body = يقرأ الأقارب الموقع بلغتهم — والروسية منها، وهي اللغة التي حُفظت بها سجلات الأحوال المدنية في نصف وسط أوروبا وشرقها. والاسم يبقى بخطّه إلى جانب نقله الحرفي؛ ولا حاجة إلى ردّ شيء إلى أبجدية واحدة كي يعمل الموقع.
home-what-export-title = الأرشيف يبقى لك
home-what-export-body = صدّر كل شيء ملفًا واحدًا متى شئت — الأشخاص والصلات والمستندات والصور معًا. وإن قرّرت يومًا الرحيل، فأنت ترحل والأرشيف كامل معك.
home-in-this-tree = ما سجّلته العائلة حتى الآن
home-showcase-title = حيث تقول هذه الشجرة أكثر من الأسماء والتواريخ
home-showcase-note = كل بند هنا مأخوذ مما هو مسجَّل فعلًا، لا من قائمة بما يستطيع الموقع فعله.
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
record-doc-photo = صورة
record-doc-certificate = وثيقة
record-doc-letter = رسالة
record-doc-record = قيد
record-doc-newspaper = صحيفة
record-doc-other = غير ذلك
record-upload-help = حتى { $mb } ميغابايت للملف. تُحفظ المرفقات بجوار الشجرة وتُكتب في الأرشيف عند التصدير، فتسافر الصورة مع العائلة التي تخصّها. ويُقرأ نوع الملف من محتواه لا من اسمه: تُقبل الصور وPDF والنص الصِّرف والصوت والفيديو. أما SVG فمرفوض، لأنه قد يحمل سكربتًا.
record-upload-help-short = حتى { $mb } ميغابايت. وSVG مرفوض.
record-verbatim-note = محفوظ كما جاء في السجل تمامًا، لأن أي محوِّل لم يستطع تأويله. والبديل كان طرحه.
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
completeness-intro = ما هو مسجَّل وما لا يزال فارغًا. وليس في ذلك خطأ: فالسطر الفارغ موضعٌ يمكن للسجل أن ينمو فيه، لا شيء أخفق.
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
