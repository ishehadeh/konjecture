use bitarray::BitArray;

pub struct Konane18x18 {
    pub white: BitArray<6, u64>,
    pub black: BitArray<6, u64>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TileState {
    White,
    Black,
    Empty,
}

impl Konane18x18 {
    pub fn empty() -> Self {
        let mut row_end_sigils = BitArray::new();
        for i in 1..18 {
            row_end_sigils.set(i * 18);
        }
        row_end_sigils.set(17 * 18 + 1);

        Self {
            white: row_end_sigils.clone(),
            black: row_end_sigils,
        }
    }
    pub fn small((rows, cols): (usize, usize), tiles: &[TileState]) -> Self {
        let row_start = (18 - rows) / 2;
        let col_start = (18 - cols) / 2;
        Self::small_at((row_start, col_start), (rows, cols), tiles)
    }
    pub fn small_at(
        (x_start, y_start): (usize, usize),
        (rows, columns): (usize, usize),
        tiles: &[TileState],
    ) -> Self {
        assert!(x_start < 18);
        assert!(y_start < 18);

        let row_end = x_start + columns;
        let col_end = y_start + rows;

        assert!(col_end <= 18);
        assert!(row_end <= 18);

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

        for x in 0..18 {
            for y in 0..18 {
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

    fn bit_index_of(x: usize, y: usize) -> usize {
        // account for row end sigils
        x + 19 * y
    }

    pub fn set_tile(&mut self, x: usize, y: usize, state: TileState) {
        assert!(x < 18);
        assert!(y < 18);

        let i = Self::bit_index_of(x, y);
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

    pub fn get_tile(&mut self, x: usize, y: usize) -> TileState {
        assert!(x < 18);
        assert!(y < 18);

        let i = Self::bit_index_of(x, y);
        match (self.black.get(i), self.white.get(i)) {
            (true, true) => panic!("Tile at <{}, {}> is marked for both black and white", x, y),
            (false, false) => TileState::Empty,
            (true, false) => TileState::Black,
            (false, true) => TileState::White,
        }
    }

    pub fn empty_spaces(&self) -> BitArray<6, u64> {
        // get empty by selecting non-black spaces that don't have a white piece.
        !self.black.clone() & !self.white.clone()
    }

    pub fn white_moves_right(&self) -> MoveIter<'_, true> {
        MoveIter {
            board: &self,
            moveset: self.white.clone(),
        }
    }

    pub fn black_moves_right(&self) -> MoveIter<'_, false> {
        MoveIter {
            board: &self,
            moveset: self.black.clone(),
        }
    }
}

pub struct MoveIter<'a, const IS_WHITE: bool> {
    board: &'a Konane18x18,
    moveset: BitArray<6, u64>,
}

impl<'a, const IS_WHITE: bool> Iterator for MoveIter<'a, IS_WHITE> {
    type Item = BitArray<6, u64>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.moveset.is_empty() {
            None
        } else {
            self.moveset <<= 1;

            // 1. verify that there's a capture-able piece to the right 1 space
            if IS_WHITE {
                self.moveset &= &self.board.black;
            } else {
                self.moveset &= &self.board.white;
            }

            // 2. verify there's an empty piece to the right 2 spaces
            self.moveset <<= 1;
            self.moveset &= self.board.empty_spaces();

            if !self.moveset.is_empty() {
                Some(self.moveset.clone())
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod test {
    use bitarray::BitArray;

    use crate::Konane18x18;

    #[test]
    pub fn checkerboard() {
        let board = Konane18x18::checkerboard();
        dbg!(&board.white);
        dbg!(&board.black);
        for i in 0..(18 * 19 - 1) {
            if i % 18 == 0 && i > 0 {
                // row end sigil
                assert_eq!(board.black.get(i), true);
                assert_eq!(board.white.get(i), true);
            } else if (i + 1) % 18 == 0 && i > 0 {
                // last element of row differs from first of row below
                assert_eq!(board.black.get(i), board.white.get(i + 2), "i = {i}");
            } else {
                // alternating pattern in rows
                assert_eq!(board.black.get(i), board.white.get(i + 1));
            }
        }
    }

    #[test]
    pub fn moveset_on_full_board_is_empty() {
        let board = Konane18x18::checkerboard();
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

        let board = Konane18x18::small_at((0, 0), (1, 2), &[White, Black]);
        let white = board.white_moves_right().collect::<Vec<_>>();
        let black = board.black_moves_right().collect::<Vec<_>>();

        let mut white_expected = BitArray::new();
        white_expected.set(Konane18x18::bit_index_of(2, 0));
        assert_eq!(white, vec![white_expected]);
        assert_eq!(black, vec![]);
    }
}
