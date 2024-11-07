use cgt::short::partizan::partizan_game::PartizanGame;
use cgt::short::partizan::transposition_table::ParallelTranspositionTable;
use konane::Konane256;
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

    let game = Konane256::<16, 16>::must_parse(&board_text);

    let mut tt = ParallelTranspositionTable::new();
    let canon = game.canonical_form(&mut tt);
    println!("{}", canon);
}
