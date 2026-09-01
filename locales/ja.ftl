# axgf-cms — インターフェース文字列、日本語。
#
# 機械品質 — 日本語を母語とする人による校閲を経ていません。系譜学の語彙には
# 記録の伝統ごとに異なる定訳があり、この翻訳は誤っている可能性があります。
# 訂正を歓迎します — CONTRIBUTING.md をご覧ください。
#
# 採用した訳語（母語話者による異論を歓迎します）:
#   union → 婚姻関係 ・ link → 関連 ・ confidence → 確度
#   reliability → 信頼度 ・ source → 出典 ・ primary source → 一次資料
#   occupation → 職業 ・ record → 記録 ・ archive → アーカイブ
#   godparent → 代父母 ・ witness → 証人 ・ speculative → 推定
#
# 複数形: 日本語に複数形の区別はないため、CLDR の分類は other のみです。
# 英語の「1つか複数か」の分岐を持ち込まないでください。
#
# 日付: 1923年4月12日。月の名前ではなく数字で組み立てる構造なので、月名の
# 一覧はありません。
#
# 約物: 全角の「。」「、」を用い、英語の句読点をそのまま持ち込みません。
#
# 原則: このファイルが訳すのはインターフェースだけです。人名、地名、覚書、
# 職業名はアーカイブに由来し、それぞれの言語と文字のまま表示されます。

app-name = ax-genealogy

## ヘッダーとフッター

nav-tree = 家系図
nav-convert = 取り込み
nav-admin = 管理
nav-sign-in = ログイン
nav-sign-out = ログアウト
footer-open-format = ご家族のアーカイブは、手元に残る一つのファイルです。公開された形式で書かれているため、このサイトがなくなったあとも長く開くことができます。
footer-open-format-link = 形式について

## 設定

prefs-title = 言語と表示
prefs-language = 言語
prefs-theme = 表示
prefs-background = 背景
prefs-background-on = ページの背後にごく淡い色のぼかしを敷く
prefs-apply = 適用
prefs-reviewed = 校閲済み
prefs-machine = 機械翻訳、{ $coverage }%
prefs-machine-complete = 全項目訳出、未校閲
prefs-machine-title = 母語話者の校閲を経ずに翻訳されています。とくに系譜学の語彙は誤っている可能性があります。婚姻関係、代父母、一次資料を表す語は、国ごとの記録の伝統によって異なります。訂正を歓迎します。CONTRIBUTING.md に着手点を記しています。

theme-light = 明るい
theme-dark = 暗い
theme-system = システムに合わせる
theme-high-contrast = 高コントラスト
theme-sepia = セピア
theme-deuteranopia = 第二色覚
theme-protanopia = 第一色覚
theme-tritanopia = 第三色覚
theme-colour-blind-note = 色覚に配慮
theme-contrast-note = 最大コントラスト

## 家系図

tree-title-around = { $name }の周辺
tree-title-whole = 家系図の全体
tree-lede-focused = 祖先{ $ancestors }名、子孫{ $descendants }名、配偶者・伴侶{ $spouses }名、各方向{ $depth }世代。
tree-filter-label = 表示中のカードを絞り込む
tree-filter-placeholder = 名前を入力…
tree-centre-on = 中心にする人
tree-depth = 各方向の世代数
tree-show = 表示
tree-hidden-notice = { $n }名は詳細を伏せて表示されています
tree-hidden-because-role = 。閲覧範囲がこのアカウントに許された範囲を超えているためです。
tree-hidden-because-anonymous = 。公開されていないためです。
tree-hidden-sign-in = アカウントをお持ちならログインしてください。
tree-restricted-card = この人の記録は閲覧できません
tree-empty = まだ描く相手がいません。
tree-unplaced = どの家族にも記録されていない

## 記録

record-identity = 人物情報
record-life-events = 生涯の出来事
record-family = 家族
record-other-relationships = その他の関係
record-occupations = 職業
record-places = 場所
record-sources-documents = 出典と資料
record-notes = 覚書
record-history = 変更履歴
record-raw = 生データ
record-raw-summary-note = このページの元になった JSON

