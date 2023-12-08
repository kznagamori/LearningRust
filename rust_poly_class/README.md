# クラスのインタフェースを使用したポリモーフィズムの実現

Rustでは、ポリモーフィズムを実現するためにトレイト（ `trait` ）を使用します。トレイトは他の言語のインターフェースに似ており、異なる型に共通の振る舞いを提供します。以下の例では、異なる構造体に対して共通のトレイトを実装し、トレイトオブジェクトを使用してポリモーフィズムを実現します。

## サンプルコード
**src/main.rs**
```rust
// 共通のトレイト（インターフェース）を定義
trait Animal {
    fn make_noise(&self) -> String;
}

// Dog 構造体の定義
struct Dog;

// Dog で Animal トレイトを実装
impl Animal for Dog {
    fn make_noise(&self) -> String {
        "Bark!".to_string()
    }
}

// Cat 構造体の定義
struct Cat;

// Cat で Animal トレイトを実装
impl Animal for Cat {
    fn make_noise(&self) -> String {
        "Meow!".to_string()
    }
}

fn animal_noise(animal: &dyn Animal) {
    println!("The animal says: {}", animal.make_noise());
}

fn main() {
    let dog = Dog;
    let cat = Cat;

    // トレイトオブジェクトを使用して異なる型に対して同じ関数を呼び出す
    animal_noise(&dog);
    animal_noise(&cat);
}
```

## 手順
1. **新しいプロジェクトを作成:**
- ターミナルで `cargo new rust_poly_class` コマンドを実行して新しいプロジェクトを作成します。

2. **ファイルの編集:**
- `src/main.rs` ファイルを開き、上記のサンプルコードを記述します。

3. **ビルドと実行:**
- プロジェクトのルートディレクトリで `cargo build` コマンドを実行してプロジェクトをビルドします。
- ビルドが完了したら `cargo run` コマンドを実行してプログラムを実行します。

このサンプルでは、`Animal` という共通のトレイトを定義し、`Dog` と `Cat` 構造体にこのトレイトを実装しています。`animal_noise` 関数は任意の `Animal` トレイトを実装する型の参照を受け取り、その型の `make_noise` メソッドを呼び出します。これにより、異なる型に対して共通のインターフェースを通じて操作を行うポリモーフィズムを実現しています。
