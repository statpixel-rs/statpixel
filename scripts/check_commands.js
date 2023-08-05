import fs from 'node:fs';

const commands = JSON.parse(fs.readFileSync('./commands.json', 'utf8'));

const locales = Object.keys({
	"fi": "tietoja",
	"cs": "about",
	"zh-TW": "關於",
	"hr": "o",
	"sv-SE": "om",
	"th": "about",
	"ja": "アバウト",
	"hu": "névjegy",
	"pl": "o-nas",
	"pt-BR": "sobre",
	"hi": "विषय-में",
	"el": "about",
	"ru": "о-боте",
	"lt": "about",
	"bg": "about",
	"de": "über",
	"fr": "àpropos",
	"tr": "about",
	"zh-CN": "关于",
	"es-ES": "acerca-de",
	"uk": "about",
	"it": "about",
	"da": "om",
	"no": "about",
	"nl": "over",
	"ro": "despre",
	"ko": "정보"
});

locales.push('en-US');

function computeSize(command, root = false) {
	let name = command.name?.length ?? 0;

	if (command.name_localizations) {
		for (const key in command.name_localizations) {
			const length = command.name_localizations[key].length;

			if (length > name) {
				name = length;
			}

			if (command.name_localizations[key] !== command.name_localizations[key].toLocaleLowerCase()) {
				console.log('CASE', command.name, key, command.name_localizations[key]);
			}
		}
	}

	let description = command.description?.length ?? 0;

	if (command.description_localizations) {
		for (const key in command.description_localizations) {
			const length = command.description_localizations[key].length;

			if (length > description) {
				description = length;
			}
		}
	}

	let value = command.value !== undefined ? command.value.toString().length : 0;

	let length = name + description + value;

	if (command.options) {
		for (const locale of locales) {
			const seen = new Set();

			for (const option of command.options) {
				const name = option.name_localizations?.[locale] ?? option.name;

				if (seen.has(name)) {
					console.log('DUPLICATE', command.name, locale, name);
				} else {
					seen.add(name);
				}
			}
		}

		for (const option of command.options) {
			length += computeSize(option);
		}
	}

	if (command.choices) {
		/*for (const choice of command.choices) {
			length += computeSize(choice);
		}*/

		for (const locale of locales) {
			const seen = new Set();

			for (const option of command.choices) {
				const name = option.name_localizations?.[locale] ?? option.name;

				if (seen.has(name)) {
					console.log('DUPLICATE', command.name, locale, name);
				} else {
					seen.add(name);
				}
			}
		}
	}

	return length;
}

function computeDebug(command) {
	let name = command.name?.length ?? 0;
	let nameKey = 'en-US';

	if (command.name_localizations) {
		for (const key in command.name_localizations) {
			const length = command.name_localizations[key].length;

			if (length > name) {
				name = length;
				nameKey = key;
			}
		}
	}

	console.log(`name key: ${nameKey} (${name}))`);

	let description = command.description?.length ?? 0;
	let descriptionKey = 'en-US';
	let descriptionValue = command.description;

	if (command.description_localizations) {
		for (const key in command.description_localizations) {
			const length = command.description_localizations[key].length;

			if (length > description) {
				description = length;
				descriptionKey = key;
				descriptionValue = command.description_localizations[key];
			}
		}
	}

	console.log(`description key: ${descriptionKey} (${description})\n${descriptionValue}`);

	let value = command.value !== undefined ? command.value.toString().length : 0;

	let length = name + description + value;

	if (command.options) {
		for (const option of command.options) {
			length += computeDebug(option);
		}
	}

	if (command.choices) {
		for (const choice of command.choices) {
			length += computeDebug(choice);
		}
	}

	return length;
}

for (const command of commands) {
	const length = computeSize(command, true);

	if (length > 4_000) {
		console.log(command.name, length);

		//computeDebug(command);
	}
}
