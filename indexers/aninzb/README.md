# AniNZB Indexer

An anime and series Usenet indexer for AniNZB's public JSON API. It supports title, TVDB, and AniDB searches together with recent, RSS, automatic, and interactive feeds.

## Configuration

AniNZB exposes only a legacy **Base URL** field so it remains installable on older Scryer versions. Its value is ignored: the plugin always uses `https://api.aninzb.moe/` with a browser-style User-Agent and fixed respectful request pacing. No API key, path, additional-parameters, or rate-limit controls are exposed. Existing AniNZB/Newznab settings are accepted for compatibility but ignored.

## Behavior and limits

AniNZB is deliberately not a movie source: movie-shaped requests return an empty result set. Each search makes one API request and returns at most 50 results. The first request is sent immediately; later requests are locally paced to no more than two per second. The plugin uses bounded retries for provider throttling and stops querying when its fixed local request budget is exhausted.

The API may return nullable metadata. Usable entries are normalized as Usenet releases when they include both a filename and NZB URL. Source, series aliases, poster, subtitle, screenshot, and thumbnail links are preserved as provider metadata; selection, scoring, and submission to a download client remain Scryer's responsibility.
