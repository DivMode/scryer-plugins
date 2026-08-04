# ntfy

Publish Scryer notifications to one or more [ntfy](https://ntfy.sh) topics. The plugin sends a separate HTTP POST for each configured topic.

## Configuration

| Setting | Required | Purpose |
| --- | --- | --- |
| **server_url** | No | ntfy server base URL; defaults to https://ntfy.sh. |
| **topics** | Yes | Comma-, semicolon-, or newline-separated topic names. |
| **access_token** | No | Bearer token authentication. |
| **username**, **password** | No | HTTP Basic authentication; provide both or neither. |
| **priority** | No | ntfy priority from 1 through 5; defaults to 3. |
| **tags** | No | Comma-separated ntfy tags or emoji. |
| **click_url** | No | URL opened by compatible ntfy clients. |
| **headers** | No | Additional headers, one per line as Header-Name: value. |

## Delivery

The Scryer title, message, priority, tags, and optional click URL become ntfy request parameters. Topic names are validated before delivery, including the provider’s reserved-name rules. If an access token is configured it takes precedence over Basic authentication. There is no topic discovery or subscription management in the plugin.
