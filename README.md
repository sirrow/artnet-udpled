artnet-udpled
=============
artnet-udpled は、Art-Net を受け取り、[udpled](https://github.com/sirrow/udpled) 用のバイナリ形式に変換し、udpled が動くマイコンへデータを送信します。

artnet-udpled は、udpled が動いているマイコンの IP アドレスが "udpled_0001.local" で解決可能であることを想定します。
udpled は、Art-Net 形式のデータを受け取り、かつ udpled_0001.local が解決できた場合、Art-Net を udpled 用のバイナリデータへ変換し udpled_0001.log の 8888/udp へ送信します。

16×16 のフルカラーのマトリクスに出力することを現時点では想定しています。

[rustup を用いた rust 環境の構築を行い](https://www.rust-lang.org/ja/tools/install) "cargo run" により実行可能です。