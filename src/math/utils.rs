use std::vec::Vec;

pub fn proper_divisors(value: u64) -> Vec<u64> {
    let mut divisors = Vec::new();
    divisors.push(1);

    let mut divisor = 2;
    loop {
        if divisor * divisor > value {
            break;
        }

        if value % divisor == 0 {
            divisors.push(divisor);
            if divisor * divisor != value {
                divisors.push(value / divisor);
            }
        }

        divisor += 1;
    }

    divisors
}

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

pub fn is_palindromic(value: u64) -> bool {
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

