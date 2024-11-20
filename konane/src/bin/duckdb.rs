use std::fmt::Write;

use cgt::short::partizan::transposition_table::ParallelTranspositionTable;
use cgt::short::partizan::{canonical_form::CanonicalForm, partizan_game::PartizanGame};
use duckdb::{types::Value, Connection};
use itertools::Itertools;
use konane::{
    bitboard::{BitBoard256, Direction},
    Konane256, TileState,
};
use thiserror::Error;

const W: usize = 11;
const H: usize = 11;

fn init() {
    const W: usize = 11;
    const H: usize = 11;
    let block_w = 4;
    let block_h = 4;

    let conn = Connection::open("konane.duckdb").expect("failed to open duckdb connection");

    conn.execute_batch(
        r"
            CREATE TABLE IF NOT EXISTS konane(
                black UHUGEINT,
                white UHUGEINT,
                
                PRIMARY KEY(black, white),
                UNIQUE(black, white));

            CREATE TABLE IF NOT EXISTS moves(
                is_left BOOLEAN,
                from_white UHUGEINT, from_black UHUGEINT,
                to_white UHUGEINT, to_black UHUGEINT);
         ",
    )
    .expect("failed to create schema");
    let existing = conn
        .query_row("SELECT count(*) FROM konane", [], |row| {
            row.get::<_, usize>(0)
        })
        .unwrap();

    let tile_values = [TileState::Black, TileState::White, TileState::Empty];
    let mut appender = conn.appender("konane").expect("failed to create appender");
    let y_pos_iter = (H - block_h) / 2..(H - block_h) / 2 + block_h;
    let x_pos_iter = (W - block_w) / 2..(W - block_w) / 2 + block_w;
    println!(
        "completed so far: {}/{}",
        existing,
        (0..block_w * block_h)
            .map(|_| tile_values.iter())
            .multi_cartesian_product()
            .count()
    );

    (0..block_w * block_h)
        .map(|_| tile_values.iter())
        .multi_cartesian_product()
        .skip(existing)
        .for_each(|v| {
            let mut game = Konane256::<W, H>::empty();
            let mut i = 0;
            for x in x_pos_iter.clone() {
                for y in y_pos_iter.clone() {
                    game.set_tile(x, y, *v[i]);
                    i += 1;
                }
            }
            let black_blocks = game.black.board.blocks();
            let white_blocks = game.white.board.blocks();
            appender
                .append_row([
                    (black_blocks[2] as i128) << 64 | black_blocks[3] as i128,
                    (white_blocks[2] as i128) << 64 | white_blocks[3] as i128,
                ])
                .unwrap()
        });
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to create appender for column '{column}': {source}")]
    CreateAppenderError {
        column: String,
        #[source]
        source: duckdb::Error,
    },

    #[error("failed to append game: {source}\ngame = {game:?}")]
    AppendGameError {
        game: Konane256<W, H>,
        #[source]
        source: duckdb::Error,
    },

    #[error("failed to append move: {source}\nfrom = {from:?}\nto = {to:?}")]
    MoveAppendError {
        from: Konane256<W, H>,
        to: Konane256<W, H>,
        #[source]
        source: duckdb::Error,
    },

    #[error("bulk append moves failed: {source}\nfrom = {from:?}")]
    BulkMoveAppendError {
        from: Konane256<W, H>,
        #[source]
        source: duckdb::Error,
    },
}

pub struct KonaneAppender<'a> {
    appender: duckdb::Appender<'a>,
}

impl<'a> KonaneAppender<'a> {
    pub fn new(conn: &'a duckdb::Connection) -> Result<KonaneAppender<'a>, Error> {
        conn.appender("konane")
            .map(|appender| Self { appender })
            .map_err(|e| Error::CreateAppenderError {
                column: "konane".to_string(),
                source: e,
            })
    }

    pub fn append(&mut self, game: &Konane256<W, H>) -> Result<(), Error> {
        let black_blocks = game.black.board.blocks();
        let white_blocks = game.white.board.blocks();
        self.appender
            .append_row([
                (black_blocks[2] as i128) << 64 | black_blocks[3] as i128,
                (white_blocks[2] as i128) << 64 | white_blocks[3] as i128,
            ])
            .map_err(|e| Error::AppendGameError {
                game: game.clone(),
                source: e,
            })
    }
}

pub struct KonaneMoveAppender<'a> {
    appender: duckdb::Appender<'a>,
}

