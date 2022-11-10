use crate::math::prime_sieve::PrimeSieve;

pub fn execute(input: &String) {
    let ceiling: u64 = input.parse().unwrap();

    let mut sieve = PrimeSieve::new();

    while sieve.next_possible_prime <= ceiling {
        sieve.expand();
    }

    let mut sum = 0;

    for prime in sieve.prime_multiples {
        if prime.prime > ceiling {
            break;
        }
        sum += prime.prime;
    }

    println!("{}", sum);
}