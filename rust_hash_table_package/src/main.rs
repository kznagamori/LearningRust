use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    // HashMapにキーと値を追加
    map.insert("apple", 3);
    map.insert("banana", 2);
    map.insert("orange", 4);

    // キーに対する値を取得して表示
    if let Some(&number) = map.get("apple") {
        println!("There are {} apples.", number);
    }

    // HashMapのキーと値を順に表示
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    // 特定のキーの存在を確認
    if map.contains_key("banana") {
        println!("There are bananas in the map.");
    } else {
        println!("There are no bananas in the map.");
    }
}
