# Gotify

Send push notifications to a Gotify server through its message API. The plugin posts to the configured server’s /message endpoint with the app token as a query parameter.

## Configuration

| Setting | Required | Purpose |
| --- | --- | --- |
| **server** | Yes | Gotify server base URL, such as https://gotify.example. |
| **app_token** | Yes | Gotify application token. |
| **priority** | No | Numeric Gotify priority; defaults to 5. |
| **include_series_poster** | No | Adds the poster URL as Gotify’s big image and switches the message to Markdown. |
| **metadata_links** | No | Comma-, semicolon-, or newline-separated choices from imdb, tvdb, trakt, and tvmaze. |
| **preferred_metadata_link** | No | Metadata link used as Gotify’s click target; defaults to tvdb. |

## Message behavior

The title and text come from Scryer’s normalized notification. When enabled and available, poster and metadata links are appended as Markdown; metadata choices that have no provider ID on the event are skipped. An invalid metadata-link name is rejected before delivery. This is a push-only integration—it does not refresh a media library or make any change in Gotify beyond creating the message.
