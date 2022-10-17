
pub fn execute(input: &String) {
    let ceiling: i32 = input.parse().unwrap();

    let mut sum = 0;

    let mut v1 = 1;
    let mut v2 = 2;

    while v2 < ceiling {
        if v2 % 2 == 0 {
            sum += v2;
        }

        let next = v1 + v2;
        v1 = v2;
        v2 = next;
    }

    println!("Sum: {}", sum);
}