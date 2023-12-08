struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // コンストラクタ
    fn new(first_name: String, last_name: String) -> Person {
        Person {
            first_name,
            last_name,
        }
    }

    // first_name のゲッター
    fn first_name(&self) -> &str {
        &self.first_name
    }

    // first_name のセッター
    fn set_first_name(&mut self, first_name: String) {
        self.first_name = first_name;
    }

    // last_name のゲッター
    fn last_name(&self) -> &str {
        &self.last_name
    }

    // last_name のセッター
    fn set_last_name(&mut self, last_name: String) {
        self.last_name = last_name;
    }

    // フルネームを返すメソッド
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

fn main() {
    let mut person = Person::new("Alice".to_string(), "Smith".to_string());

    // ゲッターを使用してフィールドの値を取得
    println!("First Name: {}", person.first_name());
    println!("Last Name: {}", person.last_name());

    // フルネームを表示
    println!("Full Name: {}", person.full_name());

    // セッターを使用してフィールドの値を変更
    person.set_first_name("Bob".to_string());
    person.set_last_name("Johnson".to_string());

    // 変更後のフルネームを表示
    println!("Full Name: {}", person.full_name());
}
