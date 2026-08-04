# Tribler

This plugin uses Tribler's downloads API for magnet-only acquisition. It
preserves Tribler-specific privacy settings instead of pretending they map to
generic torrent controls.

## Connection and configuration

Configure **host**, **port** (default 20100), optional **url_base**, and
**use_ssl**. **api_key** is sent when the Tribler endpoint requires it.
**category** becomes the fallback child directory beneath Tribler's configured
save location when neither Scryer nor **directory** supplies a destination.
**directory** itself is a fallback that a request can override.

**anonymity_level** supplies the default number of Tribler anonymity hops.
**safe_seeding** defaults to true. Scryer may supply either setting for an
individual torrent; otherwise these configured values are sent to Tribler.

## Behavior and limits

Only magnet URIs are accepted. The plugin reports Tribler's destination and
content paths, can remove a download with or without its data, and exposes the
anonymity and safe-seeding request options. It does not advertise pause,
resume, queue priority, seed limits, start-paused, or post-import actions.

An import acknowledgement does not mutate Tribler. Retention and seeding
remain controlled by the Tribler client after the item has been imported.
