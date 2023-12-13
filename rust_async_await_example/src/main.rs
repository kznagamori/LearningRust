use async_std::task;
use std::time::Duration;

// 非同期関数
async fn do_something(id: u32) {
    println!("Task {} is starting", id);
    // 何かの処理をシミュレートするために非同期で待機
    task::sleep(Duration::from_secs(2)).await;
    println!("Task {} is done", id);
}

#[async_std::main]
async fn main() {
    // 複数の非同期タスクを生成して実行
    let task1 = task::spawn(do_something(1));
    let task2 = task::spawn(do_something(2));

    // すべてのタスクが完了するのを待つ
    task1.await;
    task2.await;

    println!("All tasks completed.");
}
