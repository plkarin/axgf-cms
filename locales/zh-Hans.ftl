# axgf-cms — 界面文本，简体中文。
#
# 机器质量 — 未经母语者校订。家谱学词汇有随记录传统而异的既定译法，本译文
# 可能有误。欢迎指正 — 参见 CONTRIBUTING.md。
#
# 采用的译法（欢迎母语者提出异议）:
#   union → 婚姻关系 · link → 关联 · confidence → 可信度
#   reliability → 可靠性 · source → 来源 · primary source → 原始史料
#   occupation → 职业 · record → 记录 · archive → 档案
#   godparent → 教父母 · witness → 见证人 · speculative → 推测
#
# 复数: 汉语没有复数变化，CLDR 只有 other 一类。不要引入英语「一个还是多个」
# 的分支。
#
# 日期: 1923年4月12日。这是以数字组成的结构，不是翻译过来的月份名称，因此
# 没有月份名称表。
#
# 标点: 使用全角的「，」「。」，不照搬英文标点。
#
# 原则: 本文件只翻译界面。人名、地名、笔记与职业名称来自档案，保持其原有的
# 语言和文字。

app-name = ax-genealogy

## 页眉与页脚

nav-tree = 家谱
nav-convert = 导入
nav-admin = 管理
nav-sign-in = 登录
nav-sign-out = 退出
footer-open-format = 您家族的档案是留在自己手里的一个文件，以公开格式写成，在本站消失很久之后依然打得开。
footer-open-format-link = 关于该格式

## 偏好设置

prefs-title = 语言与外观
prefs-language = 语言
prefs-theme = 外观
prefs-apply = 应用
prefs-reviewed = 已校订
prefs-machine = 机器翻译，{ $coverage }%
prefs-machine-complete = 已译全，尚未校订
prefs-machine-title = 未经母语者校订的翻译。家谱学词汇尤其可能有误：表示婚姻关系、教父母或原始史料的词，各国记录传统各不相同。欢迎指正，CONTRIBUTING.md 说明了从何入手。

theme-light = 浅色
theme-dark = 深色
theme-system = 跟随系统
theme-high-contrast = 高对比度
theme-sepia = 棕褐
theme-deuteranopia = 绿色盲
theme-protanopia = 红色盲
theme-tritanopia = 蓝色盲
theme-colour-blind-note = 色觉友好
theme-contrast-note = 最高对比度

## 家谱

tree-title-around = { $name }的周边
tree-title-whole = 整棵家谱
tree-lede-focused = 祖先{ $ancestors }人、后代{ $descendants }人、配偶或伴侣{ $spouses }人，每个方向{ $depth }代。
tree-filter-label = 筛选可见的卡片
tree-filter-placeholder = 输入姓名…
tree-centre-on = 以此人为中心
tree-depth = 每个方向的代数
tree-show = 显示
tree-hidden-notice = 有{ $n }人只显示姓名而不显示细节
tree-hidden-because-role = ，因为其可见范围高于您的账户所能读取的范围。
tree-hidden-because-anonymous = ，因为他们并未公开。
tree-hidden-sign-in = 如果您有账户，请登录。
tree-restricted-card = 此人的记录对您不可见
tree-empty = 还没有可以绘制的人。
tree-unplaced = 不属于任何已记录的家庭

## 记录

record-identity = 身份
record-life-events = 生平事件
record-family = 家庭
record-other-relationships = 其他关系
record-occupations = 职业
record-places = 地点
record-sources-documents = 来源与文献
record-notes = 笔记
record-history = 修改历史
record-raw = 原始数据
record-raw-summary-note = 生成本页所依据的 JSON

record-identity-help = 每一个已记录的姓名，连同其类别、使用的时期和支持它的来源；原文文字与拉丁转写不同的，并列显示。此外还有性别、是否在世和可见范围。
record-life-events-help = 出生、去世，以及此人参与的每一件事，按日期排列，各自标明其身份——这样一场他只是见证的婚礼，会与他自己的婚礼并排出现。没有日期的事实排在最后，而不是假装排在最前。
record-family-help = 父母与兄弟姐妹，然后是每一段婚姻关系的类别、日期、地点、结束方式，以及按出生顺序排列的子女。
record-other-relationships-help = 每一条以此人为一端的关联，都从他这一侧来读：同一条记录从一端看是「某人的教父」，从另一端看是「某人的教子」。
record-occupations-help = 职业以时间段的形式落在同一根轴上，两段任职可以一眼比较；缺少端点的，条形保持开口。
record-places-help = 这条记录触及的每一个地点，连同在那里发生的事，以及使一个地方在不同时代仍可理解的疆界变迁。
record-sources-documents-help = 每条来源都列出本页中依赖于它的事实，按证据的强弱排列。
record-notes-help = 关于这条记录的笔记，包括任何转换器都无法解读、因而原样保留而非丢弃的文字。
record-history-help = 这条记录每一次保存的修改，最新的在前。谁改了什么，是关于维护家谱的人的事实，而不是关于其中家族的事实，因此它留在导出的档案之外，只向已登录的亲属显示。
record-raw-help = 这里没有为显示而生成的内容：这就是记录被保存时的样子，直到字段名。若有朝一日您需要不借助本站阅读档案，看到的正是这些。
record-help-toggle = 本节显示的内容

