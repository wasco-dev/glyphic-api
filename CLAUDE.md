# WASM Functions Project

This project contains WebAssembly components built with Rust and WIT
specifications.

## Project Structure

```
functions/
└── {api-name}/
    └── {version}/
        ├── Cargo.toml      # Rust project configuration
        ├── Justfile        # Build automation
        ├── function.json   # Function metadata (typically empty)
        ├── src/
        │   └── lib.rs     # Rust implementation
        └── wit/
            └── world.wit   # WIT interface definition
```

## Naming Conventions

- **WIT files**: Use kebab-case (e.g., `get-users`, `create-user`)
- **Rust code**: Use snake_case (e.g., `get_users`, `create_user`)
- **Struct names**: Use UpperCamelCase (e.g., `ApiName`)
- **Package naming**: `wasco-dev:{api-name}@{version}`

## Building

```bash
# Build all functions
just build

# Build specific function
cd functions/{api-name}/{version}
just build
```

The build process:

1. Fetches WIT dependencies with `wkg wit fetch`
2. Compiles to WASM with `cargo build --release --target wasm32-wasip2`
3. Moves the WASM file to the function root directory

## Testing

Tests are written in TypeScript using Deno:

```bash
# Run all tests
deno task test

# This will:
# 1. Build all components (via generate-wasm task)
# 2. Run Deno tests with proper permissions
```

Test files are located at: `tests/functions/{api-name}/{version}/index.test.ts`

## WIT Structure

Every function follows this WIT pattern:

```wit
package wasco-dev:{api-name}@{version};

interface {api-name} {
    // Export functions here
    function-name: func(param: string) -> string;
}

world main {
    export {api-name};
}
```

## Rust Implementation Pattern

```rust
mod bindings {
    wit_bindgen::generate!({ generate_all });

    use crate::ApiName;
    export! {ApiName}
}

use crate::bindings::exports::wasco_dev::api_name::api_name::Guest;

struct ApiName;

impl Guest for ApiName {
    fn function_name(_param: String) -> String {
        todo!()
    }
}
```

## Dependencies

- `wit-bindgen = "0.42.0"` - Generate Rust bindings from WIT
- `wstd = "0.6.6"` - WebAssembly Standard Library

## Custom Skills

### OpenAPI to WASM

Location: `.claude/skills/openapi-to-wasm/`

This skill converts OpenAPI specifications to WASM components:

- Parses OpenAPI v3.x specs
- Generates WIT definitions (each endpoint → function)
- Creates Rust boilerplate
- Generates test files
- Follows project conventions

Usage: Mention converting an OpenAPI spec, or use `/openapi-to-wasm`

## Common Operations

### Adding a New Function

1. Create directory: `functions/{name}/{version}/`
2. Generate or copy WIT file to `wit/world.wit`
3. Create `Cargo.toml` with cdylib crate type
4. Implement in `src/lib.rs`
5. Copy `Justfile` from workspace-justfile
6. Create empty `function.json`
7. Add tests in `tests/functions/{name}/{version}/`

### Type Conversions

| Purpose         | Type                       |
| --------------- | -------------------------- |
| Simple strings  | `string`                   |
| Numbers         | `s32`, `s64`, `f32`, `f64` |
| Booleans        | `bool`                     |
| Lists           | `list<T>`                  |
| Complex objects | `string` (JSON)            |
| Optional values | `option<T>`                |

### Working with JSON

For complex types, use JSON strings:

```rust
fn process_data(json_input: String) -> String {
    // Parse JSON
    // Process
    // Return JSON
    todo!()
}
```

## Code Quality

Before committing:

```bash
# Format
just format

# Check formatting
just format-check

# Lint
just quality-check

# Test
just test
```

## Project Commands

- `just build` - Build all functions
- `just test` - Test all functions (Rust tests)
- `just format` - Format all Rust code
- `just format-check` - Check formatting
- `just quality-check` - Run clippy
- `just clean` - Clean build artifacts
- `just integration-test` - Run Deno integration tests
