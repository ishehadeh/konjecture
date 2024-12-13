use std::str::FromStr;

use thiserror::Error;

use crate::{
    bitboard::Direction,
    const_direction::{ConstDirection, Down, Left, Right, Up},
    BitBoard, TileState,
};

pub trait BoardGeometry: Clone + std::fmt::Debug + PartialEq + Eq {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

impl BoardGeometry for (usize, usize) {
    fn width(&self) -> usize {
        self.0
    }

    fn height(&self) -> usize {
        self.1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, PartialOrd, Ord, Hash)]
pub struct StaticBoard<const W: usize, const H: usize>;
impl<const W: usize, const H: usize> BoardGeometry for StaticBoard<W, H> {
    fn width(&self) -> usize {
        W
    }

    fn height(&self) -> usize {
        H
    }
}

pub fn border_mask<B: BitBoard, G: BoardGeometry>(geom: &G, mut base: B, dir: Direction) -> B {
    match dir {
        Direction::Up => {
            for i in 0..geom.width() {
                base.set(i)
            }
        }
        Direction::Down => {
            for i in geom.width() * (geom.height() - 1)..geom.width() * geom.height() {
                base.set(i)
            }
        }
        Direction::Right => {
            for i in 1..=geom.height() {
                base.set((geom.width() - 1) * i)
            }
        }
        Direction::Left => {
            for i in 0..geom.height() {
                base.set(geom.width() * i)
            }
        }
    }

    base
}

pub fn shift_in_direction<Dir: ConstDirection, G: BoardGeometry, B: BitBoard>(
    geom: &G,
    board: &mut B,
) {
    match Dir::VALUE {
        Direction::Right => *board <<= 1,
        Direction::Left => *board >>= 1,
        Direction::Up => *board >>= geom.width(),
        Direction::Down => *board <<= geom.width(),
    }
}

pub fn bit_offset_of_direction_abs<Dir: ConstDirection, G: BoardGeometry>(geom: &G) -> usize {
    match Dir::VALUE {
        Direction::Right => 1,
        Direction::Left => 1,
        Direction::Up => geom.width(),
        Direction::Down => geom.width(),
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Konane<G: BoardGeometry = (usize, usize), B: BitBoard = u64> {
    pub geometry: G,
    pub white: B,
    pub black: B,
}

impl<G: BoardGeometry, B: BitBoard> std::fmt::Debug for Konane<G, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Konane<{}> {{", std::any::type_name::<B>())?;
        for y in 0..self.height() {
            write!(f, "   ")?;
            for x in 0..self.width() {
                match self.get_tile(x, y) {
                    TileState::White => write!(f, "x")?,
                    TileState::Black => write!(f, "o")?,
                    TileState::Empty => write!(f, "_")?,
                }
            }
            writeln!(f, "")?;
        }
        writeln!(f, "}}")
    }
}

impl<const W: usize, const H: usize, B: BitBoard> Konane<StaticBoard<W, H>, B> {
    pub fn must_parse<S: AsRef<str>>(str: S) -> Self {
        FromStr::from_str(str.as_ref()).expect("failed to parse board")
    }
}

impl<G: BoardGeometry, B: BitBoard> Konane<G, B> {
    pub fn empty(geometry: G) -> Self {
        assert!(geometry.width() * geometry.height() <= B::BIT_LENGTH);

        Self {
            geometry,
            white: B::empty(),
            black: B::empty(),
        }
    }

    pub fn checkerboard(geometry: G) -> Self {
        let mut board = Self::empty(geometry);
        for x in 0..board.width() {
            for y in 0..board.height() {
                let tile = if (y + x) % 2 == 0 {
                    TileState::Black
                } else {
                    TileState::White
                };
                board.set_tile(x, y, tile);
            }
        }
        board
    }

    pub fn width(&self) -> usize {
        self.geometry.width()
    }

    pub fn height(&self) -> usize {
        self.geometry.height()
    }

    pub fn xy_to_idx(&self, x: usize, y: usize) -> usize {
        y * self.width() + x
    }

    pub fn set_tile(&mut self, x: usize, y: usize, state: TileState) {
        assert!(x < self.width());
        assert!(y < self.height());

        let i = self.xy_to_idx(x, y);
        match state {
            TileState::White => {
                self.white.set(i);
                self.black.clear(i);
            }
            TileState::Black => {
                self.black.set(i);
                self.white.clear(i);
            }
            TileState::Empty => {
                self.white.clear(i);
                self.black.clear(i);
            }
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> TileState {
        let i = self.xy_to_idx(x, y);
        match (self.black.get(i), self.white.get(i)) {
            (true, true) => panic!("Tile at <{}, {}> is marked for both black and white", x, y),
            (false, false) => TileState::Empty,
            (true, false) => TileState::Black,
            (false, true) => TileState::White,
        }
    }

    pub fn empty_spaces(&self) -> B {
        // get empty by selecting non-black spaces that don't have a white piece.
        // and clear extra bits
        !(self.black.clone()
            | &self.white
            | if self.width() * self.height() < B::BIT_LENGTH {
                // necessary to avoid overflow panics
                B::all() << (self.width() * self.height())
            } else {
                B::empty()
            })
    }

    pub fn move_bitmap<const WHITE: bool, Dir: ConstDirection>(&self) -> MoveBitmap<B> {
        MoveBitmap::new_from_game_in_dir::<WHITE, G, Dir>(self)
    }

    pub fn move_iter<const WHITE: bool>(&self) -> MoveIter<'_, WHITE, G, B> {
        let mut iter = MoveIter {
            game: self,
            dir: Direction::Up,
            gen: self.move_bitmap::<WHITE, Up>(),
            iter: unsafe { std::mem::MaybeUninit::zeroed().assume_init() },
        };

        iter.gen.advance_against::<WHITE, Up, _>(self);
        iter.reset_iter();
        iter
    }

    pub fn all_moves_black(&self) -> Vec<Self> {
        self.move_iter::<false>().collect()
    }

    pub fn all_moves_white(&self) -> Vec<Self> {
        self.move_iter::<true>().collect()
    }

    pub fn svg<W: std::io::Write>(&self, out: &mut W) -> std::io::Result<()> {
        // content of viewBox property, every grid cell is 100x100
        let circle_stroke_width = 4;
        let grid_stroke_width = 4;
        let base_width = self.width() * 100;
        let base_height = self.height() * 100;
        let view_box = format!(
            "0 0 {} {}",
            base_width + grid_stroke_width,
            base_height + grid_stroke_width
        );
        let width = format!("{}em", self.width() + 2);
        let height = format!("{}em", self.height() + 2);

        // define grid pattern
        write!(
            out,
            r#"
<svg xmlns="http://www.w3.org/2000/svg" version="1.1" xmlns:xlink="http://www.w3.org/1999/xlink" xmlns:svgjs="http://svgjs.dev/svgjs" viewBox="{view_box}" width="{width}" height="{height}">
    <defs>
        <pattern id="grid" width="100" height="100" patternUnits="userSpaceOnUse">
            <path d="M 100 0 L 0 0 0 100" fill="none" stroke="black" stroke-width="{grid_stroke_width}"/>
        </pattern>
    </defs>
"#
        )?;

        let mut stone = |x: usize, y: usize, color: &str| {
            write!(
                out,
                r#"<circle cx="{svg_x}" cy="{svg_y}" r="38" fill="{color}" stroke="black" stroke-width="{circle_stroke_width}" />"#,
                svg_x = x * 100 + 50,
                svg_y = y * 100 + 50
            )
        };

        for y in 0..self.height() {
            for x in 0..self.width() {
                match self.get_tile(x, y) {
                    TileState::White => stone(x, y, "white")?,
                    TileState::Black => stone(x, y, "black")?,
                    TileState::Empty => (),
                }
            }
        }

        write!(
            out,
            r#"<rect width="100%" height="100%" fill="url(#grid)" /></svg>"#
        )
    }
}

pub struct MoveIter<'a, const WHITE: bool, G: BoardGeometry, B: BitBoard> {
    game: &'a Konane<G, B>,
    dir: Direction,
    gen: MoveBitmap<B>,
    // not actually 'a, but references gen
    iter: B::Iter<'a>,
}

impl<'a, const WHITE: bool, G: BoardGeometry, B: BitBoard> MoveIter<'a, WHITE, G, B> {
    fn reset_iter(&mut self) {
        self.iter = unsafe { std::mem::transmute(self.gen.moves.iter_set()) };
    }
}

impl<'a, const WHITE: bool, G: BoardGeometry, B: BitBoard> Iterator for MoveIter<'a, WHITE, G, B> {
    type Item = Konane<G, B>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.iter.next();
        match next {
            Some(ind) => {
                let new_game = self
                    .gen
                    .apply_move_to_dyn::<WHITE, _>(self.game, self.dir, ind);
                Some(new_game)
            }
            None => {
                if self.gen.is_complete() {
                    match self.dir {
                        Direction::Up => {
                            self.dir = Direction::Down;
                            self.gen =
                                MoveBitmap::new_from_game_in_dir::<WHITE, _, Down>(self.game);
                        }
                        Direction::Down => {
                            self.dir = Direction::Left;
                            self.gen =
                                MoveBitmap::new_from_game_in_dir::<WHITE, _, Left>(self.game);
                        }
                        Direction::Left => {
                            self.dir = Direction::Right;
                            self.gen =
                                MoveBitmap::new_from_game_in_dir::<WHITE, _, Right>(self.game);
                        }
                        Direction::Right => return None,
                    }
                }

                self.gen
                    .advance_against_dyn::<WHITE, _>(self.game, self.dir);
                self.reset_iter();
                self.next()
            }
        }
    }
}

#[derive(Error, Debug, Clone)]
pub enum KonaneParseError {
    #[error("expected one of 'x', 'o', or '_', got '{c}'")]
    UnexpectedCharacter { c: char },

    #[error("tile at <{x}, {y}> '{c}', is out of bounds for board of width {w} and height {h}")]
    OutOfBounds {
        c: char,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
    },
}

impl<B: BitBoard> FromStr for Konane<(usize, usize), B> {
    fn from_str(s: &str) -> Result<Self, KonaneParseError> {
        let row_iter = s.trim().split("\n").map(|row| row.trim());
        let w = row_iter.clone().map(|r| r.len()).max().unwrap_or(1);
        let h = row_iter.clone().count().max(1);

        let mut game = Self::empty((w, h));

        for (y, row_txt) in row_iter.enumerate() {
            for (x, c) in row_txt.chars().enumerate() {
                match c {
                    'x' => game.set_tile(x, y, TileState::White),
                    'o' => game.set_tile(x, y, TileState::Black),
                    '_' => game.set_tile(x, y, TileState::Empty),
                    c => return Err(KonaneParseError::UnexpectedCharacter { c }),
                }
            }
        }

        Ok(game)
    }

    type Err = KonaneParseError;
}

impl<const W: usize, const H: usize, B: BitBoard> FromStr for Konane<StaticBoard<W, H>, B> {
    fn from_str(s: &str) -> Result<Self, KonaneParseError> {
        let row_iter = s.trim().split("\n").map(|row| row.trim());
        let mut game = Self::empty(Default::default());

        for (y, row_txt) in row_iter.enumerate() {
            for (x, c) in row_txt.chars().enumerate() {
                if x >= W || y >= H {
                    return Err(KonaneParseError::OutOfBounds {
                        c,
                        x,
                        y,
                        w: W,
                        h: H,
                    });
                }
                match c {
                    'x' => game.set_tile(x, y, TileState::White),
                    'o' => game.set_tile(x, y, TileState::Black),
                    '_' => game.set_tile(x, y, TileState::Empty),
                    c => return Err(KonaneParseError::UnexpectedCharacter { c }),
                }
            }
        }

        Ok(game)
    }

    type Err = KonaneParseError;
}

#[derive(Debug)]
pub struct MoveBitmap<B: BitBoard> {
    pub moves: B,
    pub offset: usize,
}

impl<B: BitBoard> MoveBitmap<B> {
    pub fn is_complete(&self) -> bool {
        self.moves == B::empty()
    }

    pub fn new_from_game_in_dir<const FROM_WHITE: bool, G: BoardGeometry, Dir: ConstDirection>(
        game: &Konane<G, B>,
    ) -> Self {
        let mut moves: B = border_mask(&game.geometry, B::empty(), Dir::VALUE);
        moves = !moves;
        moves &= if FROM_WHITE { &game.white } else { &game.black };

        Self { moves, offset: 0 }
    }

    pub fn apply_move_to_dyn<const TO_WHITE: bool, G: BoardGeometry>(
        &mut self,
        g: &Konane<G, B>,
        dir: Direction,
        ind: usize,
    ) -> Konane<G, B> {
        match dir {
            Direction::Up => self.apply_move_to::<TO_WHITE, _, Up>(g, ind),
            Direction::Down => self.apply_move_to::<TO_WHITE, _, Down>(g, ind),
            Direction::Left => self.apply_move_to::<TO_WHITE, _, Left>(g, ind),
            Direction::Right => self.apply_move_to::<TO_WHITE, _, Right>(g, ind),
        }
    }

    pub fn advance_against_dyn<const AGAINST_BLACK: bool, G: BoardGeometry>(
        &mut self,
        g: &Konane<G, B>,
        dir: Direction,
    ) {
        match dir {
            Direction::Up => self.advance_against::<AGAINST_BLACK, Up, _>(g),
            Direction::Down => self.advance_against::<AGAINST_BLACK, Down, _>(g),
            Direction::Left => self.advance_against::<AGAINST_BLACK, Left, _>(g),
            Direction::Right => self.advance_against::<AGAINST_BLACK, Right, _>(g),
        }
    }

    pub fn advance_against<const AGAINST_BLACK: bool, Dir: ConstDirection, G: BoardGeometry>(
        &mut self,
        g: &Konane<G, B>,
    ) {
        if self.moves == B::empty() {
            return;
        }

        // 1. verify that there's a capture-able adjacent piece
        shift_in_direction::<Dir, G, B>(&g.geometry, &mut self.moves);
        if AGAINST_BLACK {
            self.moves &= &g.black;
        } else {
            self.moves &= &g.white;
        }

        // 2. verify there's an empty space after the piece to be jumped
        shift_in_direction::<Dir, G, B>(&g.geometry, &mut self.moves);
        self.moves &= g.empty_spaces();

        self.offset += 1;
    }

    pub fn get_origin_of<G: BoardGeometry, Dir: ConstDirection>(
        &self,
        geom: &G,
        ind: usize,
    ) -> usize {
        match Dir::VALUE {
            Direction::Right => ind - 2 * self.offset,
            Direction::Left => ind + 2 * self.offset,
            Direction::Up => ind + geom.width() * 2 * self.offset,
            Direction::Down => ind - geom.width() * 2 * self.offset,
        }
    }
    pub fn apply_move_to_mut<const TO_WHITE: bool, G: BoardGeometry, Dir: ConstDirection>(
        &self,
        g: &mut Konane<G, B>,
        ind: usize,
    ) {
        let step = bit_offset_of_direction_abs::<Dir, G>(&g.geometry);
        let origin = self.get_origin_of::<G, Dir>(&g.geometry, ind);
        let start = ind.min(origin);
        let end = ind.max(origin);
        let mut i = start;
        while i <= end {
            g.black.clear(i);
            g.white.clear(i);
            i += step;
        }
        if TO_WHITE {
            g.white.set(ind)
        } else {
            g.black.set(ind)
        }
    }

    pub fn apply_move_to<const TO_WHITE: bool, G: BoardGeometry, Dir: ConstDirection>(
        &self,
        g: &Konane<G, B>,
        ind: usize,
    ) -> Konane<G, B> {
        let mut new_game = g.clone();
        self.apply_move_to_mut::<TO_WHITE, G, Dir>(&mut new_game, ind);
        new_game
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use bitarray::BitArray;

    use crate::{Konane, TileState};

    #[test]
    pub fn move_over_block_boundary() {
        let board: Konane<(usize, usize), u128> = Konane::from_str(
            r#"_oxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxox_"#,
        )
        .unwrap();

        let w: Konane<(usize, usize), u128> = Konane::from_str(
            r#"x__oxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxox_"#,
        )
        .unwrap();
        let b: Konane<(usize, usize), u128> = Konane::from_str(
            r#"_oxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxoxox__o"#,
        )
        .unwrap();
        let w_moves: Vec<_> = board.move_iter::<true>().collect();
        let b_moves: Vec<_> = board.move_iter::<false>().collect();
        assert_eq!(w_moves, vec![w]);
        assert_eq!(b_moves, vec![b]);
    }

    #[test]
    pub fn moveset_on_full_board_is_empty_16x16() {
        let board: Konane<(usize, usize), BitArray<4, u64>> = Konane::empty((16, 16));

        assert_eq!(board.move_iter::<false>().next(), None);
        assert_eq!(board.move_iter::<true>().next(), None);
    }

    #[test]
    pub fn moveset_one_piece() {
        let mut board: Konane<(usize, usize), BitArray<4, u64>> = Konane::empty((16, 16));
        board.set_tile(3, 3, TileState::Black);
        assert_eq!(board.move_iter::<false>().next(), None);
        assert_eq!(board.move_iter::<true>().next(), None);
    }
}
