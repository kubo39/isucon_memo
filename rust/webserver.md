# HTTPサーバ側

## hyper

v0.11からtokioベースの非同期I/Oになった http://seanmonstar.com/post/161786147642/hyper-v011

I/Oの組み合わせを考えると [mysql_async](https://github.com/blackbeam/mysql_async) を使うことを考えたい。

http2対応はしてないので、そのへんは自作するかリバースプロキシにnginxとかh2oとかでバックエンドとしてhyper使うとか。

## tokio-minihttp

[Framework Benchmark](https://www.techempower.com/benchmarks/previews/round15/) でいい成績出している。

めちゃくちゃ軽量なのでどのくらいか自前実装する必要ありそう。

コアはtokio + futuresなのでhyperとだいたい同じI/O特性なんだけど、acceptorとworker threadでそれぞれプールを持つような設計。
