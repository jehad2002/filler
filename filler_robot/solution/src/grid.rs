use std::io::{self, BufRead};
use thiserror::Error;

#[derive(Debug)]
pub struct Grid {
    pub cells: Vec<Vec<char>>,
    pub pcoords: Vec<(usize, usize)>,
    pub ecoords: Vec<(usize, usize)>,
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
        if let Some(row) = self.cells.get(y) {
            if let Some(_) = row.get(x) {
                return true;
            }
        }
        false
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

    for i in 0..grid_lines + 1 {
        input.clear();
        stdin.lock().read_line(input).map_err(|_| GridError::ReadError)?;
        if i < 1 {
            continue;
        } else {
            let row: Vec<char> = input[4..input.len() - 1].chars().collect();
            for j in 0..row.len() {
                if pchars.contains(&row[j]) {
                    pcoords.push((j, i));
                }
                if echars.contains(&row[j]) {
                    ecoords.push((j, i));
                }
            }
            grid.push(row);
        }
    }

    Ok((Grid { cells: grid, pcoords: pcoords.clone(), ecoords: ecoords.clone() }, pcoords, ecoords))
}
