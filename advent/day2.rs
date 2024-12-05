use crate::utils::{Task, TaskError};
pub struct Day2;
impl Task for Day2 {
    #[allow(unused_variables)]
    fn task_part_one(&self, file_content: &str) -> Result<String, TaskError> {
        Err(TaskError::NotImplemented(1))
    }
    #[allow(unused_variables)]
    fn task_part_two(&self, file_content: &str) -> Result<String, TaskError> {
        Err(TaskError::NotImplemented(2))
    }
    fn get_day(&self) -> u32 {
        2u32
    }
}
