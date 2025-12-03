// so we have a load of lines which are callled banks
// a bank has a load of batteries
// a battery has a joltagea
// the bank also has a joltage which is calculated
// by taking the largest possible number from each battery
//
//
//

use std::{
    fs::read,
    io::{BufRead, Result},
    time::Instant,
};

#[derive(Debug, PartialEq)]
pub struct Battery {
    joltage: u8,
}

pub struct Bank {
    batteries: Vec<Battery>,
    joltage: u8,
}

impl Battery {
    pub fn new(joltage: u8) -> Self {
        Self { joltage }
    }
}

impl Bank {
    fn new(batteries: String) -> Self {
        let batteries: Vec<Battery> = batteries
            .chars()
            .map(|c| {
                let d = c.to_string().parse().unwrap();
                Battery::new(d)
            })
            .collect();
        let joltage = Self::calculate_joltage(&batteries);
        Self { batteries, joltage }
    }

    fn calculate_joltage(batteries: &[Battery]) -> u8 {
        let mut first: u8 = 0;
        let mut first_pos: usize = 0;
        let mut second: u8 = 0;

        for (i, j) in batteries.iter().enumerate().take(batteries.len() - 1) {
            if j.joltage > first {
                first = j.joltage;
                first_pos = i;
            }
        }

        for j in batteries.iter().skip(first_pos + 1) {
            if j.joltage > second {
                second = j.joltage;
            }
        }

        first * 10 + second
    }
}

pub fn day_three(path: &str) -> Result<()> {
    let now = Instant::now();

    let content = read(path)?;

    let answer: u64 = content
        .lines()
        .map(|l| Bank::new(l.unwrap()).joltage as u64)
        .sum();

    println!(
        "the answer is {} and it took {}",
        answer,
        now.elapsed().as_micros()
    );

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_battery_new() {
        assert_eq!(Battery::new(8), Battery { joltage: 8 })
    }

    #[test]
    fn test_bank_new() {
        let batteries = vec![Battery::new(1), Battery::new(2), Battery::new(3)];
        let bank = Bank::new("123".to_string());
        assert_eq!(bank.batteries, batteries);
        assert_eq!(bank.joltage, 23);

        let bank = Bank::new("987654321111111".to_string());
        assert_eq!(bank.joltage, 98);

        let bank = Bank::new("818181911112111".to_string());
        assert_eq!(bank.joltage, 92);
    }

    #[test]
    fn test_bank_calculate_joltage() {
        let batteries = vec![Battery::new(1), Battery::new(2), Battery::new(3)];
        assert_eq!(Bank::calculate_joltage(&batteries), 23);

        let batteries = vec![Battery::new(9), Battery::new(3), Battery::new(2)];
        assert_eq!(Bank::calculate_joltage(&batteries), 93);
    }
}
