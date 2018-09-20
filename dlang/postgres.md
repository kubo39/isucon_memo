# Postgres

## 戦略

vibe.d前提ということであれば迷わず vibe.d.db.postgresql を使えばよさそうだ。
libpq依存であることには注意。

## dpq2

調査時点でのバージョンは v1.0.0-rc.6

- 中ではlibpqを使ってる
- コネクションプールは提供してない

## vibe.d.db.postgresql

調査時点でのバージョンは v2.0.0-alpha3

- 裏側ではdpq2を使ってる
  - なので実体はこちらもlibpq
  - dpq2 + vibe.dのコネクションプールを組み合わせてる
