# Synology Download Station

This plugin speaks Synology's Download Station task API, discovering the
available API versions and retaining its authenticated DSM session internally.
It supports torrent work only; it is not a general Synology file-management
integration.

## Connection and configuration

Set **host** (default 127.0.0.1), **port** (default 5000), and **use_ssl** for
the DSM endpoint, plus an account in **username** and **password** that can
create and control Download Station tasks. **category** is used to group
Scryer work. Use **directory** when tasks should start in a specific Download
Station destination; request-specific routing overrides that default.

## Behavior and limits

The plugin accepts magnets and torrent URLs, files, or bytes. It exposes
category and directory isolation and reports queued, historical, and completed
tasks with their destination paths. It can pause, resume, and remove tasks,
including removal with data.

There is no plugin-side post-import mutation. An import recorded by Scryer
does not relabel or remove the Download Station task, so retention remains the
NAS administrator's policy. Connection testing authenticates against the
selected DSM API before the client is enabled.
