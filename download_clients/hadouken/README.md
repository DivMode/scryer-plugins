# Hadouken

This plugin talks to Hadouken's Web UI JSON-RPC API. It uses Hadouken labels
as the only isolation boundary, so choose a dedicated category rather than
expecting directory-level scoping.

## Connection and configuration

Configure **host**, **port** (default 7070), optional **url_base**, and
**use_ssl**, together with the Web UI **username** and **password**. The
**category** label defaults to scryer-tv. Queue, history, and completed
polling are filtered to that label when it is non-empty.

## Behavior and limits

Magnets and torrent URLs, files, or bytes are sent through Hadouken's add
operation and labeled on creation. The plugin reports Hadouken's save path
and supports remove and remove-with-data. It does not advertise pause, resume,
per-download directories, queue priority, start-paused, seed-limit, or
post-import controls.

There is no post-import relabel or cleanup action. Keep the selected label and
Hadouken's own storage/retention rules aligned with the paths Scryer is
expected to import from.
