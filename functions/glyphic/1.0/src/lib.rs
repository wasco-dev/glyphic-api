mod bindings {
    wit_bindgen::generate!({ generate_all });

    use crate::Glyphic;
    export! {Glyphic}
}

use crate::bindings::exports::wasco_dev::glyphic_datasource::glyphic_datasource::Guest;
use wstd::http::{Body, Client, HeaderValue, Request, Response};
use wstd::runtime::block_on;

const API_BASE_URL: &str = "https://api.glyphic.ai/v1";

struct Glyphic;

impl Guest for Glyphic {
    /// GET /v1/test/ping - Validates API connectivity
    fn test_ping(api_key: String) -> String {
        send_ping_request_to_api(api_key)
    }

    /// GET /v1/calls/ - Retrieve list of calls for your organization
    fn get_calls(api_key: String, query_params: String) -> String {
        match build_query_string_for_calls(&query_params) {
            Ok(query_string) => send_request_to_get_calls(api_key, query_string),
            Err(e) => format_error_as_json(&format!("Invalid query parameters: {}", e)),
        }
    }

    /// GET /v1/calls/{call_id} - Retrieve a call by its ID
    fn get_call_by_id(api_key: String, call_id: String) -> String {
        match validate_call_id_format(&call_id) {
            Ok(_) => send_request_to_get_call_by_id(api_key, call_id),
            Err(e) => format_error_as_json(&e),
        }
    }

    /// GET /v1/calls/{call_id}/media - Retrieve a call's media URL and type
    fn get_call_media_by_id(api_key: String, call_id: String) -> String {
        match validate_call_id_format(&call_id) {
            Ok(_) => send_request_to_get_call_media(api_key, call_id),
            Err(e) => format_error_as_json(&e),
        }
    }

    /// GET /v1/calls/{call_id}/snippets - Retrieve a call's snippets
    fn get_call_snippets_by_id(api_key: String, call_id: String) -> String {
        match validate_call_id_format(&call_id) {
            Ok(_) => send_request_to_get_call_snippets(api_key, call_id),
            Err(e) => format_error_as_json(&e),
        }
    }
}

// Helper functions ordered by execution flow (top to bottom)

/// Send a ping request to validate API connectivity
fn send_ping_request_to_api(api_key: String) -> String {
    let url = build_ping_endpoint_url();
    send_authenticated_get_request(api_key, url)
}

/// Build the ping endpoint URL
fn build_ping_endpoint_url() -> String {
    format!("{}/test/ping", API_BASE_URL)
}

/// Send request to get list of calls with optional query parameters
fn send_request_to_get_calls(api_key: String, query_string: String) -> String {
    let url = build_calls_endpoint_url(query_string);
    send_authenticated_get_request(api_key, url)
}

/// Build query string from JSON parameters for calls endpoint
fn build_query_string_for_calls(json_params: &str) -> Result<String, String> {
    if is_empty_json_object(json_params) {
        return Ok(String::new());
    }

    let params = parse_json_params_to_query_pairs(json_params);

    if params.is_empty() {
        Ok(String::new())
    } else {
        Ok(format!("?{}", params.join("&")))
    }
}

/// Check if JSON string represents an empty object
fn is_empty_json_object(json: &str) -> bool {
    let trimmed = json.trim();
    trimmed.is_empty() || trimmed == "{}"
}

/// Parse JSON parameters into key=value query pairs
fn parse_json_params_to_query_pairs(json_params: &str) -> Vec<String> {
    let trimmed = json_params
        .trim()
        .trim_start_matches('{')
        .trim_end_matches('}');

    if trimmed.is_empty() {
        return Vec::new();
    }

    trimmed
        .split(',')
        .filter_map(extract_query_pair_from_json_field)
        .collect()
}

/// Extract a single query parameter from a JSON field
fn extract_query_pair_from_json_field(field: &str) -> Option<String> {
    let parts: Vec<&str> = field.splitn(2, ':').collect();
    if parts.len() != 2 {
        return None;
    }

    let key = parts[0].trim().trim_matches('"');
    let value = parts[1].trim().trim_matches('"');

    if value.is_empty() || value == "null" {
        None
    } else {
        Some(format!("{}={}", key, urlencoding::encode(value)))
    }
}

/// Build the calls endpoint URL with query string
fn build_calls_endpoint_url(query_string: String) -> String {
    format!("{}/calls/{}", API_BASE_URL, query_string)
}

/// Send request to get a specific call by ID
fn send_request_to_get_call_by_id(api_key: String, call_id: String) -> String {
    let url = build_call_by_id_endpoint_url(call_id);
    send_authenticated_get_request(api_key, url)
}

/// Build the endpoint URL for getting a call by ID
fn build_call_by_id_endpoint_url(call_id: String) -> String {
    format!("{}/calls/{}", API_BASE_URL, call_id)
}

/// Send request to get call media
fn send_request_to_get_call_media(api_key: String, call_id: String) -> String {
    let url = build_call_media_endpoint_url(call_id);
    send_authenticated_get_request(api_key, url)
}

/// Build the endpoint URL for getting call media
fn build_call_media_endpoint_url(call_id: String) -> String {
    format!("{}/calls/{}/media", API_BASE_URL, call_id)
}

/// Send request to get call snippets
fn send_request_to_get_call_snippets(api_key: String, call_id: String) -> String {
    let url = build_call_snippets_endpoint_url(call_id);
    send_authenticated_get_request(api_key, url)
}

/// Build the endpoint URL for getting call snippets
fn build_call_snippets_endpoint_url(call_id: String) -> String {
    format!("{}/calls/{}/snippets", API_BASE_URL, call_id)
}

