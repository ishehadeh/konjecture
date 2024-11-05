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

    pub fn white_moves_right(&self) -> MoveIter<'_, true, 0, W, H> {
        MoveIter::new_white(self)
    }

    pub fn black_moves_right(&self) -> MoveIter<'_, false, 0, W, H> {
        MoveIter::new_black(self)
    }

    pub fn white_moves_left(&self) -> MoveIter<'_, true, 1, W, H> {
        MoveIter::new_white(self)
    }

    pub fn black_moves_left(&self) -> MoveIter<'_, false, 1, W, H> {
        MoveIter::new_black(self)
    }

    pub fn white_moves_up(&self) -> MoveIter<'_, true, 2, W, H> {
        MoveIter::new_white(self)
    }

    pub fn black_moves_up(&self) -> MoveIter<'_, false, 2, W, H> {
        MoveIter::new_black(self)
    }

    pub fn white_moves_down(&self) -> MoveIter<'_, true, 3, W, H> {
        MoveIter::new_white(self)
    }

    pub fn black_moves_down(&self) -> MoveIter<'_, false, 3, W, H> {
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

pub struct MoveIter<'a, const IS_WHITE: bool, const DIR: usize, const W: usize, const H: usize> {
    board: &'a Konane256<W, H>,
    moveset: BitBoard256<W, H>,
}
impl<'a, const IS_WHITE: bool, const DIR: usize, const W: usize, const H: usize>
    MoveIter<'a, IS_WHITE, DIR, W, H>
{
    fn step(&mut self) {
        match DIR {
            // right
            0 => self.moveset.board <<= 1,
            // left
            1 => self.moveset.board >>= 1,
            // up
            2 => self.moveset.board >>= W,
            // down
            3 => self.moveset.board <<= W,
            _ => panic!("invalid direction"),
        }
    }

    pub fn direction() -> Direction {
        match DIR {
            // right
            0 => Direction::Right,
            // left
            1 => Direction::Left,
            // up
            2 => Direction::Up,
            // down
            3 => Direction::Down,
            _ => panic!("invalid direction"),
        }
    }
}

impl<'a, const DIR: usize, const W: usize, const H: usize> MoveIter<'a, true, DIR, W, H> {
    pub fn new_white(board: &'a Konane256<W, H>) -> Self {
        let mut moveset = BitBoard256::border_mask(Self::direction());
        moveset.board = !moveset.board;
        moveset.board &= &board.white.board;
        MoveIter { board, moveset }
    }
}

impl<'a, const DIR: usize, const W: usize, const H: usize> MoveIter<'a, false, DIR, W, H> {
    pub fn new_black(board: &'a Konane256<W, H>) -> Self {
        let mut moveset = BitBoard256::border_mask(Self::direction());
        moveset.board = !moveset.board;
        moveset.board &= &board.black.board;
        MoveIter { board, moveset }
    }
}