record-identity-help = 記録されたすべての名前と、その種別、使われた期間、裏づけとなる出典。原文の文字とラテン文字転写が異なる場合は並べて示します。性別、存命かどうか、公開範囲も含みます。
record-life-events-help = 出生、死亡、そしてこの人が関わったすべての出来事を日付順に、それぞれの立場とともに示します。証人として立ち会っただけの結婚も、本人の結婚と並びます。日付のない事実は先頭を装わず、末尾に置かれます。
record-family-help = 両親ときょうだい、続いて各婚姻関係の種別、日付、場所、終わり方、そして出生順の子どもたち。
record-other-relationships-help = この人が一方の端にあるすべての関連を、その人の側から読んだもの。同じ記録が一方から見れば「〜の代父」、他方から見れば「〜の代子」となります。
record-occupations-help = 職業を一本の共通軸の上の期間として示すので、二つの勤めを目で比べられます。端が不明なものは棒が開いたままになります。
record-places-help = この記録が触れるすべての場所と、そこで起きたこと、そして時代をまたいで場所の意味を支える国境の変遷。
record-sources-documents-help = 各出典は、そのページ上でその出典に依拠している事実を、根拠の強さの順に挙げます。
record-notes-help = この記録についての覚書。どの変換器も解釈できず、捨てる代わりに原文のまま保存された文字列も含みます。
record-history-help = この記録に保存されたすべての変更を新しい順に。誰が何を直したかは、家系図を管理する人についての事実であって、そこに記された家族についての事実ではありません。そのため書き出されるアーカイブには含めず、ログインした親族にのみ表示します。
record-raw-help = ここに表示用に作られたものはありません。項目名に至るまで、保存されているままの記録です。将来このサイトなしでアーカイブを読む必要が生じたとき、目にするのはこれです。
record-help-toggle = この節に表示される内容

record-gender = 性別
record-living = 存命
record-visibility = 公開範囲
record-yes = はい
record-no = いいえ
record-name-type = 名前の種別
record-name-used = 使用期間
record-name-evidence = 根拠
record-transliteration = ラテン文字転写
record-born = 出生
record-died = 死亡
record-parents = 両親
record-siblings = きょうだい
record-children = 子ども
record-unknown-person = ［不明］
record-restricted-person = 非公開
record-restricted-title = この人の記録は閲覧できません
record-absent-person-title = この家系図に名前は出るが記録はない
record-confidence = 確度
record-source = 出典
record-download = ダウンロード

## アクセス

access-restricted-title = 閲覧できません
access-restricted-signed-in = この記録の公開範囲は、このアカウントに許された範囲を超えています。管理者が記録の公開範囲か、あなたの権限のいずれかを変更できます。
access-restricted-anonymous = この記録は公開されていません。ログインして、このアカウントで読めるかどうかご確認ください。
access-role-title = この権限では扱えません
access-role-admin = ここは管理者用のページです。このアカウントは記録の作成と編集はできますが、アカウントの管理、記録の削除、アーカイブの書き出しはできません。
access-role-write = このアカウントは家系図を読めますが、変更はできません。管理者が権限を編集者に引き上げられます。
access-scope-title = 担当する系統の外です
access-scope-named = このアカウントは家系図の一つの系統に限られており、この記録はその外の人に関わります。記録に名前が挙がるすべての人が担当系統の内側になければなりません。そうでなければ、外部の配偶者を含む家族が、その人の親子関係を書き換える抜け道になってしまいます。
access-scope-unnamed = このアカウントは家系図の一つの系統に限られており、この記録には照合できる人物が挙がっていません。出典と場所は、家系図全体にアクセスできるアカウントが編集します。

## エラー

error-not-found-title = 見つかりません
error-not-found-detail = そのページはここにはありません。
error-no-such-person-title = 該当する人がいません
error-no-such-person-detail = その識別子を持つ人はここにいません。
error-no-such-entity-title = 該当するものがありません
error-no-such-entity-detail = その識別子を持つ記録はここにありません。
error-deleted-while-editing = その識別子を持つ記録はここにありません。編集中に削除された可能性があります。
error-no-such-file-title = 該当するファイルがありません
error-no-such-file-detail = その識別子を持つ資料はここにないか、資料がファイルなしで記録されています。参照だけの資料は、別の場所にあるものを指しています。
error-not-an-image-title = 画像ではありません
error-not-an-image-detail = この資料には縮小画像がありません。この版で解読できる画像ではないためです。
error-back = 戻る

## ログイン

login-title = ログイン
login-lede = アカウントは管理者が作成します。
login-username = ユーザー名
login-password = パスワード
login-submit = ログイン
login-wrong = そのユーザー名とパスワードは一致しません。
login-token-wrong = そのトークンは正しくありません。
login-throttled = 失敗が続いています。数分おいてからもう一度お試しください。
login-no-accounts-title = この導入環境にはまだアカウントがありません。
login-no-accounts-detail = 初期設定ページを意図的に置いていません。設置から最初のログインまでの間こそ、環境が無防備になる時間だからです。最初の管理者はコマンドラインから作成します。
login-no-accounts-note = 生成されたパスワードは stderr に一度だけ出力され、二度と表示されません。それまでの唯一の入口が下の緊急トークンです。
login-emergency-summary = 緊急アクセス
login-emergency-detail = 共有トークンは今も管理者セッションを開きます。目的は一つ、.acl ファイルを失ったときや、すべての管理者が締め出されたときに戻る手段です。これはアカウントではありません。固有の設定を持たず、変更履歴には人物ではなく emergency-token として記録されます。使用は警告として記録に残ります。
login-emergency-label = 緊急トークン
login-emergency-submit = 緊急トークンを使う
login-sign-in-prompt = 管理画面に入るにはログインしてください。

