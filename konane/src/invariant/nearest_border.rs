use crate::{BitBoard, BoardGeometry, Konane};

use super::SinglePlayerInvariant;

pub struct NearestBorder;

impl<G: BoardGeometry, B: BitBoard> SinglePlayerInvariant<Konane<G, B>> for NearestBorder {
    fn compute(&self, (geom, board): (&G, &B)) -> f64 {
        let w = geom.width();
        let h = geom.height();
        (0..B::BIT_LENGTH)
            .filter(|&index| B::one() << index & board != B::empty())
            .map(|piece| {
                let y = piece / w;
                let x = piece - w * y;
                let x_dist = if x > w / 2 { w - x } else { x };
                let y_dist = if y > h { h - y } else { y };
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
