use todoist_api::*;

#[test]
fn test_todoist_wrapper_creation() {
    let _wrapper = TodoistWrapper::new("test-token".to_string());
    // Test that the wrapper was created successfully without panicking
    // We can't access private fields, so we just verify creation works
    // No assertion needed - if this function completes without panic, the test passes
}

#[test]
fn test_create_task_args_builder() {
    let args = CreateTaskArgs {
        content: "Test task".to_string(),
        description: Some("Test description".to_string()),
        project_id: Some("proj_123".to_string()),
        priority: Some(4),
        labels: Some(vec!["important".to_string(), "work".to_string()]),
        due_string: Some("tomorrow".to_string()),
        ..Default::default()
    };

    assert_eq!(args.content, "Test task");
    assert_eq!(args.description, Some("Test description".to_string()));
    assert_eq!(args.project_id, Some("proj_123".to_string()));
    assert_eq!(args.priority, Some(4));
    assert_eq!(args.labels, Some(vec!["important".to_string(), "work".to_string()]));
    assert_eq!(args.due_string, Some("tomorrow".to_string()));
}

#[test]
fn test_update_task_args_builder() {
    let args = UpdateTaskArgs {
        content: Some("Updated content".to_string()),
        priority: Some(1),
        due_string: Some("next week".to_string()),
        labels: Some(vec!["urgent".to_string()]),
        ..Default::default()
    };

    assert_eq!(args.content, Some("Updated content".to_string()));
    assert_eq!(args.priority, Some(1));
    assert_eq!(args.due_string, Some("next week".to_string()));
    assert_eq!(args.labels, Some(vec!["urgent".to_string()]));
}

#[test]
fn test_create_project_args_builder() {
    let args = CreateProjectArgs {
        name: "New Project".to_string(),
        color: Some("red".to_string()),
        is_favorite: Some(true),
        view_style: Some("board".to_string()),
        parent_id: None,
    };

    assert_eq!(args.name, "New Project");
    assert_eq!(args.color, Some("red".to_string()));
    assert_eq!(args.is_favorite, Some(true));
    assert_eq!(args.view_style, Some("board".to_string()));
    assert!(args.parent_id.is_none());
}

#[test]
fn test_update_project_args_builder() {
    let args = UpdateProjectArgs {
        name: Some("Updated Project Name".to_string()),
        color: Some("blue".to_string()),
        is_favorite: Some(false),
        view_style: Some("list".to_string()),
    };

    assert_eq!(args.name, Some("Updated Project Name".to_string()));
    assert_eq!(args.color, Some("blue".to_string()));
    assert_eq!(args.is_favorite, Some(false));
    assert_eq!(args.view_style, Some("list".to_string()));
}

#[test]
fn test_create_label_args_builder() {
    let args = CreateLabelArgs {
        name: "New Label".to_string(),
        color: Some("green".to_string()),
        order: Some(5),
        is_favorite: Some(false),
    };

    assert_eq!(args.name, "New Label");
    assert_eq!(args.color, Some("green".to_string()));
    assert_eq!(args.order, Some(5));
    assert_eq!(args.is_favorite, Some(false));
}

#[test]
fn test_update_label_args_builder() {
    let args = UpdateLabelArgs {
        name: Some("Very Important".to_string()),
        color: Some("purple".to_string()),
        order: Some(10),
        is_favorite: Some(true),
    };

    assert_eq!(args.name, Some("Very Important".to_string()));
    assert_eq!(args.color, Some("purple".to_string()));
    assert_eq!(args.order, Some(10));
    assert_eq!(args.is_favorite, Some(true));
}

#[test]
fn test_create_section_args_builder() {
    let args = CreateSectionArgs {
        name: "New Section".to_string(),
        project_id: "proj_123".to_string(),
        order: Some(3),
    };

    assert_eq!(args.name, "New Section");
    assert_eq!(args.project_id, "proj_123");
    assert_eq!(args.order, Some(3));
}

#[test]
fn test_update_section_args_builder() {
    let args = UpdateSectionArgs {
        name: "Updated Section Name".to_string(),
    };

    assert_eq!(args.name, "Updated Section Name");
}

#[test]
fn test_create_comment_args_builder() {
    let attachment = Attachment {
        file_name: "document.pdf".to_string(),
        file_type: "application/pdf".to_string(),
        file_url: "https://example.com/doc.pdf".to_string(),
        resource_type: "file".to_string(),
    };

    let args = CreateCommentArgs {
        content: "New comment".to_string(),
        task_id: Some("task_123".to_string()),
        project_id: None,
        attachment: Some(attachment),
    };

    assert_eq!(args.content, "New comment");
    assert_eq!(args.task_id, Some("task_123".to_string()));
    assert!(args.project_id.is_none());
    assert!(args.attachment.is_some());
}

#[test]
fn test_update_comment_args_builder() {
    let args = UpdateCommentArgs {
        content: "Updated comment content".to_string(),
    };

    assert_eq!(args.content, "Updated comment content");
}

#[test]
fn test_task_filter_args_builder() {
    let args = TaskFilterArgs {
        query: "overdue".to_string(),
        lang: Some("en".to_string()),
        limit: Some(50),
        cursor: Some("cursor_123".to_string()),
    };

    assert_eq!(args.query, "overdue");
    assert_eq!(args.lang, Some("en".to_string()));
    assert_eq!(args.limit, Some(50));
    assert_eq!(args.cursor, Some("cursor_123".to_string()));
}

