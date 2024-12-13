use std::{
    fmt::Debug,
    ops::{
        BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
        ShrAssign,
    },
};

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

pub trait BitBoard:
    BitAnd<Self, Output = Self>
    + for<'a> BitAnd<&'a Self, Output = Self>
    + BitOr<Self, Output = Self>
    + for<'a> BitOr<&'a Self, Output = Self>
    + BitAndAssign
    + for<'a> BitAndAssign<&'a Self>
    + BitOrAssign
    + BitXor<Self, Output = Self>
    + for<'a> BitXor<&'a Self, Output = Self>
    + BitXorAssign
    + Shl<usize, Output = Self>
    + ShlAssign<usize>
    + Shr<usize, Output = Self>
    + ShrAssign<usize>
    + Not<Output = Self>
    + PartialEq
    + Clone
    + Eq
    + std::fmt::Binary
    + Debug
where
    Self: Sized,
{
    type Iter<'a>: Iterator<Item = usize> + std::fmt::Debug
    where
        Self: 'a;
    const BIT_LENGTH: usize;

    fn empty() -> Self;
    fn all() -> Self;
    fn one() -> Self;

    fn set(&mut self, idx: usize);
    fn clear(&mut self, idx: usize);
    fn get(&self, idx: usize) -> bool;
    fn first_set(&self) -> Option<usize>;
    fn first_clear(&self) -> Option<usize>;
    fn last_set(&self) -> Option<usize>;
    fn count_set(&self) -> usize;
    fn count_clear(&self) -> usize;
    fn iter_set<'a>(&'a self) -> Self::Iter<'a>;
}

#[derive(Clone, Copy, Debug)]
pub struct BitIter<T: BitBoard> {
    index: usize,
    value: T,
}

impl<T: BitBoard> BitIter<T> {
    pub fn new(value: T) -> BitIter<T> {
        BitIter {
            index: value.first_set().unwrap_or(T::BIT_LENGTH),
            value,
        }
    }
}

impl<T: BitBoard> Iterator for BitIter<T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == T::BIT_LENGTH {
            None
        } else {
            self.value.clear(self.index);
            let index = self.index;
            self.index = self.value.first_set().unwrap_or(T::BIT_LENGTH);
            Some(index)
        }
    }
}

macro_rules! impl_bit_board {
    ($ty:path) => {
        impl BitBoard for $ty {
            const BIT_LENGTH: usize = std::mem::size_of::<$ty>() * 8;
            type Iter<'a> = BitIter<$ty>;

            #[inline(always)]
            fn empty() -> Self {
                0u8.into()
            }

            #[inline(always)]
            fn one() -> Self {
                1u8.into()
            }

            #[inline(always)]
            fn all() -> Self {
                !Self::empty()
            }

            #[inline(always)]
            fn set(&mut self, idx: usize) {
                *self |= Self::one() << idx
            }

            #[inline(always)]
            fn get(&self, idx: usize) -> bool {
                *self & Self::one() << idx != Self::empty()
            }

            #[inline(always)]
            fn clear(&mut self, idx: usize) {
                *self &= !(Self::one() << idx)
            }

            #[inline(always)]
            fn first_set(&self) -> Option<usize> {
                match (*self).trailing_zeros() as usize {
                    Self::BIT_LENGTH => None,
                    i => Some(i),
                }
            }

            #[inline(always)]
            fn first_clear(&self) -> Option<usize> {
                match (*self).trailing_ones() as usize {
                    Self::BIT_LENGTH => None,
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

            #[inline(always)]
            fn last_set(&self) -> Option<usize> {
                match (*self).leading_zeros() as usize {
                    Self::BIT_LENGTH => None,
                    i => Some(Self::BIT_LENGTH - 1 - i),
                }
            }

            fn iter_set(&self) -> Self::Iter<'_> {
                BitIter::new(*self)
            }
        }
    };
}

impl_bit_board!(u8);
impl_bit_board!(u16);
impl_bit_board!(u32);
impl_bit_board!(u64);
impl_bit_board!(u128);
impl_bit_board!(usize);

impl<const N: usize> BitBoard for bnum::BUint<N> {
    const BIT_LENGTH: usize = Self::BITS as usize;
    type Iter<'a> = BitIter<Self>;

    #[inline(always)]
    fn empty() -> Self {
        0u8.into()
    }

    #[inline(always)]
    fn one() -> Self {
        1u8.into()
    }

    #[inline(always)]
    fn all() -> Self {
        Self::MAX
    }

    #[inline(always)]
    fn set(&mut self, idx: usize) {
        self.set_bit(idx as u32, true);
    }

    #[inline(always)]
    fn get(&self, idx: usize) -> bool {
        self.bit(idx as u32)
    }

    #[inline(always)]
    fn clear(&mut self, idx: usize) {
        self.set_bit(idx as u32, false)
    }

    #[inline(always)]
    fn first_set(&self) -> Option<usize> {
        let trailing_zeros = (*self).trailing_zeros() as usize;
        if trailing_zeros == Self::BIT_LENGTH {
            None
        } else {
            Some(trailing_zeros)
        }
    }

    #[inline(always)]
    fn first_clear(&self) -> Option<usize> {
        let trailing_ones = (*self).trailing_ones() as usize;
        if trailing_ones == Self::BIT_LENGTH {
            None
        } else {
            Some(trailing_ones)
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

    #[inline(always)]
    fn last_set(&self) -> Option<usize> {
        let leading_zeros = (*self).leading_zeros() as usize;
        if leading_zeros == Self::BIT_LENGTH {
            None
        } else {
            Some(Self::BIT_LENGTH - 1 - leading_zeros)
        }
    }

    fn iter_set(&self) -> Self::Iter<'_> {
        BitIter::new(*self)
    }
}
