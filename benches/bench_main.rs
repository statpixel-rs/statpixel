use criterion::criterion_main;

mod benchmarks;

criterion_main! {
	benchmarks::player_data_de_json::benches,
	benchmarks::player_data_se_bson::benches,
	benchmarks::player_data_de_bson::benches,
	benchmarks::player_data_compress::benches,
	benchmarks::player_data_decompress::benches,
}
