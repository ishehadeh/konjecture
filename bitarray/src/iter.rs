use crate::{block::BitArrayBlock, BitArray};

#[derive(Debug, Clone)]
pub struct BitArrayIter<
    'a,
    const VALUE: bool,
    const REV: bool,
    const BLOCK_COUNT: usize,
    Block: BitArrayBlock,
> {
    target: &'a BitArray<BLOCK_COUNT, Block>,
    block_index: u32,
    bit_index: u32,
}

impl<'a, const VALUE: bool, const REV: bool, const BLOCK_COUNT: usize, Block: BitArrayBlock>
    BitArrayIter<'a, VALUE, REV, BLOCK_COUNT, Block>
{
    pub(crate) fn new(target: &'a BitArray<BLOCK_COUNT, Block>) -> Self {
        Self {
            target,
            block_index: if !REV { BLOCK_COUNT as u32 - 1 } else { 0 },
            bit_index: if REV {
                Block::BLOCK_LENGTH as u32 - 1
            } else {
                0
            },
        }
    }

    fn next_in_block(&self) -> Option<usize> {
        let unchecked = self.get_unchecked_in_block();

        if !REV {
            unchecked.first(VALUE)
        } else {
            unchecked.last(VALUE)
        }
    }

    fn get_unchecked_in_block(&self) -> Block {
        let mask = if REV {
            !(Block::all() << self.bit_index as usize)
        } else {
            Block::all() << self.bit_index as usize
        };
        // println!("{mask:064b}, {}", self.bit_index);

        if VALUE {
            // set all checked bits to 0
            self.target.blocks[self.block_index as usize] & mask
        } else {
            // set all checked bits to 1
            self.target.blocks[self.block_index as usize] | !mask
        }
    }

    fn count_unchecked(&self) -> usize {
        self.count_unchecked_in_block() + self.count_unchecked_after_block()
    }

    fn count_unchecked_in_block(&self) -> usize {
        let unchecked = self.get_unchecked_in_block();
        if VALUE {
            unchecked.count_set()
        } else {
            unchecked.count_clear()
        }
    }

    fn count_unchecked_after_block(&self) -> usize {
        if !REV {
            self.target.blocks[0..self.block_index as usize]
                .iter()
                .map(|b| b.count(VALUE))
                .sum()
        } else {
            self.target.blocks[self.block_index as usize + 1..]
                .iter()
                .map(|b| b.count(VALUE))
                .sum()
        }
    }
}

impl<'a, const VALUE: bool, const REV: bool, const BLOCK_COUNT: usize, Block: BitArrayBlock>
    Iterator for BitArrayIter<'a, VALUE, REV, BLOCK_COUNT, Block>
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.block_index as usize >= BLOCK_COUNT {
            None
        } else if let Some(index) = self.next_in_block() {
            let result =
                Some(index + Block::BLOCK_LENGTH * (BLOCK_COUNT - self.block_index as usize - 1));
            self.bit_index = index as u32 + 1;
            if self.bit_index >= 64 {
                if REV {
                    // wrap around if we subtract below zero, so the above >= check still passes
                    self.block_index = self.block_index.wrapping_add(1);
                    self.bit_index = 0;
                } else {
                    // wrap around to zero if we're below, so we can reverse the iterator after
                    // falling below zero
                    self.block_index = self.block_index.wrapping_sub(1);
                    self.bit_index = 0;
                }
            }

            result
        } else {
            if REV {
                // wrap around if we subtract below zero, so the above >= check still passes
                self.block_index = self.block_index.wrapping_add(1);
                self.bit_index = 0;
            } else {
                // wrap around to zero if we're below, so we can reverse the iterator after
                // falling below zero
                self.block_index = self.block_index.wrapping_sub(1);
                self.bit_index = 0;
            }
            self.next()
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.count_unchecked();
        (remaining, Some(remaining))
    }

    fn count(self) -> usize {
        self.count_unchecked()
    }
}

#[cfg(test)]
mod test {
    use crate::BitArray;
    use proptest::prelude::*;

    #[test]
    fn first_bit_second_block_u64x2() {
        let mut arr = BitArray::<2, u64>::new();
        arr.set(64);
        arr.set(65);
        assert_eq!(arr.iter_set().collect::<Vec<_>>(), vec![64, 65])
    }

    proptest! {
        #[test]
        fn iter_set_u64x4(bits in any::<BitArray<4, u64>>()) {
            let mut last_index = 0;
            let mut iter = bits.iter_set();
            prop_assert_eq!(bits.iter_set().count(), bits.blocks().iter().map(|b| b.count_ones() as usize).sum());
            while let Some(bit_i) = iter.next(){
                prop_assert!(bits.get(bit_i), "Bit index from iter not set.\ni = {bit_i}\niter = {:?}", &iter);
                for unset_i in last_index+1..bit_i {
                    prop_assert!(!bits.get(unset_i), "Bit not returned by iter set.\ni = {unset_i}\niter = {:?}", &iter);
                }
                last_index = bit_i;
            }
            for unset_i in last_index+1..bits.bits() {
                prop_assert!(!bits.get(unset_i), "Bit not returned by iter set.\ni = {unset_i}\niter = {:?}", &iter);
            }
        }

        #[test]
        fn iter_clear_u64x4(bits in any::<BitArray<4, u64>>()) {
            let mut last_index = 0;
            prop_assert_eq!(bits.iter_clear().count(), bits.blocks().iter().map(|b| b.count_zeros() as usize).sum());
            let mut iter = bits.iter_clear();
            while let Some(bit_i) = iter.next() {
                prop_assert!(!bits.get(bit_i), "Bit index from iter set.\ni = {bit_i}\niter = {:?}", &iter);
                for unset_i in last_index+1..bit_i {
                    prop_assert!(bits.get(unset_i), "Bit not returned by iter not set.\ni = {unset_i}\niter = {:?}", &iter);
                }
                last_index = bit_i;
            }
            for unset_i in last_index+1..bits.bits() {
                prop_assert!(bits.get(unset_i), "Bit not returned by iter not set.\ni = {unset_i}\niter = {:?}", &iter);
            }
        }
    }
}
