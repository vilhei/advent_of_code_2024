use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use crate::utils::{Coords, Matrix, Task, TaskError};
pub struct Day8;
impl Task for Day8 {
    #[allow(unused_variables)]
    fn task_part_one(&self, file_content: &str) -> Result<String, TaskError> {
        let matrix = Matrix::from(file_content);

        let mut map: HashMap<char, Vec<Coords>> = HashMap::new();

        for (x, row) in matrix.data().iter().enumerate() {
            for (y, element) in row.iter().enumerate() {
                if *element != '.' {
                    // dbg!(&element);
                    map.entry(*element)
                        .and_modify(|v| v.push(Coords::new(x, y)))
                        .or_insert(vec![Coords::new(x, y)]);
                }
            }
        }
        // dbg!(&map);
        let mut antinodes = HashSet::new();
        for (key, coords) in map {
            let coord_combinations: Vec<(&Coords, &Coords)> =
                coords.iter().tuple_combinations().collect_vec();
            coord_combinations.iter().for_each(|(&ref a, &ref b)| {
                let diff = a - b;
                let antinode1 = a + &diff;
                if antinode1.inbounds(&matrix) {
                    antinodes.insert(antinode1);
                }
                let antinode2 = b - &diff;
                if antinode2.inbounds(&matrix) {
                    antinodes.insert(antinode2);
                }
            });
        }

        Ok(antinodes.len().to_string())
    }
    #[allow(unused_variables)]
    fn task_part_two(&self, file_content: &str) -> Result<String, TaskError> {
        let mut matrix = Matrix::from(file_content);

        let mut map: HashMap<char, Vec<Coords>> = HashMap::new();
        let mut nodes = 0;
        for (x, row) in matrix.data().iter().enumerate() {
            for (y, element) in row.iter().enumerate() {
                if *element != '.' {
                    // dbg!(&element);
                    map.entry(*element)
                        .and_modify(|v| v.push(Coords::new(x, y)))
                        .or_insert(vec![Coords::new(x, y)]);
                    nodes += 1;
                }
            }
        }
        // dbg!(&map);
        let mut antinodes = HashSet::new();
        for (key, coords) in map {
            coords.iter().for_each(|c| {
                antinodes.insert(c.clone());
            });
            let coord_combinations: Vec<(&Coords, &Coords)> =
                coords.iter().tuple_combinations().collect_vec();
            coord_combinations.iter().for_each(|(&ref a, &ref b)| {
                let diff = a - b;
                let mut i = 1;
                loop {
                    let antinode1 = a + &diff * i;

                    if antinode1.inbounds(&matrix) {
                        antinodes.insert(antinode1);
                    } else {
                        break;
                    }
                    i += 1
                }
                i = 1;
                loop {
                    let antinode2 = b - &diff * i;

                    if antinode2.inbounds(&matrix) {
                        antinodes.insert(antinode2);
                    } else {
                        break;
                    }
                    i += 1
                }
            });
        }
        for node in &antinodes {
            let element = &mut matrix[node];
            *element = '#';
        }
        matrix.write_to_file("./test_output.txt");
        // dbg!(&antinodes);
        Ok((antinodes.len()).to_string())
    }
    fn get_day(&self) -> u32 {
        8u32
    }
}