impl<'a, const IS_WHITE: bool, const DIR: usize, const W: usize, const H: usize> Iterator
    for MoveIter<'a, IS_WHITE, DIR, W, H>
{
    type Item = BitBoard256<W, H>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.moveset.board.is_empty() {
            None
        } else {
            // 1. verify that there's a capture-able adjacent piece
            self.step();
            if IS_WHITE {
                self.moveset.board &= &self.board.black.board;
            } else {
                self.moveset.board &= &self.board.white.board;
            }

            // 2. verify there's an empty tile to the right 2 spaces
            self.step();

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
        let w_r = board.white_moves_right().collect::<Vec<_>>();
        let b_r = board.black_moves_right().collect::<Vec<_>>();
        let w_l = board.white_moves_left().collect::<Vec<_>>();
        let b_l = board.black_moves_left().collect::<Vec<_>>();
        let w_u = board.white_moves_up().collect::<Vec<_>>();
        let b_u = board.black_moves_up().collect::<Vec<_>>();
        let w_d = board.white_moves_down().collect::<Vec<_>>();
        let b_d = board.black_moves_down().collect::<Vec<_>>();

        assert_eq!(w_r, vec![]);
        assert_eq!(b_r, vec![]);
        assert_eq!(w_l, vec![]);
        assert_eq!(b_l, vec![]);
        assert_eq!(w_u, vec![]);
        assert_eq!(b_u, vec![]);
        assert_eq!(w_d, vec![]);
        assert_eq!(b_d, vec![]);
    }

    #[test]
    pub fn moveset_white_right_jump() {
        use crate::TileState::{Black, White};

        let board: Konane256 = Konane256::small_at((0, 0), (1, 2), &[White, Black]);
        let mut expected_w_r = BitBoard256::new();
        expected_w_r.set(2, 0, true);
        let w_r = board.white_moves_right().collect::<Vec<_>>();
        let b_r = board.black_moves_right().collect::<Vec<_>>();
        let w_l = board.white_moves_left().collect::<Vec<_>>();
        let b_l = board.black_moves_left().collect::<Vec<_>>();
        let w_u = board.white_moves_up().collect::<Vec<_>>();
        let b_u = board.black_moves_up().collect::<Vec<_>>();
        let w_d = board.white_moves_down().collect::<Vec<_>>();
        let b_d = board.black_moves_down().collect::<Vec<_>>();

        assert_eq!(w_r, vec![expected_w_r]);
        assert_eq!(b_r, vec![]);
        assert_eq!(w_l, vec![]);
        assert_eq!(b_l, vec![]);
        assert_eq!(w_u, vec![]);
        assert_eq!(b_u, vec![]);
        assert_eq!(w_d, vec![]);
        assert_eq!(b_d, vec![]);
    }

    #[test]
    pub fn moveset_black_right_jump() {
        use crate::TileState::{Black, White};

        let board: Konane256 = Konane256::small_at((0, 0), (1, 2), &[Black, White]);
        let mut expected_b_r = BitBoard256::new();
        expected_b_r.set(2, 0, true);
        let w_r = board.white_moves_right().collect::<Vec<_>>();
        let b_r = board.black_moves_right().collect::<Vec<_>>();
        let w_l = board.white_moves_left().collect::<Vec<_>>();
        let b_l = board.black_moves_left().collect::<Vec<_>>();
        let w_u = board.white_moves_up().collect::<Vec<_>>();
        let b_u = board.black_moves_up().collect::<Vec<_>>();
        let w_d = board.white_moves_down().collect::<Vec<_>>();
        let b_d = board.black_moves_down().collect::<Vec<_>>();

        assert_eq!(w_r, vec![]);
        assert_eq!(b_r, vec![expected_b_r]);
        assert_eq!(w_l, vec![]);
        assert_eq!(b_l, vec![]);
        assert_eq!(w_u, vec![]);
        assert_eq!(b_u, vec![]);
        assert_eq!(w_d, vec![]);
        assert_eq!(b_d, vec![]);
    }

    #[test]
    pub fn moveset_white_left_jump() {
        use crate::TileState::{Black, White};

        let board: Konane256 = Konane256::small_at((1, 0), (1, 3), &[Black, White, White]);
        let mut expected_w_l = BitBoard256::new();
        expected_w_l.set(0, 0, true);
        let w_r = board.white_moves_right().collect::<Vec<_>>();
        let b_r = board.black_moves_right().collect::<Vec<_>>();
        let w_l = board.white_moves_left().collect::<Vec<_>>();
        let b_l = board.black_moves_left().collect::<Vec<_>>();
        let w_u = board.white_moves_up().collect::<Vec<_>>();
        let b_u = board.black_moves_up().collect::<Vec<_>>();
        let w_d = board.white_moves_down().collect::<Vec<_>>();
        let b_d = board.black_moves_down().collect::<Vec<_>>();

        assert_eq!(w_r, vec![]);
        assert_eq!(b_r, vec![]);
        assert_eq!(w_l, vec![expected_w_l]);
        assert_eq!(b_l, vec![]);
        assert_eq!(w_u, vec![]);
        assert_eq!(b_u, vec![]);
        assert_eq!(w_d, vec![]);
        assert_eq!(b_d, vec![]);
    }

    #[test]
    pub fn moveset_black_left_jump() {
        use crate::TileState::{Black, White};

        let board: Konane256 = Konane256::small_at((1, 0), (1, 3), &[White, Black, Black]);
        let mut expected_b_l = BitBoard256::new();
        expected_b_l.set(0, 0, true);
        let w_r = board.white_moves_right().collect::<Vec<_>>();
        let b_r = board.black_moves_right().collect::<Vec<_>>();
        let w_l = board.white_moves_left().collect::<Vec<_>>();
        let b_l = board.black_moves_left().collect::<Vec<_>>();
        let w_u = board.white_moves_up().collect::<Vec<_>>();
        let b_u = board.black_moves_up().collect::<Vec<_>>();
        let w_d = board.white_moves_down().collect::<Vec<_>>();
        let b_d = board.black_moves_down().collect::<Vec<_>>();

        assert_eq!(w_r, vec![]);
        assert_eq!(b_r, vec![]);
        assert_eq!(w_l, vec![]);
        assert_eq!(b_l, vec![expected_b_l]);
        assert_eq!(w_u, vec![]);
        assert_eq!(b_u, vec![]);
        assert_eq!(w_d, vec![]);
        assert_eq!(b_d, vec![]);
    }

    #[test]
    pub fn moveset_white_up_jump() {
        use crate::TileState::{Black, White};

        let board: Konane256 = Konane256::small_at((0, 1), (3, 1), &[Black, White, White]);
        let mut expected_w_u = BitBoard256::new();
        expected_w_u.set(0, 0, true);
        let w_r = board.white_moves_right().collect::<Vec<_>>();
        let b_r = board.black_moves_right().collect::<Vec<_>>();
        let w_l = board.white_moves_left().collect::<Vec<_>>();
        let b_l = board.black_moves_left().collect::<Vec<_>>();
        let w_u = board.white_moves_up().collect::<Vec<_>>();
        let b_u = board.black_moves_up().collect::<Vec<_>>();
        let w_d = board.white_moves_down().collect::<Vec<_>>();
        let b_d = board.black_moves_down().collect::<Vec<_>>();

        assert_eq!(w_r, vec![]);
        assert_eq!(b_r, vec![]);
        assert_eq!(w_l, vec![]);
        assert_eq!(b_l, vec![]);
        assert_eq!(w_u, vec![expected_w_u]);
        assert_eq!(b_u, vec![]);
        assert_eq!(w_d, vec![]);
        assert_eq!(b_d, vec![]);
    }

    #[test]
    pub fn moveset_black_up_jump() {
        use crate::TileState::{Black, White};

        let board: Konane256 = Konane256::small_at((0, 1), (3, 1), &[White, Black, Black]);
        let mut expected_b_u = BitBoard256::new();
        expected_b_u.set(0, 0, true);
        let w_r = board.white_moves_right().collect::<Vec<_>>();
        let b_r = board.black_moves_right().collect::<Vec<_>>();
        let w_l = board.white_moves_left().collect::<Vec<_>>();
        let b_l = board.black_moves_left().collect::<Vec<_>>();
        let w_u = board.white_moves_up().collect::<Vec<_>>();
        let b_u = board.black_moves_up().collect::<Vec<_>>();
        let w_d = board.white_moves_down().collect::<Vec<_>>();
        let b_d = board.black_moves_down().collect::<Vec<_>>();

        assert_eq!(w_r, vec![]);
        assert_eq!(b_r, vec![]);
        assert_eq!(w_l, vec![]);
        assert_eq!(b_l, vec![]);
        assert_eq!(w_u, vec![]);
        assert_eq!(b_u, vec![expected_b_u]);
        assert_eq!(w_d, vec![]);
        assert_eq!(b_d, vec![]);
    }

    #[test]
    pub fn moveset_white_down_jump() {
        use crate::TileState::{Black, White};

        let board: Konane256 = Konane256::small_at((0, 0), (2, 1), &[White, Black]);
        let mut expected_w_d = BitBoard256::new();
        expected_w_d.set(0, 2, true);
        let w_r = board.white_moves_right().collect::<Vec<_>>();
        let b_r = board.black_moves_right().collect::<Vec<_>>();
        let w_l = board.white_moves_left().collect::<Vec<_>>();
        let b_l = board.black_moves_left().collect::<Vec<_>>();
        let w_u = board.white_moves_up().collect::<Vec<_>>();
        let b_u = board.black_moves_up().collect::<Vec<_>>();
        let w_d = board.white_moves_down().collect::<Vec<_>>();
        let b_d = board.black_moves_down().collect::<Vec<_>>();

        assert_eq!(w_r, vec![]);
        assert_eq!(b_r, vec![]);
        assert_eq!(w_l, vec![]);
        assert_eq!(b_l, vec![]);
        assert_eq!(w_u, vec![]);
        assert_eq!(b_u, vec![]);
        assert_eq!(w_d, vec![expected_w_d]);
        assert_eq!(b_d, vec![]);
    }

    #[test]
    pub fn moveset_black_down_jump() {
        use crate::TileState::{Black, White};

        let board: Konane256 = Konane256::small_at((0, 0), (2, 1), &[Black, White]);
        let mut expected_b_d = BitBoard256::new();
        expected_b_d.set(0, 2, true);
        let w_r = board.white_moves_right().collect::<Vec<_>>();
        let b_r = board.black_moves_right().collect::<Vec<_>>();
        let w_l = board.white_moves_left().collect::<Vec<_>>();
        let b_l = board.black_moves_left().collect::<Vec<_>>();
        let w_u = board.white_moves_up().collect::<Vec<_>>();
        let b_u = board.black_moves_up().collect::<Vec<_>>();
        let w_d = board.white_moves_down().collect::<Vec<_>>();
        let b_d = board.black_moves_down().collect::<Vec<_>>();

        assert_eq!(w_r, vec![]);
        assert_eq!(b_r, vec![]);
        assert_eq!(w_l, vec![]);
        assert_eq!(b_l, vec![]);
        assert_eq!(w_u, vec![]);
        assert_eq!(b_u, vec![]);
        assert_eq!(w_d, vec![]);
        assert_eq!(b_d, vec![expected_b_d]);
    }
}
