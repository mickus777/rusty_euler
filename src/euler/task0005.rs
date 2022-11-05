use crate::math::prime_sieve::PrimeSieve;

pub fn execute(input: &String) {
    let ceiling: u64 = input.parse().unwrap();

    let mut sieve = PrimeSieve::new();
    while sieve.next_possible_prime <= ceiling {
        sieve.expand();
    }

    let mut product = 1;

    for prime in sieve.prime_multiples.iter() {
        if prime.prime <= ceiling {
            let mut max_prime_product = prime.prime;
            while max_prime_product * prime.prime <= ceiling {
                max_prime_product *= prime.prime;
            }
            product *= max_prime_product;
        }
    }

    println!("{}", product);
}