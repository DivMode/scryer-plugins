# OpenSubtitles

A catalog subtitle provider for OpenSubtitles movies and episodes. It authenticates with the OpenSubtitles API, searches by the available title, identifier, language, and optional media-file hash data, then downloads the selected provider file.

## Configure in Scryer

**username** and **password** are required OpenSubtitles account credentials. For Scryer's built-in plugin, **api_key** is a host-bound credential supplied by SMG; users should not paste a separate API key into the plugin settings. **enable_hash_lookup** defaults to true and permits file-hash matching when Scryer has a usable hash.

Validation clears any cached session and verifies an authenticated API session with the current credentials.

## Search and download behavior

The provider supports movies and episodes, media-file hash lookup, forced subtitles, hearing-impaired subtitles, and AI- or machine-translated labels where OpenSubtitles reports them. Requested languages are normalized to OpenSubtitles' language codes before search.

The plugin owns the short-lived authentication token and refreshes it when credentials change or the session expires. It observes provider rate-limit responses with a bounded wait; Scryer selects the candidate and handles the eventual library workflow.
