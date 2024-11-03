#[cfg(any(test, feature = "proptest"))]
mod arbitrary;
mod block;

use std::ops;

use block::BitArrayBlock;

#[derive(Clone, PartialEq, Eq)]
pub struct BitArray<const BLOCK_COUNT: usize, Block: BitArrayBlock = u64> {
    // blocks are stored in reverse order
    blocks: [Block; BLOCK_COUNT],
}

impl<const BLOCK_COUNT: usize, Block: BitArrayBlock> BitArray<BLOCK_COUNT, Block> {
    // #region Static Utility Functions
    const fn block_len() -> usize {
        Block::BLOCK_LENGTH
    }

    /// Get a (block-index, bit-within-block-index) from a bit index
    const fn addr(bit: usize) -> (usize, usize) {
        assert!(bit < Block::BLOCK_LENGTH * BLOCK_COUNT);

        // blocks are stored first to last
        let block = BLOCK_COUNT - bit / Self::block_len() - 1;
        let bit = bit % Self::block_len();
        (block, bit)
    }
    // #endregion

    pub fn new() -> Self {
        Self {
            blocks: [Block::empty(); BLOCK_COUNT],
        }
    }

    pub fn set(&mut self, bit: usize) {
        let (block, bit) = Self::addr(bit);
        self.blocks[block].set(bit);
    }

    pub fn clear(&mut self, bit: usize) {
        let (block, bit) = Self::addr(bit);
        self.blocks[block].clear(bit)
    }

    pub fn get(&self, bit: usize) -> bool {
        let (block, bit) = Self::addr(bit);
        self.blocks[block].get(bit)
    }

    pub fn first_set(&self) -> Option<usize> {
        for (block_i, block) in self.blocks.iter().rev().enumerate() {
            if let Some(bit_i) = block.first_set() {
                return Some(block_i * Self::block_len() + bit_i);
            }
        }

        None
    }

    pub fn last_set(&self) -> Option<usize> {
        for (block_i_inv, block) in self.blocks.iter().enumerate() {
            if let Some(bit_i) = block.first_set() {
                let block_i = self.blocks.len() - block_i_inv - 1;
                return Some(block_i * Self::block_len() + bit_i);
            }
        }

        None
    }

    pub fn bits(&self) -> usize {
        Block::BLOCK_LENGTH * BLOCK_COUNT
    }

    pub fn is_empty(&self) -> bool {
        self.blocks.iter().all(|&b| b == Block::empty())
    }
}

impl<const BLOCK_COUNT: usize, Block: BitArrayBlock> Default for BitArray<BLOCK_COUNT, Block> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const BLOCK_COUNT: usize, Block: BitArrayBlock> std::fmt::Debug
    for BitArray<BLOCK_COUNT, Block>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BitArray<{}, {}> [ ",
            BLOCK_COUNT,
            std::any::type_name::<Block>()
        )?;
        for block in self.blocks {
            write!(f, "{:0pad$b}", block, pad = Block::BLOCK_LENGTH)?;
        }

        Ok(())
    }
}

// #region Bit Ops
impl<const N: usize, B: BitArrayBlock> ops::BitAnd for BitArray<N, B> {
    type Output = Self;

    fn bitand(mut self, Self { blocks }: Self) -> Self::Output {
        self.blocks
            .iter_mut()
            .zip(blocks.into_iter())
            .for_each(|(lhs, rhs)| *lhs &= rhs);
        self
    }
}
impl<const N: usize, B: BitArrayBlock> ops::BitOr for BitArray<N, B> {
    type Output = Self;

    fn bitor(mut self, Self { blocks }: Self) -> Self::Output {
        self.blocks
            .iter_mut()
            .zip(blocks.into_iter())
            .for_each(|(lhs, rhs)| *lhs |= rhs);
        self
    }
}
impl<const N: usize, B: BitArrayBlock> ops::BitXor for BitArray<N, B> {
    type Output = Self;

    fn bitxor(mut self, Self { blocks }: Self) -> Self::Output {
        self.blocks
            .iter_mut()
            .zip(blocks.into_iter())
            .for_each(|(lhs, rhs)| *lhs ^= rhs);
        self
    }
}
// #endregion

// #region Bit Ops Assign
impl<const N: usize, B: BitArrayBlock> ops::BitAndAssign for BitArray<N, B> {
    fn bitand_assign(&mut self, Self { blocks }: Self) {
        self.blocks
            .iter_mut()
            .zip(blocks.into_iter())
            .for_each(|(lhs, rhs)| *lhs &= rhs);
    }
}
impl<const N: usize, B: BitArrayBlock> ops::BitOrAssign for BitArray<N, B> {
    fn bitor_assign(&mut self, Self { blocks }: Self) {
        self.blocks
            .iter_mut()
            .zip(blocks.into_iter())
            .for_each(|(lhs, rhs)| *lhs |= rhs);
    }
}
impl<const N: usize, B: BitArrayBlock> ops::BitXorAssign for BitArray<N, B> {
    fn bitxor_assign(&mut self, Self { blocks }: Self) {
        self.blocks
            .iter_mut()
            .zip(blocks.into_iter())
            .for_each(|(lhs, rhs)| *lhs ^= rhs);
    }
}
// #endregion

