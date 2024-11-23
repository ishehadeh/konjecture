use bitarray::BitArray;
use cgt::short::partizan::partizan_game::PartizanGame;
use cgt::short::partizan::transposition_table::ParallelTranspositionTable;
use itertools::Itertools;
use konane::{
    invariant::{
        CanonicalFormNimber, CanonicalFormNumber, CaptureCount, Invariant, MoveCount,
        NearestBorder, PartizanInvariant, PieceCount, PieceHeight, PieceWidth,
    },
    BitBoard, Konane256, TileState,
};
use std::{collections::VecDeque, io::Write};
pub struct PermutationIter<const W: usize, const H: usize, B: BitBoard> {
    pub min: (usize, usize),
    pub max: (usize, usize),
    pub idx: usize,
    pub stack: VecDeque<Konane256<W, H, B>>,
}

fn linear_with_tail(
    tail_len: usize,
    n: usize,
    offset: usize,
) -> Konane256<64, 12, BitArray<12, u64>> {
    let mut game = Konane256::empty();
    for i in 0..tail_len {
        game.set_tile(
            1,
            (12 - tail_len) / 2 + i,
            if i % 2 == 0 {
                TileState::White
            } else {
                TileState::Black
            },
        );
    }

    for x in offset..(n - tail_len + offset) {
        game.set_tile(
            x + 1,
            (12 - tail_len) / 2,
            if x % 2 == 0 {
                TileState::Black
            } else {
                TileState::White
            },
        )
    }

    game
}

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

pub struct ConjecturingInput<const W: usize, const H: usize, B: BitBoard> {
    pub invariants: Vec<(&'static str, Box<dyn Invariant<Konane256<W, H, B>>>)>,
    pub ops: Vec<&'static str>,
    pub main_invariant: usize,
    pub objects: Vec<Konane256<W, H, B>>,
}

impl<const W: usize, const H: usize, B: BitBoard> ConjecturingInput<W, H, B> {
    fn get_op_line(pretty_op: &str) -> &'static str {
        [
            ("-1", "U 0"),
            ("+1", "U 1"),
            ("*2", "U 2"),
            ("/2", "U 3"),
            ("^2", "U 4"),
            ("-()", "U 5"),
            ("1/", "U 6"),
            ("sqrt", "U 7"),
            ("ln", "U 8"),
            ("log10", "U 9"),
            ("exp", "U 10"),
            ("10^", "U 11"),
            ("ceil", "U 12"),
            ("floor", "U 13"),
            ("abs", "U 14"),
            ("sin", "U 15"),
            ("cos", "U 16"),
            ("tan", "U 17"),
            ("asin", "U 18"),
            ("acos", "U 19"),
            ("atan", "U 20"),
            ("sinh", "U 21"),
            ("cosh", "U 22"),
            ("tanh", "U 23"),
            ("asinh", "U 24"),
            ("acosh", "U 25"),
            ("atanh", "U 26"),
            ("+", "C 0"),
            ("*", "C 1"),
            ("max", "C 2"),
            ("min", "C 3"),
            ("-", "N 0"),
            ("/", "N 1"),
            ("^", "N 2"),
        ]
        .into_iter()
        .find(|(a, _)| pretty_op == *a)
        .unwrap()
        .1
    }

    pub fn write_header<O: Write>(&self, invariant_names: bool, out: &mut O) {
        writeln!(out, "{}", self.ops.len()).unwrap();
        for &op in &self.ops {
            writeln!(out, "{}", Self::get_op_line(op)).unwrap();
        }

        writeln!(
            out,
            "{} {} {}",
            self.objects.len(),
            self.invariants.len(),
            self.main_invariant
        )
        .unwrap();
        if invariant_names {
            for (n, _) in &self.invariants {
                writeln!(out, "{}", n).unwrap();
            }
        }
    }

    pub fn write_data<O: Write>(&self, out: &mut O) {
        for object in &self.objects {
            for (_, invariant) in &self.invariants {
                writeln!(out, "{}", invariant.compute(object.clone())).unwrap();
            }
        }
    }

    pub fn csv(&self) -> String {
        let mut s = String::with_capacity(1024);
        for (n, _) in &self.invariants {
            s.push_str(n);
            s.push(',');
        }
        // remove last comma
        s.pop();
        s.push('\n');
        for object in &self.objects {
            for (_, invariant) in &self.invariants {
                s.push_str(&invariant.compute(object.clone()).to_string());
                s.push(',');
            }
            s.pop();
            s.push('\n');
        }

        s
    }
}
pub fn main() {
    // keep our transposition table (i.e. canonical form cache) around for the entire program
    let tt = Box::leak(Box::new(ParallelTranspositionTable::new()));

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

    let conjecturing = ConjecturingInput {
        invariants: vec![
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
        ],
        ops: vec!["+", "-", "*", "-()", "/", "*2", "+1", "-1"],
        main_invariant: 13,
        objects: all_NxN_in_8x8(4, 3)
            .filter(|g| {
                let c = g.canonical_form(tt);
                c.is_nimber() && c.to_nus().unwrap().nimber().value() > 1
            })
            .collect(),
    };
    let mut stdout = std::io::stdout().lock();
    // conjecturing.write_header(true, &mut stdout);
    // conjecturing.write_data(&mut stdout);
    println!("{}", conjecturing.csv());
}
