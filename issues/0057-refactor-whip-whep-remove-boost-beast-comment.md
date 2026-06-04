# whip/whep の boost::beast 由来のレガシーコメントを削除する

- Priority: Low
- Created: 2026-06-05
- Model: Claude Opus 4.8
- Branch: feature/refactor-whip-whep-remove-boost-beast-comment

## 目的

`webrtc/src/whip.cpp` と `webrtc/src/whep.cpp` には、本コードベースに存在しない `boost::beast` および `req_` / `config_` などのメンバを参照する旧実装のコピペ残骸がコメントとして残っている。実体と対応しない誤解を招くコメントであるため削除する。

## 優先度根拠

ビルドや動作には影響しないコメントのみの問題であり Low とする。ただし存在しないライブラリやメンバを参照しているため、読み手を誤解させる。

## 現状

レビュー時点で実コードを確認済み。`whip.cpp:743-745` で、HTTP リクエスト文字列を組み立てる箇所にコメントが残っている。

```cpp
              std::string req = "POST " + target + " HTTP/1.1\r\n";
              // self->req_.set(boost::beast::http::field::authorization, "Bearer " + self->config_.secret_key);
              req += "Host: " + parts.host + ":" + parts.GetPort() + "\r\n";
```

`whep.cpp:483-485` にも同一のコメント（`// self->req_.set(boost::beast::http::field::authorization, ...)`）が残っており、両ファイルで重複している。本コードベースは `boost::beast` に依存しておらず、`req_` / `config_` / `secret_key` といったメンバもこのコードには存在しない。

## 設計方針

- `whip.cpp:744` および `whep.cpp:484` の `boost::beast` 由来のレガシーコメントを削除する。

## 完了条件

- 当該コメントが両ファイルから除去される。
