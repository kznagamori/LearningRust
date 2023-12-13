## リンクドリスト

Rust言語でリンクドリスト（ `LinkedList` ）を使用した実行可能なサンプルプログラムをご紹介します。Rustの標準ライブラリには `LinkedList` 型が含まれており、これを使ってリンクドリストの操作を行うことができます。

以下の例では、`LinkedList` をインポートし、新しいリンクドリストを作成していくつかの要素を追加し、リストの内容を表示する方法を示しています。

## サンプルプログラム
```rust
use std::collections::LinkedList;

fn main() {
    // 新しいリンクドリストを作成
    let mut list = LinkedList::new();

    // 要素をリストの末尾に追加
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    // リストの先頭に要素を追加
    list.push_front(0);

    // リンクドリストの要素をイテレートして表示
    for element in list.iter() {
        println!("{}", element);
    }

    // リストの長さを表示
    println!("Length of the list: {}", list.len());
}
```

このプログラムは、リンクドリストに整数を追加し、それらを順に表示するシンプルな例です。リストへの要素の追加、イテレーション、長さの取得など、基本的なリンクドリストの操作を行っています。Rustの強力な型システムと所有権モデルにより、安全かつ効率的にデータ構造を扱うことができます。
