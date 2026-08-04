# Prowl

Send iOS push notifications through the Prowl public API.

## Configuration

| Setting | Required | Purpose |
| --- | --- | --- |
| **api_key** | Yes | Prowl API key. |
| **priority** | No | Numeric Prowl priority; defaults to 0. |

## Delivery

The plugin posts one form request to Prowl. The Scryer application name becomes the Prowl application field, the notification summary becomes the event field, and the summary message becomes the description. It does not support device targeting, attachments, or HTML content.
