# メッセージボックス

Rustの標準ライブラリを使用して、説明されたようなメッセージボックス機能を実装するサンプルプログラムを示します。ここでは、`std::sync::mpsc（multiple producer, single consumer）`モジュールを使用して、メッセージの送受信を行います。タイムアウト機能付きのブロッキング受信は、`ecv_timeout` メソッドを用いて実現します。

## サンプルプログラム
```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let mut children = vec![];
    let mut receivers = vec![];

    // 3つの子スレッドを生成し、それぞれに独立した受信チャネルを設定
    for i in 0..3 {
        let (tx, rx) = mpsc::channel();
        receivers.push(tx);

        let child = thread::spawn(move || {
            match rx.recv_timeout(Duration::from_secs(5)) {
                Ok(message) => {
                    println!("Thread {} received: {}", i, message);
                }
                Err(e) => {
                    println!("Thread {} had an error: {:?}", i, e);
                }
            }
        });

        children.push(child);
    }

    // メインスレッドから各子スレッドにメッセージを送信
    for (i, tx) in receivers.into_iter().enumerate() {
        tx.send(format!("Hello from main thread to thread {}", i)).unwrap();
        println!("Main thread sent message to thread {}", i);
    }

    // 子スレッドの終了を待つ
    for child in children {
        child.join().unwrap();
    }
}
```

このプログラムでは、各子スレッドが独自の `Receiver` を持っており、メインスレッドは各`Receiver` に対応する `Sender` を使用して特定の子スレッドにメッセージを送信します。子スレッドはメインスレッドからのメッセージを受け取り、内容を表示した後に終了します。タイムアウトが発生すると、エラーメッセージが表示されます。
