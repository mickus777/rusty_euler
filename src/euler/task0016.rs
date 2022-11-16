use std::ops::Div;

use num_bigint::{BigUint, ToBigUint};
use num_traits::Zero;

pub fn execute(input: &String) {
    let exponent: u32 = input.parse().unwrap();

    let mut value = 2.to_biguint().unwrap().pow(exponent);

    let mut sum : BigUint = Zero::zero();

    while value > Zero::zero() {
        sum += value.modpow(&1.to_biguint().unwrap(), &10.to_biguint().unwrap());
        value = value.div(10.to_biguint().unwrap());
    }

    println!("{}", sum);
}