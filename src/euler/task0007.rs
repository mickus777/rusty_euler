use crate::math::prime_sieve::PrimeSieve;

fn index(count: usize, sieve: &PrimeSieve) -> u64 {
    let mut index = 0;
    let mut prime = sieve.prime_multiples.iter();
    loop {
        if index == count - 1 {
            break;
        }
        prime.next();
        index += 1;
    }

    prime.next().unwrap().prime
}

pub fn execute(input: &String) {
    let count: usize = input.parse().unwrap();

    let mut sieve = PrimeSieve::new();

    while sieve.prime_multiples.len() < count {
        sieve.expand();
    }

    println!("{}", index(count, &sieve));
}