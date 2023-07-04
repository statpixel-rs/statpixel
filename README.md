# StatPixel

![test status](https://github.com/matteopolak/statpixel/actions/workflows/test.yml/badge.svg)
![release status](https://github.com/matteopolak/statpixel/actions/workflows/release.yml/badge.svg)

## Environment variables

- `HYPIXEL_API_KEY`: Your API key from running `/api new`
- `DISCORD_TOKEN`: Your Discord bot token from [discord.dev](https://discord.dev)
- `DATABASE_URL`: Your PostgreSQL database URL

## Stats

```bash
cloc . --exclude-dir target,node_modules,assets --exclude-ext json
---------------------------------------------------------------------------------
Language                       files          blank        comment           code
---------------------------------------------------------------------------------
Rust                             184           3656            394          30569
Freemarker Template                6           1114              0           5871
YAML                               5            113              4            743
SQL                               90            128            101            279
TOML                              12             34              1            274
JavaScript                         6             61             10            181
Markdown                           1              3              0              7
Text                               1              0              0              1
---------------------------------------------------------------------------------
SUM:                             305           5109            510          37925
---------------------------------------------------------------------------------
```
