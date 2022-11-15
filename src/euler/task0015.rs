use std::vec::Vec;

pub fn execute(input: &String) {
    let mut size: usize = input.parse().unwrap();
    size += 1;

    let mut lattice : Vec<Vec<u64>> = Vec::new();

    for row_index in 0..size {
        let mut row = Vec::new();

        for column_index in 0..size {
            if row_index == 0 {
                row.push(1);
            } else {
                let mut value = *lattice.get(row_index - 1).unwrap().get(column_index).unwrap();
                if column_index > 0 {
                    value += row.get(column_index - 1).unwrap();
                }
                row.push(value);
            }
        }

        lattice.push(row);
    }

    println!("{}", lattice.get(size - 1).unwrap().get(size - 1).unwrap());
}