record-gender = 性别
record-living = 在世
record-visibility = 可见范围
record-yes = 是
record-no = 否
record-name-type = 姓名类别
record-name-used = 使用时期
record-name-evidence = 依据
record-transliteration = 拉丁转写
record-born = 生
record-died = 卒
record-parents = 父母
record-siblings = 兄弟姐妹
record-children = 子女
record-unknown-person = ［不详］
record-restricted-person = 私密
record-restricted-title = 此人的记录对您不可见
record-absent-person-title = 在本家谱中被提到，但没有自己的记录
record-confidence = 可信度
record-source = 来源
record-download = 下载

## 访问

access-restricted-title = 对您不可见
access-restricted-signed-in = 这条记录的可见范围高于您的账户所能读取的范围。管理员可以更改记录的可见范围，或更改您的角色。
access-restricted-anonymous = 这条记录并未公开。请登录以查看您的账户能否读取。
access-role-title = 不属于您的角色
access-role-admin = 这是管理员页面。您的账户可以创建和编辑记录，但不能管理账户、删除记录或导出档案。
access-role-write = 您的账户可以阅读这棵家谱，但不能更改。管理员可以把您的角色提升为编辑者。
access-scope-title = 在您负责的支系之外
access-scope-named = 您的账户仅限于家谱的一个支系，而这条记录涉及支系之外的人。记录中提到的每一个人都必须在您的支系之内——否则，一个配偶来自外部的家庭，就成了改写那个人亲缘关系的途径。
access-scope-unnamed = 您的账户仅限于家谱的一个支系，而这条记录没有提到可供比对的任何人。来源和地点由能访问整棵家谱的账户来编辑。

## 错误

error-not-found-title = 未找到
error-not-found-detail = 此处没有该页面。
error-no-such-person-title = 没有这个人
error-no-such-person-detail = 此处没有使用该标识的人。
error-no-such-entity-title = 没有这条记录
error-no-such-entity-detail = 此处没有使用该标识的记录。
error-deleted-while-editing = 此处没有使用该标识的记录。它可能在您编辑期间被删除了。
error-no-such-file-title = 没有该文件
error-no-such-file-detail = 此处没有使用该标识的文献，或者该文献记录时没有附文件——被引用的文献指向存放在别处的东西。
error-not-an-image-title = 不是图像
error-not-an-image-detail = 该文献没有缩略图，因为它不是本版本能够解码的图像。
error-back = 返回

## 登录

login-title = 登录
login-lede = 账户由管理员创建。
login-username = 用户名
login-password = 密码
login-submit = 登录
login-wrong = 用户名与密码不匹配。
login-token-wrong = 该令牌不正确。
login-throttled = 失败次数过多。请等待几分钟后再试。
login-no-accounts-title = 本安装尚无任何账户。
login-no-accounts-detail = 这里有意不设初始配置页：从部署到首次登录之间，正是一套安装最无防护的时候，因此第一个管理员从命令行创建。
login-no-accounts-note = 它会把生成的密码在 stderr 上打印一次，此后不再显示。在那之前，唯一的入口是下面的应急令牌。
login-emergency-summary = 应急访问
login-emergency-detail = 共享令牌仍能开启一个管理员会话，它只为一件事而存在：在 .acl 文件丢失或所有管理员都被挡在门外时重新进入。它不是账户——没有自己的偏好设置，修改日志会把它记为 emergency-token 而不是某个人。它的使用会以警告形式记入日志。
login-emergency-label = 应急令牌
login-emergency-submit = 使用应急令牌
login-sign-in-prompt = 请登录以进入管理面板。

## 管理

