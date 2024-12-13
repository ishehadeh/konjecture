use std::marker::PhantomData;

use crate::{
    const_direction::{Down, Left, Right, Up},
    BitBoard, BoardGeometry, Konane,
};

// Invariant Submodules
pub use nearest_border::*;
mod nearest_border;

#[cfg(feature = "cgt")]
mod cgt_value;
#[cfg(feature = "cgt")]
pub use cgt_value::*;

/// A game which can be split into seperate structures representing each player individually.
pub trait TwoPlayerGame {
    type B<'a>
    where
        Self: 'a;

    fn left<'a>(&'a self) -> Self::B<'a>;
    fn right<'a>(&'a self) -> Self::B<'a>;
}

impl<G: BoardGeometry, B: BitBoard> TwoPlayerGame for Konane<G, B> {
    type B<'a> = (&'a G, &'a B) where G: 'a, B: 'a;

    fn left(&self) -> Self::B<'_> {
        (&self.geometry, &self.black)
    }

    fn right(&self) -> Self::B<'_> {
        (&self.geometry, &self.white)
    }
}

pub trait SinglePlayerInvariant<G: TwoPlayerGame> {
    fn compute(&self, player: G::B<'_>) -> f64;
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

impl<'a, G: TwoPlayerGame, T: SinglePlayerInvariant<G>> Invariant<G>
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

impl<G: BoardGeometry, B: BitBoard, T> Invariant<Konane<G, B>>
    for ImpartialInvariant<Konane<G, B>, T>
where
    T: SinglePlayerInvariant<Konane<G, B>>,
{
    #[inline(always)]
    fn compute(&self, game: Konane<G, B>) -> f64 {
        let board = game.white.clone() | &game.black;
        self.inner.compute((&game.geometry, &board))
    }
}

/// Vertical distance between the highest and lowest piece
pub struct PieceHeight;
impl<G: BoardGeometry, B: BitBoard> SinglePlayerInvariant<Konane<G, B>> for PieceHeight {
    fn compute(&self, (geom, board): (&G, &B)) -> f64 {
        let Some(first_set_ind) = board.first_set() else {
            return 0f64;
        };
        let first = first_set_ind / geom.width();
        let last = board.last_set().unwrap_or(0) / geom.width();
        (last - first + 1) as f64
    }
}

pub struct PieceCount;
impl<G: BoardGeometry, B: BitBoard> SinglePlayerInvariant<Konane<G, B>> for PieceCount {
    fn compute(&self, (_, board): (&G, &B)) -> f64 {
        board.count_set() as f64
    }
}

/// distance between the first and last column with at least one peice
pub struct PieceWidth;
impl<G: BoardGeometry, B: BitBoard> SinglePlayerInvariant<Konane<G, B>> for PieceWidth {
    fn compute(&self, (geom, board): (&G, &B)) -> f64 {
        let mut row_mask: B = B::empty();
        for y in 0..geom.height() {
            row_mask |= B::one() << (geom.width() * y);
        }

        let mut first_last = None;
        for x in 0..geom.width() {
            if row_mask.clone() & board != B::empty() {
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

pub struct MoveCount<G: BoardGeometry, B: BitBoard, const IS_WHITE: bool>(PhantomData<(B, G)>);

impl<G: BoardGeometry, B: BitBoard> MoveCount<G, B, false> {
    pub fn left() -> Self {
        Self(PhantomData)
    }
}

impl<G: BoardGeometry, B: BitBoard> MoveCount<G, B, true> {
    pub fn right() -> Self {
        Self(PhantomData)
    }
}
impl<G: BoardGeometry, B: BitBoard, const IS_WHITE: bool> Invariant<Konane<G, B>>
    for MoveCount<G, B, IS_WHITE>
{
    fn compute(&self, game: Konane<G, B>) -> f64 {
        let mut sum = 0;

        let mut l = game.move_bitmap::<IS_WHITE, Left>();
        let mut r = game.move_bitmap::<IS_WHITE, Right>();
        let mut u = game.move_bitmap::<IS_WHITE, Up>();
        let mut d = game.move_bitmap::<IS_WHITE, Down>();
        while !l.is_complete() || !r.is_complete() || !u.is_complete() || !d.is_complete() {
            l.advance_against::<IS_WHITE, Left, G>(&game);
            r.advance_against::<IS_WHITE, Right, G>(&game);
            u.advance_against::<IS_WHITE, Up, G>(&game);
            d.advance_against::<IS_WHITE, Up, G>(&game);
            sum += l.moves.count_set();
            sum += r.moves.count_set();
            sum += u.moves.count_set();
            sum += d.moves.count_set();
        }

        sum as f64
    }
}

pub struct CaptureCount<G: BoardGeometry, B: BitBoard, const IS_WHITE: bool>(PhantomData<(B, G)>);

impl<G: BoardGeometry, B: BitBoard> CaptureCount<G, B, false> {
    pub fn left() -> Self {
        Self(PhantomData)
    }
}

impl<G: BoardGeometry, B: BitBoard> CaptureCount<G, B, true> {
    pub fn right() -> Self {
        Self(PhantomData)
    }
}

impl<G: BoardGeometry, B: BitBoard, const IS_WHITE: bool> Invariant<Konane<G, B>>
    for CaptureCount<G, B, IS_WHITE>
{
    fn compute(&self, game: Konane<G, B>) -> f64 {
        let mut l = game.move_bitmap::<IS_WHITE, Left>();
        let mut r = game.move_bitmap::<IS_WHITE, Right>();
        let mut u = game.move_bitmap::<IS_WHITE, Up>();
        let mut d = game.move_bitmap::<IS_WHITE, Down>();

        // game where we run all given moves for the current player
        let mut all_captured = game.clone();
        while !l.is_complete() || !r.is_complete() || !u.is_complete() || !d.is_complete() {
            l.advance_against::<IS_WHITE, Left, G>(&game);
            r.advance_against::<IS_WHITE, Right, G>(&game);
            u.advance_against::<IS_WHITE, Up, G>(&game);
            d.advance_against::<IS_WHITE, Up, G>(&game);

            for new_pos in l.moves.iter_set() {
                l.apply_move_to_mut::<IS_WHITE, G, Left>(&mut all_captured, new_pos);
            }
            for new_pos in r.moves.iter_set() {
                l.apply_move_to_mut::<IS_WHITE, G, Right>(&mut all_captured, new_pos);
            }
            for new_pos in u.moves.iter_set() {
                l.apply_move_to_mut::<IS_WHITE, G, Up>(&mut all_captured, new_pos);
            }
            for new_pos in d.moves.iter_set() {
                l.apply_move_to_mut::<IS_WHITE, G, Down>(&mut all_captured, new_pos);
            }
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
