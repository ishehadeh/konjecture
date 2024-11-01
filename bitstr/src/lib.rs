#[cfg(any(test, feature = "proptest"))]
mod arbitrary;

use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
    ShrAssign,
};

pub trait BitArrayBlock:
    BitOr<Output = Self>
    + BitAnd<Output = Self>
    + BitXor<Output = Self>
    + BitOrAssign
    + BitAndAssign
    + BitXorAssign
    + Shl<Output = Self>
    + Shr<Output = Self>
    + ShrAssign
    + ShlAssign
    + Copy
    + Not<Output = Self>
    + PartialEq
    + Eq
    + std::fmt::Binary
{
    const BLOCK_LENGTH: usize;

    fn block_from_indicies<const N: usize>(indicies: [usize; N]) -> Self;

    fn set(&mut self, bit: usize) {
        *self |= Self::block_from_indicies([bit]);
    }

    fn clear(&mut self, bit: usize) {
        *self &= !Self::block_from_indicies([bit]);
    }

    fn includes(&mut self, bit: usize) -> bool {
        *self & Self::block_from_indicies([bit]) != Self::block_from_indicies([])
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
impl_bit_array_block!(usize, bits = usize::BITS as usize);

#[derive(Clone)]
pub struct BitArray<const BLOCK_COUNT: usize, Block: BitArrayBlock = u64> {
    // blocks are stored in reverse order
    blocks: [Block; BLOCK_COUNT],
}

impl<const BLOCK_COUNT: usize, Block: BitArrayBlock> BitArray<BLOCK_COUNT, Block> {
    // #region Static Utility Functions
    const fn block_len() -> usize {
        Block::BLOCK_LENGTH as usize
    }

    /// Get a (block-index, bit-within-block-index) from a bit index
    const fn addr(bit: usize) -> (usize, usize) {
        // blocks are stored first to last
        let block = BLOCK_COUNT - bit / Self::block_len() - 1;
        let bit = bit % Self::block_len();
        (block, bit)
    }
    // #endregion

    pub fn new() -> Self {
        Self {
            blocks: [Block::block_from_indicies([]); BLOCK_COUNT],
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
}
