
pub fn execute(input: &String) {
    let ceiling: i32 = input.parse().unwrap();

    let mut sum = 0;
 
    for value in 1..ceiling {
        if value % 3 == 0 || value % 5 == 0 {
            sum += value;
        }
    }

    println!("Sum: {}", sum);
}