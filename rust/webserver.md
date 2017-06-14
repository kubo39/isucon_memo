# HTTPサーバ側

## hyper

v0.11からtokioベースの非同期I/Oになった http://seanmonstar.com/post/161786147642/hyper-v011

I/Oの組み合わせを考えると [mysql_async](https://github.com/blackbeam/mysql_async) を使うことを考えたい。

http2対応はしてないので、そのへんは自作するかリバースプロキシにnginxとかh2oとかでバックエンドとしてhyper使うとか。
