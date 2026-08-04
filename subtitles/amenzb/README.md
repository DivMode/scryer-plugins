# ameNZB Subtitles

An anime episode subtitle catalog backed by ameNZB's Newznab releases and their public subtitle attachments. It validates configuration, searches, and downloads; it is not a generic movie or series subtitle provider.

## Configure in Scryer

**api_key** is required and is IP-pinned by ameNZB. **base_url** defaults to https://amenzb.moe and **api_path** defaults to /api. The search controls are **max_results** (capped at 100), **max_detail_fetches**, **category** (default 5070 for anime), and **healthy_only**.

Set **hourly_hit_cap** and **daily_hit_cap** to restrict locally tracked API use; the defaults are 450/hour and 9,000/day. The plugin reports an invalid configuration when its required key is missing.

## Search and download behavior

Only episode requests in the anime facet are considered. The provider does not support a media-file hash lookup, forced, hearing-impaired, or translated-subtitle filtering. It uses available anime identifiers and release metadata to find attachments, then filters candidates by the requested language.

Downloads are limited to ameNZB's configured origin, including redirects, and subtitle payloads are capped at 2 MiB. A local rate-limit budget or a provider throttle produces no candidates instead of a cross-provider fallback. Scryer remains responsible for selecting and installing the returned subtitle.
