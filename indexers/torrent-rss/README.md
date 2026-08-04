# Torrent RSS Indexer

A generic adapter for torrent tracker and aggregator RSS feeds. It supports recent and RSS feeds only; it cannot issue interactive or automatic search queries to a tracker.

## Configure in Scryer

**feed_url** is required. **download_preference** chooses which available feed download reference to use when an item provides more than one. For protected feeds, configure optional **username** and **password** for HTTP Basic authentication, **cookie** for a raw Cookie header, **user_agent**, and **additional_headers** as one Name: value pair per line.

## Behavior and limits

The plugin fetches the current feed, parses its torrent entries, filters them against the requested feed criteria, and returns at most 200 results. Its two-second rate-limit hint applies to feed requests. Matching quality depends on the names and metadata present in the feed; this plugin has no tracker-specific search or account-management behavior.
