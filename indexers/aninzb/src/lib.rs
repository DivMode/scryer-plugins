use std::collections::{HashMap, HashSet};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use indexer_command_compat::{LogLevel, log};
use newznab_common::{
    Capabilities, IndexerCategoryModel, IndexerCategoryValueKind, IndexerDescriptor,
    IndexerFeedMode, IndexerLimitCapabilities, IndexerProtocol, IndexerResponseFeatures,
    IndexerSearchInput, IndexerSourceKind, NewznabHitBudget, NewznabHttpBehavior, PluginDescriptor,
    PluginSearchSubjectKind, ProviderDescriptor, SDK_VERSION, SearchRequest, SearchResponse,
    SearchResult, current_sdk_constraint, is_hit_budget_exhausted_error, polite_http_get,
};
use scryer_plugin_pdk::*;
use scryer_plugin_sdk::{ConfigFieldDef, ConfigFieldRole, ConfigFieldType, ConfigFieldValueSource};
use serde::Deserialize;
use url::Url;

const ANINZB_API_BASE_URL: &str = "https://api.aninzb.moe/";
const ANINZB_API_HOST: &str = "api.aninzb.moe";
const LEGACY_BASE_URL_DEFAULT: &str = "https://aninzb.moe";
const API_MAX_RESULTS: usize = 50;
const MAX_API_RESPONSE_BYTES: usize = 20 * 1024 * 1024;
const API_REQUESTS_PER_SECOND: u32 = 2;
const API_REQUEST_INTERVAL: Duration =
    Duration::from_millis(1_000 / API_REQUESTS_PER_SECOND as u64);
