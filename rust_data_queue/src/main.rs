struct Queue<T> {
    front: Vec<T>,
    back: Vec<T>,
}

impl<T> Queue<T> {
    // 新しいキューを生成する関数
    fn new() -> Self {
        Queue {
            front: Vec::new(),
            back: Vec::new(),
        }
    }

    // キューの背面に要素を追加する関数
    fn enqueue(&mut self, item: T) {
        self.back.push(item);
    }

    // キューの前面から要素を取り出す関数
    fn dequeue(&mut self) -> Option<T> {
        if self.front.is_empty() {
            // 前面が空の場合、背面を逆順にして前面に移動します
            if self.back.is_empty() {
                return None;
            }
            std::mem::swap(&mut self.front, &mut self.back);
            self.front.reverse();
        }

        // 前面から要素を取り出します
        self.front.pop()
    }
}

fn main() {
    let mut queue = Queue::new();

    // キューに要素を追加します
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);

    // キューから要素を取り出し、表示します
    while let Some(value) = queue.dequeue() {
        println!("{}", value);
    }
}
