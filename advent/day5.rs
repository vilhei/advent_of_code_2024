use std::{cmp::Ordering, collections::HashMap};

use crate::utils::{Task, TaskError};
pub struct Day5;
impl Task for Day5 {
    #[allow(unused_variables)]
    fn task_part_one(&self, file_content: &str) -> Result<String, TaskError> {
        let mut lines = file_content.lines();

        // If any values are in same update rule as the key, the values must be BEFORE the key
        let page_rules = construct_page_ordering_rules(&mut lines);

        // Rest of the input should be the updates which we need to check

        let sum = lines
            .map(|line| {
                line.split(',')
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<_>>()
            })
            .filter(|e| {
                e.is_sorted_by(|a, b| {
                    if let Some(r) = page_rules.get(a) {
                        if r.contains(b) {
                            return false;
                        }
                    }
                    true
                })
            })
            .fold(0, |a, e| a + e[e.len() / 2]);

        // for line in lines {
        //     let update: Vec<usize> = line.split(',').map(|s| s.parse().unwrap()).collect();

        //     if update.is_sorted_by(|a, b| {
        //         if let Some(r) = page_rules.get(a) {
        //             if r.contains(b) {
        //                 return false;
        //             }
        //         }
        //         true
        //     }) {
        //         sum += update[update.len() / 2];
        //     }
        // }

        Ok(sum.to_string())
    }

    #[allow(unused_variables)]
    fn task_part_two(&self, file_content: &str) -> Result<String, TaskError> {
        let mut lines = file_content.lines();

        // If any values are in same update rule as the key, the values must be BEFORE the key
        let page_rules = construct_page_ordering_rules(&mut lines);

        // Rest of the input should be the updates which we need to check
        let mut sum = 0;
        'outer: for line in lines {
            let mut update: Vec<usize> = line.split(',').map(|s| s.parse().unwrap()).collect();

            for (idx, num) in update.iter().enumerate() {
                if let Some(rules) = page_rules.get(num) {
                    for n in &update[idx + 1..] {
                        if rules.contains(n) {
                            sum += correct_update(&mut update, &page_rules);
                            continue 'outer;
                        }
                    }
                }
            }
        }

        Ok(sum.to_string())
    }
    fn get_day(&self) -> u32 {
        5u32
    }
}

fn construct_page_ordering_rules(lines: &mut std::str::Lines<'_>) -> HashMap<usize, Vec<usize>> {
    let mut page_rules: HashMap<usize, Vec<usize>> = HashMap::new();

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let (left, right) = line.split_once('|').unwrap();
        let (left, right) = (left.parse().unwrap(), right.parse().unwrap());

        page_rules
            .entry(right)
            .and_modify(|e| e.push(left))
            .or_insert(vec![left]);
    }
    page_rules
}

fn correct_update(update: &mut [usize], rules: &HashMap<usize, Vec<usize>>) -> usize {
    update.sort_by(|a, b| {
        if let Some(r) = rules.get(b) {
            if r.contains(a) {
                return Ordering::Less;
            }
        }
        Ordering::Equal
    });

    update[update.len() / 2]
}
