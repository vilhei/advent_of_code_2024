use std::cmp::PartialEq;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::ops::Add;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Mul;
use std::ops::Sub;
use std::path::Path;

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

impl Matrix<char> {
    pub fn write_to_file<F: AsRef<Path>>(&self, path: F) {
        let file = File::create(path.as_ref()).unwrap();
        let mut writer = BufWriter::new(file);
        for row in &self.data {
            let r: String = row.iter().collect();
            writeln!(&mut writer, "{}", r).unwrap();
        }
        writer.flush().unwrap();
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
impl<T> Index<Coords> for Matrix<T> {
    type Output = T;

    // TODO check that coords are positive
    fn index(&self, index: Coords) -> &Self::Output {
        &self.data[index.x as usize][index.y as usize]
    }
}

impl<T> IndexMut<Coords> for Matrix<T> {
    // TODO check that coords are positive
    fn index_mut(&mut self, index: Coords) -> &mut Self::Output {
        &mut self.data[index.x as usize][index.y as usize]
    }
}
impl<T> Index<&Coords> for Matrix<T> {
    type Output = T;

    // TODO check that coords are positive
    fn index(&self, index: &Coords) -> &Self::Output {
        &self.data[index.x as usize][index.y as usize]
    }
}

impl<T> IndexMut<&Coords> for Matrix<T> {
    // TODO check that coords are positive
    fn index_mut(&mut self, index: &Coords) -> &mut Self::Output {
        &mut self.data[index.x as usize][index.y as usize]
    }
}

pub fn parse_2d_char_array(input: &str) -> Matrix<char> {
    Matrix::from(input)
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Coords {
    pub x: isize,
    pub y: isize,
}
impl Sub for Coords {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub for &Coords {
    type Output = Coords;

    fn sub(self, rhs: Self) -> Self::Output {
        Coords {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<Coords> for &Coords {
    type Output = Coords;

    fn sub(self, rhs: Coords) -> Self::Output {
        Coords {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add for Coords {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<&Coords> for &Coords {
    type Output = Coords;

    fn add(self, rhs: &Coords) -> Self::Output {
        Coords {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<&Coords> for Coords {
    type Output = Coords;

    fn add(self, rhs: &Coords) -> Self::Output {
        Coords {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Add<Coords> for &Coords {
    type Output = Coords;

    fn add(self, rhs: Coords) -> Self::Output {
        Coords {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: num_traits::AsPrimitive<isize>> Mul<T> for Coords {
    type Output = Coords;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs.as_(),
            y: self.y * rhs.as_(),
        }
    }
}
impl<T: num_traits::AsPrimitive<isize>> Mul<T> for &Coords {
    type Output = Coords;

    fn mul(self, rhs: T) -> Coords {
        Coords {
            x: self.x * rhs.as_(),
            y: self.y * rhs.as_(),
        }
    }
}

impl Coords {
    pub fn new<T>(a: T, b: T) -> Self
    where
        T: num_traits::AsPrimitive<isize>,
    {
        Self {
            x: a.as_(),
            y: b.as_(),
        }
    }
    pub fn inbounds(&self, matrix: &Matrix) -> bool {
        if self.x >= 0
            && (self.x as usize) < matrix.rows
            && self.y >= 0
            && (self.y as usize) < matrix.columns
        {
            return true;
        }
        false
    }
}