// #region Bit Ops Ref
impl<'a, const N: usize, B: BitArrayBlock> ops::BitAnd<&'a Self> for BitArray<N, B> {
    type Output = Self;

    fn bitand(mut self, Self { blocks }: &Self) -> Self::Output {
        self.blocks
            .iter_mut()
            .zip(blocks.iter())
            .for_each(|(lhs, rhs)| *lhs &= *rhs);
        self
    }
}
impl<'a, const N: usize, B: BitArrayBlock> ops::BitOr<&'a Self> for BitArray<N, B> {
    type Output = Self;

    fn bitor(mut self, Self { blocks }: &Self) -> Self::Output {
        self.blocks
            .iter_mut()
            .zip(blocks.iter())
            .for_each(|(lhs, rhs)| *lhs |= *rhs);
        self
    }
}
impl<'a, const N: usize, B: BitArrayBlock> ops::BitXor<&'a Self> for BitArray<N, B> {
    type Output = Self;

    fn bitxor(mut self, Self { blocks }: &Self) -> Self::Output {
        self.blocks
            .iter_mut()
            .zip(blocks.iter())
            .for_each(|(lhs, rhs)| *lhs ^= *rhs);
        self
    }
}
// #endregion

// #region Bit Ops Assign Ref
impl<'a, const N: usize, B: BitArrayBlock> ops::BitAndAssign<&'a Self> for BitArray<N, B> {
    fn bitand_assign(&mut self, Self { blocks }: &Self) {
        self.blocks
            .iter_mut()
            .zip(blocks.iter())
            .for_each(|(lhs, rhs)| *lhs &= *rhs);
    }
}
impl<'a, const N: usize, B: BitArrayBlock> ops::BitOrAssign<&'a Self> for BitArray<N, B> {
    fn bitor_assign(&mut self, Self { blocks }: &Self) {
        self.blocks
            .iter_mut()
            .zip(blocks.iter())
            .for_each(|(lhs, rhs)| *lhs |= *rhs);
    }
}
impl<'a, const N: usize, B: BitArrayBlock> ops::BitXorAssign<&'a Self> for BitArray<N, B> {
    fn bitxor_assign(&mut self, Self { blocks }: &Self) {
        self.blocks
            .iter_mut()
            .zip(blocks.iter())
            .for_each(|(lhs, rhs)| *lhs ^= *rhs);
    }
}
// #endregion

// #region Shift Ops Assign
impl<const BLOCK_COUNT: usize, Block: BitArrayBlock> ops::ShlAssign<usize>
    for BitArray<BLOCK_COUNT, Block>
{
    fn shl_assign(&mut self, n: usize) {
        if n == 0 {
            return;
        }

        let shift_overflow_mask = !(Block::all() >> n);
        let mut shift_overflow: [Block; BLOCK_COUNT] = [Block::empty(); BLOCK_COUNT];
        for i in 0..(BLOCK_COUNT - 1) {
            shift_overflow[i] = self.blocks[i + 1] & shift_overflow_mask;
        }

        for i in 0..BLOCK_COUNT {
            self.blocks[i] = self.blocks[i].shl(n);
        }

        for i in 0..(BLOCK_COUNT - 1) {
            self.blocks[i] |= shift_overflow[i] >> (Block::BLOCK_LENGTH - n)
        }
    }
}

impl<const BLOCK_COUNT: usize, Block: BitArrayBlock> ops::ShrAssign<usize>
    for BitArray<BLOCK_COUNT, Block>
{
    fn shr_assign(&mut self, n: usize) {
        if n == 0 {
            return;
        }

        let shift_overflow_mask = !(Block::all() << n);
        let mut shift_overflow: [Block; BLOCK_COUNT] = [Block::empty(); BLOCK_COUNT];
        for i in 1..BLOCK_COUNT {
            shift_overflow[i] = self.blocks[i - 1] & shift_overflow_mask;
        }

        for i in 0..BLOCK_COUNT {
            self.blocks[i] = self.blocks[i].shr(n);
        }

        for i in 1..BLOCK_COUNT {
            self.blocks[i] |= shift_overflow[i] << (Block::BLOCK_LENGTH - n)
        }
    }
}
// #endregion

// #region Shift Ops
impl<const N: usize, B: BitArrayBlock> ops::Shl<usize> for BitArray<N, B> {
    type Output = Self;

    fn shl(mut self, n: usize) -> Self::Output {
        ops::ShlAssign::shl_assign(&mut self, n);
        self
    }
}

impl<const N: usize, B: BitArrayBlock> ops::Shr<usize> for BitArray<N, B> {
    type Output = Self;

    fn shr(mut self, n: usize) -> Self::Output {
        ops::ShrAssign::shr_assign(&mut self, n);
        self
    }
}
// #endregion

impl<const N: usize, B: BitArrayBlock> ops::Not for BitArray<N, B> {
    type Output = Self;

