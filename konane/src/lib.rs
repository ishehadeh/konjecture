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

    pub fn moveset(&self, is_white: bool, dir: Direction) -> BitBoard256<W, H> {
        let empty = self.empty_spaces();
        let mut collected_moves = BitBoard256::new();

        let mut current_moves: BitBoard256<W, H> = BitBoard256::border_mask(dir);
        current_moves.board = !current_moves.board;
        current_moves.board &= if is_white {
            &self.white.board
        } else {
            &self.black.board
        };

        while !current_moves.board.is_empty() {
            // 1. verify that there's a capture-able adjacent piece
            match dir {
                Direction::Right => current_moves.board <<= 1,
                Direction::Left => current_moves.board >>= 1,
                Direction::Up => current_moves.board >>= W,
                Direction::Down => current_moves.board <<= W,
            }
            if is_white {
                current_moves.board &= &self.black.board;
            } else {
                current_moves.board &= &self.white.board;
            }

            // 2. verify there's an empty space after the piece to be jumped
            match dir {
                Direction::Right => current_moves.board <<= 1,
                Direction::Left => current_moves.board >>= 1,
                Direction::Up => current_moves.board >>= W,
                Direction::Down => current_moves.board <<= W,
            }
            current_moves.board &= &empty;
            collected_moves.board |= &current_moves.board;
        }

        collected_moves
    }
}

#[cfg(test)]
mod test {
    use crate::{
        bitboard::{BitBoard256, Direction},
        Konane256,
    };

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
        let w_r = board.moveset(true, Direction::Right);
        let b_r = board.moveset(false, Direction::Right);
        let w_l = board.moveset(true, Direction::Left);
        let b_l = board.moveset(false, Direction::Left);
        let w_u = board.moveset(true, Direction::Up);
        let b_u = board.moveset(false, Direction::Up);
        let w_d = board.moveset(true, Direction::Down);
        let b_d = board.moveset(false, Direction::Down);

