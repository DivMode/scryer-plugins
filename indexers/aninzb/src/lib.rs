use std::collections::HashMap;
use std::time::Duration;

use extism_pdk::*;
use newznab_common::{
    Capabilities, IndexerCategoryModel, IndexerCategoryValueKind, IndexerDescriptor,
    IndexerFeedMode, IndexerLimitCapabilities, IndexerProtocol, IndexerResponseFeatures,
    IndexerSearchInput, IndexerSourceKind, NewznabHitBudget, NewznabHttpBehavior, PluginDescriptor,
    PluginResult, PluginSearchSubjectKind, ProviderDescriptor, SDK_VERSION, SearchRequest,
    SearchResponse, SearchResult, current_sdk_constraint, is_hit_budget_exhausted_error,
    polite_http_get,
};
use serde::Deserialize;
use url::Url;

const ANINZB_API_BASE_URL: &str = "https://api.aninzb.moe/";
const ANINZB_API_HOST: &str = "api.aninzb.moe";
const API_MAX_RESULTS: usize = 50;
const MAX_API_RESPONSE_BYTES: usize = 20 * 1024 * 1024;
const REQUEST_DELAY_SECONDS: u64 = 3;
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
    fn from_extism() -> Self {
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
    fn from_extism() -> Self {
        let legacy = LegacyAniNzbConfig::from_extism();
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
            pre_request_delay: Duration::from_secs(REQUEST_DELAY_SECONDS),
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

#[plugin_fn]
pub fn scryer_describe(_input: String) -> FnResult<String> {
    Ok(serde_json::to_string(&build_descriptor())?)
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
                    rate_limit_hint_seconds: Some(REQUEST_DELAY_SECONDS as u32),
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
            config_fields: vec![],
            allowed_hosts: vec![ANINZB_API_HOST.to_string()],
            rate_limit_seconds: Some(REQUEST_DELAY_SECONDS as i64),
        }),
    }
}

#[plugin_fn]
pub fn scryer_indexer_search(input: String) -> FnResult<String> {
    let request: SearchRequest = serde_json::from_str(&input)?;
    if request_is_movie_shaped(&request) {
        return Ok(serde_json::to_string(&PluginResult::Ok(
            SearchResponse::default(),
        ))?);
    }

    let config = AniNzbConfig::from_extism();
    let response = match execute_api_search(&config, &request) {
        Ok(response) => response,
        Err(error) if is_hit_budget_exhausted_error(&error) => SearchResponse::default(),
        Err(error) => return Err(error.into()),
    };
    Ok(serde_json::to_string(&PluginResult::Ok(response))?)
}

