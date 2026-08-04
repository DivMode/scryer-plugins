# Usenet Blackhole

Usenet Blackhole is a local filesystem handoff for an external NZB consumer.
It writes NZBs for that consumer to pick up; it does not speak to a Usenet
server or downloader API.

## Required folders

Set **nzb_folder** to the external client's watched NZB directory and
**watch_folder** to the directory Scryer should scan for finished output. The
plugin host must be able to access both. It writes a sanitized-name .nzb file
to **nzb_folder** from an NZB payload or a fetched NZB URL.

The queue, history, and completed views scan **watch_folder**. An entry is
considered complete after a thirty-second grace period, so configure
**watch_folder** as a stable completed-output location rather than the
downloader's actively changing work directory.

## Lifecycle boundary

There is no live downloader status, pause, resume, or post-import operation.
Removal is a local cleanup only: it requires remove-with-data and deletes the
tracked file or directory in **watch_folder**. The external Usenet application's
retention, repair, extraction, and active download lifecycle remain outside
Scryer's control.
