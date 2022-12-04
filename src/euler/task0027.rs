use crate::math::sequences::PrimeNumbers;

fn count_prime_results(primes : &mut PrimeNumbers, a: i64, b: i64) -> i64 {

    let mut n = 0;

    loop {
        let result = n * n + a * n + b;
        if result < 2 {
            break;
        }

        if primes.factorize(result as u64).len() != 1 {
            break;
        }

        n += 1;
    }

    n
}

pub fn execute(input: &String) {
    let ceiling: i64 = input.parse().unwrap();

    let mut primes = PrimeNumbers::new();
    let mut longest_series = 0;
    let mut product = 0;

    for a in -ceiling..ceiling {
        for b in -ceiling..ceiling {
            let series = count_prime_results(&mut primes, a, b);
            if series > longest_series {
                longest_series = series;
                product = a * b;
            }
        }
    }

    println!("{}", product);
}