## 管理

admin-title = 管理
admin-lede = { $path } を編集中 — 記録{ $total }件、添付ファイル{ $files }件、ディスク上{ $size }。変更はすべて一度に書き込まれ、拒否された変更はファイルに触れません。
admin-entities = 記録
admin-create = 作成
admin-new-kind = 新規: { $kind }
admin-operations = 操作
admin-validate = 検証
admin-deduplicate = 重複を統合
admin-export = アーカイブを書き出す
admin-accounts = アカウント
admin-roles-note = 検証、重複の統合、書き出し、削除、アカウント管理は管理者のみが行えます。編集者はここにあるその他すべてのページを利用できます。
admin-dedup-confirm = 重複の統合は記録を融合し、アーカイブを書き換えます。続けますか。
admin-recent-changes = 最近の変更
admin-recent-note = { $path } に記録された{ $total }件のうち直近の{ $shown }件。
admin-sessions-open = 現在{ $n }件のセッションが開いています。
admin-no-changes-yet = このアプリケーションからはまだ何も変更されていません。これ以降の保存はすべて { $path } に記録されます。
admin-last-validation = 前回の検証
admin-bundle-heavy = このアーカイブは{ $size }です。起動時に全体を読み込んで常時メモリに保持するため、おおよそ{ $warn }を超えるとサイトが実メモリを消費しはじめ、再起動も遅くなります。この設計が向くのは家族のアーカイブであってメディア書庫ではありません。添付が際限なく増えるなら、ファイル保管庫に置き、アーカイブからはそれを指すようにしてください。

admin-fields = 項目
admin-raw-json = 生の JSON
admin-raw-json-help = 記録の全体です。編集できないものは何もありません。家族の配偶者や子どもの一覧、場所の国境変遷といった配列はここにあります。これが出発点の文書で、上の各項目はそのあと自分の担当する場所へ書き込まれます。ですから値の編集はどちらか一方で行い、両方では行わないでください。JSON として解釈できなければ何も保存されません。
admin-save = 保存
admin-cancel = 取消
admin-delete = 削除
admin-not-set = — 未設定 —
admin-edit = 編集
admin-page-of = { $pages }ページ中{ $page }ページ目
admin-previous = 前へ
admin-next = 次へ
admin-saved = 版{ $version }として保存しました — { $summary }
admin-not-saved = 保存されませんでした
admin-created = 作成しました
admin-not-created = 作成されませんでした
admin-deleted = 削除しました
admin-not-deleted = 削除していません — 何も変更されていません
admin-what-changed = 変更内容
admin-field = 項目
admin-from = 変更前
admin-to = 変更後
admin-version = 版{ $version }

## アカウント

accounts-title = アカウント
accounts-lede = { $path } に権限 600 で保存されます。アーカイブの隣であって、決して中ではありません。アーカイブは複製され、送られ、公開されます。その中をパスワードのハッシュが一緒に運ばれるなら、家系図の複製がそのまま認証情報の複製になってしまいます。
accounts-existing = 既存のアカウント
accounts-username = ユーザー名
accounts-role = 権限
accounts-status = 状態
accounts-branch = 担当系統
accounts-last-seen = 最終ログイン
accounts-change = 変更
accounts-you = （あなた）
accounts-active = 有効
accounts-disabled = 無効
accounts-never = なし
accounts-whole-tree = 家系図全体
accounts-roots = 起点{ $n }名
accounts-add = アカウントを追加
accounts-no-registration = 自己登録も招待の仕組みも意図的に設けていません。家族のアーカイブなら全員を知る管理者がいれば足り、防ぐ代わりに悪用の余地そのものを取り除けます。
accounts-password-hint = 空欄にすると自動生成され、一度だけ表示されます。ご自身で設定する場合は{ $min }文字以上。
accounts-new-password-placeholder = 新しいパスワード（空欄なら現状のまま）
accounts-email = メールアドレス
accounts-optional = （任意）
accounts-create = アカウントを作成
accounts-role-viewer = 閲覧者 — 公開記録と家族向け記録を読めます
accounts-role-contributor = 編集者 — 加えて作成、編集、ファイルの追加ができます
accounts-role-admin = 管理者 — 加えてアカウント管理、削除、書き出しができます
accounts-branch-hint = このアカウントが編集できる範囲を、指定した人々とその子孫、配偶者に限ります。
accounts-branch-reading = 読める範囲は制限しません。それは各記録の公開範囲が決めることであり、二つは意図的に切り離してあります。
accounts-branch-placeholder = 1行に1つずつ人物の識別子
accounts-ids-in-bundle = この家系図にある人物の識別子
accounts-emergency-warning = 緊急トークンでログインしています。このセッションのあいだ管理者権限を与えますが、アカウントではありません。固有の設定を持たず、変更履歴にはあなたの変更が人物ではなく emergency-token として記録されます。下で正式なアカウントを作り、それでログインし直してください。
accounts-created-with-password = { $username } を作成しました。パスワードは { $password } です。表示は一度きりで、保存されるのは Argon2id のハッシュだけですから、今のうちにお伝えください。
accounts-created = { $username } を作成しました。
accounts-updated = { $username } を更新しました。開いていたセッションはすべてログアウトされました。
accounts-username-taken = そのユーザー名はすでに使われています。
accounts-pick-role = 権限を選んでください。
accounts-no-such = そのアカウントはありません。
accounts-last-admin = 有効な管理者はこれ一つだけです。先に別の人を昇格させてください。管理者のいない環境は、.acl ファイルを直接編集するか緊急トークンを使う以外に戻す手立てがありません。
accounts-not-saved = 保存されませんでした: { $error }

