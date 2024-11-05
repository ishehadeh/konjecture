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

    // get the index of the first zero bit in the block
    fn first_clear(&self) -> Option<usize>;

    // get the index of the last zero bit in the block
    fn last_clear(&self) -> Option<usize>;

    fn count_set(&self) -> usize;

    fn count_clear(&self) -> usize;

    fn first(&self, is_set: bool) -> Option<usize> {
        if is_set {
            self.first_set()
        } else {
            self.first_clear()
        }
    }

    fn last(&self, is_set: bool) -> Option<usize> {
        if is_set {
            self.last_set()
        } else {
            self.last_clear()
        }
    }

    fn count(&self, is_set: bool) -> usize {
        if is_set {
            self.count_set()
        } else {
            self.count_clear()
        }
    }
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
                    i => Some(i - 1),
                }
            }

            #[inline(always)]
            fn first_clear(&self) -> Option<usize> {
                match (*self).trailing_ones() as usize {
                    Self::BLOCK_LENGTH => None,
                    i => Some(i),
                }
            }

            #[inline(always)]
            fn last_clear(&self) -> Option<usize> {
                match (*self).leading_ones() as usize {
                    Self::BLOCK_LENGTH => None,
                    i => Some(i),
                }
            }

            #[inline(always)]
            fn count_set(&self) -> usize {
                self.count_ones() as usize
            }

            #[inline(always)]
            fn count_clear(&self) -> usize {
                self.count_zeros() as usize
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
