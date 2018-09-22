# PostgreSQL

## 戦略

I/Oモデルにかかわらずrust-postgresを使うので特に悩む必要はなし

## rust-postgres

- ネイティブなpostgresqlドライバなのでCのライブラリへの依存がない
- tokio対応もしている
- postgresqlは高負荷であればコネクションプールは必須
  - 同じ作者がジェネリックなコネクションプール実装であるr2d2も書いている
  - sfackler氏が書いてる安心感
- prepared statementをサポート
  - コネクションプールとのかみ合わせは調べておいたほうがよい
  - トップレベルにstmtを保持するとか
- トランザクションをサポート
- Postgresの型とのマッピングもサポート
  - JSON型などもマッピングされている
