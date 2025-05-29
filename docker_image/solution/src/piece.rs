use std::io::{self, BufRead};
use thiserror::Error;

#[derive(Debug)]
pub struct Piece {
    pub cells: Vec<Vec<char>>,
}

#[derive(Debug, Error)]
pub enum PieceError {
    #[error("Failed to read input")]
    ReadError,
    #[error("Failed to parse piece details")]
    ParseError,
}

impl Piece {
    pub fn read_piece(input: &mut String) -> Result<Piece, PieceError> {
        let stdin = io::stdin();
        let mut piece = Vec::new();

        input.clear();
        stdin.lock().read_line(input).map_err(|_| PieceError::ReadError)?;
        let piece_details = input.split_whitespace().collect::<Vec<&str>>();
        let lines = piece_details[2];
        let piece_lines = lines[..lines.len() - 1].parse::<i32>().map_err(|_| PieceError::ParseError)?;

        for _ in 0..piece_lines {
            input.clear();
            stdin.lock().read_line(input).map_err(|_| PieceError::ReadError)?;
            let row: Vec<char> = input[..input.len() - 1].chars().collect();
            piece.push(row);
        }

        Ok(Piece { cells: piece })
    }
}
