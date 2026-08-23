# axgf-cms — インターフェイス文言、日本語。
#
# 機械品質 — 日本語を母語とする者による校閲を経ていません。とくに系譜の
# 専門語（union、affiliation、confidence）には各国の記録制度によって異なる
# 定訳があり、ここでの訳語は誤っている可能性があります。訂正を歓迎します —
# CONTRIBUTING.md を参照。
#
# 日本語に複数形はないため、下記の複数選択子は other 一つだけです。これは
# 抜けではなく、CLDR の日本語規則そのものです。英語の「一つ／それ以上」の
# 論理を当てはめる方が誤りになります。
#
# 規則：このファイルが訳すのはインターフェイスだけです。人名・地名・注記・
# 職業は .axgf ファイル由来で、常にその言語と文字のまま表示されます。

app-name = axgf-cms

nav-tree = 家系図
nav-convert = GEDCOM を変換
nav-admin = 管理
nav-sign-in = サインイン
nav-sign-out = サインアウト
footer-served-from = 単一の .axgf ファイルから配信されています。系譜のロジックはすべて axgf-rs にあり、形式は axgf-spec が定めています。

prefs-title = 言語と外観
prefs-language = 言語
prefs-language-note = これはインターフェイスのみを変更します。人名・地名・注記は常にその言語と文字のまま表示されます。
prefs-theme = 外観
prefs-apply = 適用
prefs-reviewed = 校閲済み
prefs-machine = 機械翻訳、{ $coverage }%

theme-light = ライト
theme-dark = ダーク
theme-system = システムに合わせる
theme-high-contrast = ハイコントラスト
theme-sepia = セピア
theme-deuteranopia = second型色覚
theme-protanopia = first型色覚
theme-tritanopia = third型色覚
theme-colour-blind-note = 色覚特性に配慮
theme-contrast-note = 最大コントラスト

tree-title-around = { $name } の周辺
tree-title-whole = 家系図全体
tree-lede-focused = 先祖 { $ancestors ->
       *[other] { $ancestors }
    } 名、子孫 { $descendants ->
       *[other] { $descendants }
    } 名、配偶者 { $spouses ->
       *[other] { $spouses }
    } 名、上下それぞれ { $depth } 世代。最年長者が下です。線の濃さは関係の確からしさを表し、薄い線は記録が確信していない主張です。
tree-lede-whole = ファイル内のすべての人物。最年長者が下、最年少者が上です。線の濃さは関係の確からしさを表します。
tree-filter-label = 表示中のカードを絞り込む
tree-filter-placeholder = 名前を入力…
tree-centre-on = 中心にする人物
tree-depth = 上下それぞれの世代数
tree-show = 表示
tree-whole-tree = 家系図全体
tree-focused = 絞り込み表示
tree-hidden-notice = { $n ->
       *[other] { $n } 名は詳細を伏せて表示されています
    }
tree-hidden-because-role = 。可視性がお使いのアカウントの閲覧範囲を超えているためです。
tree-hidden-because-anonymous = 。公開対象ではないためです。
tree-hidden-sign-in = アカウントをお持ちならサインインしてください。
tree-restricted-card = この人物の記録は閲覧できません
tree-empty = このファイルには描画できる人物がいません。
tree-unplaced = 記録された家族に属さない
tree-legend-confidence = 線の濃さは確からしさを表します
tree-recentre = ここを中心に描き直す
tree-open-record = 全記録を開く

record-identity = 身元
record-life-events = 生涯の出来事
record-family = 家族
record-other-relationships = その他の関係
record-occupations = 職業
record-places = 地名
record-sources-documents = 出典と文書
record-notes = 注記
record-history = 変更履歴
record-raw = 生データ
record-raw-summary-note = このページの元になった JSON
record-sources-documents-help = 各出典は、このページのうちそれに依拠する事実を挙げ、証拠の強さ順に並びます。
record-notes-help = この記録への注記。どの変換器も解釈できず、破棄せずそのまま残した文字列を含みます。
record-help-toggle = この節が示すもの

