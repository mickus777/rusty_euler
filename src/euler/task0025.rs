use crate::math::sequences::FibonacciNumbers;

pub fn execute(input: &String) {
    let ceiling: usize = input.parse().unwrap();

    let mut sequence = FibonacciNumbers::new();

    let mut count: usize = 0;

    loop {
        count += 1;

        let value = sequence.next();
        if value.to_string().len() == ceiling {
            break;
        }
    }

    println!("{}", count);
}