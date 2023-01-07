use crate::math::sequences::TriangleNumbers;

use std::collections::HashSet;
use std::fs;
use std::path::Path;

fn word_value(word : &str) -> u64 {
    word.chars().map(|ch| ch as u64 - 64).sum()
}

pub fn execute(input: &String) {
    let mut file_name = input;
    let default_file_name = String::from("resources/p0042_words.txt");
    if file_name.len() < 1 {
        file_name = &default_file_name;
    }

    let mut triangle_sequence = TriangleNumbers::new();
    let mut triangle_numbers : HashSet<u64> = HashSet::new();
    let mut last_triangle_number = 0;

    let word_list = fs::read_to_string(Path::new(file_name)).unwrap();
    let mut words : Vec<&str> = word_list[1..(word_list.len() - 1)].split("\",\"").collect();
    
    let mut count = 0;
    for word in words.iter() {
        let value = word_value(*word);

        while value >= last_triangle_number {
            last_triangle_number = triangle_sequence.next();
            triangle_numbers.insert(last_triangle_number);
        }

        if triangle_numbers.contains(&value) {
            println!("{} {}", value, word);
            count += 1;
        }
    }


    println!("=============");
    println!("Count {}", count);
}