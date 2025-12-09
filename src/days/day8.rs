// so we need a few structs and that
// we need junction, we need circuit, we need 3D position
//

use std::{
    collections::{HashMap, HashSet},
    fs::read,
    io::{self, BufRead},
    time::Instant,
};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Position {
    x: u32,
    y: u32,
    z: u32,
}

impl Position {
    pub fn new(x: u32, y: u32, z: u32) -> Self {
        Self { x, y, z }
    }

    pub fn read(pos_str: &str) -> Result<Self, GridError> {
        let nums: Vec<_> = pos_str.split(",").map(|n| n.parse::<u32>()).collect();

        match nums.as_slice() {
            [Ok(x), Ok(y), Ok(z)] => Ok(Self {
                x: *x,
                y: *y,
                z: *z,
            }),
            _ => Err(GridError::InvalidPositionString),
        }
    }

    pub fn distance_squared(&self, another: &Self) -> u64 {
        let mut sum = 0u64;
        if self.x > another.x {
            sum += (((self.x) - another.x) as u64) * ((self.x - another.x) as u64);
        } else {
            sum += (((another.x) - self.x) as u64) * ((another.x - self.x) as u64);
        }
        if self.y > another.y {
            sum += (((self.y) - another.y) as u64) * ((self.y - another.y) as u64);
        } else {
            sum += (((another.y) - self.y) as u64) * ((another.y - self.y) as u64);
        }
        if self.z > another.z {
            sum += (((self.z) - another.z) as u64) * ((self.z - another.z) as u64);
        } else {
            sum += (((another.z) - self.z) as u64) * ((another.z - self.z) as u64);
        }
        sum
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct JunctionBox {
    id: usize,
    pos: Position,
}

impl JunctionBox {
    pub fn new(id: usize, pos: Position) -> Self {
        Self { id, pos }
    }

    pub fn distance_squared(&self, another: &Self) -> u64 {
        self.pos.distance_squared(&another.pos)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Connection {
    from: usize,
    to: usize,
}

impl Connection {
    pub fn new(from: &usize, to: &usize) -> Self {
        if from < to {
            Self {
                from: *from,
                to: *to,
            }
        } else {
            Self {
                from: *to,
                to: *from,
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Circuit {
    boxes: Vec<JunctionBox>,
    connections: HashMap<Connection, u64>, // ordered lower to higher i.e. 1,2 never 2,1
    box_set: HashSet<usize>,               // ids of boxes in the circuit
}

#[derive(Debug, PartialEq)]
pub enum GridError {
    DuplicateIDs,
    InvalidPositionString,
}

impl Circuit {
    pub fn new(abox: JunctionBox, another: JunctionBox) -> Result<Self, GridError> {
        if abox.id == another.id {
            return Err(GridError::DuplicateIDs);
        }
        let boxes = vec![abox.clone(), another.clone()];
        let mut connections = HashMap::new();
        connections.insert(
            Connection::new(&abox.id, &another.id),
            abox.distance_squared(&another),
        );
        let mut box_set = HashSet::new();
        box_set.insert(abox.id);
        box_set.insert(another.id);
        Ok(Self {
            boxes,
            connections,
            box_set,
        })
    }

    fn check_is_in(&self, conn: &Connection) -> Option<usize> {
        if self.box_set.contains(&conn.from) {
            Some(conn.to)
        } else if self.box_set.contains(&conn.to) {
            Some(conn.from)
        } else {
            None
        }
    }

    fn add(&mut self, j: &JunctionBox, conn: &Connection, distance: u64) {
        // add box to boxes
        // add id to box_set
        // add connection to hmap
        self.boxes.push(j.clone());
        self.box_set.insert(j.id);
        self.connections.insert(conn.clone(), distance);
    }
}

#[derive(Debug)]
pub struct Grid {
    circuits: Vec<Circuit>,
    junction_boxes: Vec<JunctionBox>,
    distance_stack: Vec<(Connection, u64)>, // sorted by distance and contains all possible
}

impl Grid {
    // this is going to need to do the grunt work
    // we will want like an evolve method that takes the next jb in the distance_stack
    // and acds it to circuits
    // and then eventually get the score which is the len of the top 3 circuits
    //
    pub fn new(junction_pos: Vec<Position>) -> Self {
        // so when i get a new one from a vec of positions i want to
        // put them into junction boxes, intialise an empty
        let junction_boxes: Vec<JunctionBox> = junction_pos
            .iter()
            .enumerate()
            .map(|(i, jp)| JunctionBox::new(i, *jp))
            .collect();

        let circuits: Vec<Circuit> = vec![];

        let distance_stack = Self::get_distance_stack(&junction_boxes);

        Self {
            junction_boxes,
            circuits,
            distance_stack,
        }
    }

    fn get_distance_stack(jbs: &[JunctionBox]) -> Vec<(Connection, u64)> {
        let mut distance_stack = vec![];
        for (i, jb) in jbs.iter().enumerate() {
            for another in jbs.iter().skip(i + 1) {
                let d = jb.distance_squared(another);
                let connection = Connection::new(&jb.id, &another.id);
                distance_stack.push((connection, d));
            }
        }
        distance_stack.sort_by(|a, b| b.1.cmp(&a.1));
        distance_stack
    }

    fn merge_circuits(first: &Circuit, second: &Circuit, conn: &Connection, d: u64) -> Circuit {
        // so append both boxes and include new box
        // add the connection and distance to connection hashmap
        // merge both connection hashmaps
        // merge both box_set hashsets and include new junction box id
        let first = first.clone();
        let mut second = second.clone();
        let mut boxes = first.boxes.clone();
        boxes.append(&mut second.boxes);
        let mut connections = first.connections;
        connections.extend(second.connections);
        connections.insert(conn.clone(), d);
        let mut box_set = first.box_set;
        box_set.extend(second.box_set.iter());
        Circuit {
            boxes,
            box_set,
            connections,
        }
    }

    pub fn evolve(&mut self) -> Connection {
        let (connection, d) = self.distance_stack.pop().unwrap();

        let matched_indices: Vec<usize> = self
            .circuits
            .clone()
            .iter()
            .enumerate()
            .filter(|(_, c)| c.check_is_in(&connection).is_some())
            .map(|(i, _)| i)
            .collect();

        match matched_indices.len() {
            0 => {
                //new
                //add a new circuit
                let first = self
                    .junction_boxes
                    .clone()
                    .into_iter()
                    .find(|jb| jb.id == connection.from)
                    .unwrap();
                let second = self
                    .junction_boxes
                    .clone()
                    .into_iter()
                    .find(|jb| jb.id == connection.to)
                    .unwrap();
                self.circuits
                    .push(Circuit::new(first, second).expect("as pulled from job"));
            }
            1 => {
                //new
                //add connection to circuit
                let circ: &mut Circuit = self
                    .circuits
                    .get_mut(*matched_indices.first().unwrap())
                    .unwrap();
                let id = circ.check_is_in(&connection).expect("already checked");
                let first = self
                    .junction_boxes
                    .clone()
                    .into_iter()
                    .find(|jb| jb.id == id)
                    .unwrap();
                circ.add(&first, &connection, d);
            }
            2 => {
                //merge circuits on conn
                let [c1, c2] = self
                    .circuits
                    .get_disjoint_mut([
                        *matched_indices.first().unwrap(),
                        *matched_indices.get(1).unwrap(),
                    ])
                    .unwrap();
                let j = matched_indices.get(1).unwrap();
                let i = matched_indices.first().unwrap();
                let new_c = Self::merge_circuits(c1, c2, &connection, d);
                self.circuits.remove(*j);
                self.circuits.remove(*i);
                self.circuits.push(new_c);
            }
            _ => panic!("should be one or two"),
        }
        connection
    }

    pub fn score(&mut self, n: usize) -> u64 {
        for _ in 0..n {
            self.evolve();
        }

        self.circuits
            .sort_by(|a, b| b.box_set.len().cmp(&a.box_set.len()));

        self.circuits
            .iter()
            .take(3)
            .map(|c| c.box_set.len() as u64)
            .product()
    }

    pub fn score_2(&mut self, total_size: usize) -> u64 {
        // go until length of circuits is one
        // and the X coorfs of the two jbs that cause it multiplied is the score
        loop {
            let popped = self.evolve();
            let first = self
                .junction_boxes
                .iter()
                .find(|jb| jb.id == popped.from)
                .unwrap();
            let second = self
                .junction_boxes
                .iter()
                .find(|jb| jb.id == popped.to)
                .unwrap();
            let l = self.circuits.len();
            if l == 1 && self.circuits.iter().map(|c| c.box_set.len()).sum::<usize>() >= total_size
            {
                return first.pos.x as u64 * second.pos.x as u64;
            }
        }
    }
}

pub fn day_eight(path: &str) -> io::Result<()> {
    let now = Instant::now();
    let content = read(path)?;

    let lines: Vec<Position> = content
        .clone()
        .lines()
        .map(|l| Position::read(l.unwrap().as_str()).unwrap())
        .collect();

    let mut g = Grid::new(lines.clone());

    let score = g.score_2(lines.len());

    println!(
        "score is {} and it took {}us",
        score,
        now.elapsed().as_micros()
    );
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_position_new() {
        assert_eq!(Position::new(1, 2, 3), Position { x: 1, y: 2, z: 3 });
    }

    #[test]
    fn test_position_read() {
        assert_eq!(Position::read("1,2,3"), Ok(Position { x: 1, y: 2, z: 3 }));
        assert_eq!(
            Position::read("1,2,3,4"),
            Err(GridError::InvalidPositionString)
        );

        assert_eq!(
            Position::read("hello,there, iam"),
            Err(GridError::InvalidPositionString)
        );
    }
    #[test]
    fn test_position_distance_squared() {
        let actual = Position::new(0, 0, 0).distance_squared(&Position::new(2, 2, 2));
        assert_eq!(actual, 12);

        let actual = Position::new(0, 0, 0).distance_squared(&Position::new(2, 0, 0));
        assert_eq!(actual, 4);
        let actual = Position::new(0, 0, 0).distance_squared(&Position::new(0, 2, 0));
        assert_eq!(actual, 4);
        let actual = Position::new(0, 0, 0).distance_squared(&Position::new(0, 0, 2));
        assert_eq!(actual, 4);
    }
    #[test]
    fn test_junction_box_new() {
        assert_eq!(
            JunctionBox::new(1, Position::new(1, 2, 3)),
            JunctionBox {
                id: 1,
                pos: Position::new(1, 2, 3)
            }
        )
    }

    #[test]
    fn test_junction_box_distance_squared() {
        let jb = JunctionBox::new(1, Position::new(0, 0, 0));
        let jb2 = JunctionBox::new(2, Position::new(2, 1, 3));
        assert_eq!(jb.distance_squared(&jb2), 14);
        assert_eq!(jb2.distance_squared(&jb), 14);
    }

    #[test]
    fn test_connection_new() {
        assert_eq!(Connection::new(&1, &2), Connection { from: 1, to: 2 });
        assert_eq!(Connection::new(&2, &1), Connection { from: 1, to: 2 })
    }

    #[test]
    fn test_circuit_new() {
        let jb = JunctionBox::new(1, Position::new(0, 0, 0));
        let jb2 = JunctionBox::new(2, Position::new(2, 1, 3));
        let c = Circuit::new(jb.clone(), jb2.clone()).unwrap();

        let mut expected_connections = HashMap::new();
        let mut expected_box_set = HashSet::new();

        expected_connections.insert(Connection::new(&1, &2), 14);
        expected_box_set.insert(1);
        expected_box_set.insert(2);

        let expected = Circuit {
            boxes: vec![jb, jb2],
            connections: expected_connections,
            box_set: expected_box_set,
        };

        assert_eq!(c, expected)
    }
    #[test]
    fn test_circuit_check_is_in() {
        let jb = JunctionBox::new(1, Position::new(0, 0, 0));
        let jb2 = JunctionBox::new(2, Position::new(2, 1, 3));
        let c = Circuit::new(jb.clone(), jb2.clone()).unwrap();

        assert_eq!(c.check_is_in(&Connection::new(&2, &3)), Some(3));
        assert_eq!(c.check_is_in(&Connection::new(&4, &3)), None);
    }

    #[test]
    fn test_grid_get_distance_stack() {
        let jb = JunctionBox::new(1, Position::new(0, 0, 0));
        let jb2 = JunctionBox::new(2, Position::new(4, 1, 3));
        let jb3 = JunctionBox::new(3, Position::new(2, 1, 3));
        let jbs = vec![jb, jb2, jb3];

        let distance_stack = Grid::get_distance_stack(&jbs);

        assert_eq!(
            distance_stack,
            vec![
                (Connection::new(&1, &2), 26),
                (Connection::new(&1, &3), 14),
                (Connection::new(&2, &3), 4),
            ]
        );
    }

    #[test]
    fn test_grid_new() {
        let positions = vec![
            Position::new(0, 0, 0),
            Position::new(2, 1, 3),
            Position::new(4, 1, 3),
        ];

        let actual = Grid::new(positions);

        assert_eq!(actual.circuits, vec![]);
        assert_eq!(
            actual.distance_stack,
            vec![
                (Connection::new(&0, &2), 26),
                (Connection::new(&0, &1), 14),
                (Connection::new(&1, &2), 4),
            ]
        );
        assert_eq!(
            actual.junction_boxes,
            vec![
                JunctionBox::new(0, Position::new(0, 0, 0)),
                JunctionBox::new(1, Position::new(2, 1, 3)),
                JunctionBox::new(2, Position::new(4, 1, 3)),
            ]
        );
    }

    #[test]
    fn test_grid_evolve() {
        let positions = vec![
            Position::read("162,817,812").unwrap(),
            Position::read("57,618,57").unwrap(),
            Position::read("906,360,560").unwrap(),
            Position::read("592,479,940").unwrap(),
            Position::read("352,342,300").unwrap(),
            Position::read("466,668,158").unwrap(),
            Position::read("542,29,236").unwrap(),
            Position::read("431,825,988").unwrap(),
            Position::read("739,650,466").unwrap(),
            Position::read("52,470,668").unwrap(),
            Position::read("216,146,977").unwrap(),
            Position::read("819,987,18").unwrap(),
            Position::read("117,168,530").unwrap(),
            Position::read("805,96,715").unwrap(),
            Position::read("346,949,466").unwrap(),
            Position::read("970,615,88").unwrap(),
            Position::read("941,993,340").unwrap(),
            Position::read("862,61,35").unwrap(),
            Position::read("984,92,344").unwrap(),
            Position::read("425,690,689").unwrap(),
        ];

        let mut g = Grid::new(positions);
        println!("{:?}", g.circuits);

        for _ in 0..10 {
            g.evolve();
        }
        assert_eq!(g.circuits.len(), 4);
    }

    #[test]
    fn test_grid_score() {
        let positions = vec![
            Position::read("162,817,812").unwrap(),
            Position::read("57,618,57").unwrap(),
            Position::read("906,360,560").unwrap(),
            Position::read("592,479,940").unwrap(),
            Position::read("352,342,300").unwrap(),
            Position::read("466,668,158").unwrap(),
            Position::read("542,29,236").unwrap(),
            Position::read("431,825,988").unwrap(),
            Position::read("739,650,466").unwrap(),
            Position::read("52,470,668").unwrap(),
            Position::read("216,146,977").unwrap(),
            Position::read("819,987,18").unwrap(),
            Position::read("117,168,530").unwrap(),
            Position::read("805,96,715").unwrap(),
            Position::read("346,949,466").unwrap(),
            Position::read("970,615,88").unwrap(),
            Position::read("941,993,340").unwrap(),
            Position::read("862,61,35").unwrap(),
            Position::read("984,92,344").unwrap(),
            Position::read("425,690,689").unwrap(),
        ];

        let mut g = Grid::new(positions);

        let score = g.score(10);

        assert_eq!(score, 40);
    }
    #[test]
    fn test_grid_score_2() {
        let positions = vec![
            Position::read("162,817,812").unwrap(),
            Position::read("57,618,57").unwrap(),
            Position::read("906,360,560").unwrap(),
            Position::read("592,479,940").unwrap(),
            Position::read("352,342,300").unwrap(),
            Position::read("466,668,158").unwrap(),
            Position::read("542,29,236").unwrap(),
            Position::read("431,825,988").unwrap(),
            Position::read("739,650,466").unwrap(),
            Position::read("52,470,668").unwrap(),
            Position::read("216,146,977").unwrap(),
            Position::read("819,987,18").unwrap(),
            Position::read("117,168,530").unwrap(),
            Position::read("805,96,715").unwrap(),
            Position::read("346,949,466").unwrap(),
            Position::read("970,615,88").unwrap(),
            Position::read("941,993,340").unwrap(),
            Position::read("862,61,35").unwrap(),
            Position::read("984,92,344").unwrap(),
            Position::read("425,690,689").unwrap(),
        ];

        let mut g = Grid::new(positions);

        let score = g.score_2(20);

        assert_eq!(score, 25272);
    }

    #[test]
    fn test_merge_circuits() {
        let jb = JunctionBox::new(1, Position::new(0, 0, 0));
        let jb2 = JunctionBox::new(2, Position::new(2, 1, 3));
        let c1 = Circuit::new(jb.clone(), jb2.clone()).unwrap();

        let jb3 = JunctionBox::new(3, Position::new(4, 1, 3));
        let jb4 = JunctionBox::new(4, Position::new(6, 1, 3));
        let c2 = Circuit::new(jb3.clone(), jb4.clone()).unwrap();

        let actual = Grid::merge_circuits(&c1, &c2, &Connection::new(&2, &3), 4);
        let expected_boxes = vec![jb, jb2, jb3, jb4];
        let mut expected_connections = HashMap::new();
        expected_connections.insert(Connection::new(&1, &2), 14);
        expected_connections.insert(Connection::new(&2, &3), 4);
        expected_connections.insert(Connection::new(&3, &4), 4);
        let mut expected_box_set = HashSet::new();
        expected_box_set.insert(1);
        expected_box_set.insert(2);
        expected_box_set.insert(3);
        expected_box_set.insert(4);

        assert_eq!(actual.boxes, expected_boxes);
        assert_eq!(actual.connections, expected_connections);
        assert_eq!(actual.box_set, expected_box_set);
    }
}