## 競合

conflict-title = 先に別の人が変更しました
conflict-lede = あなたが開いたあと、{ $when } に { $who } がこの{ $kind }への変更を保存しました。あなたの編集は保存されておらず、何も上書きされていません。
conflict-no-merge = ここでは自動的な統合を行いません。二人の編集を融合すると、どちらも選んでいない記録ができあがります。系譜では日付をめぐる二人の食い違いは、たいてい別々の出典を読んでいることを意味します。それはプログラムではなく人が答えるべき問いです。下の二つを見比べて判断してください。
conflict-versions = あなたが着手したのは版{ $expected }ですが、記録は現在版{ $current }です。
conflict-both-changed = 双方が変更した項目
conflict-both-changed-detail = これらの項目は双方が編集しています。何を保存しても、{ $who } が入れた内容を置き換えます。
conflict-different-fields = 変更した項目が異なるため、{ $who } の作業と争う点はありません。ただし再適用すると、あなたの記録全体が相手のものに上書きされます。保存前に両方の列をご確認ください。
conflict-field-by-field = 項目ごとの比較
conflict-theirs = { $who } による変更後の値
conflict-yours = あなたによる変更後の値
conflict-unchanged-by-you = あなたは変更していません
conflict-unchanged-by-them = 相手は変更していません
conflict-nothing-differs = このページに表示されるどの項目でも、両方の版があなたの着手時と違いません。版番号だけが進んでいるので、誰かが内容を変えずに保存したことになります。
conflict-what-now = ここからどうするか
conflict-reapply = あなたの版を相手の版の上に適用し直す
conflict-reapply-hint = 版{ $version }に対して持ち越したあなたの編集です。{ $who } の作業から残したい部分をここで取り込んでから保存してください。相手の版は下に、書き写せるように表示しています。
conflict-save-over = これを相手の版の上に保存する
conflict-discard = 自分の編集を捨ててやり直す
conflict-their-version = { $who } の版（現在の内容）
conflict-history-of = この{ $kind }の履歴

## 取り込み

convert-title = 家族のファイルを取り込む
convert-submit = 取り込む
convert-result-title = 取り込み報告
convert-download = アーカイブをダウンロード

## 日付

date-unknown = 日付不明
date-not-recorded = 記録なし
date-circa = { $date }頃
date-between = { $from }から{ $to }のあいだ
date-before = { $date }以前
date-after = { $date }以降
date-preserved = 「{ $text }」と記録
date-day-month-year = { $year }年{ $month }月{ $day }日
date-month-year = { $year }年{ $month }月
date-decade = { $decade }年代
date-century = { $century }世紀
date-quarter-century = { $century }世紀第{ $quarter ->
        [1] 1
        [2] 2
        [3] 3
       *[other] 4
    }四半期

## その他のエラーページ

error-back-to-start = 最初に戻る
error-payload-missing-title = 該当するファイルがありません
error-payload-missing-detail = その資料の中身はキャッシュにありません。
error-payload-unopenable-detail = その資料の中身を開けませんでした。
error-no-such-document-detail = その識別子を持つ資料はここにありません。
error-bad-preference-title = 選択肢にありません
error-bad-preference-detail = このサイトが提供する言語でも表示設定でもありません。何も変更していません。
error-unknown-kind-title = 未知の種別
error-unknown-kind-detail = 「{ $kind }」は記録の種別ではありません。このアーカイブに含まれるのは { $kinds } です。
error-io-title = 保存できませんでした
error-io-detail = { $error }。ディスク上は何も変更されていません。
error-upload-too-large = そのファイルは{ $mb } MB の上限を超えています。何も保存されておらず、アーカイブは元のままです。
error-upload-refused = 資料は受け付けられませんでした: { $reason }。アーカイブは元のままです。
error-back-to-person = 記録に戻る
error-no-such-person-to-attach = その識別子を持つ人はここにいないため、資料を添付する相手がありません。
error-upload-title = そのアップロードは保存されませんでした
error-download-expired-title = そのダウンロードは期限切れです
error-download-expired-detail = 取り込んだ結果は十五分間保持され、その後破棄されます。ファイルをもう一度取り込んでください。
error-upload-none = ファイルが送られていません。まずファイルを選んでください。
error-upload-unsupported = その種類のファイルはアーカイブに保存できません。画像、PDF、平文テキスト、音声、動画を受け付けます。種類はファイル自身のバイト列から判定するので、実行ファイルの名前を変えても通りません。SVG はスクリプトを含みうるため、一律に拒否します。
error-export-unreadable-title = 書き出したアーカイブを読めませんでした
error-export-unreadable-detail = { $error }

