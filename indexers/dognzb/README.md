# DogNZB Indexer

A DogNZB-specific Usenet indexer using its Newznab-compatible API. It supports recent, RSS, automatic, and interactive movie, series, and anime searches, including IMDb and TVDB lookups.

## Configure in Scryer

The default **base_url** is https://api.dognzb.cr. Configure **api_key** when your DogNZB account requires it. The shared Newznab settings are **api_path** (default /api) and **additional_params**; **base_url** is the only field the common adapter requires syntactically.

## Behavior and limits

Search results are capped at 100 per page and at ten pages. Besides ordinary Newznab fields, the plugin extracts DogNZB rating, genre, and comment metadata and supplies DogNZB's rating signal to Scryer's provider scoring policy. It does not make the final scoring or acquisition decision.
