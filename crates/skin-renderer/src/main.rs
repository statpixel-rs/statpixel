#[tokio::main]
async fn main() {
	let url = std::env::args().nth(1).expect("No url provided");
	let url = url.as_str();

	let renderer = skin_renderer::SKIN_RENDERER
		.get_or_init(skin_renderer::create_renderer)
		.await;

	// save the output to a file
	let output = renderer
		.render(
			skin_renderer::SkinKind::Classic,
			if url == "none" { None } else { Some(url) },
		)
		.await
		.unwrap();

	std::fs::write("output.png", output).unwrap();

	for _ in 0..1_000 {
		// Warmup
		let _ = renderer
			.render(
				skin_renderer::SkinKind::Classic,
				if url == "none" { None } else { Some(url) },
			)
			.await
			.unwrap();
	}

	let mut total = std::time::Duration::new(0, 0);

	for _ in 0..1_000 {
		// time it
		let start = std::time::Instant::now();
		let _ = renderer
			.render(
				skin_renderer::SkinKind::Classic,
				if url == "none" { None } else { Some(url) },
			)
			.await
			.unwrap();
		let end = std::time::Instant::now();

		total += end - start;
	}

	println!("Average time: {:?}", total / 1_000);
}
