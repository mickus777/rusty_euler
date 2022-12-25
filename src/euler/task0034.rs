use std::collections::HashMap;

fn digit_factorial(mut number : u32, factorials : &HashMap<u32, u32>) -> u32 {
    let mut sum = 0;

    while number > 0 {
        let digit = number % 10;
        sum += factorials[&digit];
        number /= 10;
    }

    sum
}

pub fn execute(input: &String) {
    let ceiling: u32 = input.parse().unwrap();

    let mut sum = 0;
 
    let mut factorials : HashMap<u32, u32> = HashMap::new();
    factorials.insert(0, 1);
    let mut product = 1;
    for i in 1..10 {
        product *= i;
        factorials.insert(i, product);
    }

    for number in 3..ceiling {
        if number == digit_factorial(number, &factorials) {
            sum += number;
        }
    }

    println!("{}", sum);
}