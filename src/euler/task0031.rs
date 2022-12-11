use std::collections::LinkedList;

struct Change {
    remaining : u32,
    least_denomination : u32
}

pub fn execute(input: &String) {
    let _ceiling: i32 = input.parse().unwrap();

    let denominations = vec![1, 2, 5, 10, 20, 50, 100, 200];

    let mut changes : LinkedList<Change> = LinkedList::new();
    changes.push_back(Change { remaining : 200, least_denomination : 1 });

    let mut count = 0;
 
    while changes.len() > 0 {
        let change = changes.pop_front().unwrap();

        if change.remaining == 0 {
            count += 1;
            continue;
        }

        for denomination in denominations.iter() {
            if *denomination >= change.least_denomination {
                if *denomination <= change.remaining {
                    changes.push_back(Change { remaining : change.remaining - *denomination, least_denomination : *denomination });
                }
            }
        }
    }

    println!("{}", count);
}