record-gender = 性別
record-living = 存命
record-visibility = 可視性
record-yes = はい
record-no = いいえ
record-name-type = 名前の種類
record-name-used = 使用時期
record-name-evidence = 証拠
record-name-primary = 主たる名前
record-transliteration = ラテン文字転写
record-born = 生誕
record-died = 死去
record-parents = 両親
record-siblings = きょうだい
record-children = 子
record-spouse = 配偶者
record-union-ended = 終了
record-no-date = 日付不明
record-unknown-person = [不明]
record-restricted-person = 非公開
record-restricted-title = この人物の記録は閲覧できません
record-absent-person-title = このファイルに言及はあるが収録されていない
record-confidence = 確からしさ
record-source = 出典
record-role = 役割
record-download = ダウンロード
record-attach-file = ファイルを添付
record-attach-hint = { $mb } MB まで。画像はギャラリーに表示され、それ以外はダウンロードリンク付きで一覧されます。
record-no-documents = この記録に添付されたファイルはありません。

access-restricted-title = 閲覧できません
access-restricted-anonymous = この記録は公開されていません。お使いのアカウントで閲覧できるかはサインインしてご確認ください。
access-role-title = お使いの役割では利用できません
access-role-write = お使いのアカウントはこのファイルを閲覧できますが変更はできません。管理者が役割を「寄稿者」に引き上げられます。
access-scope-title = 担当する枝の外です

error-not-found-title = 見つかりません
error-not-found-detail = このファイルにそのページはありません。
error-no-such-person-title = 該当する人物がいません
error-no-such-person-detail = このファイルにその識別子の人物はいません。
error-no-such-entity-title = 該当する記録がありません
error-no-such-entity-detail = このファイルにその識別子の記録はありません。
error-deleted-while-editing = このファイルにその識別子の記録はありません。編集中に削除された可能性があります。
error-no-such-file-title = 該当するファイルがありません
error-not-an-image-title = 画像ではありません
error-not-an-image-detail = この文書のサムネイルはありません。この版が解読できる画像ではないためです。
error-back = 戻る

login-title = サインイン
login-lede = アカウントは管理者が作成します。
login-username = ユーザー名
login-password = パスワード
login-submit = サインイン
login-wrong = ユーザー名とパスワードが一致しません。
login-token-wrong = そのトークンは正しくありません。
login-throttled = 失敗が多すぎます。数分待ってからやり直してください。
login-no-accounts-title = このインストールにはまだアカウントがありません。
login-emergency-summary = 緊急アクセス
login-emergency-label = 緊急トークン
login-emergency-submit = 緊急トークンを使う
login-sign-in-prompt = 管理画面に入るにはサインインしてください。

admin-title = 管理
admin-entities = 記録
admin-create = 作成
admin-new-kind = 新規：{ $kind }
admin-operations = 操作
admin-validate = 検証
admin-deduplicate = 重複を統合
admin-export = ファイルを書き出す
admin-accounts = アカウント
admin-dedup-confirm = 重複の統合は記録を併合しファイルを書き換えます。続けますか？
admin-recent-changes = 最近の変更
admin-sessions-open = { $n ->
       *[other] 現在 { $n } 件のセッションが開いています。
    }
admin-no-changes-yet = このアプリケーションからはまだ何も変更されていません。以後の保存はすべて { $path } に記録されます。
admin-last-validation = 前回の検証
admin-fields = 項目
admin-raw-json = 生の JSON
admin-save = 保存
admin-cancel = 取消
admin-delete = 削除
admin-not-set = — 未設定 —
admin-edit = 編集
admin-search = 検索
admin-page-of = { $pages } ページ中 { $page } ページ目
admin-previous = 前へ
admin-next = 次へ
admin-nothing-here = この種の記録はこのファイルにまだありません。
admin-saved = バージョン { $version } として保存しました — { $summary }
admin-not-saved = 保存されませんでした
admin-created = 作成しました
admin-not-created = 作成されませんでした
admin-deleted = 削除しました
admin-not-deleted = 削除されませんでした — ファイルは変更されていません
admin-delete-policy = 参照整合性
admin-what-changed = 変更内容
admin-field = 項目
admin-from = 変更前
admin-to = 変更後
admin-version = バージョン { $version }

