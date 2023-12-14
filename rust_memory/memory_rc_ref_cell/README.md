# Rc\<RefCell\<T>>

`Rc<RefCell<T>>` の組み合わせは、Rustで参照カウントされた共有状態を持ち、実行時に内部可変性を許容するためによく使われます。以下に、`Rc<RefCell<T>>` の使用例を示します。

## 1. 基本的な使用
```rust
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let rc_refcell = Rc::new(RefCell::new(5));
    *rc_refcell.borrow_mut() += 1; // 値を変更
    println!("Basic use: {}", rc_refcell.borrow());
}
```

## 2. 大きなデータ構造を扱う
```rust
use std::rc::Rc;
use std::cell::RefCell;

struct LargeData {
    data: [u8; 1000],
}

fn main() {
    let large_data = Rc::new(RefCell::new(LargeData { data: [0; 1000] }));
    large_data.borrow_mut().data[0] = 1; // データを変更
    println!("Handling large data structure: {:?}", large_data.borrow().data[0]);
}
```

## 3. 配列を扱う
```rust
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let rc_refcell_array = Rc::new(RefCell::new([1, 2, 3, 4, 5]));
    rc_refcell_array.borrow_mut()[0] = 10; // 配列の要素を変更
    println!("Array handling: {:?}", rc_refcell_array.borrow());
}
```

## 4. 再帰的なデータ構造
```rust
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<RefCell<List>>),
    Nil,
}

fn main() {
    let list = Rc::new(RefCell::new(List::Cons(1, Rc::new(RefCell::new(List::Nil)))));
    println!("Recursive data structure: {:?}", list.borrow());
}
```

## 5. 構造体の使用
```rust
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Person {
    name: String,
    age: RefCell<u8>,
}

fn main() {
    let person = Rc::new(RefCell::new(Person { name: String::from("Alice"), age: RefCell::new(30) }));
    person.borrow_mut().age.replace(31); // 年齢を変更
    println!("Struct usage: {} is {}", person.borrow().name, person.borrow().age.borrow());
}
```

## 6. 構造体の配列の使用
```rust
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let people = Rc::new(RefCell::new([
        Person { name: String::from("Alice"), age: RefCell::new(30) },
        Person { name: String::from("Bob"), age: RefCell::new(25) },
    ]));
    people.borrow_mut()[0].age.replace(31); // Aliceの年齢を変更
    println!("Array of structs: {:?}", people.borrow());
}
```

## 7. トレイトオブジェクト
```rust
use std::rc::Rc;
use std::cell::RefCell;

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

fn main() {
    let dog = Rc::new(RefCell::new(Dog { volume: RefCell::new(5) }));
    dog.borrow().set_volume(10);
    dog.borrow().speak();
}
```

## 8. 所有権の移動
`Rc<RefCell<T>>` では、所有権を共有するために `Rc::clone` を使います。

```rust
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let rc_refcell = Rc::new(RefCell::new(10));
    let rc_refcell_clone = Rc::clone(&rc_refcell);
    println!("Ownership shared: {}", rc_refcell_clone.borrow());
}
```

## 9. 所有権の貸し出し
```rust
use std::rc::Rc;
use std::cell::RefCell;

fn borrow(rc_refcell: &Rc<RefCell<i32>>) {
    println!("Ownership borrowed: {}", rc_refcell.borrow());
}

fn main() {
    let rc_refcell = Rc::new(RefCell::new(10));
    borrow(&rc_refcell);
}
```

## 10. 固定データの参照
```rust
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let rc_refcell = Rc::new(RefCell::new(10));
    let reference = rc_refcell.borrow(); // 不変の借用
    println!("Immutable reference: {}", reference);
}
```

## 11. 変更データの参照
```rust
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let rc_refcell = Rc::new(RefCell::new(10));
    let mut reference = rc_refcell.borrow_mut(); // 可変の借用
    *reference += 5;
    println!("Mutable reference: {}", reference);
}
```

これらの例は、`Rc<RefCell<T>>` を使用するさまざまな方法を示しています。この組み合わせは、共有された状態を持つが、実行時にその状態を変更する必要がある場合に特に便利です。
