fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // クロージャを使って各要素を2倍にする
    let doubled: Vec<_> = numbers.iter().map(|&x| x * 2).collect();
    println!("Doubled numbers: {:?}", doubled);

    // クロージャを使って偶数のみをフィルタリングする
    let evens: Vec<_> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    println!("Even numbers: {:?}", evens);

    // 外部の変数をキャプチャするクロージャ
    let factor = 3;
    let multiplied: Vec<_> = numbers.iter().map(|x| x * factor).collect();
    println!("Numbers multiplied by {}: {:?}", factor, multiplied);
}
