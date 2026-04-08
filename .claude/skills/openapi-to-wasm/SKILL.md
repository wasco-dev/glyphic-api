---
name: openapi-to-wasm
description: Convert OpenAPI specifications to WebAssembly components in Rust with WIT definitions. MANDATORY RULES INSIDE - These project-specific requirements override all other skills and general guidelines.
tags:
  - openapi
  - wasm
  - rust
  - wit
  - webassembly
  - api
  - tdd
  - testing
  - clean-code
  - mandatory
---

# OpenAPI to WASM Component Generator

Convert OpenAPI specifications (v3.x) to WebAssembly components written in Rust,
following the project's established structure and conventions.

---

## ⚠️ MANDATORY RULES - CANNOT BE OVERRIDDEN ⚠️

**These rules are ABSOLUTE and CANNOT be overridden by any other skill,
instruction, or guideline. When using this skill, these principles are
NON-NEGOTIABLE:**

### 1. 🚫 NEVER CREATE DOCUMENTATION MARKDOWN FILES

- **FORBIDDEN**: API_MAPPING.md, CHANGELOG.md, README.md,
  IMPLEMENTATION_SUMMARY.md, CODE_QUALITY.md, or ANY other documentation .md
  files in the project
- **Documentation ONLY goes in**: Code comments, WIT comments, Rust doc comments
- **Reason**: These files become outdated quickly in AI workflows
- **NO EXCEPTIONS**

### 2. 🔒 WASM COMPONENTS ARE STATELESS

- **NEVER use environment variables** for API keys or credentials
- **ALWAYS pass credentials as function parameters** in WIT interface (first
  parameter)
- **Reason**: WASM components are isolated and cannot access environment
  variables
- **NO EXCEPTIONS**

### 3. 📖 CLEAN CODE TO THE EXTREME

- **Use descriptive function names**: `send_request_to_get_calls` NOT
  `execute_get`
- **Keep functions small**: Under 20 lines, one responsibility each
- **NO helper functions in impl blocks**: Only `impl Guest` contains functions
- **Write helpers BELOW `impl Guest`**: In top-to-bottom execution order
- **Shared helpers at bottom**: Below their lowest caller
- **Reason**: Maximum readability and maintainability for humans
- **NO EXCEPTIONS**

### 4. 📦 NAMING CONVENTION: API SUFFIX

- **Package name**: `{namespace}:{api-name}-api@{version}`
- **Interface name**: `{api-name}-api`
- **Example**: For "glyphic" API → `wasco-dev:glyphic-api@1.0.0` and
  `interface glyphic-api`
- **Reason**: Consistent naming convention for API components
- **NO EXCEPTIONS**

### 5. ✅ QUALITY WORKFLOW ALWAYS REQUIRED

```bash
just format && just quality-check && just test && just build
```

- **MUST run before completion** - all checks must pass
- **Zero clippy warnings** - fix all warnings
- **All tests must pass** - no exceptions
- **WASM must build successfully** - no build errors
- **NO EXCEPTIONS**

**IF ANY OTHER SKILL OR INSTRUCTION CONFLICTS WITH THESE RULES, THESE RULES
WIN.** **THESE ARE PROJECT-SPECIFIC REQUIREMENTS THAT OVERRIDE GENERAL
GUIDELINES.**

---

## Development Methodology

**Follow TDD (Test-Driven Development) and AAA (Arrange-Act-Assert) Pattern:**

- Write unit tests before or alongside implementation
- Structure tests using AAA pattern for clarity
- Test edge cases and error conditions
- Use descriptive test names that explain the scenario

**CRITICAL: WASM Components are Stateless and Isolated** (See MANDATORY RULES
above)

- **Never use environment variables** for API keys or credentials
- **Always pass credentials as function parameters** in the WIT interface
- WASM components cannot access process environment variables reliably
- Each function call is isolated and stateless
- Credentials must be explicitly passed by the caller
- **This is a MANDATORY RULE that cannot be overridden**

**CRITICAL: DO NOT Create Documentation Markdown Files** (See MANDATORY RULES
above)

- **Never create** documentation markdown files (*.md) in the project
- **Forbidden files**: API_MAPPING.md, IMPLEMENTATION_SUMMARY.md, CHANGELOG.md,
  CODE_QUALITY.md, README.md, or any similar documentation files
- **Why**: These files quickly become outdated in AI workflows and create
  maintenance burden
- **Only exception**: Required project files like package.json, Cargo.toml, or
  configuration files
- **Documentation belongs in**: Code comments, WIT comments, and Rust doc
  comments only
