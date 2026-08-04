# X (Twitter)

Send Scryer summaries as a public status update or an X direct message using OAuth 1.0a credentials. The implementation calls the legacy Twitter API v1.1 endpoints.

## Configuration

| Setting | Required | Purpose |
| --- | --- | --- |
| **consumer_key**, **consumer_secret** | Yes | OAuth application consumer credentials. |
| **access_token**, **access_token_secret** | Yes | OAuth access credentials for the sending account. |
| **mention** | Depends | Required for direct messages; optional for public statuses, where it is appended as @name. |
| **direct_message** | No | Sends a direct message when true; defaults to true. |

The plugin provides startOAuth and getOAuthToken actions for Scryer’s credential setup flow.

## Delivery

The text is the Scryer title and message joined with a colon. Direct messages are sent to the configured screen name; public mode posts a status and optionally appends the mention. The plugin does not upload media, create threads, shorten text, or support X API v2.