## 家系図のページ（続き）

tree-title-suffix = 家系図
tree-back-to-focused = 一人を中心とした表示に戻る
tree-show-all = { $n }名すべてを表示
tree-width-notice = この表示は幅{ $width }ピクセルです。幅1500ピクセルの画面なら{ $screens }画面分の横スクロールにあたります。
tree-confidence-label = 確度:
tree-band-certain = 確実
tree-band-high = 高い
tree-band-medium = 中程度
tree-band-low = 推定
tree-counts = { $total }名中{ $drawn }名 ・ { $generations }世代
tree-unplaced-count = 未配置{ $n }名
tree-contradicts-title = この家系図は矛盾しています。
tree-contradicts-detail = どのように行を並べても成り立たないため、下の関係は世代の番号付けから除いてあり、一部の行が誤った位置に描かれている可能性があります。二つの記録のうち誤っているほうを直してください。
tree-contradicts-pair = 夫婦としても親子としても記録されています:
tree-contradicts-more = ほかに{ $n }件の矛盾は挙げていません。
tree-no-people = この家系図にはまだ誰もいません。
tree-no-people-cta = 家族のファイルを取り込むか、最初の一人を登録してください。
tree-nobody-selected = その条件では描く相手がいません。
tree-nobody-selected-cta = 既定の表示から始めてください。
tree-edge-union = 記録された婚姻関係
tree-edge-parentage = 記録された親子関係

## トップページ

home-empty = まだ何も記録されていません。家族のファイルを取り込んで既存の家系図を移すか、最初の一人を手で登録してください。
home-count = 記録{ $total }件。家族のものである一つのファイルに収まっています。
home-browse = 家系図を見る
home-convert = 家族のファイルを取り込む
home-unnamed-family = この家系図
home-in-this-tree = 家族がこれまでに記録したもの
home-showcase-title = この家系図がすでに名前と日付以上を語っているところ
home-showcase-example = 例を見る →
home-nothing-title = まだお見せするものがありません。
home-nothing-detail = 家族のファイルを取り込んで既存の家系図を移すか、何もないところから最初の一人をご自身で登録してください。

## 概観カード

showcase-links-title = 家族の外の関係{ $n }件
showcase-links-detail = 代父母、雇い主、証人、師。それぞれに固有の日付と出典、そしてあなたの確度がつきます。
showcase-occupations-title = 始まりと終わりのある職業{ $n }件
showcase-occupations-detail = 「小学校教員、1948〜1978年」はその長さを保ち、一本の日付の行ではなく年をまたぐ帯として描かれます。
showcase-uncertain-title = 伝えられたままの曖昧さで残された日付{ $n }件
showcase-uncertain-detail = 頃、以前、以降、あいだは、それぞれ別の主張のまま残ります。出典が絞り込めなかった日付を、絞り込めたかのように見せることはありません。
showcase-preserved-title = 書かれた言葉のまま残された日付{ $n }件
showcase-preserved-detail = 誰も日付として読めなかった表現は、黙って捨てられるのではなく、書かれたとおりに残ります。
showcase-sources-title = 信頼度が記録された出典{ $n }件
showcase-sources-detail = うち一次資料{ $primary }件。どの事実も、何を根拠にしているか、その根拠がどれだけ強いかを示します。
showcase-places-title = 国境が動いた場所{ $n }件
showcase-places-detail = 一つの町が時代によって別の国に属することがあり、記録はどの時期にどちらだったかを示します。

## 記録の細目

record-also-recorded-as = 別の記載
record-borders-moved = 国境の変遷:
record-display-name = 表示名
record-read-as = 読み
record-note = 覚書
record-living-yes = 存命
record-deceased = 故人
record-centre-tree-here = ここを家系図の中心にする
record-centre-tree-title = この人を中心にして家系図を動かす
record-open-full-page = 単独のページで開く ↗
record-open-full-title = 共有できる単独のページを開く
record-edit = 編集
panel-empty = カードを選ぶと、その人の記録の全体がここに表示されます。
person-see-in-tree = この人を家系図で見る
person-visibility-inline = 公開範囲:

## 操作の結果

result-diagnostics = 診断
result-diagnostics-note = ライブラリが返したすべての診断です。操作を止めなかった警告も含み、除外はしていません。
result-no-diagnostics = ライブラリからの診断はありませんでした。
result-continue = 続ける
result-dashboard = ダッシュボード
person-sections-label = このページの節

