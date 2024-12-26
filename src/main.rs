use std::env;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();

    dbg!(&args);

    let query = &args[1];
    let filename = &args[2];

    println!("query is {}", query);
    println!("filename is {}", filename);
}
