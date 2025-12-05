use std::{
    cmp::{max, min},
    fs::read_to_string,
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

    pub fn combine(&mut self) {
        // we want to have the simplest repr
        // want to pairwise combine them from the left
        //

        let mut buffer = vec![self.ranges.first().unwrap().to_owned()];

        self.ranges.iter().skip(1).for_each(|r| {
            // from the left of the buffer i want to combine with r and take that result and
            // combine the nect elemeent of the buffer
            //
            // so i need to go to
            let mut r_clone = r.clone();
            let mut local = vec![];
            for b in buffer.iter() {
                // everything in buffer is always exclusive
                let combined = r_clone.combine(b);
                if combined.len() == 1 {
                    // r combined with b to make a new extended range
                    // so now for the next b we continue with r+b
                    r_clone = combined.first().unwrap().to_owned();
                } else {
                    // r and b are exlusive so now for the next b we continue with r
                    local.push(b.to_owned())
                }
            }
            // this is now all of the exclusive bs and the combined r+bs
            local.push(r_clone);
            local.sort_by(|a, b| a.start.cmp(&b.start));
            buffer = local;
        });

        self.ranges = buffer;
    }
}

#[derive(Debug, PartialEq, Clone)]
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

    pub fn combine(&self, another: &Range) -> Vec<Self> {
        // this will either extend or return two
        if another.start <= self.end && another.end >= self.start {
            // they can be combined
            let new = vec![Self::new(
                min(another.start, self.start),
                max(another.end, self.end),
            )];
            return new;
        };
        let mut new = vec![self.clone(), another.clone()];
        new.sort_by(|a, b| a.start.cmp(&b.start));
        new
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

    let ranges: Vec<Range> = ranges
        .split("\n")
        .map(|r| {
            let mut nums = r.split("-");
            let start = nums.next().unwrap().parse().unwrap();
            let end = nums.next().unwrap().parse().unwrap();
            Range::new(start, end)
        })
        .collect();

    let mut r = Ranges::new(ranges);
    r.combine();

    let score: u64 = r.ranges.iter().map(|r| (r.end - r.start) + 1).sum();

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
    fn test_range_combine() {
        let r1 = Range::new(1, 2);
        let r2 = Range::new(2, 6);
        assert_eq!(r1.combine(&r2), vec![Range::new(1, 6)]);

        let r1 = Range::new(1, 2);
        let r2 = Range::new(2, 6);
        assert_eq!(r2.combine(&r1), vec![Range::new(1, 6)]);

        let r1 = Range::new(1, 2);
        let r2 = Range::new(4, 6);
        assert_eq!(r2.combine(&r1), vec![Range::new(1, 2), Range::new(4, 6)]);
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
    #[test]
    fn test_combine() {
        let ranges = vec![
            Range::new(3, 5),
            Range::new(10, 14),
            Range::new(16, 20),
            Range::new(12, 18),
        ];

        let expected = Ranges::new(vec![Range::new(3, 5), Range::new(10, 20)]);
        let mut actual = Ranges::new(ranges);
        actual.combine();

        assert_eq!(actual, expected);
    }
}
