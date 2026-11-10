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
account-emergency = 紧急令牌
footer-credit = © 2026 · 由 { $author } 设计与开发
footer-format-link = GitHub 上的 AXGF 格式

## 偏好设置

settings-title = 设置
edit-mode-label = 编辑模式 — 保存的内容会更改家族记录
display-menu-label = 语言和显示
display-settings = 主题和显示…
settings-tab-theme = 配色
settings-tab-language = 语言
settings-tab-appearance = 外观
settings-done = 完成
prefs-language = 语言
prefs-theme = 配色
prefs-background-on = 页面背后一层淡淡的色晕
prefs-apply = 应用
busy-saving = 正在保存…
busy-working = 处理中…
prefs-machine = 机器翻译，{ $coverage }%
prefs-machine-complete = 已译全，尚未校订

theme-canopee = 林冠
theme-canopee-note = 深色 · 森林绿 · 默认
theme-fusion = 融合
theme-fusion-note = 深色 · 霓虹 · 一棵活的树
theme-galaxy = 星系
theme-galaxy-note = 深空 · 一个太阳系
theme-summit = 顶峰
theme-summit-note = 深色，金色黎明 · 一次攀登

## 家谱

tree-title-around = { $name }的周边
tree-title-whole = 整棵家谱
tree-lede-focused = 祖先{ $ancestors }人、后代{ $descendants }人、配偶或伴侣{ $spouses }人，每个方向{ $depth }代。
tree-filter-label = 筛选可见的卡片
tree-depth = 每个方向的代数
tree-show = 显示
tree-hidden-notice = 有{ $n }人只显示姓名而不显示细节
tree-hidden-because-role = ，因为其可见范围高于您的账户所能读取的范围。
tree-hidden-because-anonymous = ，因为他们并未公开。
tree-hidden-sign-in = 如果您有账户，请登录。
tree-restricted-card = 此人的记录对您不可见
tree-empty = 还没有可以绘制的人。

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

record-identity-help = 每一个已记录的姓名，连同其类别、使用的时期和支持它的来源；原文文字与拉丁转写不同的，并列显示。此外还有性别、是否在世和可见范围。
record-life-events-help = 出生、去世，以及此人参与的每一件事，按日期排列，各自标明其身份——这样一场他只是见证的婚礼，会与他自己的婚礼并排出现。没有日期的事实排在最后，而不是假装排在最前。
record-family-help = 父母与兄弟姐妹，然后是每一段婚姻关系的类别、日期、地点、结束方式，以及按出生顺序排列的子女。
record-other-relationships-help = 每一条以此人为一端的关联，都从他这一侧来读：同一条记录从一端看是「某人的教父」，从另一端看是「某人的教子」。
record-places-help = 这条记录触及的每一个地点，连同在那里发生的事，以及使一个地方在不同时代仍可理解的疆界变迁。
record-sources-documents-help = 每条来源都列出本页中依赖于它的事实，按证据的强弱排列。
record-notes-help = 关于这条记录的笔记，包括任何转换器都无法解读、因而原样保留而非丢弃的文字。
record-history-help = 这条记录每一次保存的修改，最新的在前。谁改了什么，是关于维护家谱的人的事实，而不是关于其中家族的事实，因此它留在导出的档案之外，只向已登录的亲属显示。
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
conf-percent = { $percent }%
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
error-merged-title = 此记录已不存在。
error-merged-detail = 它已于{ $date }并入{ $name }。指向旧地址的链接现在通向合并后的记录。
error-merged-open = 打开合并后的记录
error-back-to-river = 返回河流
view-tree-back = 返回生命之树
view-galaxy-back = 返回星系
view-summit-back = 返回山峰
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
setup-title = 启用此档案
setup-lede = 此安装中还没有任何家族。请选择从何开始。
setup-door-archive-title = 从 AXGF 档案开始
setup-door-archive-detail = 来自另一个安装或备份的 .axgf 文件。其中的人物、来源和照片将原样迁入。
setup-door-archive-file = 档案（.axgf）
setup-door-archive-go = 用此档案启用
setup-limit-gb = 最多 { $n } GB，直接写入磁盘。
setup-door-scratch-title = 从零开始
setup-door-scratch-detail = 一个空档案。先添加第一个人，再逐条添加其家族。
setup-door-scratch-go = 添加第一个人
setup-door-gedcom-title = 从 GEDCOM 文件开始
setup-door-gedcom-detail = 任何家谱软件都能导出的文件：GEDCOM 5.5、5.5.1、5.5.5 或 7.0，.ged 或 .gdz 格式，任何编码均可。在保存任何内容之前，您会看到它是如何读取的、哪些内容会迁入。
setup-door-gedcom-file = GEDCOM 文件（.ged、.gdz）
setup-door-gedcom-go = 读取此文件
setup-never-over = 已有家族时，此页面只会在完成经过验证的备份并输入确认词之后才替换它。
setup-adopt-title = 将其作为档案的家族
setup-adopt-detail = 将成为此安装家族的记录：{ $total }。在您按下按钮之前，不会保存任何内容。
setup-adopt-go = 用此家族启用档案
setup-adopt-back = 选择其他方式
setup-done-title = 档案已启用
setup-done-detail = 此安装家族中现有记录：{ $total }
setup-done-photos = 随之迁入的照片和文件：{ $n }
setup-done-mismatches = 内容与记录的指纹不符的文件：{ $n }。已保留，请检查。
setup-done-river = 打开河流
view-tree-open = 打开生命之树
view-galaxy-open = 打开星系
view-summit-open = 打开山峰
setup-done-admin = 管理
setup-refused-title = 此内容无法成为档案
setup-refused-detail = 未写入任何内容。阻止它的原因：
setup-closed-title = 此档案已启用
setup-closed-detail = 其间已有家族到来，未写入任何内容。请重新打开导入页面：已有家族时，它会要求确认并先备份该家族。
setup-not-archive = 这不是 .axgf 档案。
setup-archive-too-large = 此档案超出了上传允许的大小。
admin-lede = 正在编辑 { $path } —— 记录{ $total }条，附件{ $files }个，磁盘占用{ $size }。每次改动都一次性写入；被拒绝的改动不会碰到文件。
admin-entities = 记录
admin-create = 新建
admin-new-kind = 新建：{ $kind }
admin-operations = 操作
admin-validate = 校验
admin-deduplicate = 合并重复
admin-export = 导出档案
admin-roles-note = 校验、合并重复、导出、删除和账户管理仅限管理员。
admin-dedup-confirm = 合并重复会融合记录并重写档案。是否继续？
admin-recent-changes = 最近的修改
admin-recent-note = 来自 { $path } 的{ $total }条已记录改动中的最近{ $shown }条。
admin-sessions-open = 当前有{ $n }个会话打开。
admin-no-changes-yet = 尚无修改。每次保存都会记录在 { $path }。
admin-last-validation = 上次校验
admin-bundle-heavy = 这份档案有{ $size }。它在启动时整体读入并常驻内存，因此超过约{ $warn } 之后，本站开始占用可观的内存，重启也会变慢。这样的设计适合家族档案而不是媒体库——如果附件无节制地增长，请把它们放进文件存储，让档案指向它们。

admin-fields = 字段
admin-raw-json = 原始 JSON
form-more-fields = 另外 { $n } 个字段
admin-raw-json-help = 整条记录，因此没有什么是不能编辑的——像家庭的配偶和子女列表、地点的疆界变迁这类数组就在这里。这是起始文档；上面的各字段随后会写到各自负责的路径上，所以同一个值请只在一处编辑，不要两处都改。它必须能按 JSON 解析，否则什么都不会保存。
admin-save = 保存
admin-cancel = 取消
place-editor-title = 编辑地点
place-names = 名称
place-name-primary = 主要
place-name-lang = 语言
place-name-value = 名称
place-names-hint = 每个记录的名称一行。曾受三个帝国管辖的地方有三个名称；主要名称是在其他各处显示的那个。
place-where = 位置
place-type = 类型
place-region = 地区
place-country-current = 今属国家
place-country-hint = ISO 3166-1 alpha-2，例如 PL、FR、DE。
place-country-history = 边界沿革
place-history-country = 国家
place-history-from = 起
place-history-until = 止
place-country-history-hint = 哪个国家在哪个时期管辖此地。这对家谱很重要：1880 年用俄文写成的记录与 1930 年用波兰文写成的记录，可能指的是同一个村庄。
place-coordinates = 坐标
place-lat = 纬度
place-lon = 经度
place-precision = 精度
place-identifiers = 标识符
place-wikidata = Wikidata
place-geonames = GeoNames
place-used-by = { $name } 还被另外 { $n } 条记录使用。
place-error-no-name = 地点至少需要一个名称。
place-error-coords-pair = 纬度和经度要一起填写：两者都填，或都不填。
place-error-coords-number = 纬度和经度必须是数字。
place-error-coords-range = 纬度范围为 -90 到 90，经度范围为 -180 到 180。
place-type-continent = 大洲
place-type-country = 国家
place-type-region = 地区
place-type-department = 省级行政区
place-type-city = 城市
place-type-village = 村庄
place-type-district = 城区
place-type-street = 街道
place-type-building = 建筑
place-type-farm = 农庄
place-type-island = 岛屿
place-type-historical = 历史地名
place-type-unknown = 未知
place-precision-exact = 精确
place-precision-building = 建筑
place-precision-street = 街道
place-precision-city_center = 城市中心
place-precision-region_center = 地区中心
place-precision-country_center = 国家中心
place-precision-approximate = 大致

place-coordinates-hint = 手工填写是常规做法。许多在旧政权治下记录的地名，用今天的检索根本查不到。
place-geocode-search = 检索该地名
place-geocode-hint = 把地名、地区和国家发给地理编码服务，一次一处。在你保存之前不会写入任何内容。
place-geocode-off = 地名检索已关闭。它需要一个联系地址，供该服务识别本次部署；用 --geocoder-contact 启动服务器即可开启。
place-geocode-query = 检索词： { $q }
place-geocode-error = 无法连接检索服务。上面的坐标栏仍然可用。
place-geocode-none = 未找到。对于在俄国、普鲁士或奥地利治下记录的村庄，这是常见结果；请手工填写位置。
place-geocode-not-a-place = 不是聚落
place-geocode-use = 采用此项
place-geocode-attribution = 结果来自 OpenStreetMap，经由 Nominatim 提供，采用 Open Database 许可。

place-paste = 粘贴一个位置
place-paste-placeholder = 地图链接，或 52.0782795, 21.2508068
place-paste-read = 读取
place-paste-hint = Google 地图或 OpenStreetMap 链接、geo: URI、一对数字，或形如 52°04'41.8"N 21°15'02.9"E 的度分秒。
place-paste-read-ok = 已读入上面的栏位。请核对后保存。
place-paste-unreadable = 这不是这里能读懂的位置。上面的栏位仍可直接填一对数字。

place-map-hint = 点击地图放置该点，或拖动图钉。作准的是上面的栏位。
place-map-clear = 清除该点
place-open-in-map = 在 OpenStreetMap 中查找此地，再把链接粘回来

person-tab-record = 记录
person-tab-life = 事件
person-tab-media = 媒体
person-tab-tree = 家系
person-tab-history = 历史
person-tab-afg = AI 家谱师
person-tab-status = 状态
status-check = 待核对
status-know = 须知
status-fix = 更正
status-open = 打开
status-dismissed = 已标记“对我们来说正常”：{ $n }
status-panel = 待核对：{ $n }
person-afg-title = 向 AI 家谱师询问{ $name }
person-afg-title-short = 与 AI 家谱师的对话
person-afg-lede = 询问{ $name }的生平、家庭与时代。您在这里的对话只属于您。
person-afg-reads = 每条消息都会读取：记录、照片（{ $photos }）、PDF（{ $pdfs }）、笔记（{ $notes }）、近亲（{ $relatives }）、家族（{ $persons }）· 约 { $tokens } 个词元。
person-afg-ask = 您关于{ $name }的问题
person-afg-example = 例如：“在笔记提到的那场战役中，{ $name }在哪个团？”
person-afg-conversations = 您的对话
person-afg-none = 还没有对话：请提出第一个问题。
person-afg-all = 您的全部对话
person-afg-back = 返回您的对话
person-afg-gone = 此对话不存在，或不属于您。
person-afg-delete-all = 删除您关于{ $name }的全部对话
person-afg-delete-all-confirm = 我要全部删除。
person-afg-deleted-all = 您关于此人的对话已删除。

admin-delete = 删除
admin-not-set = —— 未设置 ——
admin-edit = 编辑
list-select-row = 选择 { $name }
list-kind-label = 类型
list-select-page = 选择本页所有行
list-selected = 已选 { $n } 项
list-export = 导出
list-escape = 按 Esc 取消选择
bulk-confirm-title = 删除 { $n } 条记录？
bulk-confirm-detail = 仍被其他记录引用的记录会被拒绝，此时不会删除任何内容。
bulk-confirm-go = 删除
bulk-deleted = 已删除 { $n } 条记录。
bulk-refused = 未删除任何内容：{ $name } 仍在别处被引用。请先移除该引用，再重试。
bulk-changed = 列表显示后有 { $n } 条记录发生了变化，因此未删除任何内容。请重新加载列表后再试。
admin-range = { $from }–{ $to }，共 { $total }
list-filiation = 亲子关系
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
accounts-lede = 保存在 { $path }（权限 600），位于归档旁边，从不在其中。
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
accounts-no-registration = 访客不能自行开设账户；注册在上方设置。
accounts-password-hint = 留空：将生成密码并显示一次。否则至少 { $min } 个字符。
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
accounts-emergency-warning = 您正以紧急令牌登录，而不是以账户登录：请在下方创建真实账户并使用它。
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
upload-file-too-large = 该文件超过{ $mb } MB 的上限，不会发送。请选择较小的文件。
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
tree-contradicts-detail = 这些关系不参与世代编号，部分行可能位置有误。请更正出错的记录。
tree-contradicts-pair = 既是夫妻又是亲子：
tree-contradicts-more = 另有{ $n }处矛盾未列出。
tree-no-people = 这棵家谱里还没有人。
tree-no-people-cta = 导入一个家族文件，或者添加第一个人。
tree-nobody-selected = 按这个选择没有可绘制的人。
tree-nobody-selected-cta = 从默认视图开始。
tree-edge-union = 已记录的婚姻关系
tree-edge-parentage = 已记录的亲子关系

## 首页

home-empty = 尚无任何记录。
home-count = 记录{ $total }条，装在一个属于这个家族的文件里。
home-unnamed-family = 这棵家谱
home-in-this-tree = 这个家族到目前为止记录了什么
home-showcase-title = 这棵家谱已经说出的，不止是姓名和日期
home-showcase-example = 看一个例子 →
home-nothing-title = 还没有可展示的内容。
home-nothing-admin = 此安装中还没有任何家族：可以从档案、GEDCOM 文件或从零开始启用。
home-nothing-visitor = 此家族档案正在准备中。
home-eyebrow = 家族档案
home-hero-1 = 每一个生命，都是一处源头。
home-hero-2 = 每一个家庭，都是一条河流。
view-tree-hero-1 = 每一个生命，都是一片叶子。
view-tree-hero-2 = 每一个家庭，都是一棵树。
view-galaxy-hero-1 = 每一个生命，都是一颗星。
view-galaxy-hero-2 = 每一个家庭，都是一个星系。
view-summit-hero-1 = 每一个生命都是一步。
view-summit-hero-2 = 每一个家庭，都是一次攀登。
home-cta-river = 探索河流
view-tree-cta = 探索这棵树
view-galaxy-cta = 探索星系
view-summit-cta = 攀登山峰
home-cta-tree = 查看家谱树
home-scroll = 向下滚动浏览
home-trust-title = 亲子关系的可信程度
home-trust-certain = 几乎确定
home-trust-high = 依据充分
home-trust-medium = 较为可信
home-trust-low = 推测
home-trust-unrated = 尚未评估
home-showcase-eyebrow = 不止姓名与日期
home-cta-title = 让我们一起守护家族的记忆。
home-cta-detail = 每添加一个名字、每核实一处来源，这条河流对后来者都更加真实。
view-tree-cta-detail = 每添加一个名字、每核实一处来源，这棵树对后来者都更加真实。
view-galaxy-cta-detail = 每添加一个名字、每核实一处来源，这个星系对后来者都更加真实。
view-summit-cta-detail = 每添加一个名字、核实一个来源，都让这座山对后来的人更加真实。
home-cta-add = 添加人物

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
record-open-full-page = 打开完整页面 ↗
record-open-full-title = 打开可单独分享的页面
record-edit = 编辑
record-see-in-river = 在河流中查看
view-tree-see = 在生命之树中查看
view-galaxy-see = 在星系中查看
view-summit-see = 在山峰上查看
panel-empty = 选择一张卡片，即可在此看到那个人的完整记录。
person-visibility-inline = 可见范围：
person-age-at-death = { $n }岁去世
person-age-now = { $n }岁
person-born-in = 出生于{ $place }
person-died-in = 逝世于{ $place }
person-children-count = { $n }名子女
person-generations-below = 下有{ $n }代
person-portrait-of = { $name }的照片
person-no-portrait = 未记录照片

## 操作结果

result-diagnostics = 诊断信息
result-diagnostics-note = 库返回的每一条诊断信息，包括没有阻断操作的警告。没有任何一条被过滤掉。
result-no-diagnostics = 库没有返回诊断信息。
result-continue = 继续
result-dashboard = 概览
person-sections-label = 本页的各节

## Vocabulary the structured editors offer

name-part-nasab = 纳萨布（世系）
name-part-laqab = 拉卡布（称号）
name-part-kunya = 库尼耶（从子名）
name-part-nisbah = 尼斯拜（出身）
name-part-alias = 别名
name-part-religious_name = 教名
name-part-pen_name = 笔名
name-type-pen_name = 笔名
gender-U = 未记录

## 记录各节（细目）

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
record-clips = 影片与录音
record-documents = 文献
record-file = 文件
record-status = 状态
record-size = 大小
record-absent-document = 此人提到过，但并未保存在此。
record-upload-help-short = 最多{ $mb } MB。SVG 会被拒绝。
record-verbatim-note = 完全按记录原样保留，因为没有任何转换器能解读它。
record-file-to-attach = 要附加的文件
record-document-type = 文献类别
record-caption = 说明
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
completeness-dates-note = 日期按给出的样子保存，精确到日或到十年；不是日期的文字逐字保留。
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
convert-lede = 从 GEDCOM 文件导入族谱。不会存储任何内容，网站上的族谱保持不变。
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
panel-selected-person = 选中的人物

## 世代带

tree-band-generation = 第{ $g }代
tree-level-self = 同辈
tree-level-up = { $n ->
        [1] 子女
        [2] 孙辈
        [3] 曾孙辈
       *[other] 后代（第{ $n }代）
    }
tree-level-down = { $n ->
        [1] 父母
        [2] 祖父母
        [3] 曾祖父母
       *[other] 祖先（第{ $n }代）
    }
tree-card-centre = 以此为中心
tree-card-same = 同一个人，第二次绘出
# The tree page's legend: what a line to a child says (2026-10-28).
tree-key-lines = 连线：
tree-key-son = 儿子
tree-key-daughter = 女儿
tree-key-adoptive = 收养
tree-key-foster = 寄养或监护
tree-key-step = 继子女
tree-key-union = 婚姻
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

note-links = 家族以外的关系：{ $n }
note-occupations = 有日期的职业：{ $n }
note-birth-imprecise = 来源中的出生日期不精确
note-death-imprecise = 来源中的死亡日期不精确
note-names = 记录的姓名：{ $n }
note-transliteration = 原文书写的姓名，附拉丁转写
note-witnessed = 作为见证人的事件：{ $n }

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
completeness-metric-confidence-many = { $slots } 个事实中有 { $with } 个带有评分：{ $assessed } 个不同于 { $modal }，共 { $distinct } 个等级。

completeness-metric-parentage = 每一段亲子关系有多确定
completeness-metric-parentage-none = 这里的亲缘关系，没有一段说明自己有多确定。收养、有争议的世系、仅凭一处提及重建的关系，恰恰是一个家族需要把疑问写下来的地方——而家谱会用更淡的线画出不那么确定的关系。
completeness-metric-parentage-some = 带有自身评分的亲子关系：{ $n }

completeness-metric-links = 血缘与婚姻之外的关系
completeness-metric-links-none = 教父母、雇主、见证人、师长、监护人。目前一条都没有记录。每一条都可以带上自己的日期、来源，以及您的可信度。
completeness-metric-links-some = 已记录{ $n }条，每条都有自己的日期、来源，以及您的可信度。

completeness-metric-occupations = 带起止记录的职业
completeness-metric-occupations-none = 没有记录任何职业。一门做了三十年的手艺，比一行带日期的记载更能说明一生。
completeness-metric-occupations-undated = 已记录职业{ $total }项，但没有日期。补上起止，它们就能在同一条时间轴上并排比较。
completeness-metric-occupations-some = { $total } 个中有 { $span } 个有开始或结束时间。

completeness-metric-sources = 标注了可靠性的来源
completeness-metric-sources-none = 没有记录任何来源。写明一个事实从何而来，才使得日后亲属可以核实它——或者提出异议并说明理由。
completeness-metric-sources-some = { $total } 个中有 { $graded } 个注明了可靠程度。

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

phys-no-source = 无来源
phys-col-date = 何时
phys-col-source = 来源
phys-col-confidence = 可信度
phys-col-note = 备注
phys-field-height-cm = 身高
phys-field-weight-kg = 体重
phys-field-eye-colour = 眼睛颜色
phys-field-hair-colour = 头发颜色
phys-field-build = 体型
phys-field-handedness = 惯用手
phys-field-features = 显著特征
phys-field-military = 兵役
phys-field-languages = 会说的语言
phys-field-blood-group = 血型
phys-field-conditions = 已知疾病
phys-field-operations = 手术与外伤
phys-field-cause-of-death = 死因
phys-field-religion = 宗教或所属
phys-field-health-notes = 备注
admin-export-health-note = 普通导出不含任何敏感类别，也不含在世者的行为资料。请勾选此文件应包含的内容。
avatar-picker-title = 选择一张图片
avatar-choose = 用哪张图片代表这个人
avatar-mode-auto = 交由程序挑选
avatar-mode-auto-note = 第一张肖像；若没有，则取与此记录相连的第一张图片。
avatar-mode-none = 改为显示姓名首字母
avatar-mode-none-note = 适用于图片皆为文书而非人脸的记录。
avatar-focal-hint = 点击图片以选定，再点击应当留在画面中的部位——头像是方的，而多数扫描件不是。
avatar-no-images = 此记录尚未连接任何图片。
avatar-upload-title = 上传一张图片并使用
avatar-upload-button = 上传并用作图片
avatar-not-available-title = 该图片不可用
avatar-not-available-detail = 所选文件未与此人相连，或你无权读取。
ai-badge = AI
ai-portraits-title = AI 修饰的肖像
ai-portraits-band = 修饰
ai-portraits-upload = 上传 AI 修饰的图片
ai-portraits-manage = 管理 AI 图片
ai-portraits-intro = AI 根据此人的照片生成的图片：此人可能的样子。它们显示在记录中头像旁边，标有“AI”，只有已登录的成员能看到。它们绝不会成为此人的头像，绝不列入其文件，也绝不作为证据。
ai-portraits-current = 记录中的图片
ai-portraits-add = 添加图片——还可添加 { $n } 张
ai-portraits-files = 图片（PNG、JPEG、WebP 或 GIF），可一次选择多张
ai-portraits-generator = 生成工具
ai-portraits-from = 依据的照片
ai-portraits-from-none = 未说明
ai-portraits-remove = 移除
ai-portraits-alt = { $name } 的肖像，经 AI 修饰
ai-portraits-not-evidence = 由 AI 根据照片生成：并非当时的图像。
ai-portraits-full = 此记录已有三张 AI 图片。请先移除一张再添加。
ai-portraits-too-many = 每条记录最多三张 AI 图片。请先移除一张再添加。
ai-portraits-images-only = AI 肖像必须是图片：PNG、JPEG、WebP 或 GIF。
ai-portraits-stale = 此页面打开期间记录已被更改。未保存任何内容：请重新打开并再试一次。
ai-portraits-back = 返回 AI 图片

record-history-withheld = 对你不显示

## A death nobody wrote down
#
# GEDCOM cannot say "died, date unknown", so a converter marks those
# people living. These three say that the page worked it out rather than
# read it, which is a distinction this product does not blur.

record-life-status = 状态
record-presumed-deceased = 推定已故
record-presumed-short = 推定
record-presumed-why = 没有记录死亡，而出生已在 { $years } 年以前，因此这条记录不可能正确。档案未被更改：这是本页的推断，不是来源所述。

## The identity editor

identity-editor-title = 姓名与身份
identity-primary-name = 到处显示的名字
identity-primary-help = 家谱卡片、标题和所有列表使用的名字。下面的其他名字是某个来源在其他时候对这个人的称呼。
identity-display = 名字
identity-display-latin = 拉丁字母写法
identity-culture = 语言
identity-direction = 书写方向
identity-direction-ltr = 从左到右
identity-direction-rtl = 从右到左
identity-direction-auto = 由文本决定
identity-components = 名字的组成部分
identity-components-help = 哪部分是名、哪部分是姓，按书写顺序排列。没有分部的记录照样显示：分部是搜索可以匹配的内容。
identity-part = 部分
identity-value = 文字
identity-other-names = 其他名字
identity-other-help = 婚后姓、教名、后来的记录使用的名字。每一个都带有使用时间和依据的来源。
identity-name-type = 名字类型
identity-valid-from = 使用自
identity-valid-until = 使用至
identity-about = 关于此人
identity-living-help = 这是来源设置的标志。当出生年代过久时，页面会另行推定死亡，而该推定绝不会改变这个复选框或档案。
identity-error-no-display = 记录需要一个用于显示的名字。未保存任何内容。
editor-blank-to-remove = 清空名字即可删除此条目。
# A BCP 47 tag, shown as an example in the language field.
identity-edit-link = 编辑姓名与身份

## Union types, statuses and date precision, said out loud

union-type-marriage = 婚姻
union-type-civil_union = 民事结合
union-type-cohabitation = 同居
union-type-religious_only = 宗教结合
union-type-polygamous = 多偶
union-type-unknown = 未记录
union-role-spouse = 配偶
union-role-partner = 伴侣
union-role-husband = 丈夫
union-role-wife = 妻子
union-status-active = 持续中
union-status-ended_by_death = 因去世结束
union-status-ended_by_divorce = 因离婚结束
union-status-ended_by_separation = 因分居结束
union-status-annulled = 已废除
union-status-unknown = 未记录
union-status-ended = 已结束
union-status-ended-by = 因{ $reason }结束
union-reason-death_of_spouse = 配偶去世
precision-exact = 精确到日
precision-year = 精确到年
precision-month = 精确到月
precision-decade = 精确到十年
precision-century = 精确到世纪
precision-unknown = 未知
record-precision = 精确度
record-place = 地点

## The relationships editor

family-editor-title = 家庭与关系
family-unions = 结合
family-no-unions = 未记录此人的任何结合。
family-union-legend = 结合 { $n }
family-writes-family = 保存将修改两人共用的家庭记录 #{ $id }，另一个人的页面也会随之改变。
family-partners = 伴侣
family-partner = 伴侣
family-role = 角色
family-children = 子女
family-children-help = 出生顺序是记录自身的说法。留空即不作任何说明：取自行序的数字会是没人写下的事实。
family-child = 子女
family-birth-order = 出生顺序
family-the-union = 结合本身
family-type = 结合类型
family-status = 状态
family-started = 开始
family-ended = 结束
family-leave = 把此人移出这个结合
family-open-entity = 打开家庭记录
family-new-union = 新的结合
family-new-union-help = 这会创建一条包含此人的新家庭记录。伴侣可不填：记录中没有任何伴侣的父母，就是单人的结合。
family-create-union = 创建结合
family-parents = 父母
family-no-parents = 此人未被记录为任何家庭的子女。
family-child-of = 这个家庭的子女
family-detach-child = 把此人移出这个家庭
family-attach-parents = 关联到父母
family-attach-help = 选择此人所属的家庭。这会把他加入该家庭记录，因此也会出现在父母的页面上。
family-the-family = 家庭
family-attach = 关联
family-error-last-partner = 一个结合至少需要一个人。请改为删除家庭记录，那会询问如何处理引用它的一切。
family-error-no-family = 未选择家庭。未保存任何内容。
family-error-already-child = 此人已经是该家庭的子女。
pick-error-empty = 未指定人物。未保存任何内容。
pick-error-not-found = 本档案中没有此姓名的人。未保存任何内容。
pick-error-ambiguous = 有不止一个人符合。请从列表中选择一个，让记录说明是谁。未保存任何内容。

