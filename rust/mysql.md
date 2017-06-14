# MySQL

RustのMySQLドライバはいくつかある

- [rust-mysql-simple](https://github.com/blackbeam/rust-mysql-simple)
- [mysql_async](https://github.com/blackbeam/mysql_async)
- [mysqlclient-sys](https://github.com/sgrif/mysqlclient-sys)

## rust-mysql-simple

low-levelなクライアントライブラリ。
ネットワークまわりでいうと、libmysqlclientとかと違ってノンブロッキングI/Oを使うことは選択できそうだが、コアのイベントループとかの連携とか別に考えてやる必要がある。
コネクションプールを内部的に実装してるので [r2d2](https://github.com/sfackler/r2d2) を使う必要がないけど、実装を見比べてベンチをとったほうがいいかも。
mysql_asyncに比べるとsslサポートとかjsonが使えるなどがあるが、isuconだとあまりこういった機能の出番はなさそう。

## mysql_async

tokioのイベントループと組み合わせて使うような実装になってるクライアントライブラリ、こちらもlow-levelな実装。
rust-mysql-simpleと同じ作者が作ってるが、これらのライブラリは共有する実装がないようだ。
非同期前提って感じになるけど、v0.11以降のhyperとreactorとか共有できるか調べたほうがよさそう。
rust-mysql-simple同様mysql_asyncは自前でコネクションプール持つので自前で用意する必要はない。

## mysqlclient-sys

CのMySQLクライアントライブラリであるlibmysqlclientのRust binding。ソケットがブロッキング前提なのでpreforkなやつかスレッドプールなサーバ実装と組み合わせるかんじ。
[greenify](https://github.com/douban/greenify) とか [Coro-MySQL](https://github.com/gitpan/Coro-Mysql) みたいに無理やり非同期I/Oと組み合わせることはできる。

