# axgf-cms — 界面文本，简体中文。
#
# 机器质量 — 未经母语者审校。族谱专用词汇（union、affiliation、confidence）
# 有约定俗成的对应译法，且因各地档案传统而异，此处可能有误。欢迎指正 —
# 参见 CONTRIBUTING.md。
#
# 中文没有单复数变化，因此下列复数选择器只有 other 一支。这不是遗漏：
# CLDR 对中文的规则就是如此，把英文的“一个 / 多个”逻辑套上来才是错的。
#
# 规则：本文件只翻译界面。姓名、地名、备注与职业均来自 .axgf 文件，
# 始终以其本身的语言和文字显示。

app-name = axgf-cms

nav-tree = 家谱图
nav-convert = 转换 GEDCOM
nav-admin = 管理
nav-sign-in = 登录
nav-sign-out = 退出
footer-served-from = 由单个 .axgf 文件提供。全部族谱逻辑位于 axgf-rs；格式由 axgf-spec 规定。

prefs-title = 语言与外观
prefs-language = 语言
prefs-language-note = 此设置只改变界面。姓名、地名和备注始终以其本身的语言和文字显示。
prefs-theme = 外观
prefs-apply = 应用
prefs-reviewed = 已审校
prefs-machine = 机器翻译，{ $coverage }%

theme-light = 浅色
theme-dark = 深色
theme-system = 跟随系统
theme-high-contrast = 高对比度
theme-sepia = 棕褐
theme-deuteranopia = 绿色盲
theme-protanopia = 红色盲
theme-tritanopia = 蓝色盲
theme-colour-blind-note = 色觉障碍友好
theme-contrast-note = 最高对比度

tree-title-around = { $name } 的周边
tree-title-whole = 整棵家谱
tree-lede-focused = { $ancestors ->
       *[other] { $ancestors } 位祖先
    }、{ $descendants ->
       *[other] { $descendants } 位后代
    }和{ $spouses ->
       *[other] { $spouses } 位配偶
    }，上下各 { $depth } 代。最年长者在下方。连线的不透明度表示关系的确信程度——线条越淡，记录越不确定。
tree-lede-whole = 文件中的所有人。最年长者在下方，最年轻者在上方。连线的不透明度表示关系的确信程度。
tree-filter-label = 筛选可见卡片
tree-filter-placeholder = 输入姓名…
tree-centre-on = 以此人为中心
tree-depth = 上下各几代
tree-show = 显示
tree-hidden-notice = { $n ->
       *[other] 有 { $n } 人只显示轮廓，不显示详情
    }
tree-hidden-because-role = ，因为其可见性高于您的账户可读取的范围。
tree-hidden-because-anonymous = ，因为他们并非公开。
tree-hidden-sign-in = 如果您有账户，请登录。
tree-restricted-card = 此人的记录对您不可见
tree-empty = 此文件中没有可绘制的人物。
tree-unplaced = 不属于任何已记录的家庭

record-identity = 身份
record-life-events = 生平事件
record-family = 家庭
record-other-relationships = 其他关系
record-occupations = 职业
record-places = 地点
record-sources-documents = 来源与文献
record-notes = 备注
record-history = 修改历史
record-raw = 原始记录
record-raw-summary-note = 构建此页面所用的 JSON
record-sources-documents-help = 每条来源都列出本页中依据它的事实，按证据强弱排序。
record-notes-help = 关于此记录的备注，包括任何转换器都无法解析、因而原样保留而非丢弃的文本。
record-help-toggle = 本节显示的内容

record-gender = 性别
record-living = 在世
record-visibility = 可见性
record-yes = 是
record-no = 否
record-name-type = 姓名类型
record-name-used = 使用期间
record-name-evidence = 证据
record-transliteration = 拉丁转写
record-born = 出生
record-died = 去世
record-parents = 父母
record-siblings = 兄弟姐妹
record-children = 子女
record-unknown-person = [不详]
record-restricted-person = 不公开
record-restricted-title = 此人的记录对您不可见
record-absent-person-title = 此文件提及但未收录此人
record-confidence = 确信程度
record-source = 来源
record-download = 下载

access-restricted-title = 对您不可见
access-restricted-anonymous = 此记录并非公开。请登录以确认您的账户是否可以读取。
access-role-title = 您的角色无权访问
access-role-write = 您的账户可以读取此文件，但不能修改。管理员可以将您的角色提升为贡献者。
access-scope-title = 超出您的分支

error-not-found-title = 未找到
error-not-found-detail = 此文件中不存在该页面。
error-no-such-person-title = 无此人
error-no-such-person-detail = 此文件中没有该标识符对应的人物。
error-no-such-entity-title = 无此记录
error-no-such-entity-detail = 此文件中没有该标识符对应的记录。
error-deleted-while-editing = 此文件中没有该标识符对应的记录。可能在您编辑期间已被删除。
error-no-such-file-title = 无此文件
error-not-an-image-title = 并非图像
error-not-an-image-detail = 此文献没有缩略图，因为它不是本版本能够解码的图像。
error-back = 返回

