use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use game_of_life_rs::game::Grid;
use pprof::criterion::{Output, PProfProfiler};


fn rule(tup: (bool, [bool; 8])) -> bool {
	let (cell, arr) = tup;	
	let neighbor_count = arr
		.iter()
		.fold(0, |acc, item| if *item { acc + 1 } else { acc });
	if neighbor_count == 3 {
		return true;
	}
	if cell && neighbor_count == 2 {
		return true;
	}
	return false;
}


fn grid_test_seq(size: usize, reps: usize) {
	let mut screen = Grid::new(size, size);
	for _i in 0..reps {
		screen.propogate(&rule);
	}
}
fn grid_test_par(size: usize, reps: usize) {
	let mut screen = Grid::new(size, size);	
	for _i in 0..reps {
		screen.propogate_par(&rule);
	}
}
fn grid_test_par2(size: usize, reps: usize) {
	let mut screen = Grid::new(size, size);	
	for _i in 0..reps {
		screen.propogate_par2(&rule);
	}
}
fn grid_test_seq2() {
	let mut screen = Grid::new(100, 100);
	for _i in 0..100 {
		screen.propogate(&rule);
	}
}

fn bench_fibs(c: &mut Criterion) {
    let mut group = c.benchmark_group("Parallelization");
	group.sampling_mode(criterion::SamplingMode::Flat);
	group.sample_size(30);
    for size in (50..=100).step_by(25) {
        group.bench_with_input(BenchmarkId::new("Sequential", size), &size, 
            |b, &s| b.iter(|| grid_test_seq(s, 100)));
        group.bench_with_input(BenchmarkId::new("Parallel", size), &size, 
            |b, &s| b.iter(|| grid_test_par(s, 100)));
        group.bench_with_input(BenchmarkId::new("Parallel2", size), &size, 
            |b, &s| b.iter(|| grid_test_par2(s, 100)));
    }
    group.finish();
}

fn bench_func(c: &mut Criterion){
    c.bench_function("Sequential", |b| b.iter(|| grid_test_seq2()));
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(30, Output::Flamegraph(None)));
    targets = bench_func
}
criterion_main!(benches);
