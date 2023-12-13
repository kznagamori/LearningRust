fn main() {
    let mut stack = Vec::new();

    // スタックに要素をプッシュ
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // スタックから要素をポップ
    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }
}
