use std::cmp;
use std::collections::LinkedList;

use bit_vec::BitVec;

struct PrimeListItem {
    prime: u64,
    next_multiple: u64
}

struct PrimeSieve {
    next_possible_prime: u64,
    prime_multiples: LinkedList<PrimeListItem>
}

impl PrimeSieve {
    fn new() -> PrimeSieve {
        PrimeSieve { next_possible_prime: 2, prime_multiples: LinkedList::new() }
    }

    fn expand(&mut self) {
        let sieve_size: usize = cmp::min(10000000, self.next_possible_prime as usize);

        let next_next_possible_prime = self.next_possible_prime + (sieve_size as u64);

        let mut possible_primes = BitVec::from_elem(sieve_size as usize, false);

        // Tick off all known primes
        for prime in self.prime_multiples.iter_mut() {
            while (*prime).next_multiple < next_next_possible_prime {
                let index = (*prime).next_multiple - self.next_possible_prime;
                possible_primes.set(index as usize, true);
                (*prime).next_multiple += (*prime).prime;
            }
        }

        // Add all nonticked as new primes
        for index in 0..sieve_size {
            if let Some(false) = possible_primes.get(index as usize) {
                let prime = self.next_possible_prime + (index as u64);
                self.prime_multiples.push_back(PrimeListItem { prime, next_multiple: prime * prime });
            }
        }

        self.next_possible_prime = next_next_possible_prime;
    }

    fn factorize(&mut self, mut value: u64) -> Vec<u64> {
        let mut factors = Vec::new();

        while value > 1 {
            self.expand();

            for prime in self.prime_multiples.iter() {
                while value % (*prime).prime == 0 {
                    factors.push((*prime).prime);
                    value /= (*prime).prime;
                }
            }
        }

        factors
    }
}

pub fn execute(input: &String) {
    let number: u64 = input.parse().unwrap();

    let mut sieve = PrimeSieve::new();

    let factors = PrimeSieve::factorize(&mut sieve, number);

    for factor in factors {
        println!("{}", factor);
    }
}