## 記録の各節（細目）

record-notes-title = この記録について特筆すべき点:
record-name = 名前
record-type = 種別
record-cause = 死因:
record-as = 立場
record-partner-not-recorded = 配偶者・伴侶は未記録
record-union-from = 開始
record-union-at = 場所
record-union-until = 終了
record-occupation-from = 開始
record-occupation-until = 終了
record-source-reliability = 信頼度
record-source-supports = 裏づける事実
record-photographs = 写真
record-documents = 資料
record-file = ファイル
record-status = 状態
record-size = サイズ
record-absent-document = この人が挙げているが、ここには保存されていません。
record-no-file = ファイルなし
record-attach-document = 資料を添付する
record-doc-photo = 写真
record-doc-certificate = 証書
record-doc-letter = 書簡
record-doc-record = 記録
record-doc-newspaper = 新聞
record-doc-other = その他
record-upload = アップロード
record-upload-help = 1ファイルにつき{ $mb } MB まで。添付は家系図の隣に保管され、書き出しのときアーカイブへ書き戻されるので、写真はそれが属する家族と一緒に移動します。種類はファイル名ではなく中身から判定し、画像、PDF、平文テキスト、音声、動画を受け付けます。SVG はスクリプトを含みうるため拒否します。
record-upload-help-short = { $mb } MB まで。SVG は拒否します。
record-verbatim-note = どの変換器も解釈できなかったため、記録にあったとおりに残してあります。
record-file-to-attach = 添付するファイル
record-document-type = 資料の種別
record-caption = 説明
record-caption-placeholder = 説明（任意）
record-history-entry-meta = — { $at }
record-history-entry-version = 、{ $version }

## 記録の種別

kind-person = 人物
kind-family = 家族
kind-event = 出来事
kind-link = 関連
kind-occupation = 職業
kind-source = 出典
kind-place = 場所
kind-document = 資料

kind-person-plural = 人物
kind-family-plural = 家族
kind-event-plural = 出来事
kind-link-plural = 関連
kind-occupation-plural = 職業
kind-source-plural = 出典
kind-place-plural = 場所
kind-document-plural = 資料

## 一覧

list-matching = 該当{ $total }件、1ページあたり{ $per_page }件。
list-filter-placeholder = 名前または識別子で絞り込む
list-filter = 絞り込む
list-clear = 解除
list-summary = 概要
list-id = 識別子
list-actions = 操作
list-nothing = ここには何もありません。
list-nothing-matching = 「{ $q }」に該当するものはここにありません。
list-delete-confirm = この{ $kind }を削除しますか。これを参照している記録の扱いを選んでください:
list-policy-reject = 拒否
list-policy-reject-detail = — まだ参照が残っていれば削除しません。何も失われません。
list-policy-cascade = 連鎖削除
list-policy-cascade-detail = — これを削除し、参照もすべて実際に取り除きます。
list-policy-orphan = 参照を空にする
list-policy-orphan-detail = — これを削除しますが、参照していた記録は残し、参照だけを空にします。

## 記録の充実度

completeness-dates-title = 日付を、実際に持っている形で分類する
completeness-no-dates = まだ日付は記録されていません。
completeness-dates-note = 日まで特定できた日付と、十年単位でしか置けなかった日付は、別の主張です。どちらも与えられたまま保存します。日付として読めなかった文字列は、捨てずに一字一句そのまま残します。
completeness-shape-exact = 確定
completeness-shape-exact-note = 暦日まで判明
completeness-shape-approximate = 概数
completeness-shape-approximate-note = 「頃」、または年か年代のみ
completeness-shape-ranged = 範囲
completeness-shape-ranged-note = 以前、以降、またはあいだ
completeness-shape-preserved = 原文のまま
completeness-shape-preserved-note = 解釈できない文字列を、そのまま保存
completeness-shape-unknown = 不明
completeness-shape-unknown-note = 不明として記録

## 取り込みのページ

convert-page-title = 家族のファイルを取り込む
convert-lede = 既存の家系図を GEDCOM ファイルから移せます。GEDCOM はたいていの系譜ソフトが書き出す形式です。ここには何も保存されず、このサイトがすでに表示している家系図もそのままです。
convert-file-label = 家族のファイル（.ged）
convert-file-hint = { $mb } MB まで。767名の家系図でおよそ 320 KB です。
convert-confidence-label = これらの事実の出発点となる確からしさ
convert-confidence-hint = 取り込むファイルには、誰がどれだけ確信していたかが書かれていません。そのためどの事実にも出発点が要ります。急いで集めた家系図なら低めに、文献に当たって組んだものなら高めにしてください。この数値の正直な読み方は「取り込んだままで、以後誰も検めていない」です。あとから一つずつ上げ下げできます。
convert-lang-label = 地名の言語
convert-lang-hint = en、fr、ja のような表示です。

