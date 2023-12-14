# Box\<T>

Rustの`Box<T>`は、ヒープ上にデータを確保するために使用されるスマートポインタです。以下に、指定された様々な使用例を示す実行可能なRustプログラムを提供します。

## 1. 基本的な使用
```rust
fn main() {
    let boxed_int = Box::new(5);
    println!("Basic use: {}", boxed_int);
}
```

## 2. 大きなデータ構造を扱う
```rust
struct LargeData {
    data: [u8; 1000],
}

fn main() {
    let large_data = Box::new(LargeData { data: [0; 1000] });
    println!("Handling large data structure: {:?}", large_data.data[0]);
}
```

## 3. 配列を扱う
```rust
fn main() {
    let boxed_array = Box::new([1, 2, 3, 4, 5]);
    println!("Array handling: {:?}", boxed_array);
}
```

## 4. 再帰的なデータ構造
```rust
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let list = Box::new(List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil)))));
    println!("Recursive data structure: {:?}", list);
}
```

## 5. 構造体の使用
```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let person = Box::new(Person { name: String::from("Alice"), age: 30 });
    println!("Struct usage: {} is {}", person.name, person.age);
}
```

## 6. 構造体の配列の使用
```rust
fn main() {
    let people = Box::new([
        Person { name: String::from("Alice"), age: 30 },
        Person { name: String::from("Bob"), age: 25 },
    ]);
    println!("Array of structs: {:?}", people);
}
```

## 7. トレイトオブジェクト
```rust
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
    let animal: Box<dyn Speak> = Box::new(Dog);
    animal.speak();
}
```

## 8. 所有権の移動
```rust
fn consume(boxed: Box<i32>) {
    println!("Ownership moved: {}", boxed);
}

fn main() {
    let boxed_num = Box::new(10);
    consume(boxed_num);
}
```

## 9. 所有権の貸し出し
```rust
fn borrow(boxed: &Box<i32>) {
    println!("Ownership borrowed: {}", boxed);
}

fn main() {
    let boxed_num = Box::new(10);
    borrow(&boxed_num);
}
```

## 10. 固定データの参照
```rust
fn main() {
    let boxed_num = Box::new(10);
    let reference: &i32 = &boxed_num;
    println!("Immutable reference: {}", reference);
}
```

## 11. 変更データの参照
```rust
fn main() {
    let mut boxed_num = Box::new(10);
    let reference: &mut i32 = &mut boxed_num;
    *reference += 5;
    println!("Mutable reference: {}", boxed_num);
}
```

これらの例は、`Box<T>` を使用するさまざまな方法を示しています。`Box<T>` は所有権の移動、メモリの確保、ポリモーフィズムなどの面でRustプログラミングにおいて重要な役割を果たします。
