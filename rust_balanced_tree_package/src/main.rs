use ABtree::AVL;

fn main() {
    let mut tree = AVL::<i32, &str>::new();

    // AVL木に要素を追加
    tree.insert(3, "three");
    tree.insert(1, "one");
    tree.insert(4, "four");
    tree.insert(2, "two");

    // AVL木を走査して要素を表示
    for (_, value) in tree.iter() {
        println!("{}", value);
    }

    // 特定のキーを検索
    if tree.contains(&2) {
        println!("Tree contains key 2");
    } else {
        println!("Tree does not contain key 2");
    }
}
