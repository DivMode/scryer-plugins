# Torrent Blackhole

Torrent Blackhole is a local filesystem handoff for a downloader that watches
files rather than exposing a usable API. Scryer writes torrent material; the
external downloader remains responsible for downloading and seeding it.

## Required folders

Set **torrent_folder** to the watched handoff folder and **watch_folder** to
the directory whose entries Scryer should scan as completed candidates. The
plugin host needs filesystem access to both. A new entry remains in the
initial thirty-second grace period before the plugin treats it as completed.

Torrent bytes and URLs are saved as torrent files. Magnet links are rejected
unless **save_magnet_files** is enabled; then they are written using
**magnet_file_extension** (default .magnet). The filenames are derived from a
sanitized release title.

## Lifecycle boundary

Queue/history/completed views are scans of **watch_folder**, not a live view
of the consuming downloader. There is no pause, resume, status, seed-limit,
or post-import operation. The only control is destructive cleanup: removal
requires remove-with-data and **read_only** must be false (it defaults true);
the plugin deletes the tracked watch-folder file or directory itself.
