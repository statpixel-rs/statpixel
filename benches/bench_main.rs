use criterion::criterion_main;

mod benchmarks;

criterion_main! {
	benchmarks::player_data_se_bincode::benches,
	benchmarks::player_data_de_bincode::benches,
	benchmarks::player_data_decompress::benches,
	benchmarks::player_data_hash::benches,
}
