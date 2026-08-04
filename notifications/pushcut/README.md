# Pushcut

Trigger a named Pushcut notification from Scryer. The request can carry a poster image and deep-link actions derived from the title’s metadata IDs.

## Configuration

| Setting | Required | Purpose |
| --- | --- | --- |
| **notification_name** | Yes | Name of the Pushcut notification to trigger. |
| **api_key** | Yes | Pushcut API key. |
| **time_sensitive** | No | Marks the Pushcut notification time-sensitive. |
| **include_poster** | No | Adds the event’s poster URL when one is available. |
| **metadata_links** | No | Comma-, semicolon-, or newline-separated choices from imdb, tvdb, trakt, and tvmaze. |

## Payload

The summary title and message become Pushcut’s title and text. Each selected metadata link with an available provider ID becomes an action in the Pushcut payload. The plugin sends only to the configured named notification; it does not create, edit, or list Pushcut notifications.
