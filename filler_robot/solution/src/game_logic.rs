use crate::grid::{Grid, read_grid};
use crate::piece::Piece;
use crate::utils::*;
use std::io::{self, BufRead};

pub fn run_game() {
    let stdin = io::stdin();
    let mut input = String::new();
    
    if stdin.lock().read_line(&mut input).is_err() {
        eprintln!("Failed to read input");
        return;
    }

    let player_number = match input.chars().nth(10) {
        Some(num) => num,
        None => {
            eprintln!("Invalid player number");
            return;
        }
    };

    let (pchars, echars) = initialize_chars(player_number);

    loop {
        let (grid, pcoords, ecoords) = match read_grid(&mut input, &pchars, &echars) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Failed to read grid: {}", e);
                return;
            }
        };

        let piece = match Piece::read_piece(&mut input) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Failed to read piece: {}", e);
                return;
            }
        };

        let (piece_x, piece_y) = find_best_placement(&grid, &piece, &pcoords, &ecoords, &pchars);

        println!("{} {}", piece_x, piece_y);
    }
}