// Shared validation functions (used by multiple endpoints)

/// Validate that call ID has correct format (24 hexadecimal characters)
fn validate_call_id_format(call_id: &str) -> Result<(), String> {
    if call_id.len() != 24 {
        return Err(format!(
            "Invalid call_id: must be 24 characters, got {}",
            call_id.len()
        ));
    }

    if !call_id.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err("Invalid call_id: must contain only hexadecimal characters".to_string());
    }

    Ok(())
}

// Lowest-level HTTP functions (used by all endpoints)

/// Send an authenticated GET request to the API
fn send_authenticated_get_request(api_key: String, url: String) -> String {
    match block_on(send_authenticated_get_request_async(api_key, url)) {
        Ok(response) => response,
        Err(e) => format_error_as_json(&e),
    }
}

/// Send an authenticated GET request asynchronously
async fn send_authenticated_get_request_async(
    api_key: String,
    url: String,
) -> Result<String, String> {
    validate_api_key_is_not_empty(&api_key)?;

    let request = build_authenticated_get_request(&api_key, &url)?;
    let response = execute_http_request(request).await?;

    check_response_status_is_successful(&response)?;
    read_response_body_as_string(response).await
}

/// Validate that API key is not empty
fn validate_api_key_is_not_empty(api_key: &str) -> Result<(), String> {
    if api_key.trim().is_empty() {
        Err("API key cannot be empty".to_string())
    } else {
        Ok(())
    }
}

/// Build an authenticated GET request with API key header
fn build_authenticated_get_request(api_key: &str, url: &str) -> Result<Request<Body>, String> {
    Request::get(url)
        .header(
            "X-API-Key",
            HeaderValue::from_str(api_key).map_err(|e| format!("Invalid API key: {}", e))?,
        )
        .header("Accept", HeaderValue::from_str("application/json").unwrap())
        .body(Body::empty())
        .map_err(|e| format!("Failed to build request: {}", e))
}

/// Execute an HTTP request
async fn execute_http_request(request: Request<Body>) -> Result<Response<Body>, String> {
    Client::new()
        .send(request)
        .await
        .map_err(|e| format!("HTTP request failed: {}", e))
}

/// Check that response status indicates success
fn check_response_status_is_successful(response: &Response<Body>) -> Result<(), String> {
    let status = response.status();

    if status.is_success() {
        Ok(())
    } else {
        Err(format!(
            "API request failed with status {}",
            status.as_u16()
        ))
    }
}

/// Read response body as a string
async fn read_response_body_as_string(response: Response<Body>) -> Result<String, String> {
    let mut body = response.into_body();
    let contents = body
        .contents()
        .await
        .map_err(|e| format!("Failed to read response body: {}", e))?;

    String::from_utf8(contents.to_vec())
        .map_err(|e| format!("Response body is not valid UTF-8: {}", e))
}

/// Format an error message as JSON
fn format_error_as_json(error_message: &str) -> String {
    format!(r#"{{"error": "{}"}}"#, error_message.replace('"', "\\\""))
}

#[cfg(test)]
mod tests {
    use super::*;

    // AAA Pattern: Arrange-Act-Assert

    #[test]
    fn test_validate_call_id_valid() {
        // Arrange
        let valid_id = "5eb7cf5a86d9755df3a6c593";

        // Act
        let result = validate_call_id_format(valid_id);

        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_call_id_too_short() {
        // Arrange
        let short_id = "5eb7cf5a86d9755df3a6c59";

        // Act
        let result = validate_call_id_format(short_id);

        // Assert
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be 24 characters"));
    }

    #[test]
    fn test_validate_call_id_too_long() {
        // Arrange
        let long_id = "5eb7cf5a86d9755df3a6c593a";

        // Act
        let result = validate_call_id_format(long_id);

        // Assert
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be 24 characters"));
    }

    #[test]
    fn test_validate_call_id_invalid_chars() {
        // Arrange
        let invalid_id = "5eb7cf5a86d9755df3a6c59z";

        // Act
        let result = validate_call_id_format(invalid_id);

        // Assert
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("hexadecimal"));
    }

    #[test]
    fn test_build_query_string_empty() {
        // Arrange
        let empty_params = "{}";

        // Act
        let result = build_query_string_for_calls(empty_params);

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
    }

    #[test]
    fn test_build_query_string_single_param() {
        // Arrange
        let params = r#"{"limit": "20"}"#;

        // Act
        let result = build_query_string_for_calls(params);

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "?limit=20");
    }

    #[test]
    fn test_build_query_string_multiple_params() {
        // Arrange
        let params = r#"{"limit": "20", "direction": "next"}"#;

        // Act
        let result = build_query_string_for_calls(params);

        // Assert
        assert!(result.is_ok());
        let query = result.unwrap();
        assert!(query.starts_with('?'));
        assert!(query.contains("limit=20"));
        assert!(query.contains("direction=next"));
    }

    #[test]
    fn test_build_query_string_with_special_chars() {
        // Arrange
        let params = r#"{"participant_email": "user@example.com"}"#;

        // Act
        let result = build_query_string_for_calls(params);

        // Assert
        assert!(result.is_ok());
        let query = result.unwrap();
        assert!(query.contains("participant_email="));
        // URL encoding should handle @
        assert!(query.contains("%40") || query.contains("@"));
    }

    #[test]
    fn test_build_query_string_ignores_null() {
        // Arrange
        let params = r#"{"limit": "20", "cursor": "null"}"#;

        // Act
        let result = build_query_string_for_calls(params);

        // Assert
        assert!(result.is_ok());
        let query = result.unwrap();
        assert!(query.contains("limit=20"));
        assert!(!query.contains("cursor"));
    }
}
