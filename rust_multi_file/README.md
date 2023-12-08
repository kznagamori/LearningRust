# 複数ファイルを使用する

Rustで複数ファイルを使用する際の一般的なアプローチは、関数やモジュールを分離して各ファイルに配置することです。以下に、簡単な例を示します。この例では、メインの実行可能ファイル `main.rs` と、別のファイル `greetings.rs` を使用します。

## ファイル構造
- `src/main.rs`
- `src/greetings.rs`

**src/greetings.rs**
このファイルでは、あいさつを行う関数を定義します。
```rust
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

**src/main.rs**
メインファイルでは、`greetings` モジュールをインポートし、その関数を使用します。
```rust
mod greetings;

fn main() {
    let name = "World";
    let greeting = greetings::greet(name);
    println!("{}", greeting);
}
```

## 手順
1. プロジェクトの作成: `cargo new rust_multi_file` コマンドを使って新しいプロジェクトを作成します。
2. ファイルの作成と編集: 上記のように `src/main.rs` と `src/greetings.rs` ファイルを作成し、それぞれのコードを記述します。
3. ビルドと実行: プロジェクトのルートディレクトリで `cargo build` と `cargo run` コマンドを実行して、プログラムをビルドし実行します。


