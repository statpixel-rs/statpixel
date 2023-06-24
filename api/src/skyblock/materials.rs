use std::collections::HashMap;

use once_cell::sync::Lazy;

pub static MATERIALS: Lazy<HashMap<String, crate::image::Image<'static>>> = Lazy::new(|| {
	let mut map = HashMap::new();

	let materials = std::fs::read_dir("assets/materials").unwrap();
	let textures = std::fs::read_dir("assets/textures").unwrap();

	map.extend(textures.into_iter().map(|f| {
		let f = f.unwrap();
		let bytes = std::fs::read(f.path()).unwrap();

		let name = f.file_name();
		let name = name.to_string_lossy().replace(';', ":");

		let idx = name.rfind('.').unwrap();
		let name = &name[..idx];

		let image = crate::image::from_bytes(bytes.leak()).unwrap();

		(name.to_string(), image)
	}));

	map.extend(materials.into_iter().map(|f| {
		let f = f.unwrap();
		let bytes = std::fs::read(f.path()).unwrap();

		let name = f.file_name();
		let name = name.to_string_lossy().replace(';', ":");

		let idx = name.rfind('.').unwrap();
		let name = &name[..idx];

		let image = crate::image::from_bytes(bytes.leak()).unwrap();

		(name.to_string(), image)
	}));

	map
});
