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