        assert_eq!(w_r, BitBoard256::new());
        assert_eq!(b_r, BitBoard256::new());
        assert_eq!(w_l, BitBoard256::new());
        assert_eq!(b_l, BitBoard256::new());
        assert_eq!(w_u, BitBoard256::new());
        assert_eq!(b_u, BitBoard256::new());
        assert_eq!(w_d, BitBoard256::new());
        assert_eq!(b_d, BitBoard256::new());
    }

    #[test]
    pub fn moveset_white_right_jump() {
        use crate::TileState::{Black, White};

        let board: Konane256 = Konane256::small_at((0, 0), (1, 2), &[White, Black]);
        let mut expected_w_r = BitBoard256::new();
        expected_w_r.set(2, 0, true);
        let w_r = board.moveset(true, Direction::Right);
        let b_r = board.moveset(false, Direction::Right);
        let w_l = board.moveset(true, Direction::Left);
        let b_l = board.moveset(false, Direction::Left);
        let w_u = board.moveset(true, Direction::Up);
        let b_u = board.moveset(false, Direction::Up);
        let w_d = board.moveset(true, Direction::Down);
        let b_d = board.moveset(false, Direction::Down);

        assert_eq!(w_r, expected_w_r);
        assert_eq!(b_r, BitBoard256::new());
        assert_eq!(w_l, BitBoard256::new());
        assert_eq!(b_l, BitBoard256::new());
        assert_eq!(w_u, BitBoard256::new());
        assert_eq!(b_u, BitBoard256::new());
        assert_eq!(w_d, BitBoard256::new());
        assert_eq!(b_d, BitBoard256::new());
    }

    #[test]
    pub fn moveset_black_right_jump() {
        use crate::TileState::{Black, White};

        let board: Konane256 = Konane256::small_at((0, 0), (1, 2), &[Black, White]);
        let mut expected_b_r = BitBoard256::new();
        expected_b_r.set(2, 0, true);
        let w_r = board.moveset(true, Direction::Right);
        let b_r = board.moveset(false, Direction::Right);
        let w_l = board.moveset(true, Direction::Left);
        let b_l = board.moveset(false, Direction::Left);
        let w_u = board.moveset(true, Direction::Up);
        let b_u = board.moveset(false, Direction::Up);
        let w_d = board.moveset(true, Direction::Down);
        let b_d = board.moveset(false, Direction::Down);

        assert_eq!(w_r, BitBoard256::new());
        assert_eq!(b_r, expected_b_r);
        assert_eq!(w_l, BitBoard256::new());
        assert_eq!(b_l, BitBoard256::new());
        assert_eq!(w_u, BitBoard256::new());
        assert_eq!(b_u, BitBoard256::new());
        assert_eq!(w_d, BitBoard256::new());
        assert_eq!(b_d, BitBoard256::new());
    }

    #[test]
    pub fn moveset_white_left_jump() {
        use crate::TileState::{Black, White};

        let board: Konane256 = Konane256::small_at((1, 0), (1, 3), &[Black, White, White]);
        let mut expected_w_l = BitBoard256::new();
        expected_w_l.set(0, 0, true);
        let w_r = board.moveset(true, Direction::Right);
        let b_r = board.moveset(false, Direction::Right);
        let w_l = board.moveset(true, Direction::Left);
        let b_l = board.moveset(false, Direction::Left);
        let w_u = board.moveset(true, Direction::Up);
        let b_u = board.moveset(false, Direction::Up);
        let w_d = board.moveset(true, Direction::Down);
        let b_d = board.moveset(false, Direction::Down);

        assert_eq!(w_r, BitBoard256::new());
        assert_eq!(b_r, BitBoard256::new());
        assert_eq!(w_l, expected_w_l);
        assert_eq!(b_l, BitBoard256::new());
        assert_eq!(w_u, BitBoard256::new());
        assert_eq!(b_u, BitBoard256::new());
        assert_eq!(w_d, BitBoard256::new());
        assert_eq!(b_d, BitBoard256::new());
    }

    #[test]
    pub fn moveset_black_left_jump() {
        use crate::TileState::{Black, White};

        let board: Konane256 = Konane256::small_at((1, 0), (1, 3), &[White, Black, Black]);
        let mut expected_b_l = BitBoard256::new();
        expected_b_l.set(0, 0, true);
        let w_r = board.moveset(true, Direction::Right);
        let b_r = board.moveset(false, Direction::Right);
        let w_l = board.moveset(true, Direction::Left);
        let b_l = board.moveset(false, Direction::Left);
        let w_u = board.moveset(true, Direction::Up);
        let b_u = board.moveset(false, Direction::Up);
        let w_d = board.moveset(true, Direction::Down);
        let b_d = board.moveset(false, Direction::Down);

        assert_eq!(w_r, BitBoard256::new());
        assert_eq!(b_r, BitBoard256::new());
        assert_eq!(w_l, BitBoard256::new());
        assert_eq!(b_l, expected_b_l);
        assert_eq!(w_u, BitBoard256::new());
        assert_eq!(b_u, BitBoard256::new());
        assert_eq!(w_d, BitBoard256::new());
        assert_eq!(b_d, BitBoard256::new());
    }

    #[test]
    pub fn moveset_white_up_jump() {
        use crate::TileState::{Black, White};

        let board: Konane256 = Konane256::small_at((0, 1), (3, 1), &[Black, White, White]);
        let mut expected_w_u = BitBoard256::new();
        expected_w_u.set(0, 0, true);
        let w_r = board.moveset(true, Direction::Right);
        let b_r = board.moveset(false, Direction::Right);
        let w_l = board.moveset(true, Direction::Left);
        let b_l = board.moveset(false, Direction::Left);
        let w_u = board.moveset(true, Direction::Up);
        let b_u = board.moveset(false, Direction::Up);
        let w_d = board.moveset(true, Direction::Down);
        let b_d = board.moveset(false, Direction::Down);

        assert_eq!(w_r, BitBoard256::new());
        assert_eq!(b_r, BitBoard256::new());
        assert_eq!(w_l, BitBoard256::new());
        assert_eq!(b_l, BitBoard256::new());
        assert_eq!(w_u, expected_w_u);
        assert_eq!(b_u, BitBoard256::new());
        assert_eq!(w_d, BitBoard256::new());
        assert_eq!(b_d, BitBoard256::new());
    }

    #[test]
    pub fn moveset_black_up_jump() {
        use crate::TileState::{Black, White};

        let board: Konane256 = Konane256::small_at((0, 1), (3, 1), &[White, Black, Black]);
        let mut expected_b_u = BitBoard256::new();
        expected_b_u.set(0, 0, true);
        let w_r = board.moveset(true, Direction::Right);
        let b_r = board.moveset(false, Direction::Right);
        let w_l = board.moveset(true, Direction::Left);
        let b_l = board.moveset(false, Direction::Left);
        let w_u = board.moveset(true, Direction::Up);
        let b_u = board.moveset(false, Direction::Up);
        let w_d = board.moveset(true, Direction::Down);
        let b_d = board.moveset(false, Direction::Down);

        assert_eq!(w_r, BitBoard256::new());
        assert_eq!(b_r, BitBoard256::new());
        assert_eq!(w_l, BitBoard256::new());
        assert_eq!(b_l, BitBoard256::new());
        assert_eq!(w_u, BitBoard256::new());
        assert_eq!(b_u, expected_b_u);
        assert_eq!(w_d, BitBoard256::new());
        assert_eq!(b_d, BitBoard256::new());
    }

    #[test]
    pub fn moveset_white_down_jump() {
        use crate::TileState::{Black, White};

        let board: Konane256 = Konane256::small_at((0, 0), (2, 1), &[White, Black]);
        let mut expected_w_d = BitBoard256::new();
        expected_w_d.set(0, 2, true);
        let w_r = board.moveset(true, Direction::Right);
        let b_r = board.moveset(false, Direction::Right);
        let w_l = board.moveset(true, Direction::Left);
        let b_l = board.moveset(false, Direction::Left);
        let w_u = board.moveset(true, Direction::Up);
        let b_u = board.moveset(false, Direction::Up);
        let w_d = board.moveset(true, Direction::Down);
        let b_d = board.moveset(false, Direction::Down);

        assert_eq!(w_r, BitBoard256::new());
        assert_eq!(b_r, BitBoard256::new());
        assert_eq!(w_l, BitBoard256::new());
        assert_eq!(b_l, BitBoard256::new());
        assert_eq!(w_u, BitBoard256::new());
        assert_eq!(b_u, BitBoard256::new());
        assert_eq!(w_d, expected_w_d);
        assert_eq!(b_d, BitBoard256::new());
    }

    #[test]
    pub fn moveset_black_down_jump() {
        use crate::TileState::{Black, White};

        let board: Konane256 = Konane256::small_at((0, 0), (2, 1), &[Black, White]);
        let mut expected_b_d = BitBoard256::new();
        expected_b_d.set(0, 2, true);
        let w_r = board.moveset(true, Direction::Right);
        let b_r = board.moveset(false, Direction::Right);
        let w_l = board.moveset(true, Direction::Left);
        let b_l = board.moveset(false, Direction::Left);
        let w_u = board.moveset(true, Direction::Up);
        let b_u = board.moveset(false, Direction::Up);
        let w_d = board.moveset(true, Direction::Down);
        let b_d = board.moveset(false, Direction::Down);

        assert_eq!(w_r, BitBoard256::new());
        assert_eq!(b_r, BitBoard256::new());
        assert_eq!(w_l, BitBoard256::new());
        assert_eq!(b_l, BitBoard256::new());
        assert_eq!(w_u, BitBoard256::new());
        assert_eq!(b_u, BitBoard256::new());
        assert_eq!(w_d, BitBoard256::new());
        assert_eq!(b_d, expected_b_d);
    }
}
