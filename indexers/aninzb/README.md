# AniNZB Indexer

An anime and series Usenet indexer for AniNZB's Newznab-compatible service. It supports title, TVDB, and AniDB searches together with recent, RSS, automatic, and interactive feeds.

## Configure in Scryer

**base_url** defaults to https://aninzb.moe; the shared Newznab fields **api_key**, **api_path** (default /api), and **additional_params** are available for the service configuration. **hourly_hit_cap** and **daily_hit_cap** control the plugin's local request budgets; their defaults are 500 and 3,000 respectively.

## Behavior and limits

AniNZB is deliberately not a movie source: movie-shaped requests return an empty result set. The plugin limits Newznab pages to 100 items, spaces requests by three seconds, and uses bounded retries for provider throttling. When either configured hit budget is exhausted, it returns no results rather than continuing to query the service.

Returned entries are normalized as Usenet releases with available grab, comment, provider metadata, and info-link fields. Selection, scoring, and submission to a download client remain Scryer's responsibility.
