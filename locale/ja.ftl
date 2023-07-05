# 一般ラベル
not-linked = 引数が不足しています
not-linked-description = リンクされていないため、プロフィールを表示するにはプロフィールを提供する必要があります。
showing-statistics = <a:clock:1110754973794451557> {$from}から{$to}までの統計を表示しています。
no-previous-statistics = **{$name}**の以前のデータが見つからなかったため、挿入されました。
showing-guild-statistics = {$from}から{$to}までのギルド統計を表示しています。
no-previous-guild-statistics = ギルド **{$name}** の以前のデータが見つからなかったため、挿入されました。

# Errors

error-player-not-found = {$name}のプロフィールが見つかりませんでした。
error-session-not-found = {$name}のセッションが見つかりませんでした。
error-player-uuid-not-found = uuid {$uuid}のプレイヤーが見つかりませんでした。
error-player-username-not-found = ユーザー名 {$name}のプレイヤーが見つかりませんでした。
error-guild-by-member-uuid-not-found = uuid {$uuid}のギルドメンバーが見つかりませんでした。
error-guild-by-member-username-not-found = ユーザー名 {$name}のギルドメンバーが見つかりませんでした。
error-guild-not-found = {$name}のギルドが見つかりませんでした。
error-internal = 内部エラーが発生しました。ログに記録され、すぐに解決されます。
error-not-linked = Minecraftアカウントにリンクされていません。 </link:1113624864524357710>を使用してアカウントをリンクしてください。
error-invalid-uuid = uuid {$uuid}は無効です。
error-invalid-username = ユーザー名 {$name}は無効です。
error-member-player-not-found = メンバー{$name}はプロフィールに見つかりませんでした。
error-skyblock-profile-not-found = {$name}のSkyBlockプロファイルが見つかりませんでした。
error-player-snapshot-not-found = {$name}のスナップショットが見つかりませんでした。 </daily bedwars:1113624864272683065>で作成してください。
error-leaderboard-not-found = 名前{$name}のリーダーボードが見つかりませんでした。
error-profile-not-found = {$name}の{$profile} SkyBlockプロファイルは、APIが無効になっているか、新しいプロファイルシステム以降ログインしていません。
error-identifier-too-long =
  このインタラクションの生成された識別子の1つが長すぎます。
  コンポーネントの数またはカスタムテキストの量を減らしてみてください。

# Quick tips

tip-background = <:knowledge_book:1117179094556233828> 生成された画像の背景を </background:1117174166056075335> で変更します。
tip-history = <:knowledge_book:1117179094556233828> </history bedwars:1113624864524357708>で統計の履歴グラフを表示します。
tip-from = <:knowledge_book:1117179094556233828> </from bedwars:1113624864524357705>で特定の日付からの統計変更を表示します。
tip-leaderboard = <:knowledge_book:1117179094556233828> </leaderboard:1113624864524357709>で多くのリーダーボードを表示します。
tip-skyblock = <:knowledge_book:1117179094556233828> SkyBlockをサポートしています！ </skyblock profile:1113624864826327074>でプロファイルを表示します。
tip-link = <:knowledge_book:1117179094556233828> </link:1113624864524357710>でMinecraftアカウントをリンクします。
tip-guild = <:knowledge_book:1117179094556233828> </guild general:1113624864524357706>でギルド統計を表示します。
tip-snapshot = <:knowledge_book:1117179094556233828> </daily bedwars:1113624864272683065>で毎日の統計にアクセスします。
tip-display = <:knowledge_book:1117179094556233828> </display:1113624864272683066>で表示形式を変更します。
tip-help = <:knowledge_book:1117179094556233828> </help:1113624864524357707>でより詳しいヘルプを取得します。
tip-website = <:knowledge_book:1117179094556233828> <https://statpixel.xyz>でウェブサイトを訪問します。
tip-support-discord = <:knowledge_book:1117179094556233828> <https://statpixel.xyz/discord>でサポートサーバーに参加します。
tip-project = <:knowledge_book:1117179094556233828> </project bedwars:1118417616541843477>ですべてのデータの推定値を表示します。
tip-winstreak = <:knowledge_book:1117179094556233828> </winstreaks:1124767485384724520>で勝利ストリークを表示します。
tip-recent = <:knowledge_book:1117179094556233828> </recent:1123839349428080690>で最近のゲームを表示します。
tip-bazaar = <:knowledge_book:1117179094556233828> </skyblock bazaar:1113624864826327074>でSkyBlockバザー価格を表示します。

# /custom

execute = execute
  .description = 識別子によってコマンドを実行します
  .id = id
  .id-description = コマンドの識別子

invalid-identifier = 無効な識別子が提供されました
invalid-identifier-description = 提供された識別子は無効です。 StatPixelからこの識別子が与えられた場合は、識別スキームが更新されているため、新しい識別子を取得する必要があります。
invalid-identifier-command-description = 提供された識別子はコマンドではありません。