## Links and occupations

links-editor-title = 关联
links-editor-help = 不属于家庭的关系：教父母、雇主、证人、团队。每条都是指向两个人的独立记录，因此在此编辑也会改变另一条记录。
links-none = 未记录此人的任何关联。
links-new = 新的关联
links-create = 创建关联
links-remove = 删除此关联
links-other-end = 另一端
links-label = 关系名称
links-label-reverse = 反向读法
links-category = 类别
links-bidirectional = 双向读法相同
links-from = 自
links-until = 至
links-reversed = 此关联是从另一条记录创建的。在此编辑修改的是同一实体。
link-error-no-label = 关联必须说明它是什么。未保存任何内容。
occupations-editor-title = 职业
occupations-editor-help = 职业是有起讫的一段时间，而不是职位名称。每一项都带有自己的日期和来源。
occupations-none = 未记录此人的任何职业。
occupations-new = 新的职业
occupations-create = 创建职业
occupations-remove = 删除此职业
occupations-title = 从事的工作
occupations-employer = 为谁工作
occupations-employer-place = 雇主所在地
occupations-from = 自
occupations-until = 至
occupation-error-no-title = 职业必须说明某人做了什么。未保存任何内容。
link-category-spiritual = 宗教
link-category-professional = 职业
link-category-social = 社会
link-category-legal = 法律
link-category-medical = 医疗
link-category-educational = 教育
link-category-conflict = 冲突
link-category-other = 其他
links-edit-link = 编辑关联
occupations-edit-link = 编辑职业
family-edit-link = 编辑家庭与关系

## Events and documents

events-editor-title = 事件
events-editor-help = 一个事件会同时涉及多人——婚礼、洗礼、人口普查——所以每个事件都是独立记录，并出现在它提到的每个页面上。
events-none = 没有事件提到此人。
events-new = 新的事件
events-new-help = 若不指定其他人，此人将作为主体加入。没有人的事件只是一个日期。
events-create = 创建事件
events-remove = 删除此事件
events-category = 发生了什么
events-subcategory = 更具体地
events-description = 描述
events-participants = 参与者
events-participants-help = 保存将修改事件记录，其中提到的每个人都会随之改变。
events-who = 何人
event-error-no-category = 事件必须说明发生了什么。未保存任何内容。
documents-editor-title = 文件
documents-editor-help = 本记录指向哪些文件，以及每个文件对它意味着什么。清空一行即解除关联：文档及其内容仍留在档案中。
documents-attached = 已关联到本记录
documents-upload = 上传文件
documents-upload-help = 最大 { $mb } MB。文件将存入档案并关联到本记录。
documents-caption = 说明文字
events-edit-link = 编辑事件

## Presentation styles: density, never colour

prefs-style = 疏密
style-comfortable = 舒适
style-comfortable-note = 默认，留有阅读空间
style-compact = 紧凑
style-compact-note = 每屏显示更多内容，便于连续查阅
style-paper = 纸面
style-paper-note = 衬线字体、以线条代替卡片，适合细读或打印

## Sensitive classes

admin-export-choose = 包含在此次导出中
scope-health = 健康与信仰
scope-biometrics = 生物特征数据
scope-genomics = 基因组数据
scope-legal = 犯罪记录
scope-behaviour = 在世者的行为画像
admin-export-with-chosen = 按勾选内容导出

## Profile

