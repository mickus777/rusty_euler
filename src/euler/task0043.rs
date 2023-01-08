use crate::math::sequences::Permutations;

fn numberize(digits : &Vec<u64>, start_index : usize) -> u64 {
    digits[start_index] * 100 + digits[start_index + 1] * 10 + digits[start_index + 2]
}

fn numberize_all(digits : &Vec<u64>) -> u64 {
    let mut value = 0;
    for digit in digits.iter() {
        value *= 10;
        value += digit;
    }
    value
}

pub fn execute(input: &String) {
    let mut _count: usize = input.parse().unwrap();

    let mut permuter = Permutations::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

    let mut sum : u64 = 0;

    loop {
        if let Option::Some(number) = permuter.next() {
            if number[0] == 0 {
                continue;
            }

            if numberize(number, 1) % 2 != 0 ||
            numberize(number, 2) % 3 != 0 ||
            numberize(number, 3) % 5 != 0 ||
            numberize(number, 4) % 7 != 0 ||
            numberize(number, 5) % 11 != 0 ||
            numberize(number, 6) % 13 != 0 ||
            numberize(number, 7) % 17 != 0 {
                continue;
            }

            let value = numberize_all(number);
            println!("{}", value);
            sum += value;

        } else {
            break;
        }
    }

    println!("=============");
    println!("Sum {}", sum);
}