# HTTPサーバ

標準ライブラリのHTTPサーバを使う。httpserver,asynchttpserverの2つ用意してる。

## httpserver

`import httpserver` で使える。使いどころがわからん。

## asynchttpserver

`import asynchttpserver` で使える。
コアの部分はasyncdispatchとselectorsに書いてる。selectorsモヂュールみたほうが雰囲気はわかるかと。
[jester](https://github.com/dom96/jester) はこっちベースなのでこっちがデファクトっぽい。
