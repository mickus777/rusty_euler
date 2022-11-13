pub fn execute(input: &String) {
    let count: u64 = input.parse().unwrap();

    let mut sum : u64 = 0;
    let mut square_sum : u64 = 0;

    for val in 1..(count + 1) {
        sum += val;
        square_sum += val * val;
    }
 
    let squared_sum : u64 = sum * sum;

    println!("{}", squared_sum - square_sum);
}