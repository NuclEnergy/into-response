# IntoResponse

`IntoResponse` is a Rust crate that provides utilities for deriving and implementing the `IntoResponse` trait for custom types. This crate simplifies the process of converting custom types into HTTP responses.

## Features

- Derive macro for `IntoResponse` trait
- Customizable response handling
- Support for common response types

## Usage

Add `into_response` to your `Cargo.toml`:

```toml
[dependencies]
into_response = "0.1"
```

### Example

```rust
use into_response::IntoResponse;

#[derive(IntoResponse)]
struct MyResponse {
  message: String,
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Acknowledgements

Special thanks to the Rust community for their contributions and support.
