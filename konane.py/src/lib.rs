use ::konane::{Konane, Konane256, TileState};
use cgt::short::partizan::canonical_form::CanonicalForm;
use cgt::short::partizan::{
    partizan_game::PartizanGame, transposition_table::ParallelTranspositionTable,
};

use pyo3::prelude::*;

pub type Konane8x8 = Konane256<8, 8, u64>;

#[macro_export]
macro_rules! wrap_struct {
    ($struct:path, $py_struct:ident, $py_class:expr $(, $trait:tt)*) => {
        #[derive($($trait),*)]
        #[pyclass(name = $py_class)]
        #[repr(transparent)]
        pub struct $py_struct {
            inner: $struct,
        }

        impl From<$struct> for $py_struct {
            fn from(inner: $struct) -> Self {
                $py_struct { inner }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_py_partizan_game {
    ($game_str:expr, $game:path, $py_game:ident, $tt_str:expr, $tt:path, $py_tt:ident, $bitboard:path) => {
        crate::wrap_struct!($tt, $py_tt, $tt_str, Default);
        crate::wrap_struct!($game, $py_game, $game_str, Clone);

        #[pymethods]
        impl $py_game {
            #[new]
            fn py_new(position: &str) -> PyResult<Self> {
                let inner = <$game as std::str::FromStr>::from_str(position)
                    .or(Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                        "Parse error",
                    )))?;
                Ok(Self::from(inner))
            }



            #[getter]
            fn width(&self) -> usize {
                self.inner.width()
            }

            #[getter]
            fn height(&self) -> usize {
                self.inner.height()
            }

            fn __repr__(&self) -> String {
                format!("{}('{:?}')", stringify!($game), self.inner)
            }


            fn _repr_svg_(&self) -> String {
                // content of viewBox property, every grid cell is 100x100
                let circle_stroke_width = 4;
                let grid_stroke_width = 4;
                let base_width = self.width() * 100;
                let base_height = self.height() * 100;
                let view_box = format!("0 0 {} {}", base_width + grid_stroke_width, base_height + grid_stroke_width);
                let width = format!("{}em", self.width() + 2);
                let height = format!("{}em", self.height() + 2);

                // define grid pattern
                let defs = format!(r#"
                    <defs>
                        <pattern id="grid" width="100" height="100" patternUnits="userSpaceOnUse">
                            <path d="M 100 0 L 0 0 0 100" fill="none" stroke="black" stroke-width="{grid_stroke_width}"/>
                        </pattern>
                    </defs>
                "#);

                let stone = |x:usize, y:usize, color: &str| format!(r#"<circle cx="{svg_x}" cy="{svg_y}" r="38" fill="{color}" stroke="black" stroke-width="{circle_stroke_width}" />"#, svg_x=x * 100 + 50, svg_y=y* 100 + 50);
                let mut stones_svg = String::new();

                for y in 0..self.height() {
                    for x in 0..self.width() {
                        match self.inner.get_tile(x, y) {
                            TileState::White => {
                                stones_svg.push_str(&stone(x, y, "white"))
                            }
                            TileState::Black => {
                                stones_svg.push_str(&stone(x, y, "black"))
                            }
                            TileState::Empty => (),
                        }
                    }
                }

                format!(r#"
                <svg xmlns="http://www.w3.org/2000/svg" version="1.1" xmlns:xlink="http://www.w3.org/1999/xlink" xmlns:svgjs="http://svgjs.dev/svgjs" viewBox="{view_box}" width="{width}" height="{height}">
                    {defs}
                    {stones_svg}
                    <rect width="100%" height="100%" fill="url(#grid)" />
                </svg>"#)
            }

            #[staticmethod]
            fn transposition_table() -> $py_tt {
                $py_tt::default()
            }

            #[pyo3(signature = (transposition_table=None))]
            fn canonical_form(&self, transposition_table: Option<&$py_tt>) -> String {
                let canon = match transposition_table {
                    Some(transposition_table) => {
                        CanonicalForm::from(self.inner.canonical_form(&transposition_table.inner))
                    }
                    None => CanonicalForm::from(
                        self.inner
                            .canonical_form(&Self::transposition_table().inner),
                    ),
                };
                canon.to_string()
            }

            fn left_moves(&self) -> Vec<Self> {
                self.inner
                    .left_moves()
                    .into_iter()
                    .map(Self::from)
                    .collect()
            }

            fn right_moves(&self) -> Vec<Self> {
                self.inner
                    .right_moves()
                    .into_iter()
                    .map(Self::from)
                    .collect()
            }
        }
    };
}

impl_py_partizan_game!(
    "Konane64",
    Konane<(usize, usize), u64>,
    Konane64Py,
    "ParallelTranspositionTableKonane",
    ParallelTranspositionTable<Konane<(usize, usize), u64>>,
    TTKonane64Py,
    u64
);

impl_py_partizan_game!(
    "Konane1024",
    Konane<(usize, usize), bnum::BUint<16>>,
    Konane1024Py,
    "ParallelTranspositionTableKonane",
    ParallelTranspositionTable<Konane<(usize, usize), bnum::BUint<16>>>,
    TTKonane1024Py,
    BitArray<16, u64>
);

#[pymethods]
impl Konane1024Py {
    #[staticmethod]
    fn from_bitmaps(width: usize, height: usize, black: &[u8], white: &[u8]) -> PyResult<Self> {
        let mut inner: Konane<(usize, usize), bnum::BUint<16>> = Konane::empty((width, height));
        assert_eq!(black.len(), white.len());
        for i in 0..black.len() / 8 {
            inner.black.digits_mut()[15 - i] = u64::from_ne_bytes(
                black[(white.len() - (i + 1) * 8)..(white.len() - i * 8)]
                    .try_into()
                    .unwrap(),
            );
            inner.white.digits_mut()[15 - i] = u64::from_ne_bytes(
                white[(white.len() - (i + 1) * 8)..(white.len() - i * 8)]
                    .try_into()
                    .unwrap(),
            );
        }

        Ok(Self::from(inner))
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn konane(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Konane64Py>()?;
    m.add_class::<Konane1024Py>()?;
    Ok(())
}
