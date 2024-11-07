use cgt::short::partizan::{
    partizan_game::PartizanGame, transposition_table::ParallelTranspositionTable,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion, SamplingMode};
use konane::Konane256;
use std::time::Duration;

fn canonical_4x4_in_6x6_checkerboard(c: &mut Criterion) {
    let mut group = c.benchmark_group("6x6 checkerboard");
    group.sample_size(10).sampling_mode(SamplingMode::Flat);
    group.measurement_time(Duration::from_secs(120));

    group.bench_function("2x2", |b| {
        let game = Konane256::<6, 6>::must_parse(
            r#"
            ______
            ______
            __ox__
            __ox__
            ______
            ______
        "#,
        );

        b.iter(move || {
            let mut tt = ParallelTranspositionTable::new();
            black_box(game.canonical_form(&mut tt))
        })
    });

    group.bench_function("4x4", |b| {
        let game = Konane256::<6, 6>::must_parse(
            r#"
            ______
            _xoxo_
            _oxox_
            _xoxo_
            _oxox_
            ______
        "#,
        );

        b.iter(move || {
            let mut tt = ParallelTranspositionTable::new();
            black_box(game.canonical_form(&mut tt))
        })
    });

    // group.bench_function("full", |b| {
    //     let mut game = Konane256::<6, 6>::checkerboard();
    //     game.set_tile(3, 4, konane::TileState::Empty);
    //     game.set_tile(4, 4, konane::TileState::Empty);
    //     b.iter(move || {
    //         let mut tt = ParallelTranspositionTable::new();
    //         black_box(game.canonical_form(&mut tt))
    //     })
    // });
    group.finish();
}

criterion_group!(canonicalize, canonical_4x4_in_6x6_checkerboard);
criterion_main!(canonicalize);
