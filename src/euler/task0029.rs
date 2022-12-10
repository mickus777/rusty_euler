use num_bigint::BigUint;
use num_bigint::ToBigUint;
use num_traits::One;
use std::collections::HashSet;

pub fn execute(input: &String) {
    let ceiling = input.parse::<i32>().unwrap().to_biguint().unwrap();

    let mut numbers : HashSet<BigUint> = HashSet::new();

    let mut a = 2.to_biguint().unwrap();
    while a <= ceiling {
        let mut b = 2;
        while b.to_biguint().unwrap() <= ceiling {

            numbers.insert(a.pow(b));

            b += 1;
        }

        a += BigUint::one();
    }

    println!("Sum: {}", numbers.len());
}