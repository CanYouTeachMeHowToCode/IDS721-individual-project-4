use rand::{thread_rng, Rng};
fn main() {
    let all_boards = project4::generate_all_chess960_boards();
    let num_board = all_boards.len(); // equals to 960 here
    let board_index = thread_rng().gen_range(0..num_board);
    let random_board = &all_boards[board_index];
    println!("{random_board:#?}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_num_boards() {
        assert_eq!(project4::generate_all_chess960_boards().len(), 960);
    }
}
