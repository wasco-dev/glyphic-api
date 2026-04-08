# Example: Petstore API to WASM Component

This example demonstrates the conversion of the Petstore OpenAPI specification
to a WebAssembly component.

## Input: OpenAPI Spec

See `../example-petstore.yaml` for the full specification.

Key endpoints:

- `GET /pets` - List all pets
- `GET /pets/{petId}` - Get a pet by ID
- `POST /pets` - Create a pet
- `PUT /pets/{petId}` - Update a pet
- `DELETE /pets/{petId}` - Delete a pet

## Output: WASM Component

### WIT Interface (`wit/world.wit`)

Each API endpoint is converted to a WIT function:

```wit
interface petstore {
    list-pets: func() -> string;
    get-pet-by-id: func(pet-id: string) -> string;
    create-pet: func(body: string) -> string;
    update-pet: func(pet-id: string, body: string) -> string;
    delete-pet: func(pet-id: string) -> string;
}
```

#### Naming Conversion

| OpenAPI                | WIT Function    | Rust Function   |
| ---------------------- | --------------- | --------------- |
| `GET /pets`            | `list-pets`     | `list_pets`     |
| `GET /pets/{petId}`    | `get-pet-by-id` | `get_pet_by_id` |
| `POST /pets`           | `create-pet`    | `create_pet`    |
| `PUT /pets/{petId}`    | `update-pet`    | `update_pet`    |
| `DELETE /pets/{petId}` | `delete-pet`    | `delete_pet`    |

### Rust Implementation (`src/lib.rs`)

The implementation follows the standard pattern:

- Uses `wit_bindgen` to generate bindings
- Implements the `Guest` trait
- Includes helpful TODO comments with implementation hints
- Documents expected request/response formats

### Project Files

- `Cargo.toml` - Standard cdylib configuration
- `function.json` - Empty metadata file
- `Justfile` - Would be copied from workspace-justfile

## Type Mappings

The petstore example demonstrates:

### Path Parameters

```
GET /pets/{petId}
OpenAPI: petId (string, path parameter)
WIT:     pet-id: string
Rust:    _pet_id: String
```

### Request Bodies

```
POST /pets
OpenAPI: Pet object in request body
WIT:     body: string (JSON)
Rust:    _body: String (parse as JSON)
```

### Response Bodies

```
All responses return JSON strings:
- Single object: {"id": "...", "name": "...", "tag": "..."}
- Array: [{"id": "...", ...}, ...]
- Empty: ""
```

## Building and Testing

To build this component:

```bash
cd functions/petstore/1.0
just build
```

This would generate: `petstore.wasm`

To test:

```bash
deno task test
```

## Implementation Notes

The generated code includes TODO markers indicating where to implement:

1. HTTP client calls to the actual API
2. JSON parsing/serialization
3. Error handling
4. Response transformation

Next steps:

1. Add HTTP client dependency (e.g., `wstd` for HTTP)
2. Implement each function to call the actual API
3. Handle errors appropriately
4. Add proper JSON serialization/deserialization
5. Write comprehensive tests

## Usage Example

Once implemented, the component can be used like:

```typescript
import { compileComponent } from "./compile-component.ts";

const wasmPath = "./functions/petstore/1.0/petstore.wasm";

const {
  petstore: { listPets, getPetById, createPet },
} = await compileComponent(wasmPath);

// List all pets
const pets = listPets();
console.log(JSON.parse(pets));

// Get a specific pet
const pet = getPetById("123");
console.log(JSON.parse(pet));

// Create a new pet
const newPet = createPet(
  JSON.stringify({
    name: "Fluffy",
    tag: "cat",
  }),
);
console.log(JSON.parse(newPet));
```