admin-title = 管理
admin-lede = 正在编辑 { $path } —— 记录{ $total }条，附件{ $files }个，磁盘占用{ $size }。每次改动都一次性写入；被拒绝的改动不会碰到文件。
admin-entities = 记录
admin-create = 新建
admin-new-kind = 新建：{ $kind }
admin-operations = 操作
admin-validate = 校验
admin-deduplicate = 合并重复
admin-export = 导出档案
admin-accounts = 账户
admin-roles-note = 校验、合并重复、导出、删除和账户管理仅限管理员。编辑者可以到达这里的其他每一个页面。
admin-dedup-confirm = 合并重复会融合记录并重写档案。是否继续？
admin-recent-changes = 最近的修改
admin-recent-note = 来自 { $path } 的{ $total }条已记录改动中的最近{ $shown }条。
admin-sessions-open = 当前有{ $n }个会话打开。
admin-no-changes-yet = 通过本应用尚未改动过任何内容。此后每一次保存都会记入 { $path }。
admin-last-validation = 上次校验
admin-bundle-heavy = 这份档案有{ $size }。它在启动时整体读入并常驻内存，因此超过约{ $warn } 之后，本站开始占用可观的内存，重启也会变慢。这样的设计适合家族档案而不是媒体库——如果附件无节制地增长，请把它们放进文件存储，让档案指向它们。

admin-fields = 字段
admin-raw-json = 原始 JSON
admin-raw-json-help = 整条记录，因此没有什么是不能编辑的——像家庭的配偶和子女列表、地点的疆界变迁这类数组就在这里。这是起始文档；上面的各字段随后会写到各自负责的路径上，所以同一个值请只在一处编辑，不要两处都改。它必须能按 JSON 解析，否则什么都不会保存。
admin-save = 保存
admin-cancel = 取消
admin-delete = 删除
admin-not-set = —— 未设置 ——
admin-edit = 编辑
admin-page-of = 第 { $page } 页，共 { $pages } 页
admin-previous = 上一页
admin-next = 下一页
admin-saved = 已保存为版本 { $version } —— { $summary }
admin-not-saved = 未保存
admin-created = 已创建
admin-not-created = 未创建
admin-deleted = 已删除
admin-not-deleted = 未删除——什么都没有改动
admin-what-changed = 改动内容
admin-field = 字段
admin-from = 原值
admin-to = 新值
admin-version = 版本 { $version }

## 账户

accounts-title = 账户
accounts-lede = 以权限 600 保存在 { $path }，在档案旁边，绝不在档案里面。档案会被复制、寄送和公开；若密码散列随之一同流传，家谱的每一份副本都会成为登录凭据的副本。
accounts-existing = 现有账户
accounts-username = 用户名
accounts-role = 角色
accounts-status = 状态
accounts-branch = 支系
accounts-last-seen = 最近登录
accounts-change = 更改
accounts-you = （您）
accounts-active = 启用
accounts-disabled = 停用
accounts-never = 从未
accounts-whole-tree = 整棵家谱
accounts-roots = 起点{ $n }人
accounts-add = 添加账户
accounts-no-registration = 这里有意不设自助注册，也不设邀请流程。对家族档案而言，一位认识所有人的管理员就够了，而且这样是把可被滥用的入口整个去掉，而不是去防守它。
accounts-password-hint = 留空则自动生成并只显示一次。若自行设置，至少{ $min }个字符。
accounts-new-password-placeholder = 新密码（留空则不变）
accounts-email = 电子邮箱
accounts-optional = （可选）
accounts-create = 创建账户
accounts-role-viewer = 阅读者 —— 可读公开记录和面向家族的记录
accounts-role-contributor = 编辑者 —— 另可创建、编辑和上传文件
accounts-role-admin = 管理员 —— 另可管理账户、删除和导出
accounts-branch-hint = 把这个账户可编辑的范围限制为这些人、他们的后代和配偶。
accounts-branch-reading = 它不限制可阅读的范围——那由每条记录的可见范围决定，两者是有意分开的。
accounts-branch-placeholder = 每行一个人物标识
accounts-ids-in-bundle = 本家谱中的人物标识
accounts-emergency-warning = 您正以应急令牌登录。它在本次会话中授予管理员权限，但它不是账户：没有自己的偏好设置，修改日志会把您的改动记为 emergency-token 而不是某个人。请在下面为自己建一个真正的账户并改用它登录。
accounts-created-with-password = 已创建 { $username }。密码是 { $password } —— 只显示这一次，且仅以 Argon2id 散列保存，请现在就转交。
accounts-created = 已创建 { $username }。
accounts-updated = 已更新 { $username }。该账户打开的会话都已退出。
accounts-username-taken = 该用户名已被占用。
accounts-pick-role = 请选择一个角色。
accounts-no-such = 没有这个账户。
accounts-last-admin = 这是唯一在用的管理员。请先提升另一个人——没有管理员的安装，只能靠编辑 .acl 文件或使用应急令牌来恢复。
accounts-not-saved = 未保存：{ $error }

