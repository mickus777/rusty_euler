use crate::math::utils::is_palindromic;

fn to_binary(mut number : u64) -> u64 {
    let mut binary = 0;

    let mut divisor = 2;
    while number / divisor > 0 {
        divisor *= 2;
    }

    while divisor > 0 {
        let one_or_zero = number / divisor;
        binary = binary * 10 + one_or_zero;
        number -= divisor * one_or_zero;
        divisor /= 2;
    }

    binary
}

pub fn execute(input: &String) {
    let mut ceiling: u64 = input.parse().unwrap();
    if ceiling == 0 {
        ceiling = 1000000;
    }

    let mut sum = 0;
    for number in 1..ceiling {
        if is_palindromic(number) && is_palindromic(to_binary(number)) {
            sum += number;
        }
    }

    println!("{}", sum);
}