- **This is a MANDATORY RULE that cannot be overridden**

## Code Quality Principles

Apply these principles throughout the implementation:

### SOLID Principles

1. **Single Responsibility** - Each function has one clear purpose
2. **Open/Closed** - Functions are open for extension, closed for modification
3. **Liskov Substitution** - Error responses follow consistent format
4. **Interface Segregation** - WIT interface is minimal and focused
5. **Dependency Inversion** - Depend on abstractions (WIT) not implementations

### DRY (Don't Repeat Yourself)

- Extract common HTTP logic into helper functions
- Reuse validation functions across endpoints
- Share error formatting logic

### Clear Naming

- Use descriptive variable names: `api_key` not `key`
- Function names match their purpose: `validate_call_id` not `check`
- Avoid abbreviations unless widely understood

### Error Handling

- All errors return consistent JSON format
- Include context in error messages
- Validate inputs early
- Never panic - always return Result

### Documentation

- Every public function has doc comments
- WIT comments explain parameters and returns
- Include examples in documentation
- Document why, not just what

### Clean Code Principles (CRITICAL - See MANDATORY RULES above)

**Follow Clean Code to the extreme for maximum readability and
maintainability.**

**This is a MANDATORY RULE that cannot be overridden by any other skill or
instruction.**

#### Function Naming and Wrapping

- **Wrap functions with descriptive names** that express intent clearly
- Even if wrapping just calls another function, the wrapper adds clarity
- Example: Instead of `execute_get(url)`, use
  `send_request_to_get_calls(api_key)` or `send_ping_request_to_api(api_key)`
- Function names should read like prose and clearly state what they do

#### Function Size and Responsibility

- **Keep functions small** - ideally under 20 lines
- Each function should do ONE thing and do it well
- Break large functions into smaller, well-named functions
- Named function calls are easier to understand than large code blocks

#### Code Organization

- **DO NOT write helper functions inside `impl` blocks** (except `impl Guest`)
- Write helper functions **below** the `impl Guest` block
- Maintain **top-to-bottom execution order**: functions are ordered by their
  level of abstraction and execution flow
- **Shared functions go below their lowest caller**: If multiple functions use a
  helper, place it below the lowest function that calls it

#### Example Structure

```rust
struct ApiClient;

impl Guest for ApiClient {
    fn test_ping(api_key: String) -> String {
        send_ping_request_to_api(api_key)  // Clear intent
    }

    fn get_calls(api_key: String, query_params: String) -> String {
        let query_string = build_query_string_for_calls(&query_params);
        send_request_to_get_calls(api_key, query_string)  // Clear intent
    }

    fn get_call_by_id(api_key: String, call_id: String) -> String {
        validate_call_id_format(&call_id);
        send_request_to_get_call_by_id(api_key, call_id)  // Clear intent
    }
}

// Helper functions in execution order (top to bottom)

fn send_ping_request_to_api(api_key: String) -> String {
    let url = build_ping_endpoint_url();
    send_authenticated_get_request(api_key, url)
}

fn build_ping_endpoint_url() -> String {
    format!("{}/test/ping", API_BASE_URL)
}

fn send_request_to_get_calls(api_key: String, query_string: String) -> String {
    let url = build_calls_endpoint_url(query_string);
    send_authenticated_get_request(api_key, url)
}

fn build_query_string_for_calls(json_params: &str) -> String {
    // Implementation
}

fn build_calls_endpoint_url(query_string: String) -> String {
    format!("{}/calls/{}", API_BASE_URL, query_string)
}

fn send_request_to_get_call_by_id(api_key: String, call_id: String) -> String {
    let url = build_call_by_id_endpoint_url(call_id);
    send_authenticated_get_request(api_key, url)
}

fn validate_call_id_format(call_id: &str) -> Result<(), String> {
    // Used by multiple functions, so placed after them
}

fn build_call_by_id_endpoint_url(call_id: String) -> String {
    format!("{}/calls/{}", API_BASE_URL, call_id)
}

// Lowest-level shared function used by all above functions
fn send_authenticated_get_request(api_key: String, url: String) -> String {
    // HTTP implementation
}
```

#### Benefits

- **Human-readable**: Code reads like a story from top to bottom
- **Intent is clear**: Function names document what the code does
- **Easy to maintain**: Small functions are easy to test and modify
- **Easy to understand**: No need to parse complex blocks, names tell the story

## Quality Workflow (MANDATORY - See MANDATORY RULES above)

