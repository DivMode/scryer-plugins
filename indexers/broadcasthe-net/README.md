# BroadcasTheNet Indexer

A private-torrent indexer that queries BroadcasTheNet's JSON-RPC API. It is for series searches and understands TVDB and TVRage identifiers, seasons, episodes, and result limits.

## Configure in Scryer

Set **api_key** to a BroadcasTheNet API key. **base_url** defaults to https://api.broadcasthe.net. **minimum_seeders** is exposed as a host-side release-selection preference with a default of 1; it is not a tracker-ratio or seeding control.

## Behavior and limits

Recent, RSS, automatic, and interactive search modes are supported. The plugin queries up to ten pages of 100 entries, de-duplicates the results, and advertises a five-second rate limit. It returns seeders, peers, torrent IDs, info URLs, and private-tracker/seed-requirement signals when the API provides them. It does not produce magnet links, manage account ratio, or manage the torrent after Scryer submits it to a download client.
