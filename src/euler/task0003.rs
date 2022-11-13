use crate::math::sequences::PrimeNumbers;

pub fn execute(input: &String) {
    let number: u64 = input.parse().unwrap();

    let mut primes = PrimeNumbers::new();

    for factor in primes.factorize(number) {
        println!("{}", factor);
    }
}