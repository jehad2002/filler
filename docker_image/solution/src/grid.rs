use std::io::{self, BufRead};
use thiserror::Error;

#[derive(Debug)]
pub struct Grid {
    pub cells: Vec<Vec<char>>,
}

#[derive(Debug, Error)]
pub enum GridError {
    #[error("Failed to read input")]
    ReadError,
    #[error("Failed to parse grid details")]
    ParseError,
}

impl Grid {
    pub fn is_inside(&self, x: usize, y: usize) -> bool {
        self.cells.get(y).and_then(|row| row.get(x)).is_some()
    }
}

pub fn read_grid(
    input: &mut String,
    pchars: &[char],
    echars: &[char],
) -> Result<(Grid, Vec<(usize, usize)>, Vec<(usize, usize)>), GridError> {
    let mut pcoords = Vec::new();
    let mut ecoords = Vec::new();
    let stdin = io::stdin();

    input.clear();
    stdin.lock().read_line(input).map_err(|_| GridError::ReadError)?;
    let grid_details = input.split_whitespace().collect::<Vec<&str>>()[2];
    let lines = &grid_details[..grid_details.len() - 1];
    let grid_lines = lines.parse::<usize>().map_err(|_| GridError::ParseError)?;

    let mut grid = Vec::new();

    for i in 0..=grid_lines {
        input.clear();
        stdin.lock().read_line(input).map_err(|_| GridError::ReadError)?;
        if i < 1 {
            continue;
        } else {
            let row: Vec<char> = input[4..input.len() - 1].chars().collect();
            for (j, &ch) in row.iter().enumerate() {
                if pchars.contains(&ch) {
                    pcoords.push((j, i));
                }
                if echars.contains(&ch) {
                    ecoords.push((j, i));
                }
            }
            grid.push(row);
        }
    }

    Ok((Grid { cells: grid }, pcoords, ecoords))
}
