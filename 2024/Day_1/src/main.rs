use std::{
    env,
    fs::{
        File,
        read_to_string
    },
    io::Write
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path: &String = &args[1];

    let content: String = read_to_string(file_path)
        .expect("Error reading file");

    let lines: Vec<&str> = content
        .split("\n")
        .collect();

    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    let mut sum: i32 = 0;

    for line in lines {
        let values: Vec<&str> = line
            .split("   ")
            .collect();

        list1.push(values[0].trim().parse().unwrap());
        list2.push(values[1].trim().parse().unwrap());
    }

    list1.sort();
    list2.sort();

    for i in 0..list1.len() {
        sum += (list1[i] - list2[i]).abs();
    }

    File::create("../output.txt")
        .expect("Error creating file")
        .write_all(sum.to_string().as_bytes())
        .expect("Error writing to file");
}
