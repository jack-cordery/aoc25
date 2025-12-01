use std::{
    fs::read,
    io::{BufRead, Result},
    time::Instant,
};

//  ok so day consists of taking in inputs like L/R {num}
//  and a position which is 0-99 and loops around
//  we want to take in the move and then count how many times i end up at 0

pub fn day_one(path: &str) -> Result<()> {
    let now = Instant::now();
    let file = read(path)?;
    let moves: Vec<Move> = file
        .lines()
        .map(|l| {
            let bind = l.unwrap();
            let (dir, dis) = bind.split_at(1);
            let dir = match dir {
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!(),
            };
            let dis = dis.parse::<u16>().unwrap();
            Move::new(dir, dis)
        })
        .collect();

    let mut lock = Lock::new();

    lock.apply_multi(moves);

    println!(
        "the password is {} and it took {}",
        lock.zeroed,
        now.elapsed().as_micros()
    );

    Ok(())
}

#[derive(Debug, PartialEq, Clone)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
pub struct Move {
    direction: Direction,
    distance: u16,
}

#[derive(Debug, PartialEq)]
pub struct Lock {
    pos: u8,
    zeroed: u64,
}

impl Move {
    pub fn new(direction: Direction, distance: u16) -> Self {
        Self {
            direction,
            distance,
        }
    }
}

impl Default for Lock {
    fn default() -> Self {
        Self::new()
    }
}

impl Lock {
    pub fn new() -> Self {
        Self { pos: 50, zeroed: 0 }
    }

    // ok so lock needs to take a move which will change its position and zeroed
    // the trick is to implement left and right movement using mod
    //
    pub fn apply(&mut self, m: &Move) {
        let d: u8 = (m.distance % 100) as u8;

        let d = match m.direction {
            Direction::Right => d,
            Direction::Left => 100 - d,
        };

        let new_pos = (self.pos + d) % 100;
        if new_pos == 0 {
            self.zeroed += 1;
        }
        self.pos = new_pos
    }

    pub fn apply_multi(&mut self, moves: Vec<Move>) {
        moves.iter().for_each(|m| self.apply(m));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_move_new() {
        let actual = Move::new(Direction::Left, 2);
        let expected = Move {
            direction: Direction::Left,
            distance: 2,
        };
        assert_eq!(actual, expected);
        let actual = Move::new(Direction::Right, 5);
        let expected = Move {
            direction: Direction::Right,
            distance: 5,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_lock_new() {
        let actual = Lock::new();
        let expected = Lock { pos: 50, zeroed: 0 };
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_lock_apply() {
        let mut lock = Lock::new();

        lock.apply(&Move::new(Direction::Right, 5));
        assert_eq!(lock.pos, 55);
        assert_eq!(lock.zeroed, 0);

        lock.apply(&Move::new(Direction::Right, 44));
        assert_eq!(lock.pos, 99);
        assert_eq!(lock.zeroed, 0);

        lock.apply(&Move::new(Direction::Right, 1));
        assert_eq!(lock.pos, 0);
        assert_eq!(lock.zeroed, 1);

        lock.apply(&Move::new(Direction::Left, 150));
        assert_eq!(lock.pos, 50);
        assert_eq!(lock.zeroed, 1);
    }

    #[test]
    fn test_lock_apply_multi() {
        let mut lock = Lock::new();

        let moves: Vec<Move> = vec![
            Move::new(Direction::Right, 5),
            Move::new(Direction::Right, 44),
            Move::new(Direction::Right, 1),
            Move::new(Direction::Left, 150),
        ];

        lock.apply_multi(moves);

        assert_eq!(lock.pos, 50);
        assert_eq!(lock.zeroed, 1);
    }
}
