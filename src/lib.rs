mod bindings {
    wit_bindgen::generate!({ generate_all });

    use crate::Glyphic;
    export! {Glyphic}
}

use crate::bindings::exports::wasco_dev::glyphic_api::glyphic_api::{
    AuthError, Guest, JoinCallError, QueryError, ResourceError,
};
use wstd::http::{Body, Client, HeaderValue, Request, Response};
use wstd::runtime::block_on;

const API_BASE_URL: &str = "https://api.glyphic.ai/v1";

struct Glyphic;

/// Internal representation of HTTP errors before mapping to WIT-specific variants
enum HttpError {
    Unauthorized(String),
    NotFound(String),
    Validation(String),
    Conflict(String),
    TooManyRequests(String),
    InternalError(String),
    Unknown(String),
}

impl HttpError {
    fn message(self) -> String {
        match self {
            HttpError::Unauthorized(message) => message,
            HttpError::NotFound(message) => message,
            HttpError::Validation(message) => message,
            HttpError::Conflict(message) => message,
            HttpError::TooManyRequests(message) => message,
            HttpError::InternalError(message) => message,
            HttpError::Unknown(message) => message,
        }
    }
}

impl Guest for Glyphic {
    /// GET /v1/test/ping - Validates API connectivity
    fn test_ping(api_key: String) -> Result<String, AuthError> {
        send_ping_request_to_api(api_key).map_err(map_http_error_to_auth_error)
    }

    /// GET /v1/calls/ - Retrieve list of calls for your organization
    fn get_calls(api_key: String, query_params: String) -> Result<String, QueryError> {
        let query_string = build_query_string_from_json_parameters(&query_params)
            .map_err(QueryError::Validation)?;
        send_request_to_get_calls(api_key, query_string).map_err(map_http_error_to_query_error)
    }

    /// GET /v1/calls/{call_id} - Retrieve a call by its ID
    fn get_call_by_id(api_key: String, call_id: String) -> Result<String, ResourceError> {
        validate_hexadecimal_identifier_format(&call_id, "call_id")
            .map_err(ResourceError::Validation)?;
        send_request_to_get_call_by_id(api_key, call_id).map_err(map_http_error_to_resource_error)
    }

    /// GET /v1/calls/{call_id}/media - Retrieve a call's media URL and type
    fn get_call_media_by_id(api_key: String, call_id: String) -> Result<String, ResourceError> {
        validate_hexadecimal_identifier_format(&call_id, "call_id")
            .map_err(ResourceError::Validation)?;
        send_request_to_get_call_media(api_key, call_id).map_err(map_http_error_to_resource_error)
    }

    /// GET /v1/calls/{call_id}/snippets - Retrieve a call's snippets
    fn get_call_snippets_by_id(api_key: String, call_id: String) -> Result<String, ResourceError> {
        validate_hexadecimal_identifier_format(&call_id, "call_id")
            .map_err(ResourceError::Validation)?;
        send_request_to_get_call_snippets(api_key, call_id)
            .map_err(map_http_error_to_resource_error)
    }

    /// POST /v1/call_bots - Join a call with a Glyphic bot
    fn join_call(api_key: String, request_body: String) -> Result<String, JoinCallError> {
        validate_join_call_request_body(&request_body).map_err(JoinCallError::Validation)?;
        send_join_call_request(api_key, request_body).map_err(map_http_error_to_join_call_error)
    }

    /// GET /v1/call_tags/ - List all call tags for your organization
    fn list_call_tags(api_key: String) -> Result<String, AuthError> {
        send_request_to_list_call_tags(api_key).map_err(map_http_error_to_auth_error)
    }

    /// GET /v1/playbooks/ - List playbooks for your organization
    fn list_playbooks(api_key: String, query_params: String) -> Result<String, QueryError> {
        let query_string = build_query_string_from_json_parameters(&query_params)
            .map_err(QueryError::Validation)?;
        send_request_to_list_playbooks(api_key, query_string).map_err(map_http_error_to_query_error)
    }

