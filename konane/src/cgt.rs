use cgt::short::partizan::partizan_game::PartizanGame;

use crate::Konane256;

impl<const W: usize, const H: usize> PartizanGame for Konane256<W, H> {
    fn left_moves(&self) -> Vec<Self> {
        self.all_moves_black()
    }

    fn right_moves(&self) -> Vec<Self> {
        self.all_moves_white()
    }
}

#[cfg(test)]
mod test {
    use cgt::{
        numeric::{dyadic_rational_number::DyadicRationalNumber, nimber::Nimber},
        short::partizan::{
            canonical_form::{CanonicalForm, Moves, Nus},
            partizan_game::PartizanGame,
            transposition_table::{ParallelTranspositionTable, TranspositionTable},
        },
    };

    use crate::{Konane256, TileState};

    fn gen_solid_linear_pattern(n: usize) -> Konane256<256, 1> {
        let alternating = 0xAAAAAAAAAAAAAAAAu64;
        assert!(alternating | (alternating >> 1) == u64::MAX);

        let mut game = Konane256::<256, 1>::empty();
        let bits_needed = n as usize + 1;
        let n_blocks_rounded_up = (bits_needed + 63) / 64;

        for i in (4 - n_blocks_rounded_up..4).rev() {
            game.black.board.blocks_mut()[i] = alternating;
            game.white.board.blocks_mut()[i] = alternating << 1;
            if i < 3 {
                game.white.board.blocks_mut()[i] |= 1;
            }
        }
        game.white
            .board
            .clear_range(bits_needed..(64 * n_blocks_rounded_up));
        game.black
            .board
            .clear_range(bits_needed..(64 * n_blocks_rounded_up));
        game
    }

    fn linear_with_offset_tail1(n: usize) -> Konane256<64, 4> {
        let mut game = Konane256::empty();

        for x in 0..n {
            if x < 1 {
                game.set_tile(
                    x,
                    2,
                    if x % 2 == 0 {
                        TileState::White
                    } else {
                        TileState::Black
                    },
                );
            } else {
                game.set_tile(
                    x,
                    1,
                    if (x - 1) % 2 == 0 {
                        TileState::White
                    } else {
                        TileState::Black
                    },
                )
            }
        }

        game
    }

    fn linear_with_tail(tail_len: usize, n: usize, offset: usize) -> Konane256<32, 8> {
        let mut game = Konane256::empty();
        for i in 0..tail_len {
            game.set_tile(
                0,
                5 + i,
                if i % 2 == 0 {
                    TileState::White
                } else {
                    TileState::Black
                },
            );
        }

        for x in offset..(n - tail_len + offset) {
            game.set_tile(
                x,
                4,
                if x % 2 == 0 {
                    TileState::Black
                } else {
                    TileState::White
                },
            )
        }

        game
    }

    #[test]
    fn solid_linear_pattern() {
        // source: https://citeseerx.ist.psu.edu/document?repid=rep1&type=pdf&doi=532fc4499a110b79b190e62e23de49c1c51b3f6f
        let slp = |n: u8| gen_solid_linear_pattern(n as usize);
        let fuzzy = Nus::new_nimber(Nimber::new(1));
        let zero = Nus::new_integer(0);

        let mut tt = ParallelTranspositionTable::new();
        let mut slp_nus = |n| slp(n).canonical_form(&mut tt).to_nus().expect("");
        assert_eq!(slp_nus(0), zero);
        assert_eq!(slp_nus(1), zero);
        assert_eq!(slp_nus(2), fuzzy);
        assert_eq!(slp_nus(3), Nus::new_integer(-1));
        assert_eq!(slp_nus(4), zero);
        assert_eq!(slp_nus(5), Nus::new_integer(-2));
        assert_eq!(slp_nus(6), fuzzy);
        assert_eq!(slp_nus(7), Nus::new_integer(-3));
        assert_eq!(slp_nus(8), zero);
        assert_eq!(slp_nus(9), Nus::new_integer(-4));
        assert_eq!(slp_nus(10), fuzzy);
        for i in 11..70 {
            let game = slp(i);
            assert!(tt.lookup_position(&game).is_none());
            let nus = game.canonical_form(&mut tt).to_nus().unwrap();
            assert_eq!(
                nus,
                if i % 4 == 0 {
                    zero
                } else if i >= 2 && (i - 2) % 4 == 0 {
                    fuzzy
                } else {
                    Nus::new_integer(-(i as i64 - 1) / 2)
                },
                "n={i}"
            );
        }
    }

