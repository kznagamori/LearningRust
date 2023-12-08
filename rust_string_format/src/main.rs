fn main() {
    let int_num = 42;
    let float_num = 3.14159;
    let str = "Hello, world!";

    println!("Decimal: {}", int_num);
    println!("Hex: {:x}", int_num);
    println!("Octal: {:o}", int_num);
    println!("Binary: {:b}", int_num);
    println!("Fixed float: {:.3}", float_num);
    println!("Scientific: {:e}", float_num);
    println!("Left align: {:<10}", str);
    println!("Right align: {:>10}", str);
    println!("Zero padded: {:08}", int_num);
    println!("Max 5 characters: {:.5}", str);
}
