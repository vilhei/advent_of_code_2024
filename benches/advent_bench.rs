use advents::main_utils::create_day;
// use advents::utils::Task;
use criterion::BenchmarkId;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_advent(c: &mut Criterion) {
    for day in 1..=25 {
        let advent = create_day(day);
        let benchmark_name = format!("bench_day{day}");
        let input = format!("./inputs/day{day}.txt");
        c.bench_with_input(BenchmarkId::new(benchmark_name, &input), &input, |b, i| {
            b.iter(|| advent.task_part_one(black_box(i)))
        });
    }
}

criterion_group!(benches, bench_advent);
criterion_main!(benches);
