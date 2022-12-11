
fn sum_power_digits(mut value : u32, power : u32) -> u32 {
    let mut sum = 0;

    while value > 0 {
        let digit = value % 10;
        value /= 10;
        sum += digit.pow(power);
    }

    sum
}

pub fn execute(input: &String) {
    let power: u32 = input.parse().unwrap();

    let mut sum = 0;

    for value in 10..1000000 {
        let digit_sum = sum_power_digits(value, power);
        if value == digit_sum {
            println!("{}", value);
            sum += value;
        }
    }

    println!("Sum: {}", sum);
}