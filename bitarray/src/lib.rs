#[cfg(any(test, feature = "proptest"))]
mod arbitrary;

use std::{
    fmt::Debug,
    ops::{
        BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
        ShrAssign,
    },
};

pub trait BitArrayBlock:
    BitOr<Output = Self>
    + BitAnd<Output = Self>
    + BitXor<Output = Self>
    + BitOrAssign
    + BitAndAssign
    + BitXorAssign
    + Shl<usize, Output = Self>
    + Shr<usize, Output = Self>
    + ShrAssign<usize>
    + ShlAssign<usize>
    + Copy
    + Not<Output = Self>
    + PartialEq
    + Debug
    + Eq
    + std::fmt::Binary
{
    const BLOCK_LENGTH: usize;

    fn block_from_indicies<const N: usize>(indicies: [usize; N]) -> Self;

    fn empty() -> Self {
        Self::block_from_indicies([])
    }

    fn all() -> Self {
        !Self::empty()
    }

    fn set(&mut self, bit: usize) {
        *self |= Self::block_from_indicies([bit]);
    }

    fn clear(&mut self, bit: usize) {
        *self &= !Self::block_from_indicies([bit]);
    }

    fn get(&self, bit: usize) -> bool {
        *self & Self::block_from_indicies([bit]) != Self::empty()
    }

    // get the index of the first non-zero bit in the block
    fn first_set(&self) -> Option<usize>;

    // get the index of the last non-zero bit in the block
    fn last_set(&self) -> Option<usize>;
}

macro_rules! impl_bit_array_block {
    ($ty:ident, bits=$bits:expr) => {
        impl BitArrayBlock for $ty {
            const BLOCK_LENGTH: usize = $bits;

            #[inline(always)]
            fn block_from_indicies<const N: usize>(indicies: [usize; N]) -> Self {
                let mut val: Self = 0;
                for i in 0..N {
                    debug_assert!(indicies[i] < Self::BLOCK_LENGTH);
                    val |= ((1 as Self) << indicies[i]);
                }

                val
            }

            #[inline(always)]
            fn first_set(&self) -> Option<usize> {
                match (*self).trailing_zeros() as usize {
                    Self::BLOCK_LENGTH => None,
                    i => Some(i),
                }
            }

            #[inline(always)]
            fn last_set(&self) -> Option<usize> {
                match (*self).leading_zeros() as usize {
                    Self::BLOCK_LENGTH => None,
                    i => Some(i),
                }
            }
        }
    };
}

impl_bit_array_block!(u8, bits = 8);
impl_bit_array_block!(u16, bits = 16);
impl_bit_array_block!(u32, bits = 32);
impl_bit_array_block!(u64, bits = 64);
impl_bit_array_block!(u128, bits = 128);
impl_bit_array_block!(usize, bits = usize::BITS as usize);

#[derive(Clone)]
pub struct BitArray<const BLOCK_COUNT: usize, Block: BitArrayBlock = u64> {
    // blocks are stored in reverse order
    blocks: [Block; BLOCK_COUNT],
}

pub fn test_asm(mut arr: BitArray<4>) -> BitArray<4> {
    arr.lsh(1);
    arr
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

    pub fn rsh(&mut self, n: usize) {
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

    pub fn lsh(&mut self, n: usize) {
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

    pub fn bits(&self) -> usize {
        Block::BLOCK_LENGTH * BLOCK_COUNT
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

#[cfg(test)]
mod test {
    use proptest::{prelude::any, prop_assert, proptest};

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

        b1_u8.rsh(1);

        assert_eq!(b1_u8.blocks, [0b00001100u8]);
    }

    #[test]
    fn simple_rhs_overflow_block() {
        let mut overflow = BitArray {
            blocks: [0b00000001u8, 0b00000000u8],
        };

        overflow.rsh(1);
        assert_eq!(overflow.blocks, [0b00000000u8, 0b10000000u8]);

        let mut overflow2 = BitArray {
            blocks: [0b00000001u8, 0b00000000u8],
        };

        overflow2.rsh(3);
        assert_eq!(overflow2.blocks, [0b00000000u8, 0b00100000u8]);
    }

    proptest! {
        #[test]
        fn rhs_u64_2(bit_arr in any::<BitArray<2, u64>>(), shift in 0usize..64) {
            let mut shifted = bit_arr.clone();
            shifted.rsh(shift);

            for i in (bit_arr.bits() - shift)..bit_arr.bits() {
                prop_assert!(!shifted.get(i), "expected zero to be shifted in at pos {i}\n  shifted  = {shifted:?}\n  original = {bit_arr:?}");
            }

            for i in 0..(bit_arr.bits() - shift) {
                prop_assert!(bit_arr.get(i + shift) == shifted.get(i), "shift mismatch at pos {i}\n  shifted  = {shifted:?}\n  original = {bit_arr:?}");
            }
        }

        #[test]
        fn lhs_u64_2(bit_arr in any::<BitArray<2, u64>>(), shift in 0usize..64) {
            let mut shifted = bit_arr.clone();
            shifted.lsh(shift);

            for i in 0..shift {
                prop_assert!(!shifted.get(i), "expected zero to be shifted in at pos {i}\n  shifted  = {shifted:?}\n  original = {bit_arr:?}");
            }

            for i in shift..bit_arr.bits() {
                prop_assert!(bit_arr.get(i - shift) == shifted.get(i), "shift mismatch at pos {i}\n  shifted  = {shifted:?}\n  original = {bit_arr:?}");
            }
        }
    }
}