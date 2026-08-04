# Pushbullet

Create Pushbullet note pushes from Scryer event summaries. The plugin can address channels, specific devices, or the account default target.

## Configuration

| Setting | Required | Purpose |
| --- | --- | --- |
| **api_key** | Yes | Pushbullet access token. |
| **channel_tags** | No | One or more channel tags. These take priority over device targets. |
| **device_ids** | No | Device identifiers, separated by commas, semicolons, or newlines. Numeric values are sent as device IDs; other values as device idens. |
| **sender_id** | No | Source device identifier sent as source_device_iden. |

## Targeting and setup help

When channel tags are present, the plugin sends one push to each channel and ignores **device_ids**. Otherwise it sends one push per device; with neither setting, Pushbullet chooses the default target for the access token.

Scryer can invoke the plugin’s **getDevices** action to list available devices after an access token is configured. The send payload is always a Pushbullet note with the Scryer summary title and message; files and links are not attached.
