use std::collections::HashMap;
use std::vec::Vec;

fn collatz(value: u64) -> u64 {
    if value % 2 == 0 {
        value / 2
    } else {
        value * 3 + 1
    }
}

fn collatz_path(distances : &HashMap<u64, u64>, mut value : u64) -> Vec<u64> {
    let mut path = Vec::new();

    while !distances.contains_key(&value) {
        path.push(value);
        value = collatz(value);
    }
    path.push(value);

    path
}

pub fn execute(input: &String) {
    let ceiling: u64 = input.parse().unwrap();

    let mut distances : HashMap<u64, u64> = HashMap::new();
    distances.insert(1, 1);
    let mut longest_length = 0;
    let mut longest_value = 1;

    for value in 1..(ceiling + 1) {
        if distances.contains_key(&value) {
            continue
        }

        let mut unknown_sequence = collatz_path(&distances, value);
        let root = unknown_sequence.pop().unwrap();
        let root_length = *distances.get(&root).unwrap();
        let path_length = root_length + u64::try_from(unknown_sequence.len()).unwrap();
        
        if path_length > longest_length {
            longest_length = path_length;
            longest_value = value;
        }

        let mut distance = root_length;
        while unknown_sequence.len() > 0 {
            distance += 1;
            let value = unknown_sequence.pop().unwrap();
            distances.insert(value, distance);
        }
    }

    println!("{}", longest_value);
}