**ALWAYS run these commands before completing. This is NON-NEGOTIABLE:**

```bash
just format && just quality-check && just test && just build
```

This ensures:

1. **`just format`** - Formats all Rust code with `cargo fmt`
2. **`just quality-check`** - Runs `cargo clippy` for linting
3. **`just test`** - Runs all Rust unit tests (`cargo test`)
4. **`just build`** - Builds the WASM component

**All checks must pass before considering the work complete.**

**MANDATORY: Zero clippy warnings, all tests pass, WASM builds. NO EXCEPTIONS.**

### Individual Commands

If you need to run checks individually:

```bash
# Format code
just format

# Quality checks
just quality-check

# Run tests
just test

# Build WASM component
just build

# Clean build artifacts
just clean
```

## When to Use

Activate this skill when:

- Converting an OpenAPI spec to a WASM component
- Generating WIT definitions from API endpoints
- Scaffolding new WASM function projects from API specs
- Creating WebAssembly wrappers for REST APIs

## Project Structure

The skill generates components following this flat structure:

```
.
├── Cargo.toml              # Rust package manifest with release optimizations
├── Justfile                # Build commands
├── wasmcloud.toml          # WasmCloud component configuration
├── wkg.lock                # WIT dependencies lock file
├── src/
│   └── lib.rs              # Main implementation
├── wit/
│   └── world.wit           # WIT interface definition
├── .github/
│   └── workflows/
│       └── cd.yml          # CI/CD pipeline for GHCR publishing
└── .gitignore
```

**Key files:**
- **Cargo.toml**: Includes `[profile.release]` optimizations for minimal WASM size
- **wasmcloud.toml**: Specifies component metadata (name, version, type)
- **Justfile**: Simple build commands (no workspace complexity)
- **.github/workflows/cd.yml**: Auto-publishes to GitHub Container Registry on push

## Conversion Process

### 1. Analyze OpenAPI Spec

Parse the OpenAPI specification to extract:

- API name and version
- Endpoints and operations (GET, POST, PUT, DELETE, etc.)
- Parameters (path, query, body)
- Request/response schemas
- Descriptions and documentation

### 2. Generate WIT Interface

Create a WIT interface where each API endpoint becomes a function:

**Naming Convention:**

- Convert endpoint paths to function names
- HTTP method becomes prefix when needed
- Use kebab-case in WIT (e.g., `get-users`, `create-user`)

**Example Mappings (with API key parameter):**

```
GET /users          → get-users: func(api-key: string) -> string
GET /users/{id}     → get-user-by-id: func(api-key: string, id: string) -> string
POST /users         → create-user: func(api-key: string, body: string) -> string
PUT /users/{id}     → update-user: func(api-key: string, id: string, body: string) -> string
DELETE /users/{id}  → delete-user: func(api-key: string, id: string) -> string
GET /users/{id}/posts → get-user-posts: func(api-key: string, user-id: string) -> string
```

**Important:** API key (or other authentication credentials) should always be
the first parameter.

**WIT Type Mappings:**

```
OpenAPI Type       → WIT Type
string             → string
integer            → s32 or s64
number             → f32 or f64
boolean            → bool
array              → list<T>
object (complex)   → string (JSON serialized)
```

**Generated WIT Structure:**

```wit
package {namespace}:{api-name}-api@{version};

interface {api-name}-api {
    // Function for each endpoint (api-key is always first parameter)
    get-users: func(api-key: string) -> string;
    get-user-by-id: func(api-key: string, id: string) -> string;
    create-user: func(api-key: string, body: string) -> string;
    // ... more functions
}

world main {
    export {api-name}-api;
}
```

**Authentication Parameter Conventions:**

- API key/token parameter should be named `api-key`, `auth-token`, or similar
- Always the first parameter in every function
- Documented in WIT comments for each function

### 3. Generate Rust Implementation

Create `src/lib.rs` with:

```rust
mod bindings {
    wit_bindgen::generate!({ generate_all });

    use crate::{ApiName};
    export! {ApiName}
}

use crate::bindings::exports::{namespace}::{api_name}::{api_name}::Guest;

struct ApiName;

impl Guest for ApiName {
    fn get_users() -> String {
        todo!("Implement GET /users")
    }

    fn get_user_by_id(_id: String) -> String {
        todo!("Implement GET /users/{id}")
    }

    fn create_user(_body: String) -> String {
        todo!("Implement POST /users")
    }

    // ... more implementations
}
```

**Rust Naming Convention:**

