pub mod bitboard;
#[cfg(feature = "cgt")]
pub mod cgt;
use std::fmt::Debug;
mod konane_dyn_dim;
pub use konane_dyn_dim::*;
pub mod invariant;
use bitboard::BitBoard;

pub type Konane256<const W: usize = 16, const H: usize = 16, B = bnum::BUint<4>> =
    Konane<StaticBoard<W, H>, B>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TileState {
    White,
    Black,
    Empty,
}
mod const_direction {
    use std::fmt::Debug;

    use crate::bitboard::Direction;

    pub trait ConstDirection: Debug {
        const VALUE: Direction;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Up;
    impl ConstDirection for Up {
        const VALUE: Direction = Direction::Up;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Down;
    impl ConstDirection for Down {
        const VALUE: Direction = Direction::Down;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Left;
    impl ConstDirection for Left {
        const VALUE: Direction = Direction::Left;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Right;
    impl ConstDirection for Right {
        const VALUE: Direction = Direction::Right;
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::{Konane256, TileState};

    #[test]
    pub fn checkerboard_16x16() {
        let board: Konane256<16, 16> = Konane256::checkerboard(Default::default());
        for x in 0..16 {
            for y in 0..16 {
                assert_ne!(board.get_tile(x, y), TileState::Empty);
                if x > 0 {
                    assert_ne!(board.get_tile(x, y), board.get_tile(x - 1, y));
                }
                if y > 1 {
                    assert_ne!(board.get_tile(x, y), board.get_tile(x, y - 1));
                }
            }
        }
    }

    #[test]
    pub fn checkerboard_11x11() {
        let board: Konane256<11, 11, u128> = Konane256::checkerboard(Default::default());
        for x in 0..11 {
            for y in 0..11 {
                assert_ne!(board.get_tile(x, y), TileState::Empty);
                if x > 0 {
                    assert_ne!(board.get_tile(x, y), board.get_tile(x - 1, y));
                }
                if y > 1 {
                    assert_ne!(board.get_tile(x, y), board.get_tile(x, y - 1));
                }
            }
        }
    }

    #[test]
    pub fn move_near_block_boundary() {
        let board: Konane256<256, 1> = Konane256::must_parse(
            r#"_oxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxox"#,
        );

        let w = Konane256::must_parse(
            r#"x__oxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxox"#,
        );
        let b = Konane256::must_parse(
            r#"_oxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxox__o"#,
        );

        assert_eq!(board.all_moves_white(), vec![w]);
        assert_eq!(board.all_moves_black(), vec![b]);
    }

    #[test]
    pub fn move_over_block_boundary() {
        let board: Konane256<256, 1> = Konane256::must_parse(
            r#"_oxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxox"#,
        );

        let w = Konane256::must_parse(
            r#"x__oxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxox"#,
        );
        let b = Konane256::must_parse(
            r#"_oxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxox__o"#,
        );

        assert_eq!(board.all_moves_white(), vec![w]);
        assert_eq!(board.all_moves_black(), vec![b]);
    }

    #[test]
    pub fn moveset_on_full_board_is_empty_16x16() {
        let board: Konane256<16, 16> = Konane256::checkerboard(Default::default());

        assert_eq!(board.all_moves_black(), vec![]);
        assert_eq!(board.all_moves_white(), vec![]);
    }

    #[test]
    pub fn moveset_on_full_board_is_empty_11x11() {
        let board = Konane256::<11, 11, u128>::checkerboard(Default::default());
        assert_eq!(board.all_moves_black(), vec![]);
        assert_eq!(board.all_moves_white(), vec![]);
    }

    #[test]
    pub fn moveset_white_right_jump() {
        let board: Konane256 = Konane256::must_parse("xo");
        assert_eq!(board.all_moves_white(), vec![Konane256::must_parse("__x")]);
        assert_eq!(board.all_moves_black(), vec![]);

        let board: Konane256<4, 4, u32> = Konane256::must_parse("xo");
        assert_eq!(board.all_moves_white(), vec![Konane256::must_parse("__x")]);
        assert_eq!(board.all_moves_black(), vec![]);
    }

    #[test]
    pub fn moveset_black_right_jump() {
        let board: Konane256 = Konane256::must_parse("ox");
        assert_eq!(board.all_moves_black(), vec![Konane256::must_parse("__o")]);
        assert_eq!(board.all_moves_white(), vec![]);
    }

    #[test]
    pub fn moveset_white_left_jump() {
        let board: Konane256 = Konane256::must_parse("_oxx");
        assert_eq!(board.all_moves_white(), vec![Konane256::must_parse("x__x")]);
        assert_eq!(board.all_moves_black(), vec![]);
    }

    #[test]
    pub fn moveset_black_left_jump() {
        let board: Konane256 = Konane256::must_parse("_xoo");
        assert_eq!(board.all_moves_black(), vec![Konane256::must_parse("o__o")]);
        assert_eq!(board.all_moves_white(), vec![]);
    }

    #[test]
    pub fn moveset_white_up_jump() {
        let board: Konane256 = Konane256::must_parse("_\no\nx\nx");
        assert_eq!(
            board.all_moves_white(),
            vec![Konane256::must_parse("x\n\n\nx")]
        );
        assert_eq!(board.all_moves_black(), vec![]);
    }

    #[test]
    pub fn moveset_black_up_jump() {
        let board: Konane256 = Konane256::must_parse("_\nx\no\no");
        assert_eq!(
            board.all_moves_black(),
            vec![Konane256::must_parse("o\n\n\no")]
        );
        assert_eq!(board.all_moves_white(), vec![]);
    }

    #[test]
    pub fn moveset_white_down_jump() {
        let board: Konane256 = Konane256::must_parse("x\no");
        assert_eq!(
            board.all_moves_white(),
            vec![Konane256::must_parse("_\n\nx")]
        );
        assert_eq!(board.all_moves_black(), vec![]);
    }

    #[test]
    pub fn moveset_black_down_jump() {
        let board: Konane256 = Konane256::must_parse("o\nx");
        assert_eq!(
            board.all_moves_black(),
            vec![Konane256::must_parse("_\n\no")]
        );
        assert_eq!(board.all_moves_white(), vec![]);
    }

    #[test]
    pub fn linear_tail_1_with_4_stones() {
        let board: Konane256 = Konane256::must_parse(
            r#"_____
               _oxo_
               _x___
               _____"#,
        );
        assert_eq!(
            board.all_moves_black(),
            vec![Konane256::must_parse(
                r#"_____
                   __xo_
                   _____
                   _o___"#
            )]
        );
        assert_eq!(
            HashSet::from_iter(board.all_moves_white().into_iter()),
            HashSet::from([
                Konane256::must_parse(
                    r#"_____
                       _o__x
                       _x___
                       _____"#
                ),
                Konane256::must_parse(
                    r#"_____
                       x__o_
                       _x___
                       _____"#
                ),
                Konane256::must_parse(
                    r#"_x___
                       __xo_
                       _____
                       _____"#
                ),
            ])
        );
    }
}
