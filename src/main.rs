use std::env;

pub mod euler;
pub mod math;

fn main() {

    let args: Vec<String> = env::args().collect();

    let task_no: i32 = args[1].parse().unwrap();
    let mut input = &"0".to_string();
    if args.len() > 2 {
        input = &args[2];
    }

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
        10 => euler::task0010::execute(input),
        11 => euler::task0011::execute(input),
        12 => euler::task0012::execute(input),
        13 => euler::task0013::execute(input),
        14 => euler::task0014::execute(input),
        15 => euler::task0015::execute(input),
        16 => euler::task0016::execute(input),
        17 => euler::task0017::execute(input),
        18 => euler::task0018::execute(input),
        19 => euler::task0019::execute(input),
        20 => euler::task0020::execute(input),
        21 => euler::task0021::execute(input),
        22 => euler::task0022::execute(input),
        23 => euler::task0023::execute(input),
        24 => euler::task0024::execute(input),
        25 => euler::task0025::execute(input),
        26 => euler::task0026::execute(input),
        27 => euler::task0027::execute(input),
        28 => euler::task0028::execute(input),
        29 => euler::task0029::execute(input),
        30 => euler::task0030::execute(input),
        31 => euler::task0031::execute(input),
        32 => euler::task0032::execute(input),
        33 => euler::task0033::execute(input),
        34 => euler::task0034::execute(input),
        35 => euler::task0035::execute(input),
        36 => euler::task0036::execute(input),
        37 => euler::task0037::execute(input),
        38 => euler::task0038::execute(input),
        39 => euler::task0039::execute(input),
        _ => println!("Unknown task"),
    }
}
