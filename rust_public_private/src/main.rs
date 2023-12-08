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
