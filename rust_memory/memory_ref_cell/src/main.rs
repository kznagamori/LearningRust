use std::cell::RefCell;
use std::rc::Rc;

struct LargeData {
    data: [u8; 1000],
}

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

#[derive(Debug)]
struct Person {
    name: String,
    age: RefCell<u8>,
}

trait Speak {
    fn speak(&self);
    fn set_volume(&self, volume: u32);
}

struct Dog {
    volume: RefCell<u32>,
}

impl Speak for Dog {
    fn speak(&self) {
        println!("Woof at volume {}", self.volume.borrow());
    }

    fn set_volume(&self, volume: u32) {
        *self.volume.borrow_mut() = volume;
    }
}

fn consume(cell: RefCell<i32>) {
    println!("Ownership moved: {}", cell.borrow());
}

fn borrow(cell: &RefCell<i32>) {
    println!("Ownership borrowed: {}", cell.borrow());
}

fn main() {
    // 1. 基本的な使用
    let cell = RefCell::new(5);
    *cell.borrow_mut() += 1; // 値を変更
    println!("Basic use: {}", cell.borrow());

    // 2. 大きなデータ構造を扱う
    let large_data = RefCell::new(LargeData { data: [0; 1000] });
    large_data.borrow_mut().data[0] = 1; // データを変更
    println!(
        "Handling large data structure: {:?}",
        large_data.borrow().data[0]
    );

    // 3. 配列を扱う
    let cell_array = RefCell::new([1, 2, 3, 4, 5]);
    cell_array.borrow_mut()[0] = 10; // 配列の要素を変更
    println!("Array handling: {:?}", cell_array.borrow());

    // 4. 再帰的なデータ構造
    let list = Rc::new(List::Cons(1, RefCell::new(Rc::new(List::Nil))));
    println!("Recursive data structure: {:?}", list);

    // 5. 構造体の使用
    let person = Person {
        name: String::from("Alice"),
        age: RefCell::new(30),
    };
    *person.age.borrow_mut() += 1; // 年齢を変更
    println!("Struct usage: {} is {}", person.name, person.age.borrow());

    // 6. 構造体の配列の使用
    let people = RefCell::new([
        Person {
            name: String::from("Alice"),
            age: RefCell::new(30),
        },
        Person {
            name: String::from("Bob"),
            age: RefCell::new(25),
        },
    ]);
    people.borrow_mut()[0].age.replace(31); // Aliceの年齢を変更
    println!("Array of structs: {:?}", people.borrow());

    // 7. トレイトオブジェクト
    let dog = Dog {
        volume: RefCell::new(5),
    };
    dog.set_volume(10);
    dog.speak();

    // 8. 所有権の移動
    let cell = RefCell::new(10);
    consume(cell);

    // 9. 所有権の貸し出し
    let cell = RefCell::new(10);
    borrow(&cell);

    // 10. 固定データの参照
    let cell = RefCell::new(10);
    let reference = cell.borrow(); // 不変の借用
    println!("Immutable reference: {}", reference);

    // 11. 変更データの参照
    let cell = RefCell::new(10);
    let mut reference = cell.borrow_mut(); // 可変の借用
    *reference += 5;
    println!("Mutable reference: {}", reference);
}
