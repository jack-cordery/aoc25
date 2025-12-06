// ok so we start with just sums of numbers
// and on input we read by any whitespace
// numbers are added column wise
// so we just need to think about how we read input
//

use std::{
    fs::read,
    io::{BufRead, Result},
    time::Instant,
};

#[derive(Debug, PartialEq)]
pub enum Operation {
    Add,
    Multiply,
    Divide,
    Subtract,
}

impl Operation {
    pub fn from_char(c: &char) -> Option<Self> {
        match c {
            '+' => Some(Self::Add),
            '-' => Some(Self::Subtract),
            '*' => Some(Self::Multiply),
            '/' => Some(Self::Divide),
            _ => None,
        }
    }
    pub fn operate(&self, a: &u64, b: &u64) -> u64 {
        match self {
            Self::Add => a + b,
            Self::Subtract => a - b,
            Self::Divide => a / b,
            Self::Multiply => a * b,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Column {
    nums: Vec<u64>,
    operation: Operation,
}

impl Column {
    pub fn new(nums: Vec<u64>, operation: Operation) -> Self {
        Self { nums, operation }
    }

    pub fn score(&self) -> u64 {
        let mut s = 0;
        if self.operation == Operation::Multiply {
            s = 1;
        }
        self.nums.iter().for_each(|n| {
            s = self.operation.operate(&s, n);
        });
        s
    }
}
pub fn day_six(path: &str) -> Result<()> {
    let now = Instant::now();

    let content = read(path)?;
    let lines: Vec<String> = content.lines().map(|l| l.unwrap()).collect();

    let char_lines: Vec<Vec<char>> = lines
        .iter()
        .rev()
        .map(|s| {
            let cs: Vec<&str> = s.split_whitespace().collect();
            let cs: Vec<char> = cs
                .iter()
                .map(|s| {
                    let chars: Vec<char> = s.chars().collect();
                    chars.first().unwrap().to_owned()
                })
                .collect();
            cs
        })
        .collect();

    let operations: Vec<Operation> = char_lines
        .first()
        .unwrap()
        .to_owned()
        .iter()
        .map(|a| Operation::from_char(a).unwrap())
        .collect();

    let columns: Vec<Vec<char>> = lines
        .iter()
        .rev()
        .skip(1)
        .map(|c| c.chars().rev().collect())
        .rev()
        .collect();

    let rows = columns.len();
    let cols = columns[0].len();

    let transposed: Vec<Vec<char>> = (0..cols)
        .map(|col| (0..rows).map(|row| columns[row][col]).collect())
        .collect();

    let transposed_filtered: Vec<Vec<char>> = transposed
        .into_iter()
        // .filter(|r| !r.iter().all(|i| i == &' ')) // get rid of this but then need a "_" on
        // each line
        .collect();

    println!("{:?}", transposed_filtered.len());

    let s: Vec<String> = transposed_filtered
        .iter()
        .map(|chunk| chunk.iter().collect())
        .collect();

    let ch = lines.len() - 1;

    println!("{:?}", ch);

    let splits: Vec<Vec<String>> = s
        .join("\n") // combine lines into one string with line breaks
        .split(" ".repeat(ch).as_str()) // split at 4 spaces
        .map(|chunk| {
            chunk
                .lines() // split each chunk back into lines
                .map(|line| line.to_string())
                .filter(|line| !line.is_empty())
                .collect::<Vec<String>>()
        })
        .collect();
    println!("{:?}", splits);
    //
    let actual_nums: Vec<Vec<u64>> = splits
        .iter()
        .map(|s| {
            s.iter()
                .map(|n| n.trim_start().trim_end())
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect();

    let score: u64 = actual_nums
        .iter()
        .rev()
        .zip(operations)
        .map(|(t, op)| {
            let s = Column::new(t.to_owned(), op).score();
            println!("{:?}= {:?}", t, s);
            s
        })
        .sum();

    println!(
        "the score is {} and it took {}us",
        score,
        now.elapsed().as_micros()
    );

    Ok(())
}

pub fn day_six_part_1(path: &str) -> Result<()> {
    let now = Instant::now();

    let content = read(path)?;
    let lines: Vec<String> = content.lines().map(|l| l.unwrap()).collect();

    let char_lines: Vec<Vec<char>> = lines
        .iter()
        .rev()
        .map(|s| {
            let cs: Vec<&str> = s.split_whitespace().collect();
            let cs: Vec<char> = cs
                .iter()
                .map(|s| {
                    let chars: Vec<char> = s.chars().collect();
                    chars.first().unwrap().to_owned()
                })
                .collect();
            cs
        })
        .collect();
    let operations: Vec<Operation> = char_lines
        .first()
        .unwrap()
        .to_owned()
        .iter()
        .map(|a| Operation::from_char(a).unwrap())
        .collect();

    let columns: Vec<Vec<u64>> = lines
        .iter()
        .rev()
        .skip(1)
        .map(|c| c.split_whitespace().map(|a| a.parse().unwrap()).collect())
        .collect();

    let rows = columns.len();
    let cols = columns[0].len();

    let transposed: Vec<Vec<u64>> = (0..cols)
        .map(|col| (0..rows).map(|row| columns[row][col]).collect())
        .collect();

    let score: u64 = transposed
        .iter()
        .zip(operations)
        .map(|(t, op)| Column::new(t.to_owned(), op).score())
        .sum();

    println!(
        "the score is {} and it took {}us",
        score,
        now.elapsed().as_micros()
    );

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_operation_from_char() {
        assert_eq!(Operation::from_char(&'+'), Some(Operation::Add));
        assert_eq!(Operation::from_char(&'-'), Some(Operation::Subtract));
        assert_eq!(Operation::from_char(&'*'), Some(Operation::Multiply));
        assert_eq!(Operation::from_char(&'/'), Some(Operation::Divide));
        assert_eq!(Operation::from_char(&'\\'), None);
    }

    #[test]
    fn test_operation_operate() {
        assert_eq!(Operation::from_char(&'+').unwrap().operate(&5, &10), 15);
        assert_eq!(Operation::from_char(&'-').unwrap().operate(&10, &5), 5);
        assert_eq!(Operation::from_char(&'*').unwrap().operate(&5, &10), 50);
        assert_eq!(Operation::from_char(&'/').unwrap().operate(&10, &5), 2);
    }

    #[test]
    fn test_column_new() {
        assert_eq!(
            Column::new(vec![1, 2, 3, 4, 5], Operation::Add),
            Column {
                nums: vec![1, 2, 3, 4, 5],
                operation: Operation::Add,
            }
        )
    }

    #[test]
    fn test_column_score() {
        let c = Column::new(vec![1, 2, 3, 4, 5], Operation::Add);
        assert_eq!(c.score(), 15);

        let c = Column::new(vec![1, 2, 3, 4, 5], Operation::Multiply);
        assert_eq!(c.score(), 120);
    }
}
