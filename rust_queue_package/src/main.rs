use std::collections::VecDeque;

fn main() {
    let mut queue = VecDeque::new();

    // キューに要素を追加
    queue.push_back(1);
    queue.push_back(2);
    queue.push_back(3);

    // キューから要素を取り出し
    while let Some(value) = queue.pop_front() {
        println!("Dequeued: {}", value);
    }
}
