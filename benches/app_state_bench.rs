use criterion::{black_box, criterion_group, criterion_main, Criterion};
use learn_liberty_app::state::AppState;

fn bench_app_state_update(c: &mut Criterion) {
    c.bench_function("app_state_update", |b| {
        let mut state = AppState::default();
        b.iter(|| {
            state.update(black_box(0.016));
        })
    });
}

fn bench_app_state_advance_lesson(c: &mut Criterion) {
    c.bench_function("app_state_advance_lesson", |b| {
        let mut state = AppState::default();
        b.iter(|| {
            state.advance_lesson(
                black_box("test_lesson".to_string()),
                black_box(0.5),
            );
        })
    });
}

fn bench_app_state_multiple_updates(c: &mut Criterion) {
    c.bench_function("app_state_multiple_updates", |b| {
        let mut state = AppState::default();
        b.iter(|| {
            for _ in 0..100 {
                state.update(black_box(0.016));
            }
        })
    });
}

criterion_group!(
    benches,
    bench_app_state_update,
    bench_app_state_advance_lesson,
    bench_app_state_multiple_updates
);
criterion_main!(benches);
