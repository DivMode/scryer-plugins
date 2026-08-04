# Emby / MediaBrowser

Notify an Emby or MediaBrowser server and request targeted media updates after Scryer lifecycle events. The two behaviors are independent: admin notifications are optional, while library updates are enabled by default.

## Configuration

| Setting | Required | Purpose |
| --- | --- | --- |
| **host** | Yes | Server host name or address. |
| **port** | Yes | Server port; defaults to 8096 and must be positive. |
| **use_ssl** | No | Uses HTTPS instead of HTTP. |
| **url_base** | No | Base-path segment for reverse-proxy deployments. |
| **api_key** | Yes | MediaBrowser API key, sent as X-MediaBrowser-Token. |
| **notify** | No | Posts the Scryer title and summary to /Notifications/Admin; defaults to false. |
| **update_library** | No | Sends targeted update requests; defaults to true. |
| **map_from**, **map_to** | No | Optional one-pair path translation. Both values must be set together. |

## Library updates

For eligible import, rename, delete, and title events, the plugin queries matching Series items and posts changed paths to /Library/Media/Updated. It prefers provider-ID matches and falls back to an exact title-name match. Imported/deleted events are marked Created/Deleted; renames are Modified.

The configured mapping is applied to every outgoing path. This is not a full library scan, and the plugin does not retain filesystem state; an event without a usable matching path can produce no media-update request.
