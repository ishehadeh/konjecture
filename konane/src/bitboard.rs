use bitarray::BitArray;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn all() -> [Self; 4] {
        [Self::Up, Self::Down, Self::Left, Self::Right]
    }

    pub fn x(&self) -> isize {
        match self {
            Direction::Up => 0,
            Direction::Down => 0,
            Direction::Left => 1,
            Direction::Right => -1,
        }
    }

    pub fn y(&self) -> isize {
        match self {
            Direction::Up => 1,
            Direction::Down => -1,
            Direction::Left => 0,
            Direction::Right => 0,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BitBoard256<const W: usize, const H: usize> {
    pub board: BitArray<4, u64>,
}

impl<const W: usize, const H: usize> Default for BitBoard256<W, H> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const W: usize, const H: usize> BitBoard256<W, H> {
    pub fn new() -> Self {
        assert!(W * H <= 256);
        assert!(W > 0);
        assert!(H > 0);
        let mut board = BitArray::new();
        // board.set_range(W * H..board.bits());
        Self { board }
    }

    pub fn border_mask(dir: Direction) -> Self {
        let mut base = Self::new();
        match dir {
            Direction::Up => base.board.set_range(0..W),
            Direction::Down => base.board.set_range(W * (H - 1)..W * H),
            Direction::Right => base.board.set_range_step((W - 1)..W * H, W),
            Direction::Left => base.board.set_range_step(0..W * H, W),
        }

        base
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        assert!(x < W);
        assert!(y < H);
        self.board.get(x + y * W)
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        assert!(x < W);
        assert!(y < H);
        if value {
            self.board.set(x + y * W)
        } else {
            self.board.clear(x + y * W)
        }
    }
}

impl<const W: usize, const H: usize> std::fmt::Debug for BitBoard256<W, H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "BitBoard256<{}, {}> {{", W, H)?;
        for y in 0..H {
            write!(f, "   ")?;
            for x in 0..W {
                if self.get(x, y) {
                    write!(f, " 1")?;
                } else {
                    write!(f, " 0")?;
                }
            }
            writeln!(f, "")?;
        }
        writeln!(f, "}}")
    }
}

#[cfg(test)]
mod test {
    use super::BitBoard256;

    #[test]
    pub fn set_get_clear_on_square_small() {
        let mut board = BitBoard256::<8, 8>::new();
        board.set(5, 2, true);
        assert!(board.board.get(8 * 2 + 5));
        assert!(board.get(5, 2));

        board.set(1, 4, true);
        assert!(board.board.get(8 * 4 + 1));
        assert!(board.get(1, 4));

        board.set(0, 2, true);
        assert!(board.board.get(8 * 2 + 0));
        assert!(board.get(0, 2));

        board.set(0, 0, true);
        assert!(board.board.get(0));
        assert!(board.get(0, 0));

        board.set(7, 7, true);
        assert!(board.board.get(8 * 8 - 1));
        assert!(board.get(7, 7));

        board.set(5, 2, false);
        board.set(0, 2, false);
        board.set(0, 0, false);
        board.set(7, 7, false);
        board.set(1, 4, false);
        assert!(board.board.is_empty());
    }

    #[test]
    pub fn set_get_clear_on_small_rectangle() {
        let mut board = BitBoard256::<3, 8>::new();
        board.set(1, 5, true);
        assert!(board.board.get(3 * 5 + 1));
        assert!(board.get(1, 5));

        board.set(0, 2, true);
        assert!(board.board.get(3 * 2 + 0));
        assert!(board.get(0, 2));

        board.set(0, 0, true);
        assert!(board.board.get(0));
        assert!(board.get(0, 0));

        board.set(2, 7, true);
        assert!(board.board.get(3 * 8 - 1));
        assert!(board.get(2, 7));

        board.set(1, 5, false);
        board.set(0, 2, false);
        board.set(0, 0, false);
        board.set(2, 7, false);
        assert!(board.board.is_empty());
    }

    #[test]
    pub fn set_get_clear_on_square_full() {
        let mut board = BitBoard256::<8, 8>::new();
        board.set(5, 2, true);
        assert!(board.board.get(8 * 2 + 5));
        assert!(board.get(5, 2));

        board.set(1, 4, true);
        assert!(board.board.get(8 * 4 + 1));
        assert!(board.get(1, 4));

        board.set(0, 2, true);
        assert!(board.board.get(8 * 2 + 0));
        assert!(board.get(0, 2));

        board.set(0, 0, true);
        assert!(board.board.get(0));
        assert!(board.get(0, 0));

        board.set(7, 7, true);
        assert!(board.board.get(8 * 8 - 1));
        assert!(board.get(7, 7));

        board.set(5, 2, false);
        board.set(1, 4, false);
        board.set(0, 2, false);
        board.set(0, 0, false);
        board.set(7, 7, false);
        assert!(board.board.is_empty());
    }

    #[test]
    pub fn set_get_clear_on_full_rectangle() {
        let mut board = BitBoard256::<64, 4>::new();
        board.set(50, 3, true);
        assert!(board.board.get(64 * 3 + 50));
        assert!(board.get(50, 3));

        board.set(0, 2, true);
        assert!(board.board.get(64 * 2 + 0));
        assert!(board.get(0, 2));

        board.set(0, 0, true);
        assert!(board.board.get(0));
        assert!(board.get(0, 0));

        board.set(63, 3, true);
        assert!(board.board.get(64 * 4 - 1));
        assert!(board.get(63, 3));

        board.set(50, 3, false);
        board.set(0, 2, false);
        board.set(0, 0, false);
        board.set(63, 3, false);
        assert!(board.board.is_empty());
    }
}