    /// GET /v1/playbooks/{id} - Retrieve a playbook by its ID
    fn get_playbook_by_id(api_key: String, playbook_id: String) -> Result<String, ResourceError> {
        validate_hexadecimal_identifier_format(&playbook_id, "playbook_id")
            .map_err(ResourceError::Validation)?;
        send_request_to_get_playbook_by_id(api_key, playbook_id)
            .map_err(map_http_error_to_resource_error)
    }

    /// GET /v1/playbooks/{id}/versions - List versions of a playbook
    fn list_playbook_versions(
        api_key: String,
        playbook_id: String,
    ) -> Result<String, ResourceError> {
        validate_hexadecimal_identifier_format(&playbook_id, "playbook_id")
            .map_err(ResourceError::Validation)?;
        send_request_to_list_playbook_versions(api_key, playbook_id)
            .map_err(map_http_error_to_resource_error)
    }

    /// GET /v1/playbooks/{id}/versions/{vid} - Retrieve a specific playbook version
    fn get_playbook_version_by_id(
        api_key: String,
        playbook_id: String,
        version_id: String,
    ) -> Result<String, ResourceError> {
        validate_hexadecimal_identifier_format(&playbook_id, "playbook_id")
            .map_err(ResourceError::Validation)?;
        validate_hexadecimal_identifier_format(&version_id, "version_id")
            .map_err(ResourceError::Validation)?;
        send_request_to_get_playbook_version_by_id(api_key, playbook_id, version_id)
            .map_err(map_http_error_to_resource_error)
    }
}

// Ping helpers

/// Send a ping request to validate API connectivity
fn send_ping_request_to_api(api_key: String) -> Result<String, HttpError> {
    let url = build_ping_endpoint_url();
    send_authenticated_get_request(api_key, url)
}

/// Build the ping endpoint URL
fn build_ping_endpoint_url() -> String {
    format!("{}/test/ping", API_BASE_URL)
}

// Calls helpers

/// Send request to get list of calls with optional query parameters
fn send_request_to_get_calls(api_key: String, query_string: String) -> Result<String, HttpError> {
    let url = build_calls_endpoint_url(query_string);
    send_authenticated_get_request(api_key, url)
}

/// Build the calls endpoint URL with query string
fn build_calls_endpoint_url(query_string: String) -> String {
    format!("{}/calls/{}", API_BASE_URL, query_string)
}

/// Send request to get a specific call by ID
fn send_request_to_get_call_by_id(api_key: String, call_id: String) -> Result<String, HttpError> {
    let url = build_call_by_id_endpoint_url(&call_id);
    send_authenticated_get_request(api_key, url)
}

/// Build the endpoint URL for getting a call by ID
fn build_call_by_id_endpoint_url(call_id: &str) -> String {
    format!("{}/calls/{}", API_BASE_URL, call_id)
}

/// Send request to get call media
fn send_request_to_get_call_media(api_key: String, call_id: String) -> Result<String, HttpError> {
    let url = build_call_media_endpoint_url(&call_id);
    send_authenticated_get_request(api_key, url)
}

/// Build the endpoint URL for getting call media
fn build_call_media_endpoint_url(call_id: &str) -> String {
    format!("{}/calls/{}/media", API_BASE_URL, call_id)
}

/// Send request to get call snippets
fn send_request_to_get_call_snippets(
    api_key: String,
    call_id: String,
) -> Result<String, HttpError> {
    let url = build_call_snippets_endpoint_url(&call_id);
    send_authenticated_get_request(api_key, url)
}

/// Build the endpoint URL for getting call snippets
fn build_call_snippets_endpoint_url(call_id: &str) -> String {
    format!("{}/calls/{}/snippets", API_BASE_URL, call_id)
}

// Join call helpers

/// Send request to join a call with a bot
fn send_join_call_request(api_key: String, request_body: String) -> Result<String, HttpError> {
    let url = build_call_bots_endpoint_url();
    send_authenticated_post_request(api_key, url, request_body)
}

/// Build the call bots endpoint URL
fn build_call_bots_endpoint_url() -> String {
    format!("{}/call_bots", API_BASE_URL)
}