## 冲突

conflict-title = 有人先一步改动了它
conflict-lede = 在您打开之后，{ $who } 于 { $when } 保存了对这条{ $kind }的改动。您的编辑没有被保存，也没有任何内容被覆盖。
conflict-no-merge = 这里不做任何自动合并。把两个人的编辑融合在一起，会得到一条谁都没有选择的记录；而在家谱里，两位编者对一个日期各执一词，通常意味着他们读的是不同的来源——那是要由人来回答的问题，不是程序。请比较下面两者再作决定。
conflict-versions = 您是从版本 { $expected } 开始的；记录现在是版本 { $current }。
conflict-both-changed = 两人都改动了这些
conflict-both-changed-detail = 这些字段你们两人都编辑过。无论您保存什么，都会替换 { $who } 填入的内容：
conflict-different-fields = 你们改动的是不同的字段，因此 { $who } 的工作没有争议之处——但重新应用仍会把您的整条记录写到对方的记录之上。保存前请核对两列。
conflict-field-by-field = 逐字段对照
conflict-theirs = { $who } 改成的值
conflict-yours = 您改成的值
conflict-unchanged-by-you = 您未改动
conflict-unchanged-by-them = 对方未改动
conflict-nothing-differs = 在本页显示的所有字段上，两个版本都与您开始时的那一版没有差别。版本号往前走了，说明有人保存了这条记录却没有改动其中的任何内容。
conflict-what-now = 接下来
conflict-reapply = 把您的版本重新应用到对方的版本之上
conflict-reapply-hint = 这是您的编辑，已挪到版本 { $version } 之上。请在此处改动，保留您想要的 { $who } 的成果，然后保存。对方的版本显示在下方，可从中抄录。
conflict-save-over = 把这一版保存到对方之上
conflict-discard = 丢弃我的改动，重新开始
conflict-their-version = { $who } 的版本，即目前的状态
conflict-history-of = 这条{ $kind }的历史

## 导入

convert-title = 导入家族文件
convert-submit = 导入
convert-result-title = 导入报告
convert-download = 下载档案

## 日期

date-unknown = 日期不详
date-not-recorded = 未记录
date-circa = 约{ $date }
date-between = { $from }至{ $to }之间
date-before = { $date }之前
date-after = { $date }之后
date-preserved = 原记为「{ $text }」
date-day-month-year = { $year }年{ $month }月{ $day }日
date-month-year = { $year }年{ $month }月
date-decade = { $decade }年代
date-century = { $century }世纪
date-quarter-century = { $century }世纪第{ $quarter ->
        [1] 一
        [2] 二
        [3] 三
       *[other] 四
    }个二十五年

## 其他错误页

error-back-to-start = 回到起点
error-payload-missing-title = 没有该文件
error-payload-missing-detail = 该文献的内容不在缓存中。
error-payload-unopenable-detail = 该文献的内容无法打开。
error-no-such-document-detail = 此处没有使用该标识的文献。
error-bad-preference-title = 不在可选项之内
error-bad-preference-detail = 这既不是本站提供的语言，也不是本站提供的外观。什么都没有改动。
error-unknown-kind-title = 未知类别
error-unknown-kind-detail = 「{ $kind }」不是记录的类别。本档案包含：{ $kinds }。
error-io-title = 无法保存
error-io-detail = { $error }。磁盘上没有任何改动。
error-upload-too-large = 该文件超过{ $mb } MB 的上限。没有保存任何内容，档案保持原样。
error-upload-refused = 该文献被拒绝：{ $reason }。档案保持原样。
error-back-to-person = 返回记录
error-no-such-person-to-attach = 此处没有使用该标识的人，因此没有可以附加文献的对象。
error-upload-title = 该上传未被保存
error-download-expired-title = 该下载已过期
error-download-expired-detail = 一次导入保留十五分钟，之后即丢弃。请重新导入该文件。
error-upload-none = 没有上传任何文件。请先选择一个文件。
error-upload-unsupported = 档案不保存这种类型的文件。可接受图像、PDF、纯文本、音频和视频；类型是从文件自身的字节读出的，因此给可执行文件改名并不管用。SVG 一律拒绝，因为 SVG 可以携带脚本。
error-export-unreadable-title = 无法读取导出的档案
error-export-unreadable-detail = { $error }

