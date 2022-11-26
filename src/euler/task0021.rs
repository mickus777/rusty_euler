use std::collections::HashMap;

use crate::math::utils;

fn has_amicable_pairing(value: u64) -> Option<u64> {
    let first_divisors = utils::proper_divisors(value);
    let possible_pairing = first_divisors.iter().sum();
    let second_divisors = utils::proper_divisors(possible_pairing);
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