use std::env;

pub mod euler;
pub mod math;

fn main() {

    let args: Vec<String> = env::args().collect();

    let task_no: i32 = args[1].parse().unwrap();
    let input = &args[2];

    match task_no {
        1 => euler::task0001::execute(input),
        2 => euler::task0002::execute(input),
        3 => euler::task0003::execute(input),
        4 => euler::task0004::execute(input),
        5 => euler::task0005::execute(input),
        6 => euler::task0006::execute(input),
        7 => euler::task0007::execute(input),
        8 => euler::task0008::execute(input),
        9 => euler::task0009::execute(input),
        _ => println!("Unknown task"),
    }
}
