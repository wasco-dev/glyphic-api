
# Glyphic API
This is a Rust WebAssembly component that provides the Glyphic API integration functionality.

## Functionality
- test-ping: Send a request to Glyphic with your API key to see if you have a valid connection.
- get-calls: Get all the public calls for your organization.
- get-call-by-id: Get information about a specific call.
- get-call-media-by-id: Get all media for a specific call.
- get-call-snippets-by-id: Get transcript snippets for a specific call.
- join-call: Join a call with a Glyphic bot by providing a meeting URL.
- list-call-tags: List all call tags for your organization.
- list-playbooks: List playbooks for your organization.
- get-playbook-by-id: Get information about a specific playbook.
- list-playbook-versions: List all versions of a specific playbook.
- get-playbook-version-by-id: Get a specific version of a playbook.

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
    import wasco-dev:glyphic-api@0.2.0/glyphic-api;

    // Your world definition.
}
```
