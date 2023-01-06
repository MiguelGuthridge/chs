use game::Position;

pub mod game;

fn main() {
    for row in 0..8 {
        for col in 0..8 {
            let pos = Position::new(row, col);
            print!("{pos} ");
        }
        println!();
    }
}
