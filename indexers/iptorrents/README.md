# IPTorrents Indexer

An RSS-only private-torrent adapter for IPTorrents. It provides recent and RSS feeds; it does not implement interactive or automatic title search.

## Configure in Scryer

**feed_url** is required and must be IPTorrents' direct-download RSS URL containing **;download**. The plugin rejects a browse-only RSS URL because its entries would not be importable as torrents. **minimum_seeders** is a host-side release-selection preference with a default of 1.

Optional RSS transport fields are **user_agent**, **cookie**, **username**, **password**, and **additional_headers**. Username/password use HTTP Basic authentication, **cookie** is a raw Cookie header, and extra headers are supplied one Name: value pair per line.

## Behavior and limits

The feed is parsed as torrent results with provider category metadata, info URLs, and private-tracker/seed-requirement signals when available. It is intentionally limited by IPTorrents' current feed contents; Scryer matches, ranks, and submits eligible entries to the configured torrent client.
