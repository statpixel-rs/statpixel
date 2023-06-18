import fs from 'node:fs';

const current = JSON.parse(fs.readFileSync('./assets/items.json', 'utf8'));
const add = JSON.parse(fs.readFileSync('./items.json', 'utf8'));

const merged = new Map();

function getId(item) {
	const petId = item?.tag?.ExtraAttributes?.petInfo?.type;

	if (petId) {
		item.Damage = 0;
	}

	return petId ?? item?.tag?.ExtraAttributes?.id;
}

for (const item of current) {
	const id = getId(item);
	if (!id) continue;

	merged.set(id, item);
}

let added = 0;

for (const item of Object.values(add).filter(i => Array.isArray(i)).flat()) {
	const id = getId(item);
	if (!id) continue;

	if (!merged.has(id)) {
		added++;
		merged.set(id, item);
	}

	if (item.containsItems) {
		for (const containedItem of item.containsItems) {
			const id = getId(containedItem);
			if (!id) continue;

			if (!merged.has(id)) {
				added++;
				merged.set(id, containedItem);
			}
		}
	}
}

fs.writeFileSync('./assets/items.json', JSON.stringify([...merged.values()], null, '\t'));

console.log(`Added ${added} items`);