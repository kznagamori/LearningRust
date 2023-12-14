// src/globals.rs
use std::sync::Mutex;

lazy_static! {
    pub static ref MUTABLE_GLOBAL_VAR: Mutex<i32> = Mutex::new(0);
}

static _PRIVATE_GLOBAL_VAR: i32 = 100;

pub fn increment_global_var() {
    let mut num = MUTABLE_GLOBAL_VAR.lock().unwrap();
    *num += 1;
}

pub fn get_mutable_global_var() -> i32 {
    *MUTABLE_GLOBAL_VAR.lock().unwrap()
}

// この関数はプライベートグローバル変数にアクセスするが、外部からは呼び出せない
fn _get_private_global_var() -> i32 {
    _PRIVATE_GLOBAL_VAR
}
