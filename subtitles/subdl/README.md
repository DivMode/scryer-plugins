# Subdl Subtitles

A catalog subtitle provider for Subdl's API at https://api.subdl.com/api/v1. It supports movie and episode lookups using title, year, IMDb, TMDB, season, episode, and requested-language data.

## Configure in Scryer

**api_key** is required and comes from a Subdl account. Validation sends a small movie probe to the API, treating a not-found probe as a reachable service but surfacing authentication and other provider failures.

## Search and download behavior

Subdl candidates retain forced and hearing-impaired flags where supplied. The provider does not offer media-file hash lookup or AI/machine-translation flags through this plugin. Searches can return season-pack candidates when an individual episode file is unavailable.

The selected artifact is fetched from Subdl's download host and returned with its provider filename, content type, and format. Archives are deliberately preserved for Scryer's normal archive handling rather than unpacked inside the plugin. Provider rate-limit waits are capped at ten seconds; retries are bounded.
