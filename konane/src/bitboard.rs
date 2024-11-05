use bitarray::BitArray;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, PartialEq, Eq)]
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

    /// rotate counter clockwise
    pub fn transpose(mut self) -> BitBoard256<H, W> {
        assert!(W == H);

        let mut block_size = W / 2;
        while block_size > 1 {
            let mut block_mask = BitArray::<4, u64>::new();

            block_mask.set_range(block_size..block_size * 2);
            self.board.detla_swap(block_mask, block_size * H);
            block_size /= 2;
        }

        let mut mask0: BitArray<4, u64> = BitArray::new();
        for i in 0..H {
            if i % 2 == 0 {
                mask0.set_range_step(1..W, 2);
            }
        }
        self.board.detla_swap(mask0.clone(), W - 1);

        if W / 2 == 1 {
            mask0 >>= 1;
            self.board.detla_swap(mask0, W);
        }

        BitBoard256 { board: self.board }
    }
}

impl<const W: usize, const H: usize> std::fmt::Debug for BitBoard256<W, H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "BitBoard256<{}, {}> {{", W, H)?;
        for y in 0..H {
            write!(f, "   ")?;
            for x in 0..W {
                if self.get(x, y) {
                    write!(f, " 0")?;
                } else {
                    write!(f, " 1")?;
                }
            }
            writeln!(f, "")?;
        }
        writeln!(f, "}}")
    }
}
