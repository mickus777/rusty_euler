use crate::math::sequences::PrimeNumbers;

pub fn execute(input: &String) {
    let ceiling: u64 = input.parse().unwrap();

    let mut primes = PrimeNumbers::new();

    let mut sum = 0;

    loop {
        let prime = primes.next();
        if prime > ceiling {
            break;
        }
        sum += prime;
    }

    println!("{}", sum);
}