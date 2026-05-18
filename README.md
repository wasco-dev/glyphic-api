
# Glyphic API
This is a Rust WebAssembly component that provides the Glyphic API integration functionality.

## Functionality
- ping: Send a request to Glyphic with your API key to see if you have a valid connection.
- get-calls: Get all the public calls for your organization.
- get-call-by-id: Get information about a specific call.
- get-call-media-by-id: Get all media for a specific call.
- get-call-snippets-by-id: Get transcript snippets for a specific call.

## Using
You can build this component by running the following command in the project in your terminal:
```Bash
wkg wit fetch
cargo build --target=wasm32-wasip2 --release
```

## Interfacing
To use this WebAssembly component in your own WebAssembly component, simply import this interface into your component like so:
```WIT
world your-world {
    import wasco-dev:glyphic-api@1.0.0/glyphic-api;

    // Your world definition.
}
```
