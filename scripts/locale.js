import { FluentBundle, FluentResource } from '@fluent/bundle';
import fs from 'node:fs';
import { encode } from 'gpt-3-encoder';

const bundle = new FluentBundle('en-US');
const resource = new FluentResource(fs.readFileSync('../locale/en-US.ftl', 'utf8'));

const errors = bundle.addResource(resource);

if (errors.length) {
	console.error(errors);
}

// all supported languages by Discord
const SUPPORTED_LANGUAGES = [
	// done
	{
		name: 'Danish',
		code: 'da',
	},
	// done
	{
		name: 'German',
		code: 'de',
	},
	{
		name: 'English (US)',
		code: 'en-US',
	},
	{
		name: 'English (UK)',
		code: 'en-GB',
	},
	// done
	{
		name: 'Spanish',
		code: 'es-ES',
	},
	// done
	{
		name: 'French',
		code: 'fr',
	},
	// done
	{
		name: 'Croatian',
		code: 'hr',
	},
	// done
	{
		name: 'Italian',
		code: 'it',
	},
	{
		name: 'Lithuanian',
		code: 'lt',
	},
	{
		name: 'Hungarian',
		code: 'hu',
	},
	{
		name: 'Dutch',
		code: 'nl',
	},
	{
		name: 'Norwegian',
		code: 'no',
	},
	{
		name: 'Polish',
		code: 'pl',
	},
	{
		name: 'Portuguese (Brazilian)',
		code: 'pt-BR',
	},
	{

		name: 'Romanian',
		code: 'ro',
	},
	{
		name: 'Finnish',
		code: 'fi',
	},
	{
		name: 'Swedish',
		code: 'sv-SE',
	},
	{
		name: 'Turkish',
		code: 'tr',
	},
	{
		name: 'Czech',
		code: 'cs',
	},
	{
		name: 'Greek',
		code: 'el',
	},
	{

		name: 'Bulgarian',
		code: 'bg',
	},
	{
		name: 'Russian',
		code: 'ru',
	},
	{
		name: 'Ukrainian',
		code: 'uk',
	},
	{
		name: 'Hindi',
		code: 'hi',
	},
	{
		name: 'Thai',
		code: 'th',
	},
	{
		name: 'Chinese (China)',
		code: 'zh-CN',
	},
	{
		name: 'Chinese (Taiwan)',
		code: 'zh-TW',
	},
	{
		name: 'Japanese',
		code: 'ja',
	},
	{
		name: 'Korean',
		code: 'ko',
	},
];

// split the english locale into chunks to be processed by GPT-3.5

const MAX_TOKENS = 2048;
const chunks = fs.readFileSync('../locale/en-US.ftl', 'utf8').split('\n\n').map(c => ({
	chunk: c,
	tokens: encode(c).length,
}));

const BASE = `Translate the entire following Fluent localization file into Lithuanian. Return the entire response in a code block using the original format.

---`;

const END = `---

Remember, only translate into Lithuanian.`;

const BASE_TOKENS = encode(BASE).length + encode(END).length;

const chunked = [];
const current = [BASE];

let currentTokens = BASE_TOKENS;

for (const chunk of chunks) {
	if (currentTokens + chunk.tokens > MAX_TOKENS) {
		current.push(END);
		chunked.push(current.join('\n\n'));
		current.length = 0;
		current.push(BASE);
		currentTokens = BASE_TOKENS;
	}

	current.push(chunk.chunk);
	currentTokens += chunk.tokens;
}

if (current.length) {
	current.push(END);
	chunked.push(current.join('\n\n'));
}

for (const [i, chunk] of chunked.entries()) {
	fs.writeFileSync(`./chunks/${i.toString().padStart(3, '0')}.ftl`, chunk);
}
