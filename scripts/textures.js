import fs from 'node:fs';

import axios from 'axios';
import sharp from 'sharp';

import { bufferUnordered } from './utils.js';

fs.mkdirSync('./assets/textures', { recursive: true });

const { data: { items } } = await axios.get('https://api.hypixel.net/resources/skyblock/items');

let i = 0;

await bufferUnordered(items, async item => {
	if (!item.skin || await fs.promises.stat(`./assets/textures/${item.id}.png`).then(() => true).catch(() => false))
		return console.log(`Skipped "${item.name}" ${++i}/${items.length}...`);

	const data = JSON.parse(Buffer.from(item.skin, 'base64').toString());
	// http://textures.minecraft.net/texture/...
	const texture = data.textures.SKIN.url;
	const id = texture.slice(texture.lastIndexOf('/') + 1);

	const { data: buf } = await axios.get(`https://sky.shiiyu.moe/head/${id}`, {
		responseType: 'arraybuffer'
	});

	await sharp(buf)
		.resize(48, 48, {
			// for crisp edges since it's pixel art
			kernel: 'nearest'
		})
		.toFile(`./assets/textures/${item.id}.png`);

	console.log(`Processed "${item.name}" ${++i}/${items.length}...`);
}, 15);
