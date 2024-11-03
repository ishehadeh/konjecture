use bitarray::BitArray;

pub struct Konane18x18 {
    pub white: BitArray<6, u64>,
    pub black: BitArray<6, u64>,
}

impl Konane18x18 {
    pub fn checkerboard() -> Self {
        let mut white = BitArray::new();
        let mut black = BitArray::new();

        for i in 0..(18 * 18) {
            if i % 2 == 0 {
                white.set(i);
            } else {
                black.set(i)
            }
        }

        Konane18x18 { white, black }
    }
}

#[cfg(test)]
mod test {
    use crate::Konane18x18;

    #[test]
    pub fn checkerboard() {
        let board = Konane18x18::checkerboard();
        for i in 0..(18 * 18 - 1) {
            assert_eq!(board.black.get(i), board.white.get(i + 1));
        }
    }
}
