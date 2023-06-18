import fs from 'node:fs';

import axios from 'axios';
import sharp from 'sharp';

import { bufferUnordered } from './utils.js';

const materials = fs.readdirSync('./assets/materials_raw');

fs.mkdirSync('./assets/materials', { recursive: true });

const ANIMATE_REGEX = /_(\d+)\.[^.]+$/;
const ITEMS = Object.values(JSON.parse(fs.readFileSync('./assets/items.json', 'utf-8'))).flat();
const MAP = new Map(Object.entries(JSON.parse(fs.readFileSync('./assets/map.json', 'utf-8'))));
const buf = fs.readFileSync('./assets/items.png');

for (const material of materials) {
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

	console.log(name, material);

	if (await fs.promises.stat(`./assets/materials/${name.toUpperCase()}.png`).then(() => true).catch(() => false)) {
		console.log(`Skipped "${name}"`);

		continue;
	}

	await sharp(`./assets/materials_raw/${material}`)
		.resize(48, 48, {
			// for crisp edges since it's pixel art
			kernel: 'nearest',
			position: 'left top',
			fit: 'cover',
		})
		.toFile(`./assets/materials/${name.toUpperCase()}.png`);
}

function stripDamage(item, id) {
	const idx = id.indexOf(':');
	const updatedId = idx !== -1 ? id.slice(0, idx) : id;

	if (item.Damage !== 0 && typeof item.Damage === 'number') {
		return `${updatedId};${item.Damage}`;
	}

	return updatedId;
}

await bufferUnordered(ITEMS, async item => {
	let id = item?.tag?.ExtraAttributes?.id;
	const texture = item?.texture_path;

	if (id?.startsWith('ENCHANTED_')) {
		id = id.slice(10);
	}

	if (item?.tag?.ExtraAttributes?.petInfo?.type) {
		id = item.tag.ExtraAttributes.petInfo.type;
		item.Damage = 0;
	}

	// Skip skulls
	if (item.id === 397 && !id) return;

	console.log(`Processing "${id}" (${item.id}:${item.Damage})...`);

	if (id && texture) {
		const { data: buf } = await axios.get(`https://sky.shiiyu.moe${texture}`, {
			responseType: 'arraybuffer'
		});

		await sharp(buf)
			.resize(48, 48, {
				// for crisp edges since it's pixel art
				kernel: 'nearest'
			})
			.toFile(`./assets/materials/${stripDamage(item, id)}.png`);
	} else if (id && item.id && MAP.has(`${item.id}${item.Damage === 0 ? '' : `:${item.Damage}`}`)) {
		const [x, y] = MAP.get(`${item.id}${item.Damage === 0 ? '' : `:${item.Damage}`}`);

		await sharp(buf)
			.extract({
				left: x,
				top: y,
				width: 128,
				height: 128,
			})
			.resize(48, 48, {
				// for crisp edges since it's pixel art
				kernel: 'nearest'
			})
			.toFile(`./assets/materials/${stripDamage(item, id)}.png`);
	}
}, 15);
