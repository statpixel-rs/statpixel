use api::player::data::Data;
use criterion::{criterion_group, Criterion};
use std::time::Duration;

use api::canvas::prelude::Game;

macro_rules! bench_player {
	($fn: ident, $name: tt) => {
		fn $fn(c: &mut Criterion) {
			let data: Data =
				serde_json::from_str(include_str!(concat!("../data/", $name, ".json"))).unwrap();
			let skin = skia_safe::Image::from_encoded(skia_safe::Data::new_copy(include_bytes!(
				"../data/skin.png"
			)))
			.unwrap();

			let mut group = c.benchmark_group("generate_bedwars_image");

			group.bench_function($name, |b| {
				b.iter(|| {
					criterion::black_box(api::player::stats::bed_wars::BedWars::canvas(
						&translate::context::Context::EMPTY,
						minecraft::style::Family::Minecraft,
						&data,
						&api::Session::default(),
						&skin,
						None,
						None,
						None,
					));
				});
			});
		}
	};
}

bench_player!(top_achivements, "anrie");
bench_player!(top_network_level, "luur");
bench_player!(top_sky_wars, "lifelong");
bench_player!(top_bed_wars, "WarOG");
bench_player!(new_player, "Notch");

fn short_warmup() -> Criterion {
	Criterion::default().warm_up_time(Duration::new(5, 0))
}

criterion_group! {
	name = benches;
	config = short_warmup();
	targets = top_achivements, top_network_level, top_sky_wars, top_bed_wars, new_player
}
