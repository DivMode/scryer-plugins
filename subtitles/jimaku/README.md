# Jimaku Subtitles

An anime-oriented subtitle catalog for the Jimaku API at https://jimaku.cc/api. It supports movie and episode requests, recommends the anime facet, and returns Japanese and English subtitle candidates, including entries marked as AI-translated by Jimaku.

## Configure in Scryer

**api_key** is required. **enable_name_search_fallback** defaults to true and controls whether an episode without a direct match may be searched by title. Movie searches retain their name-search path even if that option is disabled.

## Search and download behavior

The plugin tries identifier-aware matching before title candidates, constrains work to five candidate entries and at most twelve name queries, then filters the provider's files for the requested episode and language. It does not support file-hash lookup, forced, hearing-impaired, or machine-translated flags.

Downloads use the provider file reference returned during search. For provider rate limiting, the request helper waits only within a 60-second total budget; a rate-limited search returns no candidates, while other provider errors remain visible. Scryer performs final candidate selection and library placement.
