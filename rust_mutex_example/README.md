# 排他処理

Rustで複数のスレッドによる排他処理を行う際に、ミューテックスで保護された構造体を使用する例を示します。この例では、`Mutex` でラップされた簡単な構造体を複数のスレッド間で共有し、スレッドごとにその構造体のデータを変更します。

## サンプルプログラム
まずは構造体を定義し、次にその構造体を `Mutex` でラップして複数のスレッドで共有します。

```rust
use std::sync::{Arc, Mutex};
use std::thread;

// 共有されるデータの構造体
struct SharedData {
    count: i32,
}

fn main() {
    // ミューテックスでラップされた構造体を作成
    let data = Arc::new(Mutex::new(SharedData { count: 0 }));

    let mut handles = vec![];

    for _ in 0..5 {
        // Arcをクローンして各スレッドに渡す
        let data = Arc::clone(&data);

        let handle = thread::spawn(move || {
            let mut data = data.lock().unwrap();
            // データのcountフィールドを変更
            data.count += 1;
        });

        handles.push(handle);
    }

    // すべてのスレッドの終了を待つ
    for handle in handles {
        handle.join().unwrap();
    }

    // 最終的なデータを表示
    println!("Result: {}", data.lock().unwrap().count);
}
```

このプログラムでは、`SharedData` 構造体を定義し、そのインスタンスを `Mutex` でラップしています。`Arc` を使用することで、このミューテックスを複数のスレッド間で安全に共有できます。

各スレッドでは、`data.lock().unwrap()` を呼び出してミューテックスをロックし、構造体のcountフィールドにアクセスして値を変更します。`MutexGuard` はスコープの終わりで自動的にドロップされ、ミューテックスのロックが解放されます。

最後に、メインスレッドで全スレッドの終了を待ち合わせ、最終的な `count` の値を表示しています。これにより、複数のスレッドが同じデータに安全にアクセスし、変更することができます。

