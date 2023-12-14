use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct LargeData {
    data: [u8; 1000],
}

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Weak<List>>),
    Nil,
}

#[derive(Debug)]
struct Person {
    name: String,
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

fn borrow(weak: &Weak<i32>) {
    match weak.upgrade() {
        Some(value) => println!("Ownership borrowed: {}", value),
        None => println!("No value"),
    }
}

fn main() {
    // 1. 基本的な使用
    let rc = Rc::new(5);
    let weak = Rc::downgrade(&rc);

    match weak.upgrade() {
        Some(value) => println!("Basic use: {}", value),
        None => println!("No value"),
    }

    // 2. 大きなデータ構造を扱う
    let rc = Rc::new(LargeData { data: [0; 1000] });
    let weak = Rc::downgrade(&rc);

    if let Some(large_data) = weak.upgrade() {
        println!("Handling large data structure: {:?}", large_data.data[0]);
    }

    // 3. 配列を扱う
    let rc = Rc::new([1, 2, 3, 4, 5]);
    let weak = Rc::downgrade(&rc);

    if let Some(array) = weak.upgrade() {
        println!("Array handling: {:?}", array);
    }

    // 4. 再帰的なデータ構造
    let list = Rc::new(List::Cons(1, RefCell::new(Weak::new())));
    let weak_list = Rc::downgrade(&list);

    if let Some(list) = weak_list.upgrade() {
        println!("Recursive data structure: {:?}", list);
    }

    // 5. 構造体の使用
    let person = Rc::new(Person {
        name: String::from("Alice"),
    });
    let weak_person = Rc::downgrade(&person);

    if let Some(person) = weak_person.upgrade() {
        println!("Struct usage: {}", person.name);
    }

    // 6. 構造体の配列の使用
    let people = Rc::new([
        Person {
            name: String::from("Alice"),
        },
        Person {
            name: String::from("Bob"),
        },
    ]);
    let weak_people = Rc::downgrade(&people);

    if let Some(people) = weak_people.upgrade() {
        println!("Array of structs: {:?}", people);
    }

    // 7. トレイトオブジェクト
    let animal: Rc<dyn Speak> = Rc::new(Dog);
    let weak_animal = Rc::downgrade(&animal);

    if let Some(animal) = weak_animal.upgrade() {
        animal.speak();
    }

    // 8. 所有権の移動
    let rc = Rc::new(10);
    let weak = Rc::downgrade(&rc); // RcからWeakへ変換
    println!("Ownership moved to weak reference");

    // 9. 所有権の貸し出し
    let rc = Rc::new(10);
    let weak = Rc::downgrade(&rc);
    borrow(&weak);

    // 10. 固定データの参照
    let rc = Rc::new(10);
    let weak = Rc::downgrade(&rc);

    if let Some(reference) = weak.upgrade() {
        println!("Immutable reference: {}", reference);
    }
}