pg-identity = 身份与户籍
pg-identity-intro = 记录中此人是谁，以及户籍与身份登记写下了什么。
pg-morphology = 体貌
pg-morphology-intro = 测量并描述下来的身体。
pg-biometrics = 生物特征
pg-biometrics-intro = 声音、双手与感官，以及可以识别一个人的特征模板。
pg-health = 健康
pg-health-intro = 疾病、治疗、测量值与检验结果。
pg-genomics = 基因组
pg-genomics-intro = DNA 检测、单倍群、变异及其他分子层面的结果。
pg-death = 死亡
pg-death-intro = 生命如何、何时、何地结束，以及遗体如何处理。
pg-residence = 居所与国籍
pg-residence-intro = 此人住在哪里、哪些国家视其为国民，以及会说哪些语言。
pg-education = 教育与工作
pg-education-intro = 学业、资格、收入与财产。
pg-military = 兵役与荣誉
pg-military-intro = 服役、军衔、部队与勋奖。
pg-legal = 刑事案件
pg-legal-intro = 刑事诉讼及其结果。
pg-belief = 信仰与归属
pg-belief-intro = 宗教、仪式、信念与所属团体。
pg-personality = 性格与行为
pg-personality-intro = 资料所描述的气质、习惯与爱好。
pg-relationships = 关系
pg-relationships-intro = 父母、配偶、子女，以及一生中的其他人。
pg-digital-legacy = 数字遗产
pg-digital-legacy-intro = 代表此人的扫描件、模型、录音与档案。
pa-identity-titles = 称号
pa-identity-sex-at-birth = 出生时性别
pa-identity-gender-identity = 性别认同
pa-birth-time = 出生时刻
pa-birth-coordinates = 出生地坐标
pa-civil-status-birth-certificate-number = 出生证明编号
pa-civil-status-register-entries = 身份登记条目
pa-civil-status-marginal-annotations = 页边批注
pa-morphology-height = 身高
pa-morphology-weight = 体重
pa-morphology-bmi = 体重指数
pa-morphology-body-composition = 身体成分
pa-morphology-build = 体型
pa-morphology-eye-colour = 眼睛颜色
pa-morphology-eye-shape = 眼形
pa-morphology-eye-spacing = 眼距
pa-morphology-hair-colour = 天然发色
pa-morphology-hair-texture = 发质
pa-morphology-hairline = 发际线
pa-morphology-facial-hair = 胡须
pa-morphology-body-hair = 体毛
pa-morphology-skin-tone = 皮肤光型（菲茨帕特里克）
pa-morphology-skin-undertone = 肤色底调
pa-morphology-freckles = 雀斑
pa-morphology-pigmentation = 色素斑
pa-morphology-scars = 疤痕
pa-morphology-tattoos = 文身
pa-morphology-moles = 痣
pa-morphology-facial-asymmetries = 面部不对称
pa-morphology-face-shape = 脸型
pa-morphology-nose-shape = 鼻型
pa-morphology-ear-shape = 耳型
pa-morphology-lip-shape = 唇型
pa-morphology-dentition = 牙列
pa-morphology-malocclusion = 错𬌗畸形（安氏分类）
pa-morphology-posture = 体态
pa-morphology-gait = 步态
pa-morphology-distinguishing-features = 显著特征
pa-biometrics-fingerprints = 指纹
pa-biometrics-retinal-print = 视网膜图样
pa-biometrics-voice-signature = 声纹
pa-biometrics-voice-frequency = 嗓音基频
pa-biometrics-vocal-timbre = 音色
pa-biometrics-spoken-accent = 口音
pa-biometrics-speech-rate = 语速
pa-biometrics-verbal-tics = 口头禅
pa-biometrics-frequent-vocabulary = 常用词汇
pa-biometrics-speech-register = 语体
pa-biometrics-motor-tics = 运动性抽动
pa-biometrics-handedness = 惯用手
pa-biometrics-hearing = 听力
pa-biometrics-visual-acuity = 视力
pa-biometrics-optical-correction = 视力矫正
pa-health-blood-group = 血型（ABO）
pa-health-rhesus = Rh 血型（RhD）
pa-health-blood-pressure = 血压
pa-health-resting-heart-rate = 静息心率
pa-health-respiratory-capacity = 肺功能
pa-health-conditions = 疾病
pa-health-surgeries = 手术史
pa-health-injuries = 外伤
pa-health-deformities = 畸形
pa-health-amputations = 截肢
pa-health-prostheses = 假体
pa-health-implants = 植入物
pa-health-devices = 植入式装置
pa-health-medications = 用药
pa-health-allergies = 过敏
pa-health-vaccinations = 疫苗接种
pa-health-serology = 血清学
pa-health-lab-results = 化验结果
pa-health-deficiencies = 营养缺乏
pa-health-sleep-disorders = 睡眠障碍
pa-health-mental-health-assessments = 心理健康评估
pa-genomics-autosomal-mapping = 常染色体 DNA 检测
pa-genomics-y-haplogroup = Y 染色体单倍群
pa-genomics-mt-haplogroup = 线粒体单倍群
pa-genomics-whole-genome-sequencing = 全基因组测序
pa-genomics-risk-variants = 风险变异
pa-genomics-hereditary-conditions = 遗传病
pa-genomics-predispositions = 易感性
pa-genomics-epigenetic-markers = 表观遗传标记
pa-genomics-epigenetic-age = 表观遗传年龄
pa-genomics-gut-microbiome = 肠道微生物组
pa-genomics-skin-microbiome = 皮肤微生物组
pa-genomics-toxicological-sensitivities = 对药物与毒物的敏感性
pa-death-time = 死亡时刻
pa-death-coordinates = 死亡地坐标
pa-death-causes = 死因
pa-death-contributing-factors = 促成因素
pa-death-autopsy = 尸检
pa-death-disposition = 遗体处理
pa-death-grave = 墓地
pa-residence-addresses = 地址
pa-residence-nationality-of-origin = 原始国籍
pa-residence-acquired-nationalities = 取得的国籍
pa-residence-mother-tongue = 母语
pa-residence-spoken-languages = 会说的语言
pa-education-level = 受教育程度
pa-education-diplomas = 学位与证书
pa-education-institutions = 学校与机构
pa-education-income = 收入
pa-education-real-estate = 不动产
pa-military-distinctions = 勋奖
pa-military-citations = 嘉奖
pa-military-ranks = 军衔
pa-military-units = 部队
pa-military-service-numbers = 军籍编号
pa-legal-criminal-record = 犯罪记录
pa-belief-religions = 宗教
pa-belief-sacraments = 圣事与仪式
pa-belief-beliefs = 信念
pa-belief-political-leanings = 政治倾向
pa-belief-memberships = 所属团体
pa-personality-big-five = 大五人格得分
pa-personality-mbti = MBTI 类型
pa-personality-introversion-extraversion = 内向与外向
pa-personality-stress-tolerance = 抗压能力
pa-personality-decision-style = 决策风格
pa-personality-interests = 兴趣
pa-personality-hobbies = 爱好
pa-personality-sports = 运动
pa-personality-dietary-habits = 饮食
pa-personality-dependencies = 成瘾
pa-digital-legacy-body-models = 身体模型
pa-digital-legacy-skin-textures = 皮肤纹理
pa-digital-legacy-rigs = 骨骼绑定
pa-digital-legacy-voice-corpora = 用于语音合成的录音
pa-digital-legacy-text-corpora = 用于语言模型的文字
pa-digital-legacy-digital-traces = 数字足迹
pa-digital-legacy-carbon-footprint = 碳足迹
pa-digital-legacy-behaviour-models = 行为模型
pf-identity-titles-text = 资料中的称号写法
pf-identity-titles-kind = 称号类型
pf-civil-status-marginal-annotations-text = 批注
pf-morphology-pigmentation-kind = 斑的类型
pf-biometrics-spoken-accent-description = 如何描述
pf-biometrics-optical-correction-kind = 矫正方式
pf-health-amputations-level = 截肢平面
pf-health-prostheses-kind = 假体类型
pf-health-implants-kind = 植入物类型
pf-health-devices-kind = 装置
pf-health-allergies-type = 过敏类型
pf-health-vaccinations-status = 接种状态
pf-health-sleep-disorders-category = 障碍类别
pf-death-autopsy-kind = 尸检类型
pf-education-institutions-name = 机构名称
pf-military-distinctions-name = 勋奖名称
pf-military-distinctions-kind = 勋奖类型
pf-military-citations-text = 嘉奖内容
pf-military-ranks-category = 军衔类别
pf-belief-political-leanings-position = 在左右轴上的位置
pf-belief-memberships-kind = 团体类型
pf-digital-legacy-carbon-footprint-method = 估算方法
pf-age-years = 年龄（岁）
pf-agreeableness = 宜人性
pf-allergen = 过敏原
pf-amount = 金额
pf-analyte = 检测项目
pf-artefact-type = 成果类型
pf-autoimmune = 自身免疫性
pf-body-region = 身体部位
pf-bone-percent = 骨骼
pf-carrier-status = 携带状态
pf-cause = 原因
pf-chronic = 慢性
pf-clock = 时钟
pf-condition = 疾病
pf-conferred-by = 授予者
pf-congenital = 先天性
pf-conscientiousness = 尽责性
pf-consent = 同意
pf-coordinates = 坐标
pf-corrected = 矫正后
pf-country = 国家
pf-court = 法院
pf-coverage = 覆盖度
pf-currency = 货币
pf-decimal = 视力（小数）
pf-denomination = 教派
pf-derived-from-id = 来源文件
pf-description = 描述
pf-details = 详情
pf-diagnosis = 诊断
pf-diameter-mm = 直径
pf-diastolic = 舒张压
pf-diet = 饮食方式
pf-document-id = 文件
pf-dose = 剂量
pf-ear = 耳
pf-entry-number = 条目编号
pf-extraversion = 外向性
pf-eye = 眼
pf-fat-percent = 脂肪
pf-fev1-fvc-ratio = FEV1/FVC 比值
pf-fev1-litres = FEV1
pf-file-format = 文件格式
pf-findings = 所见
pf-flag = 提示
pf-format = 格式
pf-fracture = 骨折
pf-fvc-litres = FVC
pf-gene = 基因
pf-generator = 制作工具
pf-grade = 程度
pf-iccs-section = 犯罪类别（ICCS）
pf-icd10-chapter = ICD-10 章节
pf-indication = 适应证
pf-inheritance = 遗传方式
pf-inscription = 碑文
pf-institution = 机构
pf-instrument = 量表
pf-isced-level = ISCED 级别
pf-jurisdiction = 管辖地
pf-language = 语言
pf-lat = 纬度
pf-level = 水平
pf-lines = 地址
pf-location = 位置
pf-lon = 经度
pf-major = 主单倍群
pf-marker = 标记
pf-metaboliser-status = 代谢型
pf-method = 方式
pf-mode = 取得方式
pf-muscle-percent = 肌肉
pf-neuroticism = 神经质
pf-number = 编号
pf-nutrient = 营养素
pf-offence = 罪名
pf-office = 登记机关
pf-openness = 开放性
pf-organisation = 团体
pf-outcome = 结果
pf-pace = 衰老速度
pf-page = 页
pf-panel = 检验套餐
pf-party = 政党
pf-pathogen = 病原体
pf-pattern = 使用模式
pf-percentile = 百分位
pf-period = 发放周期
pf-place-id = 地点
pf-plot = 墓位
pf-polygenic-score = 多基因评分
pf-postal-code = 邮政编码
pf-precision = 精度
pf-prescription = 处方
pf-proficiency = 熟练程度
pf-provider = 服务机构
pf-quintile = 收入五分位
pf-rank = 军衔
pf-rank-text = 资料中的军衔写法
pf-reaction = 反应
pf-reference-build = 参考基因组
pf-reference-high = 参考范围上限
pf-reference-low = 参考范围下限
pf-register-type = 登记类型
pf-result = 结果
pf-role = 职务
pf-sacrament = 圣事或仪式
pf-score = 得分
pf-sentence = 刑罚
pf-sequence = 死因链中的位置
pf-service = 军种
pf-severity = 严重程度
pf-shannon-diversity = 香农多样性
pf-shape = 形状
pf-significance = 临床意义
pf-snp-count = 检测的 SNP 数
pf-sport = 项目
pf-subclade = 下游支系
pf-substance = 物质
pf-summary = 摘要
pf-systolic = 收缩压
pf-tenure = 产权形式
pf-test = 检测
pf-threshold-db = 听阈
pf-title = 名称
pf-tonnes-co2e-per-year = 排放量
pf-tradition = 传统
pf-tree-version = 系统树版本
pf-unit = 单位
pf-use = 用途
pf-variant = 变异
pf-volume = 卷
pf-zygosity = 合子型
pu-cm = { $n } 厘米
pu-kg = { $n } 千克
pu-kg-m2 = { $n } kg/m²
pu-percent = { $n }%
pu-mm = { $n } 毫米
pu-hz = { $n } 赫兹
pu-words-min = 每分钟 { $n } 词
pu-db-hl = { $n } dB HL
pu-mmhg = { $n } mmHg
pu-bpm = 每分钟 { $n } 次
pu-litres = { $n } 升
pu-coverage = { $n }×
pu-years = { $n } 岁
pu-t-co2e-yr = 每年 { $n } 吨二氧化碳当量
pv-sensitive-class-health = 健康与信仰
pv-sensitive-class-biometrics = 生物特征数据
pv-sensitive-class-genomics = 基因组数据
pv-sensitive-class-legal = 犯罪记录
pv-laterality-left = 左侧
pv-laterality-right = 右侧
pv-laterality-both = 双侧
pv-body-region-head = 头部
pv-body-region-face = 面部
pv-body-region-neck = 颈部
pv-body-region-left-shoulder = 左肩
pv-body-region-right-shoulder = 右肩
pv-body-region-left-arm = 左臂
pv-body-region-right-arm = 右臂
pv-body-region-left-hand = 左手
pv-body-region-right-hand = 右手
pv-body-region-chest = 胸部
pv-body-region-abdomen = 腹部
pv-body-region-upper-back = 上背部
pv-body-region-lower-back = 下背部
pv-body-region-pelvis = 骨盆与髋部
pv-body-region-left-leg = 左腿
pv-body-region-right-leg = 右腿
pv-body-region-left-foot = 左脚
pv-body-region-right-foot = 右脚
pv-body-region-internal = 体内
pv-body-region-whole-body = 全身
pv-body-region-other = 其他部位
pv-artefact-type-mesh = 网格模型
pv-artefact-type-point-cloud = 点云
pv-artefact-type-skin-texture-map = 皮肤纹理贴图
pv-artefact-type-skeletal-rig = 骨骼绑定
pv-artefact-type-voice-corpus = 语音语料
pv-artefact-type-text-corpus = 文本语料
pv-artefact-type-trace-archive = 网络活动存档
pv-artefact-type-behaviour-model = 行为模型
pv-artefact-type-fingerprint-card = 指纹卡
pv-artefact-type-fingerprint-template = 指纹模板
pv-artefact-type-retinal-image = 视网膜图像
pv-artefact-type-voiceprint = 声纹
pv-consent-given = 已同意
pv-consent-given-by-estate = 由继承人同意
pv-consent-refused = 拒绝
pv-consent-withdrawn = 已撤回
pv-consent-not-asked = 未征询
pv-consent-unknown = 不详
pv-sex-at-birth-female = 女
pv-sex-at-birth-male = 男
pv-sex-at-birth-intersex = 间性
pv-sex-at-birth-undetermined = 未确定
pv-sex-at-birth-unknown = 不详
pv-gender-identity-woman = 女性
pv-gender-identity-man = 男性
pv-gender-identity-non-binary = 非二元
pv-gender-identity-other = 其他
pv-gender-identity-undisclosed = 未透露
pv-gender-identity-unknown = 不详
pv-title-kind-nobility = 爵位
pv-title-kind-academic = 学术头衔
pv-title-kind-professional = 职业头衔
pv-title-kind-religious = 宗教头衔
pv-title-kind-military = 军事头衔
pv-title-kind-civic = 荣誉称号
pv-title-kind-courtesy = 敬称
pv-title-kind-other = 其他
pv-register-type-birth = 出生
pv-register-type-baptism = 洗礼
pv-register-type-marriage = 婚姻
pv-register-type-death = 死亡
pv-register-type-burial = 安葬
pv-register-type-divorce = 离婚
pv-register-type-recognition = 认领子女
pv-register-type-legitimation = 准正
pv-register-type-adoption = 收养
pv-register-type-name-change = 更名
pv-register-type-other = 其他
pv-build-slight = 纤细
pv-build-slim = 苗条
pv-build-average = 中等
pv-build-sturdy = 结实
pv-build-stout = 粗壮
pv-build-heavy = 肥胖
pv-eye-colour-light-blue = 浅蓝
pv-eye-colour-blue = 蓝
pv-eye-colour-dark-blue = 深蓝
pv-eye-colour-grey = 灰
pv-eye-colour-blue-grey = 蓝灰
pv-eye-colour-green = 绿
pv-eye-colour-grey-green = 灰绿
pv-eye-colour-hazel = 榛色
pv-eye-colour-amber = 琥珀色
pv-eye-colour-light-brown = 浅棕
pv-eye-colour-brown = 棕
pv-eye-colour-dark-brown = 深棕
pv-eye-colour-black = 黑
pv-eye-colour-mixed = 混合
pv-eye-colour-other = 其他
pv-eye-shape-almond = 杏眼
pv-eye-shape-round = 圆眼
pv-eye-shape-hooded = 内双
pv-eye-shape-monolid = 单眼皮
pv-eye-shape-deep-set = 深眼窝
pv-eye-shape-protruding = 突眼
pv-eye-shape-upturned = 上扬眼
pv-eye-shape-downturned = 下垂眼
pv-eye-shape-other = 其他
pv-eye-spacing-close-set = 眼距窄
pv-eye-spacing-average = 眼距适中
pv-eye-spacing-wide-set = 眼距宽
pv-hair-colour-black = 黑
pv-hair-colour-dark-brown = 深棕
pv-hair-colour-brown = 棕
pv-hair-colour-light-brown = 浅棕
pv-hair-colour-auburn = 红褐
pv-hair-colour-red = 红
pv-hair-colour-strawberry-blond = 草莓金
pv-hair-colour-dark-blond = 深金
pv-hair-colour-blond = 金
pv-hair-colour-light-blond = 浅金
pv-hair-colour-grey = 花白
pv-hair-colour-white = 全白
pv-hair-colour-none = 无头发
pv-hair-colour-other = 其他
pv-hair-texture-straight = 直发
pv-hair-texture-wavy = 波浪卷
pv-hair-texture-curly = 卷发
pv-hair-texture-coily = 紧密卷
pv-hair-texture-other = 其他
pv-hairline-straight = 平直
pv-hairline-rounded = 圆弧
pv-hairline-widows-peak = 美人尖
pv-hairline-m-shaped = M 形
pv-hairline-bell-shaped = 钟形
pv-hairline-uneven = 不规则
pv-hairline-receding = 后移
pv-hairline-bald = 秃顶
pv-facial-hair-none = 无
pv-facial-hair-stubble = 胡茬
pv-facial-hair-moustache = 上唇胡
pv-facial-hair-goatee = 山羊胡
pv-facial-hair-full-beard = 络腮胡
pv-facial-hair-sideburns = 鬓角
pv-facial-hair-other = 其他
pv-body-hair-none = 无
pv-body-hair-sparse = 稀少
pv-body-hair-moderate = 适中
pv-body-hair-dense = 浓密
pv-skin-tone-type-i = I 型 — 总是晒伤，从不晒黑
pv-skin-tone-type-ii = II 型 — 常晒伤，很少晒黑
pv-skin-tone-type-iii = III 型 — 有时晒伤，均匀晒黑
pv-skin-tone-type-iv = IV 型 — 很少晒伤，容易晒黑
pv-skin-tone-type-v = V 型 — 极少晒伤
pv-skin-tone-type-vi = VI 型 — 从不晒伤
pv-skin-undertone-cool = 冷调
pv-skin-undertone-neutral = 中性
pv-skin-undertone-warm = 暖调
pv-skin-undertone-olive = 橄榄调
pv-freckles-none = 无
pv-freckles-few = 少量
pv-freckles-moderate = 适中
pv-freckles-many = 较多
pv-pigmentation-mark-birthmark = 胎记
pv-pigmentation-mark-port-wine-stain = 葡萄酒色斑
pv-pigmentation-mark-cafe-au-lait-spot = 咖啡牛奶斑
pv-pigmentation-mark-depigmented-patch = 色素减退斑
pv-pigmentation-mark-hyperpigmented-patch = 色素沉着斑
pv-pigmentation-mark-other = 其他
pv-mole-shape-round = 圆形
pv-mole-shape-oval = 椭圆形
pv-mole-shape-irregular = 不规则
pv-mole-shape-other = 其他
pv-face-shape-oval = 鹅蛋脸
pv-face-shape-round = 圆脸
pv-face-shape-square = 方脸
pv-face-shape-oblong = 长脸
pv-face-shape-heart = 心形脸
pv-face-shape-diamond = 菱形脸
pv-face-shape-triangular = 三角脸
pv-nose-shape-straight = 直鼻
pv-nose-shape-aquiline = 鹰钩鼻
pv-nose-shape-snub = 塌鼻
pv-nose-shape-upturned = 朝天鼻
pv-nose-shape-flat = 扁鼻
pv-nose-shape-broad = 宽鼻
pv-nose-shape-bulbous = 蒜头鼻
pv-nose-shape-crooked = 歪鼻
pv-nose-shape-other = 其他
pv-ear-shape-free-lobe = 耳垂分离
pv-ear-shape-attached-lobe = 耳垂相连
pv-ear-shape-protruding = 招风耳
pv-ear-shape-close-set = 贴头耳
pv-ear-shape-pointed = 尖耳
pv-ear-shape-other = 其他
pv-lip-shape-thin = 薄唇
pv-lip-shape-medium = 适中
pv-lip-shape-full = 厚唇
pv-lip-shape-bow-shaped = 弓形唇
pv-lip-shape-wide = 宽唇
pv-lip-shape-downturned = 嘴角下垂
pv-lip-shape-other = 其他
pv-dentition-primary = 乳牙列
pv-dentition-mixed = 混合牙列
pv-dentition-permanent-complete = 恒牙列完整
pv-dentition-permanent-partial-loss = 恒牙列部分缺失
pv-dentition-edentulous = 无牙颌
pv-dentition-partial-denture = 局部义齿
pv-dentition-full-denture = 全口义齿
pv-dentition-implants = 种植牙
pv-malocclusion-normal = 正常𬌗
pv-malocclusion-class-i = I 类
pv-malocclusion-class-ii-division-1 = II 类 1 分类
pv-malocclusion-class-ii-division-2 = II 类 2 分类
pv-malocclusion-class-iii = III 类
pv-posture-ideal = 端正
pv-posture-kyphotic-lordotic = 驼背伴腰前凸
pv-posture-flat-back = 平背
pv-posture-sway-back = 摇摆背
pv-posture-stooped = 弯腰驼背
pv-posture-scoliotic = 脊柱侧弯
pv-posture-other = 其他
pv-gait-brisk = 轻快
pv-gait-average = 普通
pv-gait-slow = 缓慢
pv-gait-shuffling = 拖步
pv-gait-limping = 跛行
pv-gait-waddling = 鸭步
pv-gait-unsteady = 不稳
pv-gait-stiff = 僵硬
pv-gait-other = 其他
pv-vocal-timbre-bright = 明亮
pv-vocal-timbre-dark = 暗沉
pv-vocal-timbre-warm = 温暖
pv-vocal-timbre-breathy = 气声
pv-vocal-timbre-nasal = 鼻音重
pv-vocal-timbre-hoarse = 沙哑
pv-vocal-timbre-resonant = 洪亮
pv-vocal-timbre-thin = 尖细
pv-vocal-timbre-other = 其他
pv-speech-register-frozen = 庄重
pv-speech-register-formal = 正式
pv-speech-register-consultative = 一般
pv-speech-register-casual = 随意
pv-speech-register-intimate = 亲昵
pv-handedness-left = 左撇子
pv-handedness-right = 右利手
pv-handedness-ambidextrous = 左右开弓
pv-handedness-mixed = 因事而异
pv-handedness-unknown = 不详
pv-hearing-grade-normal = 正常
pv-hearing-grade-mild = 轻度听力损失
pv-hearing-grade-moderate = 中度听力损失
pv-hearing-grade-moderately-severe = 中重度听力损失
pv-hearing-grade-severe = 重度听力损失
pv-hearing-grade-profound = 极重度听力损失
pv-hearing-grade-complete = 全聋
pv-optical-correction-none = 无
pv-optical-correction-glasses = 眼镜
pv-optical-correction-contact-lenses = 隐形眼镜
pv-optical-correction-glasses-and-contact-lenses = 眼镜与隐形眼镜
pv-optical-correction-refractive-surgery = 屈光手术
pv-optical-correction-intraocular-lens = 人工晶体
pv-optical-correction-other = 其他
pv-rhesus-positive = RhD 阳性
pv-rhesus-negative = RhD 阴性
pv-rhesus-weak-d = 弱 D
pv-rhesus-unknown = 不详
pv-icd10-chapter-infectious-parasitic = I 某些传染病和寄生虫病
pv-icd10-chapter-neoplasms = II 肿瘤
pv-icd10-chapter-blood-immune = III 血液及免疫机制疾病
pv-icd10-chapter-endocrine-metabolic = IV 内分泌、营养和代谢疾病
pv-icd10-chapter-mental-behavioural = V 精神和行为障碍
pv-icd10-chapter-nervous-system = VI 神经系统疾病
pv-icd10-chapter-eye-adnexa = VII 眼和附器疾病
pv-icd10-chapter-ear-mastoid = VIII 耳和乳突疾病
pv-icd10-chapter-circulatory = IX 循环系统疾病
pv-icd10-chapter-respiratory = X 呼吸系统疾病
pv-icd10-chapter-digestive = XI 消化系统疾病
pv-icd10-chapter-skin = XII 皮肤和皮下组织疾病
pv-icd10-chapter-musculoskeletal = XIII 肌肉骨骼系统疾病
pv-icd10-chapter-genitourinary = XIV 泌尿生殖系统疾病
pv-icd10-chapter-pregnancy-childbirth = XV 妊娠、分娩和产褥期
pv-icd10-chapter-perinatal = XVI 围生期病症
pv-icd10-chapter-congenital = XVII 先天性畸形
pv-icd10-chapter-ill-defined = XVIII 症状与原因不明
pv-icd10-chapter-injury-poisoning = XIX 损伤和中毒
pv-icd10-chapter-external-causes = XX 外部原因
pv-icd10-chapter-health-factors = XXI 影响健康状态的因素
pv-icd10-chapter-special-purposes = XXII 特殊用途编码
pv-diagnosis-status-diagnosed = 医生确诊
pv-diagnosis-status-suspected = 疑似
pv-diagnosis-status-self-reported = 本人或家属自述
pv-diagnosis-status-unknown = 不详
pv-prosthesis-kind-limb = 假肢
pv-prosthesis-kind-joint = 人工关节
pv-prosthesis-kind-ocular = 义眼
pv-prosthesis-kind-dental = 义齿
pv-prosthesis-kind-auditory = 听觉假体
pv-prosthesis-kind-breast = 乳房假体
pv-prosthesis-kind-other = 其他
pv-implant-kind-orthopaedic = 骨科植入物
pv-implant-kind-dental = 牙科植入物
pv-implant-kind-cochlear = 人工耳蜗
pv-implant-kind-breast = 乳房植入物
pv-implant-kind-intraocular-lens = 人工晶体
pv-implant-kind-contraceptive = 避孕植入物
pv-implant-kind-cosmetic = 美容植入物
pv-implant-kind-other = 其他
pv-device-kind-pacemaker = 心脏起搏器
pv-device-kind-implantable-defibrillator = 植入式除颤器
pv-device-kind-cardiac-resynchronisation = 心脏再同步装置
pv-device-kind-ventricular-assist = 心室辅助装置
pv-device-kind-neurostimulator = 神经刺激器
pv-device-kind-insulin-pump = 胰岛素泵
pv-device-kind-drug-port = 输液港
pv-device-kind-shunt = 分流管
pv-device-kind-stent = 支架
pv-device-kind-other = 其他
pv-allergy-type-drug = 药物
pv-allergy-type-food = 食物
pv-allergy-type-environmental = 环境
pv-allergy-type-insect-venom = 昆虫毒液
pv-allergy-type-latex = 乳胶
pv-allergy-type-other = 其他
pv-allergy-severity-mild = 轻度
pv-allergy-severity-moderate = 中度
pv-allergy-severity-severe = 重度
pv-allergy-severity-anaphylactic = 过敏性休克
pv-allergy-severity-unknown = 不详
pv-pathogen-diphtheria = 白喉
pv-pathogen-tetanus = 破伤风
pv-pathogen-pertussis = 百日咳
pv-pathogen-poliomyelitis = 脊髓灰质炎
pv-pathogen-measles = 麻疹
pv-pathogen-mumps = 流行性腮腺炎
pv-pathogen-rubella = 风疹
pv-pathogen-varicella = 水痘
pv-pathogen-smallpox = 天花
pv-pathogen-tuberculosis = 结核病
pv-pathogen-hepatitis-a = 甲型肝炎
pv-pathogen-hepatitis-b = 乙型肝炎
pv-pathogen-hepatitis-c = 丙型肝炎
pv-pathogen-haemophilus-influenzae-b = b 型流感嗜血杆菌
pv-pathogen-pneumococcal = 肺炎球菌
pv-pathogen-meningococcal = 脑膜炎球菌
pv-pathogen-human-papillomavirus = 人乳头瘤病毒
pv-pathogen-influenza = 流感
pv-pathogen-covid-19 = 新冠肺炎
pv-pathogen-rotavirus = 轮状病毒
pv-pathogen-yellow-fever = 黄热病
pv-pathogen-typhoid = 伤寒
pv-pathogen-cholera = 霍乱
pv-pathogen-rabies = 狂犬病
pv-pathogen-japanese-encephalitis = 乙型脑炎
pv-pathogen-tick-borne-encephalitis = 蜱传脑炎
pv-pathogen-hiv = 艾滋病毒
pv-pathogen-syphilis = 梅毒
pv-pathogen-toxoplasmosis = 弓形虫病
pv-pathogen-cytomegalovirus = 巨细胞病毒
pv-pathogen-epstein-barr = EB 病毒
pv-pathogen-other = 其他
pv-vaccination-status-vaccinated = 已接种
pv-vaccination-status-partially-vaccinated = 部分接种
pv-vaccination-status-unvaccinated = 未接种
pv-vaccination-status-contraindicated = 禁忌接种
pv-vaccination-status-unknown = 不详
pv-serology-result-positive = 阳性
pv-serology-result-negative = 阴性
pv-serology-result-equivocal = 可疑
pv-serology-result-unknown = 不详
pv-lab-panel-basic-metabolic = 基础代谢组合
pv-lab-panel-lipid = 血脂
pv-lab-panel-liver = 肝功能
pv-lab-panel-renal = 肾功能
pv-lab-panel-glycated-haemoglobin = 糖化血红蛋白
pv-lab-panel-iron = 铁代谢
pv-lab-analyte-sodium = 钠
pv-lab-analyte-potassium = 钾
pv-lab-analyte-chloride = 氯
pv-lab-analyte-bicarbonate = 碳酸氢盐
pv-lab-analyte-urea = 尿素
pv-lab-analyte-creatinine = 肌酐
pv-lab-analyte-glucose = 葡萄糖
pv-lab-analyte-calcium = 钙
pv-lab-analyte-total-cholesterol = 总胆固醇
pv-lab-analyte-ldl-cholesterol = 低密度脂蛋白胆固醇
pv-lab-analyte-hdl-cholesterol = 高密度脂蛋白胆固醇
pv-lab-analyte-triglycerides = 甘油三酯
pv-lab-analyte-non-hdl-cholesterol = 非高密度脂蛋白胆固醇
pv-lab-analyte-alt = 丙氨酸氨基转移酶（ALT）
pv-lab-analyte-ast = 天冬氨酸氨基转移酶（AST）
pv-lab-analyte-alp = 碱性磷酸酶（ALP）
pv-lab-analyte-ggt = γ-谷氨酰转移酶（GGT）
pv-lab-analyte-total-bilirubin = 总胆红素
pv-lab-analyte-direct-bilirubin = 直接胆红素
pv-lab-analyte-albumin = 白蛋白
pv-lab-analyte-total-protein = 总蛋白
pv-lab-analyte-egfr = 估算肾小球滤过率
pv-lab-analyte-uric-acid = 尿酸
pv-lab-analyte-phosphate = 磷
pv-lab-analyte-urine-albumin-creatinine-ratio = 尿白蛋白/肌酐比值
pv-lab-analyte-hba1c = HbA1c
pv-lab-analyte-serum-iron = 血清铁
pv-lab-analyte-ferritin = 铁蛋白
pv-lab-analyte-transferrin = 转铁蛋白
pv-lab-analyte-transferrin-saturation = 转铁蛋白饱和度
pv-lab-analyte-tibc = 总铁结合力
pv-lab-flag-low = 偏低
pv-lab-flag-normal = 正常
pv-lab-flag-high = 偏高
pv-lab-flag-critical-low = 危急低值
pv-lab-flag-critical-high = 危急高值
pv-nutrient-vitamin-a = 维生素 A
pv-nutrient-thiamine = 硫胺素（B1）
pv-nutrient-riboflavin = 核黄素（B2）
pv-nutrient-niacin = 烟酸（B3）
pv-nutrient-vitamin-b6 = 维生素 B6
pv-nutrient-folate = 叶酸（B9）
pv-nutrient-vitamin-b12 = 维生素 B12
pv-nutrient-vitamin-c = 维生素 C
pv-nutrient-vitamin-d = 维生素 D
pv-nutrient-vitamin-e = 维生素 E
pv-nutrient-vitamin-k = 维生素 K
pv-nutrient-iron = 铁
pv-nutrient-zinc = 锌
pv-nutrient-magnesium = 镁
pv-nutrient-calcium = 钙
pv-nutrient-iodine = 碘
pv-nutrient-selenium = 硒
pv-nutrient-copper = 铜
pv-nutrient-potassium = 钾
pv-nutrient-phosphorus = 磷
pv-nutrient-other = 其他
pv-sleep-disorder-insomnia = 失眠
pv-sleep-disorder-sleep-related-breathing = 睡眠相关呼吸障碍
pv-sleep-disorder-central-hypersomnolence = 中枢性嗜睡
pv-sleep-disorder-circadian-rhythm = 昼夜节律睡眠障碍
pv-sleep-disorder-parasomnia = 异态睡眠
pv-sleep-disorder-sleep-related-movement = 睡眠相关运动障碍
pv-sleep-disorder-other = 其他
pv-assessment-instrument-phq-9 = PHQ-9
pv-assessment-instrument-gad-7 = GAD-7
pv-assessment-instrument-bdi-ii = BDI-II
pv-assessment-instrument-hads = HADS
pv-assessment-instrument-k10 = K10
pv-assessment-instrument-gds-15 = GDS-15
pv-assessment-instrument-mmse = MMSE
pv-assessment-instrument-moca = MoCA
pv-assessment-instrument-audit = AUDIT
pv-assessment-instrument-clinical-interview = 临床访谈
pv-assessment-instrument-other = 其他
pv-assessment-severity-none-minimal = 无或极轻
pv-assessment-severity-mild = 轻度
pv-assessment-severity-moderate = 中度
pv-assessment-severity-moderately-severe = 中重度
pv-assessment-severity-severe = 重度
pv-reference-build-grch36 = GRCh36 (hg18)
pv-reference-build-grch37 = GRCh37 (hg19)
pv-reference-build-grch38 = GRCh38 (hg38)
pv-reference-build-t2t-chm13 = T2T-CHM13
pv-genomic-file-format-raw-microarray = 芯片原始数据
pv-genomic-file-format-fastq = FASTQ
pv-genomic-file-format-bam = BAM
pv-genomic-file-format-cram = CRAM
pv-genomic-file-format-vcf = VCF
pv-genomic-file-format-gvcf = gVCF
pv-genomic-file-format-other = 其他
pv-zygosity-heterozygous = 杂合
pv-zygosity-homozygous = 纯合
pv-zygosity-hemizygous = 半合子
pv-zygosity-compound-heterozygous = 复合杂合
pv-clinical-significance-pathogenic = 致病
pv-clinical-significance-likely-pathogenic = 可能致病
pv-clinical-significance-uncertain-significance = 意义未明
pv-clinical-significance-likely-benign = 可能良性
pv-clinical-significance-benign = 良性
pv-inheritance-pattern-autosomal-dominant = 常染色体显性
pv-inheritance-pattern-autosomal-recessive = 常染色体隐性
pv-inheritance-pattern-x-linked-dominant = X 连锁显性
pv-inheritance-pattern-x-linked-recessive = X 连锁隐性
pv-inheritance-pattern-y-linked = Y 连锁
pv-inheritance-pattern-mitochondrial = 线粒体遗传
pv-inheritance-pattern-multifactorial = 多因素遗传
pv-inheritance-pattern-unknown = 不详
pv-carrier-status-affected = 患病
pv-carrier-status-carrier = 携带者
pv-carrier-status-not-carrier = 非携带者
pv-carrier-status-unknown = 不详
pv-epigenetic-clock-horvath = Horvath
pv-epigenetic-clock-hannum = Hannum
pv-epigenetic-clock-phenoage = PhenoAge
pv-epigenetic-clock-grimage = GrimAge
pv-epigenetic-clock-dunedinpace = DunedinPACE
pv-epigenetic-clock-other = 其他
pv-metaboliser-status-poor = 慢代谢型
pv-metaboliser-status-intermediate = 中间代谢型
pv-metaboliser-status-normal = 正常代谢型
pv-metaboliser-status-rapid = 快代谢型
pv-metaboliser-status-ultrarapid = 超快代谢型
pv-autopsy-not-performed = 未进行
pv-autopsy-clinical = 病理解剖
pv-autopsy-forensic = 法医解剖
pv-autopsy-external-examination = 仅体表检查
pv-autopsy-unknown = 不详
pv-disposition-burial = 土葬
pv-disposition-cremation = 火化
pv-disposition-entombment = 入墓室安葬
pv-disposition-burial-at-sea = 海葬
pv-disposition-natural-burial = 自然葬
pv-disposition-body-donation = 遗体捐献
pv-disposition-other = 其他
pv-disposition-unknown = 不详
pv-address-use-principal = 主要住所
pv-address-use-secondary = 第二住所
pv-address-use-temporary = 临时住所
pv-address-use-postal = 通信地址
pv-address-use-other = 其他
pv-nationality-mode-descent = 因血统
pv-nationality-mode-birth-in-territory = 因出生地
pv-nationality-mode-naturalisation = 因入籍
pv-nationality-mode-marriage = 因婚姻
pv-nationality-mode-registration = 因申报登记
pv-nationality-mode-restoration = 因恢复国籍
pv-nationality-mode-state-succession = 因主权变更
pv-nationality-mode-other = 其他
pv-language-proficiency-a1 = A1 入门
pv-language-proficiency-a2 = A2 初级
pv-language-proficiency-b1 = B1 中级
pv-language-proficiency-b2 = B2 中高级
pv-language-proficiency-c1 = C1 高级
pv-language-proficiency-c2 = C2 精通
pv-language-proficiency-native = 第一语言
pv-isced-level-isced-0 = 0 学前教育
pv-isced-level-isced-1 = 1 初等教育
pv-isced-level-isced-2 = 2 初中
pv-isced-level-isced-3 = 3 高中
pv-isced-level-isced-4 = 4 中等后非高等教育
pv-isced-level-isced-5 = 5 短期高等教育
pv-isced-level-isced-6 = 6 学士或同等
pv-isced-level-isced-7 = 7 硕士或同等
pv-isced-level-isced-8 = 8 博士或同等
pv-income-quintile-q1 = 最低五分之一
pv-income-quintile-q2 = 次低五分之一
pv-income-quintile-q3 = 中间五分之一
pv-income-quintile-q4 = 次高五分之一
pv-income-quintile-q5 = 最高五分之一
pv-pay-period-hourly = 按小时
pv-pay-period-daily = 按日
pv-pay-period-weekly = 按周
pv-pay-period-monthly = 按月
pv-pay-period-annual = 按年
pv-tenure-owned = 自有
pv-tenure-co-owned = 共有
pv-tenure-leasehold = 长期租赁
pv-tenure-rented = 租用
pv-tenure-usufruct = 用益权
pv-tenure-other = 其他
pv-distinction-kind-order = 勋章
pv-distinction-kind-decoration = 奖章
pv-distinction-kind-medal = 纪念章
pv-distinction-kind-title = 荣誉称号
pv-distinction-kind-other = 其他
pv-military-service-army = 陆军
pv-military-service-navy = 海军
pv-military-service-air-force = 空军
pv-military-service-marines = 海军陆战队
pv-military-service-gendarmerie = 宪兵
pv-military-service-border-guard = 边防部队
pv-military-service-national-guard = 国民警卫队
pv-military-service-other = 其他
pv-rank-category-enlisted = 士兵
pv-rank-category-non-commissioned = 士官
pv-rank-category-warrant = 准尉
pv-rank-category-officer-cadet = 军校学员
pv-rank-category-junior-officer = 尉官
pv-rank-category-senior-officer = 校官
pv-rank-category-general-officer = 将官
pv-iccs-section-acts-leading-to-death = 01 致人死亡的行为
pv-iccs-section-acts-causing-harm = 02 致人伤害的行为
pv-iccs-section-sexual-acts = 03 性侵害行为
pv-iccs-section-property-with-violence = 04 使用暴力侵犯财产
pv-iccs-section-property-only = 05 仅侵犯财产
pv-iccs-section-controlled-substances = 06 涉及管制物质
pv-iccs-section-fraud-deception-corruption = 07 欺诈、蒙骗或腐败
pv-iccs-section-public-order-and-state = 08 危害公共秩序和国家
pv-iccs-section-public-safety-and-security = 09 危害公共安全
pv-iccs-section-natural-environment = 10 破坏自然环境
pv-iccs-section-other-criminal-acts = 11 其他犯罪
pv-case-outcome-convicted = 定罪
pv-case-outcome-acquitted = 无罪释放
pv-case-outcome-dismissed = 撤销案件
pv-case-outcome-conviction-quashed = 判决被撤销
pv-case-outcome-pardoned = 特赦
pv-case-outcome-amnestied = 大赦
pv-case-outcome-expunged = 记录已消除
pv-case-outcome-pending = 审理中
pv-case-outcome-unknown = 不详
pv-religion-buddhism = 佛教
pv-religion-christianity-catholic = 基督宗教：天主教
pv-religion-christianity-orthodox = 基督宗教：东正教
pv-religion-christianity-protestant = 基督宗教：新教
pv-religion-christianity-other = 基督宗教：其他
pv-religion-hinduism = 印度教
pv-religion-islam-sunni = 伊斯兰教：逊尼派
pv-religion-islam-shia = 伊斯兰教：什叶派
pv-religion-islam-other = 伊斯兰教：其他
pv-religion-jainism = 耆那教
pv-religion-judaism = 犹太教
pv-religion-sikhism = 锡克教
pv-religion-bahai = 巴哈伊信仰
pv-religion-shinto = 神道教
pv-religion-taoism = 道教
pv-religion-zoroastrianism = 琐罗亚斯德教
pv-religion-traditional = 传统或民间宗教
pv-religion-other = 其他
pv-religion-none = 无宗教信仰
pv-religion-unknown = 不详
pv-sacrament-baptism = 洗礼
pv-sacrament-confirmation = 坚振
pv-sacrament-first-communion = 初领圣体
pv-sacrament-reconciliation = 告解
pv-sacrament-anointing-of-the-sick = 病人傅油
pv-sacrament-holy-orders = 圣秩
pv-sacrament-matrimony = 婚配
pv-sacrament-other-rite = 其他仪式
pv-political-position-far-left = 极左
pv-political-position-left = 左翼
pv-political-position-centre-left = 中左
pv-political-position-centre = 中间派
pv-political-position-centre-right = 中右
pv-political-position-right = 右翼
pv-political-position-far-right = 极右
pv-political-position-apolitical = 不问政治
pv-political-position-other = 不在此轴上
pv-political-position-unknown = 不详
pv-membership-kind-trade-union = 工会
pv-membership-kind-political-party = 政党
pv-membership-kind-professional-body = 专业团体
pv-membership-kind-religious-order = 修会
pv-membership-kind-religious-association = 宗教团体
pv-membership-kind-fraternal-order = 兄弟会
pv-membership-kind-veterans-association = 退伍军人协会
pv-membership-kind-sports-club = 体育俱乐部
pv-membership-kind-cultural-association = 文化团体
pv-membership-kind-charitable-association = 慈善团体
pv-membership-kind-other = 其他
pv-personality-instrument-neo-pi-3 = NEO-PI-3
pv-personality-instrument-neo-ffi-3 = NEO-FFI-3
pv-personality-instrument-bfi-2 = BFI-2
pv-personality-instrument-ipip-neo-120 = IPIP-NEO-120
pv-personality-instrument-tipi = TIPI
pv-personality-instrument-hexaco-pi-r = HEXACO-PI-R
pv-personality-instrument-observer-rating = 熟人评定
pv-personality-instrument-inferred = 根据资料推断
pv-personality-instrument-other = 其他
pv-introversion-extraversion-strongly-introverted = 非常内向
pv-introversion-extraversion-introverted = 内向
pv-introversion-extraversion-ambiverted = 中间型
pv-introversion-extraversion-extraverted = 外向
pv-introversion-extraversion-strongly-extraverted = 非常外向
pv-stress-tolerance-very-low = 很低
pv-stress-tolerance-low = 低
pv-stress-tolerance-moderate = 中等
pv-stress-tolerance-high = 高
pv-stress-tolerance-very-high = 很高
pv-decision-style-rational = 理性型
pv-decision-style-intuitive = 直觉型
pv-decision-style-dependent = 依赖型
pv-decision-style-avoidant = 回避型
pv-decision-style-spontaneous = 冲动型
pv-sport-level-recreational = 休闲
pv-sport-level-amateur-competitive = 业余竞技
pv-sport-level-semi-professional = 半职业
pv-sport-level-professional = 职业
pv-diet-omnivore = 杂食
pv-diet-flexitarian = 弹性素食
pv-diet-pescatarian = 鱼素
pv-diet-vegetarian = 素食
pv-diet-vegan = 纯素
pv-diet-other = 其他
pv-substance-tobacco = 烟草与尼古丁
pv-substance-alcohol = 酒精
pv-substance-cannabis = 大麻
pv-substance-opioids = 阿片类
pv-substance-stimulants = 兴奋剂
pv-substance-sedatives-hypnotics = 镇静催眠药
pv-substance-hallucinogens = 致幻剂
pv-substance-inhalants = 吸入剂
pv-substance-gambling = 赌博
pv-substance-gaming = 电子游戏
pv-substance-other = 其他
pv-use-pattern-occasional-use = 偶尔使用
pv-use-pattern-regular-use = 经常使用
pv-use-pattern-harmful-use = 有害使用
pv-use-pattern-dependence = 依赖
pv-use-pattern-in-remission = 缓解期
pv-lineage-biological = 亲生
pv-lineage-adoptive = 收养
pv-lineage-foster = 寄养
pv-lineage-step = 继亲
pv-lineage-guardianship = 监护
pv-lineage-unknown = 不详
pv-link-relation-godparent = 教父母
pv-link-relation-godchild = 教子女
pv-link-relation-witness = 见证人
pv-link-relation-officiant = 主礼人
pv-link-relation-business-partner = 生意伙伴
pv-link-relation-employer = 雇主
pv-link-relation-employee = 雇员
pv-link-relation-mentor = 导师
pv-link-relation-apprentice = 学徒
pv-link-relation-close-friend = 挚友
pv-link-relation-neighbour = 邻居
pv-link-relation-guardian = 监护人
pv-link-relation-ward = 被监护人
pv-link-relation-other = 其他
pv-country-AD = 安道尔
pv-country-AE = 阿拉伯联合酋长国
pv-country-AF = 阿富汗
pv-country-AG = 安提瓜和巴布达
pv-country-AI = 安圭拉
pv-country-AL = 阿尔巴尼亚
pv-country-AM = 亚美尼亚
pv-country-AO = 安哥拉
pv-country-AQ = 南极洲
pv-country-AR = 阿根廷
pv-country-AS = 美属萨摩亚
pv-country-AT = 奥地利
pv-country-AU = 澳大利亚
pv-country-AW = 阿鲁巴
pv-country-AX = 奥兰群岛
pv-country-AZ = 阿塞拜疆
pv-country-BA = 波斯尼亚和黑塞哥维那
pv-country-BB = 巴巴多斯
pv-country-BD = 孟加拉国
pv-country-BE = 比利时
pv-country-BF = 布基纳法索
pv-country-BG = 保加利亚
pv-country-BH = 巴林
pv-country-BI = 布隆迪
pv-country-BJ = 贝宁
pv-country-BL = 圣巴泰勒米
pv-country-BM = 百慕大
pv-country-BN = 文莱
pv-country-BO = 玻利维亚
pv-country-BQ = 荷属加勒比区
pv-country-BR = 巴西
pv-country-BS = 巴哈马
pv-country-BT = 不丹
pv-country-BV = 布韦岛
pv-country-BW = 博茨瓦纳
pv-country-BY = 白俄罗斯
pv-country-BZ = 伯利兹
pv-country-CA = 加拿大
pv-country-CC = 科科斯（基林）群岛
pv-country-CD = 刚果（金）
pv-country-CF = 中非共和国
pv-country-CG = 刚果（布）
pv-country-CH = 瑞士
pv-country-CI = 科特迪瓦
pv-country-CK = 库克群岛
pv-country-CL = 智利
pv-country-CM = 喀麦隆
pv-country-CN = 中国
pv-country-CO = 哥伦比亚
pv-country-CR = 哥斯达黎加
pv-country-CU = 古巴
pv-country-CV = 佛得角
pv-country-CW = 库拉索
pv-country-CX = 圣诞岛
pv-country-CY = 塞浦路斯
pv-country-CZ = 捷克
pv-country-DE = 德国
pv-country-DJ = 吉布提
pv-country-DK = 丹麦
pv-country-DM = 多米尼克
pv-country-DO = 多米尼加共和国
pv-country-DZ = 阿尔及利亚
pv-country-EC = 厄瓜多尔
pv-country-EE = 爱沙尼亚
pv-country-EG = 埃及
pv-country-EH = 西撒哈拉
pv-country-ER = 厄立特里亚
pv-country-ES = 西班牙
pv-country-ET = 埃塞俄比亚
pv-country-FI = 芬兰
pv-country-FJ = 斐济
pv-country-FK = 福克兰群岛
pv-country-FM = 密克罗尼西亚
pv-country-FO = 法罗群岛
pv-country-FR = 法国
pv-country-GA = 加蓬
pv-country-GB = 英国
pv-country-GD = 格林纳达
pv-country-GE = 格鲁吉亚
pv-country-GF = 法属圭亚那
pv-country-GG = 根西岛
pv-country-GH = 加纳
pv-country-GI = 直布罗陀
pv-country-GL = 格陵兰
pv-country-GM = 冈比亚
pv-country-GN = 几内亚
pv-country-GP = 瓜德罗普
pv-country-GQ = 赤道几内亚
pv-country-GR = 希腊
pv-country-GS = 南乔治亚和南桑威奇群岛
pv-country-GT = 危地马拉
pv-country-GU = 关岛
pv-country-GW = 几内亚比绍
pv-country-GY = 圭亚那
pv-country-HK = 中国香港特别行政区
pv-country-HM = 赫德岛和麦克唐纳群岛
pv-country-HN = 洪都拉斯
pv-country-HR = 克罗地亚
pv-country-HT = 海地
pv-country-HU = 匈牙利
pv-country-ID = 印度尼西亚
pv-country-IE = 爱尔兰
pv-country-IL = 以色列
pv-country-IM = 马恩岛
pv-country-IN = 印度
pv-country-IO = 英属印度洋领地
pv-country-IQ = 伊拉克
pv-country-IR = 伊朗
pv-country-IS = 冰岛
pv-country-IT = 意大利
pv-country-JE = 泽西岛
pv-country-JM = 牙买加
pv-country-JO = 约旦
pv-country-JP = 日本
pv-country-KE = 肯尼亚
pv-country-KG = 吉尔吉斯斯坦
pv-country-KH = 柬埔寨
pv-country-KI = 基里巴斯
pv-country-KM = 科摩罗
pv-country-KN = 圣基茨和尼维斯
pv-country-KP = 朝鲜
pv-country-KR = 韩国
pv-country-KW = 科威特
pv-country-KY = 开曼群岛
pv-country-KZ = 哈萨克斯坦
pv-country-LA = 老挝
pv-country-LB = 黎巴嫩
pv-country-LC = 圣卢西亚
pv-country-LI = 列支敦士登
pv-country-LK = 斯里兰卡
pv-country-LR = 利比里亚
pv-country-LS = 莱索托
pv-country-LT = 立陶宛
pv-country-LU = 卢森堡
pv-country-LV = 拉脱维亚
pv-country-LY = 利比亚
pv-country-MA = 摩洛哥
pv-country-MC = 摩纳哥
pv-country-MD = 摩尔多瓦
pv-country-ME = 黑山
pv-country-MF = 法属圣马丁
pv-country-MG = 马达加斯加
pv-country-MH = 马绍尔群岛
pv-country-MK = 北马其顿
pv-country-ML = 马里
pv-country-MM = 缅甸
pv-country-MN = 蒙古
pv-country-MO = 中国澳门特别行政区
pv-country-MP = 北马里亚纳群岛
pv-country-MQ = 马提尼克
pv-country-MR = 毛里塔尼亚
pv-country-MS = 蒙特塞拉特
pv-country-MT = 马耳他
pv-country-MU = 毛里求斯
pv-country-MV = 马尔代夫
pv-country-MW = 马拉维
pv-country-MX = 墨西哥
pv-country-MY = 马来西亚
pv-country-MZ = 莫桑比克
pv-country-NA = 纳米比亚
pv-country-NC = 新喀里多尼亚
pv-country-NE = 尼日尔
pv-country-NF = 诺福克岛
pv-country-NG = 尼日利亚
pv-country-NI = 尼加拉瓜
pv-country-NL = 荷兰
pv-country-NO = 挪威
pv-country-NP = 尼泊尔
pv-country-NR = 瑙鲁
pv-country-NU = 纽埃
pv-country-NZ = 新西兰
pv-country-OM = 阿曼
pv-country-PA = 巴拿马
pv-country-PE = 秘鲁
pv-country-PF = 法属波利尼西亚
pv-country-PG = 巴布亚新几内亚
pv-country-PH = 菲律宾
pv-country-PK = 巴基斯坦
pv-country-PL = 波兰
pv-country-PM = 圣皮埃尔和密克隆群岛
pv-country-PN = 皮特凯恩群岛
pv-country-PR = 波多黎各
pv-country-PS = 巴勒斯坦领土
pv-country-PT = 葡萄牙
pv-country-PW = 帕劳
pv-country-PY = 巴拉圭
pv-country-QA = 卡塔尔
pv-country-RE = 留尼汪
pv-country-RO = 罗马尼亚
pv-country-RS = 塞尔维亚
pv-country-RU = 俄罗斯
pv-country-RW = 卢旺达
pv-country-SA = 沙特阿拉伯
pv-country-SB = 所罗门群岛
pv-country-SC = 塞舌尔
pv-country-SD = 苏丹
pv-country-SE = 瑞典
pv-country-SG = 新加坡
pv-country-SH = 圣赫勒拿
pv-country-SI = 斯洛文尼亚
pv-country-SJ = 斯瓦尔巴和扬马延
pv-country-SK = 斯洛伐克
pv-country-SL = 塞拉利昂
pv-country-SM = 圣马力诺
pv-country-SN = 塞内加尔
pv-country-SO = 索马里
pv-country-SR = 苏里南
pv-country-SS = 南苏丹
pv-country-ST = 圣多美和普林西比
pv-country-SV = 萨尔瓦多
pv-country-SX = 荷属圣马丁
pv-country-SY = 叙利亚
pv-country-SZ = 斯威士兰
pv-country-TC = 特克斯和凯科斯群岛
pv-country-TD = 乍得
pv-country-TF = 法属南部领地
pv-country-TG = 多哥
pv-country-TH = 泰国
pv-country-TJ = 塔吉克斯坦
pv-country-TK = 托克劳
pv-country-TL = 东帝汶
pv-country-TM = 土库曼斯坦
pv-country-TN = 突尼斯
pv-country-TO = 汤加
pv-country-TR = 土耳其
pv-country-TT = 特立尼达和多巴哥
pv-country-TV = 图瓦卢
pv-country-TW = 台湾
pv-country-TZ = 坦桑尼亚
pv-country-UA = 乌克兰
pv-country-UG = 乌干达
pv-country-UM = 美国本土外小岛屿
pv-country-US = 美国
pv-country-UY = 乌拉圭
pv-country-UZ = 乌兹别克斯坦
pv-country-VA = 梵蒂冈
pv-country-VC = 圣文森特和格林纳丁斯
pv-country-VE = 委内瑞拉
pv-country-VG = 英属维尔京群岛
pv-country-VI = 美属维尔京群岛
pv-country-VN = 越南
pv-country-VU = 瓦努阿图
pv-country-WF = 瓦利斯和富图纳
pv-country-WS = 萨摩亚
pv-country-YE = 也门
pv-country-YT = 马约特
pv-country-ZA = 南非
pv-country-ZM = 赞比亚
pv-country-ZW = 津巴布韦
pv-country-SU = 苏联
pv-country-DD = 东德
pv-country-YU = 南斯拉夫
pv-country-CS = 捷克斯洛伐克
pv-country-OT = 奥斯曼帝国
lang-aa = 阿法尔语
lang-ab = 阿布哈西亚语
lang-ae = 阿维斯塔语
lang-af = 南非荷兰语
lang-ak = 阿肯语
lang-am = 阿姆哈拉语
lang-an = 阿拉贡语
lang-ar = 阿拉伯语
lang-as = 阿萨姆语
lang-av = 阿瓦尔语
lang-ay = 艾马拉语
lang-az = 阿塞拜疆语
lang-ba = 巴什基尔语
lang-be = 白俄罗斯语
lang-bg = 保加利亚语
lang-bi = 比斯拉马语
lang-bm = 班巴拉语
lang-bn = 孟加拉语
lang-bo = 藏语
lang-br = 布列塔尼语
lang-bs = 波斯尼亚语
lang-ca = 加泰罗尼亚语
lang-ce = 车臣语
lang-ch = 查莫罗语
lang-co = 科西嘉语
lang-cr = 克里族语
lang-cs = 捷克语
lang-cu = 教会斯拉夫语
lang-cv = 楚瓦什语
lang-cy = 威尔士语
lang-da = 丹麦语
lang-de = 德语
lang-dv = 迪维希语
lang-dz = 宗卡语
lang-ee = 埃维语
lang-el = 希腊语
lang-en = 英语
lang-eo = 世界语
lang-es = 西班牙语
lang-et = 爱沙尼亚语
lang-eu = 巴斯克语
lang-fa = 波斯语
lang-ff = 富拉语
lang-fi = 芬兰语
lang-fj = 斐济语
lang-fo = 法罗语
lang-fr = 法语
lang-fy = 西弗里西亚语
lang-ga = 爱尔兰语
lang-gd = 苏格兰盖尔语
lang-gl = 加利西亚语
lang-gn = 瓜拉尼语
lang-gu = 古吉拉特语
lang-gv = 马恩语
lang-ha = 豪萨语
lang-he = 希伯来语
lang-hi = 印地语
lang-ho = 希里莫图语
lang-hr = 克罗地亚语
lang-ht = 海地克里奥尔语
lang-hu = 匈牙利语
lang-hy = 亚美尼亚语
lang-hz = 赫雷罗语
lang-ia = 国际语
lang-id = 印度尼西亚语
lang-ie = 国际文字（E）
lang-ig = 伊博语
lang-ii = 四川彝语
lang-ik = 伊努皮克语
lang-io = 伊多语
lang-is = 冰岛语
lang-it = 意大利语
lang-iu = 因纽特语
lang-ja = 日语
lang-jv = 爪哇语
lang-ka = 格鲁吉亚语
lang-kg = 刚果语
lang-ki = 吉库尤语
lang-kj = 宽亚玛语
lang-kk = 哈萨克语
lang-kl = 格陵兰语
lang-km = 高棉语
lang-kn = 卡纳达语
lang-ko = 韩语
lang-kr = 卡努里语
lang-ks = 克什米尔语
lang-ku = 库尔德语
lang-kv = 科米语
lang-kw = 康沃尔语
lang-ky = 柯尔克孜语
lang-la = 拉丁语
lang-lb = 卢森堡语
lang-lg = 卢干达语
lang-li = 林堡语
lang-ln = 林加拉语
lang-lo = 老挝语
lang-lt = 立陶宛语
lang-lu = 鲁巴加丹加语
lang-lv = 拉脱维亚语
lang-mg = 马拉加斯语
lang-mh = 马绍尔语
lang-mi = 毛利语
lang-mk = 马其顿语
lang-ml = 马拉雅拉姆语
lang-mn = 蒙古语
lang-mr = 马拉地语
lang-ms = 马来语
lang-mt = 马耳他语
lang-my = 缅甸语
lang-na = 瑙鲁语
lang-nb = 书面挪威语
lang-nd = 北恩德贝勒语
lang-ne = 尼泊尔语
lang-ng = 恩东加语
lang-nl = 荷兰语
lang-nn = 挪威尼诺斯克语
lang-no = 挪威语
lang-nr = 南恩德贝勒语
lang-nv = 纳瓦霍语
lang-ny = 齐切瓦语
lang-oc = 奥克语
lang-oj = 奥吉布瓦语
lang-om = 奥罗莫语
lang-or = 奥里亚语
lang-os = 奥塞梯语
lang-pa = 旁遮普语
lang-pi = 巴利语
lang-pl = 波兰语
lang-ps = 普什图语
lang-pt = 葡萄牙语
lang-qu = 克丘亚语
lang-rm = 罗曼什语
lang-rn = 隆迪语
lang-ro = 罗马尼亚语
lang-ru = 俄语
lang-rw = 卢旺达语
lang-sa = 梵语
lang-sc = 萨丁语
lang-sd = 信德语
lang-se = 北方萨米语
lang-sg = 桑戈语
lang-sh = 塞尔维亚-克罗地亚语
lang-si = 僧伽罗语
lang-sk = 斯洛伐克语
lang-sl = 斯洛文尼亚语
lang-sm = 萨摩亚语
lang-sn = 绍纳语
lang-so = 索马里语
lang-sq = 阿尔巴尼亚语
lang-sr = 塞尔维亚语
lang-ss = 斯瓦蒂语
lang-st = 南索托语
lang-su = 巽他语
lang-sv = 瑞典语
lang-sw = 斯瓦希里语
lang-ta = 泰米尔语
lang-te = 泰卢固语
lang-tg = 塔吉克语
lang-th = 泰语
lang-ti = 提格利尼亚语
lang-tk = 土库曼语
lang-tl = 他加禄语
lang-tn = 茨瓦纳语
lang-to = 汤加语
lang-tr = 土耳其语
lang-ts = 聪加语
lang-tt = 鞑靼语
lang-tw = 契维语
lang-ty = 塔希提语
lang-ug = 维吾尔语
lang-uk = 乌克兰语
lang-ur = 乌尔都语
lang-uz = 乌兹别克语
lang-ve = 文达语
lang-vi = 越南语
lang-vo = 沃拉普克语
lang-wa = 瓦隆语
lang-wo = 沃洛夫语
lang-xh = 科萨语
lang-yi = 意第绪语
lang-yo = 约鲁巴语
lang-za = 壮语
lang-zh = 中文
lang-zu = 祖鲁语
person-tab-profile = 档案
profile-groups-label = 档案分类
profile-group-withheld = 此分类中有部分内容对你隐藏
profile-withheld = 已为此人记录但对你隐藏：{ $classes }。
profile-empty = 此分类尚无记录。
profile-earlier = 旧表单
profile-earlier-title = 由本应用的早期版本记录在 AXGF 1.1 没有对应位置的字段中，按原样保留。
profile-other-names = 另有 { $n } 个名字
profile-edit-group = 编辑“{ $group }”
profile-summary-link = 档案中有 { $n } 条记录
profile-from = 自
profile-until = 至
profile-yes = 是
profile-no = 否
profile-value = 值
profile-editor-title = 档案
profile-group-filled = 已填 { $n } 项
profile-problems = 部分输入未能保存。每个问题都标在对应字段旁边，没有写入任何内容。
profile-editor-withheld = 此分类中还有此人的“{ $classes }”数据，你无权查看。这些数据不会在此显示，保存此表单也不会改动它们。
profile-living-class-note = 此人被记录为在世。你在此处填写的敏感类别内容仅管理员可见。
profile-relationships-elsewhere = 亲属、教父母和证人记录在家庭、关系和事件中：在此修改也会修改这些记录。
profile-documents-first = 成果引用的是附在此人名下的文件。请先上传文件。
profile-editor-nothing = 此分类中没有你可以编辑的内容。
profile-new-entry = 新条目
profile-provenance = 日期、来源与可信度
profile-from-date = 有效起始
profile-until-date = 有效截止
profile-remove-entry = 删除此条目
profile-add-entry = 再添加一条
profile-no-such-group-title = 没有这个分类
profile-no-such-group-detail = 档案中没有这个名称的分类。
profile-error-number = 此字段的值应为数字。
profile-error-integer = 此字段的值应为整数。
profile-error-range = 有数字超出了此属性允许的范围。
profile-error-term = 有值不在可选项之中。
profile-error-required = 有条目缺少必填字段。
profile-error-one-of = 条目至少需要填写一个主要字段。
profile-error-confidence = 可信度取值 0 到 1，例如 0.8。
profile-error-time = 时间按时和分填写，例如 05:40。
profile-error-currency = 货币请填写三个字母的代码，例如 CNY。
profile-error-language = 语言请填写代码，例如 zh-Hans 或 pl。
profile-error-coordinates = 坐标需要 −90 到 90 之间的纬度和 −180 到 180 之间的经度。
profile-error-rank-country = 该军衔属于另一个国家，而非所选国家。
record-unknown-place = ［未知地点］
record-missing-document = ［缺失的文件］

