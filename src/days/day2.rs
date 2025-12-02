use core::str;
use std::{fs::read_to_string, io::Result, time::Instant};

pub fn day_two(path: &str) -> Result<()> {
    // so here we need to take in one line
    // split by , and then split by - and parse number to u64

    let now = Instant::now();

    let content = read_to_string(path)?;
    let ranges: Vec<Range> = content
        .trim_end()
        .split(',')
        .map(|pair| {
            let mut splits = pair.split('-');
            let (first, second) = (splits.next().unwrap(), splits.next().unwrap());
            Range::new(
                first.parse::<u64>().unwrap(),
                second.parse::<u64>().unwrap(),
            )
        })
        .collect();

    let sum: u64 = ranges.iter().map(|r| r.sum_invalid()).sum();
    println!(
        "the sum is {} and it took {}",
        sum,
        now.elapsed().as_micros()
    );
    Ok(())
}

/// in this day we are given id ranges which are inclusive on both ends
/// we need to take that range of numbers
/// and assign validitiy to them - they are invalid if the number
/// is represented by the same number twice i.e. 6464 or 11
/// so it should be pretty easy to do that by splitting the string repr number
/// in half and then checking for equality
///

#[derive(Debug, PartialEq)]
pub struct Id {
    stri: String,
    num: u64,
}

impl Id {
    pub fn new(num: u64) -> Self {
        Self {
            stri: num.to_string(),
            num,
        }
    }

    pub fn is_valid(&self) -> bool {
        // so now we need to find if there is a repeating set of characters
        // so rough guide is divide by numbers that have no remainder up to the length
        // for each of those split the string into that many equally spaced strings and check
        // equality aceoss each
        let l = self.stri.chars().count();

        for d in 2..=l {
            if l % d == 0 {
                let chunked: Vec<String> = self
                    .stri
                    .as_bytes()
                    .chunks(l / d)
                    .map(|s| str::from_utf8(s).unwrap().to_string())
                    .collect();

                let first = chunked.first().unwrap();
                if chunked.iter().all(|item| item == first) {
                    return false;
                }
            }
        }
        true
    }
}

#[derive(Debug, PartialEq)]
pub struct Range {
    start: Id,
    end: Id,
    range: Vec<Id>,
}

impl Range {
    pub fn new(start: u64, end: u64) -> Self {
        let range = (start..=end).map(Id::new).collect();
        Self {
            start: Id::new(start),
            end: Id::new(end),
            range,
        }
    }

    pub fn sum_invalid(&self) -> u64 {
        self.range
            .iter()
            .map(|v| match v.is_valid() {
                true => 0,
                false => v.num,
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_id_new() {
        assert_eq!(
            Id::new(10),
            Id {
                stri: "10".to_string(),
                num: 10
            }
        )
    }

    #[test]
    fn test_id_is_valid() {
        assert!(Id::new(100).is_valid());
        assert!(Id::new(1001).is_valid());
        assert!(!Id::new(11).is_valid());
        assert!(!Id::new(99).is_valid());
        assert!(Id::new(95).is_valid());
        assert!(!Id::new(121212).is_valid());
    }

    #[test]
    fn test_range_new() {
        assert_eq!(
            Range::new(1, 5),
            Range {
                start: Id::new(1),
                end: Id::new(5),
                range: vec![Id::new(1), Id::new(2), Id::new(3), Id::new(4), Id::new(5)]
            }
        )
    }

    #[test]
    fn test_sum_invalid() {
        let range = Range::new(38593856, 38593862);
        assert_eq!(range.sum_invalid(), 38593859);

        let range = Range::new(1698522, 1698528);
        assert_eq!(range.sum_invalid(), 0);

        let range = Range::new(11, 22);
        assert_eq!(range.sum_invalid(), 33);
    }
}