## 家谱页（续）

tree-title-suffix = 家谱
tree-back-to-focused = 回到以一个人为中心的视图
tree-show-all = 显示全部{ $n }人
tree-width-notice = 此视图宽{ $width }像素——在1500像素的屏幕上相当于{ $screens }屏的横向滚动。
tree-confidence-label = 可信度：
tree-band-certain = 确定
tree-band-high = 高
tree-band-medium = 中
tree-band-low = 推测
tree-counts = { $total }人中的{ $drawn }人 · { $generations }代
tree-unplaced-count = { $n }人未归位
tree-contradicts-title = 这棵家谱自相矛盾。
tree-contradicts-detail = 任何行的排布都无法满足这一点，因此下面这段关系被排除在代数编号之外，有些行可能画错了位置。请修正两条记录中出错的那一条。
tree-contradicts-pair = 同时被记为夫妻和亲子：
tree-contradicts-more = 另有{ $n }处矛盾未列出。
tree-no-people = 这棵家谱里还没有人。
tree-no-people-cta = 导入一个家族文件，或者添加第一个人。
tree-nobody-selected = 按这个选择没有可绘制的人。
tree-nobody-selected-cta = 从默认视图开始。
tree-edge-union = 已记录的婚姻关系
tree-edge-parentage = 已记录的亲子关系

## 首页

home-empty = 还没有记录任何内容。导入一个家族文件以迁入已有的家谱，或者手动添加第一个人。
home-count = 记录{ $total }条，装在一个属于这个家族的文件里。
home-browse = 浏览家谱
home-convert = 导入家族文件
home-unnamed-family = 这棵家谱
home-in-this-tree = 这个家族到目前为止记录了什么
home-showcase-title = 这棵家谱已经说出的，不止是姓名和日期
home-showcase-example = 看一个例子 →
home-nothing-title = 还没有可展示的内容。
home-nothing-detail = 导入一个家族文件以迁入已有的家谱，或者从零开始，自己添加第一个人。

## 概览卡片

showcase-links-title = 家族之外的关系{ $n }条
showcase-links-detail = 教父母、雇主、见证人和师长，各有自己的日期、来源，以及您对它的可信度。
showcase-occupations-title = 有起止的职业{ $n }项
showcase-occupations-detail = 「小学教师，1948–1978」保留了它的长度，画成横跨若干年的一道条，而不是一行带日期的记载。
showcase-uncertain-title = 保留原有不确定程度的日期{ $n }个
showcase-uncertain-detail = 约、之前、之后、之间，仍是四种不同的说法。来源无法确定的日期，绝不会被显示成好像它确定了一样。
showcase-preserved-title = 按原文措辞保留的日期{ $n }个
showcase-preserved-detail = 没有人能读作日期的措辞，会照原样保留，而不是被悄悄丢弃。
showcase-sources-title = 记录了可靠性的来源{ $n }条
showcase-sources-detail = 其中原始史料{ $primary }条。每个事实都显示它依据的是什么证据，以及那份证据有多强。
showcase-places-title = 疆界变动过的地点{ $n }处
showcase-places-detail = 一座城镇在不同时代可以属于不同的国家，记录会说明哪个时期属于哪一国。

## 记录细目

record-also-recorded-as = 另记作
record-borders-moved = 疆界变迁：
record-display-name = 显示名
record-read-as = 读作
record-note = 笔记
record-living-yes = 在世
record-deceased = 已故
record-centre-tree-here = 以此为家谱中心
record-centre-tree-title = 移动家谱，使其以此人为中心
record-open-full-page = 打开完整页面 ↗
record-open-full-title = 打开可单独分享的页面
record-edit = 编辑
panel-empty = 选择一张卡片，即可在此看到那个人的完整记录。
person-see-in-tree = 在家谱中查看此人
person-visibility-inline = 可见范围：

## 操作结果

result-diagnostics = 诊断信息
result-diagnostics-note = 库返回的每一条诊断信息，包括没有阻断操作的警告。没有任何一条被过滤掉。
result-no-diagnostics = 库没有返回诊断信息。
result-continue = 继续
result-dashboard = 概览
person-sections-label = 本页的各节

## 记录各节（细目）

