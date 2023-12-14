# Rc\<T>

Rustの `Rc<T>` は、参照カウントされたスマートポインタで、データへの共有不変参照を複数持つことができます。以下に `Rc<T>` の使用例を示す実行可能なRustプログラムを提供します。

## 1. 基本的な使用
```rust
use std::rc::Rc;

fn main() {
    let rc_int = Rc::new(5);
    println!("Basic use: {}", rc_int);
}
```

## 2. 大きなデータ構造を扱う
```rust
use std::rc::Rc;

struct LargeData {
    data: [u8; 1000],
}

fn main() {
    let large_data = Rc::new(LargeData { data: [0; 1000] });
    println!("Handling large data structure: {:?}", large_data.data[0]);
}
```

## 3. 配列を扱う
```rust
use std::rc::Rc;

fn main() {
    let rc_array = Rc::new([1, 2, 3, 4, 5]);
    println!("Array handling: {:?}", rc_array);
}
```

## 4. 再帰的なデータ構造
```rust
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let list = Rc::new(List::Cons(1, Rc::new(List::Cons(2, Rc::new(List::Nil)))));
    println!("Recursive data structure: {:?}", list);
}
```

## 5. 構造体の使用
```rust
use std::rc::Rc;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let person = Rc::new(Person { name: String::from("Alice"), age: 30 });
    println!("Struct usage: {} is {}", person.name, person.age);
}
```

## 6. 構造体の配列の使用
```rust
use std::rc::Rc;

fn main() {
    let people = Rc::new([
        Person { name: String::from("Alice"), age: 30 },
        Person { name: String::from("Bob"), age: 25 },
    ]);
    println!("Array of structs: {:?}", people);
}
```

## 7. トレイトオブジェクト
```rust
use std::rc::Rc;

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
    let animal: Rc<dyn Speak> = Rc::new(Dog);
    animal.speak();
}
```

## 8. 所有権の移動
`Rc<T>`は所有権を移動する代わりに共有します。
```rust
use std::rc::Rc;

fn main() {
    let rc_num = Rc::new(10);
    let rc_num_clone = Rc::clone(&rc_num);
    println!("Ownership shared: {}", rc_num_clone);
}
```

## 9. 所有権の貸し出し
```rust
use std::rc::Rc;

fn borrow(rc_num: &Rc<i32>) {
    println!("Ownership borrowed: {}", rc_num);
}

fn main() {
    let rc_num = Rc::new(10);
    borrow(&rc_num);
}
```

## 10. 固定データの参照
```rust
use std::rc::Rc;

fn main() {
    let rc_num = Rc::new(10);
    let reference: &i32 = &rc_num;
    println!("Immutable reference: {}", reference);
}
```

## 11. 変更データの参照
`Rc<T>`は不変なので、変更可能な参照を作成することはできません。RefCell<T>などを使って内部可変性を実現する必要があります。

これらの例は、`Rc<T>`を使用するさまざまな方法を示しています。`Rc<T>`は共有された所有権やデータの共有など、特定のシナリオで役立つ重要なツールです。
