use duckdb::{types::Value, Connection};
use itertools::Itertools;
use konane::{bitboard::BitBoard256, Konane256, TileState};
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

pub fn main() {
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
