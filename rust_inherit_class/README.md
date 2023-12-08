# クラスの継承を実現する

Rustではクラスベースの継承がサポートされていませんが、構造体の合成（ `composition` ）やトレイト（ `trait` ）を用いて似たような振る舞いを実現することができます。以下の例では、トレイトと合成を使って継承のような機能を実現します。

## プロジェクト構造
- `trait_example`: トレイトを使った例
- `composition_example`: 合成を使った例


## トレイトを使った例
トレイトを使って共通のインターフェースを定義し、異なる構造体でこのインターフェースを実装する方法を示します。

**src/main.rs**
```rust
// 共通のトレイトを定義
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

fn main() {
    let dog = Dog;
    let cat = Cat;

    // トレイトを通じて動物の鳴き声を出力
    println!("Dog says: {}", dog.make_noise());
    println!("Cat says: {}", cat.make_noise());
}
```
### 手順
1. **新しいプロジェクトを作成:**
- ターミナルで `cargo new trait_example` コマンドを実行して新しいプロジェクトを作成します。

2. **ファイルの編集:**
- `src/main.rs` ファイルを開き、上記のサンプルコードのいずれかを記述します。

3. **ビルドと実行:**
- プロジェクトのルートディレクトリで `cargo build` コマンドを実行してプロジェクトをビルドします。
- ビルドが完了したら `cargo run` コマンドを実行してプログラムを実行します。


## 合成を使った例
基本となる構造体を定義し、他の構造体がこの構造体を内包することで継承のような機能を実現します。

**src/main.rs**
```rust
// 基本となる構造体
struct Animal {
    name: String,
    noise: String,
}

impl Animal {
    fn new(name: String, noise: String) -> Animal {
        Animal { name, noise }
    }

    fn make_noise(&self) -> String {
        self.noise.clone()
    }
}

// Dog 構造体
struct Dog {
    animal: Animal,
}

impl Dog {
    fn new() -> Dog {
        Dog {
            animal: Animal::new("Dog".to_string(), "Bark!".to_string()),
        }
    }

    fn make_noise(&self) -> String {
        self.animal.make_noise()
    }
}

// Cat 構造体
struct Cat {
    animal: Animal,
}

impl Cat {
    fn new() -> Cat {
        Cat {
            animal: Animal::new("Cat".to_string(), "Meow!".to_string()),
        }
    }

    fn make_noise(&self) -> String {
        self.animal.make_noise()
    }
}

fn main() {
    let dog = Dog::new();
    let cat = Cat::new();

    println!("Dog says: {}", dog.make_noise());
    println!("Cat says: {}", cat.make_noise());
}
```

### 手順
1. **新しいプロジェクトを作成:**
- ターミナルで `cargo new composition_example` コマンドを実行して新しいプロジェクトを作成します。

2. **ファイルの編集:**
- `src/main.rs` ファイルを開き、上記のサンプルコードのいずれかを記述します。

3. **ビルドと実行:**
- プロジェクトのルートディレクトリで `cargo build` コマンドを実行してプロジェクトをビルドします。
- ビルドが完了したら `cargo run` コマンドを実行してプログラムを実行します。


これらのサンプルでは、Rustの継承のような機能をトレイトと合成を使用して実現しています。Rustの哲学においては、合成を優先し、トレイトを使って共通の振る舞いを抽象化することが推奨されます。

