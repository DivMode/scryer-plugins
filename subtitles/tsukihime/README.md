# Tsukihime Subtitles

An anime-focused catalog provider that finds subtitle attachments cached alongside releases in Tsukihime's public v1 API. It supports movie and episode requests, recommends the anime facet, and can return forced tracks.

## Configure in Scryer

No credentials are used. **base_url** defaults to https://api.tsukihime.org/v1. **max_results** defaults to 50; **max_detail_fetches** limits per-torrent detail requests; **include_adult** defaults to false.

## Search and download behavior

The plugin resolves available AniDB, AniList, or MyAnimeList identifiers before falling back to title search, then examines matching completed torrent records for cached subtitle tracks. It filters candidates by requested language and supports the languages advertised by Tsukihime, including English, Japanese, Chinese, and common European languages.

Downloads come only from the declared Tsukihime storage origin. Compressed subtitle payloads are capped at 2 MiB and decompressed payloads at 16 MiB. The client locally respects Tsukihime's public budgets of 60 API requests and 25 search requests per minute; when exhausted it returns no candidates rather than continuing to query the service.
