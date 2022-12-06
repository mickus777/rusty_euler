
pub fn execute(input: &String) {
    let ceiling: i32 = input.parse().unwrap();

    let mut sum = 1;

    let mut gap = 2;
    let mut value = 1;
    while gap < ceiling {
        for _i in 0..4 {
            value += gap;
            sum += value;
        }
        gap += 2;
    }
    
    println!("Sum: {}", sum);
}
