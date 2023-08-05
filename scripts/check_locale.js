import { FluentResource } from '@fluent/bundle';
import fs from 'node:fs';

const truth = new FluentResource(fs.readFileSync('../locale/en-US.ftl', 'utf8'));
const locales = fs.readdirSync('../locale');

const required = new Set(truth.body.map(entry => entry.id));

for (const locale of locales) {
	if (locale === 'en-US.ftl') continue;

	const resource = new FluentResource(fs.readFileSync(`../locale/${locale}`, 'utf8'));
	const has = new Set(resource.body.map(entry => entry.id));

	for (const [i, entry] of resource.body.entries()) {
		if (!required.has(entry.id)) {
			console.log(`extra ${entry.id} in ${locale}`);
			console.log(`comes right after ${resource.body[i - 1]?.id}`);
			process.exit(0);
		}
	}

	// missing
	for (const [i, { id }] of truth.body.entries()) {
		if (!has.has(id)) {
			console.log(`missing ${id} in ${locale}`);
			console.log(`should come right after ${truth.body[i - 1]?.id}`);

			process.exit(0);
		}
	}
}
