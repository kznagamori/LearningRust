use std::rc::Rc;

struct LargeData {
    data: [u8; 1000],
}

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
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

fn borrow(rc_num: &Rc<i32>) {
    println!("Ownership borrowed: {}", rc_num);
}

fn main() {
    // 1. 基本的な使用
    let rc_int = Rc::new(5);
    println!("Basic use: {}", rc_int);

    // 2. 大きなデータ構造を扱う
    let large_data = Rc::new(LargeData { data: [0; 1000] });
    println!("Handling large data structure: {:?}", large_data.data[0]);

    // 3. 配列を扱う
    let rc_array = Rc::new([1, 2, 3, 4, 5]);
    println!("Array handling: {:?}", rc_array);

    // 4. 再帰的なデータ構造
    let list = Rc::new(List::Cons(1, Rc::new(List::Cons(2, Rc::new(List::Nil)))));
    println!("Recursive data structure: {:?}", list);

    // 5. 構造体の使用
    let person = Rc::new(Person {
        name: String::from("Alice"),
        age: 30,
    });
    println!("Struct usage: {} is {}", person.name, person.age);

    // 6. 構造体の配列の使用
    let people = Rc::new([
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
    let animal: Rc<dyn Speak> = Rc::new(Dog);
    animal.speak();

    // 8. 所有権の移動
    let rc_num = Rc::new(10);
    let rc_num_clone = Rc::clone(&rc_num);
    println!("Ownership shared: {}", rc_num_clone);

    // 9. 所有権の貸し出し
    let rc_num = Rc::new(10);
    borrow(&rc_num);

    // 10. 固定データの参照
    let rc_num = Rc::new(10);
    let reference: &i32 = &rc_num;
    println!("Immutable reference: {}", reference);
}
