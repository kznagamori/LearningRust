# Arc\<T>

`Arc<T>` は、Rustでのスレッド間共有のための参照カウントされたスマートポインタです。`Rc<T>` と同様ですが、`Arc<T>` はスレッドセーフです。以下に `Arc<T>` の使用例をいくつか示します。

## 1. 基本的な使用
```rust
use std::sync::Arc;

fn main() {
    let arc = Arc::new(5);
    let arc_clone = arc.clone();

    println!("Basic use: {}", arc_clone);
}
```

## 2. 大きなデータ構造を扱う
```rust
use std::sync::Arc;

struct LargeData {
    data: [u8; 1000],
}

fn main() {
    let large_data = Arc::new(LargeData { data: [0; 1000] });
    let large_data_clone = large_data.clone();

    println!("Handling large data structure: {:?}", large_data_clone.data[0]);
}
```

## 3. 配列を扱う
```rust
use std::sync::Arc;

fn main() {
    let arc_array = Arc::new([1, 2, 3, 4, 5]);
    let arc_array_clone = arc_array.clone();

    println!("Array handling: {:?}", arc_array_clone);
}
```

## 4. 再帰的なデータ構造
```rust
use std::sync::{Arc, Mutex};

#[derive(Debug)]
enum List {
    Cons(i32, Arc<Mutex<List>>),
    Nil,
}

fn main() {
    let list = Arc::new(Mutex::new(List::Cons(1, Arc::new(Mutex::new(List::Nil)))));
    let list_clone = list.clone();

    println!("Recursive data structure: {:?}", list_clone.lock().unwrap());
}
```

## 5. 構造体の使用
```rust
use std::sync::Arc;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let person = Arc::new(Person { name: String::from("Alice"), age: 30 });
    let person_clone = person.clone();

    println!("Struct usage: {} is {}", person_clone.name, person_clone.age);
}
```

## 6. 構造体の配列の使用
```rust
use std::sync::Arc;

fn main() {
    let people = Arc::new([
        Person { name: String::from("Alice"), age: 30 },
        Person { name: String::from("Bob"), age: 25 },
    ]);
    let people_clone = people.clone();

    println!("Array of structs: {:?}", people_clone);
}
```

## 7. トレイトオブジェクト
```rust
use std::sync::Arc;

trait Speak {
    fn speak(&self);
}

struct Dog;

impl Speak for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

fn main() {
    let animal: Arc<dyn Speak> = Arc::new(Dog);
    let animal_clone = animal.clone();

    animal_clone.speak();
}
```

## 8. 所有権の移動
```rust
use std::sync::Arc;

fn consume(arc: Arc<i32>) {
    println!("Ownership moved: {}", arc);
}

fn main() {
    let arc_num = Arc::new(10);
    consume(arc_num);
}
```

## 9. 所有権の貸し出し
```rust
use std::sync::Arc;

fn borrow(arc: &Arc<i32>) {
    println!("Ownership borrowed: {}", arc);
}

fn main() {
    let arc_num = Arc::new(10);
    borrow(&arc_num);
}
```

## 10. 固定データの参照
```rust
use std::sync::Arc;

fn main() {
    let arc_num = Arc::new(10);
    let reference = Arc::clone(&arc_num);

    println!("Immutable reference: {}", reference);
}
```

## 11. 変更データの参照
`Arc<T>` は不変なので、変更可能な参照を作成することはできません。`Mutex<T>` や `RwLock<T>` などを使って内部可変性を実現する必要があります。

これらの例は、`Arc<T>` を使用するさまざまな方法を示しています。`Arc<T>` は主にスレッド間でデータを共有する場合に使用され、スレッドセーフな方法でデータへの参照を管理します。
