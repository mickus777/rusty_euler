use crate::math::sequences::PrimeNumbers;

pub fn execute(input: &String) {
    let ceiling: u64 = input.parse().unwrap();

    let mut primes = PrimeNumbers::new();

    let mut product = 1;

    loop {
        let prime = primes.next();

        if prime > ceiling {
            break;
        }

        let mut max_prime_product = prime;
        while max_prime_product * prime <= ceiling {
            max_prime_product *= prime;
        }

        product *= max_prime_product;
    }

    println!("{}", product);
}