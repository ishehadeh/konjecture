pub mod bitboard;
#[cfg(feature = "cgt")]
pub mod cgt;
use std::{fmt::Debug, marker::PhantomData, str::FromStr};
mod konane_dyn_dim;
pub use konane_dyn_dim::*;
pub mod invariant;
use bitarray::{iter::BitArrayIter, BitArray};
use bitboard::{BitBoard, Direction};
use const_direction::ConstDirection;
use thiserror::Error;

impl<const N: usize> BitBoard for BitArray<N, u64> {
    const BIT_LENGTH: usize = 64 * N;
    type Iter<'a> = BitArrayIter<'a, true, false, N, u64>;

    fn empty() -> Self {
        [0u64; N].into()
    }

    fn one() -> Self {
        let mut v = BitArray::empty();
        v.set(0);
        v
    }

    #[inline(always)]
    fn all() -> Self {
        !Self::empty()
    }

    #[inline(always)]
    fn first_set(&self) -> Option<usize> {
        BitArray::<N, u64>::first_set(self)
    }

    #[inline(always)]
    fn first_clear(&self) -> Option<usize> {
        BitArray::<N, u64>::first_clear(self)
    }

    #[inline(always)]
    fn count_set(&self) -> usize {
        BitArray::<N, u64>::iter_set(self).count()
    }

    #[inline(always)]
    fn count_clear(&self) -> usize {
        BitArray::<N, u64>::iter_set(self).count()
    }

    #[inline(always)]
    fn last_set(&self) -> Option<usize> {
        self.last_set()
    }

