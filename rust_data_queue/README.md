# キュー構造

Rustでキュー（ `queue` ）構造を実装する基本的な方法は、2つの `Vec` を使用してキューの前面と背面を管理することです。この実装では、要素を追加する際には背面の `Vec` にプッシュし、要素を取り出す際には前面の `Vec` からポップします。前面の `Vec` が空の場合、背面の `Vec` を逆順にして前面に移動します。以下にサンプルプログラムを示します。

## サンプルプログラム
```rust
struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    // 新しいキューを生成する関数
    fn new() -> Self {
        Queue {
            elements: Vec::new(),
        }
    }

    // キューの背面に要素を追加する関数
    fn enqueue(&mut self, item: T) {
        self.elements.push(item);
    }

    // キューの前面から要素を取り出す関数
    fn dequeue(&mut self) -> Option<T> {
        if self.elements.len() == 0 {
            return None;
        }
        // 前面から要素を取り出します
        // remove()は、T型を返す
        let value = self.elements.remove(0);
        // valueはT型なのでSome(T)でOption<T>型に変換して返す
        return Some(value);
    }
}

fn main() {
    let mut queue = Queue::new();

    // キューに要素を追加します
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);

    // キューから要素を取り出し、表示します
    while let Some(value) = queue.dequeue() {
        println!("{}", value);
    }
}
```

このコードでは、`Queue` 構造体を使用してキューを実装しています。`enqueue` メソッドでキューに要素を追加し、`dequeue` メソッドでキューから要素を取り出します。`main` 関数では、キューにいくつかの要素を追加し、それらを取り出して表示しています。

この実装は基本的なキューの機能を提供しますが、より効率的な実装や追加の機能が必要な場合は、実装をさらに拡張することができます。