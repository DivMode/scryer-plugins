# Freebox Download

This plugin controls the BitTorrent portion of the Freebox OS download
manager. It authenticates with Freebox's challenge-response API and retains
the resulting session token in plugin state; a normal HTTP username/password
login is not used.

## Connection and configuration

Set **host** (default mafreebox.freebox.fr), **port** (default 443),
**api_url** (default /api/v1/), and **use_ssl**. The credentials are the
Freebox application pair: **app_id** and **app_token**. They must be created
for an application permitted to use the download API.

**destination_directory** overrides Freebox's configured download directory.
With no explicit destination, **category** is appended to the Freebox default
directory and also scopes polling. **recent_priority** and **older_priority**
can be first; **add_paused** prevents new torrents from starting immediately.

## Behavior and limits

The plugin accepts magnets and torrent URLs, files, or bytes. It supports tag
and directory isolation, per-request directories, queue placement, add-paused,
and a torrent seed-ratio limit. Removal, including remove-with-data, is
available, but pause, resume, force-start, and seed-time limits are not.

Freebox tasks are not changed after a Scryer import. In particular, there is
no imported tag/category or cleanup policy: keeping or deleting the data
remains an explicit Scryer control request or Freebox policy decision.