    fn iter_set(&self) -> Self::Iter<'_> {
        BitArray::iter_set(&self)
    }

    fn set(&mut self, idx: usize) {
        BitArray::set(self, idx);
    }

    fn get(&self, idx: usize) -> bool {
        BitArray::get(self, idx)
    }

    fn clear(&mut self, idx: usize) {
        BitArray::clear(self, idx);
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Konane256<const W: usize = 16, const H: usize = 16, B: BitBoard = BitArray<4, u64>> {
    pub white: B,
    pub black: B,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TileState {
    White,
    Black,
    Empty,
}

impl<const W: usize, const H: usize, B: BitBoard> std::fmt::Debug for Konane256<W, H, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Konane256<{}, {}> {{", W, H)?;
        for y in 0..H {
            write!(f, "   ")?;
            for x in 0..W {
                match self.get_tile(x, y) {
                    TileState::White => write!(f, "x")?,
                    TileState::Black => write!(f, "o")?,
                    TileState::Empty => write!(f, "_")?,
                }
            }
            writeln!(f, "")?;
        }
        writeln!(f, "}}")
    }
}

#[derive(Error, Debug, Clone)]
pub enum KonaneParseError {
    #[error("expected one of 'x', 'o', or '_', got '{c}'")]
    UnexpectedCharacter { c: char },
}

impl<const W: usize, const H: usize, B: BitBoard> FromStr for Konane256<W, H, B> {
    fn from_str(s: &str) -> Result<Self, KonaneParseError> {
        let mut game = Self::empty();
        for (y, row_txt) in s
            .trim()
            .split("\n")
            .map(|row| row.trim())
            .take(H)
            .enumerate()
        {
            for (x, c) in row_txt.chars().take(W).enumerate() {
                match c {
                    'x' => game.set_tile(x, y, TileState::White),
                    'o' => game.set_tile(x, y, TileState::Black),
                    '_' => game.set_tile(x, y, TileState::Empty),
                    c => return Err(KonaneParseError::UnexpectedCharacter { c }),
                }
            }
        }

        Ok(game)
    }

    type Err = KonaneParseError;
}

impl<const W: usize, const H: usize, B: BitBoard> Konane256<W, H, B> {
    /// x => white, o => black, _ => empty
    pub fn must_parse(s: &str) -> Self {
        Self::from_str(s).expect("failed to parse game")
    }

    pub fn empty() -> Self {
        assert!(W * H <= B::BIT_LENGTH);
        Self {
            white: B::empty(),
            black: B::empty(),
        }
    }

    pub fn border_mask(dir: Direction) -> B {
        let mut base = B::empty();
        match dir {
            Direction::Up => {
                for i in 0..W {
                    base.set(i)
                }
            }
            Direction::Down => {
                for i in W * (H - 1)..W * H {
                    base.set(i)
                }
            }
            Direction::Right => {
                for i in 1..=H {
                    base.set((W - 1) * i)
                }
            }
            Direction::Left => {
                for i in 0..H {
                    base.set(W * i)
                }
            }
        }

        base
    }

    pub fn small((rows, cols): (usize, usize), tiles: &[TileState]) -> Self {
        let row_start = (H - rows) / 2;
        let col_start = (W - cols) / 2;
        Self::small_at((row_start, col_start), (rows, cols), tiles)
    }

    pub fn small_at(
        (x_start, y_start): (usize, usize),
        (rows, columns): (usize, usize),
        tiles: &[TileState],
    ) -> Self {
        assert!(x_start < W);
        assert!(y_start < H);

        let row_end = x_start + columns;
        let col_end = y_start + rows;

        assert!(col_end <= W);
        assert!(row_end <= H);

        let mut board = Self::empty();
        for i in 0..columns {
            for j in 0..rows {
                board.set_tile(i + x_start, j + y_start, tiles[i + j * columns]);
            }
        }

        board
    }

    pub fn checkerboard() -> Self {
        let mut board = Self::empty();

        for x in 0..W {
            for y in 0..H {
                board.set_tile(
                    x,
                    y,
                    if (x + y) % 2 == 0 {
                        TileState::Black
                    } else {
                        TileState::White
                    },
                );
            }
        }

        board
    }

    pub fn xy_to_idx(x: usize, y: usize) -> usize {
        y * W + x
    }

    pub fn set_tile(&mut self, x: usize, y: usize, state: TileState) {
        assert!(x < W);
        assert!(y < H);

        let i = Self::xy_to_idx(x, y);
        match state {
            TileState::White => {
                self.white.set(i);
                self.black.clear(i);
            }
            TileState::Black => {
                self.black.set(i);
                self.white.clear(i);
            }
            TileState::Empty => {
                self.white.clear(i);
                self.black.clear(i);
            }
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> TileState {
        let i = Self::xy_to_idx(x, y);
        match (self.black.get(i), self.white.get(i)) {
            (true, true) => panic!("Tile at <{}, {}> is marked for both black and white", x, y),
            (false, false) => TileState::Empty,
            (true, false) => TileState::Black,
            (false, true) => TileState::White,
        }
    }

    pub fn empty_spaces(&self) -> B {
        // get empty by selecting non-black spaces that don't have a white piece.
        // and clear extra bits
        !(self.black.clone()
            | &self.white
            | if W * H < B::BIT_LENGTH {
                // necessary to avoid overflow panics
                B::all() << W * H
            } else {
                B::empty()
            })
    }

    pub fn move_generator_white<'a, Dir: ConstDirection>(
        &'a self,
        _: Dir,
    ) -> MoveGenerator<'a, Dir, W, H, true, B> {
        MoveGenerator::new(self)
    }

    pub fn move_generator_black<'a, Dir: ConstDirection>(
        &'a self,
        _: Dir,
    ) -> MoveGenerator<'a, Dir, W, H, false, B> {
        MoveGenerator::new(self)
    }

    pub fn move_generator<'a, const IS_WHITE: bool, Dir: ConstDirection>(
        &'a self,
        _: Dir,
    ) -> MoveGenerator<'a, Dir, W, H, IS_WHITE, B> {
        MoveGenerator::new(self)
    }

    pub fn all_moves_white(&self) -> Vec<Self> {
        self.all_moves::<true>()
    }

    pub fn all_moves_black(&self) -> Vec<Self> {
        self.all_moves::<false>()
    }

    pub fn all_moves<const IS_WHITE: bool>(&self) -> Vec<Self> {
        let mut moves: Vec<Self> = Default::default();
        let mut r = self.move_generator::<IS_WHITE, _>(const_direction::Right);
        let mut l = self.move_generator::<IS_WHITE, _>(const_direction::Left);
        let mut u = self.move_generator::<IS_WHITE, _>(const_direction::Up);
        let mut d = self.move_generator::<IS_WHITE, _>(const_direction::Down);
        while !r.is_complete() {
            r.advance();
            for mv in r.move_iter() {
                moves.push(mv.apply(self.clone()))
            }
        }
        while !l.is_complete() {
            l.advance();
            for mv in l.move_iter() {
                moves.push(mv.apply(self.clone()))
            }
        }
        while !u.is_complete() {
            u.advance();
            for mv in u.move_iter() {
                moves.push(mv.apply(self.clone()))
            }
        }
        while !d.is_complete() {
            d.advance();
            for mv in d.move_iter() {
                moves.push(mv.apply(self.clone()))
            }
        }

        moves
    }
}

#[derive(Debug)]
pub struct MoveGenerator<
    'a,
    Dir: ConstDirection,
    const W: usize,
    const H: usize,
    const IS_WHITE: bool,
    B: BitBoard,
> {
    game: &'a Konane256<W, H, B>,
    moves: B,
    hops: usize,
    _dir: PhantomData<Dir>,
}

