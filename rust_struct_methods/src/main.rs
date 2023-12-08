// 構造体の定義
struct Person {
    name: String,
    age: u8,
}

// Person 構造体に対するメソッドの実装
impl Person {
    // 新しい Person を作成する関数
    fn new(name: String, age: u8) -> Person {
        Person { name, age }
    }

    // Person の自己紹介を行うメソッド
    fn introduce(&self) {
        println!(
            "Hello, my name is {} and I am {} years old.",
            self.name, self.age
        );
    }
}

fn main() {
    // Person 構造体のインスタンスを作成
    let person = Person::new("Alice".to_string(), 30);

    // メソッドを呼び出し
    person.introduce();
}
