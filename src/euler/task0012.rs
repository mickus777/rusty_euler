use crate::math::sequences::TriangleNumbers;
use crate::math::prime_sieve::PrimeSieve;

use std::collections::HashMap;

pub fn execute(input: &String) {
    let count: u64 = input.parse().unwrap();

    let mut sieve = PrimeSieve::new();

    let mut sequence = TriangleNumbers::new();
    loop {
        let number = sequence.next();

        let factors = sieve.factorize(number);

        let mut factor_counts : HashMap<u64, u64> = HashMap::new();

        for factor in factors {
            if factor_counts.contains_key(&factor) {
                factor_counts.insert(factor, 1 + factor_counts.get(&factor).unwrap());
            } else {
                factor_counts.insert(factor, 1);
            }
        }

        let mut combination_count = 1;
        for factor_count in factor_counts {
            combination_count *= 1 + factor_count.1;
        }

        if combination_count > count {
            println!("{}", number);
            break;
        }
    }
}