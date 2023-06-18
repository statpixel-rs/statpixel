import fs from 'node:fs';

import axios from 'axios';
import { fdir } from 'fdir';
import sharp from 'sharp';

import { bufferUnordered } from './utils.js';

const ORDER = [
	'Vanilla',
	'Vanilla+_1_441',
	'Worlds_and_Beyond_1_4_1',
	'Hypixel_Plus',
	'PacksHQ_16x_14',
	'RNBW+_0_7',
	'FurfSky_Reborn_1_6_3'
];

const ANIMATE_REGEX = /_(\d+)\.[^.]+$/;

fs.mkdirSync('./assets/materials', { recursive: true });
fs.mkdirSync('./assets/textures', { recursive: true });

for (const name of ORDER) {
	const files = new fdir()
		.withFullPaths()
		.crawl(`./assets/resourcepacks/${name}`)
		.sync();

	for (const path of files) {
		const material = path.slice(path.lastIndexOf('\\') + 1);
		let name = material;

		if (ANIMATE_REGEX.test(material)) {
			const [, num] = material.match(ANIMATE_REGEX);
			const n = parseInt(num);

			if (n === 0)
				name = material.slice(0, material.lastIndexOf('_'));
			else {
				const dotIndex = name.lastIndexOf('.');

				name = name.slice(0, dotIndex);
			}
		} else {
			const dotIndex = name.lastIndexOf('.');

			name = name.slice(0, dotIndex);
		}

		if (name === 'item') {
			// go back up one directory
			name = path.slice(0, path.lastIndexOf('\\'));
			name = name.slice(name.lastIndexOf('\\') + 1);
		}

		if (path.endsWith('.properties')) {
			const b64 = fs.readFileSync(path, 'utf-8').match(/nbt\.SkullOwner\.Properties\.textures\.0\.Value=(.+)/)?.[1];

			if (b64) {
				try {
					console.log(`Processed "${name}" (${path})`);

					const data = JSON.parse(Buffer.from(b64, 'base64').toString('utf-8'));
					const texture = data.textures.SKIN.url;
					const id = texture.slice(texture.lastIndexOf('/') + 1);

					let tryagain = true;

					while (tryagain) {
						try {
							const { data: buf } = await axios.get(`https://sky.shiiyu.moe/head/${id}`, {
								responseType: 'arraybuffer'
							});

							await sharp(buf)
								.resize(48, 48, {
									// for crisp edges since it's pixel art
									kernel: 'nearest'
								})
								.toFile(`./assets/textures/${name.replace(/:/g, '_').toUpperCase()}.png`);

							tryagain = false;
						} catch (e) {
							console.error(e);
						}
					}
				} catch (e) {
					console.error(e);
				}
			}
		}

		if (!path.endsWith('.png')) {
			continue;
		}

		await sharp(path)
			.resize(48, 48, {
				// for crisp edges since it's pixel art
				kernel: 'nearest',
				position: 'left top',
				fit: 'cover',
			})
			.toFile(`./assets/materials/${name.replace(/:/g, '_').toUpperCase()}.png`);
	}

	console.log(`Finished "${name}"`);
}
