# MySQL

## 戦略

- デファクトのVibe.dを使う前提で、I/Oモデルなどもそれに合わせるような選定になる
- 非同期でガンガン投げる前提だとコネクションプールは欲しい
  - コネクションプールの話にかんしては以下の資料を参照
    - [「コネクションプール都市伝説」はほんとに都市伝説？](https://yamaz.hatenablog.com/entry/20060903)
    - [RDBMSでコネクションプールが必要な理由、わからない。(togetter)](https://togetter.com/li/558788)

## MySQLドライバ

vibe.d前提なので、ドライバの選定もそれを踏まえたものになる。

- [mysql-native](https://github.com/mysql-d/mysql-native)
- [mysql-lited](https://github.com/eBookingServices/mysql-lited)

どちらもVibeSocket前提かつコネクションプールを自前で持つ実装になってるが、mysql-litedのほうがAPI的に使いやすい感(個人の感想です)。
単に非同期I/Oなだけでなく、EAGAINとかのときにFiberで実行主体が切り替わるのがミソなので、コアがvibe.dのリアクターでないと意味がない。

## mysql-native

調査している時点での最新版は 2.2.2。

- prepared statementは使える
- Vibe.dのソケットとPhobosのソケットの両方に対応している、デフォルトだとVibe.dのソケットを使う
- 自前でコネクションプールをもっている
  - 基本的にはここから `lockConnection` でDBサーバと通信を行う
    - 再接続などもよしなにやってくれる
  - 最大同時接続数は `maxCurrency` で変更可能
- selectクエリの結果は `ResultRange` というrangeが返り、 `array` で `Row` の配列に変換できる
  - ResultRange / Row ともにデフォルトだとVariantとして扱う必要がある
  - Row は `toStruct` で構造体に変換できる