const API_REQUEST_PACING_VAR: &str = "aninzb.api_request_pacing";
const DEFAULT_HOURLY_HIT_CAP: u32 = 500;
const DEFAULT_DAILY_HIT_CAP: u32 = 3000;
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 \
    (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";

#[derive(Clone, Debug, Default)]
struct LegacyAniNzbConfig {
    base_url: Option<String>,
    api_key_present: bool,
    api_path: Option<String>,
    additional_params: Option<String>,
    hourly_hit_cap: Option<String>,
    daily_hit_cap: Option<String>,
}

impl LegacyAniNzbConfig {
    fn from_host() -> Self {
        Self {
            base_url: config_optional("base_url"),
            api_key_present: config::get("api_key")
                .ok()
                .flatten()
                .is_some_and(|value| !value.trim().is_empty()),
            api_path: config_optional("api_path"),
            additional_params: config_optional("additional_params"),
            hourly_hit_cap: config_optional("hourly_hit_cap"),
            daily_hit_cap: config_optional("daily_hit_cap"),
        }
    }

    fn is_present(&self) -> bool {
        self.base_url.is_some()
            || self.api_key_present
            || self.api_path.is_some()
            || self.additional_params.is_some()
            || self.hourly_hit_cap.is_some()
            || self.daily_hit_cap.is_some()
    }
}

#[derive(Clone, Debug)]
struct AniNzbConfig {
    api_base_url: &'static str,
    http_behavior: NewznabHttpBehavior,
}

impl AniNzbConfig {
    fn from_host() -> Self {
        let legacy = LegacyAniNzbConfig::from_host();
        if legacy.is_present() {
            log!(
                LogLevel::Debug,
                "AniNZB legacy configuration ignored; using fixed public API"
            );
        }
        migrate_legacy_config(legacy)
    }
}

fn migrate_legacy_config(_legacy: LegacyAniNzbConfig) -> AniNzbConfig {
    AniNzbConfig {
        api_base_url: ANINZB_API_BASE_URL,
        http_behavior: NewznabHttpBehavior {
            plugin_id: "aninzb".to_string(),
            user_agent: USER_AGENT.to_string(),
            pre_request_delay: Duration::ZERO,
            retry_total_budget: Duration::from_secs(300),
            retry_default_delay: Duration::from_secs(60),
            retry_max_delay: Duration::from_secs(300),
            retry_max_attempts: 5,
            max_search_pages: 1,
            hit_budget: Some(NewznabHitBudget {
                var_key: "aninzb.http_hits".to_string(),
                hourly_limit: DEFAULT_HOURLY_HIT_CAP,
                daily_limit: DEFAULT_DAILY_HIT_CAP,
            }),
        },
    }
}

#[derive(Debug, Deserialize)]
struct AniNzbApiResponse {
    #[serde(default, rename = "total_count")]
    _total_count: Option<u64>,
    #[serde(default)]
    items: Option<Vec<AniNzbApiItem>>,
}

#[derive(Debug, Deserialize)]
struct AniNzbApiItem {
    #[serde(default)]
    source: Option<String>,
    #[serde(default)]
    id: Option<u64>,
    #[serde(default)]
    filename: Option<String>,
    #[serde(default)]
    anidb: Option<u64>,
    #[serde(default)]
    series_name: Option<Vec<String>>,
    #[serde(default)]
    episode: Option<f64>,
    #[serde(default)]
    season: Option<i64>,
    #[serde(default)]
    tvdb: Option<String>,
    #[serde(default)]
    size: Option<i64>,
    #[serde(default)]
    group: Option<String>,
    #[serde(default)]
    date: Option<i64>,
    #[serde(default)]
    nzb: Option<String>,
    #[serde(default)]
    poster: Option<String>,
    #[serde(default)]
    subtitles: Option<Vec<String>>,
    #[serde(default)]
    screenshots: Option<Vec<String>>,
    #[serde(default)]
    thumbnails: Option<Vec<String>>,
}

fn build_descriptor() -> PluginDescriptor {
    PluginDescriptor {
        id: "aninzb".to_string(),
        name: "AniNZB Indexer".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        sdk_version: SDK_VERSION.to_string(),
        sdk_constraint: current_sdk_constraint(),
        socket_permissions: vec![],
        provider: ProviderDescriptor::Indexer(IndexerDescriptor {
            provider_type: "aninzb".to_string(),
            provider_aliases: vec![],
            source_kind: IndexerSourceKind::Usenet,
            capabilities: Capabilities {
                supported_ids: HashMap::from([
                    ("series".into(), vec!["tvdb_id".into(), "anidb_id".into()]),
                    ("anime".into(), vec!["tvdb_id".into(), "anidb_id".into()]),
                ]),
                deduplicates_aliases: false,
                season_param: Some("season".into()),
                episode_param: Some("episode".into()),
                query_param: Some("name".into()),
                supported_query_facets: vec![],
                search: true,
                imdb_search: false,
                tvdb_search: true,
                anidb_search: true,
                rss: true,
                protocols: vec![IndexerProtocol::Usenet],
                feed_modes: vec![
                    IndexerFeedMode::Recent,
                    IndexerFeedMode::Rss,
                    IndexerFeedMode::AutomaticSearch,
                    IndexerFeedMode::InteractiveSearch,
                ],
                search_inputs: vec![
                    IndexerSearchInput::TitleQuery,
                    IndexerSearchInput::IdQuery,
                    IndexerSearchInput::Season,
                    IndexerSearchInput::Episode,
                    IndexerSearchInput::AbsoluteEpisode,
                    IndexerSearchInput::Limit,
                ],
                supported_external_ids: vec!["tvdb_id".into(), "anidb_id".into()],
                category_model: Some(IndexerCategoryModel {
                    value_kinds: vec![IndexerCategoryValueKind::Numeric],
                    separate_anime_categories: true,
                    provider_category_metadata: true,
                    ..IndexerCategoryModel::default()
                }),
                limits: Some(IndexerLimitCapabilities {
                    page_size: Some(API_MAX_RESULTS as u32),
                    max_page_size: Some(API_MAX_RESULTS as u32),
                    max_pages: Some(1),
                    rate_limit_hint_seconds: None,
                    ..IndexerLimitCapabilities::default()
                }),
                torrent: None,
                response_features: Some(IndexerResponseFeatures {
                    guid: true,
                    raw_provider_metadata: true,
                    ..IndexerResponseFeatures::default()
                }),
            },
            scoring_policies: vec![],
            config_fields: legacy_config_fields(),
            allowed_hosts: vec![ANINZB_API_HOST.to_string()],
            rate_limit_seconds: None,
        }),
    }
}

fn legacy_config_fields() -> Vec<ConfigFieldDef> {
    vec![ConfigFieldDef {
        key: "base_url".to_string(),
        label: "Base URL".to_string(),
        field_type: ConfigFieldType::String,
        required: true,
        default_value: Some(LEGACY_BASE_URL_DEFAULT.to_string()),
        value_source: ConfigFieldValueSource::User,
        role: Some(ConfigFieldRole::ConnectionUrl),
        host_binding: None,
        options: vec![],
        help_text: Some(
            "Retained for compatibility; AniNZB always uses its fixed public API endpoint."
                .to_string(),
        ),
    }]
}

fn search(request: SearchRequest) -> FnResult<SearchResponse> {
    if request_is_movie_shaped(&request) {
        return Ok(SearchResponse::default());
    }

    let config = AniNzbConfig::from_host();
    let response = match execute_api_search(&config, &request) {
        Ok(response) => response,
        Err(error) if is_hit_budget_exhausted_error(&error) => SearchResponse::default(),
        Err(error) => return Err(error),
    };
    Ok(response)
}

fn execute_api_search(
    config: &AniNzbConfig,
    request: &SearchRequest,
) -> Result<SearchResponse, Error> {
    let urls = build_api_search_urls(config, request)?;
    let mut result_sets = Vec::with_capacity(urls.len());
    let mut first_error = None;
    for url in urls {
        match execute_api_search_url(config, &url) {
            Ok(results) => result_sets.push(results),
            Err(error) => {
                log!(
                    LogLevel::Warn,
                    "AniNZB search variant failed; retaining results from other variants: {}",
                    error
                );
                if first_error.is_none() {
                    first_error = Some(error);
                }
            }
        }
    }
    if result_sets.is_empty()
        && let Some(error) = first_error
    {
        return Err(error);
    }
    let results = merge_api_result_sets(result_sets, result_limit(request.limit));
    Ok(SearchResponse {
        results,
        ..SearchResponse::default()
    })
}

fn execute_api_search_url(
    config: &AniNzbConfig,
    url: &str,
) -> Result<Vec<SearchResult>, Error> {
    wait_for_api_request_slot()?;
    let (status, body) =
        polite_http_get(url, "application/json, */*;q=0.8", &config.http_behavior)?;
    if !(200..300).contains(&status) {
        return Err(Error::msg(format!("AniNZB API returned HTTP {status}")));
    }

    let response = parse_api_response(&body)?;
    Ok(response
        .items
        .unwrap_or_default()
        .iter()
        .filter_map(api_item_to_search_result)
        .collect())
}

fn merge_api_result_sets(result_sets: Vec<Vec<SearchResult>>, limit: usize) -> Vec<SearchResult> {
    let mut iterators = result_sets
        .into_iter()
        .map(Vec::into_iter)
        .collect::<Vec<_>>();
    let mut results = Vec::with_capacity(limit);
    let mut seen = HashSet::new();

    while results.len() < limit {
        let mut advanced = false;
        for iterator in &mut iterators {
            let Some(result) = iterator.next() else {
                continue;
            };
            advanced = true;
            let dedupe_key = result
                .guid
                .clone()
                .or_else(|| result.download_url.clone())
                .unwrap_or_else(|| result.title.clone());
            if seen.insert(dedupe_key) {
                results.push(result);
                if results.len() == limit {
                    break;
                }
            }
        }
        if !advanced {
            break;
        }
    }

    results
}

fn parse_api_response(body: &str) -> Result<AniNzbApiResponse, Error> {
    validate_api_response_size(body.len())?;
    serde_json::from_str(body)
        .map_err(|error| Error::msg(format!("invalid AniNZB API response: {error}")))
}

fn validate_api_response_size(response_bytes: usize) -> Result<(), Error> {
    if response_bytes > MAX_API_RESPONSE_BYTES {
        return Err(Error::msg(format!(
            "AniNZB API response exceeded {MAX_API_RESPONSE_BYTES} bytes"
        )));
    }
    Ok(())
}

fn wait_for_api_request_slot() -> Result<(), Error> {
    let prior_request_millis = var::get::<String>(API_REQUEST_PACING_VAR)
        .map_err(|error| {
            Error::msg(format!(
                "failed to read AniNZB request pacing state: {error}"
            ))
        })?
        .map(|value| {
            value.parse::<u64>().map_err(|error| {
                Error::msg(format!(
                    "failed to parse AniNZB request pacing state: {error}"
                ))
            })
        })
        .transpose()?;

    let delay = api_request_delay(prior_request_millis, current_epoch_millis());
    if !delay.is_zero() {
        std::thread::sleep(delay);
    }

    var::set(API_REQUEST_PACING_VAR, current_epoch_millis().to_string()).map_err(|error| {
        Error::msg(format!(
            "failed to store AniNZB request pacing state: {error}"
        ))
    })
}

fn api_request_delay(prior_request_millis: Option<u64>, now_millis: u64) -> Duration {
    let Some(prior_request_millis) = prior_request_millis else {
        return Duration::ZERO;
    };
    let interval_millis = API_REQUEST_INTERVAL
        .as_millis()
        .try_into()
        .unwrap_or(u64::MAX);
    Duration::from_millis(
        interval_millis.saturating_sub(now_millis.saturating_sub(prior_request_millis)),
    )
}

fn current_epoch_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::ZERO)
        .as_millis()
        .try_into()
        .unwrap_or(u64::MAX)
}

