fn main() {
    let mut numbers = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];

    // ベクターをソート
    numbers.sort();

    println!("Sorted numbers: {:?}", numbers);

    // 逆順にソート
    numbers.sort_by(|a, b| b.cmp(a));

    println!("Sorted in reverse order: {:?}", numbers);
}
