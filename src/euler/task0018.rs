use std::cmp::max;
use std::vec::Vec;

fn split_into_ints(text: &String) -> Vec<u64> {
    let mut ints : Vec<u64> = Vec::new();

    for num in text.split(' ') {
        ints.push(num.parse().unwrap());
    }

    ints
}

fn triangle_ify(numbers : Vec<u64>) -> Vec<Vec<u64>> {
    let mut triangle : Vec<Vec<u64>> = Vec::new();
    let mut row = Vec::new();

    let mut row_index = 0;
    let mut column_index = 0;
    for number in numbers {
        if column_index > row_index {
            row_index += 1;
            column_index = 0;
            triangle.push(row);
            row = Vec::new();
        }

        row.push(number);

        column_index += 1;
    }
    triangle.push(row);

    triangle
}

pub fn execute(input: &String) {
    let numbers = split_into_ints(input);
    let mut triangle = triangle_ify(numbers);

    let mut row_index = 0;
    let mut column_index = 0;

    loop {
        if column_index > row_index {
            row_index += 1;
            column_index = 0;
        }
        if row_index >= triangle.len() {
            break;
        }

        let mut total_cost = triangle[row_index][column_index];
        if row_index > 0 {
            if column_index == 0 {
                total_cost += triangle[row_index - 1][column_index];
            } else if column_index == row_index {
                total_cost += triangle[row_index - 1][column_index - 1];
            } else {
                total_cost += max(triangle[row_index - 1][column_index], triangle[row_index - 1][column_index - 1]);
            }
        }
        triangle[row_index][column_index] = total_cost;

        column_index += 1;
    }

    println!("{:?}", triangle);
}