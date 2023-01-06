use crate::math::sequences::PrimeNumbers;
use crate::math::utils::is_pandigital;

pub fn execute(input: &String) {
    let mut ceiling: u64 = input.parse().unwrap();
    if ceiling == 0 {
        ceiling = 1000000000;
    }

    let mut prime_sequence = PrimeNumbers::new();

    let mut largest_pandigital_prime = 0;

    loop {
        let prime = prime_sequence.next();
        if prime >= ceiling {
            break;
        }
        if is_pandigital(&vec![prime]) {
            largest_pandigital_prime = prime;
            println!("{}", prime);
        }
    }

    println!("=============");
    println!("Largest {}", largest_pandigital_prime);
}