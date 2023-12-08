# ファイル内グローバル変数と関数、ファイル外グローバル変数と関数(アトミック)

グローバル変数の扱いは他の多くの言語とは異なります。Rustのグローバル変数は、通常、`static` キーワードを用いて定義され、不変（ `immutable` ）です。変更可能な（ `mutable` ）グローバル変数を扱う場合は、スレッドセーフな方法でアクセスするために特別な注意が必要です。これは通常、`Mutex` や `Atomic` 型を用いて行われます。

以下の例では、ファイル内とファイル外のグローバル変数を扱う方法を示します。また、変更可能なグローバル変数にアクセスする方法も含まれます。

## プロジェクト構造
- src/main.rs
- src/globals.rs

**src/globals.rs**
このファイルでは、公開された変更可能なグローバル変数（ `MUTABLE_GLOBAL_VAR` ）と公開されていないグローバル変数（ `PRIVATE_GLOBAL_VAR`）を定義します。また、これらの変数にアクセスするための公開関数も提供します。
```rust
// src/globals.rs
use std::sync::Mutex;

lazy_static! {
    pub static ref MUTABLE_GLOBAL_VAR: Mutex<i32> = Mutex::new(0);
}

static PRIVATE_GLOBAL_VAR: i32 = 100;

pub fn increment_global_var() {
    let mut num = MUTABLE_GLOBAL_VAR.lock().unwrap();
    *num += 1;
}

pub fn get_mutable_global_var() -> i32 {
    *MUTABLE_GLOBAL_VAR.lock().unwrap()
}

// この関数はプライベートグローバル変数にアクセスするが、外部からは呼び出せない
fn get_private_global_var() -> i32 {
    PRIVATE_GLOBAL_VAR
}
```

**src/main.rs**
メインファイルでは、`globals.rs` で定義された公開されたグローバル変数と関数にアクセスし、非公開のグローバル変数と関数にはアクセスできないことを示します。

```rust
// src/main.rs
#[macro_use]
extern crate lazy_static;

mod globals;

fn main() {
    // 公開された変更可能なグローバル変数にアクセスして値を変更
    globals::increment_global_var();
    println!("Mutable Global Var: {}", globals::get_mutable_global_var());

    // 非公開のグローバル変数には直接アクセスできない（以下の行はエラーを引き起こすためコメントアウト）
    // println!("Private Global Var: {}", globals::PRIVATE_GLOBAL_VAR);
    // println!("Private Global Var: {}", globals::get_private_global_var());
}
```

## Cargo.tomlの更新
この例では、`lazy_static` クレートを使用して変更可能なグローバル変数を扱っています。`Cargo.toml` ファイルに以下の行を追加してください：
```
[dependencies]
lazy_static = "1.4.0"
```

## 手順
**1. 新しいプロジェクトを作成:**
- cargo new rust_atomic_global_local_scope コマンドを使って新しいプロジェクトを作成します。

**2. ファイルの作成と編集:**
- `src/globals.rs` と `src/main.rs` に上記のコードを記述します。

**3. Cargo.tomlの更新:**
- 必要な依存関係を追加します。

**4. ビルドと実行:**
- `cargo build` でプロジェクトをビルドし、`cargo run` でプログラムを実行します。

このサンプルでは、`main.rs` から `globals.rs` に定義された公開された変更可能なグローバル変数にアクセスし、その値を変更しています。一方で、非公開のグローバル変数や関数には直接アクセスすることはできません。これはRustのアクセス制御の基本的な特徴です。

