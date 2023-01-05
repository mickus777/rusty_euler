use std::collections::{linked_list, LinkedList};

fn split(mut value : u32) -> LinkedList<u32> {
    let mut list = LinkedList::new();

    while value > 0 {
        list.push_front(value % 10);
        value /= 10;
    }

    list
}

pub fn execute(input: &String) {
    let mut last: i32 = input.parse().unwrap();
    if last == 0 {
        last = 1000000;
    }

    let mut number = 1;
    let mut next_digit_count = 1;
    let mut digit_count = 1;
    let mut value = 1;
    loop {
        let mut list = split(value);
        while list.len() > 0 {
            let n = list.pop_front().unwrap();
            if digit_count == next_digit_count {
                println!("{}", n);
                number *= n;
                next_digit_count *= 10;
            }
            digit_count += 1;
        }
        if next_digit_count > last {
            break;
        }

        value += 1;
    }
    
    println!("=============");
    println!("Number: {}", number);
}