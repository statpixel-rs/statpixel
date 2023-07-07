# StatPixel

![test status](https://github.com/matteopolak/statpixel/actions/workflows/test.yml/badge.svg)
![release status](https://github.com/matteopolak/statpixel/actions/workflows/release.yml/badge.svg)

## Environment variables

- `HYPIXEL_API_KEY`: Your API key from running `/api new`
- `DISCORD_TOKEN`: Your Discord bot token from [discord.dev](https://discord.dev)
- `DATABASE_URL`: Your PostgreSQL database URL

## Stats

```bash
tokei
===============================================================================
 Language            Files        Lines         Code     Comments       Blanks
===============================================================================
 FreeMarker              6         7051         5925            0         1126
 INI                     6           90           90            0            0
 JavaScript              4          229          166           10           53
 JSON                  946       259278       259270            0            8
 SQL                    90          508          279          101          128
 TOML                    9          282          250            1           31
-------------------------------------------------------------------------------
 Markdown                1           15            0           10            5
 |- BASH                 1           15           15            0            0
 (Total)                             30           15           10            5
-------------------------------------------------------------------------------
 Rust                  188        36105        32127          204         3774
 |- Markdown            55          226            0          219            7
 (Total)                          36331        32127          423         3781
===============================================================================
 Total                1250       303558       298107          326         5125
===============================================================================
```
