# Environment

## Setting up SkyBlock textures

If you aren't making changes to SkyBlock, you can skip this section.

To set up SkyBlock textures, run the following scripts with a recent version of [Node.js](https://nodejs.org/):

```bash
node scripts/materials.js
node scripts/heads.js
```

## Setting up leaderboards

This step must be done *after* starting the bot for the first time and querying a player.

To set up leaderboards, generate them all with the following command:

```bash
node scripts/generate_leaderboards.js
```

## Setting up the database

StatPixel uses PostgreSQL as its database. Ensure you also have the [diesel cli](https://diesel.rs/) installed:

```bash
cargo install diesel_cli --no-default-features --features postgres
```

Once it's set up, you will need to apply the migrations:

```bash
diesel migration run
```

## Setting up Redis

StatPixel uses Redis as its cache. Ensure you also have a Redis server up and running.

## Other steps

See the [installation](../installation.md) page for more information.
