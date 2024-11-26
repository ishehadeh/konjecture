use std::{marker::PhantomData, ops::BitOr};

use crate::{
    const_direction::{Down, Left, Right, Up},
    BitBoard, Konane256,
};

use cgt::short::partizan::{partizan_game::PartizanGame, transposition_table::TranspositionTable};
// Invariant Submodules
pub use nearest_border::*;
mod nearest_border;

#[cfg(feature = "cgt")]
mod cgt_value;
#[cfg(feature = "cgt")]
pub use cgt_value::*;

/// A game which can be split into seperate structures representing each player individually.
pub trait TwoPlayerGame {
    type B;

    fn left(&self) -> Self::B;
    fn right(&self) -> Self::B;
}

impl<const W: usize, const H: usize, B: BitBoard> TwoPlayerGame for Konane256<W, H, B> {
    type B = B;

    fn left(&self) -> Self::B {
        self.black.clone()
    }

    fn right(&self) -> Self::B {
        self.white.clone()
    }
}

pub trait SinglePlayerInvariant<G: TwoPlayerGame> {
    fn compute(&self, player: G::B) -> f64;
}

pub trait Invariant<G> {
    fn compute(&self, game: G) -> f64;
}

/// Construct a game invariant from a single player invariant by taking the union of player positions
pub struct ImpartialInvariant<G: TwoPlayerGame, T: SinglePlayerInvariant<G>> {
    pub inner: T,
    g: PhantomData<G>,
}

impl<G: TwoPlayerGame, T: SinglePlayerInvariant<G>> ImpartialInvariant<G, T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            g: PhantomData,
        }
    }
}

/// Construct a game invariant from a single player invariant by only applying it to left
pub struct PartizanInvariant<G: TwoPlayerGame, T: SinglePlayerInvariant<G>, const IS_LEFT: bool> {
    pub inner: T,
    g: PhantomData<G>,
}

impl<G: TwoPlayerGame, T: SinglePlayerInvariant<G>> PartizanInvariant<G, T, false> {
    pub fn right(inner: T) -> Self {
        Self {
            inner,
            g: PhantomData,
        }
    }
}

impl<G: TwoPlayerGame, T: SinglePlayerInvariant<G>> PartizanInvariant<G, T, true> {
    pub fn left(inner: T) -> Self {
        Self {
            inner,
            g: PhantomData,
        }
    }
}

impl<G: TwoPlayerGame, T: SinglePlayerInvariant<G>> Invariant<G>
    for PartizanInvariant<G, T, false>
{
    #[inline(always)]
    fn compute(&self, game: G) -> f64 {
        self.inner.compute(game.right())
    }
}

impl<G: TwoPlayerGame, T: SinglePlayerInvariant<G>> Invariant<G> for PartizanInvariant<G, T, true> {
    #[inline(always)]
    fn compute(&self, game: G) -> f64 {
        self.inner.compute(game.left())
    }
}

impl<G: TwoPlayerGame, T: SinglePlayerInvariant<G>> Invariant<G> for ImpartialInvariant<G, T>
where
    G::B: BitOr<G::B, Output = G::B>,
{
    #[inline(always)]
    fn compute(&self, game: G) -> f64 {
        self.inner.compute(game.left() | game.right())
    }
}

/// Vertical distance between the highest and lowest piece
pub struct PieceHeight;
impl<const W: usize, const H: usize, B: BitBoard> SinglePlayerInvariant<Konane256<W, H, B>>
    for PieceHeight
{
    fn compute(&self, game: B) -> f64 {
        let Some(first_set_ind) = game.first_set() else {
            return 0f64;
        };
        let first = first_set_ind / W;
        let last = game.last_set().unwrap_or(0) / W;
        (last - first + 1) as f64
    }
}

pub struct PieceCount;
impl<const W: usize, const H: usize, B: BitBoard> SinglePlayerInvariant<Konane256<W, H, B>>
    for PieceCount
{
    fn compute(&self, game: B) -> f64 {
        game.count_set() as f64
    }
}

/// distance between the first and last column with at least one peice
pub struct PieceWidth;
impl<const W: usize, const H: usize, B: BitBoard> SinglePlayerInvariant<Konane256<W, H, B>>
    for PieceWidth
{
    fn compute(&self, game: B) -> f64 {
        let mut row_mask: B = B::empty();
        for y in 0..H {
            row_mask |= B::one() << (W * y);
        }

        let mut first_last = None;
        for x in 0..W {
            if row_mask.clone() & &game != B::empty() {
                if let Some((_, last)) = &mut first_last {
                    *last = x;
                } else {
                    first_last = Some((x, x))
                }
            }
            row_mask <<= 1;
        }

        first_last
            .map(|(first, last)| last - first + 1)
            .unwrap_or(0) as f64
    }
}

