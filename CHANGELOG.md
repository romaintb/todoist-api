# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0-alpha.1] - 2026-01-09

### **Complete API Migration**

This is a major release with comprehensive changes due to Todoist deprecating REST API v2. The entire library has been rebuilt to wrap the new Todoist Unified API v1.

**⚠️ This release contains breaking changes across all APIs.** If you're upgrading from 0.x, expect to refactor your code. See the [migration guide](./docs/MIGRATION.md) for upgrade instructions.

### Breaking Changes
- **Complete rewrite**: All public APIs have changed to align with the new Unified API v1 structure
- **Model changes**: Data structures refactored to match Unified API response formats
- **Method signatures**: All wrapper methods adjusted for new API parameters and response types

### Added
- Full support for Todoist Unified API v1
- All models updated with new fields and structure from the new API
- Enhanced compatibility with latest Todoist features

### Removed
- All support for deprecated Todoist REST API v2

## [0.3.1] - 2025-12-23

### Fixed
- Fixed URL query string escaping in `get_tasks_for_project()` method

### Changed
- Minor improvements to README.md documentation

## [0.3.0] - 2025-09-10

### Breaking Changes
- BREAKING: Replaced generic `anyhow::Result` with `TodoistResult<T>` for better error handling
- BREAKING: All API methods now return specific `TodoistError` types instead of generic errors
- BREAKING: Removed dependency on `anyhow` crate

### Added
- Comprehensive error handling system with specific error types
- Rate limiting detection and retry information (HTTP 429)
- Empty response detection and meaningful error messages
- Automatic error conversion from network and parsing errors
- Helper functions for common error scenarios
- Extensive error handling examples and documentation
- Abstracted HTTP methods (GET, POST, DELETE) to reduce code duplication
- Enhanced README.md with project badges and improved documentation

### Changed
- Removed `url` field from `Section` model to simplify data structure
- Refactored HTTP client methods for better maintainability
- Improved error handling consistency across all endpoints
- Better response handling for empty DELETE responses (HTTP 204)
- Enhanced POST response handling for empty responses (HTTP 204)

### Fixed
- Fixed wrong error message wording in error handling
- Fixed linting errors throughout the codebase
- Improved code quality based on code review feedback

### Error Handling
- `TodoistError` enum with specific variants:
  - `RateLimited` - HTTP 429 responses with retry timing information
  - `AuthenticationError` - HTTP 401 authentication failures
  - `AuthorizationError` - HTTP 403 permission errors
  - `NotFound` - HTTP 404 resource not found errors
  - `ValidationError` - HTTP 400 request validation errors
  - `ServerError` - HTTP 5xx server-side errors
  - `NetworkError` - Connection and network-related errors
  - `ParseError` - JSON parsing and serialization errors
  - `EmptyResponse` - Unexpected empty API responses
  - `Generic` - Fallback for other error scenarios
- Helper methods for checking error types (`is_rate_limited()`, `is_authentication_error()`, etc.)
- Retry information extraction from rate limiting responses
- Status code access for HTTP-related errors
- Comprehensive test coverage with 12+ test cases for all error scenarios

## [0.2.0] - 2025-08-16

### Added
- Complete CRUD operations for all Todoist entities
- Advanced filtering and pagination support for all endpoints
- Comprehensive data models for all API entities
- Section management (create, read, update, delete)
- Comment system (create, read, update, delete)
- Enhanced project management with full lifecycle support
- Enhanced label management with filtering and pagination
- Advanced task creation and update with all available fields
- Support for task attachments and file handling
- User model for collaborator information
- Flexible argument types for all operations
- Backward compatibility methods for existing code

### Changed
- Renamed library from `todoist-rs` to `todoist-api`
- Enhanced task creation with comprehensive options
- Better API organization with logical grouping

### Removed
- `TaskDisplay` and `ProjectDisplay` types (moved to consumer responsibility)

## [0.1.0] - 2025-08-16

### Added
- Initial release of todoist-api
- `TodoistWrapper` struct for API interactions
- Full CRUD operations for tasks
- Project and label management
- Async/await support with Tokio
- Basic error handling with anyhow
- Serde serialization/deserialization
- Basic HTTP client with timeout handling
