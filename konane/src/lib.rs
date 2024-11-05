pub mod bitboard;

use bitarray::BitArray;
use bitboard::{BitBoard256, Direction};

pub struct Konane18x18 {
    pub white: BitArray<6, u64>,
    pub black: BitArray<6, u64>,
}

pub struct Konane256<const W: usize = 16, const H: usize = 16> {
    pub white: BitBoard256<W, H>,
    pub black: BitBoard256<W, H>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TileState {
    White,
    Black,
    Empty,
}

impl<const W: usize, const H: usize> Konane256<W, H> {
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
                    if (x + y * 18) % 2 == 0 {
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
        assert!(x < 18);
        assert!(y < 18);

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

    pub fn get_tile(&mut self, x: usize, y: usize) -> TileState {
        match (self.black.get(x, y), self.white.get(x, y)) {
            (true, true) => panic!("Tile at <{}, {}> is marked for both black and white", x, y),
            (false, false) => TileState::Empty,
            (true, false) => TileState::Black,
            (false, true) => TileState::White,
        }
    }

    pub fn empty_spaces(&self) -> BitArray<4, u64> {
        // get empty by selecting non-black spaces that don't have a white piece.
        !self.black.board.clone() & !self.white.board.clone()
    }

    pub fn white_moves_right(&self) -> MoveIter<'_, true, true, W, H> {
        MoveIter::new_white(self)
    }

    pub fn black_moves_right(&self) -> MoveIter<'_, false, true, W, H> {
        MoveIter::new_black(self)
    }

    pub fn white_moves_left(&self) -> MoveIter<'_, true, false, W, H> {
        MoveIter::new_white(self)
    }

    pub fn black_moves_left(&self) -> MoveIter<'_, false, false, W, H> {
        MoveIter::new_black(self)
    }

    /// Get a bitboard with only the last tile in every row set to 1
    pub fn right_border_mask() -> BitArray<6, u64> {
        let mut mask = BitArray::new();
        for i in 1..18 {
            mask.set(i * 18 - 1);
        }

        mask
    }
}

pub struct MoveIter<'a, const IS_WHITE: bool, const RIGHT: bool, const W: usize, const H: usize> {
    board: &'a Konane256<W, H>,
    moveset: BitBoard256<W, H>,
}

impl<'a, const RIGHT: bool, const W: usize, const H: usize> MoveIter<'a, true, RIGHT, W, H> {
    pub fn new_white(board: &'a Konane256<W, H>) -> Self {
        let mut moveset = BitBoard256::border_mask(Direction::Right);
        moveset.board = !moveset.board;
        moveset.board &= &board.white.board;
        MoveIter { board, moveset }
    }
}

impl<'a, const RIGHT: bool, const W: usize, const H: usize> MoveIter<'a, false, RIGHT, W, H> {
    pub fn new_black(board: &'a Konane256<W, H>) -> Self {
        let mut moveset = BitBoard256::border_mask(Direction::Right);
        dbg!(&moveset);
        moveset.board = !moveset.board;
        moveset.board &= &board.black.board;
        MoveIter { board, moveset }
    }
}

impl<'a, const IS_WHITE: bool, const RIGHT: bool, const W: usize, const H: usize> Iterator
    for MoveIter<'a, IS_WHITE, RIGHT, W, H>
{
    type Item = BitBoard256<W, H>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.moveset.board.is_empty() {
            None
        } else {
            if RIGHT {
                self.moveset.board <<= 1;
            } else {
                self.moveset.board >>= 1;
            }

            // 1. verify that there's a capture-able piece to the right 1 space
            if IS_WHITE {
                self.moveset.board &= &self.board.black.board;
            } else {
                self.moveset.board &= &self.board.white.board;
            }

            // 2. verify there's an empty tile to the right 2 spaces
            if RIGHT {
                self.moveset.board <<= 1;
            } else {
                self.moveset.board >>= 1;
            }

            self.moveset.board &= self.board.empty_spaces();

            if !self.moveset.board.is_empty() {
                Some(self.moveset.clone())
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{bitboard::BitBoard256, Konane256};

    #[test]
    pub fn checkerboard_16x16() {
        let board: Konane256<16, 16> = Konane256::checkerboard();
        for i in 0..(16 * 16 - 1) {
            assert_eq!(board.black.board.get(i), board.white.board.get(i + 1));
        }
    }

    #[test]
    pub fn moveset_on_full_board_is_empty_16x16() {
        let board: Konane256<16, 16> = Konane256::checkerboard();
        dbg!(board.empty_spaces());
        dbg!(&board.white);
        dbg!(&board.black);
        let white = board.white_moves_right().collect::<Vec<_>>();
        let black = board.black_moves_right().collect::<Vec<_>>();

        assert_eq!(white, vec![]);
        assert_eq!(black, vec![]);
    }

    #[test]
    pub fn moveset_white_right_jump() {
        use crate::TileState::{Black, White};

        let board: Konane256 = Konane256::small_at((0, 0), (1, 2), &[White, Black]);
        let white = board.white_moves_right().collect::<Vec<_>>();
        let black = board.black_moves_right().collect::<Vec<_>>();

        let mut white_expected = BitBoard256::new();
        white_expected.set(2, 0, true);
        assert_eq!(white, vec![white_expected]);
        assert_eq!(black, vec![]);
    }

    #[test]
    pub fn moveset_black_right_jump() {
        use crate::TileState::{Black, White};

        let board: Konane256 = Konane256::small_at((0, 0), (1, 2), &[Black, White]);
        let white = board.white_moves_right().collect::<Vec<_>>();
        let black = board.black_moves_right().collect::<Vec<_>>();

        let mut black_expected = BitBoard256::new();
        black_expected.set(2, 0, true);
        assert_eq!(black, vec![black_expected]);
        assert_eq!(white, vec![]);
    }

    #[test]
    pub fn moveset_white_left_jump() {
        use crate::TileState::{Black, White};

        let board: Konane256 = Konane256::small_at((1, 0), (1, 2), &[Black, White]);
        let white = board.white_moves_left().collect::<Vec<_>>();
        let black = board.black_moves_left().collect::<Vec<_>>();

        let mut white_expected = BitBoard256::new();
        white_expected.set(0, 0, true);
        assert_eq!(white, vec![white_expected]);
        assert_eq!(black, vec![]);
    }

    #[test]
    pub fn moveset_black_left_jump() {
        use crate::TileState::{Black, White};

        let board: Konane256 = Konane256::small_at((1, 0), (1, 2), &[White, Black]);
        let white = board.white_moves_left().collect::<Vec<_>>();
        let black = board.black_moves_left().collect::<Vec<_>>();

        let mut black_expected = BitBoard256::new();
        black_expected.set(0, 0, true);
        assert_eq!(black, vec![black_expected]);
        assert_eq!(white, vec![]);
    }
}
