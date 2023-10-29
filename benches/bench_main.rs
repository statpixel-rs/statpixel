use criterion::criterion_main;

mod benchmarks;

criterion_main! {
	benchmarks::generate_bedwars_image::benches,
}
