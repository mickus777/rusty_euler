
struct Fraction {
    dividend : u32,
    divisor : u32
}

static PRIMES : &'static [u32] = &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];

fn shorten_fraction(fraction : Fraction) -> Fraction {
    let mut dividend = fraction.dividend;
    let mut divisor = fraction.divisor;

    for prime in PRIMES {
        while dividend % prime == 0 && divisor % prime == 0 {
            dividend /= prime;
            divisor /= prime;
        }
    }

    Fraction { dividend: dividend, divisor: divisor }
}

fn attempt_cancel(fraction : Fraction) -> Option<Fraction> {
    let dividend_high = fraction.dividend / 10;
    let dividend_low = fraction.dividend % 10;
    let divisor_high = fraction.divisor / 10;
    let divisor_low = fraction.divisor % 10;
    for digit in 1..10 {
        let mut dividend_new = Option::None;
        if dividend_high == digit {
            dividend_new = Option::from(dividend_low);
        } else if dividend_low == digit {
            dividend_new = Option::from(dividend_high);
        }
        let mut divisor_new = Option::None;
        if divisor_high == digit {
            divisor_new = Option::from(divisor_low);
        } else if divisor_low == digit {
            divisor_new = Option::from(divisor_high);
        }
        if let Option::Some(dividend) = dividend_new {
            if let Option::Some(divisor) = divisor_new {
                if dividend != 0 && divisor != 0 {
                    return Option::Some(Fraction { dividend: dividend, divisor: divisor });
                }
            }
        }
    }

    Option::None
}

pub fn execute(input: &String) {
    let _ceiling: i32 = input.parse().unwrap();

    let mut product = Fraction { dividend : 1, divisor : 1 };

    for dividend in 11..100 {
        for divisor in (dividend + 1)..100 {
            let shortened = shorten_fraction(Fraction { dividend : dividend, divisor : divisor });
            if let Option::Some(fraction) = attempt_cancel(Fraction { dividend : dividend, divisor : divisor }) {
                let shortened_fraction = shorten_fraction(Fraction { dividend : fraction.dividend, divisor : fraction.divisor });
                if shortened.dividend == shortened_fraction.dividend && shortened.divisor == shortened_fraction.divisor {
                    product = shorten_fraction(Fraction { dividend : product.dividend * shortened_fraction.dividend, divisor : product.divisor * shortened_fraction.divisor });
                }
            }
        }
    }

    println!("{}", product.divisor);
}