record-notes-title = 关于这条记录值得一提的是：
record-name = 姓名
record-type = 类别
record-cause = 死因：
record-as = 身份
record-partner-not-recorded = 配偶或伴侣未记录
record-union-from = 始于
record-union-at = 地点
record-union-until = 止于
record-occupation-from = 始于
record-occupation-until = 止于
record-source-reliability = 可靠性
record-source-supports = 支持
record-photographs = 照片
record-documents = 文献
record-file = 文件
record-status = 状态
record-size = 大小
record-absent-document = 此人提到过，但并未保存在此。
record-no-file = 无文件
record-attach-document = 附加文献
record-doc-photo = 照片
record-doc-certificate = 证书
record-doc-letter = 书信
record-doc-record = 档案记载
record-doc-newspaper = 报纸
record-doc-other = 其他
record-upload = 上传
record-upload-help = 每个文件最多{ $mb } MB。附件保存在家谱旁边，导出时写回档案，因此一张照片会随它所属的家族一同移动。文件类型从其自身内容读出，而不是文件名：可接受图像、PDF、纯文本、音频和视频。SVG 会被拒绝，因为 SVG 可以携带脚本。
record-upload-help-short = 最多{ $mb } MB。SVG 会被拒绝。
record-verbatim-note = 完全按记录原样保留，因为没有任何转换器能解读它。
record-file-to-attach = 要附加的文件
record-document-type = 文献类别
record-caption = 说明
record-caption-placeholder = 说明（可选）
record-history-entry-meta = —— { $at }
record-history-entry-version = ，{ $version }

## 记录类别

kind-person = 人物
kind-family = 家庭
kind-event = 事件
kind-link = 关联
kind-occupation = 职业
kind-source = 来源
kind-place = 地点
kind-document = 文献

kind-person-plural = 人物
kind-family-plural = 家庭
kind-event-plural = 事件
kind-link-plural = 关联
kind-occupation-plural = 职业
kind-source-plural = 来源
kind-place-plural = 地点
kind-document-plural = 文献

## 列表

list-matching = 匹配{ $total }条，每页{ $per_page }条。
list-filter-placeholder = 按名称或标识筛选
list-filter = 筛选
list-clear = 清除
list-summary = 摘要
list-id = 标识
list-actions = 操作
list-nothing = 这里什么都没有。
list-nothing-matching = 这里没有与「{ $q }」匹配的内容。
list-delete-confirm = 要删除这条{ $kind }吗？请选择引用它的记录该如何处理：
list-policy-reject = 拒绝
list-policy-reject-detail = —— 只要还有引用就拒绝删除。什么都不会丢失。
list-policy-cascade = 级联
list-policy-cascade-detail = —— 删除它，并真正移除指向它的每一处引用。
list-policy-orphan = 置空引用
list-policy-orphan-detail = —— 删除它，但保留引用它的记录，只把引用置空。

## 记录的完备程度

completeness-dates-title = 按日期实际具有的形态分类
completeness-no-dates = 尚未记录任何日期。
completeness-dates-note = 有人能精确到某一天的日期，和只能落到某个十年的日期，是两种不同的说法，两者都按给出的样子保存。完全无法读作日期的文字，会一字不改地保留，而不是被丢弃。
completeness-shape-exact = 确切
completeness-shape-exact-note = 精确到某一天
completeness-shape-approximate = 约略
completeness-shape-approximate-note = 「约」，或只有年份或年代
completeness-shape-ranged = 有范围
completeness-shape-ranged-note = 之前、之后或之间
completeness-shape-preserved = 保留原文
completeness-shape-preserved-note = 无法解读的文字，原样保存
completeness-shape-unknown = 不详
completeness-shape-unknown-note = 记为不详

## 导入页

convert-page-title = 导入家族文件
convert-lede = 从 GEDCOM 文件迁入已有的家谱——GEDCOM 是大多数家谱软件导出的格式。这里不保存任何内容，本站已经展示的家谱也原封不动。
convert-file-label = 家族文件（.ged）
convert-file-hint = 最多{ $mb } MB。767人的家谱大约 320 KB。
convert-confidence-label = 这些事实起初有多可信
convert-confidence-hint = 待导入的文件没有说任何人当初有多确定，因此每个事实都需要一个起点。匆匆拼凑的家谱设低一些，依据文献做过功课的设高一些。这个数字的诚实读法是「导入之后无人复核」——之后您可以逐条上调或下调。
convert-lang-label = 地名所用的语言
convert-lang-hint = 类似 en、fr 或 zh 的标记。

## 导入报告

