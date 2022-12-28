use crate::math::sequences::PrimeNumbers;

use std::collections::HashSet;

fn is_truncatable_prime(mut prime : u64, primes : &HashSet<u64>) -> bool {
    let mut leftover = 0;
    let mut multiplicand = 1;
    while prime > 0 {
        let digit = prime % 10;
        leftover += multiplicand * digit;
        if !primes.contains(&prime) || !primes.contains(&leftover) {
            return false;
        }
        prime /= 10;
        multiplicand *= 10;
    }
    
    true
}

pub fn execute(input: &String) {
    let mut count: usize = input.parse().unwrap();
    if count == 0 {
        count = 11;
    }

    let mut prime_sequence = PrimeNumbers::new();

    let mut primes : HashSet<u64> = HashSet::new();
    for _i in 0..4 {
        // 2, 3, 5, 7
        primes.insert(prime_sequence.next());
    }

    let mut sum = 0;
    while count > 0 {
        let prime = prime_sequence.next();
        primes.insert(prime);
        if is_truncatable_prime(prime, &primes) {
            println!("{}", prime);
            sum += prime;
            count -= 1;
        }
    }

    println!("=============");
    println!("Sum {}", sum);
}