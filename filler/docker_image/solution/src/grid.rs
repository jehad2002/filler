use std::io::{self, BufRead};
use thiserror::Error;

#[derive(Debug)]
pub struct Grid {
    pub cells: Vec<Vec<char>>,
}

#[derive(Debug, Error)]
pub enum GridError { // type for errors
    #[error("Failed to read input")]
    ReadError,
    #[error("Failed to parse grid details")]
    ParseError,
}

impl Grid {
    pub fn is_inside(&self, x: usize, y: usize) -> bool { // check if the coordinates(x, y) are inside the grid
        self.cells.get(y).and_then(|row| row.get(x)).is_some() // get the row at index y and then get the cell at index x to check if it exists
    }
}

pub fn read_grid(
    input: &mut String,
    pchars: &[char],
    echars: &[char],
) -> Result<(Grid, Vec<(usize, usize)>, Vec<(usize, usize)>), GridError> {
    let mut pcoords = Vec::new();
    let mut ecoords = Vec::new();
    let stdin = io::stdin(); // Use stdin for reading input

    input.clear(); // Clear the input string 
    stdin.lock().read_line(input).map_err(|_| GridError::ReadError)?; // read a line from stdin and store it in input
    let grid_details = input.split_whitespace().collect::<Vec<&str>>()[2];  // split the input by whitespace and collect the third element which contains grid details
    let lines = &grid_details[..grid_details.len() - 1]; // remove the last character which is a semicolon :
    let grid_lines = lines.parse::<usize>().map_err(|_| GridError::ParseError)?; // change the string to unsize 

    let mut grid = Vec::new();

    for i in 0..=grid_lines { // to read the grid lines
        input.clear();
        stdin.lock().read_line(input).map_err(|_| GridError::ReadError)?;
        if i < 1 { // skip the first line which contains the grid details like 0123456789........
            continue;
        } else {
            let row: Vec<char> = input[4..input.len() - 1].chars().collect();
            for (j, &ch) in row.iter().enumerate() {
                if pchars.contains(&ch) { // check if the character is a player character
                    pcoords.push((j, i)); // if it is, then add the coordinates to pcoords
                }
                if echars.contains(&ch) {
                    ecoords.push((j, i));
                }
            }
            grid.push(row);
        }
    }

    Ok((Grid { cells: grid }, pcoords, ecoords)) // return the grid, player coordinates and enemy coordinates
}
