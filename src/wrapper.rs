use reqwest::Client;
use serde_json::Value;

use crate::models::*;

const TODOIST_API_BASE: &str = "https://api.todoist.com/rest/v2";

/// A comprehensive wrapper around the Todoist REST API v2
#[derive(Clone)]
pub struct TodoistWrapper {
    client: Client,
    api_token: String,
}

impl TodoistWrapper {
    /// Create a new Todoist client
    #[must_use]
    pub fn new(api_token: String) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .unwrap_or_else(|_| Client::new());
        Self { client, api_token }
    }

    /// Helper method for making GET requests
    async fn make_get_request<T>(&self, endpoint: &str) -> TodoistResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        self.make_get_request_with_params(endpoint, &[] as &[(&str, String)])
            .await
    }

    /// Helper method for making GET requests with query parameters
    async fn make_get_request_with_params<T>(&self, endpoint: &str, query_params: &[(&str, String)]) -> TodoistResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = format!("{TODOIST_API_BASE}{endpoint}");

        let response = self
            .client
            .get(&url)
            .query(query_params)
            .bearer_auth(&self.api_token)
            .send()
            .await
            .map_err(|e| TodoistError::NetworkError {
                message: format!("Failed to send request: {}", e),
            })?;

        self.handle_response("GET", endpoint, response).await
    }

    /// Helper method for making POST requests
    async fn make_post_request<T>(&self, endpoint: &str, body: Option<&Value>) -> TodoistResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = format!("{TODOIST_API_BASE}{endpoint}");
        let mut request = self
            .client
            .post(&url)
            .bearer_auth(&self.api_token)
            .header("Content-Type", "application/json");

        if let Some(body_value) = body {
            request = request.json(body_value);
        }

        let response = request.send().await.map_err(|e| TodoistError::NetworkError {
            message: format!("Failed to send request: {}", e),
        })?;

        self.handle_response("POST", endpoint, response).await
    }

    /// Helper method for making DELETE requests
    async fn make_delete_request<T>(&self, endpoint: &str) -> TodoistResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = format!("{TODOIST_API_BASE}{endpoint}");
        let response = self
            .client
            .delete(&url)
            .bearer_auth(&self.api_token)
            .send()
            .await
            .map_err(|e| TodoistError::NetworkError {
                message: format!("Failed to send request: {}", e),
            })?;

        self.handle_response("DELETE", endpoint, response).await
    }

    /// Helper method to handle HTTP responses and convert them to TodoistResult
    async fn handle_response<T>(
        &self,
        http_method: &str,
        endpoint: &str,
        response: reqwest::Response,
    ) -> TodoistResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let status = response.status();
        let headers = response.headers().clone();

        if status.is_success() {
            // Read response body
            let text = response.text().await.map_err(|e| TodoistError::NetworkError {
                message: format!("Failed to read response body: {}", e),
            })?;

            // For DELETE requests, empty responses are expected and valid
            if http_method == "DELETE" && text.trim().is_empty() {
                // Try to deserialize "null" for empty DELETE responses
                return serde_json::from_str::<T>("null").map_err(|e| TodoistError::ParseError {
                    message: format!("Failed to deserialize empty DELETE response: {}", e),
                });
            }

            // For POST requests to close/reopen tasks, empty responses or 204 are expected and valid
            if http_method == "POST" && (status.as_u16() == 204 || text.trim().is_empty()) {
                // Try to deserialize "null" for empty POST responses
                return serde_json::from_str::<T>("null").map_err(|e| TodoistError::ParseError {
                    message: format!("Failed to deserialize empty POST response: {}", e),
                });
            }

            // Handle empty responses for other methods
            if text.trim().is_empty() {
                return Err(empty_response_error(endpoint, "API returned empty response body"));
            }

            // Try to parse response
            serde_json::from_str::<T>(&text).map_err(|e| TodoistError::ParseError {
                message: format!("Failed to parse response: {}", e),
            })
        } else {
            // Handle different error status codes
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| format!("Unknown error occurred (HTTP {})", status));

            let error = match status.as_u16() {
                401 => TodoistError::AuthenticationError { message: error_text },
                403 => TodoistError::AuthorizationError { message: error_text },
                404 => TodoistError::NotFound {
                    resource_type: "Resource".to_string(),
                    resource_id: None,
                    message: error_text,
                },
                429 => {
                    let retry_after = headers
                        .get("Retry-After")
                        .and_then(|v| v.to_str().ok())
                        .and_then(|s| s.parse::<u64>().ok());
                    TodoistError::RateLimited {
                        retry_after,
                        message: error_text,
                    }
                }
                400 => TodoistError::ValidationError {
                    field: None,
                    message: error_text,
                },
                500..=599 => TodoistError::ServerError {
                    status_code: status.as_u16(),
                    message: error_text,
                },
                _ => TodoistError::Generic {
                    status_code: Some(status.as_u16()),
                    message: error_text,
                },
            };

            Err(error)
        }
    }

    // ===== PROJECT OPERATIONS =====

    /// Get all projects
    pub async fn get_projects(&self) -> TodoistResult<Vec<Project>> {
        self.make_get_request("/projects").await
    }

    /// Get projects with filtering and pagination
    pub async fn get_projects_filtered(&self, args: &ProjectFilterArgs) -> TodoistResult<Vec<Project>> {
        let mut query_params = Vec::new();

        if let Some(limit) = args.limit {
            query_params.push(("limit", limit.to_string()));
        }
        if let Some(cursor) = &args.cursor {
            query_params.push(("cursor", cursor.clone()));
        }

        self.make_get_request_with_params("/projects", &query_params).await
    }

    /// Get a specific project by ID
    pub async fn get_project(&self, project_id: &str) -> TodoistResult<Project> {
        self.make_get_request(&format!("/projects/{project_id}")).await
    }

    /// Create a new project
    pub async fn create_project(&self, args: &CreateProjectArgs) -> TodoistResult<Project> {
        let body_value = serde_json::to_value(args)?;
        self.make_post_request("/projects", Some(&body_value)).await
    }

    /// Update an existing project
    pub async fn update_project(&self, project_id: &str, args: &UpdateProjectArgs) -> TodoistResult<Project> {
        if !args.has_updates() {
            return Err(TodoistError::ValidationError {
                field: None,
                message: "No fields specified for update".to_string(),
            });
        }
        let body_value = serde_json::to_value(args)?;
        self.make_post_request(&format!("/projects/{project_id}"), Some(&body_value))
            .await
    }

    /// Delete a project
    pub async fn delete_project(&self, project_id: &str) -> TodoistResult<()> {
        self.make_delete_request(&format!("/projects/{project_id}")).await
    }

    // ===== TASK OPERATIONS =====

    /// Get all tasks
    pub async fn get_tasks(&self) -> TodoistResult<Vec<Task>> {
        self.make_get_request("/tasks").await
    }

    /// Get tasks for a specific project
    pub async fn get_tasks_for_project(&self, project_id: &str) -> TodoistResult<Vec<Task>> {
        self.make_get_request(&format!("/tasks?project_id={project_id}")).await
    }

    /// Get a specific task by ID
    pub async fn get_task(&self, task_id: &str) -> TodoistResult<Task> {
        self.make_get_request(&format!("/tasks/{task_id}")).await
    }

    /// Get tasks by filter query
    pub async fn get_tasks_by_filter(&self, args: &TaskFilterArgs) -> TodoistResult<Vec<Task>> {
        let mut query_params = vec![("query", args.query.clone())];

        if let Some(lang) = &args.lang {
            query_params.push(("lang", lang.clone()));
        }
        if let Some(limit) = args.limit {
            query_params.push(("limit", limit.to_string()));
        }
        if let Some(cursor) = &args.cursor {
            query_params.push(("cursor", cursor.clone()));
        }

        self.make_get_request_with_params("/tasks", &query_params).await
    }

    /// Create a new task
    pub async fn create_task(&self, args: &CreateTaskArgs) -> TodoistResult<Task> {
        let body_value = serde_json::to_value(args)?;
        self.make_post_request("/tasks", Some(&body_value)).await
    }

    /// Update an existing task
    pub async fn update_task(&self, task_id: &str, args: &UpdateTaskArgs) -> TodoistResult<Task> {
        if !args.has_updates() {
            return Err(TodoistError::ValidationError {
                field: None,
                message: "No fields specified for update".to_string(),
            });
        }
        let body_value = serde_json::to_value(args)?;
        self.make_post_request(&format!("/tasks/{task_id}"), Some(&body_value))
            .await
    }

    /// Complete a task
    pub async fn complete_task(&self, task_id: &str) -> TodoistResult<()> {
        self.make_post_request(&format!("/tasks/{task_id}/close"), None).await
    }

    /// Reopen a completed task
    pub async fn reopen_task(&self, task_id: &str) -> TodoistResult<()> {
        self.make_post_request(&format!("/tasks/{task_id}/reopen"), None).await
    }

    /// Delete a task
    pub async fn delete_task(&self, task_id: &str) -> TodoistResult<()> {
        self.make_delete_request(&format!("/tasks/{task_id}")).await
    }

    // ===== LABEL OPERATIONS =====

    /// Get all labels
    pub async fn get_labels(&self) -> TodoistResult<Vec<Label>> {
        self.make_get_request("/labels").await
    }

    /// Get labels with filtering and pagination
    pub async fn get_labels_filtered(&self, args: &LabelFilterArgs) -> TodoistResult<Vec<Label>> {
        let mut query_params = Vec::new();

        if let Some(limit) = args.limit {
            query_params.push(("limit", limit.to_string()));
        }
        if let Some(cursor) = &args.cursor {
            query_params.push(("cursor", cursor.clone()));
        }

        self.make_get_request_with_params("/labels", &query_params).await
    }

    /// Get a specific label by ID
    pub async fn get_label(&self, label_id: &str) -> TodoistResult<Label> {
        self.make_get_request(&format!("/labels/{label_id}")).await
    }

    /// Create a new label
    pub async fn create_label(&self, args: &CreateLabelArgs) -> TodoistResult<Label> {
        let body_value = serde_json::to_value(args)?;
        self.make_post_request("/labels", Some(&body_value)).await
    }

    /// Update an existing label
    pub async fn update_label(&self, label_id: &str, args: &UpdateLabelArgs) -> TodoistResult<Label> {
        if !args.has_updates() {
            return Err(TodoistError::ValidationError {
                field: None,
                message: "No fields specified for update".to_string(),
            });
        }
        let body_value = serde_json::to_value(args)?;
        self.make_post_request(&format!("/labels/{label_id}"), Some(&body_value))
            .await
    }

    /// Delete a label
    pub async fn delete_label(&self, label_id: &str) -> TodoistResult<()> {
        self.make_delete_request(&format!("/labels/{label_id}")).await
    }

    // ===== SECTION OPERATIONS =====

    /// Get all sections
    pub async fn get_sections(&self) -> TodoistResult<Vec<Section>> {
        self.make_get_request("/sections").await
    }

    /// Get sections with filtering and pagination
    pub async fn get_sections_filtered(&self, args: &SectionFilterArgs) -> TodoistResult<Vec<Section>> {
        let mut query_params = Vec::new();

        if let Some(project_id) = &args.project_id {
            query_params.push(("project_id", project_id.clone()));
        }
        if let Some(limit) = args.limit {
            query_params.push(("limit", limit.to_string()));
        }
        if let Some(cursor) = &args.cursor {
            query_params.push(("cursor", cursor.clone()));
        }

        self.make_get_request_with_params("/sections", &query_params).await
    }

    /// Get a specific section by ID
    pub async fn get_section(&self, section_id: &str) -> TodoistResult<Section> {
        self.make_get_request(&format!("/sections/{section_id}")).await
    }

    /// Create a new section
    pub async fn create_section(&self, args: &CreateSectionArgs) -> TodoistResult<Section> {
        let body_value = serde_json::to_value(args)?;
        self.make_post_request("/sections", Some(&body_value)).await
    }

    /// Update an existing section
    pub async fn update_section(&self, section_id: &str, args: &UpdateSectionArgs) -> TodoistResult<Section> {
        let body_value = serde_json::to_value(args)?;
        self.make_post_request(&format!("/sections/{section_id}"), Some(&body_value))
            .await
    }

    /// Delete a section
    pub async fn delete_section(&self, section_id: &str) -> TodoistResult<()> {
        self.make_delete_request(&format!("/sections/{section_id}")).await
    }

    // ===== COMMENT OPERATIONS =====

    /// Get all comments
    pub async fn get_comments(&self) -> TodoistResult<Vec<Comment>> {
        self.make_get_request("/comments").await
    }

    /// Get comments with filtering and pagination
    pub async fn get_comments_filtered(&self, args: &CommentFilterArgs) -> TodoistResult<Vec<Comment>> {
        let mut query_params = Vec::new();

        if let Some(task_id) = &args.task_id {
            query_params.push(("task_id", task_id.clone()));
        }
        if let Some(project_id) = &args.project_id {
            query_params.push(("project_id", project_id.clone()));
        }
        if let Some(limit) = args.limit {
            query_params.push(("limit", limit.to_string()));
        }
        if let Some(cursor) = &args.cursor {
            query_params.push(("cursor", cursor.clone()));
        }

        self.make_get_request_with_params("/comments", &query_params).await
    }

    /// Get a specific comment by ID
    pub async fn get_comment(&self, comment_id: &str) -> TodoistResult<Comment> {
        self.make_get_request(&format!("/comments/{comment_id}")).await
    }

    /// Create a new comment
    pub async fn create_comment(&self, args: &CreateCommentArgs) -> TodoistResult<Comment> {
        let body_value = serde_json::to_value(args)?;
        self.make_post_request("/comments", Some(&body_value)).await
    }

    /// Update an existing comment
    pub async fn update_comment(&self, comment_id: &str, args: &UpdateCommentArgs) -> TodoistResult<Comment> {
        if !args.has_updates() {
            return Err(TodoistError::ValidationError {
                field: None,
                message: "No fields specified for update".to_string(),
            });
        }
        let body_value = serde_json::to_value(args)?;
        self.make_post_request(&format!("/comments/{comment_id}"), Some(&body_value))
            .await
    }

    /// Delete a comment
    pub async fn delete_comment(&self, comment_id: &str) -> TodoistResult<()> {
        self.make_delete_request(&format!("/comments/{comment_id}")).await
    }
}
