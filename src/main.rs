use game::Board;

pub mod game;

fn num_moves(board: &mut Board, depth: i32) -> i64 {
    if depth == 0 {
        // println!("{}", board);
        return 1;
    }

    let mut count = 0;
    let moves = board.get_moves();
    for turn in moves {
        board.make_turn(turn);
        count += num_moves(board, depth - 1);
        board.undo_turn().expect("Should be a turn");
    }
    count
}

fn main() {
    let depth = 6;

    let mut board = Board::from_start();

    let num = num_moves(&mut board, depth);

    assert!(board.undo_turn().is_none());

    println!("Num moves at {} ply: {}", depth, num);
}
