use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let all_boards = project4::generate_all_chess960_boards();
    println!("{all_boards:?}");
}
