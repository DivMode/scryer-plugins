# Deluge

This plugin uses Deluge's Web JSON-RPC API. It manages only torrents within
the configured category, so an existing Deluge deployment can coexist with
other work without Scryer treating every torrent as its own.

## Connection and configuration

Set **host**, **port** (default 8112), optional **url_base**, and **use_ssl**
for the Web UI endpoint. **password** is the Deluge Web password (default
deluge), not a daemon credential. The plugin caches its Web API session in
plugin state.

**category** defaults to scryer-tv and scopes listing and adds. Optional
**download_directory** and **completed_directory** bound the paths it reports.
**add_paused**, **recent_priority**, and **older_priority** control the initial
torrent state and position. **post_import_category** can relabel an imported
torrent; **post_import_action** supports retain, remove, and remove_with_data.

## Behavior and limits

Magnets and torrent URLs, files, or bytes are supported. Scryer can route a
torrent by tag/category or directory, remove it (with or without data), set
queue placement, and request seed ratio/time limits. Deluge's descriptor does
not advertise pause or resume controls, even though it reports paused state.

On a successful import, the optional imported category is applied before the
selected cleanup action. Keeping the action at retain leaves Deluge's seeding
policy intact; remove_with_data is deliberately destructive to the
downloader's content.
