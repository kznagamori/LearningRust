# キュー

Rustでキュー（ `Queue` ）を利用する際、標準ライブラリの `VecDeque` を使うことが一般的です。`VecDeque` は、両端に効率的に要素を追加・削除できる二重終端キュー（ `double-ended queue` ）を提供します。外部クレートを使用せずに、この `VecDeque` を用いたキューの実装例を紹介します。

以下のプログラムは、`VecDeque` を使用して、キューに要素を追加し、それらを取り出す基本的な操作を行います。

## サンプルプログラム
```rust
use std::collections::VecDeque;

fn main() {
    let mut queue = VecDeque::new();

    // キューに要素を追加
    queue.push_back(1);
    queue.push_back(2);
    queue.push_back(3);

    // キューから要素を取り出し
    while let Some(value) = queue.pop_front() {
        println!("Dequeued: {}", value);
    }
}
```

このプログラムでは、`VecDeque` を使用してキューを作成し、push_backメソッドで要素をキューの末尾に追加しています。その後、`pop_front` メソッドを使用してキューの先頭から要素を取り出し、それを表示しています。

`VecDeque` は、Rustの標準ライブラリに含まれているため、追加の依存関係を指定する必要はありません。これにより、キューの基本的な機能を利用することができます。なお、`VecDeque` は他にも多くの有用なメソッドを提供しているため、詳細は公式ドキュメントで確認することをお勧めします。
