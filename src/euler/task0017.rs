
fn digit_chars(value : u32) -> u32 {
    match value {
        1 => 3,
        2 => 3,
        3 => 5,
        4 => 4,
        5 => 4,
        6 => 3,
        7 => 5,
        8 => 5,
        9 => 4,
        10 => 3,
        11 => 6,
        12 => 6,
        13 => 8,
        14 => 8,
        15 => 7,
        16 => 7,
        17 => 9,
        18 => 8,
        19 => 8,
        20 => 6,
        30 => 6,
        40 => 5,
        50 => 5,
        60 => 5,
        70 => 7,
        80 => 6,
        90 => 6,
        _ => 0
    }
}

fn char_count(value : u32) -> u32 {
    if value == 1000 {
        11
    } else if value > 99 {
        let mut count = digit_chars(value / 100) + 7 + char_count(value % 100);
        if value % 100 > 0 {
            count += 3;
        }
        count
    } else if value > 19 {
        digit_chars(value - (value % 10)) + digit_chars(value % 10)
    } else {
        digit_chars(value)
    }
}

pub fn execute(input: &String) {
    let value: u32 = input.parse().unwrap();

    let mut sum = 0;
 
    for value in 1..(value + 1) {
        sum += char_count(value);
    }

    println!("{}", sum);
}
