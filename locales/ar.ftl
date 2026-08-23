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
tree-whole-tree = الشجرة كاملة
tree-focused = عرض مركّز
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
tree-legend-confidence = تعتيم الخط يدل على درجة اليقين
tree-recentre = اجعل الشجرة تتمركز هنا
tree-open-record = افتح السجل الكامل

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
record-name-primary = الاسم الأساسي
record-transliteration = النقحرة اللاتينية
record-born = وُلد
record-died = تُوفّي
record-parents = الوالدان
record-siblings = الإخوة والأخوات
record-children = الأبناء
record-spouse = الزوج
record-union-ended = انتهى
record-no-date = التاريخ غير معروف
record-unknown-person = [غير معروف]
record-restricted-person = خاص
record-restricted-title = سجل هذا الشخص غير مرئي لك
record-absent-person-title = مذكور في هذا الملف لكنه غير موجود فيه
record-confidence = درجة اليقين
record-source = المصدر
record-role = الدور
record-download = تنزيل
record-attach-file = إرفاق ملف
record-attach-hint = حتى { $mb } ميغابايت. تُعرض الصور في المعرض، وما عداها يُدرج مع رابط للتنزيل.
record-no-documents = لا ملفات مرفقة بهذا السجل.

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
admin-search = بحث
admin-page-of = صفحة { $page } من { $pages }
admin-previous = السابق
admin-next = التالي
admin-nothing-here = لا شيء من هذا النوع مسجّل في هذا الملف بعد.
admin-saved = حُفظ بوصفه الإصدار { $version } — { $summary }
admin-not-saved = لم يُحفظ
admin-created = أُنشئ
admin-not-created = لم يُنشأ
admin-deleted = حُذف
admin-not-deleted = لم يُحذف — الملف كما هو
admin-delete-policy = التكامل المرجعي
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

home-lede = { $family } — { $total ->
        [zero] لا سجلات
        [one] سجل واحد
        [two] سجلان
        [few] { $total } سجلات
        [many] { $total } سجلًا
       *[other] { $total } سجل
    } في ملف ‎.axgf‎ واحد.
home-why-title = لماذا AXGF
home-what-this-bundle-has = ما يحتويه هذا الملف فعلًا
home-browse-tree = تصفّح الشجرة
home-convert-gedcom = حوّل ملف GEDCOM
home-see-example = انظر مثالًا

convert-title = تحويل GEDCOM إلى AXGF
convert-choose-file = ملف GEDCOM
convert-submit = تحويل
convert-result-title = نتيجة التحويل
convert-download = نزّل ملف ‎.axgf‎
convert-diagnostics = ما أبلغ عنه المحوّل
convert-unchanged-note = التحويل لا يمس أبدًا الملف الذي يقدّمه هذا الموقع.

completeness-title = اكتمال الملف
completeness-recorded = مسجَّل
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
