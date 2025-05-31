use crate::grid::read_grid;
use crate::piece::Piece;
use crate::utils::*;
use std::io::{self, BufRead};

pub fn run_game() {
    let stdin = io::stdin(); // Use stdin for reading input
    let mut input = String::new(); // to save all lines of input

    if stdin.lock().read_line(&mut input).is_err() { 
        eprintln!("Failed to read input"); 
        return;
    }

    let player_number = match input.chars().nth(10) { // get player number is at index 10
        Some(num) => num,
        None => {
            eprintln!("Invalid player number");
            return;
        }
    };

    let (pchars, echars) = initialize_chars(player_number); // initialize player and enemy characters
// like 'X' for player 1 and 'O' for player 2
    loop { 
        let (grid, pcoords, ecoords) = match read_grid(&mut input, &pchars, &echars) { // read the grid and the point of player and enemy
            Ok(data) => data,
            Err(e) => {
                eprintln!("Failed to read grid: {}", e);
                return;
            }
        };

        let piece = match Piece::read_piece(&mut input) { // read the piece from input
            Ok(p) => p,
            Err(e) => {
                eprintln!("Failed to read piece: {}", e);
                return;
            }
        };

        let (piece_x, piece_y) = find_best_placement(&grid, &piece, &pcoords, &ecoords, &pchars); // find the best placement best place for the piece

        println!("{} {}", piece_x, piece_y);
    }
}
