# Flood

This plugin uses Flood's API to manage the torrent client behind Flood. Its
scope is tag-based: configured tags determine which torrents Scryer considers
when it polls queue, history, and completed work.

## Connection and configuration

Configure **host**, **port** (default 3000), optional **url_base**, **use_ssl**,
**username**, and **password**. The authenticated Flood cookie is stored in
plugin state. **destination** is a fallback directory, while a request-level
directory wins for that download.

**tags** define the scope for Scryer-managed torrents. **additional_tags** are
added to every new torrent. **start_on_add** controls whether Flood begins it
immediately. **post_import_tags** are applied after a successful import rather
than removing the torrent.

## Behavior and limits

Flood accepts magnets and torrent URLs, files, or bytes; it can route by tag
or directory, remove torrents with or without data, and track per-release
seed ratio and seed-time limits. It reports real content paths from Flood.

Pause, resume, queue priority, and start-paused controls are not advertised by
this adapter. Import marking is intentionally tag-based: set
**post_import_tags** if an imported torrent should continue seeding while being
visibly separated from active acquisition.
