use serde::{Deserialize, Serialize};
use std::fmt;

/// Todoist Task model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: String,
    pub content: String,
    pub description: String,
    pub project_id: String,
    pub section_id: Option<String>,
    pub parent_id: Option<String>,
    pub order: i32,
    pub priority: i32,
    pub is_completed: bool,
    pub labels: Vec<String>,
    pub created_at: String,
    pub due: Option<Due>,
    pub deadline: Option<Deadline>,
    pub duration: Option<Duration>,
    pub assignee_id: Option<String>,
    pub url: String,
    pub comment_count: i32,
}

/// Todoist Project model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub comment_count: i32,
    pub order: i32,
    pub color: String,
    pub is_shared: bool,
    pub is_favorite: bool,
    pub is_inbox_project: bool,
    pub is_team_inbox: bool,
    pub view_style: String,
    pub url: String,
    pub parent_id: Option<String>,
}

/// Todoist Label model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Label {
    pub id: String,
    pub name: String,
    pub color: String,
    pub order: i32,
    pub is_favorite: bool,
}

/// Todoist Section model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Section {
    pub id: String,
    pub name: String,
    pub project_id: String,
    pub order: i32,
}

/// Todoist Comment model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Comment {
    pub id: String,
    pub content: String,
    pub posted_at: String,
    pub attachment: Option<Attachment>,
    pub project_id: Option<String>,
    pub task_id: Option<String>,
}

/// Todoist Attachment model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attachment {
    pub file_name: String,
    pub file_type: String,
    pub file_url: String,
    pub resource_type: String,
}

/// Todoist User model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub is_premium: bool,
    pub is_business_account: bool,
}

/// Todoist Due date model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Due {
    pub string: String,
    pub date: String,
    pub is_recurring: bool,
    pub datetime: Option<String>,
    pub timezone: Option<String>,
}

/// Todoist Deadline model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Deadline {
    pub date: String,
}

/// Todoist Duration model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Duration {
    pub amount: i32,
    pub unit: String, // "minute", "hour", "day"
}

/// Task creation arguments
#[derive(Debug, Serialize, Default)]
pub struct CreateTaskArgs {
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_datetime: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_lang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline_lang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_unit: Option<String>,
}

/// Task update arguments
#[derive(Debug, Serialize, Default)]
pub struct UpdateTaskArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_datetime: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_lang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline_lang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_unit: Option<String>,
}

impl UpdateTaskArgs {
    /// Check if any fields are set for updating
    pub fn has_updates(&self) -> bool {
        self.content.is_some() ||
        self.description.is_some() ||
        self.priority.is_some() ||
        self.labels.is_some() ||
        self.due_string.is_some() ||
        self.due_date.is_some() ||
        self.due_datetime.is_some() ||
        self.due_lang.is_some() ||
        self.deadline_date.is_some() ||
        self.deadline_lang.is_some() ||
        self.assignee_id.is_some() ||
        self.duration.is_some() ||
        self.duration_unit.is_some()
    }
}

/// Project creation arguments
#[derive(Debug, Serialize, Default)]
pub struct CreateProjectArgs {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_favorite: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_style: Option<String>,
}

/// Project update arguments
#[derive(Debug, Serialize, Default)]
pub struct UpdateProjectArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_favorite: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_style: Option<String>,
}

impl UpdateProjectArgs {
    /// Check if any fields are set for updating
    pub fn has_updates(&self) -> bool {
        self.name.is_some() ||
        self.color.is_some() ||
        self.is_favorite.is_some() ||
        self.view_style.is_some()
    }
}

/// Label creation arguments
#[derive(Debug, Serialize, Default)]
pub struct CreateLabelArgs {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_favorite: Option<bool>,
}

/// Label update arguments
#[derive(Debug, Serialize, Default)]
pub struct UpdateLabelArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_favorite: Option<bool>,
}