impl<
        'a,
        Dir: ConstDirection,
        const W: usize,
        const H: usize,
        const IS_WHITE: bool,
        B: BitBoard,
    > MoveGenerator<'a, Dir, W, H, IS_WHITE, B>
{
    pub const fn direction() -> Direction {
        Dir::VALUE
    }

    pub fn is_complete(&self) -> bool {
        self.moves == B::empty()
    }

    pub fn new(game: &'a Konane256<W, H, B>) -> Self {
        let mut moves: B = Konane256::<W, H, B>::border_mask(Self::direction());
        moves = !moves;
        moves &= if IS_WHITE { &game.white } else { &game.black };

        Self {
            game,
            moves,
            hops: 0,
            _dir: PhantomData,
        }
    }

    /// "move" the pieces by using a bit shift on the board
    fn shift(&mut self) {
        match Self::direction() {
            Direction::Right => self.moves <<= 1,
            Direction::Left => self.moves >>= 1,
            Direction::Up => self.moves >>= W,
            Direction::Down => self.moves <<= W,
        }
    }

    pub fn advance(&mut self) {
        if self.moves == B::empty() {
            return;
        }

        // 1. verify that there's a capture-able adjacent piece
        self.shift();
        if IS_WHITE {
            self.moves &= &self.game.black;
        } else {
            self.moves &= &self.game.white;
        }

        // 2. verify there's an empty space after the piece to be jumped
        self.shift();
        self.moves &= self.game.empty_spaces();

        self.hops += 1;
    }

    pub fn move_iter(&'a self) -> impl Iterator<Item = Move<W, H, IS_WHITE, B>> + 'a {
        self.moves.iter_set().map(|index| Move {
            from: (index as isize
                + 2 * (Dir::VALUE.x() + Dir::VALUE.y() * W as isize) * self.hops as isize)
                as u16,
            to: index as u16,
            _game: PhantomData,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Move<const W: usize, const H: usize, const IS_WHITE: bool, B: BitBoard> {
    from: u16,
    to: u16,
    _game: PhantomData<Konane256<W, H, B>>,
}

impl<const W: usize, const H: usize, const IS_WHITE: bool, B: BitBoard> Move<W, H, IS_WHITE, B> {
    pub fn apply(&self, mut game: Konane256<W, H, B>) -> Konane256<W, H, B> {
        let start = self.from.min(self.to);
        let end = self.from.max(self.to);
        let step = if end - start >= W as u16 { W as u16 } else { 1 };
        let mut i = start;
        while i <= end {
            game.black.clear(i as usize);
            game.white.clear(i as usize);
            i += step;
        }
        if IS_WHITE {
            game.white.set(self.to as usize)
        } else {
            game.black.set(self.to as usize)
        }

        game
    }
}

mod const_direction {
    use std::fmt::Debug;

    use crate::bitboard::Direction;

    pub trait ConstDirection: Debug {
        const VALUE: Direction;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Up;
    impl ConstDirection for Up {
        const VALUE: Direction = Direction::Up;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Down;
    impl ConstDirection for Down {
        const VALUE: Direction = Direction::Down;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Left;
    impl ConstDirection for Left {
        const VALUE: Direction = Direction::Left;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Right;
    impl ConstDirection for Right {
        const VALUE: Direction = Direction::Right;
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::{Konane256, TileState};

    #[test]
    pub fn checkerboard_16x16() {
        let board: Konane256<16, 16> = Konane256::checkerboard();
        for x in 0..16 {
            for y in 0..16 {
                assert_ne!(board.get_tile(x, y), TileState::Empty);
                if x > 0 {
                    assert_ne!(board.get_tile(x, y), board.get_tile(x - 1, y));
                }
                if y > 1 {
                    assert_ne!(board.get_tile(x, y), board.get_tile(x, y - 1));
                }
            }
        }
    }

    #[test]
    pub fn checkerboard_11x11() {
        let board: Konane256<11, 11, u128> = Konane256::checkerboard();
        for x in 0..11 {
            for y in 0..11 {
                assert_ne!(board.get_tile(x, y), TileState::Empty);
                if x > 0 {
                    assert_ne!(board.get_tile(x, y), board.get_tile(x - 1, y));
                }
                if y > 1 {
                    assert_ne!(board.get_tile(x, y), board.get_tile(x, y - 1));
                }
            }
        }
    }

    #[test]
    pub fn move_near_block_boundary() {
        let board: Konane256<256, 1> = Konane256::must_parse(
            r#"_oxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxox"#,
        );

        let w = Konane256::must_parse(
            r#"x__oxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxox"#,
        );
        let b = Konane256::must_parse(
            r#"_oxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxox__o"#,
        );

        assert_eq!(board.all_moves_white(), vec![w]);
        assert_eq!(board.all_moves_black(), vec![b]);
    }

    #[test]
    pub fn move_over_block_boundary() {
        let board: Konane256<256, 1> = Konane256::must_parse(
            r#"_oxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxox"#,
        );

        let w = Konane256::must_parse(
            r#"x__oxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxox"#,
        );
        let b = Konane256::must_parse(
            r#"_oxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxox__o"#,
        );

        assert_eq!(board.all_moves_white(), vec![w]);
        assert_eq!(board.all_moves_black(), vec![b]);
    }

    #[test]
    pub fn moveset_on_full_board_is_empty_16x16() {
        let board: Konane256<16, 16> = Konane256::checkerboard();

        assert_eq!(board.all_moves_black(), vec![]);
        assert_eq!(board.all_moves_white(), vec![]);
    }

    #[test]
    pub fn moveset_on_full_board_is_empty_11x11() {
        let board = Konane256::<11, 11, u128>::checkerboard();
        dbg!(&board);
        assert_eq!(board.all_moves_black(), vec![]);
        assert_eq!(board.all_moves_white(), vec![]);
    }

    #[test]
    pub fn moveset_white_right_jump() {
        let board: Konane256 = Konane256::must_parse("xo");
        assert_eq!(board.all_moves_white(), vec![Konane256::must_parse("__x")]);
        assert_eq!(board.all_moves_black(), vec![]);

        let board: Konane256<4, 4, u32> = Konane256::must_parse("xo");
        assert_eq!(board.all_moves_white(), vec![Konane256::must_parse("__x")]);
        assert_eq!(board.all_moves_black(), vec![]);
    }

    #[test]
    pub fn moveset_black_right_jump() {
        let board: Konane256 = Konane256::must_parse("ox");
        assert_eq!(board.all_moves_black(), vec![Konane256::must_parse("__o")]);
        assert_eq!(board.all_moves_white(), vec![]);
    }

    #[test]
    pub fn moveset_white_left_jump() {
        let board: Konane256 = Konane256::must_parse("_oxx");
        assert_eq!(board.all_moves_white(), vec![Konane256::must_parse("x__x")]);
        assert_eq!(board.all_moves_black(), vec![]);
    }

    #[test]
    pub fn moveset_black_left_jump() {
        let board: Konane256 = Konane256::must_parse("_xoo");
        assert_eq!(board.all_moves_black(), vec![Konane256::must_parse("o__o")]);
        assert_eq!(board.all_moves_white(), vec![]);
    }

    #[test]
    pub fn moveset_white_up_jump() {
        let board: Konane256 = Konane256::must_parse("_\no\nx\nx");
        assert_eq!(
            board.all_moves_white(),
            vec![Konane256::must_parse("x\n\n\nx")]
        );
        assert_eq!(board.all_moves_black(), vec![]);
    }

    #[test]
    pub fn moveset_black_up_jump() {
        let board: Konane256 = Konane256::must_parse("_\nx\no\no");
        assert_eq!(
            board.all_moves_black(),
            vec![Konane256::must_parse("o\n\n\no")]
        );
        assert_eq!(board.all_moves_white(), vec![]);
    }

    #[test]
    pub fn moveset_white_down_jump() {
        let board: Konane256 = Konane256::must_parse("x\no");
        assert_eq!(
            board.all_moves_white(),
            vec![Konane256::must_parse("_\n\nx")]
        );
        assert_eq!(board.all_moves_black(), vec![]);
    }

    #[test]
    pub fn moveset_black_down_jump() {
        let board: Konane256 = Konane256::must_parse("o\nx");
        assert_eq!(
            board.all_moves_black(),
            vec![Konane256::must_parse("_\n\no")]
        );
        assert_eq!(board.all_moves_white(), vec![]);
    }

    #[test]
    pub fn linear_tail_1_with_4_stones() {
        let board: Konane256 = Konane256::must_parse(
            r#"_____
               _oxo_
               _x___
               _____"#,
        );
        assert_eq!(
            board.all_moves_black(),
            vec![Konane256::must_parse(
                r#"_____
                   __xo_
                   _____
                   _o___"#
            )]
        );
        assert_eq!(
            HashSet::from_iter(board.all_moves_white().into_iter()),
            HashSet::from([
                Konane256::must_parse(
                    r#"_____
                       _o__x
                       _x___
                       _____"#
                ),
                Konane256::must_parse(
                    r#"_____
                       x__o_
                       _x___
                       _____"#
                ),
                Konane256::must_parse(
                    r#"_x___
                       __xo_
                       _____
                       _____"#
                ),
            ])
        );
    }
}
