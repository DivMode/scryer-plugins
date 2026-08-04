# Pushover

Send push notifications through the Pushover Messages API. It supports Pushover device targeting, emergency delivery settings, sound selection, time-to-live, and the compatible encrypted-message format.

## Configuration

| Setting | Required | Purpose |
| --- | --- | --- |
| **api_key** | Yes | Pushover application token. |
| **user_key** | Yes | Pushover user or group key. |
| **devices** | No | Device names separated by commas, semicolons, or newlines. |
| **priority** | No | Priority from -2 through 2; defaults to 0. |
| **retry**, **expire** | No | Emergency-priority cadence. Retry must be 30–86400 seconds when priority is 2. |
| **ttl** | No | Message lifetime in seconds; zero disables TTL and negative values are rejected. |
| **sound** | No | Pushover sound name. |
| **encryption_key** | No | A 64-character hexadecimal Pushover-compatible key. |

## Encrypted delivery

With **encryption_key**, the plugin compresses, encrypts, and authenticates both the title and message before sending them, and marks the request encrypted. Otherwise it sends readable plaintext. The plugin does not create users, groups, devices, or applications.