#[derive(Clone, Debug, Default)]
struct AniNzbApiQuery {
    anidb_id: Option<String>,
    tvdb_id: Option<String>,
    name: Option<String>,
    season: Option<u32>,
    episode: Option<u32>,
    filename: Option<String>,
    largest_first: bool,
}

fn build_api_search_urls(
    config: &AniNzbConfig,
    request: &SearchRequest,
) -> Result<Vec<String>, Error> {
    let anidb_id = request_id(request, "anidb_id");
    let tvdb_id = request_id(request, "tvdb_id");
    let name = search_name(request);
    let mut searches = Vec::new();

    // AniNZB caps every query at 50 rows, and season packs may have a null
    // `season`. Search the filename token largest-first so those packs are not
    // buried behind episode releases.
    if let Some(season) = request
        .season
        .filter(|_| request.episode.is_none() && request.absolute_episode.is_none())
    {
        let filename = format!("S{season:02}");
        let mut pack_search = AniNzbApiQuery {
            filename: Some(filename),
            largest_first: true,
            ..AniNzbApiQuery::default()
        };
        if let Some(name) = name.clone() {
            pack_search.name = Some(name);
        } else if let Some(anidb_id) = anidb_id.clone() {
            pack_search.anidb_id = Some(anidb_id);
        } else if let Some(tvdb_id) = tvdb_id.clone() {
            pack_search.tvdb_id = Some(tvdb_id);
        }
        searches.push(pack_search);

        // AniDB identifies the anime season already, so adding the TV season
        // number can over-constrain or misdirect that lookup. TVDB identifies
        // the series and therefore keeps the explicit season filter. Their
        // result sets are complementary, so query and merge both.
        if let Some(anidb_id) = anidb_id {
            searches.push(AniNzbApiQuery {
                anidb_id: Some(anidb_id),
                ..AniNzbApiQuery::default()
            });
        }
        if let Some(tvdb_id) = tvdb_id {
            searches.push(AniNzbApiQuery {
                tvdb_id: Some(tvdb_id),
                season: Some(season),
                ..AniNzbApiQuery::default()
            });
        }
        if searches.len() == 1
            && let Some(name) = name
        {
            searches.push(AniNzbApiQuery {
                name: Some(name),
                season: Some(season),
                ..AniNzbApiQuery::default()
            });
        }
    } else if request.episode.is_some() || request.absolute_episode.is_some() {
        let is_anime = request_is_anime_shaped(request);
        if let Some(anidb_id) = anidb_id {
            searches.push(AniNzbApiQuery {
                anidb_id: Some(anidb_id),
                episode: if is_anime {
                    request.absolute_episode.or(request.episode)
                } else {
                    request.episode.or(request.absolute_episode)
                },
                ..AniNzbApiQuery::default()
            });
        }
        if let (Some(tvdb_id), Some(episode)) = (tvdb_id, request.episode) {
            searches.push(AniNzbApiQuery {
                tvdb_id: Some(tvdb_id),
                season: request.season,
                episode: Some(episode),
                ..AniNzbApiQuery::default()
            });
        }
        if searches.is_empty()
            && let Some(name) = name
        {
            searches.push(AniNzbApiQuery {
                name: Some(name),
                season: request.season,
                episode: request.absolute_episode.or(request.episode),
                ..AniNzbApiQuery::default()
            });
        }
    } else {
        // Unscoped title searches serve series and multi-season pack planning.
        // Merge the largest releases with the default newest-first results so
        // the API's 50-row cap does not hide packs behind recent episodes.
        if let Some(anidb_id) = anidb_id {
            searches.push(AniNzbApiQuery {
                anidb_id: Some(anidb_id.clone()),
                largest_first: true,
                ..AniNzbApiQuery::default()
            });
            searches.push(AniNzbApiQuery {
                anidb_id: Some(anidb_id),
                ..AniNzbApiQuery::default()
            });
        }
        if let Some(tvdb_id) = tvdb_id {
            searches.push(AniNzbApiQuery {
                tvdb_id: Some(tvdb_id.clone()),
                largest_first: true,
                ..AniNzbApiQuery::default()
            });
            searches.push(AniNzbApiQuery {
                tvdb_id: Some(tvdb_id),
                ..AniNzbApiQuery::default()
            });
        }
        if searches.is_empty()
            && let Some(name) = name
        {
            searches.push(AniNzbApiQuery {
                name: Some(name),
                ..AniNzbApiQuery::default()
            });
        }
    }

    if searches.is_empty() {
        searches.push(AniNzbApiQuery::default());
    }

    let mut seen = HashSet::new();
    searches
        .iter()
        .map(|search| build_api_search_url(config, search))
        .filter(|url| match url {
            Ok(url) => seen.insert(url.clone()),
            Err(_) => true,
        })
        .collect()
}

