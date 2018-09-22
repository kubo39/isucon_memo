# プロファイリング

## 実行計画

- explainで実行計画をみる
- 読み方は「漢のコンピュータ道」さんのサイトなど解説資料などがあるので割愛
- どのように改善したらいいかという情報はないので自分で考える必要がある

## myprofiler

- KLab/myprofiler で遅いクエリを調査する
- 実体は `SHOW FULL PROCESSLIST`
- 注意: OPERATOR権限を持つユーザかHTTPサーバがDBに接続するときと同じユーザで実行
  - SHOW FULL PROCESSLISTなので当たり前だけど
