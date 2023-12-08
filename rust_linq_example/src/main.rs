fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];

    // 偶数だけを選択
    let even_numbers: Vec<_> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    println!("Even numbers: {:?}", even_numbers);

    // 各要素を2倍にする
    let doubled_numbers: Vec<_> = numbers.iter().map(|&x| x * 2).collect();
    println!("Doubled numbers: {:?}", doubled_numbers);

    // 偶数のみを選択し、それぞれを2倍にする
    let processed_numbers: Vec<_> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * 2)
        .collect();
    println!("Processed numbers: {:?}", processed_numbers);

    // 最初の5要素を足し合わせる
    let sum_of_first_five: i32 = numbers.iter().take(5).fold(0, |acc, &x| acc + x);
    println!("Sum of first five numbers: {}", sum_of_first_five);
}
