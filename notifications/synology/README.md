# Synology Indexer

Update a local Synology media index after Scryer media changes. This plugin invokes the Synology host binary /usr/syno/bin/synoindex; it is not a web or push-notification provider.

## Configuration

| Setting | Required | Purpose |
| --- | --- | --- |
| **update_library** | No | Runs synoindex for supported events; defaults to true. |

## Commands and requirements

The plugin requires Scryer to grant host-process access and requires synoindex to exist at the fixed Synology path. It runs with a 20-second timeout.

- Imports and upgrades add new media paths and remove paths reported as deleted.
- Import completion, rename, and title addition rescan the title path recursively.
- File deletion removes the affected media path.
- Title deletion removes the title path from the index.
- A test notification runs synoindex --help.

No network credentials, server URL, or remote Synology connection are used. Non-zero exit status, output on stderr, or unexpected stdout is reported as a delivery failure.
