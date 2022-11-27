fn permute(numbers : &mut Vec<u64>) {

    let size = numbers.len();
    let mut first_character_index = None;

    for i in 0..(size - 1) {
        if numbers[i] < numbers[i + 1] {
            first_character_index = Some(i);
        }
    }
    if let None = first_character_index {
        numbers.sort();
        return;
    }

    let first_character = numbers[first_character_index.unwrap()];
    let mut second_character_index = None;

    for i in (first_character_index.unwrap() + 1)..size {
        let second_character = numbers[i];
        if second_character > first_character && (second_character_index == None || second_character < numbers[second_character_index.unwrap()]) {
            second_character_index = Some(i);
        }
    }

    numbers[first_character_index.unwrap()] = numbers[second_character_index.unwrap()];
    numbers[second_character_index.unwrap()] = first_character;
    numbers[(first_character_index.unwrap() + 1)..].sort();
}

pub fn execute(input: &String) {
    let permutations: usize = input.parse().unwrap();

    let mut numbers = Vec::new();
    for i in 0..10 {
        numbers.push(i);
    }

    for _i in 1..permutations {
        permute(&mut numbers);
    }

    println!("{:?}", numbers);
}