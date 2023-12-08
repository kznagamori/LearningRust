# 複数ファイルを機能ごとにクレートを分けて使用する

Rustでは、クレートを分けて複数の機能を管理することが一般的です。以下は、ライブラリクレートと実行可能なバイナリクレートを別々のプロジェクトとして作成し、相互に使用するための手順とサンプルコードです。

## ライブラリクレートの作成
**1. 新しいライブラリクレートを作成:**
- ターミナルで次のコマンドを実行します:
```bash
cargo new my_library --lib
```
- これにより my_library という名前の新しいライブラリクレートが作成されます。

**2. ライブラリクレートの編集:**
- `src/lib.rs` ファイルを開き、関数やモジュールを定義します。
```rust
// src/lib.rs
pub fn greet(name: &str) -> String {
    format!("Hello, {} from the library!", name)
}
```
**3. Cargo.tomlの確認:**
- `Cargo.toml` には、クレートの名前とバージョンが記載されていることを確認します。
```
[package]
name = "my_library"
version = "0.1.0"
edition = "2021"

[dependencies]
```

## 実行可能クレートの作成
**1. 新しいバイナリクレートを作成:**
- 別のディレクトリで次のコマンドを実行します:
```bash
cargo new my_app
```
- これにより `my_app`` という名前の新しい実行可能クレートが作成されます。

**2.ライブラリクレートの依存関係を追加:**
- `my_app` ディレクトリの `Cargo.toml` を開き、`my_library` クレートへのパスを依存関係に追加します。
```
[dependencies]
my_library = { path = "../my_library" }
```

**3. バイナリクレートの編集:**
- `src/main.rs` ファイルを開き、`my_library` クレートの関数を使用します。
```rust
// src/main.rs
use my_library::greet;

fn main() {
    println!("{}", greet("World"));
}
```

## ビルドと実行
- **ライブラリクレート**:
  - `my_library` ディレクトリで `cargo build` を実行してライブラリをビルドします。
- **実行可能クレート**:
  - `my_app` ディレクトリで `cargo run` を実行してアプリケーションをビルドし実行します。

このプロセスにより、`my_library` ライブラリクレートの機能を my_app バイナリクレートで利用することができます。`my_library` クレートをローカルファイルシステム上で参照するために、`Cargo.toml` ファイルで `path` 属性を使用しています。これにより、外部のクレートレジストリに公開することなく、ローカルでクレートを共有することが可能です。

