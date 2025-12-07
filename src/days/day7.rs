use std::{collections::HashMap, fs::read_to_string, io::Result, time::Instant};

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum Tile {
    Start,
    Space,
    Light,
    Point,
    Splitter,
}

impl Tile {
    pub fn from_char(c: &char) -> Self {
        match c {
            'S' => Self::Start,
            '.' => Self::Space,
            '|' => Self::Light,
            'v' => Self::Point,
            '^' => Self::Splitter,
            _ => panic!("unexpected char"),
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Self::Start => 'S',
            Self::Space => '.',
            Self::Light => '|',
            Self::Point => 'v',
            Self::Splitter => '^',
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Grid {
    grid: Vec<Vec<Tile>>,
    cursor: usize, // current last light ray
    splits: u16,
    timelines: u64,
}

impl Grid {
    pub fn new(char_grid: Vec<Vec<char>>) -> Self {
        Self {
            grid: char_grid
                .iter()
                .map(|row| row.iter().map(Tile::from_char).collect())
                .collect(),
            cursor: 0,
            splits: 0,
            timelines: 1,
        }
    }

    pub fn read(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        Self::new(grid)
    }

    pub fn step(&mut self) -> Option<()> {
        let [current, next] = self
            .grid
            .get_disjoint_mut([self.cursor, self.cursor + 1])
            .unwrap();
        current.iter_mut().enumerate().for_each(|(i, tile)| {
            // ok so i want to take a tile and if its
            // space -> do nothing
            // light -> panic
            // point -> look down and either split or fall straight down
            // Start -> light down
            // splitter -> nothing
            match tile {
                Tile::Space => {}
                Tile::Light => {
                    panic!("light shouldnt exist here")
                }
                Tile::Splitter => {}
                Tile::Point => {
                    // here we look down and if its a splitter
                    // we look left and right of the splitter and turn that cell into a pointer
                    //  it should be a space or a pointer or will never be another splitter
                    let next_tile = next.get_mut(i).unwrap();
                    match next_tile {
                        Tile::Space => {
                            *tile = Tile::Light;
                            *next_tile = Tile::Point;
                        }
                        Tile::Splitter => {
                            self.splits += 1;
                            *tile = Tile::Light;
                            let [left, right] = next.get_disjoint_mut([i - 1, i + 1]).unwrap();
                            *left = Tile::Point;
                            *right = Tile::Point;
                        }
                        Tile::Point => *tile = Tile::Light,
                        _ => panic!("should be space or splitter"),
                    }
                }
                Tile::Start => {
                    // here we want to look below and if space then turn it to
                    // a Point, and panic else
                    let next_tile = next.get_mut(i).unwrap();
                    match next_tile {
                        Tile::Space => *next_tile = Tile::Point,
                        _ => panic!("this has to be a space"),
                    }
                }
            };
        });
        self.cursor += 1;
        Some(())
    }

    pub fn step_until(&mut self) {
        let length = self.grid.first().unwrap().len();
        for _ in 0..length {
            self.step();
        }
    }
    pub fn timeline_step(&self) -> Vec<Self> {
        let mut result: Vec<Self> = vec![];
        let mut potential_flag = false;
        let mut self_clone = self.clone();
        let mut potential_clone = self.clone();
        let [current, next] = self_clone
            .grid
            .get_disjoint_mut([self.cursor, self.cursor + 1])
            .unwrap();
        current.iter_mut().enumerate().for_each(|(i, tile)| {
            // ok so i want to take a tile and if its
            // space -> do nothing
            // light -> panic
            // point -> look down and either split or fall straight down
            // Start -> light down
            // splitter -> nothing
            match tile {
                Tile::Space => {}
                Tile::Light => {
                    panic!("light shouldnt exist here")
                }
                Tile::Splitter => {}
                Tile::Point => {
                    // here we look down and if its a splitter
                    // we look left and right of the splitter and turn that cell into a pointer
                    //  it should be a space or a pointer or will never be another splitter
                    let next_tile = next.get_mut(i).unwrap();
                    match next_tile {
                        Tile::Space => {
                            *tile = Tile::Light;
                            *next_tile = Tile::Point;
                        }
                        Tile::Splitter => {
                            *tile = Tile::Light;
                            potential_clone = self.clone();
                            let left = next.get_mut(i - 1).unwrap();
                            *left = Tile::Point;
                            let right = potential_clone
                                .grid
                                .get_mut(self.cursor + 1)
                                .unwrap()
                                .get_mut(i + 1)
                                .unwrap();
                            *right = Tile::Point;
                            potential_flag = true;
                        }
                        Tile::Point => *tile = Tile::Light,
                        _ => panic!("should be space or splitter"),
                    }
                }
                Tile::Start => {
                    // here we want to look below and if space then turn it to
                    // a Point, and panic else
                    let next_tile = next.get_mut(i).unwrap();
                    match next_tile {
                        Tile::Space => {
                            *next_tile = Tile::Point;
                        }

                        _ => panic!("this has to be a space"),
                    }
                }
            };
        });
        self_clone.cursor += 1;
        result.push(self_clone);
        if potential_flag {
            potential_clone.cursor += 1;
            result.push(potential_clone);
        }
        result
    }
}

#[derive(Debug, PartialEq)]
pub struct Timelines {
    grids: Vec<Grid>,
    l: usize,
}

impl Timelines {
    pub fn new(grid: Grid) -> Self {
        let l = grid.grid.len();
        Self {
            grids: vec![grid],
            l,
        }
    }

    pub fn reduce(&mut self) {
        // put all grids with the same cursor row together into one
        // with a timeline count
        //
        let mut hmap: HashMap<&Vec<Tile>, Grid> = HashMap::new();
        for g in self.grids.iter() {
            let curr_cursor_row = g.grid.get(g.cursor).unwrap();
            let curr_timeline_count = g.timelines;

            if hmap.contains_key(curr_cursor_row) {
                let mut new_grid: Grid = hmap.get(curr_cursor_row).unwrap().clone();
                new_grid.timelines += curr_timeline_count;

                hmap.insert(curr_cursor_row, new_grid);
            } else {
                hmap.insert(curr_cursor_row, g.to_owned());
            };
        }
        self.grids = hmap.into_values().collect();
    }

    pub fn step(&mut self) {
        let new_grids = self
            .grids
            .iter()
            .flat_map(|g| g.timeline_step().to_vec())
            .collect();

        self.grids = new_grids;
    }

    pub fn step_until(&mut self) {
        for _ in 0..self.l - 1 {
            self.step();
            self.reduce();
        }
    }

    // so timelines needs to take each grid and do a step
    // where the step actually returns the new grids which will either be 1 or 2 grids
}

pub fn day_seven_1(path: &str) -> Result<()> {
    let now = Instant::now();
    let content = read_to_string(path)?;

    let mut grid = Grid::read(&content);

    grid.step_until();

    let result = grid.splits;

    println!(
        "the score is {} and it took {}",
        result,
        now.elapsed().as_micros()
    );

    Ok(())
}

pub fn day_seven(path: &str) -> Result<()> {
    let now = Instant::now();
    let content = read_to_string(path)?;

    let grid = Grid::read(&content);
    let mut timelines = Timelines::new(grid);

    timelines.step_until();

    let result: u64 = timelines.grids.iter().map(|g| g.timelines as u64).sum();

    println!(
        "the score is {} and it took {}",
        result,
        now.elapsed().as_micros()
    );

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tile_from_char() {
        assert_eq!(Tile::from_char(&'S'), Tile::Start);
        assert_eq!(Tile::from_char(&'.'), Tile::Space);
        assert_eq!(Tile::from_char(&'|'), Tile::Light);
        assert_eq!(Tile::from_char(&'v'), Tile::Point);
        assert_eq!(Tile::from_char(&'^'), Tile::Splitter);
    }

    #[test]
    #[should_panic]
    fn test_tile_from_char_panic() {
        let _ = Tile::from_char(&'£');
    }

    #[test]
    fn test_tile_to_char() {
        assert_eq!(Tile::from_char(&'S').to_char(), 'S');
        assert_eq!(Tile::from_char(&'.').to_char(), '.');
        assert_eq!(Tile::from_char(&'|').to_char(), '|');
        assert_eq!(Tile::from_char(&'v').to_char(), 'v');
        assert_eq!(Tile::from_char(&'^').to_char(), '^');
    }

    #[test]
    fn test_grid_new() {
        let grid = vec![vec!['.', '.', 'S'], vec!['.', '^', '.']];
        let expected = vec![
            vec![Tile::Space, Tile::Space, Tile::Start],
            vec![Tile::Space, Tile::Splitter, Tile::Space],
        ];

        assert_eq!(
            Grid::new(grid),
            Grid {
                grid: expected,
                cursor: 0,
                splits: 0,
                timelines: 1,
            }
        )
    }
    #[test]
    fn test_grid_read() {
        let input = "..S
.^.";
        let expected = vec![
            vec![Tile::Space, Tile::Space, Tile::Start],
            vec![Tile::Space, Tile::Splitter, Tile::Space],
        ];

        assert_eq!(
            Grid::read(input),
            Grid {
                grid: expected,
                cursor: 0,
                splits: 0,
                timelines: 1,
            }
        )
    }

    #[test]
    fn test_grid_step() {
        let mut grid = Grid::new(vec![vec!['.', '.', 'S'], vec!['.', '^', '.']]);
        let mut expected = Grid::new(vec![vec!['.', '.', 'S'], vec!['.', '^', 'v']]);
        expected.cursor = 1;

        grid.step();

        assert_eq!(grid, expected)
    }

    #[test]
    fn test_grid_step_more() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        let expected = ".......S.......
.......v.......
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        let mut grid = Grid::read(input);
        let mut expected = Grid::read(expected);

        expected.cursor = 1;

        grid.step();

        assert_eq!(grid, expected)
    }
    #[test]
    fn test_grid_step_more_2() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        let expected = ".......S.......
.......|.......
......|^|......
......|.|......
.....|^|^|.....
.....|.|.|.....
....|^|^|^|....
....|.|.|.|....
...|^|^|||^|...
...|.|.|||.|...
..|^|^|||^|^|..
..|.|.|||.|.|..
.|^|||^||.||^|.
.|.|||.||.||.|.
|^|^|^|^|^|||^|
v.v.v.v.v.vvv.v";
        let mut grid = Grid::read(input);
        let mut expected = Grid::read(expected);

        expected.cursor = 15;
        expected.splits = 21;

        for _ in 0..15 {
            grid.step();
        }

        assert_eq!(grid, expected)
    }
    #[test]
    fn test_grid_step_until() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        let mut grid = Grid::read(input);

        grid.step_until();

        assert_eq!(grid.splits, 21);
    }

    #[test]
    fn test_timelines_new() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!(
            Timelines::new(Grid::read(input)),
            Timelines {
                grids: vec![Grid::read(input)],
                l: 16,
            }
        )
    }
    #[test]
    fn test_timelines_step() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        let expected = ".......S.......
.......v.......
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

        let mut expected_grid = Grid::read(expected);
        expected_grid.cursor = 1;
        let mut timelines = Timelines::new(Grid::read(input));
        timelines.step();
        assert_eq!(
            timelines,
            Timelines {
                grids: vec![expected_grid],
                l: 16,
            }
        )
    }

    #[test]
    fn test_timelines_steps() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

        let mut timelines = Timelines::new(Grid::read(input));
        for _ in 0..15 {
            timelines.step();
        }
        assert_eq!(timelines.grids.len(), 40)
    }
    #[test]
    fn test_timelines_step_until() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

        let mut timelines = Timelines::new(Grid::read(input));
        timelines.step_until();
        assert_eq!(timelines.grids.iter().map(|t| t.timelines).sum::<u64>(), 40)
    }
}