convert-failed = 导入没有完成
convert-try-another = 试试别的文件
convert-converted = 已导入 { $filename }
convert-result-lede = 记录{ $total }条，{ $size } KB。全部以{ $confidence }的可信度进入，地名按{ $lang }解读。本站展示的家谱未被触动。
convert-produced = 迁过来的内容
convert-skipped-title = 无法读取的条目{ $n }项
convert-skipped-note = 这些条目里没有可以迁移的内容。
convert-other-diagnostics = 另有{ $n }件值得知道的事
convert-clean = 没有任何内容留在后面——文件中的每一个条目都迁了过来。
convert-download-title = 下载
convert-download-named = 下载 { $name }
convert-download-note = 在此保留十五分钟，随后丢弃，请现在就下载。
convert-another = 导入另一个文件
admin-history-on = 于
admin-history-meta = —— { $kind }，{ $at }
admin-validation-counts = 错误{ $errors }处，警告{ $warnings }处，提示{ $infos }处。
admin-warnings-never-block = 警告从不阻断——它们是信息，不是关卡。
admin-validator-clean = 校验没有报告任何问题。
record-occupations-help-undated = 职业记录时带起止，几段职业便能落在同一条时间轴上比较。这份档案有职名却没有日期——导入之后这很常见，因为大多数家族文件没有地方存放它们——所以还没有可以画出的刻度。
record-occupations-help-axis = 职业是有持续时间的状态，不是发生在某一天的事件。所有时间段共用一条轴，{ $from }–{ $to }。
admin-value-not-set = 未设置
admin-validation-report = 校验报告
admin-dedup-complete = 重复项合并完成
admin-dedup-refused = 重复项合并被拒绝
record-birth-order = 出生次序
record-start-not-recorded = 起始未记录
record-end-not-recorded = 结束未记录
record-document-no-file = 文献已在此记录，但文件本身并不在
panel-selected-person = 选中的人物

## 世代带

tree-band-generation = 第{ $g }代
tree-band-people = { $n }人
tree-band-unplaced = 未归位
tree-band-unplaced-note = 不属于任何家庭的{ $n }人 —— 予以显示而非略去

## 受控词汇

gender-M = 男
gender-F = 女
gender-NB = 非二元
gender-unrecorded = 未记录

name-part-given_name = 名
name-part-family_name = 姓
name-part-patronymic = 父称
name-part-matronymic = 母称
name-part-middle_name = 中间名
name-part-nickname = 别号
name-part-prefix = 前缀
name-part-suffix = 后缀
name-part-particle = 连词
name-part-part = 组成部分

name-type-primary = 主名
name-type-other = 其他
name-type-alias = 别名
name-type-birth = 本名
name-type-married = 婚后姓
name-type-religious = 教名
name-type-transliteration = 转写
name-type-nickname = 别号

## 记录旁的注记

note-links = 家族之外的关系{ $n }条，各有自己的日期和来源
note-occupations = 带起止记录的职业{ $n }项
note-birth-imprecise = 来源无法确定的出生日期，按记录原样显示
note-death-imprecise = 来源无法确定的去世日期，按记录原样显示
note-names = 已记录的姓名{ $n }个
note-transliteration = 姓名保留原有文字，旁附拉丁转写
note-witnessed = 作为见证人而非当事人参与的事件{ $n }件

visibility-public = 公开
visibility-members = 家族成员
visibility-contributors = 编辑者
visibility-private = 私密

## 管理列表的行标题

family-label-couple = { $children ->
        [0] { $a }与{ $b }
       *[other] { $a }与{ $b } —— 子女{ $children }人
    }
family-label-half = { $children ->
        [0] { $a }与{ $unknown }
       *[other] { $a }与{ $unknown } —— 子女{ $children }人
    }
family-label-children = { $others ->
        [0] { $first } —— 父母未记录
       *[other] { $first }等兄弟姐妹共{ $others }人 —— 父母未记录
    }
family-label-empty = 没有记录任何人的家庭

event-label = { $category } —— { $who }，{ $date }
event-label-nobody = { $category } —— { $date }
event-two-people = { $a }与{ $b }
event-more-people = { $a }、{ $b }等共{ $others }人

link-label = { $label }：{ $from } → { $to }
occupation-label = { $who } —— { $title }
source-label = { $title } —— { $reliability }
source-label-plain = { $title }
document-label = { $filename } —— { $type }
document-label-untitled = 无题{ $type }
list-unnamed = 未命名的{ $kind }

## 列表中使用的规范词汇

