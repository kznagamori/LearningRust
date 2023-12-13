# 平衡二分木

Rustで平衡二分木を使用する際、外部クレートの一つである `ABtree`を利用することができます。`ABtree` は、`AVL` 木（ `Adelson-Velsky and Landis Tree` ）などの平衡二分木の実装を提供するクレートです。`AVL` 木は、挿入、削除、検索操作が平均および最悪ケースで `O(log n)` の時間複雑度を持つ自己平衡二分探索木です。

ここでは、`ABtree` クレートを使用した平衡二分木の実装例を提供します。まず、`Cargo.toml` ファイルに `ABtree` クレートを依存関係として追加する必要があります。

## サンプルプログラム
**Cargo.toml**
```
[dependencies]
ABtree = "0.8.0"
```

**src/main.rc**
```rust
use ABtree::AVL;

fn main() {
    let mut tree = AVL::<i32, &str>::new();

    // AVL木に要素を追加
    tree.insert(3, "three");
    tree.insert(1, "one");
    tree.insert(4, "four");
    tree.insert(2, "two");

    // AVL木を走査して要素を表示
    for (_, value) in tree.iter() {
        println!("{}", value);
    }

    // 特定のキーを検索
    if tree.contains(&2) {
        println!("Tree contains key 2");
    } else {
        println!("Tree does not contain key 2");
    }
}
```

このプログラムでは、`AVLTree` を使用して平衡二分木を作成し、`insert` メソッドで要素を追加しています。iterメソッドを使用して木を走査し、`contains`` メソッドで特定の値の存在を確認しています。

`ABtree` クレートは、AVL木を使って効率的に要素の管理を行うための多くの機能を提供します。ただし、上記のコードは基本的な使用例を示しており、`ABtree`クレートには他にも多くの機能があります。詳細については、公式のドキュメントを参照することをお勧めします。
