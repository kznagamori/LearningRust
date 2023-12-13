use std::thread;
use std::time::Duration;

// スレッドで実行する関数
fn thread_function(id: u32) {
    println!("Thread {} is starting", id);
    // スレッドが何かの作業を行っていることをシミュレート
    thread::sleep(Duration::from_millis(500));
    println!("Thread {} has finished", id);
}

fn main() {
    let mut handles = vec![];

    for i in 0..5 {
        // 新しいスレッドを生成し、定義された関数を実行
        let handle = thread::spawn(move || {
            thread_function(i);
        });
        handles.push(handle);
    }

    // すべてのスレッドが終了するのを待つ
    for handle in handles {
        handle.join().unwrap();
    }

    println!("All threads have finished.");
}
