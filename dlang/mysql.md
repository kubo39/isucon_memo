# MySQLドライバ

vibe.d前提なので、ドライバの選定もそれを踏まえたものになる。

- [mysql-native](https://github.com/mysql-d/mysql-native)
- [mysql-lited](https://github.com/eBookingServices/mysql-lited)

どちらもVibeSocket前提かつコネクションプールを自前で持つ実装になってるが、mysql-litedのほうがAPI的に使いやすい感(個人の感想です)。
単に非同期I/Oなだけでなく、EAGAINとかのときにFiberで実行主体が切り替わるのがミソなので、コアがvibe.dのリアクターでないと意味がない。