## Interface

confidence-certain = 可信度 { $percent }% — 几乎确定
confidence-high = 可信度 { $percent }% — 证据充分
confidence-medium = 可信度 { $percent }% — 可能但未证实
confidence-low = 可信度 { $percent }% — 推测
tree-edge-union-between = { $from } 与 { $to } — { $confidence }
tree-edge-parentage-of = { $from } 是 { $to } 的父母 — { $confidence }
record-note-biography = 生平
record-note-birth-date-as-recorded = 记录中的出生日期
record-note-death-date-as-recorded = 记录中的死亡日期
record-note-event-date-as-recorded = 记录中“{ $event }”的日期
record-unknown-source = ［未知来源］
record-untitled-source = ［无标题来源］
record-unnamed = ［无名］
record-untitled = ［无名称］
record-period-from = 自 { $date }
record-period-until = 至 { $date }
record-dates-unrecorded = 日期未记录
record-link-unlabelled = 关联到
record-link-reverse = { $label }（的对方）
record-place-worked-as = 担任{ $title }
record-place-married-to = 与{ $name }结婚
record-place-married = 结婚
record-source-use-name = 名字“{ $name }”
record-source-use-working-as = 担任{ $title }的工作
record-source-use-union-with = 与{ $name }的婚姻关系
record-source-use-union = 婚姻关系
record-lifespan-born = 生于 { $year }
record-lifespan-died = 卒于 { $year }
size-bytes = { $n } 字节
size-kb = { $n } KB
size-mb = { $n } MB
size-gb = { $n } GB
calendar-gregorian = 公历
calendar-julian = 儒略历
calendar-hebrew = 希伯来历
calendar-hijri = 伊斯兰历
calendar-persian = 波斯历
calendar-chinese = 农历
calendar-ethiopian = 埃塞俄比亚历
calendar-japanese_era = 日本年号
calendar-republican_french = 法国共和历
calendar-roman = 罗马历
diff-summary-none = 未修改任何字段
diff-summary-one = 修改了 { $a }
diff-summary-two = 修改了 { $a } 和 { $b }
diff-summary-many = 修改了 { $a }、{ $b } 等另外 { $n } 个字段
diff-saved-none = 没有字段被修改
diff-saved-one = 已修改 { $a }
diff-saved-two = 已修改 { $a } 和 { $b }
diff-saved-many = 已修改 { $a }、{ $b } 等另外 { $n } 个字段
diff-section-other = 其他字段
diff-section-avatar = 肖像照片
diff-section-partners = 伴侣
diff-section-ai = AI 辅助
diff-section-birth = 出生
diff-section-civil-status = 婚姻状况
diff-section-documents = 附加文档
diff-section-extensions = 应用数据
diff-section-children = 子女
diff-section-union = 结合
diff-section-date = 日期
diff-section-participants = 参与者
diff-section-from = 链接自
diff-section-to = 链接到
diff-section-valid-from = 有效起始
diff-section-valid-until = 有效截止
diff-section-visibility = 可见性
diff-section-employer = 雇主
diff-section-title-normalized = 规范化头衔
diff-section-conflicts = 相互矛盾的来源
diff-section-dna = DNA
diff-section-document = 文档
diff-section-language = 语言
diff-section-place = 地点
diff-section-repository = 收藏机构
diff-section-script = 文字
diff-section-coordinates = 坐标
diff-section-country-history = 国家沿革
diff-section-identifiers = 标识符
diff-section-names = 名称
diff-section-file = 文件
diff-section-linked-to = 附加于
diff-section-ocr = 识别出的文本
diff-section-format-version = 格式版本
history-created = 创建了此记录
history-deleted = 删除了此记录
history-merged = 并入了另一条记录
history-setup = 启用了档案
history-attached = 附加了一个文件
admin-raw-json-unparsed = 无法读取原始 JSON（{ $error }）。未保存任何内容。
admin-raw-json-dangling = 原始 JSON 中的 { $field } 指向 { $target }，但本档案中没有它。请在上方的字段中选择，或更正该 ID。未保存任何内容。
conflict-someone = 有人
conflict-unrecorded-time = 未记录的时间
dedup-merged-persons = 合并了 { $n } 个人物
dedup-merged-families = 合并了 { $n } 个家庭
dedup-manual-review = { $n } 个情况留待人工审阅
dedup-nothing = 没有需要报告的内容。
record-union-duplicate = 一对夫妻，多条记录。
record-union-duplicate-detail = 存档为这两人保存了不同的家庭记录。这是数据缺陷，而非第二段结合。
dedup-pair-merged = 所指的一对现已合并为一条记录。
dedup-pair-refused = 所指的一对未被合并：库拒绝合并，留待人工审核。

validate-errors = { $n } 个错误
validate-warnings = { $n } 个警告
validate-notes = { $n } 条提示
validate-nothing = 没有需要报告的内容。
list-separator = { "、" }
result-written = 档案已写入磁盘。
result-refused = 程序库拒绝了此操作。磁盘上的档案没有变化。
convert-error-no-file = 没有上传文件。请先选择一个 .ged 文件。
convert-error-file-too-large = 该文件为 { $size } MB，上限为 { $limit } MB。未转换任何内容。
convert-error-too-large = 上传内容超过 { $limit } MB 的上限。未转换任何内容。
convert-error-unreadable = 无法读取上传内容（{ $error }）。未转换任何内容。
convert-error-not-gedcom = 这看起来不是 GEDCOM 文件：GEDCOM 5.5.1 文件以“0 HEAD”一行开头。未转换任何内容。
gedcom-read-title = 文件的读取方式
gedcom-read-as = 按 GEDCOM { $version } 读取，编码为 { $encoding }。
gedcom-read-as-unversioned = 按 GEDCOM 读取（文件头未注明版本），编码为 { $encoding }。
gedcom-read-declared = 文件头声明为 { $declared }，但字节内容并非如此，已按实际内容读取。
gedcom-read-gedzip = GEDCOM 7 归档（.gdz）：已读取其中的数据集。其中未带入的其他文件（照片、扫描件）：{ $n }
gedcom-read-crs = 以旧式麦金塔换行结尾、已修复的行：{ $n }
gedcom-read-voids = 指向“无记录”占位符（GEDCOM 7）的引用已搁置，未当作人物读取：{ $n }
gedcom-read-notes = 作为注释读取的共享注释：{ $n }
gedcom-read-stray = 此 UTF-8 文件中不是 UTF-8 的字节（逐个按 Windows-1252 读取）：{ $n }
gedcom-read-tidied = 以转换器不读的方式书写、已修复的行（缩进、字段之间的制表符或多余空格、小写标签、CONC 连接前的空格、控制字符、公历标记）：{ $n }
gedcom-read-too-deep = 嵌套深度超过任何 GEDCOM 结构、连同内容一并搁置的行：{ $n }
convert-error-no-dataset = 此压缩包中既没有 gedcom.ged，也没有可以代替读取的单个 .ged 文件。
convert-error-refused = 转换程序拒绝了此文件。下面的诊断说明了原因。没有保存任何内容。
convert-error-packaging = 文件已转换，但无法打包（{ $error }）。
completeness-fraction = { $whole } 项中的 { $part } 项
event-category-adoption = 收养
event-category-migration = 迁徙
event-category-naturalization = 入籍
event-category-incarceration = 入狱
event-category-name_change = 更名
event-category-legal = 法律事务
event-category-religious = 宗教事件
event-category-social = 社会事件
event-category-historical = 历史事件
precision-quarter_century = 精确到四分之一世纪
source-type-birth_certificate = 出生证明
source-type-death_certificate = 死亡证明
source-type-marriage_certificate = 结婚证明
source-type-census = 人口普查
source-type-baptism_record = 洗礼记录
source-type-burial_record = 安葬记录
source-type-will = 遗嘱
source-type-land_record = 地籍记录
source-type-military_record = 军籍记录
source-type-immigration_record = 移民记录
source-type-naturalization = 入籍记录
source-type-passport = 护照
source-type-photograph = 照片
source-type-letter = 信件
source-type-diary = 日记
source-type-newspaper = 报纸
source-type-oral_tradition = 口述传承
source-type-dna = DNA 检测
source-type-family_bible = 家庭圣经
source-type-gravestone = 墓碑
source-type-published_genealogy = 已出版的家谱
source-type-other = 其他来源
source-status-verified = 已与原件核对
source-status-unverified = 尚未核对
source-status-lost = 已遗失
source-status-known_missing = 已知缺失
document-type-birth_certificate = 出生证明
document-type-death_certificate = 死亡证明
document-type-marriage_certificate = 结婚证明
document-type-census_page = 人口普查页
document-type-baptism_record = 洗礼记录
document-type-military_record = 军籍记录
document-type-will = 遗嘱
document-type-land_record = 地籍记录
document-type-diary = 日记
document-type-newspaper_clipping = 剪报
document-type-gravestone_photo = 墓碑照片
document-type-family_tree_drawing = 手绘家谱图
document-type-audio = 录音
document-type-video = 录像
document-status-present = 存于此处
document-status-referenced = 仅被提及，存于别处
document-status-known_missing = 已知缺失
document-status-lost = 已遗失
document-status-unknown = 下落不明
diag-unsupported_spec_version = 档案声明的 AXGF 版本是本程序无法读取的。
diag-invalid_json = 本应是 JSON 的内容无法读取。
diag-invalid_bundle_structure = 档案的组织方式不符合 AXGF 的要求。
diag-schema_validation_failed = 有一条记录不符合 AXGF 模式。
diag-dangling_reference = 有一条记录指向档案中不存在的另一条记录。
diag-duplicate_entity_id = 两条记录使用了同一个标识。
diag-duplicate_unique_ref = 两条记录声明了同一个本应唯一的引用。
diag-cycle_detected = 家庭关系绕成了一个圈：有人会成为自己的祖先。
diag-chronology_conflict = 日期互相矛盾，例如子女早于父母出生。
diag-out_of_vocabulary = 某个值不在其列表允许的词语之中。
diag-claim_inconsistent = 某项陈述自相矛盾，或与关于同一事物的另一项陈述矛盾。
diag-spec_version_mismatch = 记录声明的 AXGF 版本与其内容不符。
diag-unknown_attribute = 记录中有 AXGF 未定义的属性。
diag-entity_not_found = 要修改的记录不在档案中。
diag-entity_already_exists = 已存在使用此标识的记录。
diag-unknown_entity_kind = AXGF 没有这种记录类型。
diag-delete_blocked_by_reference = 只要还有其他记录指向它，这条记录就无法删除。
diag-manual_review_required = 需要由人来查看；未作自动修改。
diag-zip_read_error = 无法读取档案文件。
diag-zip_write_error = 无法写入档案文件。
diag-payloads_external = 附加文件保存在档案数据之外。
diag-payload_source_failed = 无法读取一个附加文件。
diag-payload_sink_failed = 无法写入一个附加文件。
diag-gedcom_parse_error = 无法理解 GEDCOM 文件中的一行。
diag-gedcom_unrecognized_tag = GEDCOM 文件使用了导入程序不认识的标签，因此该条目未被导入。
diag-internal = 程序库内部出了问题。
diag-merge_refused = 这两条记录目前无法合并。
diag-merge_unresolved = 两条记录之间的分歧尚未决定。
diag-merge_unsupported = 此类记录无法合并。
diag-stale_plan = 制定计划后，其中一条记录已被修改：请重新制定计划。
field-person-display-name = 显示名称
field-person-display-name-hint = 在本站各处显示的名字。
field-person-gender = 性别
field-person-living = 在世
field-person-birth-date = 出生日期
field-date-value-hint = 1923、1923-04 或 1923-04-12；未知则留空。
field-person-birth-precision = 出生日期精度
field-precision-hint = 来源对此确定到何种程度。
field-person-birth-circa = 出生日期为约数
field-circa-hint = 显示为“约1923”，而不是确切的陈述。
field-person-birth-place = 出生地标识
field-person-birth-confidence = 出生的可信度
field-person-confidence-hint = 你有多确定。本站绘制的正是这个值。
field-person-death-date = 死亡日期
field-person-death-precision = 死亡日期精度
field-person-death-circa = 死亡日期为约数
field-person-death-place = 死亡地标识
field-person-death-confidence = 死亡的可信度
field-person-death-cause = 死因
field-person-bio = 生平
field-notes = 笔记
field-family-name = 家庭名称
field-description = 描述
field-family-union-type = 婚姻关系类型
field-family-union-status = 婚姻关系状态
field-family-union-confidence = 婚姻关系可信度
field-family-union-confidence-hint = 决定家谱树中伴侣之间的连线画得多深。
field-family-union-start = 婚姻关系开始
field-family-union-end = 婚姻关系结束
field-family-notes-hint = 伴侣和子女是列表——请在下方的原始 JSON 中编辑，或在此人的关系页面编辑。
field-category = 类别
field-required-hint = 必填。
field-event-subcategory = 子类别
field-date = 日期
field-event-date-hint = 模式要求填写。
field-precision = 精度
field-circa = 约数
field-place-id = 地点标识
field-confidence = 可信度
field-source-id = 来源标识
field-link-from-type = 起点类型
field-link-from-id = 起点标识
field-link-to-type = 终点类型
field-link-to-id = 终点标识
field-link-label = 关系名称
field-link-label-hint = 按关系方向读：“教父”“雇主”“证人”。必填。
field-link-label-reverse = 反向名称
field-link-label-reverse-hint = 从另一端读：“教子”“雇员”。
field-link-bidirectional = 双向读法相同
field-valid-from = 生效起始
field-link-valid-from-hint = 这段关系何时开始。
field-valid-until = 生效截止
field-link-confidence-hint = “据一封家书，有 85% 把握”——这是 GEDCOM 无法表达的。
field-note = 备注
field-occupation-person-id = 人物标识
field-occupation-title = 职业
field-occupation-title-hint = 必填，例如：教师。
field-occupation-title-latin = 职业（拉丁字母）
field-occupation-employer = 雇主
field-occupation-from = 起
field-occupation-from-hint = 职业是一段时间。填写两端才能画成一条横条。
field-occupation-until = 止
field-source-title = 标题
field-source-type = 来源类型
field-source-reliability = 可靠性
field-source-reliability-hint = 必填。以标记形式显示在依据此来源的每一项事实旁边。
field-source-status = 来源状态
field-source-repository = 收藏机构
field-source-repository-reference = 收藏机构中的编号
field-source-transcription = 誊录
field-place-name = 主要名称
field-place-name-lang = 名称的语言
field-place-name-lang-hint = 语言代码，例如 en、fr 或 pl。
field-place-type = 地点类型
field-place-region = 地区
field-place-country-current = 今属国家
field-place-country-current-hint = 其边界沿革是一个列表——请在下方的原始 JSON 中编辑。
field-document-filename = 文件名
field-document-mime-type = 媒体类型
field-document-mime-type-hint = 必填，例如 image/jpeg。
field-document-type = 文件类型
field-document-status = 文件状态
field-document-url = 网址
field-document-caption = 说明
lang-zh-Hans = 简体中文
family-lineage = 亲子关系类型
links-relation = 关系类型
occupations-position = 职位
field-link-relation = 关系类型
field-link-relation-hint = AXGF 1.1 命名的关系之一。上方的名称保留记录原有的措辞。
field-occupation-position = 职位
field-occupation-position-hint = 该职业中担任的职位：例如职业为教师时的校长。
error-delete-changed-title = 在你查看之后已被修改
error-delete-changed-detail = 你从中删除它的页面显示之后，这条记录又被保存过，现在是版本 { $version }。未删除任何内容。请先查看它现在的样子，再重新决定。
error-delete-changed-look = 重新查看
documents-files = 附加在此的文件
documents-files-help = 编辑文件的详细信息，或删除文件本身。删除会从档案中移除该文件及其数据，对所有附加了它的人都是如此；若只想从此人名下移除，请清空上方它所在的行。
documents-edit-details = 编辑详细信息
documents-delete = 删除此文件

