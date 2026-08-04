# ameNZB Indexer

An anime-focused Usenet indexer for ameNZB's Newznab API. It supports recent and RSS feeds plus automatic and interactive searches for anime, series, and movies. Anime lookups can use AniDB, TVDB, or an exact torrent info hash.

## Configure in Scryer

**api_key** is required and ameNZB pins keys to the caller IP. **base_url** defaults to https://amenzb.moe; **api_path** defaults to /api. The usual Newznab **additional_params** field is available for provider query parameters.

The plugin also accepts:

- **page_size** — 1–100 results per API page (default 50).
- **category** — default Newznab category, 5070 for anime.
- **healthy_only** — sends ameNZB's healthy=1 filter.
- **audio_lang**, **sub_lang**, **translation**, **source**, **resolution**, and **release_group** — provider-specific filters.
- **hourly_hit_cap** and **daily_hit_cap** — local request budgets, defaulting to 450/hour and 9,000/day.

## Behavior and limits

Searches are paced and retry rate-limit responses, but stop with no results once the local hit budget is exhausted. A search uses at most two API pages and never asks ameNZB for more than 100 entries per page. Results retain provider metadata such as language, grabs, comments, and info URLs; Scryer still makes the final release and download-client decision.
