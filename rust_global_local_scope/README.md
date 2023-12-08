# ファイル内グローバル変数と関数、ファイル外グローバル変数と関数
Rustにおいて、あるファイル内で定義されたプライベートなグローバル変数や関数は他のファイルから直接アクセスすることができません。以下のサンプルでは、この特性を示すとともに、ファイル外の公開されたグローバル変数に対して読み書きを行う例を示します。

## プロジェクト構造
- src/main.rs
- src/globals.rs

**src/globals.rs**
このファイルでは、公開されたグローバル変数と関数、および非公開のグローバル変数と関数を定義します。
```rust
// src/globals.rs

// 公開された変更可能なグローバル変数
pub static mut GLOBAL_VAR: i32 = 0;

// 非公開のグローバル変数
static PRIVATE_GLOBAL_VAR: i32 = 100;

pub fn increment_global_var() {
    unsafe {
        GLOBAL_VAR += 1;
    }
}

pub fn get_global_var() -> i32 {
    unsafe {
        GLOBAL_VAR
    }
}

fn get_private_global_var() -> i32 {
    PRIVATE_GLOBAL_VAR
}
```

**src/main.rs**
メインファイルでは、`globals.rs` で定義された公開されたグローバル変数と関数にアクセスし、非公開のグローバル変数と関数にはアクセスできないことを示します。
```rust
// src/main.rs
mod globals;

fn main() {
    // 公開された変更可能なグローバル変数にアクセスして値を変更
    globals::increment_global_var();
    println!("Global Var: {}", globals::get_global_var());

    // 非公開のグローバル変数には直接アクセスできない
    // 以下の行をコメントアウト解除するとコンパイルエラーになります
    // println!("Private Global Var: {}", globals::get_private_global_var());
}
```

## 手順
**1. 新しいプロジェクトを作成:**
- `cargo new rust_global_local_scope` コマンドを使って新しいプロジェクトを作成します。

**2. ファイルの作成と編集:**
- `src/globals.rs` と `src/main.rs` に上記のコードを記述します。

**3. ビルドと実行:**
- `cargo build` でプロジェクトをビルドし、`cargo run` でプログラムを実行します。

このサンプルでは、`main.rs` から `globals.rs` に定義された公開された変更可能なグローバル変数にアクセスしています。一方、非公開のグローバル変数や関数には直接アクセスすることはできません。これはRustのプライバシー規則によるもので、モジュール内の要素はデフォルトでプライベートです。公開された要素にのみ外部からアクセスすることができます。
