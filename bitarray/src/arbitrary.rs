use std::marker::PhantomData;

use proptest::{
    prelude::{Arbitrary, Strategy},
    strategy::ValueTree,
};
use rand::{distributions::Standard, prelude::Distribution};

use crate::{BitArray, BitArrayBlock};

impl<const BLOCK_COUNT: usize> Arbitrary for BitArray<BLOCK_COUNT, u64> {
    type Parameters = ();

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        BitArrayStrategy {
            p: PhantomData,
            distribution: Standard,
        }
    }

    type Strategy = BitArrayStrategy<BLOCK_COUNT, u64, Standard>;
}

pub struct BitArrayStrategy<
    const BLOCK_COUNT: usize,
    Block: BitArrayBlock,
    BlockDistr: Distribution<Block>,
> {
    p: PhantomData<BitArray<BLOCK_COUNT, Block>>,
    distribution: BlockDistr,
}

impl<const BLOCK_COUNT: usize, Block: BitArrayBlock, BlockDistr: Distribution<Block>>
    std::fmt::Debug for BitArrayStrategy<BLOCK_COUNT, Block, BlockDistr>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BitArrayStrategy<{}, {}> {{ }}",
            BLOCK_COUNT,
            std::any::type_name::<Block>()
        )
    }
}

impl<const BLOCK_COUNT: usize, Block: BitArrayBlock, BlockDistr: Distribution<Block>> Strategy
    for BitArrayStrategy<BLOCK_COUNT, Block, BlockDistr>
{
    type Tree = BitArrayValueTree<BLOCK_COUNT, Block>;

    type Value = BitArray<BLOCK_COUNT, Block>;

    fn new_tree(
        &self,
        runner: &mut proptest::test_runner::TestRunner,
    ) -> proptest::strategy::NewTree<Self> {
        let mut val = BitArray::new();

        for block in val.blocks.iter_mut() {
            *block = self.distribution.sample(runner.rng());
        }

        Ok(BitArrayValueTree {
            current: val,
            toggled: BitArray::new(),
            from_start: true,
        })
    }
}

pub struct BitArrayValueTree<const BLOCK_COUNT: usize, Block: BitArrayBlock> {
    current: BitArray<BLOCK_COUNT, Block>,
    toggled: BitArray<BLOCK_COUNT, Block>,
    from_start: bool,
}

impl<const BLOCK_COUNT: usize, Block: BitArrayBlock> ValueTree
    for BitArrayValueTree<BLOCK_COUNT, Block>
{
    type Value = BitArray<BLOCK_COUNT, Block>;

    fn current(&self) -> Self::Value {
        self.current.clone()
    }

    fn simplify(&mut self) -> bool {
        let maybe_i = if self.from_start {
            self.current.first_set()
        } else {
            self.current.last_set()
        };

        if let Some(i) = maybe_i {
            self.current.clear(i);
            self.toggled.set(i);
            self.from_start = !self.from_start;
            true
        } else {
            false
        }
    }

    fn complicate(&mut self) -> bool {
        let maybe_i = if !self.from_start {
            self.toggled.first_set()
        } else {
            self.toggled.last_set()
        };

        if let Some(i) = maybe_i {
            self.current.set(i);
            self.toggled.clear(i);
            self.from_start = !self.from_start;
            true
        } else {
            false
        }
    }
}
