use crate::math::sequences::PrimeNumbers;

pub fn execute(input: &String) {
    let count: usize = input.parse().unwrap();

    let mut primes = PrimeNumbers::new();

    for _ in 1..count {
        primes.next();
    }

    println!("{}", primes.next());
}