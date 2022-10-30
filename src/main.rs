use std::env;

pub mod euler;

fn main() {

    let args: Vec<String> = env::args().collect();

    let task_no: i32 = args[1].parse().unwrap();
    let input = &args[2];

    match task_no {
        1 => euler::task0001::execute(input),
        2 => euler::task0002::execute(input),
        3 => euler::task0003::execute(input),
        _ => println!("Unknown task"),
    }
}