deprecated-interaction = インタラクションが廃止されました
deprecated-interaction-description = このインタラクションは廃止され、機能しません。 元のコマンドをもう一度実行してください。
identifier = <:id:1125971775755407390> 識別子: `{$identifier}`

# /builder

builder = builder
  .description = 新しいカスタム画像ビルダーを作成します

builder-welcome =
  StatPixel画像ビルダーへようこそ。

  最初のコンポーネントを追加するには、下のボタンをクリックして、完了したら作成ボタンを使用して最終的に作成します。
  何かを間違えた場合は、元に戻すを使用して元に戻します。 Redoボタンは現在ありませんので、注意してください！

  作成されたら、提供された識別子を使用して、いつでも更新された統計でその画像を表示し、友達と共有します！
  また、画像APIと共に使用して、フォーラム署名やインターネット上の他の場所に表示することもできます。

documentation = ドキュメンテーション
down = 下
down-description = 直前の形状の下に形状を追加します。
down-start = 下（開始）
down-start-description = 直前のものの下に形状を追加しますが、すべて左にあります。
right = 右
right-description = 直前の形状の右に形状を追加します。
right-start = 右（開始）
right-start-description = 直前の形状の右に形状を追加しますが、すべて上にあります。
select-position = 形状の位置を選択します

title = タイトル
title-description = ユーザー名を表示します。
level-description = 任意のゲームのレベルを表示します。
skin = スキン
skin-description = プレイヤーのスキンの画像。
bubble = バブル
bubble-description = 任意のゲームの任意の統計を表示するボックス。
subtitle = サブタイトル
subtitle-description = サブタイトルは、最大16文字までの任意のテキストを表示するために使用されます。
select-shape = 形状タイプを選択します
select-colour = 色を選択します

add-shape = 形状を追加
undo = 元に戻す
create = 作成

subtitle-modal-title = 新しいサブタイトル形状を作成する
subtitle-text = サブタイトルテキスト
subtitle-placeholder = 最大16文字を入力してください

level-modal-title = 新しいレベル形状を作成する
level-type = レベルタイプ
level-type-placeholder = bedwars、buildbattle、duels、network、pit、skywars、woolwarsのいずれか

bubble-modal-title = 新しいバブル形状を作成する
game-type = ゲームタイプ
game-type-placeholder = ドキュメントを参照：https://statpixel.xyz/docs/builder
statistic = 統計
statistic-placeholder = ドキュメントを参照：https://statpixel.xyz/docs/builder

create-modal-title = 画像の作成を完了する
username = ユーザー名
username-placeholder = 使用するユーザー名を入力してください

invalid-statistic =
  {$game}の無効な統計{$statistic}。 ドキュメントを確認して、もう一度お試しください。

invalid-level-type =
  無効なレベルタイプ{$kind}。 ドキュメントを確認して、もう一度お試しください。

invalid-game-type =
  無効なゲームタイプ{$game}。 ドキュメントを確認して、もう一度お試しください。

image-created =
  あなたの画像が作成されました！ </execute:1125992506501365892>で試してみてください。

  識別子: {$id}
  リンク: {$link}

# /about

author = 作者 🤖
guilds = ギルド 🏰
profiles = プロファイル 🤺
users = ユーザー 🤸
snapshots = スナップショット 📒

about-description =
  StatPixelは、Wool WarsやSkyBlockを含むHypixelネットワークのすべてのゲームをサポートしています。詳細については、</help:1113624864524357707>を使用するか、<https://statpixel.xyz/docs/commands>のドキュメントを参照してください。

about = about
  .description = ボットに関する情報を表示します

# /winstreaks

winstreaks = winstreaks
  .description = 勝利ストリークを表示します
  .username = username
  .username-description = Minecraftユーザー名
  .uuid = uuid
  .uuid-description = Minecraft UUID

# /recent

mode = モード
map = マップ
started = 開始
duration = 継続時間
playing = アクティブ

recent = recent
  .description = 最近のゲームを表示します
  .username = username
  .username-description = Minecraftユーザー名
  .uuid = uuid
  .uuid-description = Minecraft UUID

# /project

accuracy = 精度
estimate = 推定
never = 決して

# /background

background = background
  .description = 生成された画像の背景を変更します。
  .colour = 色
  .colour-description = 背景の色

