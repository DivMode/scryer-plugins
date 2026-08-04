# Transmission

This plugin uses Transmission's RPC endpoint, including the required session
ID handshake. It also recognises compatible Vuze/Azureus-style RPC responses
advertised through the same provider aliases.

## Connection and configuration

Set **host**, **port** (default 9091), **use_ssl**, and **url_base** (default
/transmission/), plus optional **username** and **password**. **category**
defaults to scryer-tv and is applied as a Transmission label. **directory** is
the fallback destination, while Scryer can provide one per torrent.

**recent_priority**, **older_priority**, and **add_paused** control queue
placement and initial state. **post_import_category** is an optional label for
completed imports. **post_import_action** is retain, remove, or
remove_with_data.

## Behavior and limits

Magnets and torrent URLs, files, or bytes are supported. The adapter can
isolate work by directory, tag, or category, pause/resume it, remove it with
or without data, and use Transmission's seed ratio and idle-seeding time
controls. It does not advertise force-start.

Post-import category assignment happens before the selected cleanup action.
Retain leaves the torrent and data under Transmission's ordinary seeding
policy; remove_with_data deletes the downloader's data after Scryer reports a
successful import.
