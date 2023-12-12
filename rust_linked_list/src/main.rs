// ジェネリック型Tを持つNode構造体を定義します。
// 各ノードは、値（value）と次のノードへのオプショナルなポインタ（next）を持ちます。
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

// ジェネリック型Tを持つLinkedList構造体を定義します。
// この構造体はリストの先頭ノードへのオプショナルなポインタ（head）を持ちます。
struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

// LinkedListの実装
impl<T> LinkedList<T> {
    // 新しい空のリストを作成するためのnew関数。
    // headは初期状態でNone（空のリスト）です。
    fn new() -> Self {
        LinkedList { head: None }
    }

    // リストの先頭に新しい要素を追加するためのpush関数。
    fn push(&mut self, value: T) {
        // 新しいノードを作成し、そのnextを現在のheadに設定します。
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        // 新しいノードをリストの新しいheadとして設定します。
        self.head = Some(new_node);
    }

    // リストから要素を取り出すためのpop関数。
    fn pop(&mut self) -> Option<T> {
        // 現在のheadを取り出し、その値を返す。
        // この操作は、現在のheadがSomeの場合にのみ実行されます。
        self.head.take().map(|node| {
            // 新しいheadを次のノードに設定します。
            self.head = node.next;
            // 取り出されたノードの値を返します。
            node.value
        })
    }
}

fn main() {
    // 新しいLinkedListを作成します。
    let mut list = LinkedList::new();
    // リストに要素を追加します。
    list.push(1);
    list.push(2);
    list.push(3);

    // リストからすべての要素を取り出し、表示します。
    while let Some(value) = list.pop() {
        println!("{}", value);
    }
}
