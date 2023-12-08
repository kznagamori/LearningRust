# ジェネリック機能

Rustでは、「クラス」や「メソッド」のジェネリックを構造体（ `struct` ）や関数（`fn` ）のジェネリック型パラメータを使って実現します。ジェネリック型を使うことで、異なるデータ型に対して同じ構造体や関数を再利用できます。

以下に、ジェネリック型を使用した構造体と関数の実装例を示します。

## サンプルコード
**src/main.rs**
```rust
// ジェネリック型を持つ構造体
struct Point<T> {
    x: T,
    y: T,
}

// ジェネリック型を持つメソッド
impl<T> Point<T> {
    fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }

    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

// ジェネリック型を持つ関数
fn print_point<T: std::fmt::Display>(point: &Point<T>) {
    println!("Point: ({}, {})", point.x(), point.y());
}

fn main() {
    let point_int = Point::new(5, 10);
    let point_float = Point::new(1.2, 3.4);

    print_point(&point_int);
    print_point(&point_float);
}
```

この例では、ジェネリック型 `T` を持つ `Point` 構造体と、そのメソッド `new` 、`x` 、`y` が定義されています。また、`print_point` 関数は任意の `Point<T>` を引数として受け取り、その内容を表示します。

## 手順
1. **新しいプロジェクトを作成:**
- ターミナルで `cargo new rust_generics_example`` コマンドを実行して新しいプロジェクトを作成します。

2. **ファイルの編集:**
- `src/main.rs` ファイルを開き、上記のサンプルコードを記述します。

3. **ビルドと実行:**
- プロジェクトのルートディレクトリで `cargo build` コマンドを実行してプロジェクトをビルドします。
- ビルドが完了したら `cargo run` コマンドを実行してプログラムを実行します。

このサンプルコードは、Rustでのジェネリック型の基本的な使い方を示しており、さまざまなデータ型で再利用可能なコードを作成するのに役立ちます。