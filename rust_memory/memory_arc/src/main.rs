use std::sync::{Arc, Mutex};

struct LargeData {
    data: [u8; 1000],
}

#[derive(Debug)]
enum List {
    Cons(i32, Arc<Mutex<List>>),
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

fn consume(arc: Arc<i32>) {
    println!("Ownership moved: {}", arc);
}

fn borrow(arc: &Arc<i32>) {
    println!("Ownership borrowed: {}", arc);
}

fn main() {
    // 1. 基本的な使用
    let arc = Arc::new(5);
    let arc_clone = arc.clone();

    println!("Basic use: {}", arc_clone);

    // 2. 大きなデータ構造を扱う
    let large_data = Arc::new(LargeData { data: [0; 1000] });
    let large_data_clone = large_data.clone();

    println!(
        "Handling large data structure: {:?}",
        large_data_clone.data[0]
    );

    // 3. 配列を扱う
    let arc_array = Arc::new([1, 2, 3, 4, 5]);
    let arc_array_clone = arc_array.clone();

    println!("Array handling: {:?}", arc_array_clone);

    // 4. 再帰的なデータ構造
    let list = Arc::new(Mutex::new(List::Cons(1, Arc::new(Mutex::new(List::Nil)))));
    let list_clone = list.clone();

    println!("Recursive data structure: {:?}", list_clone.lock().unwrap());

    // 5. 構造体の使用
    let person = Arc::new(Person {
        name: String::from("Alice"),
        age: 30,
    });
    let person_clone = person.clone();

    println!(
        "Struct usage: {} is {}",
        person_clone.name, person_clone.age
    );

    // 6. 構造体の配列の使用
    let people = Arc::new([
        Person {
            name: String::from("Alice"),
            age: 30,
        },
        Person {
            name: String::from("Bob"),
            age: 25,
        },
    ]);
    let people_clone = people.clone();

    println!("Array of structs: {:?}", people_clone);

    // 7. トレイトオブジェクト
    let animal: Arc<dyn Speak> = Arc::new(Dog);
    let animal_clone = animal.clone();

    animal_clone.speak();

    // 8. 所有権の移動
    let arc_num = Arc::new(10);
    consume(arc_num);

    // 9. 所有権の貸し出し
    let arc_num = Arc::new(10);
    borrow(&arc_num);

    // 10. 固定データの参照
    let arc_num = Arc::new(10);
    let reference = Arc::clone(&arc_num);

    println!("Immutable reference: {}", reference);
}
