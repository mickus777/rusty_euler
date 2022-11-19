
#[derive(Debug)]
enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December
}

fn month_days(month : Month, year : i32) -> i32 {
    match month {
        Month::January => 31,
        Month::February => {
            if year % 400 == 0 {
                29
            } else if year % 100 == 0 {
                28
            } else if year % 4 == 0 {
                29
            } else {
                28
            }
        },
        Month::March => 31,
        Month::April => 30,
        Month::May => 31,
        Month::June => 30,
        Month::July => 31,
        Month::August => 31,
        Month::September => 30,
        Month::October => 31,
        Month::November => 30,
        Month::December => 31
    }
}

fn check_month(first_sunday : &mut i32, month : Month, year : i32) -> i32 {
    let mut count = 0;

    if *first_sunday == 1 {
        count += 1;
    }

    let days_of_month = month_days(month, year);
    while *first_sunday <= days_of_month {
        *first_sunday += 7;
    }
    *first_sunday -= days_of_month;

    count
}

pub fn execute(input: &String) {
    let _: i32 = input.parse().unwrap();

    let mut first_sunday = 7 - (365) % 7;
    let mut count = 0;

    for year in 1901..2001 {
        count += check_month(&mut first_sunday, Month::January, year);
        count += check_month(&mut first_sunday, Month::February, year);
        count += check_month(&mut first_sunday, Month::March, year);
        count += check_month(&mut first_sunday, Month::April, year);
        count += check_month(&mut first_sunday, Month::May, year);
        count += check_month(&mut first_sunday, Month::June, year);
        count += check_month(&mut first_sunday, Month::July, year);
        count += check_month(&mut first_sunday, Month::August, year);
        count += check_month(&mut first_sunday, Month::September, year);
        count += check_month(&mut first_sunday, Month::October, year);
        count += check_month(&mut first_sunday, Month::November, year);
        count += check_month(&mut first_sunday, Month::December, year);
    }

    println!("{}", count);
}