fn build_api_search_url(config: &AniNzbConfig, search: &AniNzbApiQuery) -> Result<String, Error> {
    let mut url = Url::parse(config.api_base_url)
        .map_err(|error| Error::msg(format!("invalid fixed AniNZB API URL: {error}")))?;
    url.set_path("/");
    url.set_query(None);
    url.set_fragment(None);

    {
        let mut query = url.query_pairs_mut();
        if search.anidb_id.is_none()
            && search.tvdb_id.is_none()
            && search.name.is_none()
            && search.season.is_none()
            && search.episode.is_none()
            && search.filename.is_none()
        {
            query.append_pair("source", "release");
        }
        if let Some(anidb_id) = search.anidb_id.as_deref() {
            query.append_pair("anidb", anidb_id);
        }
        if let Some(tvdb_id) = search.tvdb_id.as_deref() {
            query.append_pair("tvdb", tvdb_id);
        }
        if let Some(name) = search.name.as_deref() {
            query.append_pair("name", name);
        }
        if let Some(season) = search.season {
            query.append_pair("season", &season.to_string());
        }
        if let Some(episode) = search.episode {
            query.append_pair("episode", &episode.to_string());
        }
        if let Some(filename) = search.filename.as_deref() {
            query.append_pair("filename", filename);
        }
        if search.largest_first {
            query.append_pair("sort", "size");
            query.append_pair("order", "desc");
        }
    }
    Ok(url.to_string())
}

fn result_limit(request_limit: usize) -> usize {
    if request_limit == 0 {
        API_MAX_RESULTS
    } else {
        request_limit.clamp(1, API_MAX_RESULTS)
    }
}

fn request_is_movie_shaped(request: &SearchRequest) -> bool {
    request
        .context
        .as_ref()
        .is_some_and(|context| context.subject_kind == PluginSearchSubjectKind::Movie)
        || request
            .facet
            .as_deref()
            .is_some_and(|facet| facet.trim().eq_ignore_ascii_case("movie"))
}

fn request_is_anime_shaped(request: &SearchRequest) -> bool {
    request.context.as_ref().is_some_and(|context| {
        context.subject_kind == PluginSearchSubjectKind::AnimeEpisode
    }) || request
        .facet
        .as_deref()
        .is_some_and(|facet| facet.trim().eq_ignore_ascii_case("anime"))
}

