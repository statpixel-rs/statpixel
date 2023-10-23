#[tokio::main]
async fn main() {
	let url = std::env::args().nth(1).expect("No url provided");
	let url = url.as_str();

	let renderer = skin_renderer::SKIN_RENDERER
		.get_or_init(skin_renderer::create_renderer)
		.await;

	let start = std::time::Instant::now();
	let png = renderer
		.render(
			skin_renderer::SkinKind::Classic,
			if url == "none" { None } else { Some(url) },
		)
		.await
		.unwrap();
	let end = std::time::Instant::now();
	println!("Rendered in {:?}", end - start);

	std::fs::write("skin.png", png).unwrap();
}
