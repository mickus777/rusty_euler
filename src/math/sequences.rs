use std::cmp;
use std::vec::Vec;

use bit_vec::BitVec;
use num_bigint::BigUint;
use num_traits::One;
use num_traits::Zero;

pub struct TriangleNumbers {
    current_sum: u64,
    last_term: u64
}

impl TriangleNumbers {
    pub fn new() -> TriangleNumbers {
        TriangleNumbers { current_sum: 0, last_term: 0 }
    }

    pub fn next(&mut self) -> u64 {
        self.last_term += 1;
        self.current_sum += self.last_term;
        self.current_sum
    }
}

pub struct FibonacciNumbers {
    first_number: BigUint,
    second_number: BigUint
}

impl FibonacciNumbers {
    pub fn new() -> FibonacciNumbers {
        FibonacciNumbers { first_number: Zero::zero(), second_number: Zero::zero() }
    }

    pub fn next(&mut self) -> &BigUint {
        if self.second_number == Zero::zero() {
            self.second_number = One::one();
        } else if self.first_number == Zero::zero() {
            self.first_number = One::one();
        } else {
            let next_number = self.first_number.clone() + self.second_number.clone();
            self.first_number = self.second_number.clone();
            self.second_number = next_number;
        }
        &self.second_number
    }
}

pub struct Permutations<T> {
    pointers : Vec<usize>,
    position : usize,
    items : Vec<T>
}

impl<T> Permutations<T> {
    pub fn new(items : Vec<T>) -> Permutations<T> {
        let mut pointers = Vec::new();
        for _i in 0..items.len() {
            pointers.push(0);
        }
        Permutations { pointers: pointers, position: 0, items: items }
    }

    pub fn next(&mut self) -> Option<&Vec<T>> {
        if self.position == 0 {
            self.position = 1;
            return Option::Some(&self.items);            
        }

        loop {
            if self.position >= self.pointers.len() {
                return Option::None;
            }
    
            if self.pointers[self.position] < self.position {
                if self.position % 2 == 0 {
                    self.items.swap(0, self.position);
                } else {
                    self.items.swap(self.pointers[self.position], self.position);
                }
        
                self.pointers[self.position] += 1;
                self.position = 1;
                break;
            } else {
                self.pointers[self.position] = 0;
                self.position += 1;
            }
        }

        Option::Some(&self.items)
    }
}

struct PrimeNumberItem {
    prime: u64,
    next_multiple: u64
}

pub struct PrimeNumbers {
    next_possible_prime: u64,
    prime_multiples: Vec<PrimeNumberItem>,
    last_returned: Option<usize>
}

impl PrimeNumbers {
    pub fn new() -> PrimeNumbers {
        PrimeNumbers { next_possible_prime: 2, prime_multiples: Vec::new(), last_returned: Option::None }
    }

    pub fn next(&mut self) -> u64 {
        let mut index = 0;
        if let Some(last_index) = self.last_returned {
            index = last_index + 1;
        }

        if self.prime_multiples.len() <= index {
            self.expand();
        }

        self.last_returned = Some(index);

        self.prime_multiples[index].prime
    }

    pub fn factorize(&mut self, value: u64) -> Vec<u64> {
        let mut factors = Vec::new();

        let mut remainder = value;

        while remainder > 1 {
            if remainder >= self.next_possible_prime {
                self.expand();
            }

            for prime in self.prime_multiples.iter() {
                while remainder % (*prime).prime == 0 {
                    factors.push((*prime).prime);
                    remainder /= (*prime).prime;
                }
            }
        }

        factors
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
                self.prime_multiples.push(PrimeNumberItem { prime, next_multiple: prime * prime });
            }
        }

        self.next_possible_prime = next_next_possible_prime;
    }
}