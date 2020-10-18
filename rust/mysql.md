# MySQL

RustのMySQLドライバはいくつかある

- [rust-mysql-simple](https://github.com/blackbeam/rust-mysql-simple)
- [mysql_async](https://github.com/blackbeam/mysql_async)
- [mysqlclient-sys](https://github.com/sgrif/mysqlclient-sys)

## rust-mysql-simple

調査時点でのバージョンは 20.0.1。

- low-levelなクライアントライブラリ
- ネットワークまわりでいうと、libmysqlclientとかと違ってノンブロッキングI/Oを使うことは選択できそうだが、コアのイベントループとかの連携とか別に考えてやる必要がある
- コネクションプール実装を自前で持つ
  - プール内のコネクションの最大・最小数は生成時に調整可
  - `check_health` というパラメータがあって、デフォルトだとオン
    - これがあると `prep_exec` 接続の度に毎回pingする
    - どうしてもパフォーマンス欲しいときは外す
- コネクションプールを内部的に実装してるので [r2d2](https://github.com/sfackler/r2d2) を使う必要がないけど、実装を見比べてベンチをとったほうがいいかも
  - そもそもr2d2と一緒に使えるのか、とか。r2d2はジェネリックな実装なので使えないことはないと思うけど
  - r2d2-mysqlというのがあって組み合わせられるようになってる
- prepared statementは使える
  - ソースみるかぎりだとコネクションプールでもうまいこと使えそう
  - コネクションごとにstatement cacheを管理してる
    - オフにしたい場合は `use_cache(false)`
- sslサポートとかjsonが使えるなどがあるが、isuconだとあまりこういった機能の出番はなさそう
  - と思ったがMySQL 8のデフォルトの認証の`caching_sha2_password`はTLSサポートのほうが認証は速くできる

### LOAD DATA LOCAL INFILE

bulk insertが書きづらいためこちらを使うほうがよさそう。

READMEやテストにはないが、複数のカラムをINSERTしたい場合はタブで連結すればよい。

タブはデフォルトの設定に過ぎないのでたとえば,(カンマ)が使いたければ `LOAD DATA LOCAL FILE 'filename' INTO TABLE tbl FIELDS TERMINATED BY ','` などFIELDS句で指定することもできる。

```rust
use std::io::Write;
use mysql::*;
use mysql::prelude::*;

fn main() {
    let url = "mysql://user:password@127.0.0.1:3307/testdb".to_string();
    let opts = Opts::from_url(&*url).unwrap();
    let mut conn = Conn::new(opts).unwrap();
    conn.query_drop("CREATE TEMPORARY TABLE tbl (
        `id` INT,
        `text` TEXT
    )").unwrap();
    conn.set_local_infile_handler(Some(LocalInfileHandler::new(|_, stream| {
        stream.write_all(format!("{}\t{}\n", 1, "hoge").as_bytes())?;
        Ok(())
    })));

    match conn.query_drop("LOAD DATA LOCAL INFILE 'file_name' INTO TABLE tbl") {
        Ok(_) => {}
        Err(err) => panic!("ERROR {}", err),
    }

    let result = conn.query_iter("SELECT `id`,`text` FROM tbl").unwrap();
    for row in result {
        println!("{:?}", row.unwrap());
    }
}
```

## mysql_async

調査時点でのバージョンは 0.25.0

- tokioのイベントループと組み合わせて使うような実装になってるクライアントライブラリ、こちらもlow-levelな実装
- rust-mysql-simpleと同じ作者が作っていてプロトコルまわりなどは共通の実装
- 非同期前提って感じになるけど、v0.11以降のhyperとreactorとか共有できるか調べたほうがよさそう
- rust-mysql-simple同様mysql_async自体でコネクションプール持つので自前で用意する必要はない
  - こちらもr2d2と組み合わせられるかなどは調査したほうがよさそう
- こちらもprepared statementを使える
  - コネクションプールとのかみ合わせはよくわからない
- プロトコルまわりはmysql-simpleと同じ

## mysqlclient-sys

- CのMySQLクライアントライブラリであるlibmysqlclientのRust binding。ソケットがブロッキング前提なのでpreforkなやつかスレッドプールなサーバ実装と組み合わせるかんじ。
- [greenify](https://github.com/douban/greenify) とか [Coro-MySQL](https://github.com/gitpan/Coro-Mysql) みたいに無理やり非同期I/Oと組み合わせることはできる。