impl<'a> KonaneMoveAppender<'a> {
    pub fn new(conn: &'a duckdb::Connection) -> Result<Self, Error> {
        conn.appender("moves")
            .map(|appender| Self { appender })
            .map_err(|e| Error::CreateAppenderError {
                column: "moves".to_string(),
                source: e,
            })
    }

    pub fn append(
        &mut self,
        is_left: bool,
        from: &Konane256<W, H>,
        to: &Konane256<W, H>,
    ) -> Result<(), Error> {
        let from_black_blocks = from.black.board.blocks();
        let from_white_blocks = from.white.board.blocks();
        let to_black_blocks = to.black.board.blocks();
        let to_white_blocks = to.white.board.blocks();
        self.appender
            .append_row([
                is_left as i128,
                (from_black_blocks[2] as i128) << 64 | from_black_blocks[3] as i128,
                (from_white_blocks[2] as i128) << 64 | from_white_blocks[3] as i128,
                (to_black_blocks[2] as i128) << 64 | to_black_blocks[3] as i128,
                (to_white_blocks[2] as i128) << 64 | to_white_blocks[3] as i128,
            ])
            .map_err(|e| Error::MoveAppendError {
                from: from.clone(),
                to: to.clone(),
                source: e,
            })
    }

    pub fn append_all(
        &mut self,
        is_left: bool,
        from: &Konane256<W, H>,
        to: &[Konane256<W, H>],
    ) -> Result<(), Error> {
        let mut rows = Vec::with_capacity(to.len());
        for to_game in to {
            let from_black_blocks = from.black.board.blocks();
            let from_white_blocks = from.white.board.blocks();
            let to_black_blocks = to_game.black.board.blocks();
            let to_white_blocks = to_game.white.board.blocks();
            rows.push([
                Value::Boolean(is_left),
                Value::HugeInt((from_black_blocks[2] as i128) << 64 | from_black_blocks[3] as i128),
                Value::HugeInt((from_white_blocks[2] as i128) << 64 | from_white_blocks[3] as i128),
                Value::HugeInt((to_black_blocks[2] as i128) << 64 | to_black_blocks[3] as i128),
                Value::HugeInt((to_white_blocks[2] as i128) << 64 | to_white_blocks[3] as i128),
            ])
        }
        self.appender
            .append_rows(rows)
            .map_err(|e| Error::BulkMoveAppendError {
                from: from.clone(),
                source: e,
            })
    }
}

pub fn get_moves_rust() {
    let conn = Connection::open("konane.duckdb").expect("failed to open duckdb connection");
    let mut stmt_collect_games = conn
        .prepare("SELECT (black, white) FROM konane")
        .expect("failed to prepare query");

    let mut move_appender = KonaneMoveAppender::new(&conn).expect("failed to create appender");
    let games = stmt_collect_games.query_arrow([]).expect("query failed");
    for game in games {
        println!("BEGIN batch ({} records)", game.num_rows());
        let data = game.column(0).to_data();
        let [black, white] = data.child_data() else {
            panic!("expected a two element struct");
        };
        for (black_buffer, white_buffer) in black.buffers().iter().zip(white.buffers().iter()) {
            let black_pos_data = black_buffer.typed_data::<u64>();
            let white_pos_data = white_buffer.typed_data::<u64>();

            println!("  BEGIN buffer ({} positions)", black_pos_data.len() / 2);
            for i in 0..(black_pos_data.len() / 2) {
                let game = Konane256::<W, H> {
                    white: BitBoard256 {
                        board: [0, 0, white_pos_data[i + 1], white_pos_data[i]].into(),
                    },

                    black: BitBoard256 {
                        board: [0, 0, black_pos_data[i + 1], black_pos_data[i]].into(),
                    },
                };
                {
                    let left = game.all_moves_black();
                    move_appender
                        .append_all(true, &game, &left)
                        .expect("failed to append left moves");
                };
                {
                    let right = game.all_moves_white();
                    move_appender
                        .append_all(true, &game, &right)
                        .expect("failed to append left moves");
                };
            }
            println!("  END buffer")
        }

        println!("END batch")
    }
}

pub struct GetMoveMaskSql {
    pub dir: Direction,
    pub is_black: bool,
    pub width: usize,
    pub hops: usize,
}

impl GetMoveMaskSql {
    pub fn dir_char(&self) -> char {
        match self.dir {
            Direction::Up => 'u',
            Direction::Down => 'd',
            Direction::Left => 'l',
            Direction::Right => 'r',
        }
    }

    pub fn next_player_char(&self) -> char {
        match self.is_black {
            true => 'b',
            false => 'w',
        }
    }