- Use snake_case for function names (e.g., `get_users`, `create_user`)
- Use UpperCamelCase for struct name (e.g., `ApiName`)

### 4. Generate Cargo.toml

```toml
[package]
name = "{api-name}"
version = "{version}"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[dependencies]
wit-bindgen = "0.42.0"
wstd = "0.6.6"
```

### 5. Generate Supporting Files

**Justfile:** Copy from `workspace-justfile` template (see project root)

**function.json:**

```json
{}
```

**wkg.lock:** Auto-generated by `wkg wit fetch` when the project is built.
Initial content:

```
# This file is automatically generated.
# It is not intended for manual editing.
version = 1
packages = []
```

**wit/deps/ directory:** Created automatically by `wkg wit fetch` for WIT
dependencies. This directory is gitignored.

### 6. Generate Tests

Create test file at `tests/functions/{api-name}/{version}/index.test.ts`:

**Important:** The WASM file name matches the package name (kebab-case), and
functions are destructured in camelCase.

```typescript
import { compileComponent } from "../../../compile-component.ts";
const { test } = Deno;
import { expect } from "@std/expect";

const wasmPath = "./functions/{api-name}/{version}/{api-name}.wasm";

const {
  {apiName}: { getUsers, getUserById, createUser },
} = await compileComponent(wasmPath);

test("get users works", () => {
  const result = getUsers();
  expect(result).toBeTruthy();
});

test("get user by id works", () => {
  const result = getUserById("123");
  expect(result).toBeTruthy();
});
```

**Function name conversions in tests:**

- WIT: `get-users` (kebab-case)
- Rust: `get_users` (snake_case)
- TypeScript: `getUsers` (camelCase)
- Package/Interface: `api-name` (kebab-case)
- TypeScript object: `apiName` (camelCase)

## Type Conversion Rules

### Path Parameters

All path parameters become string arguments in order:

```
GET /users/{userId}/posts/{postId}
→ get-user-post: func(user-id: string, post-id: string) -> string
```

### Query Parameters

For simple cases, add query parameters as optional string arguments:

```
GET /users?filter=active&limit=10
→ get-users: func(filter: option<string>, limit: option<string>) -> string
```

For complex queries, use a single JSON string parameter:

```
→ get-users: func(query: string) -> string
```

### Request Bodies

Complex request bodies are passed as JSON strings:

```
POST /users with JSON body
→ create-user: func(body: string) -> string
```

For simple bodies with 1-2 fields, optionally expand to parameters:

```
POST /users with { name, email }
→ create-user: func(name: string, email: string) -> string
```

### Response Bodies

Return values are JSON strings for complex responses:

```
→ func() -> string  // JSON response
```

For simple responses, use appropriate WIT types:

```
→ func() -> bool    // Boolean response
→ func() -> s32     // Number response
```

### Headers

For APIs requiring specific headers:

- Document required headers in function comments
- Pass as JSON string if many headers needed
- Add dedicated parameters for critical headers (e.g., auth tokens)

## Advanced Features

### Authentication

For APIs with authentication:

```wit
// Add auth parameter to functions that need it
get-protected-resource: func(auth-token: string, id: string) -> string;
```

Or use a global config approach:

```wit
set-auth-token: func(token: string);
get-protected-resource: func(id: string) -> string;
```

### Error Handling

Use result types for operations that can fail:

```wit
record error {
    code: s32,
    message: string,
}

get-user: func(id: string) -> result<string, error>;
```

### Streaming/Pagination

For paginated endpoints:

```wit
record page-params {
    offset: s32,
    limit: s32,
}

get-users-page: func(params: page-params) -> string;
```

### Versioning

Place each API version in its own directory:

```
functions/myapi/1.0/    # v1.0
functions/myapi/2.0/    # v2.0
```

## Step-by-Step Workflow

When asked to convert an OpenAPI spec:

1. **Read the OpenAPI spec** file
2. **Extract metadata**: API name, version, base path
3. **Parse all endpoints**: paths, methods, parameters, schemas
4. **Generate WIT interface**:
   - Create function for each endpoint
   - Map types appropriately
   - Add documentation comments
   - Use kebab-case for function names
5. **Generate Rust implementation**:
   - Create struct and impl block (snake_case functions)
   - Add todo!() for each function with helpful error messages
   - Include helpful comments about request/response formats
6. **Generate Cargo.toml** with proper metadata
7. **Copy Justfile** from workspace-justfile template
8. **Create empty function.json** (just `{}`)
9. **Create initial wkg.lock** file
10. **Generate test file** with basic test cases
    - Use camelCase for function names in TypeScript
    - WASM file name matches package name (kebab-case)
