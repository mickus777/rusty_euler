use std::fs;
use std::path::Path;

fn name_value(name : &str) -> u64 {
    name.chars().map(|ch| ch as u64 - 64).sum()
}

pub fn execute(input: &String) {

    let name_list = fs::read_to_string(Path::new(input)).unwrap();
    let mut names : Vec<&str> = name_list[1..(name_list.len() - 1)].split("\",\"").collect();

    names.sort();
    let mut index = 1;
    let mut sum = 0;

    for name in names {
        sum += name_value(name) * index;
        index += 1;
    }
    
    println!("{}", sum);
}