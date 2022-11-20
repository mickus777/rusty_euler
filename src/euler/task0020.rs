use std::ops::DivAssign;

use num_bigint::{BigUint, ToBigUint};
use num_traits::{One, FromPrimitive, Zero};

pub fn execute(input: &String) {
    let ceiling: u32 = input.parse().unwrap();

    let mut product : BigUint = One::one();
 
    for factor in 1..(ceiling + 1) {
        product *= BigUint::from_u32(factor).unwrap();
    }

    let mut sum : BigUint = Zero::zero();

    loop {
        if product == 0.to_biguint().unwrap() {
            break;
        }

        let addend = product.modpow(&1.to_biguint().unwrap(), &10.to_biguint().unwrap());
        sum += addend;
        product.div_assign(10.to_biguint().unwrap());
    }

    println!("Sum: {}", sum);
}