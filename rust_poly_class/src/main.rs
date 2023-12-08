// 共通のトレイト（インターフェース）を定義
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

fn animal_noise(animal: &dyn Animal) {
    println!("The animal says: {}", animal.make_noise());
}

fn main() {
    let dog = Dog;
    let cat = Cat;

    // トレイトオブジェクトを使用して異なる型に対して同じ関数を呼び出す
    animal_noise(&dog);
    animal_noise(&cat);
}
