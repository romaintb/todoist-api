use todoist_api::{empty_response_error, not_found_error, rate_limited_error, TodoistError};

#[test]
fn test_rate_limited_error() {
    let error = rate_limited_error("Too many requests", Some(60));

    assert!(error.is_rate_limited());
    assert_eq!(error.retry_after(), Some(60));
    assert_eq!(
        error.to_string(),
        "Rate limited: Too many requests (retry after 60 seconds)"
    );
}

#[test]
fn test_empty_response_error() {
    let error = empty_response_error("/projects", "API returned empty response");

    assert!(error.is_empty_response());
    assert_eq!(
        error.to_string(),
        "Empty response from /projects: API returned empty response"
    );
}

#[test]
fn test_not_found_error() {
    let error = not_found_error("Project", Some("123"), "Project not found");

    assert!(error.is_not_found());
    assert_eq!(error.to_string(), "Project not found (ID: 123): Project not found");
}

#[test]
fn test_authentication_error() {
    let error = TodoistError::AuthenticationError {
        message: "Invalid token".to_string(),
    };

    assert!(error.is_authentication_error());
    assert_eq!(error.to_string(), "Authentication error: Invalid token");
}

#[test]
fn test_authorization_error() {
    let error = TodoistError::AuthorizationError {
        message: "Insufficient permissions".to_string(),
    };

    assert!(error.is_authorization_error());
    assert_eq!(error.to_string(), "Authorization error: Insufficient permissions");
}

#[test]
fn test_validation_error() {
    let error = TodoistError::ValidationError {
        field: Some("name".to_string()),
        message: "Name is required".to_string(),
    };

    assert!(error.is_validation_error());
    assert_eq!(error.to_string(), "Validation error for field 'name': Name is required");
}

#[test]
fn test_server_error() {
    let error = TodoistError::ServerError {
        status_code: 500,
        message: "Internal server error".to_string(),
    };

    assert!(error.is_server_error());
    assert_eq!(error.status_code(), Some(500));
    assert_eq!(error.to_string(), "Server error (500): Internal server error");
}

#[test]
fn test_network_error() {
    let error = TodoistError::NetworkError {
        message: "Connection timeout".to_string(),
    };

    assert!(error.is_network_error());
    assert_eq!(error.to_string(), "Network error: Connection timeout");
}

#[test]
fn test_parse_error() {
    let error = TodoistError::ParseError {
        message: "Invalid JSON".to_string(),
    };

    assert_eq!(error.to_string(), "Parse error: Invalid JSON");
}

#[test]
fn test_generic_error() {
    let error = TodoistError::Generic {
        status_code: Some(418),
        message: "I'm a teapot".to_string(),
    };

    assert_eq!(error.status_code(), Some(418));
    assert_eq!(error.to_string(), "Error (418): I'm a teapot");
}

#[test]
fn test_generic_error_no_status() {
    let error = TodoistError::Generic {
        status_code: None,
        message: "Unknown error".to_string(),
    };

    assert_eq!(error.status_code(), None);
    assert_eq!(error.to_string(), "Error: Unknown error");
}

#[test]
fn test_error_display_formatting() {
    let errors = vec![
        (
            rate_limited_error("Rate limit exceeded", Some(30)),
            "Rate limited: Rate limit exceeded (retry after 30 seconds)",
        ),
        (
            TodoistError::NotFound {
                resource_type: "Task".to_string(),
                resource_id: None,
                message: "Task not found".to_string(),
            },
            "Task not found: Task not found",
        ),
        (
            TodoistError::ValidationError {
                field: None,
                message: "Invalid input".to_string(),
            },
            "Validation error: Invalid input",
        ),
    ];

    for (error, expected) in errors {
        assert_eq!(error.to_string(), expected);
    }
}