#[test]
fn test_project_filter_args_builder() {
    let args = ProjectFilterArgs {
        limit: Some(25),
        cursor: None,
    };

    assert_eq!(args.limit, Some(25));
    assert!(args.cursor.is_none());
}

#[test]
fn test_label_filter_args_builder() {
    let args = LabelFilterArgs {
        limit: Some(100),
        cursor: Some("label_cursor".to_string()),
    };

    assert_eq!(args.limit, Some(100));
    assert_eq!(args.cursor, Some("label_cursor".to_string()));
}

#[test]
fn test_section_filter_args_builder() {
    let args = SectionFilterArgs {
        project_id: Some("proj_123".to_string()),
        limit: Some(15),
        cursor: None,
    };

    assert_eq!(args.project_id, Some("proj_123".to_string()));
    assert_eq!(args.limit, Some(15));
    assert!(args.cursor.is_none());
}

#[test]
fn test_comment_filter_args_builder() {
    let args = CommentFilterArgs {
        task_id: Some("task_123".to_string()),
        project_id: None,
        limit: Some(30),
        cursor: Some("comment_cursor".to_string()),
    };

    assert_eq!(args.task_id, Some("task_123".to_string()));
    assert!(args.project_id.is_none());
    assert_eq!(args.limit, Some(30));
    assert_eq!(args.cursor, Some("comment_cursor".to_string()));
}

#[test]
fn test_serde_serialization() {
    let task = Task {
        id: "123".to_string(),
        content: "Test task".to_string(),
        description: "Test description".to_string(),
        project_id: "proj_123".to_string(),
        section_id: None,
        parent_id: None,
        order: 1,
        priority: 3,
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

    // Test that we can serialize to JSON
    let json = serde_json::to_string(&task).unwrap();
    assert!(json.contains("Test task"));
    assert!(json.contains("proj_123"));
    assert!(json.contains("false"));

    // Test that we can deserialize from JSON
    let deserialized_task: Task = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized_task.id, task.id);
    assert_eq!(deserialized_task.content, task.content);
    assert_eq!(deserialized_task.project_id, task.project_id);
}

#[test]
fn test_serde_deserialization() {
    let json = r#"{
        "id": "456",
        "content": "Deserialized task",
        "description": "Test deserialization",
        "project_id": "proj_456",
        "section_id": null,
        "parent_id": null,
        "order": 2,
        "priority": 4,
        "is_completed": true,
        "labels": ["deserialized", "test"],
        "created_at": "2024-01-02T00:00:00Z",
        "due": null,
        "deadline": null,
        "duration": null,
        "assignee_id": null,
        "url": "https://todoist.com",
        "comment_count": 1
    }"#;

    let task: Task = serde_json::from_str(json).unwrap();
    assert_eq!(task.id, "456");
    assert_eq!(task.content, "Deserialized task");
    assert_eq!(task.description, "Test deserialization");
    assert_eq!(task.project_id, "proj_456");
    assert_eq!(task.order, 2);
    assert_eq!(task.priority, 4);
    assert!(task.is_completed);
    assert_eq!(task.labels.len(), 2);
    assert!(task.labels.contains(&"deserialized".to_string()));
    assert!(task.labels.contains(&"test".to_string()));
    assert_eq!(task.comment_count, 1);
}

#[test]
fn test_clone_functionality() {
    let original_task = Task {
        id: "789".to_string(),
        content: "Original task".to_string(),
        description: "Original description".to_string(),
        project_id: "proj_789".to_string(),
        section_id: None,
        parent_id: None,
        order: 3,
        priority: 2,
        is_completed: false,
        labels: vec!["original".to_string()],
        created_at: "2024-01-03T00:00:00Z".to_string(),
        due: None,
        deadline: None,
        duration: None,
        assignee_id: None,
        url: "https://todoist.com".to_string(),
        comment_count: 0,
    };

    let cloned_task = original_task.clone();

    assert_eq!(cloned_task.id, original_task.id);
    assert_eq!(cloned_task.content, original_task.content);
    assert_eq!(cloned_task.description, original_task.description);
    assert_eq!(cloned_task.project_id, original_task.project_id);
    assert_eq!(cloned_task.order, original_task.order);
    assert_eq!(cloned_task.priority, original_task.priority);
    assert_eq!(cloned_task.is_completed, original_task.is_completed);
    assert_eq!(cloned_task.labels, original_task.labels);
    assert_eq!(cloned_task.created_at, original_task.created_at);
    assert_eq!(cloned_task.comment_count, original_task.comment_count);
}

#[test]
fn test_debug_formatting() {
    let task = Task {
        id: "debug_123".to_string(),
        content: "Debug task".to_string(),
        description: "Debug description".to_string(),
        project_id: "proj_debug".to_string(),
        section_id: None,
        parent_id: None,
        order: 1,
        priority: 1,
        is_completed: false,
        labels: vec!["debug".to_string()],
        created_at: "2024-01-01T00:00:00Z".to_string(),
        due: None,
        deadline: None,
        duration: None,
        assignee_id: None,
        url: "https://todoist.com".to_string(),
        comment_count: 0,
    };

    let debug_output = format!("{:?}", task);
    assert!(debug_output.contains("Debug task"));
    assert!(debug_output.contains("proj_debug"));
    assert!(debug_output.contains("debug"));
}
