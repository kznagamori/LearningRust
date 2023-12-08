# パブリックなクラスのメンバ、メソッドを定義とプライベートなクラスのメンバ、メソッドを定義する

Rustでは、「クラス」の代わりに「構造体（ `struct` ）」と「メソッド」を使用してクラスのような機能を実装します。構造体のフィールドやメソッドはデフォルトでプライベートですが、`pub` キーワードを使って公開することができます。以下に、パブリックな構造体とメソッド、プライベートな構造体とメソッドを定義するサンプルコードを示します。

## サンプルコード
**src/main.rs**
```rust
// パブリックな構造体とメソッドを持つ構造体
pub struct PublicStruct {
    pub public_field: String,
    private_field: String,
}

impl PublicStruct {
    // パブリックなコンストラクタ
    pub fn new(public_field: String, private_field: String) -> PublicStruct {
        PublicStruct {
            public_field,
            private_field,
        }
    }

    // パブリックなメソッド
    pub fn show_public(&self) {
        println!("Public field is: {}", self.public_field);
        // プライベートなフィールドもアクセスできる
        println!("Private field is: {}", self.private_field);
    }
}

// プライベートな構造体とメソッドを持つ構造体
struct PrivateStruct {
    field: String,
}

impl PrivateStruct {
    // プライベートなコンストラクタ
    fn new(field: String) -> PrivateStruct {
        PrivateStruct { field }
    }

    // プライベートなメソッド
    fn show(&self) {
        println!("Field is: {}", self.field);
    }
}

fn main() {
    // PublicStruct のインスタンス生成
    let public_struct = PublicStruct::new("Hello".to_string(), "World".to_string());
    public_struct.show_public();

    // PrivateStruct のインスタンス生成はできない（以下の行はエラーになるためコメントアウト）
    // let private_struct = PrivateStruct::new("Secret".to_string());
    // private_struct.show();
}
```

## 手順
1. **新しいプロジェクトを作成:**
- ターミナルで `cargo new rust_public_private` コマンドを実行して新しいプロジェクトを作成します。

2. **ファイルの編集:**
- `src/main.rs` ファイルを開き、上記のサンプルコードを記述します。

3. **ビルドと実行:**
- プロジェクトのルートディレクトリで `cargo build` コマンドを実行してプロジェクトをビルドします。
- ビルドが完了したら `cargo run` コマンドを実行してプログラムを実行します。

このサンプルでは、`PublicStruct` はパブリックなフィールドとメソッドを持ち、外部からアクセス可能です。一方、`PrivateStruct` はプライベートなフィールドとメソッドを持ち、外部からのアクセスはできません。Rustでは、このように `pub` キーワードを使ってアクセスレベルを制御します。

