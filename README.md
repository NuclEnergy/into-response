# IntoResponse

IntoResponse is a Rust crate that provides utilities for deriving and implementing the `IntoResponse` trait for custom types. It offers a convenient way to convert your custom types into HTTP responses with minimal boilerplate.

## Features

- Derive macro for the `IntoResponse` trait
- Customizable response handling logic
- Support for common response types (e.g., JSON)
- Custom status code support using the `#[into_response(status = ...)]` attribute
- Automatic serialization constraints for generic types

## Usage

Add `into_response` to your `Cargo.toml`:

```toml
[dependencies]
into_response = "0.3"
```

### Examples

#### Default Response

```rust
use into_response::IntoResponse;

#[derive(IntoResponse)]
struct MyResponse {
    message: String,
}

fn main() {
    let response = MyResponse {
        message: "Hello, world!".to_string(),
    };
    // By default, the HTTP status is axum::http::StatusCode::OK.
    let response = response.into_response();
    assert_eq!(response.status(), axum::http::StatusCode::OK);
}
```

#### Custom Status Code

You can specify a custom HTTP status code using the `#[into_response(status = ...)]` attribute:

```rust
use into_response::IntoResponse;

#[derive(IntoResponse)]
#[into_response(status = 201)]
struct MyResponse {
    message: String,
}

fn main() {
    let response = MyResponse {
        message: "Created successfully".to_string(),
    };
    let response = response.into_response();
    // The HTTP status will be axum::http::StatusCode::CREATED.
    assert_eq!(response.status(), axum::http::StatusCode::CREATED);
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Acknowledgements

Special thanks to the Rust community for their contributions and support.
