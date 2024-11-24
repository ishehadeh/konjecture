use std::fs::File;

use cgt::short::partizan::partizan_game::PartizanGame;
use cgt::short::partizan::transposition_table::ParallelTranspositionTable;
use itertools::Itertools;
use konane::{invariant::*, Konane256, TileState};
use polars::prelude::*;

#[allow(non_snake_case)]
fn all_NxN_in_8x8(w: usize, h: usize) -> impl Iterator<Item = Konane256<8, 8, u64>> {
    let start_x = (8 - w) / 2;
    let start_y = (8 - h) / 2;
    (0..w * h)
        .map(|_| [TileState::Empty, TileState::Black, TileState::White].into_iter())
        .multi_cartesian_product()
        .map(move |v| {
            let mut game = Konane256::empty();
            let mut i = 0;
            for x in start_x..(start_x + w) {
                for y in start_y..(start_y + h) {
                    game.set_tile(x, y, v[i]);
                    i += 1;
                }
            }
            game
        })
}

pub fn main() {
    macro_rules! b {
        ($v:expr) => {
            Box::new(PartizanInvariant::left($v))
        };
    }
    macro_rules! w {
        ($v:expr) => {
            Box::new(PartizanInvariant::right($v))
        };
    }
    // keep our transposition table (i.e. canonical form cache) around for the entire program
    let tt = Box::leak(Box::new(ParallelTranspositionTable::new()));

    let invariants: Vec<(&'static str, Box<dyn Invariant<Konane256<8, 8, _>>>)> = vec![
        ("wH", w!(PieceHeight)),
        ("bH", b!(PieceHeight)),
        ("wCnt", w!(PieceCount)),
        ("bCnt", b!(PieceCount)),
        ("wBrdr", w!(NearestBorder)),
        ("bBrdr", b!(NearestBorder)),
        ("wW", w!(PieceWidth)),
        ("bW", b!(PieceWidth)),
        ("wCaptures", Box::new(CaptureCount::right())),
        ("bCaptures", Box::new(CaptureCount::left())),
        ("wMoves", Box::new(MoveCount::right())),
        ("bMoves", Box::new(MoveCount::left())),
        ("nimber", Box::new(CanonicalFormNimber::new(tt))),
        ("number", Box::new(CanonicalFormNumber::new(tt))),
    ];

    const W: u32 = 4;
    const H: u32 = 3;
    let mut b_bitmaps = Vec::new();
    let mut w_bitmaps = Vec::new();
    let mut canon = Vec::new();
    let mut invariant_values = Vec::from_iter(invariants.iter().map(|_| Vec::new()));
    let mut i = 0u64;
    let five_percent = 3u64.pow(W * H) / 20;
    for game in all_NxN_in_8x8(W as usize, H as usize) {
        canon.push(game.canonical_form(tt).to_string());
        b_bitmaps.push(game.black);
        w_bitmaps.push(game.white);

        for (i, (_, invariant)) in invariants.iter().enumerate() {
            invariant_values[i].push(invariant.compute(game.clone()));
        }
        if i % five_percent == 0 {
            println!("complete: {}%", (i / five_percent) * 5)
        }
        i += 1;
    }

    let mut df = df!(
        "bBitmaps" => b_bitmaps,
        "wBitmaps" => w_bitmaps,
        "canonicalForm" => canon,
    )
    .expect("failed to create dataframe");
    for (i, (name, _)) in invariants.iter().enumerate() {
        df.with_column(Column::new((*name).into(), &invariant_values[i]))
            .unwrap();
    }

    let file = File::create("polars.parquet").expect("could not create file");

    ParquetWriter::new(file).finish(&mut df);
}