fn request_id(request: &SearchRequest, key: &str) -> Option<String> {
    request
        .ids
        .get(key)
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn search_name(request: &SearchRequest) -> Option<String> {
    let query = request.query.trim();
    if !query.is_empty() {
        if request.season.is_some()
            || request.episode.is_some()
            || request.absolute_episode.is_some()
        {
            if let Some(alias) = request
                .tagged_aliases
                .iter()
                .map(|alias| alias.name.trim())
                .filter(|alias| !alias.is_empty())
                .filter(|alias| {
                    query
                        .get(..alias.len())
                        .is_some_and(|prefix| prefix.eq_ignore_ascii_case(alias))
                        && query
                            .get(alias.len()..)
                            .is_some_and(|suffix| {
                                suffix.is_empty()
                                    || suffix.starts_with([' ', '.', '-', '_'])
                            })
                })
                .max_by_key(|alias| alias.len())
            {
                return Some(alias.to_string());
            }
            if let Some(base_name) = strip_search_scope_suffix(query, request) {
                return Some(base_name.to_string());
            }
        }
        return Some(query.to_string());
    }
    request
        .tagged_aliases
        .iter()
        .map(|alias| alias.name.trim())
        .find(|alias| !alias.is_empty())
        .map(ToOwned::to_owned)
}

fn strip_search_scope_suffix<'a>(query: &'a str, request: &SearchRequest) -> Option<&'a str> {
    let mut suffixes = Vec::new();
    if let (Some(season), Some(episode)) = (request.season, request.episode) {
        suffixes.push(format!(" S{season:02}E{episode:02}"));
        suffixes.push(format!(" S{season}E{episode}"));
    }
    if let Some(season) = request.season {
        suffixes.push(format!(" S{season:02}"));
        suffixes.push(format!(" S{season}"));
    }
    if let Some(absolute_episode) = request.absolute_episode {
        suffixes.push(format!(" {absolute_episode:03}"));
        suffixes.push(format!(" {absolute_episode}"));
    }

    suffixes.into_iter().find_map(|suffix| {
        let split_at = query.len().checked_sub(suffix.len())?;
        query
            .get(split_at..)
            .filter(|candidate| candidate.eq_ignore_ascii_case(&suffix))
            .and_then(|_| query.get(..split_at))
            .map(str::trim_end)
            .filter(|base_name| !base_name.is_empty())
    })
}

fn api_item_to_search_result(item: &AniNzbApiItem) -> Option<SearchResult> {
    let title = required_text(item.filename.as_deref())?;
    let download_url = api_download_url(item.nzb.as_deref())?;
    let source = item
        .source
        .as_deref()
        .map(str::trim)
        .filter(|source| !source.is_empty())
        .unwrap_or("unknown")
        .to_ascii_lowercase();

    let mut external_ids = HashMap::new();
    if let Some(anidb_id) = item.anidb {
        external_ids.insert("anidb_id".to_string(), anidb_id.to_string());
    }
    if let Some(tvdb_id) = required_text(item.tvdb.as_deref()) {
        external_ids.insert("tvdb_id".to_string(), tvdb_id.to_string());
    }

    let mut provider_extra = HashMap::new();
    provider_extra.insert(
        "source".to_string(),
        serde_json::Value::from(source.clone()),
    );
    if let Some(id) = item.id {
        provider_extra.insert("api_item_id".to_string(), serde_json::Value::from(id));
    }
    insert_string_list(
        &mut provider_extra,
        "series_names",
        item.series_name.as_deref(),
    );
    insert_optional_text(&mut provider_extra, "group", item.group.as_deref());
    if let Some(season) = item.season {
        provider_extra.insert("season".to_string(), serde_json::Value::from(season));
    }
    if let Some(episode) = item.episode {
        provider_extra.insert("episode".to_string(), serde_json::Value::from(episode));
    }
    insert_optional_text(&mut provider_extra, "poster", item.poster.as_deref());
    insert_string_list(&mut provider_extra, "subtitles", item.subtitles.as_deref());
    insert_string_list(
        &mut provider_extra,
        "screenshots",
        item.screenshots.as_deref(),
    );
    insert_string_list(
        &mut provider_extra,
        "thumbnails",
        item.thumbnails.as_deref(),
    );

    let guid = item
        .id
        .map(|id| format!("aninzb:{source}:{id}"))
        .unwrap_or_else(|| format!("aninzb:{source}:{download_url}"));
    Some(SearchResult {
        title: title.to_string(),
        link: Some(download_url.clone()),
        download_url: Some(download_url),
        size_bytes: item.size,
        published_at: item.date.map(format_unix_timestamp),
        provider_extra,
        guid: Some(guid),
        source_kind: Some(IndexerSourceKind::Usenet),
        protocol: Some(IndexerProtocol::Usenet),
        external_ids,
        categories: vec!["5070".to_string()],
        provider_categories: vec!["TV/Anime".to_string()],
        ..SearchResult::default()
    })
}

fn required_text(value: Option<&str>) -> Option<&str> {
    value.map(str::trim).filter(|value| !value.is_empty())
}

fn api_download_url(value: Option<&str>) -> Option<String> {
    let url = Url::parse(required_text(value)?).ok()?;
    (url.scheme() == "https"
        && url
            .host_str()
            .is_some_and(|host| host.eq_ignore_ascii_case(ANINZB_API_HOST))
        && url.port_or_known_default() == Some(443)
        && url.username().is_empty()
        && url.password().is_none())
    .then(|| url.to_string())
}

fn insert_optional_text(
    provider_extra: &mut HashMap<String, serde_json::Value>,
    key: &str,
    value: Option<&str>,
) {
    if let Some(value) = required_text(value) {
        provider_extra.insert(key.to_string(), serde_json::Value::from(value));
    }
}

fn insert_string_list(
    provider_extra: &mut HashMap<String, serde_json::Value>,
    key: &str,
    values: Option<&[String]>,
) {
    if let Some(values) = values.filter(|values| !values.is_empty()) {
        provider_extra.insert(key.to_string(), serde_json::Value::from(values.to_vec()));
    }
}

