use crate::utils::{Matrix, Task, TaskError};
pub struct Day4;
impl Task for Day4 {
    #[allow(unused_variables)]
    fn task_part_one(&self, file_content: &str) -> Result<String, TaskError> {
        let m = Matrix::from(file_content);
        let mut sum = 0;
        for i in 0..m.rows {
            for j in 0..m.columns {
                if m[i][j] == 'X' {
                    sum += check_for_xmas(&m, i, j);
                }
            }
        }
        Ok(sum.to_string())
    }
    #[allow(unused_variables)]
    fn task_part_two(&self, file_content: &str) -> Result<String, TaskError> {
        let m = Matrix::from(file_content);
        let mut sum = 0;
        for i in 0..m.rows {
            for j in 0..m.columns {
                if m[i][j] == 'A' {
                    sum += check_for_mas(&m, i, j);
                }
            }
        }
        Ok(sum.to_string())
    }
    fn get_day(&self) -> u32 {
        4u32
    }
}

fn check_for_mas(m: &Matrix, i: usize, j: usize) -> usize {
    if i < 1 || i + 1 >= m.rows || j < 1 || j + 1 >= m.columns {
        return 0;
    }

    let ul = m[i - 1][j - 1];
    let ur = m[i - 1][j + 1];
    let bl = m[i + 1][j - 1];
    let br = m[i + 1][j + 1];

    if ul == 'S' && br == 'M' && ((ur == 'S' && bl == 'M') || (ur == 'M' && bl == 'S')) {
        return 1;
    }

    if ul == 'M' && br == 'S' && ((ur == 'S' && bl == 'M') || (ur == 'M' && bl == 'S')) {
        return 1;
    }

    0
}

fn check_for_xmas(m: &Matrix, i: usize, j: usize) -> usize {
    let mut total = 0;
    if check_forward(m, i, j) {
        total += 1;
    }
    if check_backward(m, i, j) {
        total += 1;
    }
    if check_down(m, i, j) {
        total += 1;
    }
    if check_up(m, i, j) {
        total += 1;
    }
    if check_north_east_diag(m, i, j) {
        total += 1;
    }
    if check_south_east_diag(m, i, j) {
        total += 1;
    }
    if check_south_west_diag(m, i, j) {
        total += 1;
    }
    if check_north_west_diag(m, i, j) {
        total += 1;
    }

    total
}

fn check_forward(m: &Matrix, i: usize, j: usize) -> bool {
    if m.columns < j + 4 {
        return false;
    }

    if m[i][j + 1] != 'M' || m[i][j + 2] != 'A' || m[i][j + 3] != 'S' {
        return false;
    }
    true
}

fn check_backward(m: &Matrix, i: usize, j: usize) -> bool {
    if j as i64 - 3 < 0 {
        return false;
    }

    if m[i][j - 1] != 'M' || m[i][j - 2] != 'A' || m[i][j - 3] != 'S' {
        return false;
    }
    true
}

fn check_down(m: &Matrix, i: usize, j: usize) -> bool {
    if m.rows < i + 4 {
        return false;
    }

    if m[i + 1][j] != 'M' || m[i + 2][j] != 'A' || m[i + 3][j] != 'S' {
        return false;
    }
    true
}

fn check_up(m: &Matrix, i: usize, j: usize) -> bool {
    if i as i64 - 3 < 0 {
        return false;
    }

    if m[i - 1][j] != 'M' || m[i - 2][j] != 'A' || m[i - 3][j] != 'S' {
        return false;
    }
    true
}

fn check_north_east_diag(m: &Matrix, i: usize, j: usize) -> bool {
    if i as i64 - 3 < 0 || m.columns < j + 4 {
        return false;
    }

    if m[i - 1][j + 1] != 'M' || m[i - 2][j + 2] != 'A' || m[i - 3][j + 3] != 'S' {
        return false;
    }
    true
}

fn check_south_east_diag(m: &Matrix, i: usize, j: usize) -> bool {
    if m.rows < i + 4 || m.columns < j + 4 {
        return false;
    }

    if m[i + 1][j + 1] != 'M' || m[i + 2][j + 2] != 'A' || m[i + 3][j + 3] != 'S' {
        return false;
    }
    true
}

fn check_south_west_diag(m: &Matrix, i: usize, j: usize) -> bool {
    if m.rows < i + 4 || j as i64 - 3 < 0 {
        return false;
    }

    if m[i + 1][j - 1] != 'M' || m[i + 2][j - 2] != 'A' || m[i + 3][j - 3] != 'S' {
        return false;
    }
    true
}

fn check_north_west_diag(m: &Matrix, i: usize, j: usize) -> bool {
    if i as i64 - 3 < 0 || j as i64 - 3 < 0 {
        return false;
    }

    if m[i - 1][j - 1] != 'M' || m[i - 2][j - 2] != 'A' || m[i - 3][j - 3] != 'S' {
        return false;
    }
    true
}
