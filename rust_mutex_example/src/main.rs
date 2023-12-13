use std::sync::{Arc, Mutex};
use std::thread;

// 共有されるデータの構造体
struct SharedData {
    count: i32,
}

fn main() {
    // ミューテックスでラップされた構造体を作成
    let data = Arc::new(Mutex::new(SharedData { count: 0 }));

    let mut handles = vec![];

    for _ in 0..5 {
        // Arcをクローンして各スレッドに渡す
        let data = Arc::clone(&data);

        let handle = thread::spawn(move || {
            let mut data = data.lock().unwrap();
            // データのcountフィールドを変更
            data.count += 1;
        });

        handles.push(handle);
    }

    // すべてのスレッドの終了を待つ
    for handle in handles {
        handle.join().unwrap();
    }

    // 最終的なデータを表示
    println!("Result: {}", data.lock().unwrap().count);
}
