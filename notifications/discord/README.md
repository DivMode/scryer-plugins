# Discord

Deliver event notifications to a Discord incoming webhook. The plugin posts a rich embed rather than a plain message: it includes the event-specific heading, Scryer summary, available release quality/indexer and download-client details, a poster thumbnail, and the event timestamp.

## Configuration

| Setting | Required | Purpose |
| --- | --- | --- |
| **webhook_url** | Yes | Discord incoming-webhook URL. |
| **username** | No | Overrides the webhook display name. |
| **avatar** | No | Avatar image URL for the webhook message. |
| **author** | No | Embed author label; defaults to the Scryer application name. |

The plugin supplies a Scryer icon for the embed author and uses event-aware colors: warnings for grabs, health issues, and manual intervention; red for deleted media; green for successful imports and additions.

## What it sends

Discord receives an embed with the title name when Scryer provides it, otherwise the notification summary. The notification body appears as an embed field, and optional metadata is added only when present in the Scryer event. The webhook channel controls where delivery occurs; this plugin does not manage Discord channels, roles, or mentions.
