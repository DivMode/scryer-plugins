//! Native command-plugin bindings for Scryer's host-owned services.
//!
//! The guest never receives unrestricted WASI network or process authority.
//! Each request is postcard-encoded, evaluated by Scryer's descriptor-scoped
//! policy, and returned through a bounded host response handle.

use std::fmt;

use scryer_plugin_sdk::host::{
    PluginConfigGetRequest, PluginHostRequest, PluginHostResponse, PluginHttpRequest,
    PluginHttpResponse, PluginProcessExecRequest, PluginProcessExecResponse,
    PluginStateDeleteRequest, PluginStateGetRequest, PluginStateSetRequest,
};
use scryer_plugin_sdk::{
    PluginError, PluginResult, SocketCloseRequest, SocketCloseResponse, SocketOpenRequest,
    SocketOpenResponse, SocketReadRequest, SocketReadResponse, SocketStartTlsRequest,
    SocketStartTlsResponse, SocketWriteRequest, SocketWriteResponse,
};

/// Maximum one-response payload accepted by the guest binding.
///
/// This is independent of host-side service limits and prevents a compromised
/// or malformed host ABI implementation from making a guest allocate without
/// bound.
const MAX_RESPONSE_BYTES: usize = 16 * 1024 * 1024;

/// A transport or protocol failure while using Scryer's native host ABI.
#[derive(Debug)]
pub enum HostCallError {
    Unavailable,
    Encode(postcard::Error),
    Decode(postcard::Error),
    InvalidHandle,
    ResponseTooLarge(usize),
    ReadFailed,
    UnexpectedResponse(&'static str),
    Service(PluginError),
}

impl fmt::Display for HostCallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unavailable => write!(f, "Scryer host services are unavailable"),
            Self::Encode(error) => write!(f, "failed to encode host request: {error}"),
            Self::Decode(error) => write!(f, "failed to decode host response: {error}"),
            Self::InvalidHandle => write!(f, "host returned an invalid response handle"),
            Self::ResponseTooLarge(size) => {
                write!(
                    f,
                    "host response exceeds the {MAX_RESPONSE_BYTES}-byte limit: {size}"
                )
            }
            Self::ReadFailed => write!(f, "failed to read host response"),
            Self::UnexpectedResponse(operation) => {
                write!(
                    f,
                    "host returned a response for another operation while handling {operation}"
                )
            }
            Self::Service(error) => write!(f, "host service error: {}", error.public_message),
        }
    }
}

impl std::error::Error for HostCallError {}

/// Invoke a typed host service. Most plugins should prefer the specific
/// convenience functions below, which also verify the response operation.
pub fn call(request: PluginHostRequest) -> Result<PluginHostResponse, HostCallError> {
    let encoded = postcard::to_allocvec(&request).map_err(HostCallError::Encode)?;
    let handle = raw::call(&encoded)?;
    let response = ResponseHandle(handle);
    let len = raw::response_len(response.0)?;
    if len > MAX_RESPONSE_BYTES {
        return Err(HostCallError::ResponseTooLarge(len));
    }
    let mut encoded = vec![0; len];
    raw::response_read(response.0, &mut encoded)?;
    postcard::from_bytes(&encoded).map_err(HostCallError::Decode)
}

pub fn config_get(key: impl Into<String>) -> Result<Option<String>, HostCallError> {
    match call(PluginHostRequest::ConfigGet(PluginConfigGetRequest {
        key: key.into(),
    }))? {
        PluginHostResponse::ConfigGet(result) => {
            result_value(result).map(|response| response.value)
        }
        _ => Err(HostCallError::UnexpectedResponse("config_get")),
    }
}

pub fn state_get(key: impl Into<String>) -> Result<Option<Vec<u8>>, HostCallError> {
    match call(PluginHostRequest::StateGet(PluginStateGetRequest {
        key: key.into(),
    }))? {
        PluginHostResponse::StateGet(result) => result_value(result).map(|response| response.value),
        _ => Err(HostCallError::UnexpectedResponse("state_get")),
    }
}

pub fn state_set(key: impl Into<String>, value: Vec<u8>) -> Result<bool, HostCallError> {
    match call(PluginHostRequest::StateSet(PluginStateSetRequest {
        key: key.into(),
        value,
    }))? {
        PluginHostResponse::StateSet(result) => {
            result_value(result).map(|response| response.changed)
        }
        _ => Err(HostCallError::UnexpectedResponse("state_set")),
    }
}

pub fn state_delete(key: impl Into<String>) -> Result<bool, HostCallError> {
    match call(PluginHostRequest::StateDelete(PluginStateDeleteRequest {
        key: key.into(),
    }))? {
        PluginHostResponse::StateDelete(result) => {
            result_value(result).map(|response| response.changed)
        }
        _ => Err(HostCallError::UnexpectedResponse("state_delete")),
    }
}

