fn calculate(sum: u32) -> u32 {
    for a in 1..sum {
        let mut b = a + 1;
        let mut c = sum - a - b;
        while b < c {
            if a * a + b * b == c * c {
                return a * b * c;
            }
            b += 1;
            c -= 1;
        }
    }

    0
}

pub fn execute(input: &String) {
    let sum: u32 = input.parse().unwrap();

    let product = calculate(sum);

    println!("{}", product);
}