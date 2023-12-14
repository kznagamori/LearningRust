struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    // 新しいキューを生成する関数
    fn new() -> Self {
        Queue {
            elements: Vec::new(),
        }
    }

    // キューの背面に要素を追加する関数
    fn enqueue(&mut self, item: T) {
        self.elements.push(item);
    }

    // キューの前面から要素を取り出す関数
    fn dequeue(&mut self) -> Option<T> {
        if self.elements.len() == 0 {
            return None;
        }
        // 前面から要素を取り出します
        // remove()は、T型を返す
        let value = self.elements.remove(0);
        // valueはT型なのでSome(T)でOption<T>型に変換して返す
        return Some(value);
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
