use duckdb::Connection;
use itertools::Itertools;
use konane::{Konane256, TileState};

pub fn main() {
    const W: usize = 11;
    const H: usize = 11;
    let block_w = 4;
    let block_h = 4;

    let conn = Connection::open("konane.duckdb").expect("failed to open duckdb connection");

    conn.execute_batch(
        r"
          CREATE TABLE IF NOT EXISTS konane (
                  black           UHUGEINT,
                  white           UHUGEINT,

                PRIMARY KEY (black, white),
                UNIQUE (black, white)
            );
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
