use crate::math::utils::is_palindromic;

pub fn execute(input: &String) {
    let exponent: u64 = input.parse().unwrap();

    let mut max_value = 10;
    for _ in 1..exponent {
        max_value *= 10;
    }
    let min_value = max_value / 10;
    max_value -= 1;

    let mut highest_palindrome = 0;
    let mut highest_f2 = 0;
    let mut f1 = max_value;
    loop {
        let mut f2 = max_value;
        while f2 >= min_value {
            let product = f1 * f2;
            if product <= highest_palindrome {
                break;
            }
            if is_palindromic(product) {
                highest_palindrome = product;
                highest_f2 = f2;
                break;
            }
            f2 -= 1;
        }
        f1 -= 1;
        if highest_f2 >= f1 {
            break;
        }
    }

    println!("{}", highest_palindrome);
}
