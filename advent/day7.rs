use crate::utils::{Task, TaskError};
pub struct Day7;
impl Task for Day7 {
    #[allow(unused_variables)]
    fn task_part_one(&self, file_content: &str) -> Result<String, TaskError> {
        Err(TaskError::NotImplemented(1))
    }
    #[allow(unused_variables)]
    fn task_part_two(&self, file_content: &str) -> Result<String, TaskError> {
        Err(TaskError::NotImplemented(2))
    }
    fn get_day(&self) -> u32 {
        7u32
    }
}
