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
