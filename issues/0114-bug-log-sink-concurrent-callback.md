# LogSinkHandler の callback が同時に呼ばれると未定義動作になる

- Created: 2026-10-11
- Completed: {YYYY-MM-DD}
- Branch: feature/fix-log-sink-concurrent-callback
- Polished: {YYYY-MM-DD}

## 目的

`log::LogSinkHandler` を複数のスレッドから同時に呼ばれても安全に使えるようにする。

`log::LoggingConfig::add_sink` で登録した sink は、 ログを出したスレッドでそのまま呼ばれる。 `log::print` はどのスレッドからでも呼べるため、 同じ sink の `on_log_message` が同時に実行され得る。 現状の `LogSinkHandler` は `&mut self` で呼ばれるため、 この状況で同じ実体への可変参照が重複し、 `unsafe` を書かずに未定義動作へ到達できる。

## 現状

- `src/rtc_base/logging.rs` の `LogSinkHandler` は `Send` のみを要求し、 `on_log_message` は `&mut self` を取る
- `handler_state` は `*mut c_void` から `&mut LogSinkHandlerState` を作る。 参照のライフタイムも引数に縛られていない
- `log_sink_on_log_line_ref` は `handler_state` を呼び出してから `on_log_message` を呼ぶ。 この入口で可変参照を作るため、 同時呼び出しでは同じ実体への `&mut` が重複する
- libwebrtc の `rtc_base/logging.cc` の `LogMessage::~LogMessage` は、 `config.sinks()` のループをロックを取らずに実行する。 直後の legacy な `streams_` のループは `GetLoggingLock()` を取るため、 直列化されるのは legacy な経路だけである
- `HandlerState` (`src/helper/handler.rs`) は `Box<H>` を持つだけなので、 `H` が `Send + Sync` なら `HandlerState<H>` も自動で `Send + Sync` になる

`unsafe` を書かずに到達できる経路は次のとおり。 これは問題の到達経路を示すコードであり、 未修正の実装で実行する回帰テストにはしない。

```rust
struct Counter {
    count: usize,
}

impl log::LogSinkHandler for Counter {
    fn on_log_message(&mut self, _line: log::LogLineRef<'_>) {
        // 同じ実体への &mut が複数スレッドで重複し得る
        self.count += 1;
    }
}

let mut config = log::LoggingConfig::new();
config.add_sink(log::LogSink::new_with_handler(Box::new(Counter { count: 0 })));
log::initialize_logging(config);

let threads: Vec<_> = (0..8)
    .map(|_| std::thread::spawn(|| log::print(log::Severity::Info, "webrtc-c", 0, "message")))
    .collect();
for thread in threads {
    thread.join().expect("スレッドの join に失敗しました");
}
```

`on_log_message` の中で `Mutex` を取っても防げない。 ロックを取得する前に、 呼び出しの入口で `&mut` が重複するためである。

## 設計方針

- `LogSinkHandler` の supertrait を `Send + Sync` にする
- `on_log_message` のレシーバを `&self` にする。 可変状態を持つ実装は実装側で `Mutex` や atomic を使う契約とし、 Rustdoc に明記する
- `handler_state` と `log_sink_on_log_line_ref` を共有参照 (`&LogSinkHandlerState`) に変え、 callback の入口で可変 state を作らない
- 既存の実装 (`src/tests.rs` の `TestSinkHandler`) を `&self` に追随させる
- `initialize_logging` が 1 回しか適用されない契約 (初期化の競合) は本 issue の対象外とする
- `CHANGES.md` に `[CHANGE]` を追記する

## 完了条件

- `LogSinkHandler` が `Send + Sync` を要求し、 `on_log_message` が `&self` を取る
- `handler_state` と `log_sink_on_log_line_ref` が共有参照を使い、 callback の入口で可変参照を作らない
- `LogSinkHandler` の Rustdoc に、 同時呼び出しがあり得ることと `&self` で受け取る理由が書かれている
- `src/tests.rs` に、 複数スレッドから同時に `log::print` を呼び sink が全件を受け取ることを検証するテストがある。 実際の C ラッパーを通し、 モックやスタブは使わないこと
- 既存のテストとサンプルが新しい trait に追随している
- `cargo fmt --all -- --check` / `cargo clippy --workspace --features source-build --all-targets -- -D warnings` / `cargo test --workspace --features source-build` が通る
- 公開 API の変更を伴うため `CHANGES.md` の `## develop` に `[CHANGE]` が記載されている
