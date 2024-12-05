use crate::utils::{Task, TaskError};

pub struct Day3;
impl Task for Day3 {
    #[allow(unused_variables)]
    fn task_part_one(&self, file_content: &str) -> Result<String, TaskError> {
        Ok(multiply_instructions(file_content).to_string())
    }
    #[allow(unused_variables)]
    fn task_part_two(&self, file_content: &str) -> Result<String, TaskError> {
        let enabled_input = remove_disabled_parts(file_content);
        Ok(multiply_instructions(&enabled_input).to_string())
    }
    fn get_day(&self) -> u32 {
        3u32
    }
}
fn remove_disabled_parts(input: &str) -> String {
    let mut input = input.to_owned();
    while let Some(new_input) = remove_one_disabled(&input) {
        input = new_input;
    }
    input
}

fn remove_one_disabled(input: &str) -> Option<String> {
    if let Some(pos) = input.find("don't()") {
        if let Some(r_pos) = input[pos + 6..].find("do()") {
            let output = input[0..pos].to_string() + &input[pos + 6 + r_pos + 3..];
            return Some(output);
        }
        return Some(input[0..pos].to_string());
    }
    None
}
fn multiply_instructions(input: &str) -> usize {
    let splits: Vec<&str> = input.split("mul").collect();

    let mut total = 0;
    for s in splits {
        if s[0..1] != *"(" {
            continue;
        }

        if let Some(c) = s.find(",") {
            let left = &s[1..c];
            if !left.chars().all(|c| c.is_numeric()) {
                continue;
            }
            if c == s.len() - 1 {
                continue;
            }

            if let Some(right) = get_right(&s[c..]) {
                let b: usize = right.parse().unwrap();
                let a: usize = left.parse().unwrap();
                total += a * b;
            }
        }
    }
    total
}

fn get_right(input: &str) -> Option<&str> {
    if let Some(par) = input.find(")") {
        let right = &input[1..par];
        if !right.chars().all(|c| c.is_numeric()) {
            return None;
        }
        return Some(right);
    }
    None
}
