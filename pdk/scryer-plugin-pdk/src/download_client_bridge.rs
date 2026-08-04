//! Temporary source bridge for the DLC-first command migration.
//!
//! The first-party clients keep their operation implementations while this
//! bridge turns the former JSON export functions into the typed command
//! protocol. It is intentionally PDK-owned so every migrated client has the
//! same exact completed-download lookup and no one reintroduces a raw path API.

use crate::sdk;
use crate::{
    FnResult, PluginDownloadClientCommand, PluginDownloadClientCommandResult,
    PluginDownloadGetCompletedRequest, run_download_client_plugin_with_descriptor,
};

pub struct LegacyDownloadClientFunctions {
    pub describe: fn(String) -> FnResult<String>,
    pub add: fn(String) -> FnResult<String>,
    pub list_queue: fn(String) -> FnResult<String>,
    pub list_history: fn(String) -> FnResult<String>,
    pub list_completed: fn(String) -> FnResult<String>,
    pub list_recent_completed: Option<fn(String) -> FnResult<String>>,
    pub control: fn(String) -> FnResult<String>,
    pub mark_imported: fn(String) -> FnResult<String>,
    pub status: fn(String) -> FnResult<String>,
    pub test_connection: fn(String) -> FnResult<String>,
}

pub fn legacy_download_client_descriptor(
    functions: &LegacyDownloadClientFunctions,
) -> sdk::PluginDescriptor {
    let raw = (functions.describe)(String::new())
        .expect("first-party command DLC descriptor must serialize successfully");
    serde_json::from_str(&raw).expect("first-party command DLC descriptor must be valid")
}

pub fn run_download_client_bridge_with_descriptor(functions: LegacyDownloadClientFunctions) -> ! {
    let descriptor = legacy_download_client_descriptor(&functions);
    run_download_client_plugin_with_descriptor(
        move || descriptor,
        move |command| bridge_download_client_command(&functions, command),
    )
}

fn bridge_download_client_command(
    functions: &LegacyDownloadClientFunctions,
    command: PluginDownloadClientCommand,
) -> PluginDownloadClientCommandResult {
    match command {
        PluginDownloadClientCommand::Add(request) => {
            PluginDownloadClientCommandResult::Add(call(functions.add, request))
        }
        PluginDownloadClientCommand::ListQueue => {
            PluginDownloadClientCommandResult::ListQueue(call(functions.list_queue, ()))
        }
        PluginDownloadClientCommand::ListHistory => {
            PluginDownloadClientCommandResult::ListHistory(call(functions.list_history, ()))
        }
        PluginDownloadClientCommand::ListCompleted => {
            PluginDownloadClientCommandResult::ListCompleted(call(functions.list_completed, ()))
        }
        PluginDownloadClientCommand::ListRecentCompleted(request) => {
            let result = if let Some(list_recent_completed) = functions.list_recent_completed {
                call(list_recent_completed, request)
            } else {
                // Existing first-party DLCs do not export a separate recent
                // endpoint. Their complete list is still downloader-owned,
                // so preserve the legacy adapter's conservative fallback.
                call(functions.list_completed, ())
            };
            PluginDownloadClientCommandResult::ListRecentCompleted(result)
        }
        PluginDownloadClientCommand::GetCompleted(PluginDownloadGetCompletedRequest {
            client_item_id,
        }) => {
            let result =
                match call::<_, Vec<sdk::PluginCompletedDownload>>(functions.list_completed, ()) {
                    sdk::PluginResult::Ok(downloads) => sdk::PluginResult::Ok(
                        downloads
                            .into_iter()
                            .find(|download| download.client_item_id == client_item_id),
                    ),
                    sdk::PluginResult::Err(error) => sdk::PluginResult::Err(error),
                };
            PluginDownloadClientCommandResult::GetCompleted(result)
        }
        PluginDownloadClientCommand::Control(request) => {
            PluginDownloadClientCommandResult::Control(call(functions.control, request))
        }
        PluginDownloadClientCommand::MarkImported(request) => {
            PluginDownloadClientCommandResult::MarkImported(call(functions.mark_imported, request))
        }
        PluginDownloadClientCommand::Status => {
            PluginDownloadClientCommandResult::Status(call(functions.status, ()))
        }
        PluginDownloadClientCommand::TestConnection => {
            PluginDownloadClientCommandResult::TestConnection(call(functions.test_connection, ()))
        }
    }
}

fn call<Request, Response>(
    function: fn(String) -> FnResult<String>,
    request: Request,
) -> sdk::PluginResult<Response>
where
    Request: serde::Serialize,
    Response: serde::de::DeserializeOwned,
{
    let request = match serde_json::to_string(&request) {
        Ok(request) => request,
        Err(error) => return bridge_error(format!("failed to encode command request: {error}")),
    };
    let raw = match function(request) {
        Ok(raw) => raw,
        Err(error) => return bridge_error(error.to_string()),
    };
    match serde_json::from_str(&raw) {
        Ok(result) => result,
        Err(error) => bridge_error(format!("plugin returned malformed response: {error}")),
    }
}

fn bridge_error<T>(message: String) -> sdk::PluginResult<T> {
    sdk::PluginResult::Err(sdk::PluginError {
        code: sdk::PluginErrorCode::Temporary,
        public_message: "download client command failed".to_string(),
        debug_message: Some(message),
        retry_after_seconds: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_lookup_returns_only_the_requested_completed_download() {
        let complete = sdk::PluginCompletedDownload {
            client_item_id: "retained-item".to_string(),
            download_id: None,
            info_hash: None,
            name: "retained item".to_string(),
            dest_dir: "/downloads".to_string(),
            category: None,
            output_kind: None,
            content_paths: Vec::new(),
            size_bytes: None,
            completed_at: None,
            parameters: Vec::new(),
        };
        let output = serde_json::to_string(&sdk::PluginResult::Ok(vec![complete])).unwrap();
        let result: sdk::PluginResult<Vec<sdk::PluginCompletedDownload>> =
            serde_json::from_str(&output).unwrap();
        assert!(matches!(result, sdk::PluginResult::Ok(downloads) if downloads.len() == 1));
    }
}