pub fn http(request: PluginHttpRequest) -> Result<PluginHttpResponse, HostCallError> {
    match call(PluginHostRequest::Http(request))? {
        PluginHostResponse::Http(result) => result_value(result),
        _ => Err(HostCallError::UnexpectedResponse("http")),
    }
}

pub fn socket_open(request: SocketOpenRequest) -> Result<SocketOpenResponse, HostCallError> {
    match call(PluginHostRequest::SocketOpen(request))? {
        PluginHostResponse::SocketOpen(result) => result_value(result),
        _ => Err(HostCallError::UnexpectedResponse("socket_open")),
    }
}

pub fn socket_read(request: SocketReadRequest) -> Result<SocketReadResponse, HostCallError> {
    match call(PluginHostRequest::SocketRead(request))? {
        PluginHostResponse::SocketRead(result) => result_value(result),
        _ => Err(HostCallError::UnexpectedResponse("socket_read")),
    }
}

pub fn socket_write(request: SocketWriteRequest) -> Result<SocketWriteResponse, HostCallError> {
    match call(PluginHostRequest::SocketWrite(request))? {
        PluginHostResponse::SocketWrite(result) => result_value(result),
        _ => Err(HostCallError::UnexpectedResponse("socket_write")),
    }
}

pub fn socket_starttls(
    request: SocketStartTlsRequest,
) -> Result<SocketStartTlsResponse, HostCallError> {
    match call(PluginHostRequest::SocketStartTls(request))? {
        PluginHostResponse::SocketStartTls(result) => result_value(result),
        _ => Err(HostCallError::UnexpectedResponse("socket_starttls")),
    }
}

pub fn socket_close(request: SocketCloseRequest) -> Result<SocketCloseResponse, HostCallError> {
    match call(PluginHostRequest::SocketClose(request))? {
        PluginHostResponse::SocketClose(result) => result_value(result),
        _ => Err(HostCallError::UnexpectedResponse("socket_close")),
    }
}

pub fn process_exec(
    request: PluginProcessExecRequest,
) -> Result<PluginProcessExecResponse, HostCallError> {
    match call(PluginHostRequest::ProcessExec(request))? {
        PluginHostResponse::ProcessExec(result) => result_value(result),
        _ => Err(HostCallError::UnexpectedResponse("process_exec")),
    }
}

fn result_value<T>(result: PluginResult<T>) -> Result<T, HostCallError> {
    match result {
        PluginResult::Ok(value) => Ok(value),
        PluginResult::Err(error) => Err(HostCallError::Service(error)),
    }
}

struct ResponseHandle(u32);

impl Drop for ResponseHandle {
    fn drop(&mut self) {
        raw::response_drop(self.0);
    }
}

#[cfg(target_arch = "wasm32")]
mod raw {
    use super::HostCallError;

    #[link(wasm_import_module = "scryer:host/v1")]
    unsafe extern "C" {
        fn scryer_host_call(request_ptr: *const u8, request_len: usize) -> u32;
        fn scryer_host_response_len(handle: u32) -> i32;
        fn scryer_host_response_read(
            handle: u32,
            destination: *mut u8,
            destination_len: usize,
        ) -> i32;
        fn scryer_host_response_drop(handle: u32);
    }

    pub(super) fn call(request: &[u8]) -> Result<u32, HostCallError> {
        let handle = unsafe { scryer_host_call(request.as_ptr(), request.len()) };
        if handle == 0 {
            return Err(HostCallError::InvalidHandle);
        }
        Ok(handle)
    }

    pub(super) fn response_len(handle: u32) -> Result<usize, HostCallError> {
        let len = unsafe { scryer_host_response_len(handle) };
        usize::try_from(len).map_err(|_| HostCallError::InvalidHandle)
    }

    pub(super) fn response_read(handle: u32, destination: &mut [u8]) -> Result<(), HostCallError> {
        let read = unsafe {
            scryer_host_response_read(handle, destination.as_mut_ptr(), destination.len())
        };
        if read == destination.len() as i32 {
            Ok(())
        } else {
            Err(HostCallError::ReadFailed)
        }
    }

    pub(super) fn response_drop(handle: u32) {
        unsafe { scryer_host_response_drop(handle) };
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod raw {
    use super::HostCallError;

    pub(super) fn call(_request: &[u8]) -> Result<u32, HostCallError> {
        Err(HostCallError::Unavailable)
    }

    pub(super) fn response_len(_handle: u32) -> Result<usize, HostCallError> {
        Err(HostCallError::Unavailable)
    }

    pub(super) fn response_read(
        _handle: u32,
        _destination: &mut [u8],
    ) -> Result<(), HostCallError> {
        Err(HostCallError::Unavailable)
    }

    pub(super) fn response_drop(_handle: u32) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_targets_report_host_unavailable() {
        let error = config_get("base_url").expect_err("native tests have no wasm host ABI");
        assert!(matches!(error, HostCallError::Unavailable));
    }
}
