# Async/Await

`async-std` クレートを使用したRustの `async/await` 機能の例を示します。`async-std` は、非同期プログラミングをサポートするための標準ライブラリの非同期版と考えることができます。

## サンプルプログラム
まず、`Cargo.toml` に `async-std` クレートを依存関係として追加します。
**Cargo.toml**
```
[dependencies]
async-std = { version = "1.9", features = ["attributes"] }
```

次に、以下のような非同期関数を含むサンプルプログラムを作成します。
**src/main.rc**
```rust
use async_std::task;
use std::time::Duration;

// 非同期関数
async fn do_something(id: u32) {
    println!("Task {} is starting", id);
    // 何かの処理をシミュレートするために非同期で待機
    task::sleep(Duration::from_secs(2)).await;
    println!("Task {} is done", id);
}

#[async_std::main]
async fn main() {
    // 複数の非同期タスクを生成して実行
    let task1 = task::spawn(do_something(1));
    let task2 = task::spawn(do_something(2));

    // すべてのタスクが完了するのを待つ
    task1.await;
    task2.await;

    println!("All tasks completed.");
}
```

このプログラムでは、`async_std::main` 属性を使用して非同期のエントリポイントを作成し、`task::spawn` で非同期タスクを生成しています。`do_something` 関数は非同期関数で、task::sleepを使用して非同期で待機しています。

各非同期タスクは、`await` を使用して待機されます。これにより、非同期タスクが完了するまでメイン関数の実行がブロックされます。

`async-std` クレートは、Rustでの非同期プログラミングを容易にするために様々なユーティリティを提供しています。これにより、非同期I/O操作、タスクのスケジューリング、同時実行などが簡単に行えます。