pub struct MoveCount<const W: usize, const H: usize, B: BitBoard, const IS_WHITE: bool>(
    PhantomData<B>,
);

impl<const W: usize, const H: usize, B: BitBoard> MoveCount<W, H, B, false> {
    pub fn left() -> Self {
        Self(PhantomData)
    }
}

impl<const W: usize, const H: usize, B: BitBoard> MoveCount<W, H, B, true> {
    pub fn right() -> Self {
        Self(PhantomData)
    }
}
impl<const W: usize, const H: usize, B: BitBoard, const IS_WHITE: bool>
    Invariant<Konane256<W, H, B>> for MoveCount<W, H, B, IS_WHITE>
{
    fn compute(&self, game: Konane256<W, H, B>) -> f64 {
        let mut sum = 0;

        let mut l = game.move_generator::<IS_WHITE, _>(Left);
        let mut r = game.move_generator::<IS_WHITE, _>(Right);
        let mut u = game.move_generator::<IS_WHITE, _>(Up);
        let mut d = game.move_generator::<IS_WHITE, _>(Down);
        while !l.is_complete() || !r.is_complete() || !u.is_complete() || !d.is_complete() {
            l.advance();
            r.advance();
            u.advance();
            d.advance();
            sum += l.moves.count_set();
            sum += r.moves.count_set();
            sum += u.moves.count_set();
            sum += d.moves.count_set();
        }

        sum as f64
    }
}

pub struct CaptureCount<const W: usize, const H: usize, B: BitBoard, const IS_WHITE: bool>(
    PhantomData<B>,
);

impl<const W: usize, const H: usize, B: BitBoard> CaptureCount<W, H, B, false> {
    pub fn left() -> Self {
        Self(PhantomData)
    }
}

impl<const W: usize, const H: usize, B: BitBoard> CaptureCount<W, H, B, true> {
    pub fn right() -> Self {
        Self(PhantomData)
    }
}

impl<const W: usize, const H: usize, B: BitBoard, const IS_WHITE: bool>
    Invariant<Konane256<W, H, B>> for CaptureCount<W, H, B, IS_WHITE>
{
    fn compute(&self, game: Konane256<W, H, B>) -> f64 {
        let mut l = game.move_generator::<IS_WHITE, _>(Left);
        let mut r = game.move_generator::<IS_WHITE, _>(Right);
        let mut u = game.move_generator::<IS_WHITE, _>(Up);
        let mut d = game.move_generator::<IS_WHITE, _>(Down);

        // game where we run all given moves for the current player
        let mut all_captured = game.clone();
        while !l.is_complete() || !r.is_complete() || !u.is_complete() || !d.is_complete() {
            l.advance();
            r.advance();
            u.advance();
            d.advance();
            all_captured = l.move_iter().fold(all_captured, |c, m| m.apply(c));
            all_captured = r.move_iter().fold(all_captured, |c, m| m.apply(c));
            all_captured = u.move_iter().fold(all_captured, |c, m| m.apply(c));
            all_captured = d.move_iter().fold(all_captured, |c, m| m.apply(c));
        }

        let capture_count = if IS_WHITE {
            game.black.count_set() - all_captured.black.count_set()
        } else {
            game.white.count_set() - all_captured.white.count_set()
        };

        capture_count as f64
    }
}

#[cfg(test)]
mod test {
    use crate::{
        invariant::{CaptureCount, Invariant},
        Konane256,
    };

    use super::{ImpartialInvariant, PieceHeight, PieceWidth};

    #[test]
    pub fn partizan_size() {
        let game = Konane256::<16, 16>::must_parse(
            r#"
            __x_o__
            ____xo_
        "#,
        );

        let h = ImpartialInvariant::new(PieceHeight).compute(game.clone());
        let w = ImpartialInvariant::new(PieceWidth).compute(game);
        assert_eq!(h, 2f64);
        assert_eq!(w, 4f64);
    }

    #[test]
    pub fn capture_count() {
        let game = Konane256::<16, 16>::must_parse(
            r#"
            __x_o__
            ____xo_
        "#,
        );

        let l = CaptureCount::left().compute(game.clone());
        let r = CaptureCount::right().compute(game.clone());
        assert_eq!(r, 1f64);
        assert_eq!(l, 1f64);

        let game = Konane256::<16, 16>::must_parse(
            r#"
            __xo__
            ___xo_
        "#,
        );

        let l = CaptureCount::left().compute(game.clone());
        let r = CaptureCount::right().compute(game.clone());
        assert_eq!(r, 2f64);
        assert_eq!(l, 2f64);
    }
}
