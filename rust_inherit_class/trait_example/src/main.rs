// 共通のトレイトを定義
trait Animal {
    fn make_noise(&self) -> String;
}

// Dog 構造体の定義
struct Dog;

// Dog で Animal トレイトを実装
impl Animal for Dog {
    fn make_noise(&self) -> String {
        "Bark!".to_string()
    }
}

// Cat 構造体の定義
struct Cat;

// Cat で Animal トレイトを実装
impl Animal for Cat {
    fn make_noise(&self) -> String {
        "Meow!".to_string()
    }
}

fn main() {
    let dog = Dog;
    let cat = Cat;

    // トレイトを通じて動物の鳴き声を出力
    println!("Dog says: {}", dog.make_noise());
    println!("Cat says: {}", cat.make_noise());
}
