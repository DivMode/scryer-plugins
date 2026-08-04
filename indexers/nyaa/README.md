# Nyaa Indexer

An anime torrent indexer that reads Nyaa's RSS endpoint. It supports recent, RSS, automatic, and interactive searches using title, category, and limit inputs.

## Configure in Scryer

Set **base_url** to a Nyaa-compatible site URL, for example https://nyaa.si. **additional_params** defaults to &cats=1_0&filter=1 and is appended to the RSS request. Enable **anime_standard_format_search** to add SxxExx and season-pack search variants. **minimum_seeders** is a host-side release-selection preference with a default of 1.

The optional transport fields are **user_agent**, **cookie**, **username**, **password**, and **additional_headers**. They support custom headers, HTTP Basic authentication, or a raw Cookie header where a compatible mirror needs them.

## Behavior and limits

The plugin derives RSS queries from title candidates and parses size, info hash, seeders, leechers, peers, and info URLs from the feed. It advertises a two-second rate limit and does not offer provider-native external-ID lookup.
