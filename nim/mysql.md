# MySQL

標準ライブラリにlibmysqlclientのbindingがある。asynchttpserverがデファクトなんでどうなんだろ。。みたいなところがあるけど。

非同期I/Oなやつはあるけど、どれも完成度はまだまだというかんじの出来。というよりメンテされていなさそうなのもある。

## [asyncmysql](https://github.com/tulayang/asyncmysql)

調査時点でのバージョンは 0.4.3。結論からいうとNimで参戦する場合で非同期なドライバを使うならこれ一択。
そうじゃない場合はlibmysqlclientとスレッドプールでI/Oでブロックする部分を逃してやる戦略をとるくらい。

- 名前のとおり非同期前提
- 複数SQL文を1クエリ発行する実装をもってる
- large result setsをストリームで受け取るのもできる
- コネクションプールを持ってる
  - prepared statementとの組み合わせとかは調べてない

## [mysqlclient.nim](https://github.com/euantorano/mysqlclient.nim)

メンテされてなさそうだし機能も少ない

##　[nim-asyncmysql](https://github.com/wiml/nim-asyncmysql)

もう今のNimだと動かないだろう
