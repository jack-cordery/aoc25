use std::{
    fs::{read, read_to_string},
    io::Result,
    slice::Iter,
    time::Instant,
};

// ok so today we have a grid which is either free '.' or a roll of paper '@'
// we just need to count how many have less than 4 papers nearby
//
//
#[derive(Debug, PartialEq)]
pub struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq)]
pub enum Tile {
    Space,
    Paper,
}

pub enum Direction {
    Above,
    Left,
    Below,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Direction {
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 8] = [
            Direction::Above,
            Direction::Left,
            Direction::Below,
            Direction::Right,
            Direction::TopLeft,
            Direction::TopRight,
            Direction::BottomLeft,
            Direction::BottomRight,
        ];
        DIRECTIONS.iter()
    }
}

impl Tile {
    pub fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Space,
            '@' => Self::Paper,
            _ => panic!("neither space nor paper"),
        }
    }
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn look(&self, direction: &Direction, max_x: usize, max_y: usize) -> Option<Self> {
        match direction {
            Direction::Above => {
                let x = self.x;
                let y = self.y;
                if self.y < 1 {
                    return None;
                }
                Some(Self { x, y: y - 1 })
            }
            Direction::Below => {
                let x = self.x;
                let y = self.y;
                if self.y >= max_y {
                    return None;
                }
                Some(Self { x, y: y + 1 })
            }
            Direction::Left => {
                let x = self.x;
                let y = self.y;
                if self.x < 1 {
                    return None;
                }
                Some(Self { x: x - 1, y })
            }
            Direction::Right => {
                let x = self.x;
                let y = self.y;
                if self.x >= max_x {
                    return None;
                }
                Some(Self { x: x + 1, y })
            }
            Direction::TopLeft => {
                let x = self.x;
                let y = self.y;
                if self.y < 1 || self.x < 1 {
                    return None;
                }
                Some(Self { x: x - 1, y: y - 1 })
            }
            Direction::TopRight => {
                let x = self.x;
                let y = self.y;
                if self.y < 1 || self.x >= max_x {
                    return None;
                }
                Some(Self { x: x + 1, y: y - 1 })
            }
            Direction::BottomLeft => {
                let x = self.x;
                let y = self.y;
                if self.x < 1 || self.y >= max_y {
                    return None;
                }
                Some(Self { x: x - 1, y: y + 1 })
            }
            Direction::BottomRight => {
                let x = self.x;
                let y = self.y;
                if self.y >= max_y || self.x >= max_x {
                    return None;
                }
                Some(Self { x: x + 1, y: y + 1 })
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Grid {
    grid: Vec<Vec<Tile>>,
}

impl Grid {
    pub fn new(grid_str: &str) -> Self {
        let grid: Vec<Vec<Tile>> = grid_str
            .trim_end_matches('\n')
            .split('\n')
            .map(|row| row.chars().map(Tile::from_char).collect())
            .collect();
        Self { grid }
    }

    pub fn score(&self) -> u64 {
        let max_y = self.grid.len() - 1;
        let max_x = self.grid.first().unwrap().len() - 1;
        let mut score = 0;

        for (j, row) in self.grid.iter().enumerate() {
            for (i, _) in row.iter().enumerate().filter(|(_, t)| t == &&Tile::Paper) {
                let pos = Position::new(i, j);
                let mut local_score = 0;
                for d in Direction::iterator() {
                    let new_pos = pos.look(d, max_x, max_y);
                    if let Some(p) = new_pos {
                        if self.grid.get(p.y).unwrap().get(p.x).unwrap() == &Tile::Paper {
                            local_score += 1;
                        }
                    };
                }
                if local_score < 4 {
                    score += 1;
                }
            }
        }
        score
    }

    // this function will take a grid of tiles and remove the removable ones
    pub fn step(&mut self) -> usize {
        // given a state see which ones can be removed
        // remove them
        let max_y = self.grid.len() - 1;
        let max_x = self.grid.first().unwrap().len() - 1;
        let mut remove_buffer: Vec<Position> = vec![];

        for (j, row) in self.grid.iter().enumerate() {
            for (i, _) in row.iter().enumerate().filter(|(_, t)| t == &&Tile::Paper) {
                let pos = Position::new(i, j);
                let mut local_score = 0;
                for d in Direction::iterator() {
                    let new_pos = pos.look(d, max_x, max_y);
                    if let Some(p) = new_pos {
                        if self.grid.get(p.y).unwrap().get(p.x).unwrap() == &Tile::Paper {
                            local_score += 1;
                        }
                    };
                }
                if local_score < 4 {
                    remove_buffer.push(pos);
                }
            }
        }

        for p in remove_buffer.iter() {
            let p_change = self.grid.get_mut(p.y).unwrap().get_mut(p.x).unwrap();
            *p_change = Tile::Space;
        }

        remove_buffer.len()
    }

    pub fn step_until(&mut self) -> usize {
        let mut n = 1;
        let mut score = 0;
        while n != 0 {
            let new_n = self.step();
            n = new_n;
            score += n
        }
        score
    }
}

pub fn day_four(path: &str) -> Result<()> {
    let now = Instant::now();

    let content = read_to_string(path)?;

    let score = Grid::new(content.as_str()).step_until();

    println!(
        "score is {}, and it took {}",
        score,
        now.elapsed().as_micros()
    );

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tile_from_char() {
        assert_eq!(Tile::from_char('.'), Tile::Space);
        assert_eq!(Tile::from_char('@'), Tile::Paper);
    }

    #[test]
    #[should_panic]
    fn test_tile_from_char_panic() {
        assert_eq!(Tile::from_char('a'), Tile::Space);
    }

    #[test]
    fn test_position_new() {
        assert_eq!(Position::new(1, 2), Position { x: 1, y: 2 })
    }

    #[test]
    fn test_position_look() {
        assert_eq!(Position::new(0, 0).look(&Direction::Above, 1, 1), None);
        assert_eq!(
            Position::new(0, 0).look(&Direction::Below, 1, 1),
            Some(Position::new(0, 1))
        );
        assert_eq!(Position::new(0, 0).look(&Direction::Left, 1, 1), None);
        assert_eq!(
            Position::new(0, 0).look(&Direction::Right, 1, 1),
            Some(Position::new(1, 0))
        );
        assert_eq!(Position::new(0, 0).look(&Direction::TopLeft, 1, 1), None);
        assert_eq!(Position::new(0, 0).look(&Direction::TopRight, 1, 1), None);
        assert_eq!(Position::new(0, 0).look(&Direction::BottomLeft, 1, 1), None);
        assert_eq!(
            Position::new(0, 0).look(&Direction::BottomRight, 1, 1),
            Some(Position::new(1, 1))
        );
    }

    #[test]
    fn test_grid_new() {
        let input = "..@@\n@@..\n";
        assert_eq!(
            Grid::new(input).grid,
            vec![
                vec![Tile::Space, Tile::Space, Tile::Paper, Tile::Paper],
                vec![Tile::Paper, Tile::Paper, Tile::Space, Tile::Space],
            ]
        );

        let input = "..@@\n@@..";
        assert_eq!(
            Grid::new(input).grid,
            vec![
                vec![Tile::Space, Tile::Space, Tile::Paper, Tile::Paper],
                vec![Tile::Paper, Tile::Paper, Tile::Space, Tile::Space],
            ]
        );
    }

    #[test]
    fn test_grid_score() {
        let input = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";

        assert_eq!(Grid::new(input).score(), 13)
    }

    #[test]
    fn test_grid_step() {
        let input = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";

        let expected = ".......@..\n.@@.@.@.@@\n@@@@@...@@\n@.@@@@..@.\n.@.@@@@.@.\n.@@@@@@@.@\n.@.@.@.@@@\n..@@@.@@@@\n.@@@@@@@@.\n....@@@...";

        let mut actual = Grid::new(input);
        let n = actual.step();
        assert_eq!(n, 13);
        assert_eq!(actual, Grid::new(expected));
    }

    #[test]
    fn test_grid_step_until() {
        let input = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";

        let expected = "..........\n..........\n..........\n....@@....\n...@@@@...\n...@@@@@..\n...@.@.@@.\n...@@.@@@.\n...@@@@@..\n....@@@...";

        let mut grid = Grid::new(input);

        let n = grid.step_until();

        assert_eq!(n, 43);

        assert_eq!(grid, Grid::new(expected));
    }
}
