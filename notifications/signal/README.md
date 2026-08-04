# Signal

Send messages through a signal-cli REST API compatible server. The plugin posts to that server’s v2 send endpoint; it does not connect to Signal directly.

## Configuration

| Setting | Required | Purpose |
| --- | --- | --- |
| **host** | Yes | signal-cli REST server host; defaults to localhost. |
| **port** | Yes | Server port; defaults to 8080. |
| **use_ssl** | No | Uses HTTPS instead of HTTP. |
| **sender_number** | Yes | Registered Signal sender number. |
| **receiver_id** | Yes | Recipient phone number or Signal group ID. |
| **auth_username**, **auth_password** | No | HTTP Basic authentication. Both are required before an Authorization header is sent. |

## Delivery

The message is three lines: Scryer summary title, summary message, and a trailing newline. The plugin sends one recipient per configuration and has no group lookup, attachment upload, or local signal-cli process management.