fn config_optional(key: &str) -> Option<String> {
    config::get(key)
        .ok()
        .flatten()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn format_unix_timestamp(seconds: i64) -> String {
    let days = seconds.div_euclid(86_400);
    let seconds_of_day = seconds.rem_euclid(86_400);
    let (year, month, day) = civil_from_days(days);
    let hour = seconds_of_day / 3_600;
    let minute = (seconds_of_day % 3_600) / 60;
    let second = seconds_of_day % 60;
    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z")
}

fn civil_from_days(days: i64) -> (i64, i64, i64) {
    let days = days + 719_468;
    let era = if days >= 0 { days } else { days - 146_096 } / 146_097;
    let day_of_era = days - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
    let month = month_prime + if month_prime < 10 { 3 } else { -9 };
    let year = year + if month <= 2 { 1 } else { 0 };
    (year, month, day)
}

indexer_command_compat::scryer_indexer_main!(descriptor = build_descriptor, search = search,);

#[cfg(test)]
mod tests {
    use super::*;

    fn legacy_config() -> LegacyAniNzbConfig {
        LegacyAniNzbConfig {
            base_url: Some("https://aninzb.moe".to_string()),
            api_key_present: true,
            api_path: Some("/api".to_string()),
            additional_params: Some("&legacy=1".to_string()),
            hourly_hit_cap: Some("1".to_string()),
            daily_hit_cap: Some("2".to_string()),
        }
    }

    fn api_queries(request: &SearchRequest) -> Vec<HashMap<String, String>> {
        let config = migrate_legacy_config(LegacyAniNzbConfig::default());
        build_api_search_urls(&config, request)
            .unwrap()
            .into_iter()
            .map(|url| {
                Url::parse(&url)
                    .unwrap()
                    .query_pairs()
                    .map(|(key, value)| (key.into_owned(), value.into_owned()))
                    .collect()
            })
            .collect()
    }

    #[test]
    fn descriptor_keeps_only_the_legacy_base_url_configuration() {
        let descriptor = build_descriptor();
        let ProviderDescriptor::Indexer(indexer) = descriptor.provider else {
            panic!("expected indexer descriptor");
        };

        assert_eq!(indexer.config_fields.len(), 1);
        let base_url = &indexer.config_fields[0];
        assert_eq!(base_url.key, "base_url");
        assert_eq!(base_url.role, Some(ConfigFieldRole::ConnectionUrl));
        assert_eq!(
            base_url.default_value.as_deref(),
            Some(LEGACY_BASE_URL_DEFAULT)
        );
        assert_eq!(indexer.allowed_hosts, vec![ANINZB_API_HOST.to_string()]);
        assert_eq!(indexer.rate_limit_seconds, None);
        assert_eq!(indexer.capabilities.query_param.as_deref(), Some("name"));
        assert_eq!(indexer.capabilities.season_param.as_deref(), Some("season"));
        assert_eq!(
            indexer.capabilities.episode_param.as_deref(),
            Some("episode")
        );
        assert!(
            indexer
                .capabilities
                .search_inputs
                .contains(&IndexerSearchInput::AbsoluteEpisode)
        );
        assert!(indexer.capabilities.rss);
        assert_eq!(
            indexer.capabilities.feed_modes,
            vec![
                IndexerFeedMode::Recent,
                IndexerFeedMode::Rss,
                IndexerFeedMode::AutomaticSearch,
                IndexerFeedMode::InteractiveSearch,
            ]
        );
        let limits = indexer.capabilities.limits.expect("limits");
        assert_eq!(limits.page_size, Some(API_MAX_RESULTS as u32));
        assert_eq!(limits.max_page_size, Some(API_MAX_RESULTS as u32));
        assert_eq!(limits.max_pages, Some(1));
        assert_eq!(limits.rate_limit_hint_seconds, None);
        assert!(!limits.api_quota_supported);
        let features = indexer
            .capabilities
            .response_features
            .expect("response features");
        assert!(features.guid);
        assert!(features.raw_provider_metadata);
        assert!(!features.info_url);
        assert!(!features.grabs);
        assert!(!features.comments);
    }

    #[test]
    fn legacy_config_is_ignored_in_favor_of_fixed_api_behavior() {
        let config = migrate_legacy_config(legacy_config());

        assert_eq!(config.api_base_url, ANINZB_API_BASE_URL);
        assert_eq!(config.http_behavior.user_agent, USER_AGENT);
        assert!(
            USER_AGENT
                .chars()
                .all(|character| !matches!(character, '\r' | '\n' | '\\'))
        );
        assert_eq!(config.http_behavior.pre_request_delay, Duration::ZERO);
        assert_eq!(API_REQUESTS_PER_SECOND, 2);
        assert_eq!(API_REQUEST_INTERVAL, Duration::from_millis(500));
        assert_eq!(config.http_behavior.max_search_pages, 1);
        let budget = config.http_behavior.hit_budget.expect("hit budget");
        assert_eq!(budget.hourly_limit, DEFAULT_HOURLY_HIT_CAP);
        assert_eq!(budget.daily_limit, DEFAULT_DAILY_HIT_CAP);
    }

    #[test]
    fn api_request_pacing_has_no_initial_delay_and_caps_at_two_per_second() {
        assert_eq!(api_request_delay(None, 10_000), Duration::ZERO);
        assert_eq!(
            api_request_delay(Some(10_000), 10_000),
            Duration::from_millis(500)
        );
        assert_eq!(
            api_request_delay(Some(10_000), 10_250),
            Duration::from_millis(250)
        );
        assert_eq!(api_request_delay(Some(10_000), 10_500), Duration::ZERO);
    }

    #[test]
    fn anime_absolute_episode_search_merges_anidb_and_tvdb_shapes() {
        let request = SearchRequest {
            query: "Bleach 055".to_string(),
            ids: HashMap::from([
                ("anidb_id".to_string(), "2369".to_string()),
                ("tvdb_id".to_string(), "74796".to_string()),
            ]),
            facet: Some("anime".to_string()),
            season: Some(3),
            episode: Some(4),
            absolute_episode: Some(55),
            ..SearchRequest::default()
        };
        let queries = api_queries(&request);

        assert_eq!(queries.len(), 2);
        assert_eq!(queries[0].get("anidb").map(String::as_str), Some("2369"));
        assert_eq!(queries[0].get("episode").map(String::as_str), Some("55"));
        assert_eq!(queries[1].get("tvdb").map(String::as_str), Some("74796"));
        assert_eq!(queries[1].get("season").map(String::as_str), Some("3"));
        assert_eq!(queries[1].get("episode").map(String::as_str), Some("4"));
    }

    #[test]
    fn anime_sxex_search_queries_anidb_and_tvdb_independently() {
        let request = SearchRequest {
            query: "Bleach S02E04".to_string(),
            ids: HashMap::from([
                ("anidb_id".to_string(), "2369".to_string()),
                ("tvdb_id".to_string(), "74796".to_string()),
            ]),
            facet: Some("anime".to_string()),
            season: Some(2),
            episode: Some(4),
            ..SearchRequest::default()
        };
        let queries = api_queries(&request);

        assert_eq!(queries.len(), 2);
        assert_eq!(queries[0].get("anidb").map(String::as_str), Some("2369"));
        assert_eq!(queries[0].get("episode").map(String::as_str), Some("4"));
        assert_eq!(queries[1].get("tvdb").map(String::as_str), Some("74796"));
        assert_eq!(queries[1].get("season").map(String::as_str), Some("2"));
        assert_eq!(queries[1].get("episode").map(String::as_str), Some("4"));
    }

    #[test]
    fn anime_season_pack_search_merges_filename_anidb_and_tvdb_queries() {
        let request = SearchRequest {
            query: "Bleach S06".to_string(),
            ids: HashMap::from([
                ("anidb_id".to_string(), "2369".to_string()),
                ("tvdb_id".to_string(), "74796".to_string()),
            ]),
            facet: Some("anime".to_string()),
            season: Some(6),
            ..SearchRequest::default()
        };
        let queries = api_queries(&request);

        assert_eq!(queries.len(), 3);
        assert_eq!(queries[0].get("name").map(String::as_str), Some("Bleach"));
        assert_eq!(queries[0].get("filename").map(String::as_str), Some("S06"));
        assert_eq!(queries[0].get("sort").map(String::as_str), Some("size"));
        assert_eq!(queries[0].get("order").map(String::as_str), Some("desc"));
        assert_eq!(queries[1].get("anidb").map(String::as_str), Some("2369"));
        assert!(!queries[1].contains_key("season"));
        assert_eq!(queries[2].get("tvdb").map(String::as_str), Some("74796"));
        assert_eq!(queries[2].get("season").map(String::as_str), Some("6"));
    }

    #[test]
    fn scoped_text_search_uses_base_title_for_pack_and_structured_queries() {
        let request = SearchRequest {
            query: "Bleach S02".to_string(),
            facet: Some("anime".to_string()),
            season: Some(2),
            ..SearchRequest::default()
        };
        let queries = api_queries(&request);

        assert_eq!(queries.len(), 2);
        assert_eq!(queries[0].get("name").map(String::as_str), Some("Bleach"));
        assert_eq!(queries[0].get("filename").map(String::as_str), Some("S02"));
        assert_eq!(queries[1].get("name").map(String::as_str), Some("Bleach"));
        assert_eq!(queries[1].get("season").map(String::as_str), Some("2"));
    }

    #[test]
    fn title_search_merges_largest_and_newest_results_for_each_identity() {
        let request = SearchRequest {
            ids: HashMap::from([
                ("anidb_id".to_string(), "2369".to_string()),
                ("tvdb_id".to_string(), "74796".to_string()),
            ]),
            facet: Some("anime".to_string()),
            ..SearchRequest::default()
        };
        let queries = api_queries(&request);

        assert_eq!(queries.len(), 4);
        assert_eq!(queries[0].get("anidb").map(String::as_str), Some("2369"));
        assert_eq!(queries[0].get("sort").map(String::as_str), Some("size"));
        assert_eq!(queries[1].get("anidb").map(String::as_str), Some("2369"));
        assert!(!queries[1].contains_key("sort"));
        assert_eq!(queries[2].get("tvdb").map(String::as_str), Some("74796"));
        assert_eq!(queries[2].get("sort").map(String::as_str), Some("size"));
        assert_eq!(queries[3].get("tvdb").map(String::as_str), Some("74796"));
        assert!(!queries[3].contains_key("sort"));
    }

    #[test]
    fn merged_api_results_are_round_robin_and_deduplicated() {
        let result = |guid: &str| SearchResult {
            title: guid.to_string(),
            guid: Some(guid.to_string()),
            ..SearchResult::default()
        };

        let merged = merge_api_result_sets(
            vec![
                vec![result("shared"), result("anidb")],
                vec![result("shared"), result("tvdb")],
            ],
            3,
        );

        assert_eq!(
            merged
                .iter()
                .filter_map(|result| result.guid.as_deref())
                .collect::<Vec<_>>(),
            vec!["shared", "anidb", "tvdb"]
        );
    }

    #[test]
    fn api_url_for_recent_search_uses_newest_release_source() {
        let queries = api_queries(&SearchRequest::default());

        assert_eq!(
            queries[0].get("source").map(String::as_str),
            Some("release")
        );
    }

    #[test]
    fn result_limit_defaults_and_caps_to_api_limit() {
        assert_eq!(result_limit(0), API_MAX_RESULTS);
        assert_eq!(result_limit(1), 1);
        assert_eq!(result_limit(500), API_MAX_RESULTS);
    }

    #[test]
    fn movie_requests_are_unsupported() {
        let request = SearchRequest {
            facet: Some("movie".to_string()),
            ..SearchRequest::default()
        };
        assert!(request_is_movie_shaped(&request));
    }

    #[test]
    fn api_item_maps_release_metadata_and_artifacts() {
        let body = r#"{
          "total_count": 1,
          "items": [{
            "source": "release", "id": 10936,
            "filename": "Mushoku.Tensei.S03E01.1080p-VARYG",
            "anidb": 14758, "series_name": ["Mushoku Tensei", "無職転生"],
            "episode": 1.0, "season": 3, "tvdb": "371310",
            "size": 1640102917, "group": "VARYG", "date": 0,
            "nzb": "https://api.aninzb.moe/releases/10936/release.nzb",
            "poster": "https://api.aninzb.moe/posters/14758",
            "subtitles": ["https://api.aninzb.moe/subtitles/1.ass"],
            "screenshots": ["https://api.aninzb.moe/screenshots/1.png"],
            "thumbnails": ["https://api.aninzb.moe/thumbnails/1.jpg"]
          }]
        }"#;
        let response: AniNzbApiResponse = serde_json::from_str(body).unwrap();
        let result = api_item_to_search_result(&response.items.as_ref().expect("items")[0])
            .expect("usable result");

        assert_eq!(result.title, "Mushoku.Tensei.S03E01.1080p-VARYG");
        assert_eq!(
            result.download_url.as_deref(),
            Some("https://api.aninzb.moe/releases/10936/release.nzb")
        );
        assert_eq!(result.guid.as_deref(), Some("aninzb:release:10936"));
        assert_eq!(result.size_bytes, Some(1_640_102_917));
        assert_eq!(result.published_at.as_deref(), Some("1970-01-01T00:00:00Z"));
        assert_eq!(result.source_kind, Some(IndexerSourceKind::Usenet));
        assert_eq!(result.protocol, Some(IndexerProtocol::Usenet));
        assert_eq!(
            result.external_ids.get("anidb_id").map(String::as_str),
            Some("14758")
        );
        assert_eq!(
            result.provider_extra.get("subtitles"),
            Some(&serde_json::json!([
                "https://api.aninzb.moe/subtitles/1.ass"
            ]))
        );
    }

    #[test]
    fn api_item_accepts_null_optional_fields_and_all_sources() {
        for source in ["release", "tosho", "usenet"] {
            let item: AniNzbApiItem = serde_json::from_value(serde_json::json!({
                "source": source,
                "filename": "Example.mkv",
                "nzb": "https://api.aninzb.moe/example.nzb",
                "anidb": null, "series_name": null, "episode": null,
                "season": null, "tvdb": null, "size": null, "group": null,
                "date": null, "poster": null, "subtitles": null,
                "screenshots": null, "thumbnails": null
            }))
            .unwrap();
            let result = api_item_to_search_result(&item).expect("usable result");
            let expected_guid = format!("aninzb:{source}:https://api.aninzb.moe/example.nzb");
            assert_eq!(result.guid.as_deref(), Some(expected_guid.as_str()));
            assert_eq!(result.published_at, None);
            assert_eq!(result.source_kind, Some(IndexerSourceKind::Usenet));
        }
    }

    #[test]
    fn api_item_skips_missing_acquisition_fields() {
        let missing_filename: AniNzbApiItem = serde_json::from_value(serde_json::json!({
            "nzb": "https://api.aninzb.moe/example.nzb"
        }))
        .unwrap();
        let missing_nzb: AniNzbApiItem = serde_json::from_value(serde_json::json!({
            "filename": "Example.mkv"
        }))
        .unwrap();

        assert!(api_item_to_search_result(&missing_filename).is_none());
        assert!(api_item_to_search_result(&missing_nzb).is_none());
    }

    #[test]
    fn api_item_rejects_non_aninzb_download_urls() {
        let item: AniNzbApiItem = serde_json::from_value(serde_json::json!({
            "filename": "Example.mkv",
            "nzb": "https://localhost/private.nzb"
        }))
        .unwrap();

        assert!(api_item_to_search_result(&item).is_none());
    }

    #[test]
    fn api_response_accepts_null_items_as_an_empty_list() {
        let response = parse_api_response(r#"{"total_count": 0, "items": null}"#).unwrap();

        assert!(response.items.unwrap_or_default().is_empty());
    }

    #[test]
    fn api_response_size_is_limited_to_20_mib() {
        assert!(validate_api_response_size(MAX_API_RESPONSE_BYTES).is_ok());
        let error = validate_api_response_size(MAX_API_RESPONSE_BYTES + 1).unwrap_err();
        assert!(error.to_string().contains("exceeded 20971520 bytes"));
    }
}
