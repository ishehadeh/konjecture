pub mod bitboard;
#[cfg(feature = "cgt")]
pub mod cgt;
use std::marker::PhantomData;
pub mod invariant;
use bitarray::BitArray;
use bitboard::{BitBoard256, Direction};
use const_direction::ConstDirection;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Konane256<const W: usize = 16, const H: usize = 16> {
    pub white: BitBoard256<W, H>,
    pub black: BitBoard256<W, H>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TileState {
    White,
    Black,
    Empty,
}

impl<const W: usize, const H: usize> std::fmt::Debug for Konane256<W, H> {
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

impl<const W: usize, const H: usize> Konane256<W, H> {
    /// x => white, o => black, _ => empty
    pub fn must_parse(s: &str) -> Self {
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
                    _ => panic!("invalid tile character: {:?}", c),
                }
            }
        }

        game
    }

    pub fn empty() -> Self {
        Self {
            white: BitBoard256::new(),
            black: BitBoard256::new(),
        }
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

    pub fn set_tile(&mut self, x: usize, y: usize, state: TileState) {
        assert!(x < W);
        assert!(y < H);

        match state {
            TileState::White => {
                self.white.set(x, y, true);
                self.black.set(x, y, false);
            }
            TileState::Black => {
                self.black.set(x, y, true);
                self.white.set(x, y, false);
            }
            TileState::Empty => {
                self.black.set(x, y, false);
                self.white.set(x, y, false);
            }
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> TileState {
        match (self.black.get(x, y), self.white.get(x, y)) {
            (true, true) => panic!("Tile at <{}, {}> is marked for both black and white", x, y),
            (false, false) => TileState::Empty,
            (true, false) => TileState::Black,
            (false, true) => TileState::White,
        }
    }

    pub fn empty_spaces(&self) -> BitArray<4, u64> {
        // get empty by selecting non-black spaces that don't have a white piece.
        !self.black.board.clone() ^ &self.white.board
    }

    pub fn move_generator_white<'a, Dir: ConstDirection>(
        &'a self,
        _: Dir,
    ) -> MoveGenerator<'a, Dir, W, H, true> {
        MoveGenerator::new(self)
    }

    pub fn move_generator_black<'a, Dir: ConstDirection>(
        &'a self,
        _: Dir,
    ) -> MoveGenerator<'a, Dir, W, H, false> {
        MoveGenerator::new(self)
    }

    pub fn move_generator<'a, const IS_WHITE: bool, Dir: ConstDirection>(
        &'a self,
        _: Dir,
    ) -> MoveGenerator<'a, Dir, W, H, IS_WHITE> {
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
> {
    game: &'a Konane256<W, H>,
    moves: BitBoard256<W, H>,
    hops: usize,
    _dir: PhantomData<Dir>,
}

impl<'a, Dir: ConstDirection, const W: usize, const H: usize, const IS_WHITE: bool>
    MoveGenerator<'a, Dir, W, H, IS_WHITE>
{
    pub const fn direction() -> Direction {
        Dir::VALUE
    }

    pub fn is_complete(&self) -> bool {
        self.moves.board.is_empty()
    }

    pub fn new(game: &'a Konane256<W, H>) -> Self {
        let mut moves: BitBoard256<W, H> = BitBoard256::border_mask(Self::direction());
        moves.board = !moves.board;
        moves.board &= if IS_WHITE {
            &game.white.board
        } else {
            &game.black.board
        };
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
            Direction::Right => self.moves.board <<= 1,
            Direction::Left => self.moves.board >>= 1,
            Direction::Up => self.moves.board >>= W,
            Direction::Down => self.moves.board <<= W,
        }
    }

    pub fn advance(&mut self) {
        if self.moves.board.is_empty() {
            return;
        }

        // 1. verify that there's a capture-able adjacent piece
        self.shift();
        if IS_WHITE {
            self.moves.board &= &self.game.black.board;
        } else {
            self.moves.board &= &self.game.white.board;
        }

        // 2. verify there's an empty space after the piece to be jumped
        self.shift();
        self.game.empty_spaces();
        self.moves.board &= self.game.empty_spaces();

        self.hops += 1;
    }

    pub fn move_iter(&'a self) -> impl Iterator<Item = Move<'a, Dir, W, H, IS_WHITE>> {
        self.moves.board.iter_set().map(|index| {
            assert!(index < 256);
            let index_u8 = index as u8;
            Move {
                move_to: index_u8,
                hops: self.hops as u8,
                _generator: PhantomData,
            }
        })
    }
}

// TODO: remove usel lifetime
#[derive(Debug, Clone, Copy)]
pub struct Move<'a, Dir: ConstDirection, const W: usize, const H: usize, const IS_WHITE: bool> {
    move_to: u8,
    hops: u8,
    _generator: PhantomData<MoveGenerator<'a, Dir, W, H, IS_WHITE>>,
}

impl<'a, Dir: ConstDirection, const W: usize, const H: usize, const IS_WHITE: bool>
    Move<'a, Dir, W, H, IS_WHITE>
{
    pub fn apply(&self, mut game: Konane256<W, H>) -> Konane256<W, H> {
        let step = Dir::VALUE.x() + Dir::VALUE.y() * W as isize;
        let itarget = self.move_to as isize;
        let original_pos = itarget + 2 * step * self.hops as isize;

        let (current_board, opponent) = if IS_WHITE {
            (&mut game.white.board, &mut game.black.board)
        } else {
            (&mut game.black.board, &mut game.white.board)
        };
        current_board.set(itarget as usize);
        current_board.clear(original_pos as usize);

        for i in 0..self.hops as isize {
            opponent.clear((itarget + step + 2 * step * i) as usize);
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
        dbg!(board.empty_spaces());
        dbg!(&board.white);
        dbg!(&board.black);

        assert_eq!(board.all_moves_black(), vec![]);
        assert_eq!(board.all_moves_white(), vec![]);
    }

    #[test]
    pub fn moveset_white_right_jump() {
        let board: Konane256 = Konane256::must_parse("xo");
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
