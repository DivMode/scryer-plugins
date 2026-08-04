# FileList Indexer

A private-torrent indexer for FileList's JSON search endpoint. It supports recent, RSS, automatic, and interactive series searches, including text, season, episode, absolute-episode, and IMDb inputs.

## Configure in Scryer

**username** and **passkey** are required. Keep the passkey in Scryer's protected credential field, not in a shared URL. **base_url** defaults to https://filelist.io.

**categories** defaults to 23,21,27; **anime_categories** is an optional separate comma-separated list. At least one of those two category fields must resolve to an ID. **minimum_seeders** is a host-side release-selection preference with a default of 1.

## Behavior and limits

The plugin queries configured categories and de-duplicates the result set before returning at most 200 entries. It preserves seeders, peers, comments, grab counts, private-tracker flags, seed requirements, and info URLs where FileList supplies them. It does not expose an info hash or magnet URI, and it does not manage account ratio or post-submission seeding.
