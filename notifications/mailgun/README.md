# Mailgun

Send plaintext email through Mailgun’s Messages API. The plugin uses HTTP Basic authentication with the fixed user name api and posts one Mailgun form request per Scryer notification.

## Configuration

| Setting | Required | Purpose |
| --- | --- | --- |
| **api_key** | Yes | Mailgun private API key. |
| **use_eu_endpoint** | No | Selects api.eu.mailgun.net; the default is the US endpoint. |
| **from** | Yes | Sender address or Mailgun-compatible sender value. |
| **sender_domain** | Yes | Mailgun sending domain used in the API path. |
| **recipients** | Yes | Recipient addresses separated by commas, semicolons, or newlines. |

## Delivery

The summary title becomes the email subject and the summary message becomes the plaintext body. All configured recipients are added as individual to fields. The plugin does not render HTML, manage Mailgun templates, or validate that the domain is authorized—Mailgun’s response determines delivery success.
