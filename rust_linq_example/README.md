# C#みたいなLINQ機能

RustにはC#の `LINQ` に完全に対応する機能はありませんが、イテレータとクロージャを使用することで `LINQ` のような操作を実現することができます。Rustのイテレータは多くの便利なメソッドを提供しており、フィルタリング、マッピング、折り畳みなどの操作が可能です。

以下の例では、Rustでのイテレータとクロージャを使用して、C#の `LINQ` のような操作を行う方法を示します。

## サンプルコード
**src/main.rs**
```rust
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
```

## 手順
1. **新しいプロジェクトを作成:**
- ターミナルで `cargo new rust_linq_example` コマンドを実行して新しいプロジェクトを作成します。

2. **ファイルの編集:**
- `src/main.rs` ファイルを開き、上記のサンプルコードを記述します。

3. **ビルドと実行:**
- プロジェクトのルートディレクトリで `cargo build` コマンドを実行してプロジェクトをビルドします。
- ビルドが完了したら `cargo run` コマンドを実行してプログラムを実行します。

このサンプルでは、Rustでのイテレータを使用した `LINQ` のような操作を行っています。イテレータは、コレクションの各要素に対して様々な変換や操作を適用する強力なツールです。





