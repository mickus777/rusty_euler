use crate::math::sequences::PrimeNumbers;

use std::collections::HashSet;
use std::collections::LinkedList;

fn rotate_digits(mut number : u64) -> HashSet<u64> {
    let mut digits = LinkedList::new();

    while number > 0 {
        digits.push_front(number % 10);
        number /= 10;
    }

    let mut rotations = HashSet::new();

    for _i in 0..digits.len() {
        let mut rotation = 0;
        for digit in digits.iter() {
            rotation = rotation * 10 + digit;
        }
        if !rotations.contains(&rotation) {
            rotations.insert(rotation);
        }
        let digit = digits.pop_front().unwrap();
        digits.push_back(digit);
    }

    rotations
}

pub fn execute(input: &String) {
    let ceiling: u64 = input.parse().unwrap();

    let mut primes : HashSet<u64> = HashSet::new();
    let mut sequence = PrimeNumbers::new();

    loop {
        let prime = sequence.next();
        if prime > ceiling {
            break;
        }
        primes.insert(prime);
    }

    let mut count = 0;
    for prime in primes.iter() {
        let rotations = rotate_digits(*prime);
        let mut all_primes = true;
        for rotation in rotations.iter() {
            if !primes.contains(rotation) {
                all_primes = false;
                break;
            }
        }
        if all_primes {
            count += 1;
        }
    }

    println!("{}", count);
}