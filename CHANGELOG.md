# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2025-08-31

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

## [Unreleased]

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
