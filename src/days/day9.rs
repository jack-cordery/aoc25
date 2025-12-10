// ok so we have a load of red tiles which have a position x,y
// we need to calculate the maximum area you can make between any two tiles

use std::cmp::max;
use std::fs::read_to_string;
use std::io::Result;
use std::time::Instant;

#[derive(Debug, PartialEq)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn read(pos: &str) -> Self {
        let (x, y) = pos.split_once(",").unwrap();
        let x = x.parse::<usize>().unwrap();
        let y = y.parse::<usize>().unwrap();
        Self { x, y }
    }
}

#[derive(Debug, PartialEq)]
pub struct Red {
    pos: Position,
}

impl Red {
    pub fn read(pos: &str) -> Self {
        Self {
            pos: Position::read(pos),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Tiles {
    tiles: Vec<Red>,
}

impl Tiles {
    pub fn new(tiles: Vec<Red>) -> Self {
        Self { tiles }
    }

    pub fn read(tiles: &str) -> Self {
        let tiles = tiles.lines().map(Red::read).collect();

        Self { tiles }
    }

    pub fn largest_rect(&self) -> u64 {
        let mut result = 0;
        for (i, t) in self.tiles.iter().enumerate() {
            for another_t in self.tiles.iter().skip(i + 1) {
                let (t_x, t_y) = (t.pos.x as i64, t.pos.y as i64);
                let (another_t_x, another_t_y) = (another_t.pos.x as i64, another_t.pos.y as i64);

                let area = ((t_x - another_t_x).abs() + 1) * ((t_y - another_t_y).abs() + 1);
                result = max(result, area);
            }
        }
        result as u64
    }
}

pub fn day_nine(path: &str) -> Result<()> {
    let now = Instant::now();

    let content = read_to_string(path)?;

    let tiles = Tiles::read(&content);

    let score = tiles.largest_rect();

    println!(
        "the score is {score} and  it took {}",
        now.elapsed().as_micros()
    );

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_position_read() {
        assert_eq!(Position::read("1,10"), Position { x: 1, y: 10 })
    }

    #[test]
    fn test_red_read() {
        assert_eq!(
            Red::read("1,10"),
            Red {
                pos: Position { x: 1, y: 10 }
            }
        )
    }

    #[test]
    fn test_tiles_new() {
        assert_eq!(
            Tiles::new(vec![Red::read("1,10"), Red::read("2,10")]),
            Tiles {
                tiles: vec![Red::read("1,10"), Red::read("2,10")]
            }
        );
    }
    #[test]
    fn test_tiles_read() {
        let input = "1,10\n2,10\n";
        assert_eq!(
            Tiles::read(input),
            Tiles {
                tiles: vec![Red::read("1,10"), Red::read("2,10")]
            }
        );
    }

    #[test]
    fn test_tiles_largest_rect() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        let tiles = Tiles::read(input);
        assert_eq!(tiles.largest_rect(), 50);
    }
}
