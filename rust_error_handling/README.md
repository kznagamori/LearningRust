# 一般的なエラー処理

Rustで一般的なエラー処理を行うためには、主に `Option` と `Result` 型を使用します。これらは列挙型で、`Option` は値が存在するかもしれないし存在しないかもしれない場合に使用され、`Result` は操作が成功したか、あるいは失敗した（エラーが発生した）場合に使用されます。

## サンプルコード
以下のサンプルでは、`Option` と `Result` 型を使用したエラー処理の方法を示します。

**src/main.rs**
```rust
// Option を使った関数
fn find_even_number(numbers: &[i32]) -> Option<i32> {
    for &num in numbers {
        if num % 2 == 0 {
            return Some(num);
        }
    }
    None
}

// Result を使った関数
fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("Cannot divide by zero")
    } else {
        Ok(a / b)
    }
}

fn main() {
    let numbers = vec![1, 3, 5, 8, 13];

    // Option の処理
    match find_even_number(&numbers) {
        Some(num) => println!("Found even number: {}", num),
        None => println!("No even numbers found"),
    }

    // Result の処理
    match divide(10, 0) {
        Ok(result) => println!("Division result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    // unwrap と expect の使用
    let num = find_even_number(&numbers).unwrap_or(-1);
    println!("Found number (or -1): {}", num);

    let result = divide(10, 2).expect("Failed to divide");
    println!("Division result (expect): {}", result);
}
```

## Option、Result、unwrap、expect の各メリットとデメリット
### 1. Option:
- メリット: 値が存在するかどうかを明示的に扱うことができ、`None` の場合の処理を強制する。
- デメリット: エラーの詳細情報を持たない。

### 2. Result:
- メリット: 成功時の値とエラー時の詳細情報の両方を扱うことができる。
- デメリット: 成功とエラーの両方の処理を書く必要があるため、コードが長くなることがある。

### 3. unwrap:
- メリット: 簡潔なコード。成功時の値を直接取得する。
- デメリット: `None` または `Err` の場合にパニックを引き起こす。

### 4. expect:
- メリット: `unwrap` と同様だが、エラーメッセージをカスタマイズできる。
- デメリット: `None` または `Err`の場合にパニックを引き起こす。

## 手順
1. **新しいプロジェクトを作成:**
- ターミナルで `cargo new rust_error_handling` コマンドを実行して新しいプロジェクトを作成します。

2. **ファイルの編集:**
- `src/main.rs` ファイルを開き、上記のサンプルコードを記述します。

3. **ビルドと実行:**
- プロジェクトのルートディレクトリで `cargo build` コマンドを実行してプロジェクトをビルドします。
- ビルドが完了したら `cargo run` コマンドを実行してプログラムを実行します。

このサンプルでは、`Option` と `Result` 型を使用して、プログラム内で発生する可能性のある異なる種類のエラーを適切に処理しています。また、`unwrap` と `expect` を使った場合の振る舞いも示しています。

