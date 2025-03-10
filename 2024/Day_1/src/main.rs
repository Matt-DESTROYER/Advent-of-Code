use std::{
    env,
    fs::read_to_string
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path: &String = &args[1];

    let content: String = read_to_string(file_path)
        .expect("Error reading file");

    println!("File content: {}", content);
}