impl UpdateLabelArgs {
    /// Check if any fields are set for updating
    pub fn has_updates(&self) -> bool {
        self.name.is_some() ||
        self.color.is_some() ||
        self.order.is_some() ||
        self.is_favorite.is_some()
    }
}

/// Section creation arguments
#[derive(Debug, Serialize, Default)]
pub struct CreateSectionArgs {
    pub name: String,
    pub project_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
}

/// Section update arguments
#[derive(Debug, Serialize, Default)]
pub struct UpdateSectionArgs {
    pub name: String,
}

/// Comment creation arguments
#[derive(Debug, Serialize, Default)]
pub struct CreateCommentArgs {
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<Attachment>,
}

/// Comment update arguments
#[derive(Debug, Serialize, Default)]
pub struct UpdateCommentArgs {
    pub content: String,
}

impl UpdateCommentArgs {
    /// Check if any fields are set for updating
    /// Note: UpdateCommentArgs only has required fields, so this always returns true when instantiated
    pub fn has_updates(&self) -> bool {
        !self.content.is_empty()
    }
}

/// Task filter arguments
#[derive(Debug, Serialize)]
pub struct TaskFilterArgs {
    pub query: String,
    pub lang: Option<String>,
    pub limit: Option<i32>,
    pub cursor: Option<String>,
}

/// Project filter arguments
#[derive(Debug, Serialize)]
pub struct ProjectFilterArgs {
    pub limit: Option<i32>,
    pub cursor: Option<String>,
}

/// Label filter arguments
#[derive(Debug, Serialize)]
pub struct LabelFilterArgs {
    pub limit: Option<i32>,
    pub cursor: Option<String>,
}

/// Section filter arguments
#[derive(Debug, Serialize)]
pub struct SectionFilterArgs {
    pub project_id: Option<String>,
    pub limit: Option<i32>,
    pub cursor: Option<String>,
}

/// Comment filter arguments
#[derive(Debug, Serialize)]
pub struct CommentFilterArgs {
    pub task_id: Option<String>,
    pub project_id: Option<String>,
    pub limit: Option<i32>,
    pub cursor: Option<String>,
}

/// Represents different types of errors that can occur when interacting with the Todoist API
#[derive(Debug, Clone)]
pub enum TodoistError {
    /// Rate limiting error (HTTP 429)
    RateLimited { retry_after: Option<u64>, message: String },
    /// Authentication error (HTTP 401)
    AuthenticationError { message: String },
    /// Authorization error (HTTP 403)
    AuthorizationError { message: String },
    /// Resource not found (HTTP 404)
    NotFound {
        resource_type: String,
        resource_id: Option<String>,
        message: String,
    },
    /// Validation error (HTTP 400)
    ValidationError { field: Option<String>, message: String },
    /// Server error (HTTP 5xx)
    ServerError { status_code: u16, message: String },
    /// Network/connection error
    NetworkError { message: String },
    /// JSON parsing error
    ParseError { message: String },
    /// Unexpected empty response (when API returns nothing)
    EmptyResponse { endpoint: String, message: String },
    /// Generic error for other cases
    Generic { status_code: Option<u16>, message: String },
}

impl TodoistError {
    /// Check if this is a rate limiting error
    pub fn is_rate_limited(&self) -> bool {
        matches!(self, TodoistError::RateLimited { .. })
    }

    /// Check if this is an authentication error
    pub fn is_authentication_error(&self) -> bool {
        matches!(self, TodoistError::AuthenticationError { .. })
    }

    /// Check if this is an authorization error
    pub fn is_authorization_error(&self) -> bool {
        matches!(self, TodoistError::AuthorizationError { .. })
    }

    /// Check if this is a not found error
    pub fn is_not_found(&self) -> bool {
        matches!(self, TodoistError::NotFound { .. })
    }

    /// Check if this is a validation error
    pub fn is_validation_error(&self) -> bool {
        matches!(self, TodoistError::ValidationError { .. })
    }

    /// Check if this is a server error
    pub fn is_server_error(&self) -> bool {
        matches!(self, TodoistError::ServerError { .. })
    }