accounts-title = アカウント
accounts-existing = 既存
accounts-username = ユーザー名
accounts-role = 役割
accounts-status = 状態
accounts-branch = 枝
accounts-last-seen = 最終サインイン
accounts-change = 変更
accounts-you = （あなた）
accounts-active = 有効
accounts-disabled = 無効
accounts-never = なし
accounts-whole-tree = 家系図全体
accounts-roots = { $n ->
       *[other] 起点 { $n } 名
    }
accounts-add = アカウントを追加
accounts-password-hint = 空欄にすると自動生成され、一度だけ表示されます。自分で決める場合は { $min } 文字以上。
accounts-new-password-placeholder = 新しいパスワード（空欄なら変更なし）
accounts-email = メールアドレス
accounts-optional = （任意）
accounts-create = アカウントを作成
accounts-role-viewer = 閲覧者 — 公開およびメンバー向けの記録を読む
accounts-role-contributor = 寄稿者 — 加えて作成・編集・アップロードができる
accounts-role-admin = 管理者 — 加えてアカウント管理・削除・書き出しができる
accounts-branch-placeholder = 1 行に 1 つの人物識別子
accounts-ids-in-bundle = このファイル内の人物識別子
accounts-created = { $username } を作成しました。
accounts-updated = { $username } を更新しました。開いていたセッションはすべてサインアウトされました。
accounts-username-taken = そのユーザー名は使われています。
accounts-pick-role = 役割を選んでください。
accounts-no-such = そのアカウントはありません。
accounts-not-saved = 保存されませんでした：{ $error }

conflict-title = 他の人が先に変更しました
conflict-versions = あなたはバージョン { $expected } から編集を始めました。ファイルは現在バージョン { $current } です。
conflict-both-changed = 二人とも変更した項目
conflict-both-changed-detail = これらの項目は二人とも編集しています。保存すると { $who } が入れた内容を置き換えます：
conflict-field-by-field = 項目ごとの比較
conflict-theirs = { $who } が変更した内容
conflict-yours = あなたが変更した内容
conflict-unchanged-by-you = あなたは変更していません
conflict-unchanged-by-them = 相手は変更していません
conflict-what-now = これからどうするか
conflict-reapply = 相手の版の上に自分の版を適用する
conflict-save-over = これで相手の版を上書きする
conflict-discard = 自分の版を破棄してやり直す
conflict-their-version = ファイルが現在保持している { $who } の版
conflict-history-of = この記録（{ $kind }）の変更履歴

home-lede = { $family } — 単一の .axgf ファイル内の { $total ->
       *[other] { $total } 件の記録
    }。
home-why-title = なぜ AXGF か
home-what-this-bundle-has = このファイルが実際に含むもの
home-browse-tree = 家系図を見る
home-convert-gedcom = GEDCOM を変換する
home-see-example = 例を見る

convert-title = GEDCOM を AXGF に変換
convert-choose-file = GEDCOM ファイル
convert-submit = 変換
convert-result-title = 変換結果
convert-download = .axgf ファイルをダウンロード
convert-diagnostics = 変換器の報告
convert-unchanged-note = 変換がこのサイトの配信しているファイルに触れることはありません。

completeness-title = ファイルの充足度
completeness-recorded = 記録あり
completeness-empty = 空
completeness-spec-field = AXGF 項目

## Dates

date-unknown = 日付不明
date-not-recorded = 記録なし
date-circa = { $date } 頃
date-between = { $from } から { $to } の間
date-before = { $date } 以前
date-after = { $date } 以降
date-preserved = 「{ $text }」と記録
date-day-month-year = { $year } 年 { $month } { $day } 日
date-month-year = { $year } 年 { $month }
date-decade = { $decade } 年代
date-century = { $century } 世紀

month-1 = 1月
month-2 = 2月
month-3 = 3月
month-4 = 4月
month-5 = 5月
month-6 = 6月
month-7 = 7月
month-8 = 8月
month-9 = 9月
month-10 = 10月
month-11 = 11月
month-12 = 12月
