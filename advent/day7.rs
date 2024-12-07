use core::panic;
use std::iter::repeat_n;

use crate::utils::{Task, TaskError};
use itertools::Itertools;
pub struct Day7;

#[derive(Debug, Clone)]
enum Operation {
    Add,
    Multiply,
    Concetenate,
}

impl Task for Day7 {
    #[allow(unused_variables)]
    fn task_part_one(&self, file_content: &str) -> Result<String, TaskError> {
        let mut res = 0;
        'outer: for line in file_content.lines() {
            let (answer, inputs) = line.split_once(':').unwrap();

            let answer: usize = answer.parse().unwrap();
            let nums: Vec<usize> = inputs
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            let operations = nums.len() - 1;
            let perms: Vec<_> = repeat_n([Operation::Add, Operation::Multiply], operations)
                .multi_cartesian_product()
                .collect();

            for oper_perm in perms {
                let mut total = nums[0];
                for (oper, num) in oper_perm.iter().zip(&nums[1..]) {
                    match oper {
                        Operation::Add => total += num,
                        Operation::Multiply => total *= num,
                        Operation::Concetenate => panic!("Concetenate should not be in part 1"),
                    }
                }
                if total == answer {
                    res += answer;
                    continue 'outer;
                }
            }
        }

        Ok(res.to_string())
    }
    #[allow(unused_variables)]
    fn task_part_two(&self, file_content: &str) -> Result<String, TaskError> {
        let mut res = 0;
        'outer: for line in file_content.lines() {
            let (answer, inputs) = line.split_once(':').unwrap();

            let answer: usize = answer.parse().unwrap();
            let nums: Vec<usize> = inputs
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            let operations = nums.len() - 1;
            let perms: Vec<_> = repeat_n(
                [Operation::Add, Operation::Multiply, Operation::Concetenate],
                operations,
            )
            .multi_cartesian_product()
            .collect();

            for oper_perm in perms {
                let mut total = nums[0];
                for (oper, num) in oper_perm.iter().zip(&nums[1..]) {
                    match oper {
                        Operation::Add => total += num,
                        Operation::Multiply => total *= num,
                        Operation::Concetenate => total = format!("{total}{num}").parse().unwrap(),
                    }
                }
                if total == answer {
                    res += answer;
                    continue 'outer;
                }
            }
        }

        Ok(res.to_string())
    }
    fn get_day(&self) -> u32 {
        7u32
    }
}
