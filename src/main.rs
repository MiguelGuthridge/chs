use game::{Position, Board};

pub mod game;

fn num_moves(board: &mut Board, depth: i32) -> i64 {
    if depth == 0 {
        println!("{}", board);
        return 1;
    }
    let mut count = 0;
    for i in 0..64 {
        let pos = Position::from(i);
        if let Some(piece) = board.at_position(pos) {
            if piece.color == board.whose_turn() {
                let moves = board.get_moves(pos);
                for turn in moves {
                    board.make_turn(turn);
                    count += num_moves(board, depth - 1);
                    board.undo_turn().expect("Should be a turn");
                }
            }
        }
    }
    count
}

fn main() {
    let depth = 6;

    let mut board = Board::new_from_start();

    let num = num_moves(&mut board, depth);

    assert!(board.undo_turn().is_none());

    println!("Num moves at {} ply: {}", depth, num);
}