login-title = 登录
login-lede = 账户由管理员创建。
login-username = 用户名
login-password = 密码
login-submit = 登录
login-wrong = 用户名与密码不匹配。
login-token-wrong = 该令牌不正确。
login-throttled = 失败尝试过多。请等待几分钟后重试。
login-no-accounts-title = 此安装尚无任何账户。
login-emergency-summary = 应急访问
login-emergency-label = 应急令牌
login-emergency-submit = 使用应急令牌
login-sign-in-prompt = 请登录以进入管理面板。

admin-title = 管理
admin-entities = 记录
admin-create = 新建
admin-new-kind = 新建：{ $kind }
admin-operations = 操作
admin-validate = 校验
admin-deduplicate = 合并重复项
admin-export = 导出文件
admin-accounts = 账户
admin-dedup-confirm = 合并重复项会归并记录并重写文件。是否继续？
admin-recent-changes = 最近的修改
admin-sessions-open = { $n ->
       *[other] 当前有 { $n } 个会话。
    }
admin-no-changes-yet = 尚未通过本应用做过任何修改。此后每次保存都会记入 { $path }。
admin-last-validation = 上次校验
admin-fields = 字段
admin-raw-json = 原始 JSON
admin-save = 保存
admin-cancel = 取消
admin-delete = 删除
admin-not-set = — 未设置 —
admin-edit = 编辑
admin-page-of = 第 { $page } 页，共 { $pages } 页
admin-previous = 上一页
admin-next = 下一页
admin-saved = 已保存为版本 { $version } — { $summary }
admin-not-saved = 未保存
admin-created = 已创建
admin-not-created = 未创建
admin-deleted = 已删除
admin-not-deleted = 未删除 — 文件未改动
admin-what-changed = 改动内容
admin-field = 字段
admin-from = 原值
admin-to = 新值
admin-version = 版本 { $version }

accounts-title = 账户
accounts-existing = 现有账户
accounts-username = 用户名
accounts-role = 角色
accounts-status = 状态
accounts-branch = 分支
accounts-last-seen = 最近登录
accounts-change = 修改
accounts-you = （您）
accounts-active = 启用
accounts-disabled = 已停用
accounts-never = 从未
accounts-whole-tree = 整棵家谱
accounts-roots = { $n ->
       *[other] { $n } 个根节点
    }
accounts-add = 添加账户
accounts-password-hint = 留空则自动生成并只显示一次。若自行设置，至少 { $min } 个字符。
accounts-new-password-placeholder = 新密码（留空则不变）
accounts-email = 电子邮件
accounts-optional = （可选）
accounts-create = 创建账户
accounts-role-viewer = 读者 — 可读公开与成员可见的记录
accounts-role-contributor = 贡献者 — 另可新建、编辑与上传
accounts-role-admin = 管理员 — 另可管理账户、删除与导出
accounts-branch-placeholder = 每行一个人物标识符
accounts-ids-in-bundle = 此文件中的人物标识符
accounts-created = 已创建 { $username }。
accounts-updated = 已更新 { $username }。其所有会话均已退出。
accounts-username-taken = 该用户名已被占用。
accounts-pick-role = 请选择一个角色。
accounts-no-such = 无此账户。
accounts-not-saved = 未保存：{ $error }

conflict-title = 他人先于您修改了此记录
conflict-versions = 您从版本 { $expected } 开始编辑；文件现为版本 { $current }。
conflict-both-changed = 你们二人都改动了这些字段
conflict-both-changed-detail = 这些字段你们二人都编辑过。您保存的内容将覆盖 { $who } 所填的内容：
conflict-field-by-field = 逐字段对照
conflict-theirs = { $who } 改成的内容
conflict-yours = 您改成的内容
conflict-unchanged-by-you = 您未改动
conflict-unchanged-by-them = 对方未改动
conflict-what-now = 接下来
conflict-reapply = 将您的版本覆盖到对方版本之上
conflict-save-over = 以此覆盖对方的版本
conflict-discard = 放弃我的修改并重新开始
conflict-their-version = { $who } 的版本，即文件当前所存
conflict-history-of = 此记录（{ $kind }）的修改历史

home-why-title = 为何选择 AXGF

convert-title = 将 GEDCOM 转换为 AXGF
convert-submit = 转换
convert-result-title = 转换结果
convert-download = 下载 .axgf 文件

completeness-title = 文件完整度
completeness-empty = 空白
completeness-spec-field = AXGF 字段

## Dates

date-unknown = 日期不详
date-not-recorded = 未记录
date-circa = 约 { $date }
date-between = { $from } 至 { $to } 之间
date-before = { $date } 之前
date-after = { $date } 之后
date-preserved = 原文记作“{ $text }”
date-day-month-year = { $year } 年 { $month } { $day } 日
date-month-year = { $year } 年 { $month }
date-decade = { $decade } 年代
date-century = { $century } 世纪

month-1 = 一月
month-2 = 二月
month-3 = 三月
month-4 = 四月
month-5 = 五月
month-6 = 六月
month-7 = 七月
month-8 = 八月
month-9 = 九月
month-10 = 十月
month-11 = 十一月
month-12 = 十二月