/// Validate that the join call request body contains required fields
fn validate_join_call_request_body(body: &str) -> Result<(), String> {
    let trimmed = body.trim();
    if trimmed.is_empty() || trimmed == "{}" {
        return Err("Request body cannot be empty".to_string());
    }
    if !trimmed.contains("\"meeting_url\"") {
        return Err("Request body must contain meeting_url field".to_string());
    }
    Ok(())
}

// Call tags helpers

/// Send request to list call tags
fn send_request_to_list_call_tags(api_key: String) -> Result<String, HttpError> {
    let url = build_call_tags_endpoint_url();
    send_authenticated_get_request(api_key, url)
}

/// Build the call tags endpoint URL
fn build_call_tags_endpoint_url() -> String {
    format!("{}/call_tags/", API_BASE_URL)
}

// Playbooks helpers

/// Send request to list playbooks with optional query parameters
fn send_request_to_list_playbooks(
    api_key: String,
    query_string: String,
) -> Result<String, HttpError> {
    let url = build_playbooks_endpoint_url(query_string);
    send_authenticated_get_request(api_key, url)
}

/// Build the playbooks endpoint URL with query string
fn build_playbooks_endpoint_url(query_string: String) -> String {
    format!("{}/playbooks/{}", API_BASE_URL, query_string)
}

/// Send request to get a specific playbook by ID
fn send_request_to_get_playbook_by_id(
    api_key: String,
    playbook_id: String,
) -> Result<String, HttpError> {
    let url = build_playbook_by_id_endpoint_url(&playbook_id);
    send_authenticated_get_request(api_key, url)
}

/// Build the endpoint URL for getting a playbook by ID
fn build_playbook_by_id_endpoint_url(playbook_id: &str) -> String {
    format!("{}/playbooks/{}", API_BASE_URL, playbook_id)
}

/// Send request to list playbook versions
fn send_request_to_list_playbook_versions(
    api_key: String,
    playbook_id: String,
) -> Result<String, HttpError> {
    let url = build_playbook_versions_endpoint_url(&playbook_id);
    send_authenticated_get_request(api_key, url)
}

/// Build the endpoint URL for listing playbook versions
fn build_playbook_versions_endpoint_url(playbook_id: &str) -> String {
    format!("{}/playbooks/{}/versions", API_BASE_URL, playbook_id)
}

/// Send request to get a specific playbook version by ID
fn send_request_to_get_playbook_version_by_id(
    api_key: String,
    playbook_id: String,
    version_id: String,
) -> Result<String, HttpError> {
    let url = build_playbook_version_by_id_endpoint_url(&playbook_id, &version_id);
    send_authenticated_get_request(api_key, url)
}

/// Build the endpoint URL for getting a specific playbook version
fn build_playbook_version_by_id_endpoint_url(playbook_id: &str, version_id: &str) -> String {
    format!(
        "{}/playbooks/{}/versions/{}",
        API_BASE_URL, playbook_id, version_id
    )
}

// Shared validation

/// Validate that an identifier has correct format (24 hexadecimal characters)
fn validate_hexadecimal_identifier_format(
    identifier: &str,
    field_name: &str,
) -> Result<(), String> {
    if identifier.len() != 24 {
        return Err(format!(
            "Invalid {}: must be 24 characters, got {}",
            field_name,
            identifier.len()
        ));
    }

    if !identifier.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(format!(
            "Invalid {}: must contain only hexadecimal characters",
            field_name
        ));
    }

    Ok(())
}

// Shared query string building