    /// Check if this is a network error
    pub fn is_network_error(&self) -> bool {
        matches!(self, TodoistError::NetworkError { .. })
    }

    /// Check if this is an empty response error
    pub fn is_empty_response(&self) -> bool {
        matches!(self, TodoistError::EmptyResponse { .. })
    }

    /// Get the retry after value for rate limiting errors
    pub fn retry_after(&self) -> Option<u64> {
        match self {
            TodoistError::RateLimited { retry_after, .. } => *retry_after,
            _ => None,
        }
    }

    /// Get the HTTP status code if available
    pub fn status_code(&self) -> Option<u16> {
        match self {
            TodoistError::ServerError { status_code, .. } => Some(*status_code),
            TodoistError::Generic { status_code, .. } => *status_code,
            _ => None,
        }
    }
}

impl fmt::Display for TodoistError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TodoistError::RateLimited { retry_after, message } => {
                if let Some(seconds) = retry_after {
                    write!(f, "Rate limited: {} (retry after {} seconds)", message, seconds)
                } else {
                    write!(f, "Rate limited: {}", message)
                }
            }
            TodoistError::AuthenticationError { message } => {
                write!(f, "Authentication error: {}", message)
            }
            TodoistError::AuthorizationError { message } => {
                write!(f, "Authorization error: {}", message)
            }
            TodoistError::NotFound {
                resource_type,
                resource_id,
                message,
            } => {
                if let Some(id) = resource_id {
                    write!(f, "{} not found (ID: {}): {}", resource_type, id, message)
                } else {
                    write!(f, "{} not found: {}", resource_type, message)
                }
            }
            TodoistError::ValidationError { field, message } => {
                if let Some(field_name) = field {
                    write!(f, "Validation error for field '{}': {}", field_name, message)
                } else {
                    write!(f, "Validation error: {}", message)
                }
            }
            TodoistError::ServerError { status_code, message } => {
                write!(f, "Server error ({}): {}", status_code, message)
            }
            TodoistError::NetworkError { message } => {
                write!(f, "Network error: {}", message)
            }
            TodoistError::ParseError { message } => {
                write!(f, "Parse error: {}", message)
            }
            TodoistError::EmptyResponse { endpoint, message } => {
                write!(f, "Empty response from {}: {}", endpoint, message)
            }
            TodoistError::Generic { status_code, message } => {
                if let Some(code) = status_code {
                    write!(f, "Error ({}): {}", code, message)
                } else {
                    write!(f, "Error: {}", message)
                }
            }
        }
    }
}

impl std::error::Error for TodoistError {}

impl From<reqwest::Error> for TodoistError {
    fn from(err: reqwest::Error) -> Self {
        TodoistError::NetworkError {
            message: format!("Request failed: {}", err),
        }
    }
}

impl From<serde_json::Error> for TodoistError {
    fn from(err: serde_json::Error) -> Self {
        TodoistError::ParseError {
            message: format!("JSON error: {}", err),
        }
    }
}

/// Result type for Todoist API operations
pub type TodoistResult<T> = Result<T, TodoistError>;

/// Helper function to create a rate limiting error
pub fn rate_limited_error(message: impl Into<String>, retry_after: Option<u64>) -> TodoistError {
    TodoistError::RateLimited {
        retry_after,
        message: message.into(),
    }
}

/// Helper function to create an empty response error
pub fn empty_response_error(endpoint: impl Into<String>, message: impl Into<String>) -> TodoistError {
    TodoistError::EmptyResponse {
        endpoint: endpoint.into(),
        message: message.into(),
    }
}

/// Helper function to create a not found error
pub fn not_found_error(
    resource_type: impl Into<String>,
    resource_id: Option<impl Into<String>>,
    message: impl Into<String>,
) -> TodoistError {
    TodoistError::NotFound {
        resource_type: resource_type.into(),
        resource_id: resource_id.map(|id| id.into()),
        message: message.into(),
    }
}
