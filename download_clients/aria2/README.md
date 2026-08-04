# aria2

This plugin drives an aria2 daemon through its XML-RPC endpoint. It is for
torrent acquisition and also accepts direct torrent sources; it is not an NZB
client.

## Connection and scope

Configure the endpoint with **host**, **port** (default 6800), **rpc_path**
(default /rpc), and **use_ssl**. Set **secret_token** when the daemon uses
aria2's RPC token authentication. **directory** is the fallback download
directory; a directory supplied by Scryer for an individual request takes
precedence.

The plugin lists aria2's active, waiting, and stopped results and reports the
content path from aria2's file list. aria2 must therefore retain the completed
result and expose a path that Scryer can reach for import.

## What Scryer can do

It submits magnet URIs, torrent URLs, torrent files, and torrent bytes using
addUri or addTorrent. Directory isolation is supported, along with pause,
resume, and removal of a tracked GID. There are no seed-limit, queue-priority,
start-paused, or force-start controls.

**post_import_action** is either retain (the default) or remove. Remove calls
aria2's completed-result removal after Scryer has imported the media; it does
not delete the imported files. Do not choose it if you need aria2's
stopped-result record to remain available for another purpose.
