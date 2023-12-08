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
