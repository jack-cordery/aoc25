use std::{env, io};

use aoc25::days::{day1::day_one, day2::day_two};

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
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Expected day_x",
            ));
        }
    }
    Ok(())
}
