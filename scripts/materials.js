import fs from 'node:fs';

import sharp from 'sharp';

const materials = fs.readdirSync('./assets/materials_raw');

fs.mkdirSync('./assets/materials', { recursive: true });

const ANIMATE_REGEX = /_(\d+)\.[^.]+$/;

for (const material of materials) {
	let name = material;

	if (ANIMATE_REGEX.test(material)) {
		const [, num] = material.match(ANIMATE_REGEX);
		const n = parseInt(num);

		if (n === 0)
			name = material.slice(0, material.lastIndexOf('_'));
		else if (n < 64) continue;
		else {
			const dotIndex = name.lastIndexOf('.');

			name = name.slice(0, dotIndex);
		}
	} else {
		const dotIndex = name.lastIndexOf('.');

		name = name.slice(0, dotIndex);
	}

	console.log(name, material);

	await sharp(`./assets/materials_raw/${material}`)
		.resize(48, 48, {
			// for crisp edges since it's pixel art
			kernel: 'nearest',
			position: 'left top',
			fit: 'cover',
		})
		.toFile(`./assets/materials/${name.toUpperCase()}.png`);
}
