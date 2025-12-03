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
    joltage: u64,
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

    fn calculate_joltage(batteries: &[Battery]) -> u64 {
        let mut digits: Vec<u8> = vec![];
        let mut prev_pos: i16 = -1;
        for a in (1..=12).rev() {
            let mut prev: u8 = 0;
            for (i, j) in batteries
                .iter()
                .enumerate()
                .take(batteries.len() - a + 1)
                .skip((prev_pos + 1) as usize)
            {
                if j.joltage > prev {
                    prev = j.joltage;
                    prev_pos = i as i16;
                }
            }

            digits.push(prev);
        }

        let s: String = digits.iter().map(|d| d.to_string()).collect();
        s.parse().unwrap()
    }
}

pub fn day_three(path: &str) -> Result<()> {
    let now = Instant::now();

    let content = read(path)?;

    let answer: u64 = content.lines().map(|l| Bank::new(l.unwrap()).joltage).sum();

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
        let bank = Bank::new("987654321111111".to_string());
        assert_eq!(bank.joltage, 987654321111);

        let bank = Bank::new("818181911112111".to_string());
        assert_eq!(bank.joltage, 888911112111);
    }

    #[test]
    fn test_bank_calculate_joltage() {
        let batteries = vec![
            Battery::new(1),
            Battery::new(2),
            Battery::new(3),
            Battery::new(1),
            Battery::new(2),
            Battery::new(3),
            Battery::new(1),
            Battery::new(2),
            Battery::new(3),
            Battery::new(1),
            Battery::new(2),
            Battery::new(3),
        ];
        assert_eq!(Bank::calculate_joltage(&batteries), 123123123123);
    }
}