event-category-birth = 出生
event-category-death = 去世
event-category-marriage = 婚姻
event-category-divorce = 离婚
event-category-baptism = 洗礼
event-category-burial = 安葬
event-category-immigration = 迁入
event-category-emigration = 迁出
event-category-census = 人口普查
event-category-residence = 居住
event-category-military = 兵役
event-category-education = 教育
event-category-other = 事件

reliability-primary = 原始史料
reliability-secondary = 二手史料
reliability-tertiary = 三手史料
reliability-recollection = 口述回忆
reliability-derivative = 衍生著作
reliability-authored = 著述作品
reliability-oral = 口述传统
reliability-unknown = 可靠性不详

document-type-photo = 照片
document-type-certificate = 证书
document-type-letter = 书信
document-type-record = 档案记载
document-type-newspaper = 剪报
document-type-other = 文献

## 这条记录还能多说些什么

completeness-title = 这棵家谱还能多说些什么
completeness-intro = 哪些已经记录，哪些仍是空的。
completeness-import-title = 导入带来了什么
completeness-import-intro = 依据您刚才上传的文件统计。空着的一行是原文件本就没有记录的内容，不是导入弄丢的内容。

completeness-headline-full = 下面每一类细节，在这棵家谱中都有记录。
completeness-headline-empty = 下面{ $total }类细节都还没有被记录。每一类都是记录可以多说一些的地方。
completeness-headline-partial = 下面已记录{ $carried }类，仍有{ $empty }类是空的。

completeness-metric-confidence = 每个事实有多确定
completeness-metric-confidence-none = 这里{ $slots }个事实，没有一个说明自己有多确定。从证书上抄下来的日期和猜出来的日期看上去一样，直到有一天不一样为止。
completeness-metric-confidence-uniform = { $slots }个事实中有{ $with }个带有分值，而且每一个都是同一个数（{ $modal }）。这正是批量导入留下的痕迹：一个无人回头复核的占位值，还没有哪一个被单独判断过。
completeness-metric-confidence-some = { $slots }个事实中有{ $with }个带有分值。其中{ $modal_count }个共用同一个值（{ $modal }），另有{ $assessed }个与之不同，说明它们被逐一看过。
completeness-metric-confidence-many = { $slots }个事实中有{ $with }个带有分值，其中{ $assessed }个与最常见的值（{ $modal }）不同，分布在{ $distinct }个层级上。这棵家谱记录的是真实而有差别的不确定。

completeness-metric-parentage = 每一段亲子关系有多确定
completeness-metric-parentage-none = 这里的亲缘关系，没有一段说明自己有多确定。收养、有争议的世系、仅凭一处提及重建的关系，恰恰是一个家族需要把疑问写下来的地方——而家谱会用更淡的线画出不那么确定的关系。
completeness-metric-parentage-some = 有{ $n }段亲缘关系带着自己的分值，因此推测出来的线明显比有据可查的线更弱。

completeness-metric-links = 血缘与婚姻之外的关系
completeness-metric-links-none = 教父母、雇主、见证人、师长、监护人。目前一条都没有记录。每一条都可以带上自己的日期、来源，以及您的可信度。
completeness-metric-links-some = 已记录{ $n }条，每条都有自己的日期、来源，以及您的可信度。

completeness-metric-occupations = 带起止记录的职业
completeness-metric-occupations-none = 没有记录任何职业。一门做了三十年的手艺，比一行带日期的记载更能说明一生。
completeness-metric-occupations-undated = 已记录职业{ $total }项，但没有日期。补上起止，它们就能在同一条时间轴上并排比较。
completeness-metric-occupations-some = { $total }项中有{ $span }项带有起点或终点，因而可以在同一条时间轴上并排比较。

completeness-metric-sources = 标注了可靠性的来源
completeness-metric-sources-none = 没有记录任何来源。写明一个事实从何而来，才使得日后亲属可以核实它——或者提出异议并说明理由。
completeness-metric-sources-some = { $total }条中有{ $graded }条说明了自己的分量，因此依据出生证书的说法，与依据回忆的说法，看上去明显不是一回事。

completeness-what-is-recorded = 记录能说明什么
completeness-in-this-tree = 在这棵家谱中
completeness-not-yet = 尚未记录

## 事件参与者的身份

role-spouse = 配偶
role-spouse_1 = 第一配偶
role-spouse_2 = 第二配偶
role-subject = 当事人
role-participant = 参与者
role-witness = 见证人
role-officiant = 主礼人
role-informant = 申报人
role-godparent = 教父母
