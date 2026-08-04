# rqbit

This plugin integrates rqbit's HTTP API. It treats rqbit's torrent info hash
as the stable client item ID and records per-torrent seed limits in plugin
state so they can be evaluated while polling.

## Connection and scope

Configure **host** (default localhost), **port** (default 3030), **use_ssl**,
and **url_base** (default /). There are no authentication settings in this
adapter; place any authentication in a compatible local or reverse-proxy setup
only if the endpoint accepts the requests unchanged.

rqbit's output folder is reported as the completed path. The descriptor has a
directory isolation mode, but the current add endpoint does not accept a
Scryer-provided per-download directory, so configure rqbit's own destination
layout to be importable.

## Behavior and limits

Magnets and torrent URLs, files, or bytes are accepted. The adapter can remove
a torrent with or without data and can observe/enforce seed ratio and seed
time supplied per release. It does not advertise pause, resume, queue
priority, start-paused, force-start, or post-import actions.

Scryer's import acknowledgement is recorded without mutating rqbit. A
completed item remains discoverable only while rqbit continues to report it.
