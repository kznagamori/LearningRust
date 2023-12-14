use std::sync::{Arc, Mutex};
use std::thread;

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

trait Speak: Send + Sync {
    fn speak(&self);
}

struct Dog;

impl Speak for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

fn borrow(arc_mutex: &Arc<Mutex<i32>>) {
    let num = arc_mutex.lock().unwrap();
    println!("Ownership borrowed: {}", num);
}

fn main() {
    // 1. 基本的な使用
    let data = Arc::new(Mutex::new(0));
    let data_clone = Arc::clone(&data);

    let handle = thread::spawn(move || {
        let mut data = data_clone.lock().unwrap();
        *data += 1;
    });

    handle.join().unwrap();

    println!("Basic use: {}", *data.lock().unwrap());

    // 2. 大きなデータ構造を扱う
    let large_data = Arc::new(Mutex::new(LargeData { data: [0; 1000] }));
    let large_data_clone = Arc::clone(&large_data);

    thread::spawn(move || {
        let mut data = large_data_clone.lock().unwrap();
        data.data[0] = 1;
    })
    .join()
    .unwrap();

    println!(
        "Handling large data structure: {:?}",
        large_data.lock().unwrap().data[0]
    );

    // 3. 配列を扱う
    let arc_mutex_array = Arc::new(Mutex::new([1, 2, 3, 4, 5]));
    let arc_mutex_array_clone = Arc::clone(&arc_mutex_array);

    thread::spawn(move || {
        let mut array = arc_mutex_array_clone.lock().unwrap();
        array[0] = 10;
    })
    .join()
    .unwrap();

    println!("Array handling: {:?}", arc_mutex_array.lock().unwrap());

    // 4. 再帰的なデータ構造
    let list = Arc::new(Mutex::new(List::Cons(1, Arc::new(Mutex::new(List::Nil)))));
    let list_clone = Arc::clone(&list);

    thread::spawn(move || {
        let list = list_clone.lock().unwrap();
        // ... 作業 ...
    })
    .join()
    .unwrap();

    println!("Recursive data structure: {:?}", list.lock().unwrap());

    // 5. 構造体の使用
    let person = Arc::new(Mutex::new(Person {
        name: String::from("Alice"),
        age: 30,
    }));
    let person_clone = Arc::clone(&person);

    thread::spawn(move || {
        let mut person = person_clone.lock().unwrap();
        person.age += 1;
    })
    .join()
    .unwrap();

    let locked_person = person.lock().unwrap();
    println!(
        "Struct usage: {} is {}",
        locked_person.name, locked_person.age
    );

    // 6. 構造体の配列の使用
    let people = Arc::new(Mutex::new([
        Person {
            name: String::from("Alice"),
            age: 30,
        },
        Person {
            name: String::from("Bob"),
            age: 25,
        },
    ]));
    let people_clone = Arc::clone(&people);

    thread::spawn(move || {
        let mut people = people_clone.lock().unwrap();
        people[0].age += 1;
    })
    .join()
    .unwrap();

    println!("Array of structs: {:?}", people.lock().unwrap());

    // 7. トレイトオブジェクト
    let animal: Arc<Mutex<dyn Speak>> = Arc::new(Mutex::new(Dog));
    let animal_clone = Arc::clone(&animal);

    thread::spawn(move || {
        animal_clone.lock().unwrap().speak();
    })
    .join()
    .unwrap();

    // 8. 所有権の移動
    let arc_mutex_num = Arc::new(Mutex::new(10));
    let arc_mutex_num_clone = Arc::clone(&arc_mutex_num);

    thread::spawn(move || {
        let mut num = arc_mutex_num_clone.lock().unwrap();
        *num += 5;
    })
    .join()
    .unwrap();

    println!("Ownership moved: {}", arc_mutex_num.lock().unwrap());

    // 9. 所有権の貸し出し
    let arc_mutex_num = Arc::new(Mutex::new(10));
    borrow(&arc_mutex_num);

    // 10. 固定データの参照
    let arc_mutex_num = Arc::new(Mutex::new(10));
    let arc_mutex_num_clone = Arc::clone(&arc_mutex_num);

    thread::spawn(move || {
        let num = arc_mutex_num_clone.lock().unwrap();
        println!("Immutable reference: {}", num);
    })
    .join()
    .unwrap();

    // 11. 変更データの参照
    let arc_mutex_num = Arc::new(Mutex::new(10));
    let arc_mutex_num_clone = Arc::clone(&arc_mutex_num);

    thread::spawn(move || {
        let mut num = arc_mutex_num_clone.lock().unwrap();
        *num += 5;
    })
    .join()
    .unwrap();

    println!("Mutable reference: {}", arc_mutex_num.lock().unwrap());
}
