# OpenAPI to WASM Component Skill

This skill converts OpenAPI specifications to WebAssembly components in Rust
with WIT definitions.

## Installation

This skill is local to the project and located in
`.claude/skills/openapi-to-wasm/`.

## Usage

To use this skill, invoke it with:

```
/openapi-to-wasm
```

Or simply mention to Claude Code:

```
"Convert this OpenAPI spec to a WASM component"
```

Then provide or reference your OpenAPI specification file.

## What It Does

1. Parses your OpenAPI specification
2. Generates WIT definitions with each endpoint as a function
3. Creates Rust boilerplate code
4. Sets up the project structure following conventions
5. Generates test files

## Project Structure

The skill generates:

```
functions/{api-name}/{version}/
├── Cargo.toml          # Rust project configuration
├── Justfile            # Build commands
├── function.json       # Function metadata
├── src/
│   └── lib.rs         # Rust implementation
└── wit/
    └── world.wit      # WIT interface definition
```

## Example

Given an OpenAPI spec for a "users" API, the skill will:

1. Create `functions/users/1.0/` directory structure
2. Generate WIT functions like:
   - `get-users: func() -> string`
   - `get-user-by-id: func(id: string) -> string`
   - `create-user: func(body: string) -> string`
3. Generate corresponding Rust implementation stubs
4. Create test files in `tests/functions/users/1.0/`

## Type Mappings

| OpenAPI Type | WIT Type | Notes              |
| ------------ | -------- | ------------------ |
| string       | string   | Direct mapping     |
| integer      | s32/s64  | Based on format    |
| number       | f32/f64  | Based on format    |
| boolean      | bool     | Direct mapping     |
| array        | list<T>  | Mapped recursively |
| object       | string   | JSON serialized    |

## Build and Test

After generation:

```bash
# Build the component
just build

# Run tests
deno task test
```

## Requirements

- OpenAPI spec version 3.x
- Valid YAML or JSON format
- Endpoints with defined operations

## Tips

- Simple endpoints work best
- Complex types are passed as JSON strings
- Path parameters become function arguments
- Query parameters can be optional parameters or JSON strings

## Related Skills

- `rust-development` - For implementing the generated Rust code
- `webassembly-component-development` - For WASM component best practices
