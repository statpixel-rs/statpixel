#[tokio::main]
async fn main() {
	let url = std::env::args().nth(1).expect("No url provided");
	let url = url.as_str();

	let start = std::time::Instant::now();
	let png = skin_renderer::render_skin(Some(url), false).await.unwrap();
	let end = std::time::Instant::now();
	println!("Rendered in {:?}", end - start);

	std::fs::write("skin.png", png).unwrap();
}
