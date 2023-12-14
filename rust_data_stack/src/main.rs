struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    // 新しいスタックを生成する関数
    fn new() -> Self {
        Stack {
            elements: Vec::new(),
        }
    }

    // スタックに要素を追加する関数
    fn push(&mut self, item: T) {
        self.elements.push(item);
    }

    // スタックから要素を取り出す関数
    fn pop(&mut self) -> Option<T> {
        //Vec::pop()は、Oprion<T>を返す
        self.elements.pop()
    }

    // スタックのトップ要素を参照する関数
    fn peek(&self) -> Option<&T> {
        self.elements.last()
    }
}

fn main() {
    let mut stack = Stack::new();

    // スタックに要素を追加します
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // スタックのトップ要素を表示します
    if let Some(top) = stack.peek() {
        println!("Top: {}", top);
    }

    // スタックから要素を取り出し、表示します
    while let Some(top) = stack.pop() {
        println!("Pop: {}", top);
    }
}
