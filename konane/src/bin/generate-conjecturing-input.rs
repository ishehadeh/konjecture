use cgt::short::partizan::partizan_game::PartizanGame;
use cgt::short::partizan::transposition_table::ParallelTranspositionTable;
use konane::{
    invariant::{
        self, ImpartialInvariant, Invariant, NearestBorder, PartizanInvariant, PieceCount,
        PieceHeight, PieceWidth,
    },
    Konane256, TileState,
};
use rand::{Rng, RngCore};
use std::{fs, io::Read};

fn arbitrary_3x3_in_8x8() -> Konane256<6, 6> {
    let mut board = Konane256::<6, 6>::empty();
    for x in 0..6usize {
        for y in 0..6usize {
            let v = rand::thread_rng().next_u32();
            let empty_range = u32::MAX - u32::MAX / 2;
            if v > empty_range {
                board.set_tile(x, y, TileState::Empty);
            } else if v > empty_range / 2 {
                board.set_tile(x, y, TileState::Black);
            } else {
                board.set_tile(x, y, TileState::White);
            }
        }
    }
    board
}

pub fn main() {
    let mut n_games = 0;
    let tt = ParallelTranspositionTable::new();
    let invariants = (
        PartizanInvariant::left(PieceCount),
        PartizanInvariant::right(PieceCount),
        PartizanInvariant::left(NearestBorder),
        PartizanInvariant::right(NearestBorder),
    );
    println!(
        r#"7
U  0    x - 1
U  1    x + 1
C 0    x + y
C 1    x * y
C 2    max(x,y)
C 3    min(x,y)
N 0    x - y"#
    );
    let max_games = 10;
    println!("{} {} 1", max_games, 4 + 1);
    while n_games < max_games {
        let next_game = arbitrary_3x3_in_8x8();
        let score = next_game.canonical_form(&tt);
        if score.is_number() {
            n_games += 1;
            let (l_count, r_count, l_nb, r_nb) = invariants.compute(next_game);
            let score = score.to_number().unwrap();
            print!(
                "{}\n{}\n{}\n{}\n{}\n",
                (score.numerator() as f64) / 2.0f64.powi(score.denominator_exponent() as i32),
                l_count,
                r_count,
                l_nb,
                r_nb,
            )
        }
    }
}
