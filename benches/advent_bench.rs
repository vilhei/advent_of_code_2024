use std::time::Duration;

use advents::main_utils::create_day;
use advents::utils::read_task_input_file;
// use advents::utils::Task;
use criterion::{black_box, BenchmarkId};
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_advent(c: &mut Criterion) {
    let mut group = c.benchmark_group("tasks");
    group.sampling_mode(criterion::SamplingMode::Auto);
    group
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(10))
        .sample_size(10);
    for day in 1..=25 {
        let advent = create_day(day);
        let benchmark_name = format!("day_{day}_");
        let file_path = format!("./inputs/day{day}.txt");
        let input = read_task_input_file(&file_path).unwrap();

        group.bench_with_input(
            BenchmarkId::new(benchmark_name.clone() + "_part1", format!(" day{day}.txt")),
            &input,
            |b, i| b.iter(|| advent.task_part_one(black_box(i))),
        );

        group.bench_with_input(
            BenchmarkId::new(benchmark_name + "_part2", format!(" day{day}.txt")),
            &input,
            |b, i| b.iter(|| advent.task_part_two(black_box(i))),
        );
    }
    group.finish();
}

criterion_group!(benches, bench_advent);
criterion_main!(benches);