11. **Document usage**: Create comments explaining the API mapping

## Documentation Standards

Include in WIT comments:

- HTTP method and path
- Required parameters
- Expected response format
- Authentication requirements
- Example usage

Example:

```wit
/// GET /users/{id}
/// Retrieves a user by their ID
///
/// # Parameters
/// - id: User identifier
///
/// # Returns
/// JSON string with user object: {"id": "...", "name": "...", "email": "..."}
get-user-by-id: func(id: string) -> string;
```

## Rust Implementation Patterns

### HTTP Client Setup

Use `wstd` for HTTP operations in WASM components:

```rust
use wstd::http::{Body, Client, HeaderValue, Request};
use wstd::runtime::block_on;

const API_BASE_URL: &str = "https://api.example.com/v1";
const API_KEY_ENV: &str = "API_KEY";
```

### Async/Sync Handling

WIT-generated functions are synchronous, but `wstd::http::Client` is async. Use
`wstd::runtime::block_on` to bridge the gap:

```rust
impl Guest for ApiName {
    fn get_data() -> String {
        // Sync function calls async implementation
        match block_on(Self::get_data_async()) {
            Ok(response) => response,
            Err(e) => format!(r#"{{"error": "{}"}}"#, e),
        }
    }
}

async fn get_data_async() -> Result<String, String> {
    let request = Request::get(url)
        .header("Authorization", HeaderValue::from_str(&api_key)?)
        .body(Body::empty())?;

    let response = Client::new().send(request).await?;

    let mut body = response.into_body();
    let contents = body.contents().await?;

    String::from_utf8(contents.to_vec())
}
```

**Important:** Body must be declared as `mut` to call `.contents()`.

### Authentication

**CRITICAL: Pass API keys as function parameters, NOT from environment
variables**

WASM components are stateless and isolated. They cannot reliably access
environment variables. Always pass credentials as parameters:

**WIT Definition:**

```wit
interface api {
    /// # Parameters
    /// - api-key: API key for authentication
    /// - id: Resource identifier
    get-resource: func(api-key: string, id: string) -> string;
}
```

**Rust Implementation:**

```rust
impl Guest for Api {
    fn get_resource(api_key: String, id: String) -> String {
        // Validate API key is not empty
        if api_key.trim().is_empty() {
            return error_json("API key cannot be empty");
        }

        // Use the api_key parameter directly
        match block_on(Self::fetch_async(api_key, id)) {
            Ok(result) => result,
            Err(e) => error_json(e),
        }
    }
}

async fn fetch_async(api_key: String, id: String) -> Result<String, String> {
    let request = Request::get(url)
        .header("X-API-Key", HeaderValue::from_str(&api_key)?)
        .header("Authorization", HeaderValue::from_str(&format!("Bearer {}", api_key))?)
        .body(Body::empty())?;

    // ... rest of implementation
}
```

**Why not environment variables?**

- WASM components are isolated and stateless
- Cannot access process environment variables
- Credentials must come from the caller
- Makes component reusable across different contexts

### Error Handling

Return JSON-formatted errors for consistency:

```rust
fn handle_error(e: impl std::fmt::Display) -> String {
    // Escape quotes in error messages
    format!(r#"{{"error": "{}"}}"#, e.to_string().replace('"', "\\\""))
}
```

Check HTTP status and read error bodies:

```rust
if !status.is_success() {
    let mut body = response.into_body();
    let contents = body.contents().await?;
    let error_msg = String::from_utf8_lossy(contents);
    return Err(format!(
        "API request failed with status {}: {}",
        status.as_u16(),
        error_msg
    ));
}
```

### Query Parameters

Parse JSON parameters and build query strings:

```rust
fn build_query_string(json_params: &str) -> Result<String, String> {
    if json_params.trim().is_empty() || json_params.trim() == "{}" {
        return Ok(String::new());
    }

    // Simple parsing (use serde_json for production)
    let params: Vec<String> = json_params
        .trim()
        .trim_start_matches('{')
        .trim_end_matches('}')
        .split(',')
        .filter_map(|pair| {
            let parts: Vec<&str> = pair.splitn(2, ':').collect();
            if parts.len() == 2 {
                let key = parts[0].trim().trim_matches('"');
                let value = parts[1].trim().trim_matches('"');
                if !value.is_empty() && value != "null" {
                    // URL encode the value
                    Some(format!("{}={}", key, urlencoding::encode(value)))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    if params.is_empty() {
        Ok(String::new())
    } else {
        Ok(format!("?{}", params.join("&")))
    }
}
```

