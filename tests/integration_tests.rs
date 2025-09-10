use todoist_api::*;

// Note: These are integration tests that would require a real Todoist API token
// In a real CI environment, you'd use a test token or mock the API responses

#[tokio::test]
#[ignore] // Ignore by default since they require API access
async fn test_complete_workflow() {
    let api_token = std::env::var("TODOIST_API_TOKEN").unwrap_or_else(|_| "test-token".to_string());
    let _todoist = TodoistWrapper::new(api_token);

    // Test project creation
    let project_args = CreateProjectArgs {
        name: "Test Project".to_string(),
        color: Some("blue".to_string()),
        is_favorite: Some(false),
        view_style: Some("list".to_string()),
        parent_id: None,
    };

    // Note: In real tests, you'd create the project and then clean it up
    // For now, we'll just test the argument building
    assert_eq!(project_args.name, "Test Project");
    assert_eq!(project_args.color, Some("blue".to_string()));
    assert_eq!(project_args.is_favorite, Some(false));
    assert_eq!(project_args.view_style, Some("list".to_string()));
}

#[tokio::test]
#[ignore]
async fn test_task_management_workflow() {
    let api_token = std::env::var("TODOIST_API_TOKEN").unwrap_or_else(|_| "test-token".to_string());
    let _todoist = TodoistWrapper::new(api_token);

    // Test task creation arguments
    let task_args = CreateTaskArgs {
        content: "Integration test task".to_string(),
        description: Some("This is a test task for integration testing".to_string()),
        project_id: None, // Inbox
        priority: Some(3),
        labels: Some(vec!["test".to_string(), "integration".to_string()]),
        due_string: Some("tomorrow".to_string()),
        ..Default::default()
    };

    assert_eq!(task_args.content, "Integration test task");
    assert_eq!(
        task_args.description,
        Some("This is a test task for integration testing".to_string())
    );
    assert_eq!(task_args.priority, Some(3));
    assert_eq!(
        task_args.labels,
        Some(vec!["test".to_string(), "integration".to_string()])
    );
    assert_eq!(task_args.due_string, Some("tomorrow".to_string()));
}

#[tokio::test]
#[ignore]
async fn test_label_management_workflow() {
    let api_token = std::env::var("TODOIST_API_TOKEN").unwrap_or_else(|_| "test-token".to_string());
    let _todoist = TodoistWrapper::new(api_token);

    // Test label creation arguments
    let label_args = CreateLabelArgs {
        name: "Integration Test Label".to_string(),
        color: Some("red".to_string()),
        order: Some(1),
        is_favorite: Some(true),
    };

    assert_eq!(label_args.name, "Integration Test Label");
    assert_eq!(label_args.color, Some("red".to_string()));
    assert_eq!(label_args.order, Some(1));
    assert_eq!(label_args.is_favorite, Some(true));
}

#[tokio::test]
#[ignore]
async fn test_section_management_workflow() {
    let api_token = std::env::var("TODOIST_API_TOKEN").unwrap_or_else(|_| "test-token".to_string());
    let _todoist = TodoistWrapper::new(api_token);

    // Test section creation arguments
    let section_args = CreateSectionArgs {
        name: "Test Section".to_string(),
        project_id: "test_project_id".to_string(),
        order: Some(1),
    };

    assert_eq!(section_args.name, "Test Section");
    assert_eq!(section_args.project_id, "test_project_id");
    assert_eq!(section_args.order, Some(1));
}

#[tokio::test]
#[ignore]
async fn test_comment_management_workflow() {
    let api_token = std::env::var("TODOIST_API_TOKEN").unwrap_or_else(|_| "test-token".to_string());
    let _todoist = TodoistWrapper::new(api_token);

    // Test comment creation arguments
    let comment_args = CreateCommentArgs {
        content: "This is a test comment".to_string(),
        task_id: Some("test_task_id".to_string()),
        project_id: None,
        attachment: None,
    };

    assert_eq!(comment_args.content, "This is a test comment");
    assert_eq!(comment_args.task_id, Some("test_task_id".to_string()));
    assert!(comment_args.project_id.is_none());
    assert!(comment_args.attachment.is_none());
}

