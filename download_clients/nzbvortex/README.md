# NZBVortex

This plugin integrates NZBVortex through its authenticated HTTP API. It
validates the selected NZBVortex group before use and tracks the server's
add UUID as the downloader-owned item ID.

## Connection and configuration

Configure **host**, optional **url_base**, and **use_ssl** (default true), then
provide the required **api_key**. **category** is the NZBVortex group used for
new jobs and for queue/history filtering; it defaults to TV Shows and must
exist in NZBVortex. **recent_priority** and **older_priority** accept -1, 0,
or 1 for low, normal, or high priority.

## Behavior and limits

Only NZB payloads and NZB URLs are supported. URLs are fetched by the plugin
and submitted as multipart NZB uploads. Jobs are routed by group, and the
plugin reports NZBVortex destination paths for completed work. It can remove a
job with or without data and acknowledges Scryer's import marker.

Pause, resume, queue controls, per-download directories, and plugin-managed
post-import cleanup are not available. An import therefore does not alter the
NZBVortex job or its retained output.
