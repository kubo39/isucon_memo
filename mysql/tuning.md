# クエリチューニング

## N+1クエリ

- 例
  - この例だと100回ループが走る

```
auto conn = client.lockConnection();
auto memos = conn.query("select * from `memos` where `is_private` = 0
    order by `created_at` desc, `id` desc limit 100");
foreach (memo; memos)
{
    memo.username = conn.queryRow("select `username` from `users` where `id` = ?",
                                  memo.user);
}
```

- inner joinを使ってクエリを書き換える
  - 結合したテーブルを作る
  - outer joinというのもある

```
auto memos = conn.query("select `memos.*`, `user.username`
    from `memos` join `users` on `memos.user` = `user.id`
    where `memos.is_private_id` = 0
    order by `memos.created_at` desc,
             `memos.id` desc limit 100");
```

## インデックス

- 以下のクエリは `is_private`, `created_at` にインデックスをはっていないので遅い
  - is_private: memosテーブルをフルスキャンして抽出してしまう
  - created_at: テーブルに対してMySQL鯖内でソートが走る(CPU負荷が高い)

```
auto memos = conn.query("select * from `memos` where `is_private` = 0 order by
    created_at desc limit 100");
```

- インデックスは `alter table` 構文を使うことで生成できる

```sql
ATLER TABLE memos ADD INDEX (is_private,created_at);
```

## オフセットと破棄

- MySQLのオフセットはソートと組わせるとつらい
  - オフセットが大きいと大量のデータ破棄が発生

```
conn.query("select * from `memos` order by `created_at` limit 100 offset 10000");
```

- 可能なら取得するデータを制限

```
conn.query("select `id` from `memos` order by `created_at` limit 100 offset 10000");
```
