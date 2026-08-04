# uTorrent

This plugin drives the uTorrent Web UI API. It obtains and caches the Web UI
token and cookie, then uses uTorrent labels as Scryer's isolation boundary.

## Connection and configuration

Set **host**, **port** (default 8080), optional **url_base**, and **use_ssl**,
plus the Web UI **username** and **password**. **category** defaults to
scryer-tv and filters queue/history/completed polling. The optional
**post_import_category** is the label assigned after a successful import.

**recent_priority** and **older_priority** choose whether recent or older
releases are moved to the top of uTorrent's queue. **initial_state** defaults
to start and is sent as the Web UI action after adding a torrent.

## Behavior and limits

Magnets and torrent URLs, files, or bytes are supported. Scryer can use tag
isolation, pause/resume or force-start a torrent, remove it with or without
data, and apply seed ratio or seed-time limits. uTorrent has no
per-download-directory feature through this adapter, so its own download
layout must be usable for imports.

Post-import handling is label-based: a non-empty **post_import_category**
moves the torrent to that label while it can continue seeding. No automatic
file deletion occurs merely because Scryer records the import.
