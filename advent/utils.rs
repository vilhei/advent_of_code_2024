use std::cmp::PartialEq;
use std::fs;
use std::ops::Index;
use std::ops::IndexMut;

#[derive(Debug)]
pub enum TaskError {
    InvalidFilePath(String),
    NotImplemented(usize),
}

pub fn read_task_input_file(path: &str) -> Result<String, TaskError> {
    let Ok(file_contents) = fs::read_to_string(path) else {
        return Err(TaskError::InvalidFilePath(path.to_string()));
    };
    Ok(file_contents)
}

pub trait Task {
    #[allow(unused_variables)]
    fn task_part_one(&self, file_content: &str) -> Result<String, TaskError> {
        Err(TaskError::NotImplemented(1))
    }
    #[allow(unused_variables)]
    fn task_part_two(&self, file_content: &str) -> Result<String, TaskError> {
        Err(TaskError::NotImplemented(2))
    }

    fn get_day(&self) -> u32;
}
#[derive(Debug, Clone)]
pub struct Matrix<T = char> {
    pub rows: usize,
    pub columns: usize,
    data: Vec<Vec<T>>,
}

impl From<&str> for Matrix<char> {
    fn from(input: &str) -> Self {
        let char_vecs: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();

        Matrix {
            rows: char_vecs.len(),
            columns: char_vecs[0].len(),
            data: char_vecs,
        }
    }
}

impl<T: PartialEq> Matrix<T> {
    pub fn data_mut(&mut self) -> &mut Vec<Vec<T>> {
        &mut self.data
    }
    pub fn data(&self) -> &Vec<Vec<T>> {
        &self.data
    }

    pub fn transform_type<U, F>(self, mut f: F) -> Matrix<U>
    where
        F: FnMut(T) -> U,
    {
        Matrix {
            columns: self.columns,
            rows: self.rows,
            data: self
                .data
                .into_iter()
                .map(move |row| {
                    let f1 = &mut f;
                    row.into_iter().map(f1).collect()
                })
                .collect(),
        }
    }

    pub fn parse<F>(input: &str, f: F) -> Self
    where
        F: Fn(&str) -> Vec<Vec<T>>,
    {
        let vecs: Vec<Vec<T>> = f(input);

        Matrix {
            rows: vecs[0].len(),
            columns: vecs.len(),
            data: vecs,
        }
    }

    pub fn column_len(&self) -> usize {
        self.columns
    }
    pub fn row_len(&self) -> usize {
        self.rows
    }

    pub fn find(&self, target: &T) -> Option<(usize, usize)> {
        for (i, row) in self.data.iter().enumerate() {
            if let Some(j) = row.iter().position(|e| e == target) {
                return Some((i, j));
            }
        }
        None
    }
    pub fn find_any(&self, target: &[T]) -> Option<(usize, usize)> {
        for (i, row) in self.data.iter().enumerate() {
            if let Some(j) = row.iter().position(|e| target.contains(e)) {
                return Some((i, j));
            }
        }
        None
    }
}

impl<T> Index<usize> for Matrix<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

pub fn parse_2d_char_array(input: &str) -> Matrix<char> {
    Matrix::from(input)
}
