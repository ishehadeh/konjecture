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
                    i => Some(Self::BLOCK_LENGTH - 1 - i),
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
                    i => Some(Self::BLOCK_LENGTH - 1 - i),
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

#[cfg(test)]
mod test {
    use crate::BitArrayBlock;
    use proptest::{bits, prelude::*};

    proptest! {
        #[test]
        fn first_set(block in bits::u64::ANY) {
            match block.first_set() {
                None => prop_assert!(block == 0, "block={block:064b}, but block.first_set() returned None"),
                Some(i) => {
                    prop_assert!(block & (1u64 << i) != 0, "block={block:064b}, and block.first_set() = {i}, but bit #{i} is not set (i.e. block & {mask:064b} != 0)", mask = (1u64 << i));
                    prop_assert!(block & !(u64::MAX << i) == 0, "block={block:064b}, and block.first_set() = {i}, but bit prior bits are set");
                }
            }
        }
        #[test]
        fn last_set(block in bits::u64::ANY) {
            match block.last_set() {
                None => prop_assert!(block == 0, "block={block:064b}, but block.last_set() returned None"),
                Some(i) => {
                    prop_assert!(block & (1u64 << i) > 0, "block={block:064b}, and block.first_set() = {i}, but bit #{i} is not set (i.e. block & {mask:064b} != 0)", mask = (1u64 << i));
                    if i != 63 {
                    prop_assert!(block & (u64::MAX << i + 1) == 0, "block={block:064b}, and block.first_set() = {i}, but bit later bits are set");
                    }
                }
            }
        }
        #[test]
        fn first_clear(block in bits::u64::ANY) {
            match block.first_clear() {
                None => prop_assert!(block == u64::MAX, "block={block:064b}, but block.first_clear() returned None"),
                Some(i) => {
                    prop_assert!(!block & (1u64 << i) != 0, "block={block:064b}, and block.first_set() = {i}, but bit #{i} is not set (i.e. block & {mask:064b} != 0)", mask = (1u64 << i));
                    prop_assert!(!block & !(u64::MAX << i) == 0, "block={block:064b}, and block.first_set() = {i}, but bit prior bits are set");
                }
            }
        }
        #[test]
        fn last_clear(block in bits::u64::ANY) {
            match block.last_clear() {
                None => prop_assert!(block == u64::MAX, "block={block:064b}, but block.last_clear() returned None"),
                Some(i) => {
                    prop_assert!(!block & (1u64 << i) > 0, "block={block:064b}, and block.last_clear() = {i}, but bit #{i} is not set (i.e. !block & {mask:064b} != 0)", mask = (1u64 << i));
                    if i != 63 {
                    prop_assert!(!block & (u64::MAX << i + 1) == 0, "block={block:064b}, and block.last_clear() = {i}, but bit later bits are set");
                    }
                }
            }
        }
    }
}
