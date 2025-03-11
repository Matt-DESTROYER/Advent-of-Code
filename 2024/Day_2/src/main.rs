use std::{
    fs::{
        File,
        read_to_string
    },
    io::Write
};

fn part1(input: &Vec<Vec<i32>>) {
    let mut safe_count: i32 = 0;

    for row in input {
        let mut last_diff: i32 = 0;
        let mut safe: bool = true;
        for i in 0..row.len()-1 {
            let diff: i32 = row[i] - row[i+1];
            let dist: i32 = diff.abs();

            if dist < 1 || dist > 3 {
                safe = false;
                break;
            }

            if diff > 0 && last_diff < 0
            || diff < 0 && last_diff > 0 {
                safe = false;
                break;
            }

            last_diff = diff;
        }
        if safe {
            safe_count += 1;
        }
    }

    File::create("./output.part1.txt")
        .expect("Error creating file")
        .write_all(safe_count.to_string().as_bytes())
        .expect("Error writing to file");
}

fn part2(input: &Vec<Vec<i32>>) {
    File::create("./output.part2.txt")
        .expect("Error creating file")
        .write_all("Hello, world!".as_bytes())
        .expect("Error writing to file");
}


fn main() {
    // parse input

    let args: Vec<String> = std::env::args().collect();

    let file_path: &String = &args[1];

    let content: String = read_to_string(file_path)
        .expect("Error reading file")
        .trim()
        .to_string();

    let lines: Vec<&str> = content
        .split("\n")
        .collect();

    let input: Vec<Vec<i32>> = lines
        .iter()
        .map(|line| {
            line
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();

    // part1
    part1(&input);

    // part2
    part2(&input);
}
