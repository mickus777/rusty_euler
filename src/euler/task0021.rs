use std::collections::HashMap;
use std::vec::Vec;

use num_traits::ToPrimitive;

fn proper_divisors(value: u64) -> Vec<u64> {
    let mut divisors = Vec::new();
    divisors.push(1);

    let highest_possible_divisor = f32::sqrt(value.to_f32().unwrap()).to_u64().unwrap();

    for divisor in 2..(highest_possible_divisor + 1) {
        if value % divisor == 0 {
            divisors.push(divisor);
            divisors.push(value / divisor);
        }
    }

    divisors
}

fn has_amicable_pairing(value: u64) -> Option<u64> {
    let first_divisors = proper_divisors(value);
    let possible_pairing = first_divisors.iter().sum();
    let second_divisors = proper_divisors(possible_pairing);
    if value == second_divisors.iter().sum() {
        Some(possible_pairing)
    } else {
        None
    }
}

pub fn execute(input: &String) {
    let ceiling: u64 = input.parse().unwrap();

    let mut amicable_pairings : HashMap<u64, Option<u64>> = HashMap::new();
    let mut sum = 0;

    for value in 1..(ceiling + 1) {
        if !amicable_pairings.contains_key(&value) {
            if let Some(pairing) = has_amicable_pairing(value) {
                if pairing <= ceiling && pairing != value {
                    amicable_pairings.insert(value, Some(pairing));
                    amicable_pairings.insert(pairing, Some(value));
                    println!("{} {}", value, pairing);
                    sum += value + pairing;
                }
            } else {

            }
        }
    }

    println!("{:?}", sum);
}