use clap::{App, Arg};

fn main() {
    let matches = App::new("My CLI App")
        .version("1.0")
        .author("Your Name")
        .about("Does awesome things")
        .arg(
            Arg::with_name("input")
                .help("The input file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    let input = matches.value_of("input").unwrap();
    println!("Using input file: {}", input);
}
