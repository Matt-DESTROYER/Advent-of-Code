use std::{
    collections::HashMap,
    env,
    fs::{
        File,
        read_to_string
    },
    io::Write
};

fn part1(mut list1: Vec<i32>, mut list2: Vec<i32>) {
    list1.sort();
    list2.sort();

    let mut sum: i32 = 0;
    for i in 0..list1.len() {
        sum += (list1[i] - list2[i]).abs();
    }

    File::create("./output.part1.txt")
        .expect("Error creating file")
        .write_all(sum.to_string().as_bytes())
        .expect("Error writing to file");
}

fn part2(mut list1: Vec<i32>, mut list2: Vec<i32>) {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut sum: i32 = 0;

    for i in 0..list1.len() {
        if map.contains_key(&list1[i]) {
            sum += map.get(&list1[i]).unwrap();
            continue;
        }

        let mut count: i32 = 0;
        for j in 0..list2.len() {
            if list1[i] == list2[j] {
                count += 1;
            }
        }
        sum += list1[i] * count;
        map.insert(list1[i], list1[i]*count);
    }

    File::create("./output.part2.txt")
        .expect("Error creating file")
        .write_all(sum.to_string().as_bytes())
        .expect("Error writing to file");
}

fn main() {
    // parse input

    let args: Vec<String> = env::args().collect();

    let file_path: &String = &args[1];

    let content: String = read_to_string(file_path)
        .expect("Error reading file");

    let lines: Vec<&str> = content
        .split("\n")
        .collect();

    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in lines {
        let values: Vec<&str> = line
            .split("   ")
            .collect();

        list1.push(values[0].trim().parse().unwrap());
        list2.push(values[1].trim().parse().unwrap());
    }

    // complete the problem
    part1(list1.clone(), list2.clone());
    part2(list1.clone(), list2.clone());
}