**Add `urlencoding` dependency:**

```toml
[dependencies]
urlencoding = "2.1"
```

### Input Validation

Validate parameters before making API calls:

```rust
fn validate_id(id: &str) -> Result<(), String> {
    if id.len() != 24 {
        return Err(format!("Invalid ID: must be 24 characters, got {}", id.len()));
    }

    if !id.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err("Invalid ID: must contain only hexadecimal characters".to_string());
    }

    Ok(())
}
```

Call validation before making requests:

```rust
fn get_by_id(id: String) -> String {
    if let Err(e) = Self::validate_id(&id) {
        return format!(r#"{{"error": "{}"}}"#, e);
    }

    // Proceed with API call...
}
```

### Unit Testing with TDD

Follow Test-Driven Development and AAA (Arrange-Act-Assert) pattern:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // AAA Pattern: Arrange-Act-Assert

    #[test]
    fn test_validate_id_valid() {
        // Arrange
        let valid_id = "5eb7cf5a86d9755df3a6c593";

        // Act
        let result = ApiName::validate_id(valid_id);

        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_id_too_short() {
        // Arrange
        let short_id = "5eb7cf5a86d9755df3a6c59";

        // Act
        let result = ApiName::validate_id(short_id);

        // Assert
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be 24 characters"));
    }

    #[test]
    fn test_build_query_string_empty() {
        // Arrange
        let empty_params = "{}";

        // Act
        let result = ApiName::build_query_string(empty_params);

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
    }

    #[test]
    fn test_build_query_string_with_special_chars() {
        // Arrange
        let params = r#"{"email": "user@example.com"}"#;

        // Act
        let result = ApiName::build_query_string(params);

        // Assert
        assert!(result.is_ok());
        let query = result.unwrap();
        assert!(query.contains("email="));
        // URL encoding should handle @
        assert!(query.contains("%40"));
    }
}
```

**Test coverage should include:**

- Valid inputs (happy path)
- Invalid inputs (error cases)
- Edge cases (empty strings, null values, special characters)
- Boundary conditions (length limits, format requirements)

### Dependencies

Essential Cargo dependencies:

```toml
[dependencies]
wit-bindgen = "0.42.0"  # WIT bindings generation
wstd = "0.6.6"          # Async runtime and HTTP client
urlencoding = "2.1"     # URL encode query parameters
```

Optional (for production):

- `serde_json` - Robust JSON parsing
- `anyhow` - Enhanced error handling
- `thiserror` - Custom error types

## Build and Test

After generation, run the complete quality workflow:

```bash
# 1. Build the component
just build

# 2. Run full quality workflow (includes tests)
just format && just quality-check && just test && just build
```

**Complete Quality Workflow:**

```bash
just format && just quality-check && just test && just build
```

This runs:

- Code formatting (Rust)
- Clippy linting
- Rust unit tests
- WASM build

**All checks must pass before the implementation is complete.**

### Individual Commands

```bash
# Format
just format

# Quality check
just quality-check

# Test
just test

# Build
just build

# Clean
just clean
```

## Common Patterns

### RESTful CRUD Operations

```wit
interface resource {
    // List all
    list: func() -> string;

    // Get by ID
    get: func(id: string) -> string;

    // Create
    create: func(body: string) -> string;

    // Update
    update: func(id: string, body: string) -> string;

    // Delete
    delete: func(id: string) -> string;
}
```

### Nested Resources

```wit
// GET /users/{userId}/posts
get-user-posts: func(user-id: string) -> string;

// POST /users/{userId}/posts
create-user-post: func(user-id: string, body: string) -> string;
```

### Batch Operations

```wit
// POST /users/batch
create-users-batch: func(bodies: list<string>) -> string;

// DELETE /users/batch?ids=1,2,3
delete-users-batch: func(ids: list<string>) -> string;
```

## Error Prevention

- **Name collisions**: If multiple operations map to same function name, append
  HTTP method
- **Reserved keywords**: Avoid Rust/WIT keywords in function names
- **Type compatibility**: Ensure OpenAPI types can be represented in WIT
- **Path complexity**: For very complex paths, simplify function names while
  maintaining clarity

## Integration with Project

This skill follows the project's established structure:

- Uses `just build` for compilation
- Follows naming conventions (kebab-case in WIT, snake_case in Rust, camelCase in TypeScript)
- Uses standard dependencies (wit-bindgen, wstd)
- Includes GitHub Actions CD pipeline for automatic publishing

## Validation

After generation, verify:

- [ ] WIT file syntax is valid (`wkg wit check`)
- [ ] Cargo.toml has correct metadata and release optimizations
- [ ] wasmcloud.toml has correct component configuration
- [ ] All endpoints are mapped to functions
- [ ] Function names follow conventions
- [ ] Component builds successfully (`just build`)
- [ ] All tests pass (`just test`)

## Example Usage

User provides OpenAPI spec for "petstore" API v1.0.0:

```yaml
openapi: 3.0.0
info:
  title: Petstore
  version: 1.0.0
