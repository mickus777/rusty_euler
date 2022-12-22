use std::collections::HashSet;

fn check_uniqe_digit(mut number : u64, digits : &mut HashSet<u64>) -> bool {
    while number > 0 {
        let digit = number % 10;
        if digit == 0 {
            return false;
        }
        number /= 10;
        if digits.contains(&digit) {
            return false;
        } else {
            digits.insert(digit);
        }
    }

    true
}

fn is_pandigital_product(f1 : u64, f2 : u64, product : u64) -> bool {
    let mut digits : HashSet<u64> = HashSet::new();

    if !check_uniqe_digit(f1, &mut digits) {
        false
    } else if !check_uniqe_digit(f2, &mut digits) {
        false
    } else if !check_uniqe_digit(product, &mut digits) {
        false
    } else {
        digits.len() == 9
    }
}

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
            if is_pandigital_product(f1, f2, product) {
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