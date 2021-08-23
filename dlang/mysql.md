# MySQL

## 戦略

- デファクトのVibe.dを使う前提で、I/Oモデルなどもそれに合わせるような選定になる
- 非同期でガンガン投げる前提だとコネクションプールは欲しい
  - コネクションプールの話にかんしては以下の資料を参照
    - [「コネクションプール都市伝説」はほんとに都市伝説？](https://yamaz.hatenablog.com/entry/20060903)
    - [RDBMSでコネクションプールが必要な理由、わからない。(togetter)](https://togetter.com/li/558788)

## MySQLドライバ

HTTPサーバがvibe.d前提なので、ドライバの選定もそれを踏まえたものになる。

- [mysql-native](https://github.com/mysql-d/mysql-native)
- [mysql-lited](https://github.com/eBookingServices/mysql-lited)

どちらもVibeSocket前提かつコネクションプールを自前で持つ実装になっている。
mysql-nativeのほうがメンテナンスは活発なのでこちらを軸に考える。
単に非同期I/Oなだけでなく、EAGAINとかのときにFiberで実行主体が切り替わるのがミソなので、コアがvibe.dのリアクターでないと意味がない。

## mysql-native

調査している時点でのバージョンは 3.0.2。(2021/07/25時点)

- 簡単なコード例

```
import std.algorithm : map;
import std.array : array;
import mysql;

struct Payment
{
    int customerId;
    int amount;
    string accountName;
}

void main()
{
    auto conn = new Connection("host=127.0.0.1;port=3307;user=testuser;pwd=testpassword;db=testdb");
    scope (exit) conn.close;

    conn.exec(`CREATE TEMPORARY TABLE payment (
      customer_id int not null,
      amount int not null,
      account_name text
    ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4`);

    Payment[] payments = [
        { customerId: 1, amount: 2, accountName: null },
        { customerId: 3, amount: 4, accountName: "foo" },
        { customerId: 5, amount: 6, accountName: null },
        { customerId: 7, amount: 8, accountName: null },
        { customerId: 9, amount: 10, accountName: "bar" }
        ];

    foreach (payment; payments)
    {
        conn.exec(`INSERT INTO payment (customer_id, amount, account_name)
          VALUES (?, ?, ?)
        `, payment.customerId, payment.amount, payment.accountName);
    }

    static Payment queryMapping(Row row)
    {
        Payment payment;
        row.toStruct(payment);
        return payment;
    }
    const selectedPayments = conn.query(`SELECT customer_id, amount, account_name FROM payment`)
        .map!queryMapping
        .array;
    assert(payments == selectedPayments);
}
```

- vibe.dのソケットとPhobosのソケットの両方に対応しており、デフォルトでvibe.dを使っているプロジェクトを判別してvibe.dのソケットを使うようになっている
  - `"versions": ["Have_vibe_d_core"]` で強制的に指定することもできる
- prepared statementを使える
  - SQLインジェクション対策
  - prepared statementはクエリのパースを毎回行う必要がないのでパフォーマンス向上が期待できる
  - コネクションプールとの併用はmysql-native側でうまいことやってくれてる
    - コネクション側でハンドルを管理してよしなにregister/releaseしてる
    - プールを使った場合コネクション毎に準備済みステートメントを管理するのではなく、全コネクションで同じものを登録/解除することになる
      - 最大限効率的というわけではないが使いやすくはある
- 自前でコネクションプールをもっている
  - といってもコネクションプールの部分はvibe.dのジェネリックな実装の上になってる
  - 基本的にはここから `lockConnection` でDBサーバと通信を行う
    - 再接続などもよしなにやってくれるので基本これを使うべき
  - 最大同時接続数は `maxConcurrency` で変更可能
    - デフォルトではuint.max
- MySQL 8.0の新しいユーザ認証に対応していない
  - MySQL 8.0ではデフォルトのユーザ認証プラグインがCachingSha2Passwordになったがこれに対応していない
  - 仮にMySQL 8で出題された場合my.cnfの設定ファイルを修正する必要がある
- SSL/TLS対応していない
  - 問題で使われていたらmy.cnfの編集が必要になる
- そもそも認証が`mysql_native_password`にハードコードされている
  - 設定で認証を外すこともできない
- 255バイトを超える認証レスポンスが使えない
- `CLIENT_SESSION_TRACK`に対応していない
  - `SERVER_SESSION_STATE_CHANGED`をとる方法がない
- 圧縮をサポートしていない
  - `zlib`/`zstd`ともにサポートしていない
- `LOAD LOCAL INFILE`構文をサポートしていない