## Charts

radar-section = 来自记录的图表
radar-section-help = 对这条记录内容的三种解读，每条轴从 0 到 100。每个数字都依据旁边列出的事实，按应用文档中写明的规则计算；不保存、不猜测，没有可读内容的轴留空，而不是给一个中间分数。
radar-physique = 体格
radar-mind = 气质与心智
radar-vitality = 健康与活力
radar-axis-stature = 身高
radar-axis-build = 体型
radar-axis-lean-mass = 去脂体重
radar-axis-posture = 姿态
radar-axis-gait = 步态
radar-axis-dentition = 牙齿
radar-axis-openness = 开放性
radar-axis-conscientiousness = 尽责性
radar-axis-extraversion = 外向性
radar-axis-agreeableness = 宜人性
radar-axis-stability = 情绪稳定性
radar-axis-cognition = 认知
radar-axis-circulation = 循环
radar-axis-breathing = 呼吸
radar-axis-metabolism = 代谢
radar-axis-illness = 无病程度
radar-axis-senses = 感官
radar-axis-rest = 睡眠与情绪
radar-folded-open = 显示此图表
radar-folded-why = 此人被记录为在世。在世者的气质画像保持折叠，直到有权阅读的人要求查看。
radar-empty = 这条记录中还没有可以用于此图表的内容。
radar-table-caption = { $chart }：各条轴、分数及其依据
radar-col-axis = 轴
radar-col-score = 分数
radar-col-from = 依据
radar-no-score = 无分数
radar-from-none = 无
record-link-outgoing = 由此人发出
record-link-incoming = 指向此人
# 管理面板中的运行警告。
health-attention = 需要处理：
health-standing-token = 紧急令牌仍处于设置状态，可绕过所有账户。请将其从服务的环境文件中删除并重启。
health-bundle-invalid = 家族数据已无法通过校验：{ $errors } 个错误，列在下方“校验”中。
health-disk-unknown = 无法读取本机的可用空间，因此无法保证下一次保存放得下。
health-disk-no-room-to-save = 无法保存：可用 { $free }，而重建这个档案包需要 { $need }。没有任何数据丢失，但在腾出空间之前无法修改。
health-disk-critical = 磁盘可用 { $percent } %——剩余 { $free }。保存很快将无法进行。
health-disk-low = 磁盘可用 { $percent } %——剩余 { $free }。宜在变得紧急之前处理。
health-backup-unconfigured = 没有任何内容被备份。一块磁盘损坏就会带走全部记录。
health-backup-never = 已设置备份目录，但从未写入过任何备份。
health-backup-stale = 最新备份已有 { $days } 天。应当每天写入一次。
health-cache-missing = { $declared } 个附件中有 { $missing } 个不在缓存里，直到下一次保存重建它们之前无法下载。

## River

nav-river = 河流
nav-home = 家族
river-aria = 以 { $name } 为中心的河流：祖先在下，后代在上。
river-band-living = 在世
river-circa = 约
river-trail = 路径
river-back = ← 返回
river-range = 范围
river-motion = 动画
river-travel = 移动
river-cut = 切换
river-legend-attested = 有据
river-legend-inferred = 推断
river-legend-speculative = 推测
river-legend-lost = 记录到此为止
river-legend-repeat = ×n 由 n 条世系到达，在每条之下各绘一次
river-legend-again = ↔ 在后来的伴侣旁再次绘出
river-legend-discharge = 后代数
river-legend-direction = ↓ 向源头 · ↑ 向在世者
view-tree-direction = ↓ 树根：祖先 · ↑ 树冠：后代
view-galaxy-direction = ↓ 太阳下方：祖先 · ↑ 上方：后代
view-summit-direction = ↓ 祖先在山谷 · ↑ 后代向着顶峰
river-legend-generation = 一代 ≈ 29 年
river-zoom = 缩放
river-still = 暂停风景
view-tree-still = 让树静止
view-galaxy-still = 让星空静止
view-summit-still = 让山峰静止
river-whole = 整个家族
river-names-all = 全部姓名
# A tap on a person in a view, on a touch screen: the drawing moves onto them.
river-pick-centre = 以此为中心
river-zoom-hint = 缩放：Ctrl + 滚轮、双指捏合或 + / − · 放大后拖动即可移动
river-tab-sources = 来源
river-legend-title = 如何阅读河流
view-tree-name = 生命之树
view-tree-legend = 如何阅读这棵树
view-galaxy-name = 星系
view-galaxy-legend = 如何阅读星系
view-summit-name = 攀登
view-summit-legend = 如何阅读这座山
hud-ancestors = 祖先
hud-descendants = 后代
hud-generations = 世代
hud-living = 在世
hud-attested = 已证实的关系

# Masthead search
search-label = 搜索人物
search-short = 人物
date-qualifier = 限定词
date-qualifier-circa = 约
search-title = 搜索
search-heading = 与“{ $q }”匹配的人物
search-none = 你能看到的人中没有叫这个名字的。
search-open-record = 打开记录
search-hint = 回车键将河流移到此人；上档键+回车键打开其记录。
view-tree-search-hint = 回车键让树围绕此人生长；上档键+回车键打开其记录。
view-galaxy-search-hint = 回车键让此人成为太阳；上档键+回车键打开其记录。
view-summit-search-hint = 回车键带你前往此人所在；上档键+回车键打开其记录。
search-summary = { $n } 人 · ↵ 前往 · ⇧↵ 打开记录
search-empty-query = 输入姓名进行搜索。

# Data quality (/quality)
nav-quality = 数据质量
quality-title = 值得一看的问题
quality-none = 没有需要查看的问题：所有检查均已通过。
quality-intro = 值得核对的地方；往往有其合理原因。打开记录，或将其标记为已核对。
quality-groups-label = 问题类别
quality-all = 全部
quality-show-label = 显示
quality-show-open-option = 待检查
quality-download = 下载
quality-group-names = 姓名
quality-group-relations = 亲属关系
quality-group-dates = 日期
quality-group-places = 地点
quality-group-evidence = 依据
quality-export-csv = 下载为 CSV
quality-export-json = 下载为 JSON
quality-open = 打开
quality-none-file = 此文件中没有值得一看的问题。
quality-show-list = 显示列表
quality-after-import = 文件中没有任何内容被修改。它成为本站的档案后，数据质量页面会列出相同的问题，并附上每条记录的链接。
quality-file-list-cut = 此处仅列出前 { $shown } 项。文件成为本站的档案后，数据质量页面会全部列出。
quality-rule-name-sentence = 姓名栏中是一句话
quality-name-sentence = “{ $value }”像一句话，而不是姓名。
quality-rule-union-parent-child = 既是夫妻又是父母与子女
quality-union-parent-child = { $parent } 与 { $child }：既是夫妻又是亲子。
quality-rule-child-listed-twice = 子女被列了两次
quality-child-listed-twice = { $child } 在子女中出现两次。
quality-rule-duplicate-union = 同一对夫妻被记录了两次
quality-duplicate-union = { $a } 与 { $b }：记录了 { $count } 段婚姻。
quality-rule-partner-twice = 同一人出现在婚姻的两方
quality-partner-twice = { $person } 出现在婚姻的两边。
quality-rule-foreign-surnames = 姓氏与父母双方都不符的儿子
quality-foreign-surnames = 姓氏与父母双方都不同：{ $who }。
quality-rule-death-before-birth = 死亡早于出生
quality-death-before-birth = { $person }：死亡（{ $death }）早于出生（{ $birth }）。
quality-rule-child-before-parent = 子女出生早于父母
quality-child-before-parent = { $child }（{ $childborn }）早于其父母 { $parent }（{ $parentborn }）出生。
quality-rule-parent-too-young = 不满十二岁的父母
quality-parent-too-young = { $child } 出生时，{ $parent } 年仅 { $age } 岁。
quality-rule-parent-too-old = 超过六十五岁的父母
quality-parent-too-old = { $child } 出生时，{ $parent } 已 { $age } 岁。
quality-rule-marriage-before-birth = 婚姻早于一方出生
quality-marriage-before-birth = 婚姻（{ $married }）早于 { $person } 出生（{ $born }）。
quality-rule-isolated-record = 没有日期也没有亲属关系的记录
quality-isolated-record = { $person }：没有日期，也没有关系。
quality-rule-place-no-region = 没有地区的地点
quality-place-no-region = { $place }：没有地区。
quality-rule-region-free-text = 像备注的地区
quality-region-free-text = { $place } 的地区：“{ $value }”像一条笔记。
quality-rule-place-near-duplicate = 可能是同一处的地点
quality-place-near-duplicate = 可能是同一地点：{ $names }。
quality-rule-not-a-place = 可能不是地点的条目
quality-not-a-place = “{ $place }”可能不是地点。
quality-rule-certain-without-source = 标为确定但没有出处
quality-certain-without-source = { $record }：标为确定（{ $claims }）但没有来源。
quality-dismiss = 已核实——我们这里正常
quality-restore = 重新显示
quality-note-label = 原因（可选）
quality-show-dismissed = 已核实并保留
quality-show-open = 返回待处理列表
quality-dismissed-by = { $by } 于 { $at } 核实
quality-gone-title = 这条问题已经不存在了
quality-gone-detail = 打开列表后记录已有改动，这条问题已不同。请刷新列表后再看。
quality-summary = 值得一看的问题：{ $n } 项
quality-summary-file = 此文件中值得一看的问题：{ $n } 项
quality-dismissed-summary = 已核实并保留的问题：{ $n } 项
quality-dismiss-all = 全部标记为已核实（{ $n }）
quality-restore-all = 全部重新显示（{ $n }）
quality-bulk-conflict = 打开列表后有改动的记录：{ $n } 条。没有标记任何内容；请刷新列表后重试。

## The record's editor (v0.4.2)

edit-tab-changed = 已修改，尚未保存
edit-remove = 移除
edit-add-partner = 添加配偶
edit-add-child = 添加子女
edit-union-with = 与 { $names } 的婚姻
edit-union-alone = 未记录配偶的婚姻
edit-child-of = { $names } 的子女
edit-new-family = 新建家庭
edit-cite-title = 依据来源的事实
edit-attach-to = 引用于
edit-add-source = 添加来源
edit-error-source-title = 新来源需要标题。未保存任何内容。
edit-detach-file = 从此记录中移除
edit-no-sources = 此人自身的事实尚未引用任何来源。
edit-no-files = 此记录尚未附加任何文件。
edit-saved = 已保存：一次写入 { $n } 条记录。
edit-saved-none = 没有任何变化，因此未保存。
edit-back-to-record = 返回记录
edit-avatar = 头像
edit-avatar-frame = 调整面部位置
edit-avatar-upload = 用作头像
edit-stale = 自此页面打开后，其他人修改了 { $what }。未保存任何内容；您输入的内容都还在。
edit-refused = 未保存任何内容：{ $reason }

# Administration console (v0.5.0, docs/adr/0011)
error-cross-site-title = 已拒绝：来自其他网站的提交
error-cross-site-detail = 此表单来自其他网站的页面，因此未予处理。如果是从本站提交的，请刷新页面后重新提交。
admin-nav-label = 管理栏目
admin-nav-overview = 概览
admin-nav-bundle = AXGF 归档
admin-nav-infrastructure = 基础设施
admin-nav-accounts = 账户
admin-nav-connectivity = 连接

# Administration console: the AXGF bundle (v0.5.0, docs/adr/0011)
bundle-title = AXGF 归档
bundle-lede = 家族归档：{ $path }。其合规情况、看似有误之处，以及如何取得副本。
bundle-unset-title = 还没有家族。
bundle-unset-detail = 本安装持有一个空归档，因此暂时没有可检查的内容。可以从归档、GEDCOM 文件或从零开始设置。
bundle-kpi-conformity = 合规性
bundle-verdict-ok = 符合
bundle-verdict-warn = 需关注
bundle-verdict-fail = 不符合
findings-tally = 正常 { $ok } · 警告 { $warn } · 严重 { $fail }
bundle-kpi-entities = 实体
bundle-kpi-entities-note = { $persons } 人 · 附件 { $files } 个
bundle-kpi-size = 磁盘占用
bundle-kpi-members = 归档内文件 { $n } 个
bundle-kpi-format = 格式
bundle-kpi-format-note = 此版本读写 AXGF { $current }。
bundle-conformity = 合规性
bundle-conformity-intro = 对当前归档的九项检查，均不会修改它。展开一行即可查看处理方法。
bundle-area-format = 格式
bundle-area-integrity = 完整性
bundle-area-privacy = 隐私
bundle-area-content = 内容
level-ok = 正常
level-warn = 警告
level-fail = 严重
findings-what-to-do = 处理方法
findings-go = 前往 →
bundle-anomalies = 异常
bundle-anomalies-intro = 数据质量页面按规则列出的发现：仍待处理的，以及已标记为已核查的。
bundle-anomalies-rule = 规则
bundle-anomalies-open = 待处理
bundle-anomalies-checked = 已核查
bundle-anomalies-none = 数据质量页面没有发现需要查看的内容。
bundle-diagnostics = 校验器诊断
bundle-facts = 详细信息
bundle-fact-path = 文件
bundle-fact-size = 大小
bundle-fact-spec = 格式
bundle-fact-schema = 随附的模式
bundle-fact-none = 无
bundle-fact-created = 创建于
bundle-fact-updated = 最后写入
bundle-fact-generator = 写入工具
bundle-fact-gedcom = 转换自
bundle-export = 导出
bundle-export-raw = 原样归档（.axgf）
bundle-export-raw-go = 下载归档
bundle-export-sealed = 为一人加密（.axgf.gpg）
bundle-export-sealed-intro = 同一归档，作为只有私钥持有人才能打开的 OpenPGP 消息。请粘贴对方的公钥：以“-----BEGIN PGP PUBLIC KEY BLOCK-----”开头的文本。
bundle-export-sealed-go = 加密下载
seal-recipient = 收件人
seal-fingerprint = 指纹
seal-algorithm = 算法
seal-fips-yes = 属于 FIPS 批准范围
seal-fips-no = 不在 FIPS 批准范围内
seal-expires = 密钥到期
seal-never = 永不
seal-key-label = 收件人的公钥
seal-key-hint = 至少 2048 位的 RSA 密钥，或椭圆曲线密钥。发送前请与持有人核对指纹。
seal-remember = 保留此密钥以便下次使用
seal-check = 仅读取密钥
seal-fips-note = 归档以 AES-256 加密到收件人的密钥。FIPS 批准的算法是 3072 位及以上的 RSA，以及 NIST 曲线 P-256、P-384 和 P-521：建议使用 P-384。此处使用的库本身并非经 FIPS 140-3 验证的模块。
seal-decrypt-note = 收件人用 GnuPG 打开：
seal-forget = 忘记已保留的密钥
seal-key-kept = 密钥 { $fingerprint } 已保留，供下次加密导出使用。
seal-error-no-key = 请先粘贴收件人的公钥。
seal-error-not-a-key = 这不是 OpenPGP 公钥。请粘贴从 BEGIN 行到 END 行的整个块，切勿粘贴私钥。
seal-error-bad-signatures = 该密钥的自签名无法验证：它可能已被篡改。请向持有人索取一份新的副本。
seal-error-expired = 该密钥或其所有可用于加密的子密钥均已过期。请让持有人延期或发送新密钥。
seal-error-revoked = 持有人已吊销该密钥。请向其索取新密钥。
seal-error-no-encryption-key = 该密钥只能签名不能加密：它没有加密子密钥。请让持有人添加一个（gpg --quick-add-key … encr）。
seal-error-too-weak = 该 RSA 密钥为 { $bits } 位；导出至少需要 { $min } 位。
seal-error-unsupported = 该密钥的算法不在导出支持范围内：仅支持 RSA 或椭圆曲线密钥。
seal-error-io = 无法生成加密副本：{ $error }
date-time-utc = { $date } { $time } UTC

# Administration console: the bundle's conformity (v0.5.0, docs/adr/0011)
bundle-check-format-title = 格式版本
bundle-check-format-ok = AXGF { $version }，即本版本读写的版本。
bundle-check-format-missing = 清单未注明 AXGF 版本。
bundle-check-format-missing-fix = 该归档并非由 AXGF 工具写入，或已损坏。请恢复最近一次通过校验的备份（axgf-cms restore）。
bundle-check-format-older = 以 AXGF { $version } 写入；下次保存将以 AXGF { $current } 写入。
bundle-check-format-older-fix = 无需处理：下次编辑会重写它。如果其他程序只能读取 { $version }，请保留旧文件的副本。
bundle-check-format-newer = 以 AXGF { $version } 写入，比本版本的 { $current } 更新：保存可能丢失本版本无法理解的内容。
bundle-check-format-newer-fix = 在任何人编辑之前，请将 axgf-cms 升级到能读取 AXGF { $version } 的版本（基础设施，版本）。
bundle-check-manifest-title = 清单中的计数
bundle-check-manifest-ok = 清单中的计数与归档内容一致。
bundle-check-manifest-drift = 清单声明为 { $declared }，而归档中实有 { $actual }（{ $collection }）。
bundle-check-manifest-fix = 不会丢失任何内容：计数会在下次保存时重写。如果该归档来自别处，请确认它被完整复制。
bundle-check-validation-title = 校验
bundle-check-validation-ok = 库的校验器未发现任何问题。
bundle-check-validation-errors = { $errors } 个错误：归档未通过校验。
bundle-check-validation-errors-fix = 下方每条诊断都指出了记录和字段。请修正该记录，或恢复最近一次通过校验的备份。
bundle-check-validation-warnings = { $warnings } 条警告。它们从不阻止保存；每条都提示可能有问题。
bundle-check-validation-warnings-fix = 请在下方查看。大多是指向本归档中不存在记录的引用，常由 GEDCOM 导入遗留。
bundle-check-payloads-title = 附件
bundle-check-payloads-none = 没有附件。
bundle-check-payloads-ok = 全部 { $declared } 个文件都在。
bundle-check-payloads-missing = 缓存中缺少 { $declared } 个附件中的 { $missing } 个。
bundle-check-payloads-fix = 下次保存会从归档重建缓存。如果归档本身就缺少这些文件，请从备份中恢复。
bundle-check-binding-title = 账户与归档
bundle-check-binding-ok = 账户文件是为此家族创建的。
bundle-check-binding-mismatch = 账户文件是为 { $expected } 创建的；而此归档是 { $found }。
bundle-check-binding-fix = 如果归档是有意恢复或重命名的，则无需处理。否则一个家族的账户正被用于另一个家族的家谱：请把正确的 .acl 放回旁边。
bundle-check-living-title = 在世者的公开
bundle-check-living-ok = 记录为在世的人对访客均不可见。
bundle-check-living-public = 有 { $n } 位记录为在世的人被标为公开：任何访客都能阅读其记录。
bundle-check-living-fix = 除非本人同意，请将其可见性设为成员（编辑，身份）。
bundle-check-special-title = 特殊类别数据
bundle-check-special-none = 没有记录包含健康、生物识别、基因组、刑事或行为数据。
bundle-check-special-some = 有 { $n } 条记录包含健康、生物识别、基因组、刑事或行为数据。在世者的这类数据无论记录如何设置，都只对管理员可见。
bundle-check-quality-title = 数据质量
bundle-check-quality-ok = 没有需要查看的内容。
bundle-check-quality-open = 有 { $n } 项需要查看，分布在 { $groups } 个类别中。
bundle-check-quality-fix = 打开数据质量页面：每一行都指出了记录及看似有误之处。请修正它，或将其标记为已核查。
bundle-check-size-title = 大小
bundle-check-size-ok = { $size }，低于本页开始提醒的 { $warn }。
bundle-check-size-heavy = { $size }，超过 { $warn }。
bundle-check-size-fix = 归档属于家族，可以不断增长。请留意磁盘和备份（基础设施，容量），如果这个大小在预期之内，可调高 --size-warn-mb。
seal-algorithm-value = { $key } · AES-256
seal-key-placeholder = -----BEGIN PGP PUBLIC KEY BLOCK-----

# Administration console: the infrastructure's checks (v0.5.0, docs/adr/0011 §3)
infra-area-security = 安全
infra-area-access = 访问
infra-area-data = 数据
infra-area-capacity = 容量
infra-area-release = 版本
infra-area-platform = 平台
infra-verdict-ok = 正常
infra-verdict-warn = 需要关注
infra-verdict-fail = 严重
duration-days = { $n } 天
duration-hours = { $n } 小时
duration-minutes = { $n } 分钟
infra-transport-title = 站点的访问方式
infra-transport-ok = 此页面通过 HTTPS 送达。
infra-transport-exposed = 此页面以明文送达，且服务器在 { $bind } 上监听、可从本机以外访问：密码和会话 Cookie 以明文穿过网络。
infra-transport-exposed-fix = 在前面放置 HTTPS 反向代理（Caddy、nginx、HAProxy），并把服务重新绑定到 127.0.0.1：用 --bind 127.0.0.1:8080 重新运行 bootstrap.sh，或删除放宽绑定的 drop-in。
infra-transport-plain = 未加密，来自 { $bind }：仅在本机可以接受。
infra-transport-plain-fix = 请通过站点的 HTTPS 地址访问。如果由代理终止 TLS，请让它发送 X-Forwarded-Proto: https，以便本服务器知晓。
infra-forwarded-title = 转发的地址
infra-forwarded-ok = 服务器只在本机监听，因此代理转发的客户端地址可信，可用于减缓反复失败的登录。
infra-forwarded-exposed = 服务器可被直接访问，且信任 X-Forwarded-For 头：自行设置该头的客户端可以绕过登录失败后的限速。
infra-forwarded-fix = 在代理之后将服务绑定到 127.0.0.1，或用防火墙限制端口，使只有代理能访问。
infra-cross-site-title = 来自其他站点的提交
infra-cross-site-ok = 过去一天内没有从其他站点向这里提交表单。
infra-cross-site-refused = 过去一天内拒绝了 { $n } 个来自其他站点的表单提交。
infra-cross-site-fix = 没有任何提交被放行。如果被拒绝的是你自己（用另一个地址访问同一站点），请在“连接”中设置公开地址，或只使用该地址。
infra-sso-title = 登录服务
infra-sso-none = 未启用任何登录服务；账户使用密码登录。
infra-sso-ok = 已启用 { $n } 个登录服务，公开地址为 HTTPS。
infra-sso-no-https = 已启用登录服务，但公开地址不是 HTTPS：提供方拒绝把应答发送到明文地址，登录会失败。
infra-sso-fix = 在“连接”中，把公开地址设为读者使用的 https:// 地址。
infra-admins-title = 管理员
infra-admins-none = 没有任何活动账户能管理本站；只有紧急令牌可以。
infra-admins-none-fix = 用紧急令牌登录，创建管理员，然后删除令牌。
infra-admins-one = 只有一名管理员。如果此人不在或丢失密码，就没有人能管理站点。
infra-admins-one-fix = 在“账户”中把管理员角色授予第二位可信的人。
infra-admins-ok = { $n } 名管理员可以登录。
infra-token-title = 紧急令牌
infra-token-generated = 未设置紧急令牌。
infra-token-only-way = 在管理员账户存在之前，紧急令牌是唯一入口。
infra-token-standing = 紧急令牌仍处于设置状态。它可绕过所有账户和权限登录。
infra-token-fix = 从服务的环境文件中删除 AXGF_CMS_ADMIN_TOKEN，然后重启服务。
infra-registration-title = 自助注册
infra-registration-closed = 访客无法创建账户；只有管理员可以。
infra-registration-secret = 访客如果知道注册密语，就可以创建成员账户。
infra-registration-open = 任何访客都可以创建成员账户。
infra-registration-open-fix = 对有意开放的站点来说这没问题。否则请在“账户”中要求注册密语，或关闭注册。
infra-failures-title = 登录失败
infra-failures-ok = 过去一天内有 { $n } 次登录失败。
infra-failures-many = 过去一天内有 { $n } 次登录失败：可能有人在猜密码。
infra-failures-fix = 每个地址在多次失败后已被限速。如果次数仍在上升，请在代理日志中查出尝试来源并在那里封禁，并确保每位管理员都使用长密码或通过登录服务登录。
infra-health-bundle-title = 家族数据
infra-health-bundle-ok = 归档可以打开并通过校验。
infra-health-bundle-fix = 打开“AXGF 归档”页面查看并修正错误，或恢复最近一次通过校验的备份（axgf-cms restore）。
infra-health-cache-title = 附件缓存
infra-health-cache-ok = 每个附件都在缓存中。
infra-health-cache-fix = 随便保存一条记录：保存会重建缓存。之后仍缺失的文件，是归档本身就缺失的。
infra-health-backup-title = 备份
infra-health-backup-ok = 存在近期备份；共保留 { $count } 个归档。
infra-health-backup-fix = 如果没有备份目录，请带 --backup-dir 重新运行 bootstrap.sh。如果有，请检查定时器（systemctl status axgf-cms-backup.timer）及其日志（journalctl -u axgf-cms-backup）。
infra-disk-title = 磁盘空间
infra-disk-ok = { $total } 中有 { $free } 可用（{ $percent }% 空闲）。
infra-disk-fix = 清理数据目录所在分区的空间（也存放在别处的旧备份、日志、其他程序的文件），或把数据迁移到更大的磁盘。
infra-growth-title = 增长
infra-growth-ok = 每月约增长 { $rate }；按此速度还够用 { $days } 天。
infra-growth-soon = 每月约增长 { $rate }：按此速度磁盘将在 { $days } 天后写满。
infra-growth-fix = 现在就规划空间：换更大的磁盘、减少同一磁盘上的备份，或把备份移到另一块磁盘。
infra-growth-flat = 备份没有增长；无需规划。
infra-growth-unknown = 备份还不够，无法测量增长：最旧与最新的备份之间至少需要相隔一天。
infra-memory-title = 内存
infra-memory-ok = 已使用服务上限 { $max } 中的 { $used }（{ $percent }%）。
infra-memory-high = 已使用服务上限 { $max } 中的 { $used }（{ $percent }%）：接近系统会停止服务的程度。
infra-memory-fix = 在 axgf-cms.service 的 drop-in 中提高 MemoryMax=，或找出占用内存的原因（超大的导入、很大的附件）并重启服务。
infra-memory-unlimited = 正在使用 { $used }，未给服务设置上限。
infra-version-title = 版本
infra-version-ok = v{ $current } 是已发布的最新版本。
infra-version-behind = 正在运行 v{ $current }；已发布 { $latest }。
infra-version-behind-fix = 请先阅读发布说明，然后在下方“版本”部分升级。升级前会先备份；新版本无法启动时会自动回到旧版本。
infra-version-unknown = 不知道是否有更新版本：尚未获取版本列表。
infra-version-unknown-fix = 请使用下方的“检查新版本”。生产环境的沙箱本身无法访问 GitHub；安装升级助手后，由助手每天获取列表。
infra-version-stale = 版本列表上次获取于 { $days } 天前。
infra-upgrades-title = 升级
infra-upgrades-ok = 升级助手已安装：下方按钮可以切换版本。
infra-upgrades-manual = 升级助手未安装，因此需要在服务器上手动切换版本。
infra-upgrades-fix = 以 root 身份运行一次当前的 bootstrap.sh：它会连同服务一起安装助手。
infra-sandbox-title = 沙箱
infra-sandbox-ok = 进程在 no-new-privileges 和系统调用过滤下运行。
infra-sandbox-unknown = 此系统不报告进程的沙箱情况。
infra-sandbox-off = 进程缺少部分沙箱：no-new-privileges 或系统调用过滤已关闭。
infra-sandbox-fix = 请使用 bootstrap.sh 写入的单元运行服务，不要用放宽限制的 drop-in。在开发机器上出现这种情况是正常的。
infra-clock-title = 时钟
infra-clock-ok = 系统时钟显示 { $year } 年，看起来合理。
infra-clock-wrong = 系统时钟显示 { $year } 年：日期、备份时长和会话都会出错。
infra-clock-fix = 请开启时间同步：timedatectl set-ntp true。

