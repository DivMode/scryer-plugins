# HDBits Indexer

A private-torrent indexer for the HDBits JSON API. It supports recent, RSS, automatic, and interactive series searches with TVDB, season, episode, and result-limit inputs.

## Configure in Scryer

**username** and **api_key** are required; the latter is HDBits' API key/passkey. **base_url** defaults to https://hdbits.org.

**categories** defaults to 2,3. Optional **codecs** and **mediums** accept comma-separated HDBits IDs to narrow results. **minimum_seeders** is a host-side release-selection preference with a default of 1.

## Behavior and limits

The plugin sends a JSON query to HDBits and returns at most 100 entries. It keeps seeders, peers, info hashes, comments, grab counts, private-tracker flags, and seed-requirement metadata when present. It does not construct magnet URIs or operate the tracker account beyond the search API; download and seeding are handled by Scryer's configured torrent client.
