use crate::{BitBoard, BoardGeometry, Konane, Konane256};

use super::SinglePlayerInvariant;

pub struct NearestBorder;

impl<const W: usize, const H: usize, B: BitBoard> SinglePlayerInvariant<Konane256<W, H, B>>
    for NearestBorder
{
    fn compute(&self, player: B) -> f64 {
        (0..B::BIT_LENGTH)
            .filter(|&index| B::one() << index & &player != B::empty())
            .map(|piece| {
                let y = piece / W;
                let x = piece - W * y;
                let x_dist = if x > W / 2 { W - x } else { x };
                let y_dist = if y > H { H - y } else { y };
                x_dist.min(y_dist) as f64
            })
            .enumerate()
            .fold(0.0f64, |avg, (i, next)| {
                (avg * i as f64 + next) / (i as f64 + 1.0)
            })
    }
}

#[cfg(test)]
mod test {
    use crate::{
        invariant::{Invariant, NearestBorder},
        Konane256,
    };

    use crate::invariant::ImpartialInvariant;

    #[test]
    pub fn partizan_size() {
        let game = Konane256::<16, 16>::must_parse(
            r#"
            _______
            __x___
            ______
        "#,
        );

        let avg = ImpartialInvariant::new(NearestBorder).compute(game);
        assert_eq!(avg, 1.0f64);

        let game = Konane256::<16, 16>::must_parse(
            r#"
            _____
            _______
            _x______o
            ______
        "#,
        );

        let avg = ImpartialInvariant::new(NearestBorder).compute(game);
        assert_eq!(avg, 1.5f64);
    }
}
