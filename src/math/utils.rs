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