    pub fn prev_player_char(&self) -> char {
        match self.is_black {
            true => 'w',
            false => 'b',
        }
    }
}

impl std::fmt::Display for GetMoveMaskSql {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = self.next_player_char();
        let p = self.prev_player_char();
        let op = match self.dir {
            Direction::Up | Direction::Left => ">>",
            Direction::Down | Direction::Right => "<<",
        };
        let shift = match self.dir {
            Direction::Up | Direction::Down => W,
            Direction::Left | Direction::Right => 1,
        };

        for _ in 0..self.hops * 3 {
            f.write_char('(')?;
        }

        f.write_char(n)?;
        for _ in 0..self.hops {
            write!(f, " {op} {shift}) & {p}) {op} {shift}) & empty_space(b, w)")?;
        }
        Ok(())
    }
}

pub fn write_macros(mut out: impl std::fmt::Write, width: usize) -> std::fmt::Result {
    writeln!(out, "CREATE MACRO empty_space(b, w) AS ~b & ~w;")?;
    for is_black in [true, false] {
        for dir in Direction::all() {
            for hops in 1..=width / 2 {
                let move_mask = GetMoveMaskSql {
                    dir,
                    is_black,
                    width,
                    hops,
                };
                writeln!(
                    out,
                    "CREATE MACRO moves_{}{}{hops}(b, w) AS {move_mask};",
                    move_mask.next_player_char(),
                    move_mask.dir_char(),
                )?;
            }
        }
    }
    Ok(())
}

pub fn main() {
    let mark_empty_moves = r#"
    UPDATE konane AS k
    SET num_numerator = 0, num_denom_exp = 0
    WHERE moves_br1(k.black, k.white) = 0
      AND moves_bl1(k.black, k.white) = 0
      AND moves_bu1(k.black, k.white) = 0
      AND moves_bd1(k.black, k.white) = 0
      AND moves_wr1(k.black, k.white) = 0
      AND moves_wl1(k.black, k.white) = 0
      AND moves_wu1(k.black, k.white) = 0
      AND moves_wd1(k.black, k.white);
"#;

    let conn = Connection::open("konane.duckdb").expect("failed to open duckdb connection");
    // conn.execute_batch(&mark_empty_moves)
    //     .expect("failed to mark empty moves");

    let mut no_val_games = conn
        .prepare("SELECT (black, white) FROM konane WHERE num_numerator is NULL AND num_denom_exp is NULL AND nimber is NULL AND up is NULL")
        .expect("failed to prepare query");

    let mut set_nus = conn
        .prepare("UPDATE konane SET num_numerator = ?, num_denom_exp = ?, nimber = ?, up = ? WHERE black = ? AND white = ?")
        .expect("failed to prepare query");

    let games = no_val_games.query_arrow([]).expect("query failed");
    let tt = ParallelTranspositionTable::new();
    for game in games {
        println!("BEGIN batch ({} records)", game.num_rows());
        let data = game.column(0).to_data();
        let [black, white] = data.child_data() else {
            panic!("expected a two element struct");
        };
        for (black_buffer, white_buffer) in black.buffers().iter().zip(white.buffers().iter()) {
            let black_pos_data = black_buffer.typed_data::<u64>();
            let white_pos_data = white_buffer.typed_data::<u64>();

            println!("  BEGIN buffer ({} positions)", black_pos_data.len() / 2);
            for i in 0..(black_pos_data.len() / 2) {
                let game = Konane256::<W, H> {
                    white: BitBoard256 {
                        board: [0, 0, white_pos_data[i + 1], white_pos_data[i]].into(),
                    },

                    black: BitBoard256 {
                        board: [0, 0, black_pos_data[i + 1], black_pos_data[i]].into(),
                    },
                };
                let canonical_form = game.canonical_form(&tt);
                if let Some(nus) = canonical_form.to_nus() {
                    let white_128 =
                        (white_pos_data[i + 1] as i128) << 64 | white_pos_data[i] as i128;
                    let black_128 =
                        (black_pos_data[i + 1] as i128) << 64 | black_pos_data[i] as i128;
                    set_nus
                        .execute([
                            Value::Int(nus.number().numerator() as i32),
                            Value::UInt(nus.number().denominator_exponent()),
                            Value::UInt(nus.nimber().value()),
                            Value::Int(nus.up_multiple()),
                            Value::HugeInt(black_128),
                            Value::HugeInt(white_128),
                        ])
                        .expect("failed to update with NUS");
                } else {
                    println!("non-nus game");
                }
            }
            println!("  END buffer")
        }

        println!("END batch")
    }
}
