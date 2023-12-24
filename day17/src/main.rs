use pathfinding::prelude::astar;
use std::fs::read_to_string;

fn main() {
    assert_eq!(part1(&"test.txt"), 102);
    println!("{}", part1(&"input.txt"));
    assert_eq!(part2(&"test.txt"), 94);
    assert_eq!(part2(&"test2.txt"), 71);
    println!("{}", part2(&"input.txt"));
}

fn part1(path: &str) -> u32 {
    // heat loss is listed at each block
    let grid: Vec<Vec<u32>> = parse_grid(&path);

    let grid_max: (usize, usize) = (grid.len() - 1, grid[0].len() - 1);
    let start_position: Position = Position {
        row: 0,
        col: 0,
        streak: 0,
        direction: Dir::E,
    };

    let goal: Position = Position {
        row: grid_max.0,
        col: grid_max.1,
        streak: 0,
        direction: Dir::N,
    };

    let result: (Vec<Position>, u32) = astar(
        &start_position,
        |p: &Position| p.successors_p1(&grid_max.0, &grid_max.1, &grid),
        |p: &Position| p.distance(&goal),
        |p: &Position| p.distance(&goal).eq(&0),
    )
    .unwrap();
    //println!("{:?}", result.0);
    result.1
}

fn part2(path: &str) -> u32 {
    let grid: Vec<Vec<u32>> = parse_grid(&path);

    let grid_max: (usize, usize) = (grid.len() - 1, grid[0].len() - 1);
    let start_position: Position = Position {
        row: 0,
        col: 0,
        streak: 0,
        direction: Dir::E,
    };
    let start_position_2: Position = Position {
        row: 0,
        col: 0,
        streak: 0,
        direction: Dir::S,
    };

    let goal: Position = Position {
        row: grid_max.0,
        col: grid_max.1,
        streak: 0,
        direction: Dir::N,
    };
    // need to check starting east and starting south separately
    let result: (Vec<Position>, u32) = astar(
        &start_position,
        |p: &Position| p.successors_p2(&grid_max.0, &grid_max.1, &grid),
        |p: &Position| p.distance(&goal),
        |p: &Position| p.distance(&goal).eq(&0) && p.streak > 3,
    )
    .unwrap();
    let result_2: (Vec<Position>, u32) = astar(
        &start_position_2,
        |p: &Position| p.successors_p2(&grid_max.0, &grid_max.1, &grid),
        |p: &Position| p.distance(&goal),
        |p: &Position| p.distance(&goal).eq(&0) && p.streak > 3,
    )
    .unwrap();

    if result_2.1 < result.1 {
        result_2.1
    } else {
        result.1
    }
}
//794 too high and 792 too high
fn parse_grid(path: &str) -> Vec<Vec<u32>> {
    let file = read_to_string(&path).unwrap();
    file.lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|chr| chr.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Dir {
    N,
    E,
    S,
    W,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Position {
    row: usize,
    col: usize,
    streak: u8,
    direction: Dir,
}

impl Position {
    fn distance(&self, other: &Self) -> u32 {
        (self.row.abs_diff(other.row) + self.col.abs_diff(other.col)) as u32
    }

    fn successors_p1(
        &self,
        max_row: &usize,
        max_col: &usize,
        grid: &Vec<Vec<u32>>,
    ) -> Vec<(Self, u32)> {
        let mut successors = vec![];
        match self.direction {
            Dir::N => {
                if self.row.ne(&0) {
                    if self.streak < 3 {
                        self.push_direction(&mut successors, &grid, None);
                    }
                }
                if self.col.ne(&0) {
                    self.push_direction(&mut successors, &grid, Some(Dir::W));
                }
                if self.col.ne(max_col) {
                    self.push_direction(&mut successors, &grid, Some(Dir::E));
                }
            }
            Dir::S => {
                if self.row.ne(max_row) {
                    if self.streak < 3 {
                        self.push_direction(&mut successors, &grid, None);
                    }
                }
                if self.col.ne(&0) {
                    self.push_direction(&mut successors, &grid, Some(Dir::W));
                }
                if self.col.ne(max_col) {
                    self.push_direction(&mut successors, &grid, Some(Dir::E));
                }
            }
            Dir::W => {
                if self.col.ne(&0) {
                    if self.streak < 3 {
                        self.push_direction(&mut successors, &grid, None);
                    }
                }
                if self.row.ne(&0) {
                    self.push_direction(&mut successors, &grid, Some(Dir::N));
                }
                if self.row.ne(max_row) {
                    self.push_direction(&mut successors, &grid, Some(Dir::S));
                }
            }
            Dir::E => {
                if self.col.ne(max_col) {
                    if self.streak < 3 {
                        self.push_direction(&mut successors, &grid, None);
                    }
                }
                if self.row.ne(&0) {
                    self.push_direction(&mut successors, &grid, Some(Dir::N));
                }
                if self.row.ne(max_row) {
                    self.push_direction(&mut successors, &grid, Some(Dir::S));
                }
            }
        };
        successors
    }

    fn successors_p2(
        &self,
        max_row: &usize,
        max_col: &usize,
        grid: &Vec<Vec<u32>>,
    ) -> Vec<(Self, u32)> {
        let mut successors = vec![];

        match self.direction {
            Dir::N => {
                if self.streak < 4 {
                    if self.row.ne(&0) {
                        self.push_direction(&mut successors, &grid, None);
                    }
                } else if self.streak < 10 {
                    if self.row.ne(&0) {
                        self.push_direction(&mut successors, &grid, None);
                    }
                    if self.col.ne(&0) {
                        self.push_direction(&mut successors, &grid, Some(Dir::W));
                    }
                    if self.col.ne(max_col) {
                        self.push_direction(&mut successors, &grid, Some(Dir::E));
                    }
                } else {
                    if self.col.ne(&0) {
                        self.push_direction(&mut successors, &grid, Some(Dir::W));
                    }
                    if self.col.ne(max_col) {
                        self.push_direction(&mut successors, &grid, Some(Dir::E));
                    }
                }
            }
            Dir::S => {
                if self.streak < 4 {
                    if self.row.ne(max_row) {
                        self.push_direction(&mut successors, &grid, None);
                    }
                } else if self.streak < 10 {
                    if self.row.ne(max_row) {
                        self.push_direction(&mut successors, &grid, None);
                    }
                    if self.col.ne(&0) {
                        self.push_direction(&mut successors, &grid, Some(Dir::W));
                    }
                    if self.col.ne(max_col) {
                        self.push_direction(&mut successors, &grid, Some(Dir::E));
                    }
                } else {
                    if self.col.ne(&0) {
                        self.push_direction(&mut successors, &grid, Some(Dir::W));
                    }
                    if self.col.ne(max_col) {
                        self.push_direction(&mut successors, &grid, Some(Dir::E));
                    }
                }
            }
            Dir::W => {
                if self.streak < 4 {
                    if self.col.ne(&0) {
                        self.push_direction(&mut successors, &grid, None);
                    }
                } else if self.streak < 10 {
                    if self.col.ne(&0) {
                        self.push_direction(&mut successors, &grid, None);
                    }
                    if self.row.ne(&0) {
                        self.push_direction(&mut successors, &grid, Some(Dir::N));
                    }

                    if self.row.ne(max_row) {
                        self.push_direction(&mut successors, &grid, Some(Dir::S));
                    }
                } else {
                    if self.row.ne(&0) {
                        self.push_direction(&mut successors, &grid, Some(Dir::N));
                    }

                    if self.row.ne(max_row) {
                        self.push_direction(&mut successors, &grid, Some(Dir::S));
                    }
                }
            }
            Dir::E => {
                if self.streak < 4 {
                    if self.col.ne(max_col) {
                        self.push_direction(&mut successors, &grid, None);
                    }
                } else if self.streak < 10 {
                    if self.col.ne(max_col) {
                        self.push_direction(&mut successors, &grid, None);
                    }
                    if self.row.ne(&0) {
                        self.push_direction(&mut successors, &grid, Some(Dir::N));
                    }

                    if self.row.ne(max_row) {
                        self.push_direction(&mut successors, &grid, Some(Dir::S));
                    }
                } else {
                    if self.row.ne(&0) {
                        self.push_direction(&mut successors, &grid, Some(Dir::N));
                    }

                    if self.row.ne(max_row) {
                        self.push_direction(&mut successors, &grid, Some(Dir::S));
                    }
                }
            }
        };

        successors
    }
    fn push_direction(
        &self,
        successors: &mut Vec<(Self, u32)>,
        grid: &Vec<Vec<u32>>,
        turn_option: Option<Dir>,
    ) {
        if let Some(new_direction) = turn_option {
            // move in the new direction and reset the streak
            match new_direction {
                Dir::N => successors.push((
                    Position {
                        row: self.row - 1,
                        col: self.col,
                        streak: 1,
                        direction: Dir::N,
                    },
                    grid[self.row - 1][self.col],
                )),
                Dir::S => successors.push((
                    Position {
                        row: self.row + 1,
                        col: self.col,
                        streak: 1,
                        direction: Dir::S,
                    },
                    grid[self.row + 1][self.col],
                )),
                Dir::E => successors.push((
                    Position {
                        row: self.row,
                        col: self.col + 1,
                        streak: 1,
                        direction: Dir::E,
                    },
                    grid[self.row][self.col + 1],
                )),
                Dir::W => successors.push((
                    Position {
                        row: self.row,
                        col: self.col - 1,
                        streak: 1,
                        direction: Dir::W,
                    },
                    grid[self.row][self.col - 1],
                )),
            }
        } else {
            //streak continues same direction
            match self.direction {
                Dir::N => successors.push((
                    Position {
                        row: self.row - 1,
                        col: self.col,
                        streak: self.streak + 1,
                        direction: self.direction,
                    },
                    grid[self.row - 1][self.col],
                )),
                Dir::S => successors.push((
                    Position {
                        row: self.row + 1,
                        col: self.col,
                        streak: self.streak + 1,
                        direction: self.direction,
                    },
                    grid[self.row + 1][self.col],
                )),
                Dir::E => successors.push((
                    Position {
                        row: self.row,
                        col: self.col + 1,
                        streak: self.streak + 1,
                        direction: self.direction,
                    },
                    grid[self.row][self.col + 1],
                )),
                Dir::W => successors.push((
                    Position {
                        row: self.row,
                        col: self.col - 1,
                        streak: self.streak + 1,
                        direction: self.direction,
                    },
                    grid[self.row][self.col - 1],
                )),
            }
        }
    }
}
