# NZBGeek Indexer

An NZBGeek-specific Usenet indexer using its Newznab-compatible API. It supports recent, RSS, automatic, and interactive searches for movies, series, and anime, including IMDb and TVDB lookups.

## Configure in Scryer

**base_url** defaults to https://api.nzbgeek.info. Set **api_key** for the NZBGeek account. The common adapter also accepts **api_path** (default /api) and **additional_params** for non-default API routing or query parameters.

## Behavior and limits

Searches use up to ten pages of 100 results. The plugin preserves language, subtitle, grab, comment, password, and protection hints from NZBGeek attributes. It contributes NZBGeek vote and language signals to Scryer's provider scoring policy, while Scryer still owns the final acquire-or-reject decision.
