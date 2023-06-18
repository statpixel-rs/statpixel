import fs from 'node:fs';

import axios from 'axios';
import sharp from 'sharp';

import { bufferUnordered } from './utils.js';

fs.mkdirSync('./assets/textures', { recursive: true });
fs.mkdirSync('./assets/materials', { recursive: true });

let i = 0;

const MAP = new Map(Object.entries(JSON.parse(fs.readFileSync('./assets/map.json', 'utf-8'))));
const buf = fs.readFileSync('./assets/items.png');
const ANIMATE_REGEX = /_(\d+)\.[^.]+$/;

const heads = JSON.parse(fs.readFileSync('./assets/heads.json', 'utf8'));
const items = JSON.parse(fs.readFileSync('./assets/textures.json', 'utf8'));

function stripDamage(item, id) {
	const idx = id.indexOf(':');
	const updatedId = idx !== -1 ? id.slice(0, idx) : id;

	if (item.Damage !== 0 && typeof item.Damage === 'number') {
		return `${updatedId};${item.Damage}`;
	}

	return updatedId;
}

await bufferUnordered(items, async item => {
	if (item.texture) return;

	const key = `${item.item_id}${item.damage === 0 || !item.damage ? '' : `:${item.damage}`}`;

	if (item.item_id !== undefined && MAP.has(key)) {
		const [x, y] = MAP.get(key);

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
			.toFile(`./assets/materials/${item.id.replace(/:/g, ';').toUpperCase()}.png`);

		console.log(`Processed "${item.name}" [${item.id}]`);
	}
}, 15);

await bufferUnordered(heads, async item => {
	if (!item.texture || await fs.promises.stat(`./assets/textures/${item.id}.png`).then(() => true).catch(e => false))
		return console.log(`Skipped "${item.name}" ${++i}/${items.length}...`);

	const { data: buf } = await axios.get(`https://sky.shiiyu.moe/head/${item.texture}`, {
		responseType: 'arraybuffer'
	});

	await sharp(buf)
		.resize(48, 48, {
			// for crisp edges since it's pixel art
			kernel: 'nearest'
		})
		.toFile(`./assets/textures/${item.id.replace(/:/g, ';').toUpperCase()}.png`);

	console.log(`Processed "${item.name}"`);
}, 15);
