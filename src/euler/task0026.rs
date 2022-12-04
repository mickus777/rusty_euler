use std::collections::HashMap;

fn find_cycle_length(denominator : u32) -> usize {

    let mut remainders : HashMap<u32, usize> = HashMap::new();

    let mut length = 0;
    let mut remainder = 10 % denominator;
    while remainder != 0 {
        if remainders.contains_key(&remainder) {
            return length - remainders.get(&remainder).unwrap();
        }

        remainders.insert(remainder, length); 
        length += 1;
        remainder *= 10;
        remainder %= denominator;
    }

    0
}

pub fn execute(input: &String) {
    let ceiling: u32 = input.parse().unwrap();

    let mut greatest_denominator = 0;
    let mut greatest_length = 0;
    for denominator in 2..ceiling {
        let length = find_cycle_length(denominator);
        if length > greatest_length {
            greatest_length = length;
            greatest_denominator = denominator;
        }
    }

    println!("{}", greatest_denominator);
}