//! # Todoist-api
//!
//! A Rust wrapper for the Todoist REST API v2.
//!
//! ## Features
//!
//! - Async/await support
//! - Full CRUD operations for tasks
//! - Project and label management
//! - Comprehensive error handling with specific error types
//! - Rate limiting detection and retry information
//! - Serde serialization/deserialization
//!
//! ## Example
//!
//! ```rust,no_run
//! use todoist_api::{TodoistWrapper, TodoistError, TodoistResult};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let todoist = TodoistWrapper::new("your-api-token".to_string());
//!     
//!     // Get all tasks with error handling
//!     match todoist.get_tasks().await {
//!         Ok(tasks) => println!("Found {} tasks", tasks.len()),
//!         Err(TodoistError::RateLimited { retry_after, message }) => {
//!             println!("Rate limited: {} (retry after {} seconds)", message, retry_after.unwrap_or(0));
//!         }
//!         Err(TodoistError::AuthenticationError { message }) => {
//!             println!("Authentication failed: {}", message);
//!         }
//!         Err(e) => println!("Other error: {}", e),
//!     }
//!     
//!     // Create a new task
//!     let new_task = todoist.create_simple_task("Buy groceries", None).await?;
//!     println!("Created task: {}", new_task.content);
//!     
//!     Ok(())
//! }
//! ```

pub mod models;
pub mod wrapper;

pub use models::*;
pub use wrapper::TodoistWrapper;

// Re-export commonly used types
pub use models::{TodoistError, TodoistResult};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todoist_wrapper_creation() {
        let _wrapper = TodoistWrapper::new("test-token".to_string());
        // Test that the wrapper was created successfully without panicking
        // We can't access private fields, so we just verify creation works
        assert!(true); // Placeholder assertion
    }

    #[test]
    fn test_library_exports() {
        // Test that all main types are properly exported
        let _task: Task = Task {
            id: "test".to_string(),
            content: "test".to_string(),
            description: "test".to_string(),
            project_id: "test".to_string(),
            section_id: None,
            parent_id: None,
            order: 1,
            priority: 1,
            is_completed: false,
            labels: vec![],
            created_at: "2024-01-01T00:00:00Z".to_string(),
            due: None,
            deadline: None,
            duration: None,
            assignee_id: None,
            url: "https://todoist.com".to_string(),
            comment_count: 0,
        };

        let _project: Project = Project {
            id: "test".to_string(),
            name: "test".to_string(),
            comment_count: 0,
            order: 1,
            color: "blue".to_string(),
            is_shared: false,
            is_favorite: false,
            is_inbox_project: false,
            is_team_inbox: false,
            view_style: "list".to_string(),
            url: "https://todoist.com".to_string(),
            parent_id: None,
        };

        let _label: Label = Label {
            id: "test".to_string(),
            name: "test".to_string(),
            color: "red".to_string(),
            order: 1,
            is_favorite: false,
        };

        let _wrapper: TodoistWrapper = TodoistWrapper::new("test".to_string());

        // Test that Result is properly exported
        let _result: TodoistResult<()> = Ok(());
    }

    #[test]
    fn test_argument_types() {
        // Test that argument types can be created and used
        let task_args = CreateTaskArgs {
            content: "Test task".to_string(),
            priority: Some(3),
            ..Default::default()
        };

        assert_eq!(task_args.content, "Test task");
        assert_eq!(task_args.priority, Some(3));

        let project_args = CreateProjectArgs {
            name: "Test project".to_string(),
            color: Some("blue".to_string()),
            ..Default::default()
        };

        assert_eq!(project_args.name, "Test project");
        assert_eq!(project_args.color, Some("blue".to_string()));
    }
}
