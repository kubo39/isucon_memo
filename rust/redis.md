# Redis

- [redis-rs](https://github.com/mitsuhiko/redis-rs)
- [redis-async-rs](https://github.com/benashford/redis-async-rs)
- [fred.rs](https://github.com/azuqua/fred.rs)

セッションストレージ的な使い方をするために特化したライブラリもある。

- [session](https://github.com/hikelee/session)

## redis-rs

調査時点でのバージョンは 0.9.1

- 非同期にも対応しているらしい。たしかにtokio/futures使っている

## redis-async-rs

調査時点でのバージョンは 0.4.2

- 非同期前提のクライアントライブラリ

## fred.rs

調査時点でのバージョンは 0.1.8

- こちらも非同期前提
- 現時点では機能的にはまだ未対応のものが多い
