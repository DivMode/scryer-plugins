# rTorrent

This plugin calls rTorrent's XML-RPC interface, using rTorrent's custom1
field as the category/tag boundary. It supports basic HTTP authentication when
the RPC endpoint is protected.

## Connection and configuration

Set **host**, **port** (default 8080), optional **url_base** (default RPC2),
and **use_ssl**, plus optional **username** and **password**. **category**
defaults to scryer-tv and scopes polling. **directory** provides a fallback
destination; an individual Scryer directory overrides it.

**recent_priority** and **older_priority** map to rTorrent priority levels.
With **add_stopped** enabled, new torrents load without starting. The optional
**post_import_category** moves an imported torrent into that custom1 category.

## Behavior and limits

Magnets and torrent URLs, files, or bytes are supported. Scryer can use tag
or directory isolation, remove a torrent without deleting its files, and
apply seed-ratio or seed-time limits. The adapter does not advertise pause,
resume, removal with data, or force-start.

An import always adds the torrent to rTorrent's scryer_imported view. It can
also preserve seeding while moving the torrent into **post_import_category**.
If that setting is empty, the item remains in its original category and no
post-import cleanup is performed.
