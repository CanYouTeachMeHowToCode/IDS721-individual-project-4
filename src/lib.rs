/* library for Chess960 Board Generator */
use itertools::Itertools;

// function for generate the back rows (row 8) all possible Chess960 start boards
pub fn generate_all_chess960_boards() -> Vec<Vec<String>> {
    // Generate all possible starting positions for the back row.
    let mut result = Vec::new();
    let back_row = ["♖", "♘", "♗", "♕", "♔", "♗", "♘", "♖"];
    let back_row_permutations = back_row.iter().permutations(8).unique();
    for back_row_permutation in back_row_permutations {
        let mut position = ["0"; 8];
        // Place the back row pieces in the position
        for i in 0..8 {
            position[i] = back_row_permutation[i];
        }

        // Check if the position is valid and add it to the result if it is
        if is_valid_chess960_position(&position) {
            result.push(generate_chess_board(position));
        }
    }
    result
}

// function for checking whether the position (back row) is a valid Chess960 position
fn is_valid_chess960_position(position: &[&str]) -> bool {
    // Check that there is exactly one king
    let mut king_count = 0;
    for piece in position {
        if *piece == "♔" {
            king_count += 1;
        }
    }
    if king_count != 1 {
        return false;
    }

    // Check that the king is between the rooks
    let mut left_rook = false;
    let mut right_rook = false;
    let mut king_found = false;
    for piece in position {
        if *piece == "♖" {
            if !king_found {
                left_rook = true;
            } else {
                right_rook = true;
            }
        }
        if *piece == "♔" {
            king_found = true;
        }
    }
    let is_between = left_rook && right_rook;
    if !is_between {
        return false;
    }

    // Check that the bishops are on opposite-colored squares
    let mut first_bishop_index = 9;
    let mut second_bishop_index = 9;
    for (i, piece) in position.iter().enumerate() {
        if *piece == "♗" {
            if first_bishop_index == 9 {
                first_bishop_index = i;
            } else {
                second_bishop_index = i;
                break;
            }
        }
    }
    let is_opposite = (first_bishop_index + second_bishop_index) % 2 == 1;
    if !is_opposite {
        return false;
    }

    // All checks passed, the position is valid
    true
}

// function for generate the board from the back row vector
fn generate_chess_board(back_row: [&str; 8]) -> Vec<String> {
    let mut board = Vec::new();
    let back_row_black = to_black_piece(&back_row.concat());
    board.push(back_row_black);
    board.push(String::from("♟♟♟♟♟♟♟♟"));

    let empty_row = ".".repeat(8);
    board.push(empty_row.clone());
    board.push(empty_row.clone());
    board.push(empty_row.clone());
    board.push(empty_row);

    board.push(String::from("♙♙♙♙♙♙♙♙"));
    board.push(back_row.concat());
    board
}

// function for mapping the back row with white pieces to corrsponding row of black pieces
fn to_black_piece(back_row: &str) -> String {
    back_row
        .chars()
        .map(|c| match c {
            '♖' => '♜',
            '♘' => '♞',
            '♗' => '♝',
            '♕' => '♛',
            '♔' => '♚',
            '♙' => '♟',
            _ => c,
        })
        .collect()
}