    fn not(mut self) -> Self::Output {
        self.blocks.iter_mut().for_each(|b| *b = !(*b));
        self
    }
}

#[cfg(test)]
mod test {
    use proptest::{prelude::any, prop_assert, prop_assert_eq, proptest};

    use crate::BitArray;

    #[test]
    fn simple_set() {
        let mut b1_u8 = BitArray::<1, u8>::new();
        b1_u8.set(4);
        assert_eq!(b1_u8.blocks, [0b0001_0000]);

        let mut b2_u8 = BitArray::<2, u8>::new();
        b2_u8.set(11);
        assert_eq!(b2_u8.blocks, [0b0000_1000, 00000000]);
    }

    #[test]
    fn simple_clear() {
        let mut b1_u8 = BitArray {
            blocks: [0b00010001u8],
        };
        b1_u8.clear(4);
        assert_eq!(b1_u8.blocks, [0b00000001]);

        let mut b2_u8 = BitArray {
            blocks: [0b00011111u8, 0b10000001u8],
        };
        b2_u8.clear(12);
        assert_eq!(b2_u8.blocks, [0b00001111u8, 0b10000001u8]);
    }

    #[test]
    fn simple_get() {
        let b1_u8 = BitArray {
            blocks: [0b00010001u8],
        };
        assert_eq!(b1_u8.get(0), true);
        assert_eq!(b1_u8.get(4), true);

        let b2_u8 = BitArray {
            blocks: [0b00011111u8, 0b10000001u8],
        };
        assert_eq!(b2_u8.get(0), true);
        assert_eq!(b2_u8.get(7), true);

        assert_eq!(b2_u8.get(8), true);
        assert_eq!(b2_u8.get(9), true);
        assert_eq!(b2_u8.get(10), true);
        assert_eq!(b2_u8.get(11), true);
        assert_eq!(b2_u8.get(12), true);
    }

    #[test]
    fn simple_rhs() {
        let mut b1_u8 = BitArray {
            blocks: [0b00011000u8],
        };

        b1_u8 >>= 1;

        assert_eq!(b1_u8.blocks, [0b00001100u8]);
    }

    #[test]
    fn simple_rhs_overflow_block() {
        let mut overflow = BitArray {
            blocks: [0b00000001u8, 0b00000000u8],
        };

        overflow >>= 1;
        assert_eq!(overflow.blocks, [0b00000000u8, 0b10000000u8]);

        let mut overflow2 = BitArray {
            blocks: [0b00000001u8, 0b00000000u8],
        };

        overflow2 >>= 3;
        assert_eq!(overflow2.blocks, [0b00000000u8, 0b00100000u8]);
    }

    proptest! {
        #[test]
        fn right_shift_u64x2(bit_arr in any::<BitArray<2, u64>>(), shift in 0usize..64) {
            let mut shifted = bit_arr.clone();
            shifted >>= shift;

            for i in (bit_arr.bits() - shift)..bit_arr.bits() {
                prop_assert!(!shifted.get(i), "expected zero to be shifted in at pos {i}\n  shifted  = {shifted:?}\n  original = {bit_arr:?}");
            }

            for i in 0..(bit_arr.bits() - shift) {
                prop_assert!(bit_arr.get(i + shift) == shifted.get(i), "shift mismatch at pos {i}\n  shifted  = {shifted:?}\n  original = {bit_arr:?}");
            }
        }

        #[test]
        fn left_shift_u64x2(bit_arr in any::<BitArray<2, u64>>(), shift in 0usize..64) {
            let mut shifted = bit_arr.clone();
            shifted <<= shift;

            for i in 0..shift {
                prop_assert!(!shifted.get(i), "expected zero to be shifted in at pos {i}\n  shifted  = {shifted:?}\n  original = {bit_arr:?}");
            }

            for i in shift..bit_arr.bits() {
                prop_assert!(bit_arr.get(i - shift) == shifted.get(i), "shift mismatch at pos {i}\n  shifted  = {shifted:?}\n  original = {bit_arr:?}");
            }
        }


        #[test]
        fn bit_ops_u64x2(lhs in any::<BitArray<2, u64>>(), rhs in any::<BitArray<2, u64>>()) {
            let l0 = lhs.blocks[0];
            let l1 = lhs.blocks[1];

            let r0 = rhs.blocks[0];
            let r1 = rhs.blocks[1];

            // check equivalence to manual impl
            prop_assert_eq!((lhs.clone() & rhs.clone()).blocks, [l0 & r0, l1 & r1]);
            prop_assert_eq!((lhs.clone() | rhs.clone()).blocks, [l0 | r0, l1 | r1]);
            prop_assert_eq!((lhs.clone() ^ rhs.clone()).blocks, [l0 ^ r0, l1 ^ r1]);

            // check commutative
            prop_assert_eq!(lhs.clone() & rhs.clone(), rhs.clone() & lhs.clone());
            prop_assert_eq!(lhs.clone() | rhs.clone(), rhs.clone() | lhs.clone());
            prop_assert_eq!(lhs.clone() ^ rhs.clone(), rhs.clone() ^ lhs.clone());
        }
    }
}
