use crate::math::utils::is_pandigital;

fn int_len(mut value : u64) -> usize {
    let mut size = 0;
    
    while value > 0 {
        size += 1;
        value /= 10;
    }

    size
}

fn concatenate_int(values : &Vec<u64>) -> u64 {
    let mut result = 0;

    for value in values.iter() {
        result *= 10u64.pow(int_len(*value) as u32);
        result += value;
    }

    result
}

pub fn execute(input: &String) {
    let _ceiling: i32 = input.parse().unwrap();

    let mut largest = 0;
 
    for value in 1..1000000 {
        let mut numbers = Vec::new();
        let mut multiplier = 1;
        let mut total_len = 0;
        loop {
            let product = multiplier * value;
            numbers.push(product);
            total_len += int_len(product);
            multiplier += 1;
            if total_len > 9 {
                break;
            } else if total_len == 9 {
                if is_pandigital(&numbers) {
                    let concatenated = concatenate_int(&numbers);
                    println!("{} {}", value, concatenated);
                    if concatenated > largest {
                        largest = concatenated;
                    }
                }
                break;
            }
        }
    }

    println!("=============");
    println!("Largest: {}", largest);
}