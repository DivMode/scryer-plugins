# Kodi / XBMC

Show Kodi GUI notifications and perform targeted video-library scans or cleans through JSON-RPC. GUI notification, scan, and clean are independently configurable.

## Connection

| Setting | Required | Purpose |
| --- | --- | --- |
| **host** | Yes | Kodi host name or address. |
| **port** | Yes | JSON-RPC port; defaults to 8080. |
| **use_ssl** | No | Uses HTTPS instead of HTTP. |
| **url_base** | No | JSON-RPC path; defaults to /jsonrpc. |
| **username**, **password** | No | Optional HTTP Basic authentication. |

## Behavior

| Setting | Default | Effect |
| --- | --- | --- |
| **notify** | false | Calls GUI.ShowNotification with the Scryer icon and configured display time. |
| **display_time** | 5 | GUI display time in seconds. |
| **update_library** | false | Calls VideoLibrary.Scan for supported media-change events. |
| **always_update** | false | Allows scans/cleans while Kodi has an active video player. |
| **clean_library** | false | Calls VideoLibrary.Clean for supported events. |

For library work, the plugin normally skips changes while a video player is active. When the event carries a title path, that path scopes the scan; otherwise Kodi receives an unscoped scan. This integration does not support Kodi add-ons, remote file copying, or notification attachments.
