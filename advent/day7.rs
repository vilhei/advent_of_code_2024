use core::panic;
use std::iter::repeat_n;

use crate::utils::{Task, TaskError};
use itertools::Itertools;
use rayon::prelude::*;
use rayon::str::ParallelString;

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
        let res = process_input(file_content, false);
        Ok(res.to_string())
    }

    fn task_part_two(&self, file_content: &str) -> Result<String, TaskError> {
        let res = process_input(file_content, true);
        Ok(res.to_string())
    }

    fn get_day(&self) -> u32 {
        7u32
    }
}

fn process_input(file_content: &str, is_part_2: bool) -> usize {
    let res = file_content
        .par_lines()
        .map(|line| {
            let (answer, inputs) = line.split_once(':').unwrap();

            let answer: usize = answer.parse().unwrap();
            let nums: Vec<usize> = inputs
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            let operations = nums.len() - 1;
            let perms: Vec<_> = if is_part_2 {
                repeat_n(
                    [Operation::Add, Operation::Multiply, Operation::Concetenate],
                    operations,
                )
                .multi_cartesian_product()
                .collect()
            } else {
                repeat_n([Operation::Add, Operation::Multiply], operations)
                    .multi_cartesian_product()
                    .collect()
            };

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
                    return answer;
                }
            }
            0
        })
        .sum::<usize>();
    res
}
