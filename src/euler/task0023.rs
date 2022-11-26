use bit_vec::BitVec;

use crate::math::utils;

enum NumberPerfectness {
    Deficient,
    Perfect,
    Abundant
}

fn find_perfectness(number : u64) -> NumberPerfectness {
    let sum : u64 = utils::proper_divisors(number).iter().sum();

    if sum == number {
        NumberPerfectness::Perfect
    } else if sum < number {
        NumberPerfectness::Deficient
    } else {
        NumberPerfectness::Abundant
    }
}

pub fn execute(input: &String) {
    let ceiling: u64 = input.parse().unwrap();

    let mut abundant_numbers : Vec<u64> = Vec::new();
    let mut abundant_sum = BitVec::from_elem((ceiling + 1) as usize, false);
    let mut sum : u64 = 0;

    for number in 1..ceiling {
        if let NumberPerfectness::Abundant = find_perfectness(number) {
            abundant_numbers.push(number);
            for abundant_number in abundant_numbers.iter() {
                let two_abundant_numbers = abundant_number + number;
                if two_abundant_numbers <= ceiling {
                    abundant_sum.set(two_abundant_numbers as usize, true);
                } else {
                    break;
                }
            }
        }

        if !abundant_sum.get(number as usize).unwrap() {
            sum += number;
        }
    }

    println!("{}", sum);
}