## 取り込み報告

convert-failed = 取り込みは完了しませんでした
convert-try-another = 別のファイルを試す
convert-converted = { $filename } を取り込みました
convert-result-lede = 記録{ $total }件、{ $size } KB。すべて確からしさ{ $confidence }で入り、地名は{ $lang }として読みました。このサイトが表示している家系図には触れていません。
convert-produced = 移ってきたもの
convert-skipped-title = 読み取れなかった項目{ $n }件
convert-skipped-note = これらの項目には、移せる中身がありませんでした。
convert-other-diagnostics = 知っておくとよいことが他に{ $n }件
convert-clean = 後に残されたものはありません。ファイルのすべての項目が移りました。
convert-download-title = ダウンロード
convert-download-named = { $name } をダウンロード
convert-download-note = ここには十五分だけ置かれ、その後破棄されますので、今のうちにダウンロードしてください。
convert-another = 別のファイルを取り込む
admin-history-on = 対象
admin-history-meta = — { $kind }、{ $at }
admin-validation-counts = エラー{ $errors }件、警告{ $warnings }件、注記{ $infos }件。
admin-warnings-never-block = 警告が処理を止めることはありません。関門ではなく情報です。
admin-validator-clean = 検証は何も報告しませんでした。
record-occupations-help-undated = 職業は始まりと終わりとともに記録するので、複数を一本の時間軸で比べられます。このアーカイブには職名はあっても日付がありません。たいていの家族向けファイルには日付を置く場所がないため、取り込みのあとではよくあることです。そのため描くべき目盛りがまだありません。
record-occupations-help-axis = 職業は一つの日付の出来事ではなく、長さを持つ状態です。すべての期間が同じ軸{ $from }〜{ $to }を共有します。
admin-value-not-set = 未設定
admin-validation-report = 検証結果
admin-dedup-complete = 重複の統合が完了しました
admin-dedup-refused = 重複の統合は行われませんでした
record-birth-order = 出生順
record-start-not-recorded = 開始は未記録
record-end-not-recorded = 終了は未記録
record-document-no-file = 資料はここに記録されていますが、ファイル自体はありません
panel-selected-person = 選択中の人物

## 世代の帯

tree-band-generation = 第{ $g }世代
tree-band-people = { $n }名
tree-band-unplaced = 未配置
tree-band-unplaced-note = どの家族にも属さない{ $n }名 — 省かずに表示しています

## 統制語彙

gender-M = 男性
gender-F = 女性
gender-NB = ノンバイナリー
gender-unrecorded = 未記録

name-part-given_name = 名
name-part-family_name = 姓
name-part-patronymic = 父称
name-part-matronymic = 母称
name-part-middle_name = ミドルネーム
name-part-nickname = 通称
name-part-prefix = 前置辞
name-part-suffix = 後置辞
name-part-particle = 冠称
name-part-part = 構成要素

name-type-primary = 主たる名
name-type-other = その他
name-type-alias = 通名
name-type-birth = 出生名
name-type-married = 婚姻後の姓
name-type-religious = 宗教上の名
name-type-transliteration = 転写
name-type-nickname = 通称

## 記録に添える注記

note-links = 固有の日付と出典を持つ家族外の関係{ $n }件
note-occupations = 始まりと終わりとともに記録された職業{ $n }件
note-birth-imprecise = 出典が特定できなかった出生日を、記録どおりに表示
note-death-imprecise = 出典が特定できなかった死亡日を、記録どおりに表示
note-names = 記録された名前{ $n }件
note-transliteration = 元の文字による名前と、そのラテン文字転写
note-witnessed = 当事者ではなく証人として関わった出来事{ $n }件

visibility-public = 公開
visibility-members = 家族
visibility-contributors = 編集者
visibility-private = 非公開

## 管理一覧の行の見出し

family-label-couple = { $children ->
        [0] { $a }と{ $b }
       *[other] { $a }と{ $b } — 子{ $children }名
    }
family-label-half = { $children ->
        [0] { $a }と{ $unknown }
       *[other] { $a }と{ $unknown } — 子{ $children }名
    }
family-label-children = { $others ->
        [0] { $first } — 両親は未記録
       *[other] { $first }ほかきょうだい{ $others }名 — 両親は未記録
    }
family-label-empty = 誰も記録されていない家族

event-label = { $category } — { $who }、{ $date }
event-label-nobody = { $category } — { $date }
event-two-people = { $a }と{ $b }
event-more-people = { $a }と{ $b }ほか{ $others }名

link-label = { $label }: { $from } → { $to }
occupation-label = { $who } — { $title }
source-label = { $title } — { $reliability }
source-label-plain = { $title }
document-label = { $filename } — { $type }
document-label-untitled = 無題の{ $type }
list-unnamed = 名称のない{ $kind }