/// Build query string from JSON parameters
fn build_query_string_from_json_parameters(json_params: &str) -> Result<String, String> {
    if is_empty_json_object(json_params) {
        return Ok(String::new());
    }

    let mut parameters = parse_json_params_to_query_pairs(json_params);
    let tag_id_parameters = extract_tag_ids_from_json_parameters(json_params);
    parameters.extend(tag_id_parameters);

    if parameters.is_empty() {
        Ok(String::new())
    } else {
        Ok(format!("?{}", parameters.join("&")))
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

    if value.is_empty() || value == "null" || value.starts_with('[') {
        None
    } else {
        Some(format!("{}={}", key, urlencoding::encode(value)))
    }
}

/// Extract tag_ids array from JSON parameters into individual query pairs
fn extract_tag_ids_from_json_parameters(json_params: &str) -> Vec<String> {
    let tag_ids_key = "\"tag_ids\"";
    let key_position = match json_params.find(tag_ids_key) {
        Some(position) => position,
        None => return Vec::new(),
    };

    let after_key = &json_params[key_position + tag_ids_key.len()..];
    let bracket_start = match after_key.find('[') {
        Some(position) => position,
        None => return Vec::new(),
    };

    let array_contents = &after_key[bracket_start + 1..];
    let bracket_end = match array_contents.find(']') {
        Some(position) => position,
        None => return Vec::new(),
    };

    let elements = &array_contents[..bracket_end];

    elements
        .split(',')
        .map(|element| element.trim().trim_matches('"'))
        .filter(|element| !element.is_empty())
        .map(|identifier| format!("tag_ids={}", urlencoding::encode(identifier)))
        .collect()
}

// Shared HTTP request functions

/// Send an authenticated GET request to the API
fn send_authenticated_get_request(api_key: String, url: String) -> Result<String, HttpError> {
    block_on(send_authenticated_get_request_async(api_key, url))
}

/// Send an authenticated GET request asynchronously
async fn send_authenticated_get_request_async(
    api_key: String,
    url: String,
) -> Result<String, HttpError> {
    validate_api_key_is_not_empty(&api_key)?;

    let request = build_authenticated_get_request(&api_key, &url).map_err(HttpError::Unknown)?;
    let response = execute_http_request(request)
        .await
        .map_err(HttpError::Unknown)?;

    read_response_body_with_status_check(response).await
}

/// Send an authenticated POST request to the API
fn send_authenticated_post_request(
    api_key: String,
    url: String,
    body: String,
) -> Result<String, HttpError> {
    block_on(send_authenticated_post_request_async(api_key, url, body))
}

/// Send an authenticated POST request asynchronously
async fn send_authenticated_post_request_async(
    api_key: String,
    url: String,
    body: String,
) -> Result<String, HttpError> {
    validate_api_key_is_not_empty(&api_key)?;

    let request =
        build_authenticated_post_request(&api_key, &url, body).map_err(HttpError::Unknown)?;
    let response = execute_http_request(request)
        .await
        .map_err(HttpError::Unknown)?;

    read_response_body_with_status_check(response).await
}

/// Validate that API key is not empty
fn validate_api_key_is_not_empty(api_key: &str) -> Result<(), HttpError> {
    if api_key.trim().is_empty() {
        Err(HttpError::Unauthorized(
            "API key cannot be empty".to_string(),
        ))
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

/// Build an authenticated POST request with API key header and JSON body
fn build_authenticated_post_request(
    api_key: &str,
    url: &str,
    body: String,
) -> Result<Request<Body>, String> {
    Request::post(url)
        .header(
            "X-API-Key",
            HeaderValue::from_str(api_key).map_err(|e| format!("Invalid API key: {}", e))?,
        )
        .header(
            "Content-Type",
            HeaderValue::from_str("application/json").unwrap(),
        )
        .header("Accept", HeaderValue::from_str("application/json").unwrap())
        .body(Body::from(body))
        .map_err(|e| format!("Failed to build request: {}", e))
}

/// Execute an HTTP request
async fn execute_http_request(request: Request<Body>) -> Result<Response<Body>, String> {
    Client::new()
        .send(request)
        .await
        .map_err(|e| format!("HTTP request failed: {}", e))
}

/// Read response body and check status, returning the appropriate HttpError on failure
async fn read_response_body_with_status_check(
    response: Response<Body>,
) -> Result<String, HttpError> {
    let status = response.status();
    let mut body = response.into_body();
    let contents = body
        .contents()
        .await
        .map_err(|e| HttpError::Unknown(format!("Failed to read response body: {}", e)))?;

    let body_string = String::from_utf8(contents.to_vec())
        .map_err(|e| HttpError::Unknown(format!("Response body is not valid UTF-8: {}", e)))?;

    if status.is_success() {
        Ok(body_string)
    } else {
        Err(map_status_code_to_http_error(status.as_u16(), body_string))
    }
}

// Error mapping functions

/// Map an HTTP status code to the appropriate HttpError variant
fn map_status_code_to_http_error(status: u16, body: String) -> HttpError {
    match status {
        401 => HttpError::Unauthorized(body),
        404 => HttpError::NotFound(body),
        409 => HttpError::Conflict(body),
        422 => HttpError::Validation(body),
        429 => HttpError::TooManyRequests(body),
        500 => HttpError::InternalError(body),
        _ => HttpError::Unknown(format!("HTTP {}: {}", status, body)),
    }
}

/// Map HttpError to AuthError (401, 429)
fn map_http_error_to_auth_error(error: HttpError) -> AuthError {
    match error {
        HttpError::Unauthorized(message) => AuthError::Unauthorized(message),
        HttpError::TooManyRequests(message) => AuthError::TooManyRequests(message),
        other => AuthError::Unknown(other.message()),
    }
}

/// Map HttpError to QueryError (401, 422, 429)
fn map_http_error_to_query_error(error: HttpError) -> QueryError {
    match error {
        HttpError::Unauthorized(message) => QueryError::Unauthorized(message),
        HttpError::Validation(message) => QueryError::Validation(message),
        HttpError::TooManyRequests(message) => QueryError::TooManyRequests(message),
        other => QueryError::Unknown(other.message()),
    }
}

/// Map HttpError to ResourceError (401, 404, 422, 429)
fn map_http_error_to_resource_error(error: HttpError) -> ResourceError {
    match error {
        HttpError::Unauthorized(message) => ResourceError::Unauthorized(message),
        HttpError::NotFound(message) => ResourceError::NotFound(message),
        HttpError::Validation(message) => ResourceError::Validation(message),
        HttpError::TooManyRequests(message) => ResourceError::TooManyRequests(message),
        other => ResourceError::Unknown(other.message()),
    }
}

/// Map HttpError to JoinCallError (401, 409, 422, 429, 500)
fn map_http_error_to_join_call_error(error: HttpError) -> JoinCallError {
    match error {
        HttpError::Unauthorized(message) => JoinCallError::Unauthorized(message),
        HttpError::Conflict(message) => JoinCallError::Conflict(message),
        HttpError::Validation(message) => JoinCallError::Validation(message),
        HttpError::TooManyRequests(message) => JoinCallError::TooManyRequests(message),
        HttpError::InternalError(message) => JoinCallError::InternalError(message),
        other => JoinCallError::Unknown(other.message()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // AAA Pattern: Arrange-Act-Assert

    // Hexadecimal identifier validation tests

    #[test]
    fn test_validate_hexadecimal_identifier_valid() {
        // Arrange
        let valid_id = "5eb7cf5a86d9755df3a6c593";

        // Act
        let result = validate_hexadecimal_identifier_format(valid_id, "call_id");

        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_hexadecimal_identifier_too_short() {
        // Arrange
        let short_id = "5eb7cf5a86d9755df3a6c59";

        // Act
        let result = validate_hexadecimal_identifier_format(short_id, "call_id");

        // Assert
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be 24 characters"));
    }

    #[test]
    fn test_validate_hexadecimal_identifier_too_long() {
        // Arrange
        let long_id = "5eb7cf5a86d9755df3a6c593a";

        // Act
        let result = validate_hexadecimal_identifier_format(long_id, "call_id");

        // Assert
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be 24 characters"));
    }

    #[test]
    fn test_validate_hexadecimal_identifier_invalid_chars() {
        // Arrange
        let invalid_id = "5eb7cf5a86d9755df3a6c59z";

        // Act
        let result = validate_hexadecimal_identifier_format(invalid_id, "call_id");

        // Assert
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("hexadecimal"));
    }

    #[test]
    fn test_validate_hexadecimal_identifier_includes_field_name_in_error() {
        // Arrange
        let short_id = "abc123";

        // Act
        let result = validate_hexadecimal_identifier_format(short_id, "playbook_id");

        // Assert
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("playbook_id"));
    }

    // Query string building tests

    #[test]
    fn test_build_query_string_empty() {
        // Arrange
        let empty_params = "{}";

        // Act
        let result = build_query_string_from_json_parameters(empty_params);

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
    }

    #[test]
    fn test_build_query_string_single_param() {
        // Arrange
        let parameters = r#"{"limit": "20"}"#;

        // Act
        let result = build_query_string_from_json_parameters(parameters);

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "?limit=20");
    }

    #[test]
    fn test_build_query_string_multiple_params() {
        // Arrange
        let parameters = r#"{"limit": "20", "direction": "next"}"#;

        // Act
        let result = build_query_string_from_json_parameters(parameters);

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
        let parameters = r#"{"participant_email": "user@example.com"}"#;

        // Act
        let result = build_query_string_from_json_parameters(parameters);

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
        let parameters = r#"{"limit": "20", "cursor": "null"}"#;

        // Act
        let result = build_query_string_from_json_parameters(parameters);

        // Assert
        assert!(result.is_ok());
        let query = result.unwrap();
        assert!(query.contains("limit=20"));
        assert!(!query.contains("cursor"));
    }

    // Join call request body validation tests

    #[test]
    fn test_validate_join_call_request_body_valid() {
        // Arrange
        let body = r#"{"meeting_url": "https://meet.example.com/abc"}"#;

        // Act
        let result = validate_join_call_request_body(body);

        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_join_call_request_body_empty() {
        // Arrange
        let body = "{}";

        // Act
        let result = validate_join_call_request_body(body);

        // Assert
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be empty"));
    }

    #[test]
    fn test_validate_join_call_request_body_empty_string() {
        // Arrange
        let body = "";

        // Act
        let result = validate_join_call_request_body(body);

        // Assert
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be empty"));
    }

    #[test]
    fn test_validate_join_call_request_body_missing_meeting_url() {
        // Arrange
        let body = r#"{"other_field": "value"}"#;

        // Act
        let result = validate_join_call_request_body(body);

        // Assert
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("meeting_url"));
    }

    // Extract tag IDs tests

    #[test]
    fn test_extract_tag_ids_with_array() {
        // Arrange
        let parameters = r#"{"tag_ids": ["abc123", "def456"]}"#;

        // Act
        let result = extract_tag_ids_from_json_parameters(parameters);

        // Assert
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], "tag_ids=abc123");
        assert_eq!(result[1], "tag_ids=def456");
    }

    #[test]
    fn test_extract_tag_ids_no_key() {
        // Arrange
        let parameters = r#"{"limit": "20"}"#;

        // Act
        let result = extract_tag_ids_from_json_parameters(parameters);

        // Assert
        assert!(result.is_empty());
    }

    #[test]
    fn test_extract_tag_ids_empty_array() {
        // Arrange
        let parameters = r#"{"tag_ids": []}"#;

        // Act
        let result = extract_tag_ids_from_json_parameters(parameters);

        // Assert
        assert!(result.is_empty());
    }

    #[test]
    fn test_build_query_string_with_tag_ids() {
        // Arrange
        let parameters = r#"{"limit": "20", "tag_ids": ["abc123", "def456"]}"#;

        // Act
        let result = build_query_string_from_json_parameters(parameters);

        // Assert
        assert!(result.is_ok());
        let query = result.unwrap();
        assert!(query.contains("limit=20"));
        assert!(query.contains("tag_ids=abc123"));
        assert!(query.contains("tag_ids=def456"));
        assert!(!query.contains("["));
    }

    // URL builder tests

    #[test]
    fn test_build_call_bots_endpoint_url() {
        // Arrange & Act
        let url = build_call_bots_endpoint_url();

        // Assert
        assert_eq!(url, "https://api.glyphic.ai/v1/call_bots");
    }

    #[test]
    fn test_build_call_tags_endpoint_url() {
        // Arrange & Act
        let url = build_call_tags_endpoint_url();

        // Assert
        assert_eq!(url, "https://api.glyphic.ai/v1/call_tags/");
    }

    #[test]
    fn test_build_playbooks_endpoint_url_no_query() {
        // Arrange & Act
        let url = build_playbooks_endpoint_url(String::new());

        // Assert
        assert_eq!(url, "https://api.glyphic.ai/v1/playbooks/");
    }

    #[test]
    fn test_build_playbooks_endpoint_url_with_query() {
        // Arrange & Act
        let url = build_playbooks_endpoint_url("?limit=20".to_string());

        // Assert
        assert_eq!(url, "https://api.glyphic.ai/v1/playbooks/?limit=20");
    }

    #[test]
    fn test_build_playbook_by_id_endpoint_url() {
        // Arrange
        let playbook_id = "5eb7cf5a86d9755df3a6c593";

        // Act
        let url = build_playbook_by_id_endpoint_url(playbook_id);

        // Assert
        assert_eq!(
            url,
            "https://api.glyphic.ai/v1/playbooks/5eb7cf5a86d9755df3a6c593"
        );
    }

    #[test]
    fn test_build_playbook_versions_endpoint_url() {
        // Arrange
        let playbook_id = "5eb7cf5a86d9755df3a6c593";

        // Act
        let url = build_playbook_versions_endpoint_url(playbook_id);

        // Assert
        assert_eq!(
            url,
            "https://api.glyphic.ai/v1/playbooks/5eb7cf5a86d9755df3a6c593/versions"
        );
    }

    #[test]
    fn test_build_playbook_version_by_id_endpoint_url() {
        // Arrange
        let playbook_id = "5eb7cf5a86d9755df3a6c593";
        let version_id = "aabbccdd11223344eeff5566";

        // Act
        let url = build_playbook_version_by_id_endpoint_url(playbook_id, version_id);

        // Assert
        assert_eq!(
            url,
            "https://api.glyphic.ai/v1/playbooks/5eb7cf5a86d9755df3a6c593/versions/aabbccdd11223344eeff5566"
        );
    }

    // Status code to HttpError mapping tests

    #[test]
    fn test_map_status_code_401_to_unauthorized() {
        // Arrange
        let body = "Unauthorized".to_string();

        // Act
        let error = map_status_code_to_http_error(401, body);

        // Assert
        assert!(matches!(error, HttpError::Unauthorized(message) if message == "Unauthorized"));
    }

    #[test]
    fn test_map_status_code_404_to_not_found() {
        // Arrange
        let body = "Not found".to_string();

        // Act
        let error = map_status_code_to_http_error(404, body);

        // Assert
        assert!(matches!(error, HttpError::NotFound(message) if message == "Not found"));
    }

    #[test]
    fn test_map_status_code_409_to_conflict() {
        // Arrange
        let body = "Conflict".to_string();

        // Act
        let error = map_status_code_to_http_error(409, body);

        // Assert
        assert!(matches!(error, HttpError::Conflict(message) if message == "Conflict"));
    }

    #[test]
    fn test_map_status_code_422_to_validation() {
        // Arrange
        let body = "Validation failed".to_string();

        // Act
        let error = map_status_code_to_http_error(422, body);

        // Assert
        assert!(matches!(error, HttpError::Validation(message) if message == "Validation failed"));
    }

    #[test]
    fn test_map_status_code_429_to_too_many_requests() {
        // Arrange
        let body = "Rate limited".to_string();

        // Act
        let error = map_status_code_to_http_error(429, body);

        // Assert
        assert!(matches!(error, HttpError::TooManyRequests(message) if message == "Rate limited"));
    }

    #[test]
    fn test_map_status_code_500_to_internal_error() {
        // Arrange
        let body = "Server error".to_string();

        // Act
        let error = map_status_code_to_http_error(500, body);

        // Assert
        assert!(matches!(error, HttpError::InternalError(message) if message == "Server error"));
    }

    #[test]
    fn test_map_status_code_unknown_to_unknown() {
        // Arrange
        let body = "Bad gateway".to_string();

        // Act
        let error = map_status_code_to_http_error(502, body);

        // Assert
        assert!(matches!(error, HttpError::Unknown(message) if message.contains("502")));
    }

    // HttpError message extraction tests

    #[test]
    fn test_http_error_message_extracts_inner_string() {
        // Arrange
        let error = HttpError::Unauthorized("test message".to_string());

        // Act
        let message = error.message();

        // Assert
        assert_eq!(message, "test message");
    }

    // Auth error mapping tests

    #[test]
    fn test_map_http_error_unauthorized_to_auth_error() {
        // Arrange
        let error = HttpError::Unauthorized("bad key".to_string());

        // Act
        let auth_error = map_http_error_to_auth_error(error);

        // Assert
        assert!(matches!(auth_error, AuthError::Unauthorized(message) if message == "bad key"));
    }

    #[test]
    fn test_map_http_error_too_many_requests_to_auth_error() {
        // Arrange
        let error = HttpError::TooManyRequests("slow down".to_string());

        // Act
        let auth_error = map_http_error_to_auth_error(error);

        // Assert
        assert!(
            matches!(auth_error, AuthError::TooManyRequests(message) if message == "slow down")
        );
    }

    #[test]
    fn test_map_http_error_not_found_falls_through_to_auth_unknown() {
        // Arrange
        let error = HttpError::NotFound("missing".to_string());

        // Act
        let auth_error = map_http_error_to_auth_error(error);

        // Assert
        assert!(matches!(auth_error, AuthError::Unknown(message) if message == "missing"));
    }

    // Query error mapping tests

    #[test]
    fn test_map_http_error_validation_to_query_error() {
        // Arrange
        let error = HttpError::Validation("bad input".to_string());

        // Act
        let query_error = map_http_error_to_query_error(error);

        // Assert
        assert!(matches!(query_error, QueryError::Validation(message) if message == "bad input"));
    }

    // Resource error mapping tests

    #[test]
    fn test_map_http_error_not_found_to_resource_error() {
        // Arrange
        let error = HttpError::NotFound("no such call".to_string());

        // Act
        let resource_error = map_http_error_to_resource_error(error);

        // Assert
        assert!(
            matches!(resource_error, ResourceError::NotFound(message) if message == "no such call")
        );
    }

    // Join call error mapping tests

    #[test]
    fn test_map_http_error_conflict_to_join_call_error() {
        // Arrange
        let error = HttpError::Conflict("already joined".to_string());

        // Act
        let join_call_error = map_http_error_to_join_call_error(error);

        // Assert
        assert!(
            matches!(join_call_error, JoinCallError::Conflict(message) if message == "already joined")
        );
    }

    #[test]
    fn test_map_http_error_internal_to_join_call_error() {
        // Arrange
        let error = HttpError::InternalError("server crash".to_string());

        // Act
        let join_call_error = map_http_error_to_join_call_error(error);

        // Assert
        assert!(
            matches!(join_call_error, JoinCallError::InternalError(message) if message == "server crash")
        );
    }

    // API key validation tests

    #[test]
    fn test_validate_empty_api_key_returns_unauthorized() {
        // Arrange
        let empty_key = "";

        // Act
        let result = validate_api_key_is_not_empty(empty_key);

        // Assert
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), HttpError::Unauthorized(_)));
    }

    #[test]
    fn test_validate_whitespace_api_key_returns_unauthorized() {
        // Arrange
        let whitespace_key = "   ";

        // Act
        let result = validate_api_key_is_not_empty(whitespace_key);

        // Assert
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), HttpError::Unauthorized(_)));
    }

    #[test]
    fn test_validate_valid_api_key_succeeds() {
        // Arrange
        let valid_key = "sk-test-key-123";

        // Act
        let result = validate_api_key_is_not_empty(valid_key);

        // Assert
        assert!(result.is_ok());
    }
}