# Administration console: the Infrastructure page (v0.5.0, docs/adr/0011)
infra-title = 基础设施
infra-lede = 本站所在的机器：访问、密钥、数据、版本与沙箱，每次打开本页都会检查。
infra-kpi-health = 健康状况
infra-kpi-version = 版本
infra-kpi-version-newer = { $version } 可用
infra-kpi-version-current = 已是最新
infra-kpi-version-unknown = 尚未检查
infra-kpi-disk = 可用空间
infra-kpi-disk-note = 已用 { $total } 的 { $percent }%
infra-kpi-compute = 机器
infra-kpi-cpus = { $n } 核
infra-kpi-load = 负载 { $load }
infra-kpi-rss = 已用 { $size }
infra-unknown = 未知
infra-health = 健康状况
infra-health-intro = 绿色表示正常，警告值得一看，严重需要立即处理。
infra-json = 同样的检查（JSON 格式，供监控使用）
infra-version = 版本
infra-version-running = 正在运行
infra-version-latest = 最新发布
infra-version-up-to-date = 已是最新
infra-version-checked = 列表获取时间
infra-version-never = 从未
infra-source-direct = 由本服务器获取
infra-source-helper = 由升级助手获取
infra-version-token = 私有仓库
infra-token-env-set = 已设置用于列出和下载版本的 GitHub 令牌。
infra-token-env-unset = 没有令牌。如果仓库是私有的，请把令牌放在只有 root 可读的 { $name } 中：升级助手会从那里读取。
infra-releases-error = 上次获取列表失败：
infra-releases-refresh = 检查新版本
infra-releases-asked = 已请升级助手获取新列表。请一分钟后刷新本页。
infra-releases-not-asked = 无法写入请求：{ $error }
infra-upgrade-status-running = 正在从 { $from } 切换到 { $to }…
infra-upgrade-status-succeeded = 已从 { $from } 切换到 { $to }。
infra-upgrade-status-failed = 从 { $from } 切换到 { $to } 失败；仍在运行 { $from }。
infra-upgrade-status-refused = 升级助手拒绝切换到 { $to }。
infra-upgrade-log = 助手输出
infra-upgrade-pending = { $by } 于 { $at } 请求了 { $version }；升级助手尚未接手。
infra-releases = 已发布版本
infra-releases-tag = 版本
infra-releases-published = 发布日期
infra-releases-relation = 与当前相比
infra-releases-notes = 发布说明
infra-releases-open = 阅读
infra-relation-newer = 较新
infra-relation-current = 当前
infra-relation-older = 较旧
infra-prerelease = 候选版本
infra-releases-none = 尚无版本列表。
infra-upgrade = 切换版本
infra-upgrade-intro = 升级助手会先备份，安装经过校验的版本，重启并检查；若失败，则恢复上一版本。
infra-upgrade-version = 要安装的版本
infra-upgrade-confirm = 要回到较旧的版本，请再次输入其标签
infra-upgrade-confirm-hint = 升级到较新版本时无需填写。较旧版本可能无法读取较新版本写入的数据：请先阅读其说明。
infra-upgrade-go = 安装此版本
infra-upgrade-safety = 服务重启期间，站点会有几秒钟不可用。
infra-upgrade-manual = 此服务器未安装升级助手，因此本页面无法切换版本。请在服务器上以 root 身份执行：
infra-upgrade-manual-helper = 运行一次当前的 bootstrap.sh 以安装助手。
infra-upgrade-asked = 已请求 { $version }。升级助手会在几秒内接手；结果会显示在本页。
infra-upgrade-no-helper = 升级助手未安装，因此无法从这里安装 { $version }。
infra-upgrade-unknown-version = { $version } 不在已发布版本列表中。
infra-upgrade-same-version = { $version } 就是当前正在运行的版本。
infra-upgrade-confirm-older = { $version } 比正在运行的版本旧。要回到该版本，请在确认栏中输入其标签。
infra-volumes = 存储
infra-volumes-intro = 数据目录（{ $path }）所在的分区，以及本站在其上保存的内容。
infra-volumes-partition = 分区
infra-of = { $total } 中的 { $used }
infra-volumes-ours = 本站
infra-volumes-breakdown = 本站保存的内容
infra-part-bundle = 家族归档
infra-part-cache = 附件缓存
infra-part-backups = 备份
infra-part-binary = 程序
infra-part-journal = 变更日志
infra-part-accounts = 账户与设置
infra-files = { $n } 个文件
infra-growth = 增长
infra-growth-month = 每月
infra-growth-not-enough = 备份不足，无法判断
infra-growth-left = 按此速度剩余
infra-backups = 备份
infra-backups-dir = 目录
infra-backups-count = 归档
infra-backups-newest = 最新
infra-backups-oldest = 最旧
infra-backups-room = 存放位置的空间
infra-free-of = { $total } 中有 { $free } 可用
infra-paths = 路径
infra-path-data = 数据目录
infra-path-binary = 程序
infra-compute = 计算资源
infra-compute-hidden = 服务的沙箱对其隐藏了机器本身的数据；这里显示的是进程能看到的内容。
infra-compute-memory = 机器内存
infra-available-of = { $total } 中有 { $available } 可用
infra-compute-ceiling = 服务内存
infra-compute-no-ceiling = { $used }，无上限
infra-compute-os = 系统
infra-compute-kernel = 内核
infra-compute-arch = 架构
infra-compute-cpu = 处理器
infra-compute-cpus = 核心数
infra-compute-load = 负载（1、5、15 分钟）
infra-compute-rss = 本进程
infra-compute-threads = 线程
infra-compute-files = 打开的文件
infra-compute-uptime = 服务已运行
infra-compute-sys-uptime = 机器已运行
infra-compute-sandbox = 沙箱
infra-compute-nnp-on = 禁止获取新权限
infra-compute-nnp-off = 可以获取权限
infra-compute-seccomp-on = 系统调用已过滤
infra-compute-seccomp-off = 系统调用未过滤
infra-activity = 活动
infra-activity-intro = 统计 { $span }；服务重启后从零开始。
infra-activity-sessions = 打开的会话
infra-activity-signed-in = 登录
infra-activity-failed = 登录失败
infra-activity-registered = 创建的账户
infra-activity-cross-site = 被拒绝的跨站提交

# Sign-in services: what the reader is told when one fails (v0.5.0, docs/adr/0012)
sso-error-not-configured = 本站未设置此登录服务。
sso-error-no-public-url = 通过服务登录需要本站的公开地址，但尚未设置。由管理员在“连接”中设置。
sso-error-unreachable = 无法连接登录服务。请稍后重试，或换一种方式登录。
sso-error-refused = 登录服务未允许你登录，或登录在服务那边被取消了。
sso-error-stale = 该登录已过期，或是在另一个浏览器中发起的。请重新开始。
sso-error-invalid = 登录服务的应答未通过本站的检查，因此未被采用。请重新开始；如果反复出现，请告知管理员。
infra-upgrade-already-pending = { $version } 已在等待升级助手。请等它完成后再请求其他版本。

# Administration console: Connectivity, sign-in services and registration (v0.5.0, docs/adr/0012)
connectivity-title = 连接
connectivity-lede = 读者如何访问本站并登录：公开地址、登录服务与规则。
connectivity-kpi-address = 公开地址
connectivity-address-unset = 未设置
connectivity-address-https = HTTPS
connectivity-address-plain = 未加密
connectivity-address-unset-note = 每个登录服务都需要它
connectivity-kpi-services = 登录服务
connectivity-kpi-services-note = 可提供
connectivity-kpi-passwords = 密码
connectivity-policy-short-everyone = 所有人
connectivity-policy-short-non-admins = 管理员除外
connectivity-policy-short-nobody = 无人
connectivity-policy-note-everyone = 每个账户都可以用密码登录，也可以通过已关联的服务登录。
connectivity-policy-note-non-admins = 管理员只能通过服务登录；其他人仍可使用密码。这样，被盗的管理员密码就打不开任何东西。
connectivity-policy-note-nobody = 所有人都通过服务登录；未关联服务的账户无法登录。
connectivity-kpi-admins-linked = 已关联服务的管理员 { $n } 名
connectivity-kpi-registration = 注册
connectivity-kpi-registration-go = 在“账户”中设置 →
registration-short-closed = 仅限管理员
registration-short-secret = 凭家族密语
registration-short-open = 对访客开放
connectivity-address = 公开地址
connectivity-address-intro = 读者输入的公开地址，例如 https://family.example；登录服务会把读者送回这里。
connectivity-address-label = 公开地址
connectivity-address-hint = 在没有人通过服务登录之前可以留空。
connectivity-address-invalid = 这不是带主机名的 http 或 https 地址。
connectivity-address-saved = 公开地址已保存。
connectivity-policy = 登录规则
connectivity-policy-intro = 规则适用于所有账户；会导致没有管理员能登录的规则会被拒绝。
connectivity-policy-passwords = 谁可以用密码登录
connectivity-policy-without-way = 有 { $n } 个活动账户既没有关联服务也没有地址：选择“无人”后它们将无法登录。
connectivity-link-by-email = 将服务关联到具有相同已验证地址的账户
connectivity-link-by-email-note = 用亲属的地址创建其账户：其首次通过验证该地址的服务登录时，即会关联。
connectivity-domains = 允许开户的域名
connectivity-domains-hint = 每行一个（family.example）。留空：不限。否则只有这些域名下经过验证的地址才能开设账户。
connectivity-domain-invalid = “{ $domain }”不是域名。
connectivity-policy-unknown = 请选择谁可以用密码登录。
connectivity-policy-lockout = 已拒绝：这项更改会使没有任何活动管理员能登录。请先通过某个登录服务登录一次，将其关联到管理员账户，或为管理员保留密码。
connectivity-policy-saved = 登录规则已保存。
connectivity-not-saved = 未保存：{ $error }
connectivity-services = 登录服务
connectivity-services-intro = 在服务的控制台中用所示的重定向地址注册本站，然后在此粘贴客户端 ID 和密钥。
connectivity-outbound-note = 通过服务登录需要允许出站连接：运行 sudo systemctl edit axgf-cms，在 [Service] 下加入 IPAddressAllow=any，然后重启。
connectivity-service-ready = 就绪
connectivity-service-incomplete = 不完整
connectivity-service-off = 已关闭
connectivity-service-enable = 在登录和注册页面提供此服务
connectivity-service-issuer = 签发者
connectivity-service-issuer-hint = 提供方的地址，例如 https://auth.family.example/realms/family；其设置从该地址下的 /.well-known/openid-configuration 读取。
connectivity-service-label = 按钮文字
connectivity-service-tenant = 目录（租户）
connectivity-service-tenant-hint = common 表示任意 Microsoft 账户，organizations 表示工作和学校账户，consumers 表示个人账户，或填写你的目录 ID。
connectivity-service-client-id = 客户端 ID
connectivity-service-secret = 客户端密钥
connectivity-service-secret-set = 已设置密钥。留空即保留。
connectivity-service-secret-unset = 尚未设置密钥。
connectivity-service-secret-clear = 删除密钥
connectivity-service-redirect = 需登记的重定向地址
connectivity-service-redirect-needs-address = 请先设置公开地址
connectivity-service-console = 在哪里注册
connectivity-service-console-open = { $name } 控制台
connectivity-service-test = 测试连接
connectivity-issuer-invalid = 签发者必须是 https:// 地址。
connectivity-tenant-invalid = 目录是名称或 ID：字母、数字、点和连字符。
connectivity-provider-incomplete = { $name } 不完整：需要客户端 ID 和客户端密钥，通用提供方还需要签发者。
connectivity-provider-saved = { $name } 已保存。
connectivity-test-ok = { $name } 已应答：{ $answer }。
connectivity-test-failed = 无法从本服务器连接 { $name }：{ $detail }
registration-title = 注册
registration-intro = 访客能否自行开设成员账户；管理员可在下方为其指定其他角色。
registration-who = 谁可以开户
registration-note-closed = 只有管理员能在此页面创建账户。
registration-note-secret = 知道家族口令的人可以开设成员账户。请像分享门禁密码一样分享它。
registration-note-open = 任何访客都可以开设成员账户。成员可以阅读仅对成员开放的记录。
registration-rotate = 生成新的密语
registration-secret = 家族密语
registration-secret-note = 在此模式开启期间，任何知道密语的人都能开户。生成新密语后旧密语失效；已开设的账户保留。
registration-link = 注册页面
registration-quick = 快捷入口
registration-quick-none = 没有就绪的登录服务：请在“连接”中设置一个
registration-changed = 最后由 { $by } 修改，{ $at }。
registration-unknown = 请选择谁可以开户。
accounts-kpi-total = 活动账户
accounts-kpi-total-note = 已停用 { $disabled } 个
accounts-kpi-admins = 管理员
accounts-kpi-admins-few = 再添加一名
accounts-kpi-admins-ok = 不止一名，理应如此
accounts-kpi-linked = 已关联服务
accounts-kpi-linked-note = { $n } 个服务就绪
accounts-kpi-registered = 访客开设 { $n } 个
accounts-roles = 角色
accounts-sign-in = 登录方式
accounts-sign-in-password = 密码
accounts-sign-in-google = Google
accounts-sign-in-microsoft = Microsoft
accounts-sign-in-github = GitHub
accounts-sign-in-oidc = 单点登录
accounts-sign-in-none = 暂无登录方式
accounts-created-via-register = 由访客开设
accounts-created-via-sso = 通过服务开设
accounts-add-also-register = 按上方设置，访客也可以自行开设成员账户。在此创建的账户可以是任何角色。
login-password-not-allowed = 本站的登录规则不允许此账户使用密码：请通过服务登录。
login-register = 还没有账户？创建一个
register-title = 创建账户
register-lede = 成员账户可阅读为家族成员保留的记录。最快的方式是使用你已有的账户。
register-closed = 这里的账户由管理员创建。请向管理员索取你的账户。
register-have-account = 已有账户？登录
register-or = 或
register-secret = 家族密语
register-secret-hint = 由管理员或家族中的某人提供给你。
register-secret-hint-services = 请在选择服务之前输入，否则下一步会要求你输入。
register-domains = 只有 { $domains } 的地址才能开户。
register-username-hint = 2 至 64 个字母、数字、点、连字符或下划线。
register-submit = 创建账户
register-member-note = 不要求填写地址：不会给你发送任何东西，也没人能冒用你的地址。
register-form-closed = 本站不通过此表单开户。
register-throttled = 此处尝试次数过多。请等一刻钟后再试。
register-secret-wrong = 这不是家族密语。
register-username-invalid = 用户名由 2 至 64 个字母、数字、点、连字符或下划线组成。
register-password-short = 密码至少需要 { $min } 个字符。
register-username-taken = 该用户名已被占用。请另选一个。
register-not-saved = 无法保存账户。请稍后再试。
register-finish-title = 还差一步
register-finish-lede = 服务已确认你的身份。要在这里开户，请输入家族密语。
register-finish-lede-email = 服务已确认你是 { $email }。要在这里开户，请输入家族密语。
sso-continue = 继续
sso-continue-with = 使用 { $name } 继续
sso-error-title = 登录未完成
sso-account-disabled = 与此登录关联的账户已停用。请联系管理员。
sso-no-account = 这里没有与此登录匹配的账户，本站也不会自行开户。请让管理员用这个地址为你创建账户：你下次登录时就会关联到它。
sso-domain-refused = 这里只能用经服务确认、属于家族域名的地址开户。

# Administration console: the overview's tiles (v0.5.0, docs/adr/0011)
overview-accounts-note = 管理员 { $admins } 名
register-password-hint = 至少 { $min } 个字符。由几个词组成的短语比一堆杂乱的符号更容易记住。

# Clarity: the light theme and its funnel (v0.5.0, docs/adr/0013)
theme-clarity = 清晰
theme-clarity-note = 浅色，白与橙 · 漏斗
view-funnel-back = 返回漏斗
view-funnel-open = 打开漏斗
view-funnel-hero-1 = 每一代，都是一个环。
view-funnel-hero-2 = 每个家族，都是一只漏斗。
view-funnel-cta = 探索漏斗
view-funnel-cta-detail = 每添加一个名字、每核实一个来源，都会让漏斗对后来的人更加真实。
view-funnel-see = 在漏斗中查看
view-funnel-direction = ↓ 最早的一代在窄端 · ↑ 最新的一代在开口
view-funnel-still = 让漏斗静止
view-funnel-name = 漏斗
view-funnel-legend = 如何阅读漏斗
view-funnel-search-hint = 按 Enter 将其移到前面；按 Shift+Enter 打开其记录。
theme-genealogy = 家谱
theme-genealogy-note = 深绿与奶油色 · 一棵可攀登的树
view-scroll-back = 返回家谱树
view-scroll-open = 打开家谱树
view-scroll-hero-1 = 每一个生命，都是一幅肖像。
view-scroll-hero-2 = 每一个家庭，都是一棵可以攀登的树。
view-scroll-cta = 探索家谱树
view-scroll-cta-detail = 每添加一个名字、每核实一处来源，家谱树对后来者都更加真实。
view-scroll-see = 在家谱树中查看
view-scroll-direction = ↓ 向着树根：祖先 · ↑ 向着树冠：后代
view-scroll-still = 让树叶静止
view-scroll-name = 家谱树
view-scroll-legend = 如何阅读家谱树
view-scroll-search-hint = 回车键将此人置于树干；上档键+回车键打开其记录。
bundle-binding-unnamed = 一个未命名的归档
bundle-binding-created = 创建于 { $date } 的归档
bundle-binding-named = “{ $family }”
bundle-binding-named-created = 创建于 { $date } 的“{ $family }”

# Notes and contributions (docs/adr/0014)
notes-section-title = 笔记与投稿
notes-section-intro = 家族成员写下的内容：回忆、研究、书信、文件。可以搜索，按首字母浏览，或打开一篇阅读并保存为 PDF。
notes-search-label = 搜索笔记
notes-search-go = 搜索
notes-all = 全部
notes-letters-label = 按标题首字母查看笔记
notes-shown = 显示：{ $shown } / { $total }
notes-none = 还没有人写过笔记。
notes-none-found = 没有符合此搜索的笔记。
notes-write = 写笔记
notes-family-wide = 整个家族
notes-by = 作者：{ $author }
notes-about = 涉及：{ $names }
notes-about-prefix = 涉及：
notes-about-family = 涉及整个家族
notes-pdf = 下载 PDF
notes-edit = 编辑
notes-close = 关闭
notes-back = 返回笔记
notes-add-about = 为此人写笔记
notes-sign-in = 家族的笔记与投稿仅供成员阅读。请登录后阅读。
notes-admin-title = 笔记与投稿
notes-admin-intro = 写下记录容纳不了的内容：一段回忆、一项研究、一封书信，可附图片。关联到人物的笔记会显示在其记录中；未关联任何人的笔记属于整个家族。每位成员都可以在“家族”页面阅读全部笔记。
notes-deleted = 笔记已删除。
notes-col-title = 标题
notes-col-category = 类别
notes-col-date = 日期
notes-col-about = 涉及
notes-col-author = 作者
notes-col-actions = 操作
notes-new-title = 新笔记
notes-edit-title = 编辑笔记
notes-section-about = 笔记
notes-field-title = 标题
notes-field-category = 类别
notes-field-category-hint = 可自由填写；会提示已用过的类别。
notes-field-date = 日期
notes-field-date-hint = 1952、1952-06 或 1952-06-14；没有则留空。
notes-field-people = 涉及的人物
notes-field-people-hint = 输入姓名并选择。未指明任何人时，笔记属于整个家族。
notes-person-add = 添加
notes-person-remove = 移除此人
notes-field-text = 正文
notes-text-hint = 使用 Markdown 或上方按钮。可粘贴或拖入图片，每张最多 { $mb } MB。
notes-tool-bold = 粗体
notes-tool-italic = 斜体
notes-tool-heading = 小标题
notes-tool-list = 列表
notes-tool-quote = 引文
notes-tool-link = 链接
notes-tool-image = 添加图片
notes-tool-preview = 预览
notes-tool-write = 返回编辑
notes-uploading = 正在添加图片…
notes-save = 保存笔记
notes-cancel = 取消
notes-delete = 删除此笔记
notes-delete-confirm = 笔记及其图片将从档案中移除。
notes-category-suggestions = 回忆, 研究, 书信, 故事, 食谱, 文件
notes-error-title = 笔记需要标题。未保存任何内容。
notes-error-title-long = 标题过长：最多 200 个字符。未保存任何内容。
notes-error-text-long = 正文过长。未保存任何内容。
notes-error-date = 无法理解此日期。请写年、月或日：1952、1952-06 或 1952-06-14。未保存任何内容。
notes-error-person = 涉及的人物：“{ $name }”。
notes-error-image-gone = 一张粘贴的图片已不再等待保存；图片只等待六小时。请重新添加。未保存任何内容。
notes-error-too-many-images = 一篇笔记最多容纳 40 张图片。未保存任何内容。
notes-error-stale = 在您编辑期间，有人修改了这篇笔记。请重新打开查看对方的版本，然后再做修改。未保存任何内容。
notes-error-refused = 档案拒绝了此修改。未保存任何内容。
notes-error-image = 此文件不是本站可以显示的图片：PNG、JPEG、GIF 或 WebP。
notes-error-image-large = 此图片过大。
notes-stale-title = 笔记未修改
notes-missing-title = 找不到笔记
notes-missing = 这里没有笔记，或者它不对您开放。
notes-not-yours = 投稿者只能修改自己写的笔记。管理员可以修改这一篇。
notes-pdf-failed-title = 无法生成 PDF
notes-pdf-failed = 生成这篇笔记的 PDF 时出错了。请先在页面上阅读，并告知管理员。
notes-dashboard-count = 已写笔记：{ $n }
notes-dashboard-open = 撰写和管理笔记

# The family the archive describes: the bundle room's form (2026-10-23)
kind-bundle = 档案
manifest-title = 家族
manifest-intro = 归档关于其家族的说明。其名称显示在“家族”页、页面标题和笔记 PDF 中。
manifest-name = 家族名称
manifest-name-hint = 显示在“家族”页面和标题中。账户会随新名称更新。
manifest-description = 描述
manifest-culture = 主要文化
manifest-culture-hint = 语言标签：fr、pl、pt-BR…
manifest-place = 主要地点
manifest-place-hint = 家族主要生活的大洲。
manifest-span = 涵盖时期
manifest-span-hint = 从最早到最晚的出生；保存时计算。
manifest-license = 许可
manifest-license-type = 许可
manifest-save = 保存家族信息
manifest-saved = 家族信息已保存。账户会随其名称更新。
manifest-error-long = 某个字段过长：名称最多 120 个字符，描述最多 2000 个。未保存任何内容。
manifest-error-culture = 文化是语言标签，例如 fr、pl 或 pt-BR。未保存任何内容。
manifest-error-stale = 在您编辑期间，有人修改了家族信息。您的版本在下方：再次保存将替换对方的版本。未保存任何内容。
manifest-error-io = 无法写入档案；服务器日志说明了原因。未保存任何内容。

# The family form: places, licences and the period (2026-10-23)
manifest-none = 未指定
manifest-span-value = { $earliest } – { $latest }
manifest-span-none = 尚未记录任何出生日期。
manifest-license-pick = 选择一种许可，即可看到它允许他人如何使用档案。
manifest-license-author = 设定者
manifest-license-author-value = { $who }，{ $at }
manifest-license-author-none = 选择许可时记录。
manifest-error-place = 请从列表中选择主要地点。未保存任何内容。
manifest-error-licence = 请从列表中选择许可。未保存任何内容。
continent-africa = 非洲
continent-antarctica = 南极洲
continent-asia = 亚洲
continent-europe = 欧洲
continent-north-america = 北美洲
continent-south-america = 南美洲
continent-oceania = 大洋洲
continent-several = 多个大洲
licence-all-rights-reserved-name = 保留所有权利
licence-all-rights-reserved-about = 未经家族许可，任何人不得复制、分享或再利用档案。未作说明时即适用此项。
licence-family-private-name = 私有，仅限家族
licence-family-private-about = 仅供家族成员使用：不公开，也不传给家族以外的人。
licence-cc0-name = CC0 1.0 — 公有领域
licence-cc0-about = 任何人都可以出于任何目的（包括商业目的）复制、修改和使用档案，无需许可或署名。
licence-cc-by-name = CC BY 4.0 — 署名
licence-cc-by-about = 任何人都可以分享和改编档案，包括用于商业目的，只要署名作者并注明所作修改。
licence-cc-by-sa-name = CC BY-SA 4.0 — 署名-相同方式共享
licence-cc-by-sa-about = 任何人都可以分享和改编，包括商业用途，但须署名作者；由此产生的作品必须以相同许可分享。
licence-cc-by-nc-name = CC BY-NC 4.0 — 署名-非商业性使用
licence-cc-by-nc-about = 任何人都可以在署名作者的前提下分享和改编，但不得用于商业目的。
licence-cc-by-nc-sa-name = CC BY-NC-SA 4.0 — 署名-非商业性使用-相同方式共享
licence-cc-by-nc-sa-about = 任何人都可以在署名作者的前提下分享和改编，限非商业用途，且只能以相同许可分享。
licence-cc-by-nd-name = CC BY-ND 4.0 — 署名-禁止演绎
licence-cc-by-nd-about = 任何人都可以原样分享，包括商业用途，但须署名作者，且不得传播修改后的版本。
licence-cc-by-nc-nd-name = CC BY-NC-ND 4.0 — 署名-非商业性使用-禁止演绎
licence-cc-by-nc-nd-about = 任何人都可以在署名作者的前提下原样分享，但不得用于商业目的，也不得修改：限制最严格的知识共享许可。
licence-odbl-name = ODbL 1.0 — 开放数据库许可
licence-odbl-about = 专为数据库设计：任何人都可以在注明来源的前提下使用、分享和改编数据，由此建立的数据库必须按相同条件保持开放。

# An account's own page and its record in the tree (2026-10-23)
nav-account = 我的账户
account-title = 我的账户
account-who = 账户
account-email = 电子邮件地址
account-since = 注册于
account-person-title = 我在族谱中的记录
account-person-intro = 您是族谱中的哪一位？由您自己选择，不做核实；可随时更改。
account-person-is = 此账户自称是
account-person-unlink = 取消关联
account-person-pick = 族谱中的人物
account-person-link = 将我的账户关联到此记录
account-person-change = 改为关联到此记录
account-person-linked = 您的账户已关联到其记录。
account-person-unlinked = 您的账户已不再关联任何记录。
account-person-error = 无法关联此记录：
account-person-io = 无法保存账户；服务器日志说明了原因。未作任何更改。
account-emergency-page = 紧急令牌不是账户：它没有自己的页面，也没有族谱中的记录。
accounts-person = 记录：

## The merge screen (src/routes/merge.rs): two records a person says are one.
merge-action = 合并
merge-needs-two = 请恰好勾选两条记录来合并。
merge-title = 合并两条记录
merge-lede = 确认之前不会有任何改动。第一条记录保留；第二条并入其中，并按原样保存在变更日志中。
merge-keep = 保留
merge-absorb = 并入其中
merge-swap = 改为保留另一条
merge-refused = 此合并被拒绝：
merge-refusal-ancestor = 其中一方是另一方的祖先
merge-refusal-spouse-and-child = 在同一家庭中，一方是配偶，另一方是子女
merge-refusal-spouses = 二者是同一结合的两位配偶
merge-refusal-siblings = 二者被记录为同一对父母的子女
merge-refusal-different-couples = 两个家庭是不同的夫妇
merge-refusal-child-is-spouse = 一个家庭的子女是另一个家庭的配偶
merge-refusal-same-record = 这是同一条记录被选了两次
merge-conflicts = 不同之处
merge-nothing = 无需选择：二者并不矛盾。
merge-choice-keep = 保留的记录
merge-choice-absorb = 另一条记录
merge-choice-both = 两者都要
merge-because-higher-confidence = 建议：可信度更高的值。
merge-because-sourced = 建议：引用了来源的值。
merge-because-kept-record = 建议：保留记录的值。
merge-because-more-restrictive = 建议：更严格的可见范围。
merge-because-protective = 建议：在世——更具保护性的理解。
merge-because-both-kept = 建议：两段文字都保留。
merge-becomes-alias = 未选的名字会作为别名保留。
merge-becomes-event = 未选的值会作为一个事件保留。
merge-agreements = 一致之处
merge-from-keep = 来自保留的记录
merge-from-absorb = 来自另一条记录
merge-from-both = 两边相同
merge-from-joined = 列表已合并
merge-from-combined = 已合并为一
merge-effects = 其他变化
merge-effect-references = 改写的引用
merge-effect-self-links = 移除的自我链接
merge-effect-listed-twice = 会把它列出两次的列表
merge-effect-same-couple = 同一对夫妇的家庭
merge-confirm-label = 他们仍可能是同一个人。如仍要合并，请准确输入姓名：{ $name }
merge-go = 合并
merge-missing = 两条记录中有一条已不存在。
merge-done-title = 已合并
merge-done-lede = { $absorbed } 现已并入 { $kept }。
merge-report-references = 改写的引用
merge-report-events = 由未选值生成的事件
merge-report-source-conflicts = 记在来源上的分歧
merge-report-links = 移除的链接
merge-report-tidied = 整理的条目
merge-absorbed-record = 被合并的记录（原样）
merge-open-kept = 打开保留的记录

