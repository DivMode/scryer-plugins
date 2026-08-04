# Tsukihime Indexer

An anime indexer for Tsukihime's public v1 API. It can return mixed torrent and Usenet releases and supports recent, RSS, automatic, and interactive anime searches through text, season, episode, absolute-episode, AniDB, AniList, and MyAnimeList data.

## Configure in Scryer

No credentials are used. **base_url** defaults to https://api.tsukihime.org/v1. **max_results** defaults to 50 and is capped at 100. Enable **include_adult** only when adult releases should be eligible.

## Behavior and limits

The plugin retains the source type for every returned release so Scryer can route it to a compatible downloader. It supports info hashes, magnet URIs, languages, subtitles, info URLs, and raw provider metadata when supplied by the API.

Tsukihime's API budget is enforced locally: general API work is limited to 60 requests per minute and search requests to 25 per minute. When the budget is exhausted the plugin returns an empty result set rather than continuing to call the public API.
