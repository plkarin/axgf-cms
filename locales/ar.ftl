# axgf-cms — نصوص الواجهة، العربية.
#
# جودة آلية — لم تراجعها متحدثة أو متحدث بالعربية لغةً أُولى. ومفردات علم
# الأنساب على وجه الخصوص (union، affiliation، confidence) لها مقابلات مستقرة
# تختلف باختلاف تقاليد حفظ السجلات، وقد تكون الترجمة هنا خاطئة. التصويبات
# مرحّب بها — انظر CONTRIBUTING.md.
#
# للعربية ست صيغ عددية في قواعد CLDR: zero وone وtwo وfew وmany وother.
# وهي مذكورة كاملة أدناه؛ فرضُ منطق «واحد أو أكثر» الإنجليزي عليها خطأ.
#
# القاعدة: هذا الملف يترجم الواجهة وحدها. الأسماء والأماكن والملاحظات والمهن
# تأتي من ملف ‎.axgf‎ وتبقى بلغتها وخطّها.

app-name = axgf-cms

nav-tree = الشجرة
nav-convert = تحويل GEDCOM
nav-admin = الإدارة
nav-sign-in = تسجيل الدخول
nav-sign-out = تسجيل الخروج
footer-served-from = يُقدَّم من ملف ‎.axgf‎ واحد. منطق الأنساب كله في axgf-rs، والصيغة يحدّدها axgf-spec.

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

home-why-title = لماذا AXGF

convert-title = تحويل GEDCOM إلى AXGF
convert-submit = تحويل
convert-result-title = نتيجة التحويل
convert-download = نزّل ملف ‎.axgf‎

completeness-title = اكتمال الملف
completeness-empty = فارغ
completeness-spec-field = حقل AXGF

## Dates

date-unknown = التاريخ غير معروف
date-not-recorded = غير مسجَّل
date-circa = نحو { $date }
date-between = بين { $from } و{ $to }
date-before = قبل { $date }
date-after = بعد { $date }
date-preserved = مسجَّل هكذا: «{ $text }»
date-day-month-year = { $day } { $month } { $year }
date-month-year = { $month } { $year }
date-decade = عقد { $decade }
date-century = القرن { $century }

month-1 = يناير
month-2 = فبراير
month-3 = مارس
month-4 = أبريل
month-5 = مايو
month-6 = يونيو
month-7 = يوليو
month-8 = أغسطس
month-9 = سبتمبر
month-10 = أكتوبر
month-11 = نوفمبر
month-12 = ديسمبر

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
        [one] جيل واحد
        [two] جيلان
        [few] { $generations } أجيال
        [many] { $generations } جيلًا
       *[other] { $generations } جيل
    }
tree-unplaced-count = { $n } بلا موضع
tree-no-people = لا أشخاص في هذا الملف بعد.
tree-no-people-cta = حوّل ملف GEDCOM لملئه.
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
record-gedcom-would-lose = ما كان GEDCOM ليفقده هنا:
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

note-links = { $n ->
        [zero] لا علاقات خارج الأسرة
        [one] علاقة واحدة خارج الأسرة بتواريخها ومصادرها ودرجة يقينها
        [two] علاقتان خارج الأسرة بتواريخهما ومصادرهما ودرجة يقينهما
        [few] { $n } علاقات خارج الأسرة بتواريخها ومصادرها ودرجة يقينها
        [many] { $n } علاقة خارج الأسرة بتواريخها ومصادرها ودرجة يقينها
       *[other] { $n } علاقة خارج الأسرة بتواريخها ومصادرها ودرجة يقينها
    }
note-occupations = { $n ->
        [zero] لا مهن
        [one] مهنة واحدة مسجَّلة كمدة لا كحدث
        [two] مهنتان مسجَّلتان كمدة لا كحدث
        [few] { $n } مهن مسجَّلة كمدد لا كأحداث
        [many] { $n } مهنة مسجَّلة كمدد لا كأحداث
       *[other] { $n } مهنة مسجَّلة كمدد لا كأحداث
    }
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