## 亲属关系 (docs/adr/0020)

kin-tab = 亲属关系
kin-parent = { $adoptive ->
        [yes] { $sex ->
            [M] 养父
            [F] 养母
           *[other] 养父母
        }
       *[no] { $sex ->
            [M] 父亲
            [F] 母亲
           *[other] 父母
        }
    }
kin-ancestor = { $n ->
        [2] { $side ->
            [F] 外
           *[other] {""}
        }{ $sex ->
            [M] 祖父
            [F] 祖母
           *[other] 祖父母
        }
        [3] { $side ->
            [F] 外
           *[other] {""}
        }{ $sex ->
            [M] 曾祖父
            [F] 曾祖母
           *[other] 曾祖父母
        }
        [4] { $side ->
            [F] 外
           *[other] {""}
        }{ $sex ->
            [M] 高祖父
            [F] 高祖母
           *[other] 高祖父母
        }
       *[other] 上{ $n }代祖先
    }
kin-child = { $adoptive ->
        [yes] { $sex ->
            [M] 养子
            [F] 养女
           *[other] 养子女
        }
       *[no] { $sex ->
            [M] 儿子
            [F] 女儿
           *[other] 子女
        }
    }
kin-descendant = { $n ->
        [2] { $side ->
            [F] 外
           *[other] {""}
        }{ $sex ->
            [M] 孙子
            [F] 孙女
           *[other] 孙辈
        }
        [3] { $side ->
            [F] 外
           *[other] {""}
        }{ $sex ->
            [M] 曾孙
            [F] 曾孙女
           *[other] 曾孙辈
        }
        [4] { $side ->
            [F] 外
           *[other] {""}
        }{ $sex ->
            [M] 玄孙
            [F] 玄孙女
           *[other] 玄孙辈
        }
       *[other] 下{ $n }代后代
    }
kin-sibling = { $half ->
        [yes] { $side ->
            [M] 同父异母的
            [F] 同母异父的
           *[other] 同父母一方的
        }{ $sex ->
            [M] { $elder ->
                [yes] 哥哥
                [no] 弟弟
               *[other] 兄弟
            }
            [F] { $elder ->
                [yes] 姐姐
                [no] 妹妹
               *[other] 姐妹
            }
           *[other] 兄弟姐妹
        }
       *[no] { $adoptive ->
            [yes] 领养的{ $sex ->
                [M] { $elder ->
                    [yes] 哥哥
                    [no] 弟弟
                   *[other] 兄弟
                }
                [F] { $elder ->
                    [yes] 姐姐
                    [no] 妹妹
                   *[other] 姐妹
                }
               *[other] 兄弟姐妹
            }
           *[no] { $sex ->
                [M] { $elder ->
                    [yes] 哥哥
                    [no] 弟弟
                   *[other] 兄弟
                }
                [F] { $elder ->
                    [yes] 姐姐
                    [no] 妹妹
                   *[other] 姐妹
                }
               *[other] 兄弟姐妹
            }
        }
    }
kin-uncle = { $n ->
        [1] { $side ->
            [M] { $sex ->
                [M] { $turnelder ->
                    [yes] 伯父
                    [no] 叔父
                   *[other] 伯父或叔父
                }
                [F] 姑母
               *[other] 伯叔或姑母
            }
            [F] { $sex ->
                [M] 舅父
                [F] { $turnelder ->
                    [yes] 大姨
                    [no] 小姨
                   *[other] 姨母
                }
               *[other] 舅父或姨母
            }
           *[other] { $sex ->
                [M] 伯叔或舅父
                [F] 姑母或姨母
               *[other] 父母的兄弟姐妹
            }
        }
        [2] { $side ->
            [F] 外
           *[other] {""}
        }{ $turna ->
            [M] 祖父
            [F] 祖母
           *[other] 祖父母
        }的{ $sex ->
            [M] 兄弟
            [F] 姐妹
           *[other] 兄弟姐妹
        }
        [3] 曾祖辈的兄弟姐妹
       *[other] 祖先的兄弟姐妹
    }
kin-nephew = { $n ->
        [1] { $via ->
            [M] { $sex ->
                [M] 侄子
                [F] 侄女
               *[other] 侄子女
            }
            [F] { $sex ->
                [M] 外甥
                [F] 外甥女
               *[other] 外甥子女
            }
           *[other] { $sex ->
                [M] 侄子或外甥
                [F] 侄女或外甥女
               *[other] 兄弟姐妹的子女
            }
        }
        [2] { $via ->
            [M] { $sex ->
                [M] 侄孙
                [F] 侄孙女
               *[other] 侄孙辈
            }
            [F] { $sex ->
                [M] 外甥孙
                [F] 外甥孙女
               *[other] 外甥孙辈
            }
           *[other] 兄弟姐妹的孙辈
        }
        [3] 兄弟姐妹的曾孙辈
       *[other] 兄弟姐妹的后代
    }
kin-cousin = { $k ->
        [1] { $line ->
            [yes] 堂
           *[no] 表
        }{ $sex ->
            [M] { $elder ->
                [yes] 兄
                [no] 弟
               *[other] 兄弟
            }
            [F] { $elder ->
                [yes] 姐
                [no] 妹
               *[other] 姐妹
            }
           *[other] 亲
        }
       *[other] 远房{ $line ->
            [yes] 堂
           *[no] 表
        }{ $sex ->
            [M] { $elder ->
                [yes] 兄
                [no] 弟
               *[other] 兄弟
            }
            [F] { $elder ->
                [yes] 姐
                [no] 妹
               *[other] 姐妹
            }
           *[other] 亲
        }
    }
kin-cousin-removed = { $older ->
        [yes] { $forebear }的{ $line ->
            [yes] 堂亲
           *[no] 表亲
        }
       *[no] { $line ->
            [yes] 堂亲
           *[no] 表亲
        }的{ $descendant }
    }
kin-of-forebear = { $n ->
        [1] { $sex ->
            [M] 父亲
            [F] 母亲
           *[other] 父母
        }
        [2] { $sex ->
            [M] 祖父
            [F] 祖母
           *[other] 祖父母
        }
        [3] { $sex ->
            [M] 曾祖父
            [F] 曾祖母
           *[other] 曾祖父母
        }
       *[other] 祖先
    }
kin-descendant-word = { $n ->
        [1] { $sex ->
            [M] 儿子
            [F] 女儿
           *[other] 子女
        }
        [2] { $sex ->
            [M] 孙子
            [F] 孙女
           *[other] 孙辈
        }
       *[other] 后代
    }
kin-of-cousin = 表亲
kin-partner = { $ended ->
        [yes] { $married ->
            [yes] { $sex ->
                [M] 前夫
                [F] 前妻
               *[other] 前配偶
            }
           *[no] 前伴侣
        }
       *[no] { $married ->
            [yes] { $sex ->
                [M] 丈夫
                [F] 妻子
               *[other] 配偶
            }
           *[no] 伴侣
        }
    }
kin-parent-in-law = { $via ->
        [F] { $sex ->
            [M] 岳父
            [F] 岳母
           *[other] 岳父母
        }
        [M] { $sex ->
            [M] 公公
            [F] 婆婆
           *[other] 公婆
        }
       *[other] { $sex ->
            [M] 配偶的父亲
            [F] 配偶的母亲
           *[other] 配偶的父母
        }
    }
kin-child-in-law = { $sex ->
        [M] 女婿
        [F] 儿媳
       *[other] 子女的配偶
    }
kin-sibling-in-law = { $kind ->
        [partner-sibling] { $via ->
            [F] { $sex ->
                [M] 妻子的兄弟
                [F] 妻子的姐妹
               *[other] 妻子的兄弟姐妹
            }
            [M] { $sex ->
                [M] 丈夫的兄弟
                [F] 丈夫的姐妹
               *[other] 丈夫的兄弟姐妹
            }
           *[other] 配偶的兄弟姐妹
        }
       *[other] { $sex ->
            [M] 姐妹的丈夫
            [F] 兄弟的妻子
           *[other] 兄弟姐妹的配偶
        }
    }
kin-step-parent = { $sex ->
        [M] 继父
        [F] 继母
       *[other] 继父母
    }
kin-step-child = { $sex ->
        [M] 继子
        [F] 继女
       *[other] 继子女
    }
kin-note-step-parent = { $next ->
        [M] 父亲
        [F] 母亲
       *[other] 父母一方
    }的配偶
kin-note-step-child = { $via ->
        [M] 丈夫
        [F] 妻子
       *[other] 配偶
    }的子女
kin-foster-parent = { $sex ->
        [M] 寄养父亲
        [F] 寄养母亲
       *[other] 寄养父母
    }
kin-foster-child = { $sex ->
        [M] 寄养儿子
        [F] 寄养女儿
       *[other] 寄养子女
    }
kin-guardian = 监护人
kin-ward = 被监护人
kin-relative-of-partner = { $via ->
        [M] 丈夫
        [F] 妻子
       *[other] 配偶
    }的{ $relative }
kin-partner-of-relative = { $relative }的{ $sex ->
        [M] 丈夫
        [F] 妻子
       *[other] 配偶
    }
kin-affine = 姻亲
kin-related = 经家族记录相连
kin-note-adoptive = 经收养
kin-of = 相对于{ $name }
kin-hop-parent = { $line ->
        [adoption] { $sex ->
            [M] 养父
            [F] 养母
           *[other] 养父母
        }
        [foster] { $sex ->
            [M] 寄养父亲
            [F] 寄养母亲
           *[other] 寄养父母
        }
        [guardianship] 监护人
        [step] { $sex ->
            [M] 继父
            [F] 继母
           *[other] 继父母
        }
       *[other] { $sex ->
            [M] 父亲
            [F] 母亲
           *[other] 父母
        }
    }
kin-hop-child = { $line ->
        [adoption] { $sex ->
            [M] 养子
            [F] 养女
           *[other] 养子女
        }
        [foster] { $sex ->
            [M] 寄养儿子
            [F] 寄养女儿
           *[other] 寄养子女
        }
        [guardianship] 被监护人
        [step] { $sex ->
            [M] 继子
            [F] 继女
           *[other] 继子女
        }
       *[other] { $sex ->
            [M] 儿子
            [F] 女儿
           *[other] 子女
        }
    }
kin-group-partners = 配偶
kin-group-ancestors = 祖先
kin-group-descendants = 后代
kin-group-siblings = 兄弟姐妹
kin-group-uncles = 叔伯姑舅姨
kin-group-nephews = 侄甥
kin-group-cousins = 堂表亲
kin-group-in-laws = 姻亲
kin-group-step = 重组家庭
kin-group-care = 寄养与监护
kin-group-rites = 仪式与习俗
kin-group-others = 其他关联
kin-find-title = 是什么亲属关系？
kin-find-lede = 选择一个人：逐步说明其与{ $name }的关系。
kin-find-label = 人物
kin-find-choose = 选择一人…
kin-find-go = 查看关系
kin-find-none = 家族记录中，{ $a }与{ $b }之间没有关联。
kin-path = 关系，一步一步
kin-degree = { $n }亲等
kin-all-title = { $name }的全部亲属
kin-all-count = { $n } 人
kin-common-list = 共同祖先：{ $names }
kin-also-list = 另为 { $list }
kin-rites-help = 教父母及传统所建立的其他关系；族谱树以虚线绘出。
kin-rite-recorded = 记录写的是“{ $label }”。
kin-hidden = 您无权查看的人
kin-native = 传统中的原称
kin-legend-rite = 教父母、仪式或习俗
kin-tree-root = { $of }：{ $term }
kin-editor-rite = 仪式或习俗
kin-editor-rite-none = 无：普通关系
kin-editor-rite-hint = 此人对另一人而言是什么。选择后会填入关系与称谓，记录会保留该仪式。
kin-editor-rite-option = { $rite }：{ $word }
kin-tradition-christian = 基督教
kin-tradition-orthodox = 东正教
kin-tradition-hispanic = 西班牙语世界与菲律宾
kin-tradition-jewish = 犹太教
kin-tradition-islamic = 伊斯兰教
kin-tradition-south-asian = 南亚
kin-tradition-east-asian = 东亚
kin-tradition-african = 非洲
kin-tradition-indigenous = 原住民与太平洋
kin-tradition-civil = 民事与世俗
kin-rite-godparent = { $role ->
        [sponsored] { $sex ->
            [M] 教子
            [F] 教女
           *[other] 教子女
        }
       *[other] { $sex ->
            [M] 教父
            [F] 教母
           *[other] 教父母
        }
    }
kin-rite-godparent-name = 教父母
kin-rite-godparent-note = 记录所载的教父或教母。
kin-rite-baptism = { $role ->
        [sponsored] { $sex ->
            [M] 教子
            [F] 教女
           *[other] 教子女
        }
       *[other] { $sex ->
            [M] 教父
            [F] 教母
           *[other] 教父母
        }
    }
kin-rite-baptism-name = 洗礼
kin-rite-baptism-note = 洗礼时的教父母。
kin-rite-confirmation = { $role ->
        [sponsored] { $sex ->
            [M] 领坚振者
            [F] 领坚振者
           *[other] 领坚振者
        }
       *[other] { $sex ->
            [M] 坚振代父
            [F] 坚振代母
           *[other] 坚振代父母
        }
    }
kin-rite-confirmation-name = 坚振
kin-rite-confirmation-note = 坚振礼或傅油礼时的代父母。
kin-rite-wedding-sponsor = { $role ->
        [sponsored] { $sex ->
            [M] 新郎
            [F] 新娘
           *[other] 新人
        }
       *[other] { $sex ->
            [M] 婚礼代父
            [F] 婚礼代母
           *[other] 婚礼代父母
        }
    }
kin-rite-wedding-sponsor-name = 婚礼代父母
kin-rite-wedding-sponsor-note = 婚姻的担保人，如西语国家的婚礼教父、菲律宾的尼农、希腊的昆巴罗斯。
kin-rite-kum = { $role ->
        [sponsored] { $sex ->
            [M] 教子
            [F] 教女
           *[other] 教子女
        }
       *[other] { $sex ->
            [M] 教父
            [F] 教母
           *[other] 教父母
        }
    }
kin-rite-kum-name = 库姆教亲
kin-rite-kum-note = 东正教斯拉夫传统中的教父母或证婚人；库姆教亲连结两个家族。
kin-rite-koumbaros = { $role ->
        [sponsored] { $sex ->
            [M] 教子
            [F] 教女
           *[other] 教子女
        }
       *[other] { $sex ->
            [M] 教父
            [F] 教母
           *[other] 教父母
        }
    }
kin-rite-koumbaros-name = 昆巴罗斯
kin-rite-koumbaros-note = 希腊东正教传统中的证婚人或教父母。
kin-rite-padrino = { $role ->
        [sponsored] { $sex ->
            [M] 教子
            [F] 教女
           *[other] 教子女
        }
       *[other] { $sex ->
            [M] 教父
            [F] 教母
           *[other] 教父母
        }
    }
kin-rite-padrino-name = 帕德里诺
kin-rite-padrino-note = 帕德里诺或马德里娜：洗礼、初领圣体、十五岁成人礼时。
kin-rite-compadre = 教亲
kin-rite-compadre-name = 教亲关系
kin-rite-compadre-note = 孩子的父母与其教父母之间的纽带。
kin-rite-ninong = { $role ->
        [sponsored] { $sex ->
            [M] 教子
            [F] 教女
           *[other] 教子女
        }
       *[other] { $sex ->
            [M] 教父
            [F] 教母
           *[other] 教父母
        }
    }
kin-rite-ninong-name = 尼农
kin-rite-ninong-note = 尼农或尼南：菲律宾洗礼、坚振或婚礼中的教父母。
kin-rite-sandek = { $role ->
        [sponsored] { $sex ->
            [M] 教子
            [F] 教女
           *[other] 教子女
        }
       *[other] 桑德克
    }
kin-rite-sandek-name = 割礼：桑德克
kin-rite-sandek-note = 割礼时抱着婴儿。
kin-rite-kvatter = { $role ->
        [sponsored] { $sex ->
            [M] 教子
            [F] 教女
           *[other] 教子女
        }
       *[other] 克瓦特
    }
kin-rite-kvatter-name = 割礼：克瓦特
kin-rite-kvatter-note = 把婴儿抱进割礼。
kin-rite-milk-parent = { $role ->
        [sponsored] { $sex ->
            [M] 乳子
            [F] 乳女
           *[other] 乳儿
        }
       *[other] { $sex ->
            [M] 乳父
            [F] 乳母
           *[other] 乳父母
        }
    }
kin-rite-milk-parent-name = 乳亲
kin-rite-milk-parent-note = 曾哺乳该儿童：在伊斯兰法中，乳亲关系如同出生一样构成家庭并禁止通婚。
kin-rite-milk-sibling = { $sex ->
        [M] 乳兄弟
        [F] 乳姐妹
       *[other] 乳亲兄弟姐妹
    }
kin-rite-milk-sibling-name = 乳亲兄弟姐妹
kin-rite-milk-sibling-note = 由同一位女性哺乳。
kin-rite-kafala = { $role ->
        [sponsored] 被担保儿童（卡法拉）
       *[other] 担保监护人（卡法拉）
    }
kin-rite-kafala-name = 卡法拉
kin-rite-kafala-note = 依伊斯兰法收留并抚育儿童，但不建立收养的亲子关系。
kin-rite-rakhi = { $sex ->
        [M] 拉基兄弟
        [F] 拉基姐妹
       *[other] 拉基兄弟姐妹
    }
kin-rite-rakhi-name = 拉基
kin-rite-rakhi-note = 在护身符节，姐妹在兄弟腕上系上拉基，兄弟发誓保护她。
kin-rite-guru = { $role ->
        [sponsored] 弟子
       *[other] 上师
    }
kin-rite-guru-name = 上师
kin-rite-guru-note = 精神导师与弟子。
kin-rite-gan-parent = { $role ->
        [sponsored] { $sex ->
            [M] 干儿子
            [F] 干女儿
           *[other] 干儿女
        }
       *[other] { $sex ->
            [M] 干爹
            [F] 干妈
           *[other] 干亲
        }
    }
kin-rite-gan-parent-name = 干亲
kin-rite-gan-parent-note = 在中国及华人家庭中认的干亲，常为保佑孩子。
kin-rite-sworn-sibling = { $sex ->
        [M] 结拜兄弟
        [F] 结拜姐妹
       *[other] 结拜兄弟姐妹
    }
kin-rite-sworn-sibling-name = 结拜
kin-rite-sworn-sibling-note = 以誓言结为兄弟姐妹（结拜）。
kin-rite-name-giver = { $role ->
        [sponsored] 被命名的孩子
       *[other] 命名人
    }
kin-rite-name-giver-name = 命名
kin-rite-name-giver-note = 为孩子取名（日本的名付け親）。
kin-rite-nakodo = { $role ->
        [sponsored] { $sex ->
            [M] 新郎
            [F] 新娘
           *[other] 新人
        }
       *[other] 媒人
    }
kin-rite-nakodo-name = 仲人
kin-rite-nakodo-note = 促成并见证了婚姻（日本的仲人）。
kin-rite-namesake = 同名人
kin-rite-namesake-name = 同名
kin-rite-namesake-note = 同名之人：在中部与东部非洲是互赠礼物、彼此照应的纽带（恩多伊、索莫）。
kin-rite-initiation = { $role ->
        [sponsored] 受礼者
       *[other] 成年礼引导人
    }
kin-rite-initiation-name = 成年礼
kin-rite-initiation-note = 引导此人完成成年礼。
kin-rite-blood-pact = { $sex ->
        [M] 歃血兄弟
        [F] 歃血姐妹
       *[other] 歃血兄弟姐妹
    }
kin-rite-blood-pact-name = 歃血为盟
kin-rite-blood-pact-note = 以血盟结为一体。
kin-rite-hanai = { $role ->
        [sponsored] { $sex ->
            [M] 哈奈儿子
            [F] 哈奈女儿
           *[other] 哈奈子女
        }
       *[other] { $sex ->
            [M] 哈奈父亲
            [F] 哈奈母亲
           *[other] 哈奈父母
        }
    }
kin-rite-hanai-name = 哈奈
kin-rite-hanai-note = 依夏威夷习俗被视如己出地抚养，未经法律收养。
kin-rite-hunka = { $role ->
        [sponsored] { $sex ->
            [M] 洪卡儿子
            [F] 洪卡女儿
           *[other] 洪卡子女
        }
       *[other] { $sex ->
            [M] 洪卡父亲
            [F] 洪卡母亲
           *[other] 洪卡父母
        }
    }
kin-rite-hunka-name = 洪卡
kin-rite-hunka-note = 在拉科塔人的洪卡仪式中被认作亲人。
kin-rite-civil-sponsor = { $role ->
        [sponsored] { $sex ->
            [M] 民事教子
            [F] 民事教女
           *[other] 民事教子女
        }
       *[other] { $sex ->
            [M] 民事教父
            [F] 民事教母
           *[other] 民事教父母
        }
    }
kin-rite-civil-sponsor-name = 民事教父母
kin-rite-civil-sponsor-note = 民事命名仪式中的教父母。
kin-rite-guardian = { $role ->
        [sponsored] 被监护人
       *[other] 监护人
    }
kin-rite-guardian-name = 监护
kin-rite-guardian-note = 记录所载的监护人。

## Polygamous unions (docs/adr/0021): one marriage per spouse, the
## co-spouses, the editor's two fields.
kin-co-spouse = { $sex ->
        [M] 同妻之夫
        [F] 同夫之妻
       *[other] 共同配偶者
    }
union-co-spouses = { $sex ->
        [F] 丈夫的其他妻子
        [M] 妻子的其他丈夫
       *[other] 配偶的其他配偶
    }
tree-key-polygamous = 多偶婚姻
family-primary = 婚姻的中心人物
family-primary-hint = 在多偶婚姻中，与其他每个人都结婚的人。其他人彼此之间没有婚姻关系。
family-child-parent = 另一位家长
family-child-parent-hint = 在多偶婚姻中，作为这个孩子另一位家长的配偶。未选择时，记录没有说明。

## Emblems (docs/adr/0022): the picture that stands for a family, and for
## the archive's.
emblem-family-edit = 徽章或纹章
emblem-family-title = 家族徽章
emblem-archive-title = 档案家族的徽章
emblem-archive-edit = 家族的徽章或纹章
emblem-saved = 徽章已保存。
emblem-choose = 代表家族的图片
emblem-help = 纹章、标记或照片：代表这个家族的东西。它会完整显示在每份记录的婚姻旁和家谱中夫妻的结上。
emblem-archive-help = 纹章、标记或照片：代表本档案所讲述家族的东西。它会完整显示在每个页面顶部和概览中，向所有访客展示。
emblem-mode-auto-note = 这个家族作为徽章关联的图片（如果有）。
emblem-mode-none = 无徽章
emblem-no-images = 还没有图片：请在下方添加一张。
emblem-upload-title = 添加图片
emblem-upload-button = 添加并使用
emblem-not-an-image = 徽章是一张图片：PNG、JPEG、GIF、WebP 或 BMP。
emblem-back = 返回徽章
emblem-no-family-title = 找不到这个家族
emblem-no-family-detail = 这个家族不在档案中，或者您无权查看。

## The Artificial family genealogist, AFG (docs/adr/0023): its room, where
## its computing comes from, and a test.
admin-nav-afg = AI 家谱师
afg-title = 人工智能家谱师
afg-lede = AI 家谱师的计算来自哪里，以及测试。
afg-kpi-source = 计算来源
afg-ready = 就绪
afg-off-note = 请选择计算来源。
afg-not-ready = 不完整
afg-kpi-sent = 发送的文件
afg-kpi-sent-note = { $kb } KB，约 { $tokens } 个词元
afg-kpi-last = 上次测试
afg-never = 尚无
afg-run-open = 打开
afg-source = 计算来源
afg-source-intro = 一个地址；带密钥和模型的地址；或已在本机登录的代理。
afg-kind = 计算方式
afg-kind-hint = 服务通过网络访问；代理在本机运行，且必须已在本机登录。
afg-kind-off = 无
afg-kind-anthropic = Claude（Anthropic 接口）
afg-kind-ollama = Ollama
afg-kind-openai = OpenAI
afg-kind-mistral = Mistral
afg-kind-vllm = vLLM
afg-kind-gemini = Gemini
afg-kind-claude-code = Claude Code
afg-kind-codex = Codex
afg-kind-opencode = OpenCode
afg-group-address = 一个地址
afg-group-service = 地址、API 密钥和模型
afg-group-agent = 本机上的编程智能体
afg-url = 地址
afg-url-hint = 留空则使用该服务的常用地址。调用：
afg-key = API 密钥
afg-key-set = 已保存一个密钥，结尾为 { $end }。
afg-key-unset = 未保存密钥。
afg-key-hint = 留空则保留已存的密钥，且不再显示。
afg-key-clear = 删除密钥
afg-model = 模型
afg-model-hint = 服务所写的模型名称。任何名称都原样发送；留空则用默认值。
afg-model-unlisted = 不在推荐列表中：将按原样发送。
afg-effort = 思考强度
afg-effort-hint = 模型思考的深度。任何词都原样发送；留空则用模型默认值。
afg-effort-unlisted = 不是此模型的候选取值：将按原样发送。
afg-effort-default = 默认
afg-program = 程序
afg-program-found = 已找到：
afg-program-missing = 本机上未找到。
afg-program-hint = 留空：在其安装程序放置的位置查找。否则填写程序的完整路径。
afg-sensitive = 同时发送敏感类别
afg-sensitive-hint = 不勾选时，健康、生物特征、基因、司法记录和行为不会发送，与共享导出一样。
afg-timeout = 允许时间（秒）
afg-timeout-hint = 放弃之前等待回答的时间。
afg-test = 测试
afg-test-intro = 将归档（JSON，不含照片）和您的记忆连同一个问题发送；回答会在窗口中打开。
afg-sent-files = 文件
afg-sent-files-value = { $n } 个 JSON 文件，{ $kb } KB
afg-sent-tokens = 词元（约）
afg-sent-records = 记录
afg-sent-records-value = { $persons } 人，{ $families } 个家庭
afg-sent-classes = 敏感类别
afg-sent-classes-in = 发送
afg-sent-classes-out = 不发送
afg-sent-question = 问题
afg-privacy = 测试会把家族记录（包括在世者）发送到上面选择的计算服务。
afg-test-button = 发送测试
afg-test-needs = 请先保存完整的计算方式。
afg-test-question = 请列出登记人数最多的前三个家族。
afg-test-empty = 归档为空
afg-saved = 已保存。
afg-error-not-saved = 未保存：{ $error }
afg-error-off = 尚未选择计算方式。
afg-error-archive = 无法读取归档：{ $error }
afg-error-url = 地址必须以 http:// 或 https:// 开头。
afg-error-key = 此服务需要 API 密钥。
afg-error-model = 此处需要模型名称，只能使用字母、数字和 . _ : / @ [ ] + -
afg-error-effort = 思考强度只能使用字母、数字和 . _ : / @ [ ] + -
afg-error-program = 程序必须是该智能体自己的程序（claude、codex 或 opencode），并填写完整路径。
afg-run-state-running = 等待回答
afg-run-state-done = 已回答
afg-run-state-failed = 失败
afg-run-seconds = { $s } 秒
afg-run-usage = 读取的词元
afg-run-usage-out = 输出 { $n }
afg-run-cost = { $usd } 美元
afg-run-waiting = 归档已发送，回答将显示在这里。本页每三秒重新查询一次。
afg-run-model = 模型：
afg-run-failed-hint = 请检查计算方式和本站可访问的网络；如果是智能体，还要确认它已安装，并已在运行本站的账户下登录。
afg-run-gone-detail = 测试在进程运行期间保留，仅保留最近十次。请重新发送。

## The AFG's brains, its window and who may use it (docs/adr/0024).
afg-kpi-brain = 您的记忆
afg-brain-exchanges = { $n } 次对话
afg-brain-note = 已记住 { $facts } 条事实 · { $brains } 个账户有记忆
afg-brain-no-account = 此会话没有账户，因此没有记忆。
afg-access-count = 可与 AI 家谱师对话的账户：{ $n }
afg-access-set = 在账户页面设置
afg-result-ok = 成功
afg-result-error = 错误
afg-result-close = 关闭
afg-result-remembered = 已存入 { $name } 的记忆：
afg-result-nothing-kept = 没有新的内容存入 { $name } 的记忆。
accounts-afg = AI 家谱师
accounts-afg-allowed = 可以与 AI 家谱师对话
accounts-afg-yes = 是
accounts-afg-no = 否
accounts-afg-hint = 未勾选则关闭。其记忆单独保存在归档旁边。

