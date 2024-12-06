use crate::utils::{Matrix, Task, TaskError};
use rayon::prelude::*;
use std::collections::HashSet;
pub struct Day6;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn switch_dir(&self) -> Self {
        match self {
            Direction::Left => Self::Up,
            Direction::Right => Self::Down,
            Direction::Up => Self::Right,
            Direction::Down => Self::Left,
        }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Guard {
    x: isize,
    y: isize,
    direction: Direction,
}

impl Guard {
    pub fn update_dir(&mut self) {
        self.direction = self.direction.switch_dir();
    }
}

impl Task for Day6 {
    #[allow(unused_variables)]
    fn task_part_one(&self, file_content: &str) -> Result<String, TaskError> {
        let mut matrix = Matrix::from(file_content);
        let guard_location = matrix.find_any(&['^', '>', '<', 'v']).unwrap();

        let guard_dir = matrix[guard_location.0][guard_location.1];
        let guard_location = (guard_location.0 as isize, guard_location.1 as isize);
        let mut guard = match guard_dir {
            '^' => Guard {
                direction: Direction::Up,
                x: guard_location.0,
                y: guard_location.1,
            },
            '>' => Guard {
                x: guard_location.0,
                y: guard_location.1,
                direction: Direction::Right,
            },
            '<' => Guard {
                x: guard_location.0,
                y: guard_location.1,
                direction: Direction::Left,
            },
            'v' => Guard {
                x: guard_location.0,
                y: guard_location.1,
                direction: Direction::Down,
            },
            _ => panic!("Illegal guard symbol"),
        };
        let mut visited: HashSet<(isize, isize)> = HashSet::new();
        visited.insert(guard_location);

        matrix[guard.x as usize][guard.y as usize] = '.';

        loop {
            let (new_x, new_y) = match guard.direction {
                Direction::Left => (guard.x, guard.y - 1),
                Direction::Right => (guard.x, guard.y + 1),
                Direction::Up => (guard.x - 1, guard.y),
                Direction::Down => (guard.x + 1, guard.y),
            };
            if new_x >= matrix.rows as isize
                || new_y >= matrix.columns as isize
                || new_x < 0
                || new_y < 0
            {
                break;
            }

            match matrix[new_x as usize][new_y as usize] {
                '#' => {
                    guard.update_dir();
                }
                '.' => {
                    visited.insert((new_x, new_y));
                    guard.x = new_x;
                    guard.y = new_y;
                }
                _ => panic!("invalid char"),
            }
        }

        Ok(visited.len().to_string())
    }
    #[allow(unused_variables)]
    fn task_part_two(&self, file_content: &str) -> Result<String, TaskError> {
        let mut matrix = Matrix::from(file_content);
        let guard_location = matrix.find_any(&['^', '>', '<', 'v']).unwrap();

        let guard_dir = matrix[guard_location.0][guard_location.1];
        let guard_initial_location = (guard_location.0 as isize, guard_location.1 as isize);
        let guard = match guard_dir {
            '^' => Guard {
                direction: Direction::Up,
                x: guard_initial_location.0,
                y: guard_initial_location.1,
            },
            '>' => Guard {
                x: guard_initial_location.0,
                y: guard_initial_location.1,
                direction: Direction::Right,
            },
            '<' => Guard {
                x: guard_initial_location.0,
                y: guard_initial_location.1,
                direction: Direction::Left,
            },
            'v' => Guard {
                x: guard_initial_location.0,
                y: guard_initial_location.1,
                direction: Direction::Down,
            },
            _ => panic!("Illegal guard symbol"),
        };
        // let mut handles = Vec::new();
        matrix[guard.x as usize][guard.y as usize] = '.';
        let res: usize = matrix
            .data()
            .par_iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .map(|(j, element)| {
                        if *element == '.'
                            && !(i == guard_initial_location.0 as usize
                                && j == guard_initial_location.1 as usize)
                        {
                            let mut matrix2 = matrix.clone();
                            matrix2[i][j] = '#';
                            check_for_loop(guard.clone(), &matrix2, guard_initial_location)
                        } else {
                            0
                        }
                    })
                    .sum::<usize>()
            })
            .sum();
        Ok(res.to_string())
    }
    fn get_day(&self) -> u32 {
        6u32
    }
}

fn check_for_loop(
    mut guard: Guard,
    matrix: &Matrix,
    guard_initial_location: (isize, isize),
) -> usize {
    let mut visited_states: HashSet<Guard> = HashSet::new();
    visited_states.insert(guard.clone());

    loop {
        let (new_x, new_y) = match guard.direction {
            Direction::Left => (guard.x, guard.y - 1),
            Direction::Right => (guard.x, guard.y + 1),
            Direction::Up => (guard.x - 1, guard.y),
            Direction::Down => (guard.x + 1, guard.y),
        };
        if new_x >= matrix.rows as isize
            || new_y >= matrix.columns as isize
            || new_x < 0
            || new_y < 0
        {
            return 0;
        }

        match matrix[new_x as usize][new_y as usize] {
            '#' => {
                guard.update_dir();
            }
            '.' => {
                guard.x = new_x;
                guard.y = new_y;
            }
            _ => panic!("invalid char"),
        }
        let inserted = visited_states.insert(guard.clone());
        if !inserted && guard_initial_location != (new_x, new_y) {
            return 1;
        }
    }
}
