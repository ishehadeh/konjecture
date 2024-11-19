use crate::{bitboard::BitBoard256, Konane256};
pub use nearest_border::*;
mod nearest_border;

pub trait InvariantValue {}

impl InvariantValue for u64 {}
impl InvariantValue for f64 {}
impl InvariantValue for usize {}

macro_rules! tuple_invariant_value {
    ([$(($ind:tt, $name:ident)),* $(,)?];,) => {};

    ([$(($ind:tt, $name:ident)),* $(,)?]; ($first_ind:tt, $first_name:ident), $(($rest_ind:tt, $rest_name:ident)),* $(,)?) => {
        impl<$($name: InvariantValue),*> InvariantValue for ($($name),*) {

        }

        impl<$($name: Invariant),*> Invariant for ($($name),*) {
            type Value = ($($name::Value),*);

            fn compute<const WIDTH: usize, const HEIGHT: usize>(&self, player: Konane256<WIDTH, HEIGHT>) -> Self::Value {
                ($(self.$ind.compute(player.clone())),*)
            }
        }

        tuple_invariant_value!([$(($ind, $name)),*, ($first_ind, $first_name) ]; $(($rest_ind, $rest_name)),*,);
    };
}

tuple_invariant_value!(
    [(0, A), (1, B)];
    (2, C),
    (3, D),
    (4, E),
    (5, F),
    (6, G),
    (7, H),
    (8, I),
    (9, J),
    (10, K),
    (11, L),
    (12, M),
    (13, N),
    (14, O),
    (15, P),
    (16, Q),
    (17, R),
    (18, S),
    (19, T),
    (20, U),
    (21, V),
    (22, W),
    (23, X),
    (24, Y),
    (25, Z)
);

pub trait SinglePlayerInvariant {
    type Value: InvariantValue;

    fn compute<const W: usize, const H: usize>(&self, player: BitBoard256<W, H>) -> Self::Value;
}

pub trait Invariant {
    type Value: InvariantValue;

    fn compute<const W: usize, const H: usize>(&self, game: Konane256<W, H>) -> Self::Value;
}

/// Construct a game invariant from a single player invariant by taking the union of player positions
pub struct ImpartialInvariant<T: SinglePlayerInvariant<Value = V>, V: InvariantValue> {
    pub inner: T,
}

impl<T: SinglePlayerInvariant<Value = V>, V: InvariantValue> ImpartialInvariant<T, V> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

/// Construct a game invariant from a single player invariant by only applying it to left
pub struct PartizanInvariant<
    T: SinglePlayerInvariant<Value = V>,
    V: InvariantValue,
    const IS_LEFT: bool,
> {
    pub inner: T,
}

impl<T: SinglePlayerInvariant<Value = V>, V: InvariantValue> PartizanInvariant<T, V, false> {
    pub fn right(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: SinglePlayerInvariant<Value = V>, V: InvariantValue> PartizanInvariant<T, V, true> {
    pub fn left(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: SinglePlayerInvariant<Value = V>, V: InvariantValue> Invariant
    for PartizanInvariant<T, V, false>
{
    type Value = V;

    #[inline(always)]
    fn compute<const W: usize, const H: usize>(&self, game: Konane256<W, H>) -> Self::Value {
        self.inner.compute(game.white)
    }
}

impl<T: SinglePlayerInvariant<Value = V>, V: InvariantValue> Invariant
    for PartizanInvariant<T, V, true>
{
    type Value = V;

    #[inline(always)]
    fn compute<const W: usize, const H: usize>(&self, game: Konane256<W, H>) -> Self::Value {
        self.inner.compute(game.black)
    }
}

impl<T: SinglePlayerInvariant<Value = V>, V: InvariantValue> Invariant
    for ImpartialInvariant<T, V>
{
    type Value = V;

    #[inline(always)]
    fn compute<const W: usize, const H: usize>(&self, game: Konane256<W, H>) -> Self::Value {
        self.inner.compute(BitBoard256::<W, H> {
            board: game.black.board | game.white.board,
        })
    }
}

/// Vertical distance between the highest and lowest piece
pub struct PieceHeight;
impl SinglePlayerInvariant for PieceHeight {
    type Value = usize;

    fn compute<const W: usize, const H: usize>(&self, game: BitBoard256<W, H>) -> Self::Value {
        let Some(first_set_ind) = game.board.first_set() else {
            return 0;
        };
        let first = first_set_ind / W;
        let last = game.board.last_set().unwrap_or(0) / W;
        last - first + 1
    }
}

pub struct PieceCount;
impl SinglePlayerInvariant for PieceCount {
    type Value = usize;

    fn compute<const W: usize, const H: usize>(&self, game: BitBoard256<W, H>) -> Self::Value {
        game.board.iter_set().count()
    }
}

/// distance between the first and last column with at least one peice
pub struct PieceWidth;
impl SinglePlayerInvariant for PieceWidth {
    type Value = usize;

    fn compute<const W: usize, const H: usize>(&self, game: BitBoard256<W, H>) -> Self::Value {
        let mut row_mask: BitBoard256<W, H> = Default::default();
        for y in 0..H {
            row_mask.set(0, y, true);
        }

        let mut first_last = None;
        for x in 0..W {
            if !(row_mask.board.clone() & &game.board).is_empty() {
                if let Some((_, last)) = &mut first_last {
                    *last = x;
                } else {
                    first_last = Some((x, x))
                }
            }
            row_mask.board >>= 1;
        }

        first_last
            .map(|(first, last)| last - first + 1)
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use crate::{invariant::Invariant, Konane256};

    use super::{ImpartialInvariant, PieceHeight, PieceWidth};

    #[test]
    pub fn partizan_size() {
        let game = Konane256::<16, 16>::must_parse(
            r#"
            __x_o__
            ____xo_
        "#,
        );

        let (h, w) = (
            ImpartialInvariant::new(PieceHeight),
            ImpartialInvariant::new(PieceWidth),
        )
            .compute(game);
        assert_eq!(h, 2);
        assert_eq!(w, 4);
    }
}
