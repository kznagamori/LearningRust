# スタック構造

Rustでスタック構造を実装するのは比較的簡単です。スタックは後入れ先出し（ `LIFO: Last-In, First-Out` ）のデータ構造で、`Vec` を使用して効率的に実装できます。以下に、スタックの基本的な操作（プッシュ、ポップ、トップの参照）を含む簡単な実装例を示します。

## サンプルプログラム
```rust
struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    // 新しいスタックを生成する関数
    fn new() -> Self {
        Stack {
            elements: Vec::new(),
        }
    }

    // スタックに要素を追加する関数
    fn push(&mut self, item: T) {
        self.elements.push(item);
    }

    // スタックから要素を取り出す関数
    fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    // スタックのトップ要素を参照する関数
    fn peek(&self) -> Option<&T> {
        self.elements.last()
    }
}

fn main() {
    let mut stack = Stack::new();

    // スタックに要素を追加します
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // スタックのトップ要素を表示します
    if let Some(top) = stack.peek() {
        println!("Top: {}", top);
    }

    // スタックから要素を取り出し、表示します
    while let Some(top) = stack.pop() {
        println!("Pop: {}", top);
    }
}
```

このコードでは、ジェネリック型 `T` を持つ `Stack` 構造体を定義しています。`Stack` は内部的に `Vec<T>` を使用しており、`push` メソッドで要素を追加し、`pop` メソッドで最後に追加された要素を取り出し、`peek` メソッドで最後の要素を参照することができます。`main` 関数では、スタックにいくつかの要素を追加し、トップ要素を表示した後、要素を取り出して表示します。

この基本的な実装は多くの用途に適していますが、必要に応じてさらに機能を追加することもできます。