paths:
  /pets:
    get:
      summary: List all pets
      responses:
        "200":
          description: List of pets
  /pets/{petId}:
    get:
      summary: Get a pet by ID
      parameters:
        - name: petId
          in: path
          required: true
          schema:
            type: string
```

Generated structure:

```
functions/petstore/1.0/
├── Cargo.toml
├── Justfile
├── function.json
├── src/
│   └── lib.rs
└── wit/
    └── world.wit
```

WIT content:

```wit
package wasco-dev:petstore@1.0.0;

interface petstore {
    /// GET /pets - List all pets
    get-pets: func() -> string;

    /// GET /pets/{petId} - Get a pet by ID
    get-pet-by-id: func(pet-id: string) -> string;
}

world main {
    export petstore;
}
```

## Tips

- Start with simple endpoints first
- Use JSON strings for complex types
- Document the mapping from OpenAPI to WIT
- Include authentication strategy in generated comments
- Test with `just build` after generation
- Run `cargo clippy` to catch Rust issues early
- Use the rust-development skill for implementation guidance
- Follow TDD: write tests before or alongside implementation
- Use AAA pattern in tests for clarity
- Validate inputs before making API calls
- Handle errors gracefully with JSON responses
- URL encode query parameters to handle special characters

## Best Practices from Implementation

### 1. Async/Sync Bridge

WASM component functions are synchronous, but modern HTTP clients are async. Use
`wstd::runtime::block_on` to bridge the gap:

```rust
fn sync_function() -> String {
    match block_on(async_implementation()) {
        Ok(result) => result,
        Err(e) => error_json(e),
    }
}
```

### 2. Mutable Body

The `wstd::http::Body` type requires mut to read contents:

```rust
let mut body = response.into_body();
let contents = body.contents().await?;
```

### 3. Error Consistency

Always return JSON-formatted errors for consistency with success responses:

```rust
Err(e) => format!(r#"{{"error": "{}"}}"#, e.replace('"', "\\\""))
```

### 4. URL Encoding

Always URL encode query parameter values:

```rust
format!("{}={}", key, urlencoding::encode(value))
```

### 5. Pass Credentials as Parameters

**DO NOT use environment variables** - WASM components cannot access them
reliably.

**ALWAYS pass credentials as function parameters:**

```rust
// WIT
get-data: func(api-key: string) -> string;

// Rust
fn get_data(api_key: String) -> String {
    if api_key.trim().is_empty() {
        return error_json("API key required");
    }
    // Use api_key...
}
```

### 6. Status Checking

Always check HTTP status before reading body:

```rust
if !status.is_success() {
    // Read error body and return error
}
// Read success body
```

### 7. Test Coverage

Write tests for:

- Valid inputs (happy path)
- Invalid inputs (error handling)
- Edge cases (empty, null, special chars)
- Boundary conditions (length, format)

### 8. Dependency Management

Essential dependencies:

- `wit-bindgen` - Required for WIT bindings
- `wstd` - Provides async runtime and HTTP client
- `urlencoding` - For URL encoding query parameters

Optional but recommended:

- `serde_json` - For robust JSON parsing
- `anyhow` - For better error handling

## Common Pitfalls to Avoid

### Critical Issues

1. **Using environment variables for API keys** - ❌ WRONG! WASM components are
   stateless. Always pass credentials as parameters
2. **Forgetting api-key parameter** - Every function needs authentication
   parameter as first argument
3. **Creating documentation markdown files** - ❌ FORBIDDEN! No API_MAPPING.md,
   CHANGELOG.md, README.md, etc. They get outdated quickly
4. **Not running quality checks** - Always run the full quality workflow before
   completing

### Code Quality Issues

4. **Forgetting `mut` on body** - Body needs to be mutable to read contents
5. **Not checking status** - Always check response status before reading body
6. **Missing URL encoding** - Special characters in query params must be encoded
7. **Not validating inputs** - Validate parameters before making API calls
8. **Inconsistent error format** - Return all errors as JSON
9. **Missing error context** - Include HTTP status codes in error messages
10. **No unit tests** - Write tests for validation and helper functions
11. **Code duplication** - Extract common logic into helper functions (DRY)
12. **Poor naming** - Use descriptive names, avoid abbreviations
13. **Missing documentation** - Every public item needs doc comments
14. **Not formatting code** - Run `deno fmt` and `just format`
15. **Ignoring clippy warnings** - Fix all warnings before completing

### Clean Code Violations

16. **Generic function names** - Use `send_request_to_get_calls` not
    `execute_get`
17. **Large functions** - Break functions over 20 lines into smaller named
    functions
18. **Helper functions in impl blocks** - Write helpers below `impl Guest`, not
    in `impl StructName`
19. **Wrong function order** - Order functions top-to-bottom by execution flow
20. **Shared helpers at top** - Place shared helpers below their lowest caller

## Implementation Checklist

When implementing an OpenAPI to WASM component:

### Design Phase

- [ ] **Add api-key parameter** to ALL WIT functions (first parameter)
- [ ] Generate WIT definitions with proper documentation
- [ ] Plan helper functions (validation, query building, HTTP execution)
- [ ] Design error handling strategy (JSON format)

### Implementation Phase

- [ ] Implement HTTP client with `wstd`
- [ ] Add `block_on` wrapper for sync/async bridge
- [ ] **Pass API key as parameter** (not from environment variables)
- [ ] Validate API key is not empty
- [ ] Add query parameter building with URL encoding
- [ ] Implement input validation functions (DRY principle)
- [ ] Extract common logic into helper methods (SRP)
- [ ] Handle errors with JSON responses
- [ ] Add comprehensive doc comments
- [ ] **Follow Clean Code**: Use descriptive function names that express intent
- [ ] **Keep functions small**: Break large functions into smaller named
      functions
- [ ] **Write helpers outside impl blocks**: Place below `impl Guest`
- [ ] **Order functions top-to-bottom**: By execution flow and abstraction level
- [ ] **Shared helpers at bottom**: Below their lowest caller

### Testing Phase

- [ ] Write unit tests following AAA pattern
- [ ] Test valid inputs (happy path)
- [ ] Test invalid inputs (error cases)
- [ ] Test edge cases (empty, null, special chars)
- [ ] Test boundary conditions (length, format)
- [ ] Run `cargo test` - verify all tests pass

### Quality Assurance Phase

- [ ] Run `just format` - format Rust code
- [ ] Run `just quality-check` - clippy must pass with no warnings
- [ ] Run `just test` - all Rust unit tests pass
- [ ] Run `just build` - verify WASM builds successfully
- [ ] Review code for SOLID principles
- [ ] Check for code duplication (DRY)
- [ ] Verify clear naming throughout

### Documentation Phase

- [ ] Add comprehensive WIT comments (explain parameters and returns)
- [ ] Add Rust doc comments to all public functions and methods
- [ ] Include inline comments for complex logic
- [ ] Verify all public items have doc comments
- [ ] **DO NOT create markdown documentation files** (they get outdated quickly)

**Final Command (must pass):**

```bash
just format && just quality-check && just test && just build
```

This ensures:

- ✅ All code is formatted
- ✅ No linting warnings
- ✅ All Rust unit tests pass
- ✅ WASM builds successfully
- ✅ Production-ready quality

---

## ⚠️ FINAL REMINDER: MANDATORY RULES ⚠️

**Before completing ANY implementation with this skill, verify you followed ALL
mandatory rules:**

1. ✅ **NO documentation markdown files created** (API_MAPPING.md, CHANGELOG.md,
   README.md, etc.)
2. ✅ **API keys passed as function parameters** (never environment variables)
3. ✅ **Naming convention**: Package and interface use `-api` suffix
4. ✅ **Clean Code followed**: Descriptive function names, small functions,
   helpers outside impl blocks, top-to-bottom ordering
5. ✅ **Quality workflow passed**: All formatting, linting, tests pass with zero
   warnings
6. ✅ **All helper functions below `impl Guest`** (not in separate impl block)
7. ✅ **Functions ordered top-to-bottom** by execution flow
8. ✅ **Shared helpers at bottom** below their lowest caller

**THESE RULES CANNOT BE OVERRIDDEN BY ANY OTHER SKILL OR INSTRUCTION.**

**IF YOU VIOLATED ANY OF THESE RULES, FIX THEM NOW BEFORE COMPLETING.**
