mod bindings {
    wit_bindgen::generate!({ generate_all });

    use crate::Glyphic;
    export! {Glyphic}
}

use crate::bindings::exports::wasco_dev::glyphic_api::glyphic_api::Guest;
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
        match build_query_string_from_json_parameters(&query_params) {
            Ok(query_string) => send_request_to_get_calls(api_key, query_string),
            Err(e) => format_error_as_json(&format!("Invalid query parameters: {}", e)),
        }
    }

    /// GET /v1/calls/{call_id} - Retrieve a call by its ID
    fn get_call_by_id(api_key: String, call_id: String) -> String {
        match validate_hexadecimal_identifier_format(&call_id, "call_id") {
            Ok(_) => send_request_to_get_call_by_id(api_key, call_id),
            Err(e) => format_error_as_json(&e),
        }
    }

    /// GET /v1/calls/{call_id}/media - Retrieve a call's media URL and type
    fn get_call_media_by_id(api_key: String, call_id: String) -> String {
        match validate_hexadecimal_identifier_format(&call_id, "call_id") {
            Ok(_) => send_request_to_get_call_media(api_key, call_id),
            Err(e) => format_error_as_json(&e),
        }
    }

    /// GET /v1/calls/{call_id}/snippets - Retrieve a call's snippets
    fn get_call_snippets_by_id(api_key: String, call_id: String) -> String {
        match validate_hexadecimal_identifier_format(&call_id, "call_id") {
            Ok(_) => send_request_to_get_call_snippets(api_key, call_id),
            Err(e) => format_error_as_json(&e),
        }
    }

    /// POST /v1/call_bots - Join a call with a Glyphic bot
    fn join_call(api_key: String, request_body: String) -> String {
        match validate_join_call_request_body(&request_body) {
            Ok(_) => send_join_call_request(api_key, request_body),
            Err(e) => format_error_as_json(&e),
        }
    }

    /// GET /v1/call_tags/ - List all call tags for your organization
    fn list_call_tags(api_key: String) -> String {
        send_request_to_list_call_tags(api_key)
    }

    /// GET /v1/playbooks/ - List playbooks for your organization
    fn list_playbooks(api_key: String, query_params: String) -> String {
        match build_query_string_from_json_parameters(&query_params) {
            Ok(query_string) => send_request_to_list_playbooks(api_key, query_string),
            Err(e) => format_error_as_json(&format!("Invalid query parameters: {}", e)),
        }
    }

    /// GET /v1/playbooks/{id} - Retrieve a playbook by its ID
    fn get_playbook_by_id(api_key: String, playbook_id: String) -> String {
        match validate_hexadecimal_identifier_format(&playbook_id, "playbook_id") {
            Ok(_) => send_request_to_get_playbook_by_id(api_key, playbook_id),
            Err(e) => format_error_as_json(&e),
        }
    }

    /// GET /v1/playbooks/{id}/versions - List versions of a playbook
    fn list_playbook_versions(api_key: String, playbook_id: String) -> String {
        match validate_hexadecimal_identifier_format(&playbook_id, "playbook_id") {
            Ok(_) => send_request_to_list_playbook_versions(api_key, playbook_id),
            Err(e) => format_error_as_json(&e),
        }
    }

    /// GET /v1/playbooks/{id}/versions/{vid} - Retrieve a specific playbook version
    fn get_playbook_version_by_id(
        api_key: String,
        playbook_id: String,
        version_id: String,
    ) -> String {
        if let Err(e) = validate_hexadecimal_identifier_format(&playbook_id, "playbook_id") {
            return format_error_as_json(&e);
        }
        if let Err(e) = validate_hexadecimal_identifier_format(&version_id, "version_id") {
            return format_error_as_json(&e);
        }
        send_request_to_get_playbook_version_by_id(api_key, playbook_id, version_id)
    }
}

// Ping helpers

/// Send a ping request to validate API connectivity
fn send_ping_request_to_api(api_key: String) -> String {
    let url = build_ping_endpoint_url();
    send_authenticated_get_request(api_key, url)
}

