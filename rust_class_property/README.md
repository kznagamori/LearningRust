# C#みたいなクラスのプロパティを定義、使用

Rustでは、C#のような「プロパティ」という概念はありませんが、構造体（struct）とメソッドを組み合わせてプロパティのような機能を実装することができます。以下の例では、ゲッター（ `getter` ）とセッター（ `setter` ）メソッドを持つ構造体を定義して、C#のプロパティのような動作を実現します。

## サンプルコード
**src/main.rs**
```rust
struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // コンストラクタ
    fn new(first_name: String, last_name: String) -> Person {
        Person { first_name, last_name }
    }

    // first_name のゲッター
    fn first_name(&self) -> &str {
        &self.first_name
    }

    // first_name のセッター
    fn set_first_name(&mut self, first_name: String) {
        self.first_name = first_name;
    }

    // last_name のゲッター
    fn last_name(&self) -> &str {
        &self.last_name
    }

    // last_name のセッター
    fn set_last_name(&mut self, last_name: String) {
        self.last_name = last_name;
    }

    // フルネームを返すメソッド
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

fn main() {
    let mut person = Person::new("Alice".to_string(), "Smith".to_string());

    // ゲッターを使用してフィールドの値を取得
    println!("First Name: {}", person.first_name());
    println!("Last Name: {}", person.last_name());

    // フルネームを表示
    println!("Full Name: {}", person.full_name());

    // セッターを使用してフィールドの値を変更
    person.set_first_name("Bob".to_string());
    person.set_last_name("Johnson".to_string());

    // 変更後のフルネームを表示
    println!("Full Name: {}", person.full_name());
}
```
## 手順

1. **新しいプロジェクトを作成:**
- ターミナルで `cargo new rust_class_property` コマンドを実行して新しいプロジェクトを作成します。

2. **ファイルの編集:**
- `src/main.rs` ファイルを開き、上記のサンプルコードを記述します。

3. **ビルドと実行:**
- プロジェクトのルートディレクトリで `cargo build`` コマンドを実行してプロジェクトをビルドします。
- ビルドが完了したら `cargo run` コマンドを実行してプログラムを実行します。

このサンプルでは、`Person` 構造体に `first_name` と `last_name` というフィールドがあり、それぞれのゲッターとセッターを定義しています。これにより、C#のプロパティに似た振る舞いを実現しています。Rustでは、このように構造体とメソッドを組み合わせることでデータとその操作をカプセル化します。

