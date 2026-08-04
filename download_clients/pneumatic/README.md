# Pneumatic

Pneumatic is a local, filesystem-backed handoff plugin for Kodi's Pneumatic
add-on. It writes an NZB and a matching STRM launcher; it does not communicate
with a remote download service.

## Required folders

Set **nzb_folder** to the location where the add-on can read NZB files and
**strm_folder** to the location it scans for generated STRM files. Scryer's
plugin host must have read/write access to both. Connection testing creates
and verifies these folders.

For each add, the plugin writes a sanitized-name .nzb file to **nzb_folder**
and a .strm file to **strm_folder** containing Pneumatic's add-file launcher
URI. It accepts an NZB payload or fetches an NZB URL. Full-season releases are
intentionally rejected.

## Lifecycle boundary

Queue, history, and completed lists are simply scans of **strm_folder**; the
STRM path is the client item ID and reported output path. There is no remote
completion signal, pause/resume, removal, or post-import mutation. Scryer
therefore requires host filesystem access and cannot infer whether Kodi or
another process has actually consumed the NZB.