## An account's AFG brain on its own page: seen and forgotten (docs/adr/0024 §6).
account-brain-title = 您的 AI 家谱师的记忆
account-brain-intro = AI 家谱师记住的关于您的内容，每次提问时都会提供给它。您可随意删除；其他人看不到。
account-brain-no-access = 您的账户不能使用 AI 家谱师：由管理员在账户页面决定。
account-brain-none = AI 家谱师还没有记住关于您的任何内容。
account-brain-facts = 它记住的内容
account-brain-facts-none = 它没有保留关于您的任何事实。
account-brain-exchanges = 您与它的对话
account-brain-exchanges-none = 没有保留任何对话。
account-brain-older = 另有 { $n } 次较早的对话已保留，未在此列出。
account-brain-context-test = 在 AI 家谱师页面中的测试
account-brain-answer = 它的回答
account-brain-forget = 忘记
account-brain-forget-all = 全部忘记…
account-brain-forget-all-confirm = 是的，忘记 AI 家谱师记住的关于我的一切
account-brain-forget-all-button = 全部忘记
account-brain-forgotten = 已忘记：AI 家谱师将不再得知此内容。
account-brain-forgotten-all = AI 家谱师不再记得关于您的任何内容。
account-brain-already = 这已经被忘记了。
account-brain-confirm-needed = 请勾选此框以全部忘记。
account-brain-error = 没有忘记任何内容：{ $error }
afg-brain-see = 查看并忘记

## My personal assistant: projects, conversations, code of conduct, second brain (docs/adr/0026).
assistant-title = 我的私人助理
assistant-entry = 您与 AI 家谱师的项目和对话。
assistant-lede = 您与 AI 家谱师的项目，各有行为准则、记忆和对话。
assistant-shut-no-account = 此会话没有账户，因此没有私人助理。请用你自己的账户登录。
assistant-shut-not-open = AI 家谱师未对你的账户开放：由管理员在账户页面决定。
assistant-shut-broken = 无法打开你的空间。请稍后再试；如果一直如此，请告知管理员。
assistant-shut-gone = 这里什么也没有：该项目或对话不存在，或不属于你。
assistant-back-account = 返回我的账户
assistant-new-project = 新建项目
assistant-not-ready = AI 家谱师的计算尚未设置：目前只能阅读。
assistant-conversations-count = { $n } 个对话
assistant-conduct-set = 已设行为准则
assistant-empty = 还没有项目。一个项目汇集同一主题的对话。
assistant-project-name = 项目名称
assistant-create = 创建项目
assistant-facts-count = 已记住 { $n } 条事实
assistant-archive-on = 附带家族归档
assistant-start = 开始对话
assistant-conversations = 对话
assistant-no-conversation = 还没有对话：你的第一个问题会开启一个。
assistant-conduct = 行为准则
assistant-conduct-label = AI 家谱师在此项目中应如何表现
assistant-conduct-hint = 你对此项目所有对话的要求：语气、语言、关注什么、略过什么。只要不是要求它欺骗你或造成伤害，AI 家谱师都会遵守。
assistant-save = 保存
assistant-memory = 项目记忆
assistant-memory-intro = AI 家谱师从此项目对话中选择保留的内容，也就是它的第二大脑。项目中每条消息都会再交给它。让它忘记你不再希望它知道的内容。
assistant-forget = 忘记
assistant-forget-all = 全部忘记…
assistant-forget-all-confirm = 是的，忘记 AI 家谱师关于此项目记住的一切
assistant-forget-all-button = 全部忘记
assistant-memory-none = AI 家谱师还没有记住关于此项目的任何内容。
assistant-settings = 设置
assistant-archive = 在此项目中把家族归档交给 AI 家谱师
assistant-archive-hint = 这样，归档中的每条记录，包括在世者，都会随每条消息发送到 AI 家谱师的计算之处。除非管理员另有决定，敏感类别不会包含在内。
assistant-archive-size = JSON 文件：{ $files } 个；每条消息的词元：约 { $tokens }。
assistant-avatar-upload = 用你自己的图片作头像
assistant-avatar-send = 使用这张图片
assistant-delete-project = 删除项目
assistant-delete-project-confirm = 是的，删除此项目、其中的对话、对话中的文件及项目记忆
assistant-delete = 删除
assistant-conversation-settings = 对话设置
assistant-conversation-title = 对话标题
assistant-delete-conversation = 删除对话
assistant-delete-conversation-confirm = 是的，删除此对话及其消息
assistant-thread = 对话
assistant-reply = 你的消息
assistant-sending = 正在发送…
assistant-attach = 附件
assistant-send = 发送
assistant-composer-hint = 按回车键发送，按上档键加回车键换行。文件、照片、音频或视频可拖放、粘贴或附加，最多 { $n } 个文件，每个不超过 { $mb } MB。
assistant-avatar-choose = 选择头像
assistant-avatar = 头像
assistant-avatar-auto = 自动选择
assistant-avatar-own = 你的图片
assistant-kept = 已记入项目记忆：{ $n } 条事实
assistant-error-answer = AI 家谱师未能回答。
assistant-retry = 再问一次
assistant-unanswered = 这条消息没有得到回答。
assistant-writing = AI 家谱师正在书写
assistant-saved = 已保存。
assistant-project-deleted = 项目已删除。
assistant-conversation-deleted = 对话已删除。
assistant-memory-forgotten = 已忘记：AI 家谱师不会再收到它。
assistant-memory-forgotten-all = AI 家谱师已不再记得关于此项目的任何内容。
assistant-avatar-saved = 头像已保存。
assistant-error-name = 需要一个名称。
assistant-error-too-many = 已达到一个账户可保留的数量上限。
assistant-error-not-saved = 未保存：服务器无法写入。请稍后再试。
assistant-error-conduct-long = 行为准则太长。
assistant-error-confirm = 请勾选以确认。
assistant-error-too-big = 消息太大。
assistant-error-file-big = 文件太大。
assistant-error-too-many-files = 一条消息中的文件太多。
assistant-error-text-long = 消息太长。
assistant-error-empty = 请写一条消息或附加一个文件。
assistant-error-busy = AI 家谱师正在回答它所能承担的全部对话。请稍后再试。
assistant-error-wait = AI 家谱师仍在此对话中回答。
assistant-error-picture = 该文件不是本站能读取的图片。
assistant-domain-genealogy = 家谱
assistant-domain-medicine = 健康与医学
assistant-domain-work = 工作与职业
assistant-domain-money = 钱财
assistant-domain-law = 法律与证件
assistant-domain-learning = 学习
assistant-domain-home = 家居
assistant-domain-cooking = 烹饪
assistant-domain-travel = 旅行
assistant-domain-nature = 自然与园艺
assistant-domain-wellbeing = 身心健康
assistant-domain-family = 家庭生活
assistant-domain-arts = 艺术与文化
assistant-domain-sports = 运动
assistant-domain-tech = 科技
assistant-domain-science = 科学
assistant-domain-history = 历史与记忆
assistant-domain-admin = 办事手续
assistant-domain-pets = 动物
assistant-domain-rites = 节庆与礼仪

## The AFG on a person's record: a face or a photograph restored, a document read, the summary (docs/adr/0029).

summary-title = 简介
summary-write = 撰写简介
summary-edit = 编辑简介
summary-language = 语言
summary-by-ai = 由 AI 撰写（{ $generator }）：请核对。
summary-note-title = 简介 — { $name }
summary-category = 简介
summary-edit-title = 此人的简介
afg-summary = 用 AI 撰写
afg-summary-busy = 撰写中…
afg-retouch = AI 修复
afg-retouch-busy = 修复中…
afg-retouched = 由 AI 修复：并非当年的照片。
afg-read = 读取文档
afg-read-busy = 读取中…
afg-read-note-title = 转录 — { $name }
afg-read-category = 转录
afg-rework = 用 AI 重新处理图像
afg-rework-busy = 处理中…
afg-images-off-admin = 尚无图像服务：请在 管理 › AI 家谱师 › 图像 中选择。
notes-manage-in-media = 在“媒体”标签页中编辑或删除
notes-delete-short = 删除
notes-none-yet = 暂无关于此人的笔记。
afg-record-title = 记录中的 AI
afg-record-back = 返回记录
afg-record-no-access = 您的账户未开通 AI 家谱师。管理员可在“账户”页面开通。
afg-record-images-off = 尚未设置图像服务。管理员可在 管理 › AI 家谱师 › 图像 中选择。
afg-record-text-off = 尚未设置 AI 家谱师的计算来源。管理员可在 管理 › AI 家谱师 中选择。
afg-record-no-face = 此人没有可处理的照片。
afg-record-no-document = 此文档不在此人的文件中，其文件未保存在此处，或者它已是 AI 制作的副本。
afg-record-not-a-picture = 只能修复图像。
afg-record-not-readable = 只能读取图像或 PDF。
afg-record-pdf-unread = AI 家谱师当前的计算来源无法读取 PDF。OpenAI、Claude、Gemini、Claude Code 和 OpenCode 可以。
afg-record-failed = AI 未能完成：{ $reason }
afg-record-bad-picture = 服务未返回本站可以保存的图像。
afg-record-no-text = AI 在此文档中未找到任何文字。
afg-record-stale = 记录在此期间已更改。请重新加载页面后重试。
afg-record-refused = 档案拒绝了此操作：{ $reason }
afg-record-summary-not-yours = 此简介由另一位成员撰写：只有其作者或管理员可以替换它。
afg-images = 图像
afg-images-intro = AI 家谱师的图片从哪里来：付费服务（OpenAI、Google、Canva）或家中带显卡的电脑。
afg-image-kind = 图像服务
afg-image-kind-hint = OpenAI 和 Google 的模型、通过您账户使用的 Canva AI，或家族自己的服务器：Ollama、任何兼容 OpenAI 的服务器、ComfyUI。
afg-image-kind-off = 无
afg-image-kind-openai = OpenAI
afg-image-kind-gemini = Google Gemini
afg-image-url-hint = 留空：使用服务的常用地址。您自己的服务器：填写它的地址（例如 http://192.168.1.20:8000/v1）。
afg-image-key-hint = 更换服务会清除密钥：请同时填写新服务的密钥。
afg-image-model-hint = 留空：使用第一个建议的模型；其他模型按所填内容发送。
afg-images-privacy = 每当成员提出请求，照片都会发送到此服务。请选择您的家族愿意托付图像的服务。
afg-images-saved = 图像服务已保存。
afg-images-ready = 图像已就绪：记录中的按钮可以请求图像。
afg-images-not-ready = 没有图像服务：记录中的图像按钮显示为灰色。
afg-images-choose = 选择图像服务

## Canva and the family's own servers as the AFG's picture service (docs/adr/0030).

afg-image-kind-canva = Canva（它的 AI，通过您的账户）
afg-image-kind-ollama = Ollama（实验性）
afg-image-kind-openai-compatible = 您自己的服务器（兼容 OpenAI）
afg-image-kind-comfyui = ComfyUI（您的工作流）
afg-image-workflow = ComfyUI 工作流（API 格式）
afg-image-workflow-hint = 从 ComfyUI 以 API 格式导出。照片会放入每个 LoadImage 节点，文本中的这个标记会变成修复指令：
afg-image-key-optional-hint = 可选：仅当您的服务器需要时填写。
afg-images-at-home = 照片留在您的网络中：此服务运行在家族自己的机器上。
afg-images-saved-canva = 已选择 Canva：现在请连接一个账户。
afg-canva-connect = 连接 Canva
afg-canva-connected = 已连接 Canva 账户 { $account }。
afg-canva-connected-unnamed = 已连接一个 Canva 账户。
afg-canva-not-connected = 尚未连接 Canva 账户。
afg-canva-disconnect = 断开 Canva
afg-canva-disconnect-confirm = 断开这个 Canva 账户？已经生成的图像仍保留在档案中。
afg-canva-step-open = 1. 打开 Canva 并同意
afg-canva-step-paste = 2. 在此粘贴 Canva 将您送回的页面地址（http://localhost…）
afg-canva-paste-hint = 该页面无法显示：这是正常的。它的地址就够了，且只能用一次。
afg-canva-finish = 完成连接
afg-canva-finish-wait = 连接最多需要一分钟：它还会在 Canva 中准备好用来修复每张照片的页面。
afg-canva-done-ready = Canva 已连接，其工作页面已在文件夹“ax-genealogy”中准备好。之后 Canva 可能会将 AI 额度暂停几分钟：如果第一次修复被拒绝，请稍后再试。
afg-canva-cooldown = Canva 已将此账户的 AI 额度暂停几分钟。请稍后再试。
afg-canva-done = Canva 已连接。
afg-canva-gone = Canva 已断开。
afg-canva-save-first = 请先选择 Canva 作为图像服务，然后保存。
afg-canva-policy = 一个连接服务整个家族：修复会使用这个 Canva 账户、它的额度和它的文件夹“ax-genealogy”。Canva 的使用政策要求每个人连接自己的账户；为 AI 家谱师连接您的账户，即由您承担这一责任。
afg-error-workflow = 缺少 ComfyUI 工作流，或它不是 API 格式（至少需要一个 LoadImage 节点）。
afg-error-canva = Canva 尚未连接：请使用“连接 Canva”。

## Impact: the Universal Synoptic Timeline (docs/adr/0034)

nav-impact = 影响
impact-title = 影响
impact-lede = 家族成员的一生，放在世界历史中。
impact-span-all = 全部历史
impact-span-family = 家族的几个世纪
impact-placed = 已放置的人：{ $n }
impact-undated = 无日期：{ $n }
impact-hidden = 未显示：{ $n }
impact-empty = 尚无已知日期：还无法放置任何人。
impact-col-world = 世界
impact-col-continents = 各大洲
impact-col-family = 家族
impact-col-life = 一生
impact-col-country = 国家
impact-continent-europe = 欧洲
impact-continent-africa = 非洲
impact-continent-asia = 亚洲
impact-continent-americas = 美洲
impact-continent-oceania = 大洋洲
impact-kind-era = 文明与时期
impact-kind-war = 战争
impact-kind-epidemic = 疫病
impact-kind-migration = 迁徙
impact-kind-event = 里程碑
impact-kind-economy = 经济与社会
impact-kind-disaster = 饥荒与灾难
impact-kind-invention = 重大发明
impact-kind-discovery = 重大发现
impact-person-title = { $name }与其时代
impact-back = 整个家族
impact-record = 打开档案
impact-states = 其居住地所属的国家
impact-countries-none = 在其地点中未找到国家。
impact-unplaced = 没有已知日期：无法在时间轴上放置此人。
impact-see = 查看影响
impact-year-bc = 公元前{ $year }年
impact-now = 至今
impact-years = { $n }年
impact-months = { $n }个月
impact-days = { $n }天
impact-age = { $n }岁时
impact-under-way = 出生时已在进行
impact-milestone-birth = 出生
impact-milestone-death = 去世
impact-milestone-marriage = 结婚
impact-milestone-child = { $name }出生
impact-context = 背景
impact-stakes = 关键所在
impact-victims = 遇难者
impact-close = 关闭
impact-hint = 将指针移到事件上可查看其年份；点击可查看背景。

# A person created with what surrounds them (docs/adr/0039), and the
# pickers' refusals by what they pick.
person-new-title = 新人物
person-new-lede = 档案中尚无的名字会成为一条新记录，与本记录一起保存。
person-new-section-name = 姓名
person-new-given = 名
person-new-family = 姓
person-new-nicknames = 别号
person-new-nickname-add = 添加别号
person-new-nickname-quoted = “{ $nick }”
person-new-remove = 移除
person-new-preview = 显示为
person-new-name-hint = 至少填写名或姓。
person-new-section-life = 出生与去世
person-new-section-texts = 生平与笔记
person-new-birth-place = 出生地
person-new-death-place = 去世地
person-new-death-hint = 在世期间留空。
person-new-place-hint = 档案中没有的地点会被创建。
person-new-section-portrait = 肖像
person-new-portrait-hint = PNG、JPEG、GIF 或 WebP，最大 { $mb } MB。
person-new-bio-hint = 可自由排版，与笔记相同。
person-new-section-note = 笔记
person-new-note-hint = 关于此人的单独笔记，可含图片。不填标题时以此人的姓名为题。
person-new-section-parents = 父母与兄弟姐妹
person-new-father = 父亲
person-new-mother = 母亲
person-new-siblings = 兄弟姐妹
person-new-sibling-add = 添加兄弟姐妹
person-new-who-hint = 从列表中选择或输入姓名：无人使用的姓名会成为新记录。
person-new-section-unions = 结合与子女
person-new-union-n = 结合 { $n }
person-new-partner = 配偶或伴侣
person-new-union-type = 结合类型
person-new-children = 子女
person-new-child-add = 添加子女
person-new-union-add = 添加结合
person-new-save = 创建记录
person-new-hint-existing = 档案中已有
person-new-hint-new = 新记录
person-new-hint-ambiguous = 有多条记录使用此姓名：请从列表中选择一条。
person-new-hint-place-existing = 档案中的地点
person-new-hint-place-new = 新地点
person-new-field-error = “{ $typed }”：{ $message }
person-new-error-name = 需要填写名或姓。未保存任何内容。
person-new-error-date = 日期写作 1890、1890-06 或 1890-06-14。未保存任何内容。
person-new-error-same-parents = 父亲和母亲是同一个人。未保存任何内容。
person-new-error-siblings-need-parent = 兄弟姐妹只能通过父母组成家庭：请指明一位父母，或一位已在某个家庭中的兄弟姐妹。未保存任何内容。
person-new-error-siblings-two-families = 这些兄弟姐妹属于不同的家庭：请指明父母。未保存任何内容。
person-new-error-portrait = 肖像不是本站能显示的图片：PNG、JPEG、GIF 或 WebP。未保存任何内容。
person-new-error-conflict = 在您编写期间档案已更改。未保存任何内容；请再次保存。
person-new-error-refused = 档案拒绝了这次保存。未保存任何内容。
person-new-created = { $others ->
        [0] 记录已创建。
       *[other] 记录已创建，另有 { $others } 条记录一同创建。
    }
pick-error-place-not-found = 本档案中没有此名称的地点。未保存任何内容。
pick-error-place-ambiguous = 有不止一个地点符合。请从列表中选择一个。未保存任何内容。
pick-error-source-not-found = 本档案中没有此名称的来源。未保存任何内容。
pick-error-source-ambiguous = 有不止一个来源符合。请从列表中选择一个。未保存任何内容。
pick-error-thing-empty = 未指明任何内容。未保存任何内容。
pick-error-thing-not-found = 本档案中没有此名称的内容。未保存任何内容。
pick-error-thing-ambiguous = 有不止一条记录符合。请从列表中选择一条。未保存任何内容。
search-create = 创建“{ $name }”

# What a file sent to the site may weigh, and the kinds refused
# (docs/adr/0041).
uploads-title = 文件上传
uploads-lede = 发送到本站的文件最大可以多大，以及档案接受哪些类型的文件。
uploads-sizes = 最大大小
uploads-limit-document = 附在记录上的文件（MB）
uploads-limit-note-image = 粘贴到笔记中的图片（MB）
uploads-limit-assistant-file = 发送给助手的文件（MB）
uploads-limit-assistant-message = 一条消息的全部文件（MB）
uploads-limit-gedcom = GEDCOM 文件（MB）
uploads-limit-archive = 整体导入的档案（MB）
uploads-limit-range = 从 { $min } 到 { $max } MB；默认 { $default }。
uploads-kinds = 文件类型
uploads-kinds-summary = 接受 { $accepted } 种，共 { $total } 种
uploads-family-image = 图片
uploads-family-document = 文档
uploads-family-audio = 音频
uploads-family-video = 视频
uploads-kinds-hint = 未勾选的类型在上传时会被拒绝；已在档案中的文件保留。
uploads-save = 保存
uploads-saved = 上传设置已保存。
uploads-error-range = { $field }：从 { $min } 到 { $max } 的整数。未保存任何内容。
uploads-not-saved = 无法保存设置：{ $error }
uploads-summary = 附在记录上的文件：最大 { $document } MB；接受 { $accepted } 种类型，共 { $total } 种。
uploads-open = 设置上传
error-upload-kind-refused = 本站拒绝此类文件（{ $kind }）：管理设置不接受它。未保存任何内容。

# An import in place of the family, behind a word and a verified backup
# (docs/adr/0042).
setup-replace-title = 替换家族
setup-replace-lede = 此档案已包含一个家族。导入文件会将其整个替换。
setup-replace-warning = 导入文件中没有的一切都将丢失：记录、笔记、照片、关系。
setup-replace-backup = 替换前，会先在 { $dir } 写入当前家族的已验证备份；可用 axgf-cms restore 恢复。
setup-replace-keyword = 替换档案
setup-replace-type = 请输入“{ $keyword }”以确认
setup-replace-go = 替换家族
setup-replace-keyword-wrong = 确认词不符：请输入“{ $keyword }”。未替换任何内容。
setup-replace-backup-failed = 当前家族的备份失败，因此未替换任何内容：{ $error }
setup-replace-changed = 家族在备份期间发生了变化，因此未替换任何内容。请重试。
setup-replace-done = 被替换的家族已备份，备份已验证：{ $file }
setup-replace-room-title = 用导入替换家族
setup-replace-room-detail = 在对当前家族做完经过验证的备份后，用档案或 GEDCOM 取而代之。
setup-replace-room-go = 打开导入

# AI 家谱师的指令与 MCP（docs/adr/0045）。
ops-line-create-person = 创建 { $name }
ops-line-create-relative = 同时创建 { $name }
ops-line-create-place = 创建地点 { $name }
ops-line-update-person = 修改 { $name }：{ $fields }
ops-line-set-birth = { $name } 的出生：{ $fact }
ops-line-set-death = { $name } 的去世：{ $fact }
ops-line-add-event = 添加{ $event }
ops-line-update-event = 修改{ $event }
ops-line-delete-event = 删除{ $event }
ops-line-add-union = 结合 { $partners }（{ $kind }）
ops-line-add-child = { $child }，{ $parents } 的子女
ops-line-remove-child = { $child } 不再是 { $parents } 的子女
ops-line-delete-person = 将 { $name } 移出所有家庭和事件，然后删除其档案
ops-line-delete-union = 解除 { $partners } 的结合：子女保留，但不再有这对父母
ops-field-name = 姓名
ops-field-gender = 性别
ops-field-living = 是否在世
ops-field-visibility = 可见范围
ops-field-bio = 生平
ops-refused-at = 操作 { $n }：{ $why }
ops-refused-empty = 没有要做的事。
ops-refused-no-person = 没有指明任何人。
ops-refused-not-found = 档案中没有与“{ $what }”对应的内容。
ops-refused-ambiguous = 有多份档案与“{ $what }”对应：请用其标识符指明。
ops-refused-date = “{ $value }”不是日期：请写作 1890、1890-06 或 1890-06-14。
ops-refused-gender = “{ $value }”不是性别：M、F、NB 或 U。
ops-refused-visibility = “{ $value }”不是可见范围：public、members、contributors 或 private。
ops-refused-category = “{ $value }”不是本站认识的事件类别。
ops-refused-role-name = “{ $value }”不是事件中的角色。
ops-refused-union-kind = “{ $value }”不是结合的类型。
ops-refused-partners = 一段结合有一位或两位伴侣。
ops-refused-parents = 请指明一位或两位父母，或指明家庭。
ops-refused-own-parent = 没有人是自己的父母。
ops-refused-already-child = { $child } 已经是这个家庭的子女。
ops-refused-not-a-child = { $child } 不是这个家庭的子女。
ops-refused-nothing = 不会有任何改变。
ops-refused-name = 一个人需要名或姓。
ops-refused-field = { $field }“{ $typed }”：{ $why }
ops-refused-role = 此账户可以阅读档案，但不能修改。
ops-refused-scope = 这涉及此账户可编辑分支之外的人。
ops-refused-admin = 只有管理员可以删除一个人。
ops-refused-dangling = { $field } 指向 { $target }，档案中任何地方都没有它。
ops-refused-invalid = 这里不能执行此操作：{ $detail }。
ops-refused-conflict = 档案在此期间已更改，没有写入任何内容。请重新检查后再应用。
ops-refused-library = 档案拒绝了此操作，没有写入任何内容：{ $detail }
ops-refused-io = 无法写入档案，没有写入任何内容：{ $detail }
journal-via-afg = AI 家谱师
afg-ops-title = 建议的修改
afg-ops-apply = 应用
afg-ops-aside = 搁置
afg-ops-note = 在您应用之前，什么都不会改变。以上内容要么全部一起应用，要么一项都不应用。
afg-ops-applied = { $n ->
    [0] 已应用：没有写入任何档案。
   *[other] 已应用：写入了 { $n } 份档案。
}
afg-ops-aside-done = 已搁置：没有修改任何内容。
afg-ops-json = AI 家谱师所写的操作
afg-ops-unreadable = AI 家谱师写的操作本站无法读取：{ $detail }
afg-ops-applied-text = { $n ->
    [0] 我应用了建议的修改：没有写入任何档案。
   *[other] 我应用了建议的修改：写入了 { $n } 份档案。
}
afg-ops-aside-text = 我搁置了建议的修改：没有修改任何内容。
afg-ops-preparing = AI 家谱师正在准备要建议的修改…
account-tokens-title = 供 AI 使用的访问令牌（MCP）
account-tokens-intro = 令牌让外部 AI（ChatGPT、Claude 或任何讲 MCP 的 AI）以您的账户身份阅读家族档案；如果令牌可以写入，还能修改档案。它所做的永远不会超出您的账户权限。
account-tokens-closed = 本站的 MCP 已关闭：在管理员打开之前，任何令牌都打不开任何东西。
account-tokens-open-it = 打开它
account-token-made = 已创建令牌“{ $label }”。
account-token-once = 请现在复制：它只显示这一次，只保存其指纹。持有它的人就以您的账户身份行事。
account-token-value = 令牌
account-token-address = MCP 的地址
account-token-address-with = 包含令牌的地址
account-token-address-with-hint = 适用于只接受地址的 AI，例如 ChatGPT 或 Claude 的连接器：请像保管令牌一样对它保密。
account-token-writes = 可读写
account-token-reads = 只读
account-token-made-on = 创建于 { $date }
account-token-used-on = 最近使用于 { $date }
account-token-never-used = 从未使用
account-token-revoke = 吊销
account-token-revoked = 令牌已吊销：它再也打不开任何东西。
account-tokens-none = 还没有令牌。
account-token-label = 令牌名称
account-token-label-example = ChatGPT、笔记本上的 Claude…
account-token-write = 可以像此账户一样修改档案
account-token-make = 创建令牌
account-tokens-too-many = 一个账户最多保留十个令牌：请吊销一个再创建新的。
account-tokens-how = 私人助理的指南说明了如何借助社区连接器 ax-genealogy 连接 ChatGPT、Claude 及其他 AI。
account-tokens-guide = 阅读指南
account-tokens-connector = 下载 Claude Desktop 连接器（.mcpb）
afg-mcp = 供外部 AI 使用的 MCP
afg-mcp-intro = MCP 让外部 AI（ChatGPT、Claude 或任何讲 Model Context Protocol 的 AI）凭某个账户在自己页面上创建的令牌，以该账户身份阅读和修改档案。每个令牌所能做的，就是其账户所能做的，仅此而已。
afg-mcp-state = 状态
afg-mcp-open = 已打开
afg-mcp-shut = 已关闭
afg-mcp-address = 地址
afg-mcp-address-relative = 接在本站公开地址之后，公开地址尚未设置：在控制台中设置。
afg-mcp-tokens = 活跃账户的令牌
afg-mcp-privacy = 账户能读到的一切都会交给读取它的 AI 所属公司，包括在世的人。只有在家人同意时才打开 MCP。
afg-mcp-switch = 向账户令牌打开 MCP
afg-mcp-close = 关闭 MCP
afg-mcp-opened = MCP 已打开：接受账户令牌。
afg-mcp-closed = MCP 已关闭：令牌保留，但打不开任何东西。
assistant-guide-title = 用 AI 家谱师管理家族谱系
assistant-guide-reader = 您的账户只阅读档案而不修改：AI 家谱师会回答您的问题，但不会提出修改。管理员可以给您的账户编辑者角色。