    #[test]
    fn linear_with_offset_tail_1() {
        // source: https://citeseerx.ist.psu.edu/document?repid=rep1&type=pdf&doi=532fc4499a110b79b190e62e23de49c1c51b3f6f
        let lot1 = |n: u8| linear_with_offset_tail1(n as usize);
        let fuzzy = Nus::new_nimber(Nimber::new(1));
        let zero = Nus::new_integer(0);
        let down = Nus::new(DyadicRationalNumber::new(0, 0), -1, Nimber::new(0));
        let int = |i: i64| Nus::new_integer(i);

        let mut tt = ParallelTranspositionTable::new();
        let mut lot1_nus = |n| dbg!(lot1(n)).canonical_form(&mut tt).to_nus().expect("");

        assert_eq!(lot1_nus(0), zero);
        assert_eq!(lot1_nus(1), zero);
        assert_eq!(lot1_nus(2), zero);
        assert_eq!(lot1_nus(3), down);
        assert_eq!(lot1_nus(4), int(4 / 2 - 1));
        assert_eq!(lot1_nus(5), fuzzy);
        assert_eq!(lot1_nus(6), int(6 / 2 - 1));
        assert_eq!(lot1_nus(7), zero);
        for i in 8..64i64 {
            let game = lot1_nus(i as u8);
            assert_eq!(
                game,
                if i % 2 == 0 {
                    int(i / 2 - 1)
                } else if (i - 1) % 4 == 0 {
                    fuzzy
                } else if (i - 3) % 4 == 0 {
                    zero
                } else {
                    panic!("no condition matched for {i}, this should be unreachable")
                },
                "n={i}"
            );
        }
    }

    #[test]
    fn linear_with_offset_tail_2() {
        // source: https://citeseerx.ist.psu.edu/document?repid=rep1&type=pdf&doi=532fc4499a110b79b190e62e23de49c1c51b3f6f
        let lot2 = |n: u8| linear_with_tail(2, n as usize, 1);
        let fuzzy = Nus::new_nimber(Nimber::new(1));
        let zero = Nus::new_integer(0);
        let down = Nus::new(DyadicRationalNumber::new(0, 0), -1, Nimber::new(0));
        let int = |i: i64| Nus::new_integer(i);
        let rat = |n: i64, d_exp: u32| Nus::new_number(DyadicRationalNumber::new(n, d_exp));

        let mut tt = ParallelTranspositionTable::new();
        let mut lot2_nus = |n| dbg!(lot2(n)).canonical_form(&mut tt).to_nus().expect("");

        assert_eq!(lot2_nus(3), down);
        assert_eq!(lot2_nus(4), zero);
        assert_eq!(lot2_nus(5), rat(1, 1));

        for i in 6..32i64 {
            let game = lot2_nus(i as u8);
            assert_eq!(
                game,
                if i % 4 == 0 {
                    int(-1) + fuzzy
                } else if (i - 1) % 4 == 0 {
                    int(2 * (i - 1) / 4 - 2)
                } else if (i - 2) % 4 == 0 {
                    int(-1)
                } else if (i - 3) % 4 == 0 {
                    int(2 * (i - 3) / 4 - 1)
                } else {
                    panic!("no condition matched for {i}, this should be unreachable")
                },
                "n={i}"
            );
        }
    }

    #[test]
    fn linear_with_tail_1() {
        // source: https://citeseerx.ist.psu.edu/document?repid=rep1&type=pdf&doi=532fc4499a110b79b190e62e23de49c1c51b3f6f
        let lt1 = |n: u8| linear_with_tail(1, n as usize, 0);
        let fuzzy = || CanonicalForm::new_nimber(DyadicRationalNumber::new(0, 0), Nimber::new(1));
        let zero = || CanonicalForm::new_integer(0);
        let down = || {
            CanonicalForm::new_nus(Nus::new(
                DyadicRationalNumber::new(0, 0),
                -1,
                Nimber::new(0),
            ))
        };
        let int = |i: i64| CanonicalForm::new_integer(i);
        let opts1 = |l: CanonicalForm, r: CanonicalForm| {
            CanonicalForm::new_from_moves(Moves {
                left: vec![l],
                right: vec![r],
            })
        };

        let mut tt = ParallelTranspositionTable::new();
        let mut lt1 = |n| dbg!(lt1(n)).canonical_form(&mut tt);

        assert_eq!(lt1(1), zero());
        assert_eq!(lt1(2), fuzzy());
        assert_eq!(lt1(3), fuzzy());
        assert_eq!(lt1(4), opts1(fuzzy(), down()));
        for i in 5..32i64 {
            let game = lt1(i as u8);
            assert_eq!(
                game,
                if i % 4 == 0 {
                    opts1(fuzzy(), int(-2 * (i / 4) + 2))
                } else if (i - 1) % 4 == 0 {
                    opts1(int(2 * ((i - 1) / 4) - 1), fuzzy())
                } else if (i - 2) % 4 == 0 {
                    opts1(zero(), int(-2 * ((i - 2) / 4) + 1))
                } else if (i - 3) % 4 == 0 {
                    opts1(int(2 * ((i - 3) / 4)), zero())
                } else {
                    panic!("no condition matched for {i}, this should be unreachable")
                },
                "n={i}"
            );
        }
    }
}
