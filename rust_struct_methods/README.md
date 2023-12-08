# クラスを使う

Rustには「クラス」という概念はありませんが、「構造体（ `struct` ）」と「メソッド」を使用してクラスのような機能を実装できます。以下は、Rustで構造体を定義し、メソッドを実装する簡単な例です。

## サンプルコード
この例では、`Person` という構造体を定義し、それにいくつかのメソッドを実装します。

**src/main.rs**
```rust
// 構造体の定義
struct Person {
    name: String,
    age: u8,
}

// Person 構造体に対するメソッドの実装
impl Person {
    // 新しい Person を作成する関数
    fn new(name: String, age: u8) -> Person {
        Person { name, age }
    }

    // Person の自己紹介を行うメソッド
    fn introduce(&self) {
        println!("Hello, my name is {} and I am {} years old.", self.name, self.age);
    }
}

fn main() {
    // Person 構造体のインスタンスを作成
    let person = Person::new("Alice".to_string(), 30);

    // メソッドを呼び出し
    person.introduce();
}
```

## 手順

1. **新しいプロジェクトを作成:**
- ターミナルで `cargo new rust_struct_methods`` コマンドを実行して新しいプロジェクトを作成します。

2. **ファイルの編集:**
- `src/main.rs` ファイルを開き、上記のサンプルコードを記述します。

3. **ビルドと実行:**
- プロジェクトのルートディレクトリで `cargo build` コマンドを実行してプロジェクトをビルドします。
- ビルドが完了したら `cargo run` コマンドを実行してプログラムを実行します。

このサンプルでは、`Person` という構造体を定義し、それに対して `new` と `introduce` という二つのメソッドを実装しています。これにより、オブジェクト指向プログラミングのクラスに似た振る舞いを実現しています。Rustでは、このように構造体とメソッドを組み合わせることでデータとその操作をカプセル化します。
