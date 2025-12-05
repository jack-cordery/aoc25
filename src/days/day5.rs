use std::{
    fs::{read, read_to_string},
    io::Result,
    time::Instant,
};

// ok so here we have "fresh" inclusive ranges
// and then a new line and a bunch of ids we need to count
// how many are in the fresh ranges
//
//
//
#[derive(Debug, PartialEq)]
pub struct Ranges {
    ranges: Vec<Range>,
}

impl Ranges {
    pub fn new(ranges: Vec<Range>) -> Self {
        Self { ranges }
    }
    pub fn score(&self, items: Vec<Item>) -> u64 {
        let mut score = 0;
        for i in items.iter() {
            for r in self.ranges.iter() {
                if r.is_item_fresh(i) {
                    score += 1;
                    break;
                }
            }
        }
        score
    }
}

#[derive(Debug, PartialEq)]
pub struct Range {
    start: u64,
    end: u64,
}

impl Range {
    pub fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }
    pub fn is_item_fresh(&self, item: &Item) -> bool {
        item.id <= self.end && item.id >= self.start
    }
}

#[derive(Debug, PartialEq)]
pub struct Item {
    id: u64,
}

impl Item {
    pub fn new(id: u64) -> Self {
        Self { id }
    }
}

pub fn day_five(path: &str) -> Result<()> {
    let now = Instant::now();

    let content = read_to_string(path)?;
    let mut content_split = content.split("\n\n");

    let ranges = content_split.next().unwrap();
    let items = content_split.next().unwrap();

    let ranges: Vec<Range> = ranges
        .split("\n")
        .map(|r| {
            let mut nums = r.split("-");
            let start = nums.next().unwrap().parse().unwrap();
            let end = nums.next().unwrap().parse().unwrap();
            Range::new(start, end)
        })
        .collect();

    let items: Vec<Item> = items
        .trim_end_matches("\n")
        .split("\n")
        .map(|i| {
            let id = i.parse().unwrap();
            Item::new(id)
        })
        .collect();

    let score = Ranges::new(ranges).score(items);

    println!(
        "todays score is {}, and took {}",
        score,
        now.elapsed().as_micros()
    );

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_range_new() {
        assert_eq!(Range::new(1, 10), Range { start: 1, end: 10 })
    }

    #[test]
    fn test_item_new() {
        assert_eq!(Item::new(10), Item { id: 10 })
    }

    #[test]
    fn test_range_is_item_fresh() {
        assert!(Range::new(1, 18).is_item_fresh(&Item::new(13)));
        assert!(Range::new(1, 18).is_item_fresh(&Item::new(18)));
        assert!(Range::new(1, 18).is_item_fresh(&Item::new(1)));
        assert!(!Range::new(1, 18).is_item_fresh(&Item::new(0)));
    }

    #[test]
    fn test_ranges_new() {
        let ranges = vec![
            Range::new(3, 5),
            Range::new(10, 14),
            Range::new(16, 20),
            Range::new(12, 18),
        ];

        let expected_ranges = vec![
            Range::new(3, 5),
            Range::new(10, 14),
            Range::new(16, 20),
            Range::new(12, 18),
        ];

        assert_eq!(
            Ranges::new(ranges),
            Ranges {
                ranges: expected_ranges
            }
        )
    }

    #[test]
    fn test_score() {
        let ranges = vec![
            Range::new(3, 5),
            Range::new(10, 14),
            Range::new(16, 20),
            Range::new(12, 18),
        ];

        let items = vec![
            Item::new(1),
            Item::new(5),
            Item::new(8),
            Item::new(11),
            Item::new(17),
            Item::new(32),
        ];

        assert_eq!(Ranges::new(ranges).score(items), 3);
    }
}
