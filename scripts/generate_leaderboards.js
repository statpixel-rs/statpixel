import 'dotenv/config';

import fs from 'node:fs';

import pg from 'pg';

const files = new Map(fs.readdirSync('./api/src/player/stats')
	.map(f => [f.slice(0, -3), fs.readFileSync(`./api/src/player/stats/${f}`, 'utf8').split('\n')]));

if (!process.env.DATABASE_URL) {
	throw new Error('DATABASE_URL environment variable is not set');
}

const GAME_TO_FILE = {
	'Pit': 'pit',
	'UHC': 'uhc',
	'MCGO': 'cops_and_crims',
	'Arena': 'arena',
	'Duels': 'duels',
	'Quake': 'quake',
	'Walls': 'walls',
	'Arcade': 'arcade',
	'Walls3': 'mega_walls',
	'Bedwars': 'bed_wars',
	'SkyWars': 'sky_wars',
	'SpeedUHC': 'speed_uhc',
	'TNTGames': 'tnt_games',
	'VampireZ': 'vampire_z',
	'Paintball': 'paintball',
	'WoolGames': 'wool_wars',
	'SuperSmash': 'smash_heroes',
	'BuildBattle': 'build_battle',
	'GingerBread': 'turbo_kart_racers',
	'HungerGames': 'blitz_sg',
	'Battleground': 'warlords',
	'MurderMystery': 'murder_mystery',
	'MainLobby': 'fishing',
};

const gameToKeys = new Map();

function findFirstModeBeforeLine(game, key, content, index) {
	if (index === -1) {
		let mode;

		switch (game) {
			case 'Bedwars':
				if (key === 'blocks_placed') return 'Practice';
				break;
		}

		return mode;
	}

	for (let i = index; i >= 0; --i) {
		const idx = content[i].indexOf('pub struct');

		if (idx !== -1) {
			const space = content[i].indexOf(' ', idx + 11);

			return content[i].slice(idx + 11, space);
		}
	}
}

const NAME_REGEX = /(\w+): /;

function findFirstNameAfterLine(content, index) {
	if (index === -1) return;

	for (let i = index; i < content.length; ++i) {
		const name = content[i].match(NAME_REGEX);

		if (name?.[1]) {
			return name[1];
		}
	}
}

function findNameTr(content, name) {
	const trLine = content.findIndex(l => l.includes(`ident = "${name}"`));

	if (trLine !== -1) {
		for (let i = trLine; i >= 0; --i) {
			const trStart = content[trLine].indexOf('tr = "');

			if (trStart !== -1) {
				const trEnd = content[trLine].indexOf('"', trStart + 6);

				return content[trLine].slice(trStart + 6, trEnd);
			}
		}
	}
}

function getMode(game, path, key, object) {
	const content = files.get(GAME_TO_FILE[game]);
	if (content === undefined) console.log(game);
	const index = content.findIndex(l => l.includes(`rename = "${key}"`) || l.includes(`rename(deserialize = "${key}")`));

	const mode = findFirstModeBeforeLine(game, key, content, index);
	const name = findFirstNameAfterLine(content, index) ?? key;
	const tr = findNameTr(content, name);

	return {
		tr: tr ?? name.replaceAll('_', '-'),
		mode: mode === 'PracticeRecords' || mode === 'PracticeMode' ? 'Practice' : mode,
	}
}

const client = new pg.Client(process.env.DATABASE_URL);
await client.connect();

const { rows } = await client.query('SELECT data FROM leaderboard LIMIT 1');

if (!rows.length) {
	throw new Error('No leaderboard data found. Please start with no leaderboards, query one player on your own hosted version, then run this again.');
}

const stats = rows[0].data.stats;
const leaderboards = Object.entries(stats).flatMap(s => getLeaderboards(s[0], `"leaderboard"."data"->'stats'->'${s[0]}'`, s[1]));

function getLeaderboards(game, path, object) {
	const leaderboards = [];

	for (const [key, value] of Object.entries(object)) {
		if (Array.isArray(value) || value === null) continue;

		if (typeof value === 'object') {
			leaderboards.push(...getLeaderboards(game, `${path}->'${key}'`, value));
		} else if (typeof value === 'number') {
			const { tr, mode } = getMode(game, path, key, object);

			const keys = gameToKeys.get(game) ?? gameToKeys.set(game, new Map()).get(game);
			const keyData = keys.get(tr) ?? keys.set(tr, { count: 0, paths: [] }).get(tr);

			keyData.count++;
			keyData.paths.push(`${path}->'${key}'`);

			leaderboards.push({ game, path: `${path}->'${key}'`, tr, mode });
		}
	}

	for (const [game, keys] of gameToKeys.entries()) {
		for (const [key, { count, paths }] of keys.entries()) {
			if (count < 2) continue;

			leaderboards.push({ game, path: `(${paths.map(p => `CAST(${p} AS INT)`).join(' + ')})`, tr: key, mode: 'Overall' });
		}
	}

	return leaderboards;
}

fs.writeFileSync('./include/leaderboards.json', JSON.stringify(leaderboards, null, '\t'));

await client.end();
