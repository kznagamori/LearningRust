# ハッシュテーブル

Rustでハッシュテーブルを使用する場合、標準ライブラリの `HashMap` を利用するのが一般的です。`HashMap` は、キーと値のペアを効率的に格納し、高速な検索を可能にするデータ構造です。外部クレートを使用する必要はありません。

以下に、`HashMap` を使用したハッシュテーブルの実装例を示します。このプログラムでは、ハッシュマップにキーと値のペアを追加し、それらを取り出す基本的な操作を行います。

## サンプルプログラム
```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    // HashMapにキーと値を追加
    map.insert("apple", 3);
    map.insert("banana", 2);
    map.insert("orange", 4);

    // キーに対する値を取得して表示
    if let Some(&number) = map.get("apple") {
        println!("There are {} apples.", number);
    }

    // HashMapのキーと値を順に表示
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    // 特定のキーの存在を確認
    if map.contains_key("banana") {
        println!("There are bananas in the map.");
    } else {
        println!("There are no bananas in the map.");
    }
}
```

このプログラムでは、`HashMap::new()` を使用して新しいハッシュマップを作成し、`insert` メソッドでキーと値のペアを追加しています。`get` メソッドを使用して特定のキーに対応する値を取得し、`contains_key` メソッドで特定のキーの存在を確認しています。また、マップのすべてのキーと値を順に表示するためにイテレータを使用しています。

`HashMap` は、キーに対して高速なアクセスを提供し、大きなデータセットを扱う際に特に有効です。また、Rustの所有権モデルにより、安全かつ効率的にデータを管理することができます。
