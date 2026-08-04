# Slack

Post Scryer event notifications to a Slack incoming webhook. Every delivery contains a text fallback plus a colored attachment with the event summary.

## Configuration

| Setting | Required | Purpose |
| --- | --- | --- |
| **webhook_url** | Yes | Slack incoming-webhook URL. |
| **username** | Yes | Display name used for the message; defaults to Scryer. |
| **icon** | No | Emoji name enclosed in colons, or an image URL. |
| **channel** | No | Channel override accepted by the configured webhook. |

## Message layout

The attachment title uses the Scryer title name where available; the notification summary is the fallback. Imports and upgrades use a green attachment, grabs and health/manual-interaction events use a warning color, and deletions use danger. The plugin does not support Slack blocks, threads, mentions, files, or interactive actions.
