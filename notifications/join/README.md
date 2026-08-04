# Join

Send push notifications through Join by João Dias. The plugin calls Join’s hosted messaging endpoint directly and includes the Scryer icon as both the normal and small icon.

## Configuration

| Setting | Required | Purpose |
| --- | --- | --- |
| **api_key** | Yes | Join API key. |
| **device_names** | No | Comma-separated Join device names. Omit it to target group.all. |
| **device_ids** | No | Deprecated imported setting. If present, delivery fails and asks you to use **device_names**. |
| **priority** | No | Join numeric priority; defaults to 0. |

## Delivery

Each event becomes a single Join request with the Scryer summary title and message. Device names are sent as Join’s deviceNames target; this plugin does not enumerate devices or translate legacy device IDs.
