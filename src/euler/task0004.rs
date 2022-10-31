fn listify_number(mut value: u64) -> Vec<u64> {
    let mut digits = Vec::new();
    while value > 0 {
        let digit = value % 10;
        value /= 10;
        digits.push(digit);
    }
    digits
}

fn is_palindromic_list(digits : &[u64]) -> bool {
    match digits {
        [first, middle @ .., last] => first == last && is_palindromic_list(middle),
        [] | [_] => true
    }
}

fn is_palindromic(value: u64) -> bool {
    let digits = listify_number(value);
    is_palindromic_list(&digits[..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_digit_is_palindromic() {
        let value = 1;
        assert_eq!(true, is_palindromic(value));
    }

    #[test]
    fn value_23_is_not_palindromic() {
        let value = 23;
        assert_eq!(false, is_palindromic(value));
    }

    #[test]
    fn value_1001_is_palindromic() {
        let value = 1001;
        assert_eq!(true, is_palindromic(value));
    }

    #[test]
    fn value_900099_is_not_palindromic() {
        let value = 900099;
        assert_eq!(false, is_palindromic(value));
    }
}

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
