use bitarray::BitArray;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
        board.set_range(W * H..board.bits());
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
        self.board.get(x + y * H)
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        assert!(x < W);
        assert!(y < H);
        if value {
            self.board.set(x + y * H)
        } else {
            self.board.clear(x + y * H)
        }
    }
}
