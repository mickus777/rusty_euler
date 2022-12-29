use crate::math::utils::is_pandigital;

use std::collections::HashSet;

pub fn execute(input: &String) {
    let _ceiling: i32 = input.parse().unwrap();

    let mut products : HashSet<u64> = HashSet::new();

    let mut f1 = 2;
    let mut f2 = 3;
    loop {
        let product = f1 * f2;
        if product > 64000 {
            f1 += 1;
            if f1 * f1 > 64000 {
                break;
            }
            f2 = f1 + 1;
        } else {
            if is_pandigital(vec! { f1, f2, product }) {
                products.insert(product);
            }
            f2 += 1;
        }
    }

    let mut sum = 0;
    for product in products.iter() {
        sum += product;
    }

    println!("{}", sum);
}