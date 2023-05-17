use api::player::Player;
use criterion::{criterion_group, Criterion};
use flate2::{write::ZlibEncoder, Compression};
use std::{io::Write, time::Duration};
use tokio::runtime::Runtime;

macro_rules! bench_player {
	($fn: ident, $name: tt) => {
		fn $fn(c: &mut Criterion) {
			let data = {
				let runtime = Runtime::new().unwrap();

				runtime.block_on(super::set_up_key());

				let player = runtime.block_on(Player::from_username($name)).unwrap();

				bincode::encode_to_vec(
					&runtime.block_on(player.get_data()).unwrap(),
					bincode::config::standard(),
				)
				.unwrap()
			};

			let mut z = ZlibEncoder::new(Vec::new(), Compression::default());
			z.write_all(data.as_slice()).unwrap();

			let data = z.finish().unwrap();
			let slice = data.as_slice();

			let mut group = c.benchmark_group("player_data_hash");

			group.bench_function($name, |b| {
				b.iter(|| {
					let _ = fxhash::hash64(criterion::black_box(slice));
				});
			});
		}
	};
}

bench_player!(top_achivements, "anrie");
bench_player!(top_network_level, "luur");
bench_player!(top_sky_wars, "lifelong");
bench_player!(top_bed_wars, "WarOG");
bench_player!(top_duels, "SkySteveSparrowS");
bench_player!(new_player, "Notch");

fn short_warmup() -> Criterion {
	Criterion::default().warm_up_time(Duration::new(1, 0))
}

criterion_group! {
	name = benches;
	config = short_warmup();
	targets = top_achivements, top_network_level, top_sky_wars, top_bed_wars, top_duels, new_player
}
