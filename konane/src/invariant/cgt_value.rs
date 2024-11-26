use std::marker::PhantomData;

use cgt::short::partizan::{partizan_game::PartizanGame, transposition_table::TranspositionTable};

use super::{Invariant, TwoPlayerGame};

pub struct CanonicalFormNumber<'a, G: TwoPlayerGame + PartizanGame, TT>
where
    TT: TranspositionTable<G> + Sync,
{
    tt: &'a TT,
    g: PhantomData<G>,
}
impl<'a, G: TwoPlayerGame + PartizanGame, TT> CanonicalFormNumber<'a, G, TT>
where
    TT: TranspositionTable<G> + Sync,
{
    pub fn new(tt: &'a TT) -> Self {
        Self { tt, g: PhantomData }
    }
}

impl<'a, G: TwoPlayerGame + PartizanGame, TT> Invariant<G> for CanonicalFormNumber<'a, G, TT>
where
    TT: TranspositionTable<G> + Sync,
{
    fn compute(&self, game: G) -> f64 {
        let canonical_form = game.canonical_form(self.tt);
        if let Some(rat) = canonical_form.to_number() {
            (rat.numerator() as f64) / 2.0f64.powi(rat.denominator_exponent() as i32)
        } else {
            f64::NAN
        }
    }
}

pub struct CanonicalFormNimber<'a, G: TwoPlayerGame + PartizanGame, TT>
where
    TT: TranspositionTable<G> + Sync,
{
    tt: &'a TT,
    g: PhantomData<G>,
}
impl<'a, G: TwoPlayerGame + PartizanGame, TT> CanonicalFormNimber<'a, G, TT>
where
    TT: TranspositionTable<G> + Sync,
{
    pub fn new(tt: &'a TT) -> Self {
        Self { tt, g: PhantomData }
    }
}

impl<'a, G: TwoPlayerGame + PartizanGame, TT> Invariant<G> for CanonicalFormNimber<'a, G, TT>
where
    TT: TranspositionTable<G> + Sync,
{
    fn compute(&self, game: G) -> f64 {
        let canonical_form = game.canonical_form(self.tt);
        if let Some(nimber) = canonical_form.to_nus().map(|n| n.nimber().value()) {
            nimber as f64
        } else {
            0.0
        }
    }
}
