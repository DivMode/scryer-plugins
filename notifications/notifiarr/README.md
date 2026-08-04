# Notifiarr

Forward Scryer lifecycle events to Notifiarr using its Sonarr-compatible notification endpoint. The plugin serializes Scryer’s normalized event into the structured webhook payload expected by that endpoint.

## Configuration

| Setting | Required | Purpose |
| --- | --- | --- |
| **api_key** | Yes | Notifiarr API key, sent as the X-API-Key request header. |

## Delivery semantics

Requests are sent to the fixed Notifiarr API URL; there is no configurable server or custom webhook path. A provider HTTP 400 is treated as a successful delivery with a warning, reflecting Notifiarr’s response compatibility behavior. Other delivery failures remain failures.

This plugin is an event-forwarder only. Notification selection and Notifiarr destination routing are configured in Scryer and Notifiarr respectively.
