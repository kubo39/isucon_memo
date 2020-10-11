# トランザクション

## START TRANSACTION

- トランザクション操作を開始する
  - トランザクション中は自動コミット(autocommit)は無効になる
  - 暗黙的に自動コミットが発生する操作(操作前にCOMMITしたかのように振る舞う)もある
    - ALTER TABLEなど

## COMMIT

- 現在のトランザクションをcommitして永続化する

## ROLLBACK

- 現在のトランザクションをrollbackして変更を取り消す
  - ロールバックできない操作もある
    - データベースやテーブルの作成・削除など
    - このような操作を含まないトランザクションを作成するべき

## ISOLATION LEVEL (やらない?)

- トランザクション分離レベル
  - Repeatable Read: デフォルト、トランザクション開始時にコミットされたデータのみみえる
  - Read Commited: 他のトランザクションのコミットされたデータがみえる
  - Read Uncommited: 他のトランザクションのコミットされていないデータがみえる
  - Serializable: すべてのselectを直列化して競合が発生しないように
  - 詳しくはMySQL公式ドキュメントのSET TRANSACTIONを調べること

## ACCESS MODE (やらない?)

- トランザクションアクセスモード
  - READ ONLY: トランザクション内の操作が読み取り操作のみ (>=MySQL5.6.5)
  - READ WRITE: 書き込み操作もあり

## CONSISTENT SNAPSHOT (やらない?)

- IsolateionLevelがREPEATABLE READのときだけ有効

## AUTOCOMMIT

- 自動コミットモード
  - INSERT/UPDATEクエリが即座に永続化されてロールバック不可
- MySQLはデフォルトで有効
