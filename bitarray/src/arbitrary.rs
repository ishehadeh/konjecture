use crate::BitArray;
use proptest::{
    array::{uniform, UniformArrayStrategy},
    bits::BitSetStrategy,
    prelude::Arbitrary,
    strategy::statics,
};

fn arr_to_bitarr<const BLOCK_COUNT: usize>(
    blocks: [u64; BLOCK_COUNT],
) -> BitArray<BLOCK_COUNT, u64> {
    BitArray { blocks }
}

impl<const BLOCK_COUNT: usize> Arbitrary for BitArray<BLOCK_COUNT, u64> {
    type Parameters = ();

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        statics::Map::new(
            uniform::<_, BLOCK_COUNT>(proptest::bits::u64::ANY),
            arr_to_bitarr,
        )
    }

    type Strategy = statics::Map<
        UniformArrayStrategy<BitSetStrategy<u64>, [u64; BLOCK_COUNT]>,
        fn([u64; BLOCK_COUNT]) -> BitArray<BLOCK_COUNT, u64>,
    >;
}
