# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0] - soon (tm)

### BREAKING CHANGES

This release migrates from the deprecated Todoist REST API v2 to the new Unified API v1. This is a major version bump due to significant breaking changes in response types, data models, and API base URL.

#### API Migration
- **Base URL changed**: `https://api.todoist.com/rest/v2` → `https://api.todoist.com/api/v1`
- All list-returning methods now return `PaginatedResponse<T>` instead of `Vec<T>`
- Pagination uses cursor-based approach with `next_cursor` field

#### Task Model Breaking Changes
- **Renamed fields**:
  - `is_completed` → `checked`
  - `created_at` → `added_at`
  - `comment_count` → `note_count`
- **Removed fields**: `assignee_id`, `url`
- **New required fields**: `user_id`, `child_order`, `day_order`, `is_collapsed`, `is_deleted`
- **New optional fields**: `added_by_uid`, `assigned_by_uid`, `responsible_uid`, `updated_at`

#### Section Model Breaking Changes
- **Renamed fields**: `order` → `section_order`
- **New required fields**: `user_id`, `added_at`, `is_archived`, `is_deleted`, `is_collapsed`
- **New optional fields**: `updated_at`, `archived_at`

#### CreateTaskArgs & UpdateTaskArgs Breaking Changes
- **Removed field**: `assignee_id` (use `responsible_uid` in the Task response model)

### Added
- `PaginatedResponse<T>` wrapper for all list endpoints
- `CompletedTasksFilterArgs` for filtering completed tasks by date ranges
- `get_completed_tasks_by_completion_date()` - Get completed tasks within date range (up to 3 months)
- `get_completed_tasks_by_due_date()` - Get completed tasks by due date (up to 6 weeks)
- Cursor-based pagination support for all list endpoints

### Changed
- All methods returning lists now return `PaginatedResponse<T>`:
  - `get_tasks()` → `TodoistResult<PaginatedResponse<Task>>`
  - `get_tasks_for_project()` → `TodoistResult<PaginatedResponse<Task>>`
  - `get_tasks_by_filter()` → `TodoistResult<PaginatedResponse<Task>>`
  - `get_sections()` → `TodoistResult<PaginatedResponse<Section>>`
  - `get_sections_filtered()` → `TodoistResult<PaginatedResponse<Section>>`
  - `get_projects()` → `TodoistResult<PaginatedResponse<Project>>`
  - `get_projects_filtered()` → `TodoistResult<PaginatedResponse<Project>>`
  - `get_labels()` → `TodoistResult<PaginatedResponse<Label>>`
  - `get_labels_filtered()` → `TodoistResult<PaginatedResponse<Label>>`
  - `get_comments()` → `TodoistResult<PaginatedResponse<Comment>>`
  - `get_comments_filtered()` → `TodoistResult<PaginatedResponse<Comment>>`

### Migration Guide

#### 1. Update Response Handling

**Before (v0.3.x)**:
```rust
let tasks: Vec<Task> = todoist.get_tasks().await?;
for task in tasks {
    println!("Task: {}", task.content);
}
```

**After (v1.0.0)**:
```rust
let response: PaginatedResponse<Task> = todoist.get_tasks().await?;
for task in response.results {
    println!("Task: {}", task.content);
}

// Handle pagination if needed
if let Some(cursor) = response.next_cursor {
    let next_page = todoist.get_tasks_by_filter(&TaskFilterArgs {
        cursor: Some(cursor),
        ..Default::default()
    }).await?;
}
```

#### 2. Update Task Field Access

**Before (v0.3.x)**:
```rust
if task.is_completed {
    println!("Created at: {}", task.created_at);
    println!("Comments: {}", task.comment_count);
}
```

**After (v1.0.0)**:
```rust
if task.checked {
    println!("Added at: {}", task.added_at);
    println!("Notes: {}", task.note_count);
}
```

#### 3. Update Section Field Access

**Before (v0.3.x)**:
```rust
println!("Section order: {}", section.order);
```

**After (v1.0.0)**:
```rust
println!("Section order: {}", section.section_order);
```

#### 4. Remove assignee_id from Task Creation

**Before (v0.3.x)**:
```rust
let args = CreateTaskArgs {
    content: "Task".to_string(),
    assignee_id: Some("user_id".to_string()),
    ..Default::default()
};
```

**After (v1.0.0)**:
```rust
let args = CreateTaskArgs {
    content: "Task".to_string(),
    // assignee_id removed - check task.responsible_uid in response
    ..Default::default()
};
```

#### 5. Access New Completed Tasks Endpoints

**New in v1.0.0**:
```rust
use todoist_api::models::CompletedTasksFilterArgs;

let args = CompletedTasksFilterArgs {
    since: Some("2025-01-01T00:00:00Z".to_string()),
    until: Some("2025-01-31T23:59:59Z".to_string()),
    project_id: Some("project_id".to_string()),
    ..Default::default()
};

// Get by completion date (up to 3 months range)
let response = todoist.get_completed_tasks_by_completion_date(&args).await?;

// Get by due date (up to 6 weeks range)
let response = todoist.get_completed_tasks_by_due_date(&args).await?;
```

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