## 一覧で使う仕様の語彙

event-category-birth = 出生
event-category-death = 死亡
event-category-marriage = 婚姻
event-category-divorce = 離婚
event-category-baptism = 洗礼
event-category-burial = 埋葬
event-category-immigration = 移入
event-category-emigration = 移出
event-category-census = 国勢調査
event-category-residence = 居住
event-category-military = 兵役
event-category-education = 教育
event-category-other = 出来事

reliability-primary = 一次資料
reliability-secondary = 二次資料
reliability-tertiary = 三次資料
reliability-recollection = 口述
reliability-derivative = 二次的著作
reliability-authored = 著述資料
reliability-oral = 口承
reliability-unknown = 信頼度不明

document-type-photo = 写真
document-type-certificate = 証書
document-type-letter = 書簡
document-type-record = 文書記録
document-type-newspaper = 新聞の切り抜き
document-type-other = 資料

## この記録がもっと語れるところ

completeness-title = この家系図がもっと語れるところ
completeness-intro = 何が記録され、何がまだ空欄かを示します。
completeness-import-title = 取り込みが運んできたもの
completeness-import-intro = いま送っていただいたファイルから数えたものです。空欄は元のファイルが記録していなかった事柄であって、取り込みが失ったものではありません。

completeness-headline-full = 以下のどの種類の情報も、この家系図のどこかに記録されています。
completeness-headline-empty = 以下の{ $total }種類の情報は、まだどこにも記録されていません。いずれも記録がもっと語れる場所です。
completeness-headline-partial = 以下のうち{ $carried }種類が記録され、{ $empty }種類がまだ空欄です。

completeness-metric-confidence = 各事実がどれだけ確かか
completeness-metric-confidence-none = ここにある{ $slots }件の事実は、どれも自らの確からしさを述べていません。証書から書き写した日付と、推測した日付は、区別がつかなくなるまで同じに見えます。
completeness-metric-confidence-uniform = { $slots }件のうち{ $with }件に数値がついていますが、すべて同じ値（{ $modal }）です。一括取り込みが残していくのはこれです。誰も戻って見直していない既定値であり、個別に判断されたものはまだありません。
completeness-metric-confidence-some = { $slots }件のうち{ $with }件に数値がついています。{ $modal_count }件が同じ値（{ $modal }）で、{ $assessed }件はそこから外れており、一件ずつ検討されたことがわかります。
completeness-metric-confidence-many = { $slots }件のうち{ $with }件に数値がつき、そのうち{ $assessed }件が最頻値（{ $modal }）から外れて、{ $distinct }段階に分かれています。この家系図は、実際に幅のある不確かさを記録しています。
completeness-metric-parentage = 各親子関係がどれだけ確かか
completeness-metric-parentage-none = ここにある親子関係は、どれも自らの確からしさを述べていません。養子縁組、争いのある系統、一度の言及からの復元こそ、家族が疑いを書き留めるべき場所です。確かさの低い関係を、家系図は薄い線で描きます。
completeness-metric-parentage-some = { $n }件の親子関係が固有の数値を持つので、推定の線が記録に裏づけられた線より目に見えて弱く見えます。

completeness-metric-links = 血縁と婚姻の外にある関係
completeness-metric-links-none = 代父母、雇い主、証人、師、後見人。まだ一件も記録されていません。それぞれに固有の日付と出典、そしてあなたの確度を持たせられます。
completeness-metric-links-some = { $n }件を記録済み。いずれも固有の日付と出典、そしてあなたの確度を持っています。

completeness-metric-occupations = 始まりと終わりとともに記録された職業
completeness-metric-occupations-none = 職業が記録されていません。三十年続けた生業は、日付のついた一行よりも一つの生涯を語ります。
completeness-metric-occupations-undated = 職業{ $total }件が日付なしで記録されています。始まりと終わりを加えれば、一本の時間軸の上で並べて比べられます。
completeness-metric-occupations-some = { $total }件のうち{ $span }件に始まりか終わりがあるので、一本の時間軸の上で並べて比べられます。

completeness-metric-sources = 信頼度が評価された出典
completeness-metric-sources-none = 出典が記録されていません。ある事実がどこから来たかを示すことが、後日それを親族が確かめられる、あるいは理由を挙げて異を唱えられる条件です。
completeness-metric-sources-some = { $total }件のうち{ $graded }件が自らの強さを示しているので、出生証書に基づく主張と、記憶に基づく主張とが目に見えて区別されます。

completeness-what-is-recorded = 記録が語りうること
completeness-in-this-tree = この家系図では
completeness-not-yet = まだ記録なし

## 出来事における参加者の立場

role-spouse = 配偶者
role-spouse_1 = 第一配偶者
role-spouse_2 = 第二配偶者
role-subject = 当事者
role-participant = 参加者
role-witness = 証人
role-officiant = 司式者
role-informant = 届出人
role-godparent = 代父母
