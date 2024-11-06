use cgt::short::partizan::partizan_game::PartizanGame;
use cgt::short::partizan::transposition_table::{ParallelTranspositionTable, TranspositionTable};
use konane::{Konane256, TileState};
use std::{fs, io::Read};

pub fn main() {
    let board_text = {
        let mut txt = String::new();
        fs::File::open("konane.txt")
            .expect("failed to open file")
            .read_to_string(&mut txt)
            .expect("failed to read file");
        txt
    };

    let mut game = Konane256::<16, 16>::empty();
    dbg!(&game);

    for (y, row_txt) in board_text.split("\n").take(16).enumerate() {
        for (x, c) in row_txt.chars().take(16).enumerate() {
            match c {
                'x' => game.set_tile(x, y, TileState::White),
                'o' => game.set_tile(x, y, TileState::Black),
                '.' => game.set_tile(x, y, TileState::Empty),
                _ => panic!("invalid tile character: {:?}", c),
            }
        }
    }
    dbg!(&game);

    let mut tt = ParallelTranspositionTable::new();
    let canon = game.canonical_form(&mut tt);
    println!("{}", canon.to_moves().print_deep_to_str());
}
