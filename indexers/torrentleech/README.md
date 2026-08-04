# TorrentLeech Indexer

A private-torrent RSS adapter for TorrentLeech. It exposes recent and RSS feeds only; it does not implement tracker search for automatic or interactive acquisition.

## Configure in Scryer

**api_key** is required and is the TorrentLeech RSS key. **base_url** defaults to http://rss.torrentleech.org; the plugin constructs the feed by appending the RSS key. **minimum_seeders** is a host-side release-selection preference with a default of 1.

Optional RSS transport fields are **user_agent**, **cookie**, **username**, **password**, and **additional_headers**. Use the latter for any tracker-required request headers, one Name: value pair per line.

## Behavior and limits

The adapter parses the current feed as torrent releases, including seeders, info URLs, private-tracker flags, and seed requirements where available. It has a two-second rate-limit hint. It neither manages a TorrentLeech account nor controls the torrent after Scryer sends it to a download client.
