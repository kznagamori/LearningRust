# スレッド

Rustで複数のスレッドを起動し、定義された関数を実行し、そのすべての終了を待つプログラムの例を示します。この例では、`std::thread` を使用しています。

まず、スレッドで実行するための関数を定義します。次に、複数のスレッドを生成し、それぞれで定義された関数を実行します。最後に、すべてのスレッドが終了するのを待ち合わせます。

## サンプルプログラム

```rust
use std::thread;
use std::time::Duration;

// スレッドで実行する関数
fn thread_function(id: u32) {
    println!("Thread {} is starting", id);
    // スレッドが何かの作業を行っていることをシミュレート
    thread::sleep(Duration::from_millis(500));
    println!("Thread {} has finished", id);
}

fn main() {
    let mut handles = vec![];

    for i in 0..5 {
        // 新しいスレッドを生成し、定義された関数を実行
        let handle = thread::spawn(move || {
            thread_function(i);
        });
        handles.push(handle);
    }

    // すべてのスレッドが終了するのを待つ
    for handle in handles {
        handle.join().unwrap();
    }

    println!("All threads have finished.");
}
```

このプログラムでは、`thread_function` という名前の関数を定義し、スレッドで実行します。この関数は引数として `id`（スレッドの識別子）を受け取り、スレッドの開始と終了を表示します。メイン関数では、5つのスレッドを生成し、各スレッドで `thread_function` を実行します。その後、`join` メソッドを使用して各スレッドの終了を待ち合わせ、すべてのスレッドが終了したことを確認します。

