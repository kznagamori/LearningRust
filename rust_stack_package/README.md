# スタック

Rustでスタック（ `Stack` ）の機能を実装する場合、標準ライブラリの `Vec` を使用するのが一般的です。`Vec` は動的配列であり、スタックの基本的な操作、つまり要素の追加（プッシュ）と削除（ポップ）を簡単に実行できます。外部クレートを使用する必要はありません。

以下に `Vec` を用いたスタックの実装例を示します。このプログラムでは、スタックに要素を追加し（プッシュ）、その後で要素を取り出す（ポップ）基本的な操作を行います。

## サンプルプログラム
```rust
fn main() {
    let mut stack = Vec::new();

    // スタックに要素をプッシュ
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // スタックから要素をポップ
    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }
}
```

このプログラムでは、`Vec::new()` を使用して新しいスタックを作成し、`push` メソッドで要素をスタックに追加しています。その後、`pop` メソッドを使用してスタックから要素を取り出し、それを表示しています。

`Vec` はスタックの他にも様々な用途に使用できる柔軟なデータ構造であり、Rustの標準ライブラリに含まれているため、追加の依存関係を指定する必要はありません。また、`Vec` は他にも多くの有用なメソッドを提供しているため、詳細は公式ドキュメントで確認することをお勧めします。