fn execute_api_search(
    config: &AniNzbConfig,
    request: &SearchRequest,
) -> Result<SearchResponse, Error> {
    let url = build_api_search_url(config, request)?;
    let (status, body) =
        polite_http_get(&url, "application/json, */*;q=0.8", &config.http_behavior)?;
    if !(200..300).contains(&status) {
        return Err(Error::msg(format!("AniNZB API returned HTTP {status}")));
    }

    let response = parse_api_response(&body)?;
    let results = response
        .items
        .unwrap_or_default()
        .iter()
        .filter_map(api_item_to_search_result)
        .take(result_limit(request.limit))
        .collect();
    Ok(SearchResponse {
        results,
        ..SearchResponse::default()
    })
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

fn build_api_search_url(config: &AniNzbConfig, request: &SearchRequest) -> Result<String, Error> {
    let mut url = Url::parse(config.api_base_url)
        .map_err(|error| Error::msg(format!("invalid fixed AniNZB API URL: {error}")))?;
    url.set_path("/");
    url.set_query(None);
    url.set_fragment(None);

    let anidb_id = request_id(request, "anidb_id");
    let tvdb_id = request_id(request, "tvdb_id");
    let name = search_name(request);
    let episode = request.episode.or(request.absolute_episode);
    if anidb_id.is_none()
        && tvdb_id.is_none()
        && name.is_none()
        && request.season.is_none()
        && episode.is_none()
    {
        return Ok(url.to_string());
    }
    {
        let mut query = url.query_pairs_mut();
        if let Some(anidb_id) = anidb_id.as_deref() {
            query.append_pair("anidb", anidb_id);
        }
        if let Some(tvdb_id) = tvdb_id.as_deref() {
            query.append_pair("tvdb", tvdb_id);
        }
        if let Some(name) = name.as_deref() {
            query.append_pair("name", name);
        }
        if let Some(season) = request.season {
            query.append_pair("season", &season.to_string());
        }
        if let Some(episode) = episode {
            query.append_pair("episode", &episode.to_string());
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
        return Some(query.to_string());
    }
    request
        .tagged_aliases
        .iter()
        .map(|alias| alias.name.trim())
        .find(|alias| !alias.is_empty())
        .map(ToOwned::to_owned)
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

    #[test]
    fn descriptor_is_fixed_api_and_has_no_configuration() {
        let descriptor = build_descriptor();
        let ProviderDescriptor::Indexer(indexer) = descriptor.provider else {
            panic!("expected indexer descriptor");
        };

        assert!(indexer.config_fields.is_empty());
        assert_eq!(indexer.allowed_hosts, vec![ANINZB_API_HOST.to_string()]);
        assert_eq!(indexer.capabilities.query_param.as_deref(), Some("name"));
        assert_eq!(indexer.capabilities.season_param.as_deref(), Some("season"));
        assert_eq!(
            indexer.capabilities.episode_param.as_deref(),
            Some("episode")
        );
        let limits = indexer.capabilities.limits.expect("limits");
        assert_eq!(limits.page_size, Some(API_MAX_RESULTS as u32));
        assert_eq!(limits.max_page_size, Some(API_MAX_RESULTS as u32));
        assert_eq!(limits.max_pages, Some(1));
        assert_eq!(
            limits.rate_limit_hint_seconds,
            Some(REQUEST_DELAY_SECONDS as u32)
        );
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
        assert_eq!(
            config.http_behavior.pre_request_delay,
            Duration::from_secs(REQUEST_DELAY_SECONDS)
        );
        assert_eq!(config.http_behavior.max_search_pages, 1);
        let budget = config.http_behavior.hit_budget.expect("hit budget");
        assert_eq!(budget.hourly_limit, DEFAULT_HOURLY_HIT_CAP);
        assert_eq!(budget.daily_limit, DEFAULT_DAILY_HIT_CAP);
    }

    #[test]
    fn api_url_maps_supported_request_filters() {
        let request = SearchRequest {
            query: "Mushoku Tensei & Friends".to_string(),
            ids: HashMap::from([
                ("anidb_id".to_string(), "14758".to_string()),
                ("tvdb_id".to_string(), "371310".to_string()),
            ]),
            season: Some(2),
            episode: Some(4),
            ..SearchRequest::default()
        };
        let config = migrate_legacy_config(LegacyAniNzbConfig::default());
        let url = Url::parse(&build_api_search_url(&config, &request).unwrap()).unwrap();
        let query = url.query_pairs().collect::<HashMap<_, _>>();

        assert_eq!(
            query.get("anidb").map(|value| value.as_ref()),
            Some("14758")
        );
        assert_eq!(
            query.get("tvdb").map(|value| value.as_ref()),
            Some("371310")
        );
        assert_eq!(
            query.get("name").map(|value| value.as_ref()),
            Some("Mushoku Tensei & Friends")
        );
        assert_eq!(query.get("season").map(|value| value.as_ref()), Some("2"));
        assert_eq!(query.get("episode").map(|value| value.as_ref()), Some("4"));
    }

    #[test]
    fn api_url_for_recent_search_has_no_query() {
        let config = migrate_legacy_config(LegacyAniNzbConfig::default());
        assert_eq!(
            build_api_search_url(&config, &SearchRequest::default()).unwrap(),
            ANINZB_API_BASE_URL
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
