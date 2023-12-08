mod greetings;

fn main() {
    let name = "World";
    let greeting = greetings::greet(name);
    println!("{}", greeting);
}
