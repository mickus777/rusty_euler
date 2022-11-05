use crate::math::prime_sieve::PrimeSieve;

pub fn execute(input: &String) {
    let number: u64 = input.parse().unwrap();

    let mut sieve = PrimeSieve::new();

    let factors = PrimeSieve::factorize(&mut sieve, number);

    for factor in factors {
        println!("{}", factor);
    }
}