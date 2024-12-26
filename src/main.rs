use std::env;
use std::fs;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();

    dbg!(&args);

    let query = &args[1];
    let filename = &args[2];

    println!("query is {}", query);
    println!("filename is {}", filename);

    let contents = fs::read_to_string(filename).expect("Should have been able to read the file");
    println!("Content is:-\n {}", contents);
}
