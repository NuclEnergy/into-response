# Changelog

All notable changes to this project will be documented in this file.

# 0.3.1 (March 21, 2025)

- Update the license with the correct information
- Update the .gitattributes file

# 0.3.0 (March 7, 2025)

- Add custom status code support for the `IntoResponse` derive macro
- Implement parsing of custom status codes via `#[into_response(status = X)]` attribute
- Add utility module for parsing status code attributes
- Update derive macro to support optional custom HTTP status codes
- Include comprehensive test cases for default and custom status codes
- Add axum as a dev-dependency for testing

# 0.2.1 (December 20, 2024)

- Update the dependency versions of the documentation in README.md

# 0.2.0 (December 20, 2024)

- Add support for generic types in `IntoResponse` derive macro
- Add serialization constraints for `IntoResponse` derive macro

# 0.1.0 (December 20, 2024)

- Initial crate release.
- Add `IntoResponse` derive macro.
