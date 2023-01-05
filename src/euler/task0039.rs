
pub fn execute(input: &String) {
    let mut ceiling: i32 = input.parse().unwrap();
    if ceiling == 0 {
        ceiling = 1001;
    }

    let mut best_perimeter = 0;
    let mut best_perimeter_count = 0;

    for p in 1..ceiling {
        let mut count = 0;
        for a in 1..p {
            for b in a..p {
                let c = p - a - b;
                if c > 0 {
                    if a * a + b * b == c * c {
                        println!("{} : {} {} {}", p, a, b, c);
                        count += 1;
                    }
                }
            }
        }
        if count > best_perimeter_count {
            best_perimeter = p;
            best_perimeter_count = count;
        }
    }

    println!("=============");
    println!("Perimeter: {}, Count: {}", best_perimeter, best_perimeter_count);
}