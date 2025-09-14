use criterion::{black_box, criterion_group, criterion_main, Criterion};
use learn_liberty_app::tests::MockGraphicsEngine;

fn bench_graphics_render(c: &mut Criterion) {
    c.bench_function("graphics_render", |b| {
        let mut engine = MockGraphicsEngine::new(1920, 1080);
        b.iter(|| {
            engine.render().unwrap();
        })
    });
}

fn bench_graphics_resize(c: &mut Criterion) {
    c.bench_function("graphics_resize", |b| {
        let mut engine = MockGraphicsEngine::new(800, 600);
        b.iter(|| {
            engine.resize(black_box(1920), black_box(1080));
        })
    });
}

fn bench_graphics_multiple_renders(c: &mut Criterion) {
    c.bench_function("graphics_multiple_renders", |b| {
        let mut engine = MockGraphicsEngine::new(1920, 1080);
        b.iter(|| {
            for _ in 0..100 {
                engine.render().unwrap();
            }
        })
    });
}

fn bench_graphics_rapid_resize(c: &mut Criterion) {
    c.bench_function("graphics_rapid_resize", |b| {
        let mut engine = MockGraphicsEngine::new(800, 600);
        let sizes = vec![(1024, 768), (1920, 1080), (800, 600), (1280, 720)];
        b.iter(|| {
            for (width, height) in &sizes {
                engine.resize(black_box(*width), black_box(*height));
            }
        })
    });
}

criterion_group!(
    benches,
    bench_graphics_render,
    bench_graphics_resize,
    bench_graphics_multiple_renders,
    bench_graphics_rapid_resize
);
criterion_main!(benches);
