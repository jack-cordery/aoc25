use std::{env, io};

use aoc25::days::{
    day1::day_one, day2::day_two, day3::day_three, day4::day_four, day5::day_five, day6::day_six,
    day7::day_seven, day8::day_eight,
};

fn main() -> io::Result<()> {
    let mut args = env::args();
    let (Some(_), Some(day), Some(path), None) =
        (args.next(), args.next(), args.next(), args.next())
    else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Expected day arg",
        ));
    };

    match day.as_str() {
        "day_one" => day_one(path.as_str())?,
        "day_two" => day_two(path.as_str())?,
        "day_three" => day_three(path.as_str())?,
        "day_four" => day_four(path.as_str())?,
        "day_five" => day_five(path.as_str())?,
        "day_six" => day_six(path.as_str())?,
        "day_seven" => day_seven(path.as_str())?,
        "day_eight" => day_eight(path.as_str())?,
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Expected day_x",
            ));
        }
    }
    Ok(())
}
