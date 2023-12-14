// src/globals.rs

// 公開された変更可能なグローバル変数
pub static mut GLOBAL_VAR: i32 = 0;

// 非公開のグローバル変数
static _PRIVATE_GLOBAL_VAR: i32 = 100;

pub fn increment_global_var() {
    unsafe {
        GLOBAL_VAR += 1;
    }
}

pub fn get_global_var() -> i32 {
    unsafe { GLOBAL_VAR }
}

fn _get_private_global_var() -> i32 {
    _PRIVATE_GLOBAL_VAR
}
