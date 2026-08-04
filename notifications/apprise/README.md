# Apprise

Send Scryer notifications through an [Apprise API](https://github.com/caronc/apprise-api) server. This plugin uses the API's `/notify` endpoint; it does not run the Apprise CLI or parse local configuration files.

## Configure the channel

Set **Server URL** to the Apprise API base URL. Choose exactly one delivery route:

- **Configuration Key** — sends `POST /notify/<key>`. Keys may contain only lowercase letters, digits, and hyphens.
- **Stateless URLs** — sends `POST /notify` with the configured URL list in the request body.

The two modes are mutually exclusive. Tags are supported only with a configuration key, not with stateless URLs.

| Setting | Purpose |
| --- | --- |
| `server_url` | Required Apprise API base URL. |
| `configuration_key` | Optional configured Apprise key; mutually exclusive with `stateless_urls`. |
| `stateless_urls` | Optional multiline Apprise destination URLs; mutually exclusive with `configuration_key`. |
| `notification_type` | Apprise type: `info` (default), `success`, `warning`, or `failure`. |
| `tags` | Comma-separated Apprise tags for a configured key. |
| `include_poster` | Adds the Scryer poster URL as an Apprise attachment when one is available. |
| `auth_username`, `auth_password` | Optional HTTP Basic authentication for the Apprise API server. |

## Delivery

The plugin sends Scryer’s summary title and message as Apprise `title` and `body`. A test notification exercises the same API request. Event subscriptions remain a Scryer channel setting; this plugin has no local queue, retry policy, or filesystem access.
