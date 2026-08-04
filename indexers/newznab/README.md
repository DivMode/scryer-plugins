# Newznab Indexer

A generic Usenet indexer for services that implement the Newznab API. It supports recent, RSS, automatic, and interactive searches for movies, series, and anime using title, external-ID, season, episode, category, and limit inputs.

## Configure in Scryer

**base_url** is required. Configure **api_key** for a service that requires one. **api_path** defaults to /api and is useful for endpoints such as /api/v1/api or /nabapi. **additional_params** appends provider-specific query parameters to every request.

## Behavior and limits

The shared adapter follows Newznab pagination and normalizes categories, languages, grabs, info URLs, and provider attributes before returning releases to Scryer. It is protocol-generic: service-specific metadata, quota behavior, and authentication quirks belong in a dedicated provider plugin rather than this adapter.
