use std::collections::LinkedList;

fn main() {
    // 新しいリンクドリストを作成
    let mut list = LinkedList::new();

    // 要素をリストの末尾に追加
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    // リストの先頭に要素を追加
    list.push_front(0);

    // リンクドリストの要素をイテレートして表示
    for element in list.iter() {
        println!("{}", element);
    }

    // リストの長さを表示
    println!("Length of the list: {}", list.len());
}