/// Build the ping endpoint URL
fn build_ping_endpoint_url() -> String {
    format!("{}/test/ping", API_BASE_URL)
}

// Calls helpers

/// Send request to get list of calls with optional query parameters
fn send_request_to_get_calls(api_key: String, query_string: String) -> String {
    let url = build_calls_endpoint_url(query_string);
    send_authenticated_get_request(api_key, url)
}

/// Build the calls endpoint URL with query string
fn build_calls_endpoint_url(query_string: String) -> String {
    format!("{}/calls/{}", API_BASE_URL, query_string)
}

/// Send request to get a specific call by ID
fn send_request_to_get_call_by_id(api_key: String, call_id: String) -> String {
    let url = build_call_by_id_endpoint_url(&call_id);
    send_authenticated_get_request(api_key, url)
}

/// Build the endpoint URL for getting a call by ID
fn build_call_by_id_endpoint_url(call_id: &str) -> String {
    format!("{}/calls/{}", API_BASE_URL, call_id)
}

/// Send request to get call media
fn send_request_to_get_call_media(api_key: String, call_id: String) -> String {
    let url = build_call_media_endpoint_url(&call_id);
    send_authenticated_get_request(api_key, url)
}

/// Build the endpoint URL for getting call media
fn build_call_media_endpoint_url(call_id: &str) -> String {
    format!("{}/calls/{}/media", API_BASE_URL, call_id)
}

/// Send request to get call snippets
fn send_request_to_get_call_snippets(api_key: String, call_id: String) -> String {
    let url = build_call_snippets_endpoint_url(&call_id);
    send_authenticated_get_request(api_key, url)
}

/// Build the endpoint URL for getting call snippets
fn build_call_snippets_endpoint_url(call_id: &str) -> String {
    format!("{}/calls/{}/snippets", API_BASE_URL, call_id)
}

// Join call helpers

/// Send request to join a call with a bot
fn send_join_call_request(api_key: String, request_body: String) -> String {
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
fn send_request_to_list_call_tags(api_key: String) -> String {
    let url = build_call_tags_endpoint_url();
    send_authenticated_get_request(api_key, url)
}

/// Build the call tags endpoint URL
fn build_call_tags_endpoint_url() -> String {
    format!("{}/call_tags/", API_BASE_URL)
}

// Playbooks helpers

/// Send request to list playbooks with optional query parameters
fn send_request_to_list_playbooks(api_key: String, query_string: String) -> String {
    let url = build_playbooks_endpoint_url(query_string);
    send_authenticated_get_request(api_key, url)
}

/// Build the playbooks endpoint URL with query string
fn build_playbooks_endpoint_url(query_string: String) -> String {
    format!("{}/playbooks/{}", API_BASE_URL, query_string)
}

/// Send request to get a specific playbook by ID
fn send_request_to_get_playbook_by_id(api_key: String, playbook_id: String) -> String {
    let url = build_playbook_by_id_endpoint_url(&playbook_id);
    send_authenticated_get_request(api_key, url)
}

/// Build the endpoint URL for getting a playbook by ID
fn build_playbook_by_id_endpoint_url(playbook_id: &str) -> String {
    format!("{}/playbooks/{}", API_BASE_URL, playbook_id)
}

/// Send request to list playbook versions
fn send_request_to_list_playbook_versions(api_key: String, playbook_id: String) -> String {
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
) -> String {
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

/// Send an authenticated POST request to the API
fn send_authenticated_post_request(api_key: String, url: String, body: String) -> String {
    match block_on(send_authenticated_post_request_async(api_key, url, body)) {
        Ok(response) => response,
        Err(e) => format_error_as_json(&e),
    }
}

/// Send an authenticated POST request asynchronously
async fn send_authenticated_post_request_async(
    api_key: String,
    url: String,
    body: String,
) -> Result<String, String> {
    validate_api_key_is_not_empty(&api_key)?;

    let request = build_authenticated_post_request(&api_key, &url, body)?;
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
}