error-invalid-colour = 無効な色が指定されました
error-invalid-colour-description = 指定された色は無効です。 [これらの色](https://simple.wikipedia.org/wiki/List_of_colors) のいずれかを提供するか、 `＃`で始まる16進色を提供してみてください。
colour-changed = 背景色が変更されました
colour-changed-description = 背景色が{$colour}に変更されました。

# /skyblock bazaar

buy-price = 購入価格
sell-price = 販売価格
last-hour = 最後の1時間
last-day = 最後の1日
last-week = 最後の1週間

bazaar = bazaar
  .description = SkyBlockバザーの価格を表示します
  .product = product
  .product-description = 表示する製品

# /skyblock auctions

Auctions = オークション
Bank = 銀行
Candy = キャンディー
EnderChest = エンダーチェスト
Equipment = 装備
Fishing = 釣り
Inventory = インベントリ
Networth = 総資産
Pets = ペット
Potions = ポーション
Profile = プロファイル
Quiver = 矢筒
Talisman = タリスマン
Vault = 金庫
Wardrobe = ワードローブ

auctions = auctions
  .description = SkyBlockオークションを表示します。
  .username = username
  .username-description = Minecraftのユーザー名
  .uuid = uuid
  .uuid-description = MinecraftのUUID

player-auctions = Auctions
highest-bid = 最高入札

# /skyblock inventory

inventory = inventory
  .description = SkyBlockインベントリを表示します。
  .username = username
  .username-description = Minecraftのユーザー名
  .uuid = uuid
  .uuid-description = MinecraftのUUID
  .profile = profile
  .profile-description = 表示するプロファイル

enderchest = enderchest
  .description = SkyBlockエンダーチェストを表示します。
  .username = username
  .username-description = Minecraftのユーザー名
  .uuid = uuid
  .uuid-description = MinecraftのUUID
  .profile = profile
  .profile-description = 表示するプロファイル

quiver = quiver
  .description = SkyBlockクイバーを表示します。
  .username = username
  .username-description = Minecraftのユーザー名
  .uuid = uuid
  .uuid-description = MinecraftのUUID
  .profile = profile
  .profile-description = 表示するプロファイル

talisman = talisman
  .description = SkyBlockタリスマンバッグを表示します。
  .username = username
  .username-description = Minecraftのユーザー名
  .uuid = uuid
  .uuid-description = MinecraftのUUID
  .profile = profile
  .profile-description = 表示するプロファイル

fishing = fishing
  .description = SkyBlock釣りバッグを表示します。
  .username = username
  .username-description = Minecraftのユーザー名
  .uuid = uuid
  .uuid-description = MinecraftのUUID
  .profile = profile
  .profile-description = 表示するプロファイル

potions = potions
  .description = SkyBlockポーションバッグを表示します。
  .username = username
  .username-description = Minecraftのユーザー名
  .uuid = uuid
  .uuid-description = MinecraftのUUID
  .profile = profile
  .profile-description = 表示するプロファイル

equipment = equipment
  .description = SkyBlock装備を表示します。
  .username = username
  .username-description = Minecraftのユーザー名
  .uuid = uuid
  .uuid-description = MinecraftのUUID
  .profile = profile
  .profile-description = 表示するプロファイル

wardrobe = wardrobe
  .description = SkyBlockワードローブを表示します。
  .username = username
  .username-description = Minecraftのユーザー名
  .uuid = uuid
  .uuid-description = MinecraftのUUID
  .profile = profile
  .profile-description = 表示するプロファイル

candy = candy
  .description = SkyBlockキャンディーインベントリを表示します。
  .username = username
  .username-description = Minecraftのユーザー名
  .uuid = uuid
  .uuid-description = MinecraftのUUID
  .profile = profile
  .profile-description = 表示するプロファイル

vault = vault
  .description = SkyBlock個人用金庫を表示します。
  .username = username
  .username-description = Minecraftのユーザー名
  .uuid = uuid
  .uuid-description = MinecraftのUUID
  .profile = profile
  .profile-description = 表示するプロファイル

pets = pets
  .description = SkyBlockペットを表示します。
  .username = username
  .username-description = Minecraftのユーザー名
  .uuid = uuid
  .uuid-description = MinecraftのUUID
  .profile = profile
  .profile-description = 表示するプロファイル

networth = networth
  .description = SkyBlockプロファイルの純資産を表示します。
  .username = username
  .username-description = Minecraftのユーザー名
  .uuid = uuid
  .uuid-description = MinecraftのUUID
  .profile = profile
  .profile-description = 表示するプロファイル

# /skyblock profile

profile = profile
  .description = SkyBlockプロファイルを表示します。
  .username = username
  .username-description = Minecraftのユーザー名
  .uuid = uuid
  .uuid-description = MinecraftのUUID
  .profile = profile
  .profile-description = 表示するプロファイル

member-profile = メンバープロファイル
fairy-souls = 妖精の魂
fairy-exchanges = 交換
fishing-treasure = 宝物
zones-visited = ゾーン
generators-crafted = ジェネレーター
highest-crit = 最高クリティカル

farming = 農業
mining = 採掘
combat = 戦闘
foraging = 採取
fishing-skill = 釣り
enchanting = エンチャント
alchemy = 錬金術
taming = 飼いならし
dungeoneering = ダンジョン
carpentry = 大工
runecrafting = ルーン
social = ソーシャル

# /skyblock bank

bank = bank
  .description = SkyBlock銀行を表示します。
  .username = username
  .username-description = Minecraftのユーザー名
  .uuid = uuid
  .uuid-description = MinecraftのUUID
  .profile = profile
  .profile-description = 表示するプロファイル

island-bank-balance = アイランド銀行残高
bank-balance = 銀行残高

# /leaderboard

leaderboard = leaderboard
  .description = さまざまなゲームのリーダーボードを表示します。
  .board = board
  .board-description = 表示するリーダーボード

# /network

network = network
  .description = ネットワーク
  .username = username
  .username-description = Minecraftのユーザー名
  .uuid = uuid
  .uuid-description = MinecraftのUUID
  .hours = hours
  .hours-description = 時間
  .days = days
  .days-description = 日
  .weeks = weeks
  .weeks-description = 週

karma = カルマ
rewards = 報酬
friend-requests = フレンドリクエスト
first-login = 最初のログイン
last-login = 最後のログイン
quests = クエスト
challenges = チャレンジ
achievement-points = 実績ポイント
language = 言語
gifts-given = 贈り物
ranks-given = ランク

# /help

help = help
  .description = ヘルプメニューを表示します。

help-general = 一般 <:cookie:1115091335565811775>
help-general-description = StatPixelはHypixelネットワークのすべてのゲームをサポートしています。 `/<game>`を使用して各ゲームの統計を表示できます。 たとえば、</bedwars:1113624864272683060>を試してみてください！

help-display = 表示 <:spyglass:1115091333657411625>
help-display-description = メーター付き接続を使用しているか、データを保存したい場合は、</display:1113624864524357705>などのテキストや縮小画像など、さまざまな形式で応答を受信できます。

help-link = リンク <a:recovery_compass:1115091332680126504>
help-link-description = 各コマンドのユーザー名を入力するのは面倒になる場合があります。これを簡単にするには、</link:1113624864524357710>で確認なしでアカウントをリンクし、後で</unlink:1113624865262538854>でリンクを解除できます。

help-snapshot = スナップショット <:book_and_quill:1110754805724479569>
help-snapshot-description = スナップショットは、統計の変更を時間の経過とともに表示する方法です。いくつかのユーティリティコマンドを使用して、生活をより簡単にすることができます。</daily bedwars:1113624864272683065>、</weekly bedwars:1113624865262538858>、</monthly bedwars:1113624864524357712>。これらはギルドにも適用されるため、</daily guild:1113624864272683065>を試してみてください！
  
  より具体的な範囲を表示するには、</from bedwars:1113624864524357705>を試してみてください。

help-history = 履歴 <a:clock:1115091329958019253>
help-history-description = 特定の統計が時間の経過とともにどのように変化したかをグラフで表示するには、</history network:1113624864524357708>などを試してみてください。

help-image-builder = イメージビルダー <:gold_pickaxe:1125980780435345488>
help-image-builder-description = イメージビルダーを使用して、独自の動的イメージを作成します！ </builder:1125992506501365891>を使用して開始するか、https://statpixel.xyz/docs/builder で詳細を確認してください。

# /history

statistics-history = {" "}の歴史{" "}

# /display

display = display
  .description = 応答の表示方法を変更します。
  .format = format
  .format-description = 使用する応答フォーマット

display-changed = 表示が変更されました
display-changed-text-description = 応答は今後テキストとして送信されます。
display-changed-image-description = 応答は適用される場合には画像として送信されます。
display-changed-compact-description = 応答は今後コンパクトとして送信されます。

Image = 画像
Compact = コンパクト
Text = テキスト

# /unlink
unlink = unlink
  .description = DiscordアカウントとMinecraftアカウントのリンクを解除します。

unlinking-failed = リンク解除に失敗しました
unlinking-failed-description = Minecraftアカウントにリンクされていません。
unlinking-succeeded = リンク解除に成功しました
unlinking-succeeded-description = Minecraftアカウントとのリンクが解除されました。

# /link
link = link
  .description = DiscordアカウントをMinecraftアカウントにリンクします。
  .username = username
  .username-description = リンクするMinecraftのユーザー名
  .uuid = uuid
  .uuid-description = リンクするMinecraftのUUID

linking-failed = リンクに失敗しました
linking-failed-uuid-description = UUID `{$uuid}`はMinecraftアカウントに属していません。
linking-failed-username-description = ユーザー名 **{$username}** はMinecraftアカウントに属していません。
linking-failed-description = 有効なUUIDまたはユーザー名を指定する必要があります。
linking-succeeded = リンクに成功しました
linking-succeeded-description = DiscordアカウントがMinecraftアカウント **{$name}** にリンクされました。

# /arcade

Party = パーティー
SantaSays = サンタが言う
SimonSays = サイモンが言う
MiniWalls = ミニウォールズ
Soccer = サッカー
OneInTheQuiver = 弓キル競争
EnderSpleef = エンダースプリーフ
FarmHunt = ファームハント
DragonWars = ドラゴンウォーズ
BlockingDead = 死霊の夜
Zombies = ゾンビ撃退
ZombiesBadBlood = ゾンビ：バッドブラッド
ZombiesDeadEnd = ゾンビ：デッドエンド
PixelPainters = ピクセルペインター
HoleInTheWall = ホールインザウォール
ThrowOut = スローアウト
EasterSimulator = イースターシミュレーター
ScubaSimulator = スキューバシミュレーター
HalloweenSimulator = ハロウィンシミュレーター
GrinchSimulator = グリンチシミュレーター

mystery-gifts = 謎の贈り物

arcade = arcade
  .description = プレイヤーのアーケードスタッツを表示します。
  .username = username
  .username-description = 表示するマインクラフトのユーザー名
  .uuid = uuid
  .uuid-description = 表示するマインクラフトのUUID
  .mode = mode
  .mode-description = 表示するアーケードモード

# /arena

magical-keys = 魔法の鍵
magical-chests = 魔法のチェスト
rating = レーティング

arena = arena
  .description = プレイヤーのアリーナブロールスタッツを表示します。
  .username = username
  .username-description = 表示するマインクラフトのユーザー名
  .uuid = uuid
  .uuid-description = 表示するマインクラフトのUUID
  .mode = mode
  .mode-description = 表示するアリーナブロールモード

# /bedwars

Solo = ソロ
Double = ダブル
Three = スリー
Four = フォー
SoloRush = ソロラッシュ
DoubleRush = ダブルラッシュ
FourRush = フォーラッシュ
SoloUltimate = ソロアルティメット
DoubleUltimate = ダブルアルティメット
FourUltimate = フォーアルティメット
Castle = キャッスル
DoubleLucky = ダブルラッキー
FourLucky = フォーラッキー
DoubleVoidless = ダブルボイドレス
FourVoidless = フォーボイドレス
DoubleArmed = ダブルアームド
FourArmed = フォーアームド
DoubleUnderworld = ダブルアンダーワールド
FourUnderworld = フォーアンダーワールド
DoubleSwap = ダブルスワップ
FourSwap = フォースワップ

final-kills = 最後のキル
final-deaths = 最後のデス
fkdr = FKDR
beds-broken = 破壊したベッド
beds-lost = 失ったベッド
bblr = BBLR

iron-collected = 鉄
gold-collected = 金
diamond-collected = ダイヤモンド
emerald-collected = エメラルド
items-purchased = 購入品

bedwars = bedwars
  .description = プレイヤーのベッドウォーズスタッツを表示します。
  .username = username
  .username-description = 表示するマインクラフトのユーザー名
  .uuid = uuid
  .uuid-description = 表示するマインクラフトのUUID
  .mode = mode
  .mode-description = 表示するベッドウォーズモード

# /blitz

Armorer = 防具職人
Scout = スカウト
Speleologist = 洞窟学者
Random = ランダム
Rogue = ローグ
Rambo = ランボー
Troll = トロール
HorseTamer = 馬使い
WolfTamer = 狼使い
Warrior = 戦士
Phoenix = フェニックス
DonkeyTamer = ラバ使い
Ranger = レンジャー
Archer = アーチャー
Necromancer = ネクロマンサー
Meatmaster = ミートマスター
Tim = ティム
Pigman = ピグマン
CreeperTamer = クリーパータマー
Florist = フローリスト
Warlock = ウォーロック
Milkman = ミルクマン
Astronaut = アストロノート
Blaze = ブレイズ

potions-drunk = 薬品飲酒回数
chests-opened = チェスト開封回数
time-played = プレイ時間

blitz = blitz
  .description = プレイヤーの Blitz Survival Games の統計情報を表示します。
  .username = username
  .username-description = 表示する Minecraft ユーザー名
  .uuid = uuid
  .uuid-description = 表示する Minecraft UUID
  .mode = mode
  .mode-description = 表示する Blitz Survival Games のモード

# /buildbattle

SoloPro = ソロプロ
GuessTheBuild = ビルドを予想する

votes = 投票数
most-points-solo = 最多ポイント (個人)
most-points-team = 最多ポイント (チーム)

buildbattle = buildbattle
  .description = プレイヤーの Build Battle の統計情報を表示します。
  .username = username
  .username-description = 表示する Minecraft ユーザー名
  .uuid = uuid
  .uuid-description = 表示する Minecraft UUID
  .mode = mode
  .mode-description = 表示する Build Battle のモード

# /copsandcrims

Defusal = Defusal
GunGame = Gun Game
Deathmatch = Deathmatch

cop-kills = 警官キル数
criminal-kills = 犯罪者キル数
headshot-kills = ヘッドショットキル数
grenade-kills = 手榴弾キル数
bombs-defused = 爆弾解除数
bombs-planted = 爆弾設置数
copsandcrims = copsandcrims
  .description = プレイヤーのCops and Crims統計を表示します。
  .username = username
  .username-description = 表示するMinecraftのユーザー名
  .uuid = uuid
  .uuid-description = 表示するMinecraftのUUID
  .mode = mode
  .mode-description = 表示するCops and Crimsモード

# /duels

UhcSolo = UHC デュエル
OpSolo = OP デュエル
UhcDouble = UHC ダブルス
BowSolo = 弓デュエル
ClassicSolo = クラシックデュエル
OpDouble = OP ダブルス
UhcFour = UHC 4v4
SkyWarsDouble = Sky Wars ダブルス
SumoSolo = 相撲デュエル
SkyWarsSolo = Sky Wars デュエル
BridgeDoubleDuel = Bridge 2v2
BridgeFourDuel = Bridge 4v4
BridgeSolo = Bridge デュエル
BridgeThree = Bridge Threes
BridgeDouble = Bridge ダブルス
ComboSolo = コンボデュエル
SumoTournament = 相撲トーナメント
SkyWarsTournament = Sky Wars トーナメント
UhcMeetup = UHC ミートアップ
PotionSolo = ポーションデュエル
BlitzSolo = ブリッツデュエル
BowSpleefSolo = 弓スプリーフデュエル
MegaWallsSolo = メガウォールズデュエル
BoxingSolo = ボクシングデュエル
Parkour = パルクール
ArenaSolo = アリーナデュエル
CaptureThree = キャプチャースリー
BridgeThreeDuel = Bridge 3v3

melee-accuracy = 近接攻撃精度
health-regenerated = 回復した体力

duels = duels
  .description = プレイヤーのDuels統計を表示します。
  .username = username
  .username-description = 表示するMinecraftのユーザー名
  .uuid = uuid
  .uuid-description = 表示するMinecraftのUUID
  .mode = mode
  .mode-description = 表示するDuelsモード

# /megawalls

FaceOff = Face Off

distance-walked = 移動距離
distance-fallen = 落下距離
bread-eaten = 食べたパンの数
wood-chopped = 木を切った回数
treasures-found = 発見した宝物の数

megawalls = megawalls
  .description = プレイヤーのメガウォールのステータスを表示します。
  .username = username
  .username-description = 表示するマインクラフトのユーザー名
  .uuid = uuid
  .uuid-description = 表示するマインクラフトのUUID
  .mode = mode
  .mode-description = 表示するメガウォールのモード

# /murdermystery

Assassins = 暗殺者
Classic = クラシック
DoubleUp = ダブルアップ
Infection = インフェクション

time-survived = 生存時間
murderer-wins = 殺人鬼の勝利
detective-wins = 探偵の勝利

murdermystery = murdermystery
  .description = プレイヤーの殺人ミステリーのステータスを表示します。
  .username = username
  .username-description = 表示するマインクラフトのユーザー名
  .uuid = uuid
  .uuid-description = 表示するマインクラフトのUUID
  .mode = mode
  .mode-description = 表示する殺人ミステリーのモード

# /paintball

adrenaline = アドレナリン
endurance = エンデュランス
fortune = フォーチュン
godfather = ゴッドファーザー
superluck = スーパーラック
transfusion = 輸血
kill-prefix = キルプレフィックス
show-kill-prefix = キルプレフィックスの表示

shots-fired = 打った弾数
killstreaks = キルストリーク
forcefield-time = フォースフィールド時間
chat-messages = チャットメッセージ数
soups-drank = 飲んだスープの数
cash-earned = 獲得した現金
highest-killstreak = 最高連続キル数

paintball = paintball
  .description = プレイヤーのペイントボールのステータスを表示します。
  .username = username
  .username-description = 表示するマインクラフトのユーザー名
  .uuid = uuid
  .uuid-description = 表示するマインクラフトのUUID
  .mode = mode
  .mode-description = 表示するペイントボールのモード

# /pit

cash = 現金
bow-damage-dealt = 弓のダメージ量
bow-damage-taken = 弓によるダメージを受けた回数
bdr = BDR
contracts-completed = 完了した契約の数
contracts-started = 開始した契約の数
cr = 完了率

pit = pit
  .description = プレイヤーのThe Pit統計を表示します。
  .username = username
  .username-description = 表示するMinecraftのユーザー名
  .uuid = uuid
  .uuid-description = 表示するMinecraftのUUID
  .mode = mode
  .mode-description = 表示するThe Pitモード

# /quake

SoloTournament = ソロトーナメント

hr = ヘッドショット率
headshots = ヘッドショット数
sight = サイト

quake = quake
  .description = プレイヤーのQuakecraft統計を表示します。
  .username = username
  .username-description = 表示するMinecraftのユーザー名
  .uuid = uuid
  .uuid-description = 表示するMinecraftのUUID
  .mode = mode
  .mode-description = 表示するQuakecraftモード

# /skywars
Overall = 全体
SoloNormal = ソロ通常
SoloInsane = ソロインセイン
TeamNormal = チーム通常
TeamInsane = チームインセイン
MegaNormal = メガ通常
MegaDouble = メガダブル
Ranked = ランク戦
SoloLab = ソロラボ
TeamLab = チームラボ
Tourney = トーナメント

opals = オパール
heads = ヘッド
souls = ソウル
tokens = トークン
bow-accuracy = 弓の精度
eggs-thrown = 投げた卵の数
fastest-win = 最速勝利

skywars = skywars
  .description = プレイヤーのSkyWars統計を表示します。
  .username = username
  .username-description = 表示するMinecraftのユーザー名
  .uuid = uuid
  .uuid-description = 表示するMinecraftのUUID
  .mode = mode
  .mode-description = 表示するSkyWarsモード

# /smash

smasher = スマッシャー
smashed = スマッシュされた回数
ssr = SSR

smash = smash
  .description = プレイヤーのSmash Heroes統計を表示します。
  .username = username
  .username-description = 表示するMinecraftのユーザー名
  .uuid = uuid
  .uuid-description = 表示するMinecraftのUUID
  .mode = mode
  .mode-description = 表示するSmash Heroesモード

# /speeduhc

tears = 涙の数
survived-players = 生き残ったプレイヤー数

speeduhc = speeduhc
  .description = プレイヤーのSpeedUHC統計を表示します。
  .username = username
  .username-description = 表示するMinecraftのユーザー名
  .uuid = uuid
  .uuid-description = 表示するMinecraftのUUID
  .mode = mode
  .mode-description = 表示するSpeedUHCモード

# /tntgames

TntRun = TNT Run
TntTag = TNT Tag
PvpRun = PvP Run
BowSpleef = Bow Spleef
Wizard = Wizards

record = 記録
double-jumps = 二段ジャンプ
tags = タグ
air-time = 空中時間
points = ポイント

tntgames = tntgames
  .description = プレイヤーのTNT Gamesの統計情報を表示します。
  .username = username
  .username-description = 表示するMinecraftのユーザー名
  .uuid = uuid
  .uuid-description = 表示するMinecraftのUUID
  .mode = mode
  .mode-description = 表示するTNT Gamesのモード

# /turbokartracers

box-pickups = 箱の回収数
coin-pickups = コインの回収数
grand-prix = グランプリ
show-prefix = プレフィックスを表示する
bronze-trophies = ブロンズトロフィー
silver-trophies = シルバートロフィー
gold-trophies = ゴールドトロフィー

turbokartracers = turbokartracers
  .description = プレイヤーのTurbo Kart Racersの統計情報を表示します。
  .username = username
  .username-description = 表示するMinecraftのユーザー名
  .uuid = uuid
  .uuid-description = 表示するMinecraftのUUID
  .mode = mode
  .mode-description = 表示するTurbo Kart Racersのモード

# /uhc

RedVsBlue = 赤 vs 青
NoDiamonds = ダイヤモンド禁止
VanillaDouble = Vanilla Double
Brawl = Brawl
SoloBrawl = ソロBrawl
DoubleBrawl = ダブルBrawl

heads-eaten = 食べた頭の数
ultimates-crafted = クラフトした究極のアイテム数

uhc = uhc
  .description = プレイヤーのUHC Championsの統計情報を表示します。
  .username = username
  .username-description = 表示するMinecraftのユーザー名
  .uuid = uuid
  .uuid-description = 表示するMinecraftのUUID
  .mode = mode
  .mode-description = 表示するUHC Championsのモード

# /vampirez

human-wins = 人間側の勝利数
vampire-wins = ヴァンパイア側の勝利数
zombie-kills = ゾンビキル数
human-kills = 人間キル数
human-deaths = 人間死亡数
vampire-kills = ヴァンパイアキル数
vampire-deaths = ヴァンパイア死亡数
blood = ブラッド
starting-compass = スタートコンパス
starting-gear = スタートギア
tracker = トラッカー
updated = 更新されました
old-vampire = 古いヴァンパイア
hkdr = HKDR
vkdr = VKDR

vampirez = vampirez
  .description = プレイヤーのVampireZの統計を表示します。
  .username = username
  .username-description = 表示するMinecraftのユーザー名
  .uuid = uuid
  .uuid-description = 表示するMinecraftのUUID
  .mode = mode
  .mode-description = 表示するVampireZのモード

# /walls

Standard = 標準

activations = アクティベーション
iron-broken = 鉄の破損

walls = walls
  .description = プレイヤーのThe Wallsの統計を表示します。
  .username = username
  .username-description = 表示するMinecraftのユーザー名
  .uuid = uuid
  .uuid-description = 表示するMinecraftのUUID
  .mode = mode
  .mode-description = 表示するThe Wallsのモード

# /warlords

CaptureTheFlag = Capture the Flag
Domination = Domination
TeamDeathmatch = Team Deathmatch

wins-blue = 勝利（青）
wins-red = 勝利（赤）
hide-prestige = プレステージを隠す
mvps = MVPs

warlords = warlords
  .description = プレイヤーのWarlordsの統計を表示します。
  .username = username
  .username-description = 表示するMinecraftのユーザー名
  .uuid = uuid
  .uuid-description = 表示するMinecraftのUUID
  .mode = mode
  .mode-description = 表示するWarlordsのモード

# /woolwars

layers = レイヤー
powerups-collected = 収集したパワーアップ
wool-placed = 置かれた羊毛

woolwars = woolwars
  .description = プレイヤーのWool Warsの統計を表示します。
  .username = username
  .username-description = 表示するMinecraftのユーザー名
  .uuid = uuid
  .uuid-description = 表示するMinecraftのUUID
  .mode = mode
  .mode-description = 表示するWool Warsのモード

# /guild

daily-xp = デイリーXP
weekly-xp = ウィークリーXP
monthly-xp = マンスリーXP
xp-since = XP以降
members_label = メンバー
date = 日付
weekly-gexp = ウィークリーGEXP
position = ポジション
guild-quests = ギルドクエスト

Member = メンバー
General = 一般
Members = メンバー
Top = トップ

member = member
  .description = ギルドのメンバーを表示します。
  .username = username
  .username-description = 表示するギルドメンバーのMinecraftのユーザー名
  .uuid = uuid
  .uuid-description = 表示するギルドメンバーのMinecraftのUUID

general = general
  .description = ギルドの統計を表示します。
  .name = name
  .name-description = 表示するギルドの名前
  .username = username
  .username-description = 表示するギルドメンバーのMinecraftのユーザー名
  .uuid = uuid
  .uuid-description = 表示するギルドメンバーのMinecraftのUUID

guild = guild
  .description = ギルドの統計を表示します。
  .name = name
  .name-description = 表示するギルドの名前
  .username = username
  .username-description = 表示するギルドメンバーのMinecraftのユーザー名
  .uuid = uuid
  .uuid-description = 表示するギルドメンバーのMinecraftのUUID

members = members
  .description = ギルドのメンバーを表示します。
  .name = name
  .name-description = 表示するギルドの名前
  .username = username
  .username-description = 表示するギルドメンバーのMinecraftのユーザー名
  .uuid = uuid
  .uuid-description = 表示するギルドメンバーのMinecraftのUUID

top = top
  .description = ギルドのXPのトップメンバーを表示します。
  .name = name
  .name-description = 表示するギルドの名前
  .username = username
  .username-description = 表示するギルドメンバーのMinecraftのユーザー名
  .uuid = uuid
  .uuid-description = 表示するギルドメンバーのMinecraftのUUID
  .days = days
  .days-description = days
  .limit = limit
  .limit-description = 表示するメンバーの数

showing-guild-xp-statistics = {$from}から{$to}までのギルドXPの獲得を表示しています。

# 共有キャンバスのラベル

Normal = 通常
Team = チーム

blocks-broken = 破壊したブロック数
blocks-placed = 設置したブロック数

coins = コイン
loot-chests = 宝箱

offline = オフライン
online = オンライン
level = レベル
progress = 進行状況

wins = 勝利
losses = 敗北
wlr = WLR
win-streak = 勝ちストリーク
kills = キル数
deaths = 死亡数
kdr = KDR
assists = アシスト
games-played = プレイしたゲーム数
wr = 勝率
damage-dealt = 与ダメージ
damage-taken = 被ダメージ
ddtr = DDTR
games = ゲーム
score = スコア
created-at = 作成日時
experience = 経験値

yes = はい
no = いいえ
none = なし

# 色

black = 黒
dark-blue = 暗い青
dark-green = 暗い緑
dark-aqua = 暗い水色
dark-red = 暗い赤
dark-purple = 暗い紫
gold = 金色
gray = 灰色
dark-gray = 暗い灰色
blue = 青
green = 緑
aqua = 水色
red = 赤
light-purple = 明るい紫
yellow = 黄色
white = 白
