# Telegram

Send HTML-formatted event messages through the Telegram Bot API.

## Configuration

| Setting | Required | Purpose |
| --- | --- | --- |
| **bot_token** | Yes | Telegram bot token. |
| **chat_id** | Yes | Destination chat, group, or channel identifier. |
| **topic_id** | No | Forum-topic ID. It must be an integer greater than 1. |
| **send_silently** | No | Sets Telegram’s disable-notification flag. |
| **include_app_name_in_title** | No | Prefixes the title with the Scryer application name. |
| **include_instance_name_in_title** | No | Adds the Scryer application name after the title. |

## Delivery

The plugin sends bold, HTML-escaped title text followed by an HTML-escaped message; link previews are disabled. It uses Telegram’s sendMessage endpoint and sends no media, buttons, or attachments. Invalid or empty topic IDs are rejected before the request.
