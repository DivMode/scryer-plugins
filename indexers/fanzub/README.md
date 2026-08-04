# Fanzub Indexer

An anime Usenet indexer that reads Fanzub's RSS feed. It supports recent and RSS feeds as well as automatic and interactive anime searches.

## Configure in Scryer

**base_url** is the Fanzub RSS URL and defaults to https://fanzub.com/rss.php. Enable **anime_standard_format_search** to add SxxExx and season-pack query variants.

The shared RSS transport fields are optional: **user_agent**, **cookie**, **username**, **password**, and **additional_headers**. Username and password are HTTP Basic authentication; **cookie** is a raw Cookie header; extra headers use one Name: value pair per line.

## Behavior and limits

The plugin requests the anime feed with up to 100 entries, uses RSS enclosure URLs and lengths, and advertises a two-second rate limit. It is a feed adapter, not a full provider API: availability and search precision are constrained by what Fanzub publishes in its RSS feed.
