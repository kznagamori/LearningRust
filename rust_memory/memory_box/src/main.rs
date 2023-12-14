struct LargeData {
    data: [u8; 1000],
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

trait Speak {
    fn speak(&self);
}

struct Dog;

impl Speak for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

fn consume(boxed: Box<i32>) {
    println!("Ownership moved: {}", boxed);
}

fn borrow(boxed: &Box<i32>) {
    println!("Ownership borrowed: {}", boxed);
}

fn main() {
    // 1. 基本的な使用
    let boxed_int = Box::new(5);
    println!("Basic use: {}", boxed_int);

    // 2. 大きなデータ構造を扱う
    let large_data = Box::new(LargeData { data: [0; 1000] });
    println!("Handling large data structure: {:?}", large_data.data[0]);

    // 3. 配列を扱う
    let boxed_array = Box::new([1, 2, 3, 4, 5]);
    println!("Array handling: {:?}", boxed_array);

    // 4. 再帰的なデータ構造
    let list = Box::new(List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil)))));
    println!("Recursive data structure: {:?}", list);

    // 5. 構造体の使用
    let person = Box::new(Person {
        name: String::from("Alice"),
        age: 30,
    });
    println!("Struct usage: {} is {}", person.name, person.age);

    // 6. 構造体の配列の使用
    let people = Box::new([
        Person {
            name: String::from("Alice"),
            age: 30,
        },
        Person {
            name: String::from("Bob"),
            age: 25,
        },
    ]);
    println!("Array of structs: {:?}", people);

    // 7. トレイトオブジェクト
    let animal: Box<dyn Speak> = Box::new(Dog);
    animal.speak();

    // 8. 所有権の移動
    let boxed_num = Box::new(10);
    consume(boxed_num);
    // boxed_numにはアクセスできない

    // 9. 所有権の貸し出し
    let boxed_num = Box::new(10);
    borrow(&boxed_num);

    // 10. 固定データの参照
    let boxed_num = Box::new(10);
    let reference: &i32 = &boxed_num;
    println!("Immutable reference: {}", reference);

    // 11. 変更データの参照
    let mut boxed_num = Box::new(10);
    let reference: &mut i32 = &mut boxed_num;
    *reference += 5;
    println!("Mutable reference: {}", boxed_num);
}