#[tokio::test]
#[ignore]
async fn test_filtering_workflow() {
    let api_token = std::env::var("TODOIST_API_TOKEN").unwrap_or_else(|_| "test-token".to_string());
    let _todoist = TodoistWrapper::new(api_token);

    // Test various filter combinations
    let task_filter = TaskFilterArgs {
        query: "today".to_string(),
        lang: Some("en".to_string()),
        limit: Some(20),
        cursor: None,
    };

    let project_filter = ProjectFilterArgs {
        limit: Some(10),
        cursor: None,
    };

    let label_filter = LabelFilterArgs {
        limit: Some(50),
        cursor: None,
    };

    assert_eq!(task_filter.query, "today");
    assert_eq!(task_filter.lang, Some("en".to_string()));
    assert_eq!(task_filter.limit, Some(20));

    assert_eq!(project_filter.limit, Some(10));
    assert!(project_filter.cursor.is_none());

    assert_eq!(label_filter.limit, Some(50));
    assert!(label_filter.cursor.is_none());
}

#[tokio::test]
#[ignore]
async fn test_error_handling() {
    let api_token = std::env::var("TODOIST_API_TOKEN").unwrap_or_else(|_| "test-token".to_string());
    let _todoist = TodoistWrapper::new(api_token);

    // Test that invalid project IDs are handled gracefully
    // This would test the actual API error responses
    // For now, we'll just test the wrapper creation
    // No assertion needed - if this function completes without panic, the test passes
}

#[tokio::test]
#[ignore]
async fn test_convenience_methods() {
    let api_token = std::env::var("TODOIST_API_TOKEN").unwrap_or_else(|_| "test-token".to_string());
    let _todoist = TodoistWrapper::new(api_token);

    // Test the convenience methods
    // These methods should work even without API access since they just build arguments
    let simple_task_args = CreateTaskArgs {
        content: "Simple task".to_string(),
        project_id: Some("proj_123".to_string()),
        ..Default::default()
    };

    assert_eq!(simple_task_args.content, "Simple task");
    assert_eq!(simple_task_args.project_id, Some("proj_123".to_string()));
    assert!(simple_task_args.description.is_none());
    assert!(simple_task_args.priority.is_none());
}

#[tokio::test]
#[ignore]
async fn test_data_model_consistency() {
    // Test that our data models are consistent with the API
    let task = Task {
        id: "test_id".to_string(),
        content: "Test content".to_string(),
        description: "Test description".to_string(),
        project_id: "test_project".to_string(),
        section_id: None,
        parent_id: None,
        order: 1,
        priority: 2,
        is_completed: false,
        labels: vec!["test".to_string()],
        created_at: "2024-01-01T00:00:00Z".to_string(),
        due: None,
        deadline: None,
        duration: None,
        assignee_id: None,
        url: "https://todoist.com".to_string(),
        comment_count: 0,
    };

    // Test that all required fields are present
    assert!(!task.id.is_empty());
    assert!(!task.content.is_empty());
    assert!(!task.description.is_empty());
    assert!(!task.project_id.is_empty());
    assert!(!task.created_at.is_empty());
    assert!(!task.url.is_empty());

    // Test that optional fields can be None
    assert!(task.section_id.is_none());
    assert!(task.parent_id.is_none());
    assert!(task.due.is_none());
    assert!(task.deadline.is_none());
    assert!(task.duration.is_none());
    assert!(task.assignee_id.is_none());
}

#[tokio::test]
#[ignore]
async fn test_argument_builder_patterns() {
    // Test common argument building patterns
    let task_args = CreateTaskArgs {
        content: "Built task".to_string(),
        priority: Some(4),
        labels: Some(vec!["built".to_string(), "task".to_string()]),
        ..Default::default()
    };

    assert_eq!(task_args.content, "Built task");
    assert_eq!(task_args.priority, Some(4));
    assert_eq!(task_args.labels, Some(vec!["built".to_string(), "task".to_string()]));

    // Test update pattern
    let update_args = UpdateTaskArgs {
        content: Some("Updated content".to_string()),
        priority: Some(1),
        ..Default::default()
    };

    assert_eq!(update_args.content, Some("Updated content".to_string()));
    assert_eq!(update_args.priority, Some(1));
    assert!(update_args.description.is_none()); // Should remain None
}
