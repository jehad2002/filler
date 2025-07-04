use crate::piece::Piece;
use crate::grid::Grid;

pub fn initialize_chars(player_number: char) -> (Vec<char>, Vec<char>) { // Initialize player and enemy characters based on the player number
    // Player 1 uses '@' and 'a', Player 2 uses '$' and 's'
    if player_number == '1' {
        (vec!['@', 'a'], vec!['$', 's'])
    } else {
        (vec!['$', 's'], vec!['@', 'a'])
    }
}

pub fn find_best_placement( // Find the best placement for a piece on the grid
    grid: &Grid,
    piece: &Piece,
    player_coords: &[(usize, usize)],
    enemy_coords: &[(usize, usize)],
    player_chars: &[char],
) -> (usize, usize) {
    // point of map 
    let grid_width = grid.cells[0].len();
    let grid_height = grid.cells.len();
    let piece_width = piece.cells[0].len();
    let piece_height = piece.cells.len();

    let mut min_distance = ((grid_width as f32).powf(2.0) + (grid_height as f32).powf(2.0)).sqrt(); 
    let mut best_placement = (0, 0); // Initialize best placement to (0, 0)

    let (mut min_x, mut max_x, mut min_y, mut max_y) = (grid_width, 0, grid_height, 0); // Initialize min and max coordinates
        // calculate the min and max coordinates of player pieces
    // to find the best placement for the piece
    for (x, y) in player_coords {
        min_x = min_x.min(*x);
        max_x = max_x.max(*x);
        min_y = min_y.min(*y);
        max_y = max_y.max(*y);
    }

    let start_x = min_x.saturating_sub(piece_width.saturating_sub(1));
    let end_x = (max_x + 1).min(grid_width - piece_width + 1);
    let start_y = min_y.saturating_sub(piece_height);
    let end_y = max_y.min(grid_height - piece_height);

    for y in start_y..end_y { // try to place the piece in all possible positions
        for x in start_x..end_x {
            if can_place_piece(grid, piece, player_chars, x, y) { // Check if the piece can be placed at (x, y)
                // Calculate the minimum distance to any enemy piece from the current placement
                let dist_to_enemy = calculate_min_distance(piece, enemy_coords, (x, y), min_distance);
                if dist_to_enemy < min_distance { // If the distance is less than the current minimum distance
                    // Update the minimum distance and best placement
                    min_distance = dist_to_enemy;
                    best_placement = (x, y); 
                }
            }
        }
    }

    best_placement // return the best placement coordinates
}

fn calculate_min_distance( // Calculate the minimum distance from the piece to any enemy piece
    piece: &Piece,
    enemy_coords: &[(usize, usize)],
    (grid_x, grid_y): (usize, usize),
    initial_distance: f32,
) -> f32 {
    piece.cells.iter().enumerate().flat_map(|(piece_y, row)| {
        row.iter().enumerate().filter_map(move |(piece_x, &cell)| {
            if cell != '.' {
                enemy_coords.iter().map(move |&(enemy_x, enemy_y)| {
                    (((enemy_y as f32) - ((piece_y + grid_y) as f32)).powi(2)
                    + ((enemy_x as f32) - ((piece_x + grid_x) as f32)).powi(2)).sqrt()
                }).min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            } else {
                None
            }
        })
    }).fold(initial_distance, f32::min) // saave the minimum distance
}

fn can_place_piece(grid: &Grid, piece: &Piece, pchars: &[char], xg: usize, yg: usize) -> bool { // Check if the piece can be placed on the grid at position (xg, yg)
    // Check if the piece can be placed on the grid at position (xg, yg)
    // the rules 1- the piece in the map if not print error 
    // 2- the piece can not cross the opponent territory
    // 3- the pice canot cross the enemy territory
    let mut territory_crossing = 0;
    let mut opponent_territory_crossing = false; // to check if the piece crosses the opponent territory

    for (yp, row) in piece.cells.iter().enumerate() {
        for (xp, &cell) in row.iter().enumerate() {
            if cell != '.' {
                let x = xg + xp;
                let y = yg + yp;

                if !grid.is_inside(x, y) {
                    return false;
                }

                let grid_cell = grid.cells[y][x];
                if pchars.contains(&grid_cell) {
                    territory_crossing += 1;
                    if territory_crossing > 1 {
                        return false;
                    }
                } else if grid_cell != '.' {
                    opponent_territory_crossing = true;
                }
            }
        }
    }

    territory_crossing == 1 && !opponent_territory_crossing
}
