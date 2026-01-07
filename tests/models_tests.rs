use todoist_api::*;

#[test]
fn test_task_creation() {
    let task = Task {
        id: "123".to_string(),
        user_id: "user123".to_string(),
        content: "Test task".to_string(),
        description: "Test description".to_string(),
        project_id: "proj_123".to_string(),
        section_id: None,
        parent_id: None,
        added_by_uid: None,
        assigned_by_uid: None,
        responsible_uid: None,
        labels: vec!["test".to_string(), "important".to_string()],
        deadline: None,
        duration: None,
        checked: false,
        is_deleted: false,
        added_at: "2024-01-01T00:00:00Z".to_string(),
        completed_at: None,
        completed_by_uid: None,
        updated_at: None,
        due: None,
        priority: 3,
        child_order: 0,
        note_count: 0,
        day_order: 0,
        is_collapsed: false,
    };

    assert_eq!(task.id, "123");
    assert_eq!(task.content, "Test task");
    assert_eq!(task.description, "Test description");
    assert_eq!(task.project_id, "proj_123");
    assert_eq!(task.priority, 3);
    assert_eq!(task.labels.len(), 2);
    assert!(task.labels.contains(&"test".to_string()));
    assert!(task.labels.contains(&"important".to_string()));
}

#[test]
fn test_project_creation() {
    let project = Project {
        id: "proj_123".to_string(),
        name: "Test Project".to_string(),
        color: "blue".to_string(),
        is_shared: false,
        is_favorite: true,
        inbox_project: false,
        view_style: "list".to_string(),
        parent_id: None,
        child_order: 0,
        creator_uid: None,
        created_at: None,
        updated_at: None,
        is_archived: false,
        is_deleted: false,
        is_frozen: false,
        is_collapsed: false,
        can_assign_tasks: false,
        default_order: 0,
        description: String::new(),
        public_key: String::new(),
        role: None,
    };

    assert_eq!(project.id, "proj_123");
    assert_eq!(project.name, "Test Project");
    assert_eq!(project.color, "blue");
    assert!(!project.is_shared);
    assert!(project.is_favorite);
    assert!(!project.inbox_project);
    assert_eq!(project.view_style, "list");
}

#[test]
fn test_label_creation() {
    let label = Label {
        id: "label_123".to_string(),
        name: "Important".to_string(),
        color: "red".to_string(),
        order: Some(1),
        is_favorite: true,
    };

    assert_eq!(label.id, "label_123");
    assert_eq!(label.name, "Important");
    assert_eq!(label.color, "red");
    assert_eq!(label.order, Some(1));
    assert!(label.is_favorite);
}

#[test]
fn test_section_creation() {
    let section = Section {
        id: "section_123".to_string(),
        user_id: "user123".to_string(),
        project_id: "proj_123".to_string(),
        added_at: "2024-01-01T00:00:00Z".to_string(),
        updated_at: None,
        archived_at: None,
        name: "Development".to_string(),
        section_order: 1,
        is_archived: false,
        is_deleted: false,
        is_collapsed: false,
    };

    assert_eq!(section.id, "section_123");
    assert_eq!(section.name, "Development");
    assert_eq!(section.project_id, "proj_123");
    assert_eq!(section.section_order, 1);
}

#[test]
fn test_comment_creation() {
    let comment = Comment {
        id: "comment_123".to_string(),
        content: "This is a comment".to_string(),
        posted_at: Some("2024-01-01T00:00:00Z".to_string()),
        posted_uid: None,
        file_attachment: None,
        uids_to_notify: None,
        is_deleted: false,
        reactions: None,
        project_id: None,
        task_id: Some("task_123".to_string()),
    };

    assert_eq!(comment.id, "comment_123");
    assert_eq!(comment.content, "This is a comment");
    assert_eq!(comment.posted_at, Some("2024-01-01T00:00:00Z".to_string()));
    assert!(comment.task_id.is_some());
    assert!(comment.project_id.is_none());
}

#[test]
fn test_attachment_creation() {
    let attachment = Attachment {
        file_name: "document.pdf".to_string(),
        file_type: "application/pdf".to_string(),
        file_url: "https://example.com/document.pdf".to_string(),
        resource_type: "file".to_string(),
    };

    assert_eq!(attachment.file_name, "document.pdf");
    assert_eq!(attachment.file_type, "application/pdf");
    assert_eq!(attachment.file_url, "https://example.com/document.pdf");
    assert_eq!(attachment.resource_type, "file");
}

#[test]
fn test_user_creation() {
    let user = User {
        id: "user_123".to_string(),
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        avatar_url: Some("https://example.com/avatar.jpg".to_string()),
        is_premium: true,
        is_business_account: false,
    };

    assert_eq!(user.id, "user_123");
    assert_eq!(user.name, "John Doe");
    assert_eq!(user.email, "john@example.com");
    assert!(user.avatar_url.is_some());
    assert!(user.is_premium);
    assert!(!user.is_business_account);
}

#[test]
fn test_due_creation() {
    let due = Due {
        string: "tomorrow at 12:00".to_string(),
        date: "2024-01-02".to_string(),
        is_recurring: false,
        datetime: Some("2024-01-02T12:00:00Z".to_string()),
        timezone: Some("UTC".to_string()),
        lang: Some("en".to_string()),
    };

    assert_eq!(due.string, "tomorrow at 12:00");
    assert_eq!(due.date, "2024-01-02");
    assert!(!due.is_recurring);
    assert!(due.datetime.is_some());
    assert!(due.timezone.is_some());
    assert_eq!(due.lang, Some("en".to_string()));
}

#[test]
fn test_deadline_creation() {
    let deadline = Deadline {
        date: "2024-01-15".to_string(),
        lang: Some("en".to_string()),
    };

    assert_eq!(deadline.date, "2024-01-15");
    assert_eq!(deadline.lang, Some("en".to_string()));
}

#[test]
fn test_duration_creation() {
    let duration = Duration {
        amount: 30,
        unit: "minute".to_string(),
    };

    assert_eq!(duration.amount, 30);
    assert_eq!(duration.unit, "minute");
}

#[test]
fn test_create_task_args_default() {
    let args = CreateTaskArgs::default();

    assert_eq!(args.content, "");
    assert!(args.description.is_none());
    assert!(args.project_id.is_none());
    assert!(args.section_id.is_none());
    assert!(args.parent_id.is_none());
    assert!(args.order.is_none());
    assert!(args.priority.is_none());
    assert!(args.labels.is_none());
    assert!(args.due_string.is_none());
    assert!(args.due_date.is_none());
    assert!(args.due_datetime.is_none());
    assert!(args.due_lang.is_none());
    assert!(args.deadline_date.is_none());
    assert!(args.deadline_lang.is_none());
    assert!(args.duration.is_none());
    assert!(args.duration_unit.is_none());
}

#[test]
fn test_update_task_args_default() {
    let args = UpdateTaskArgs::default();

    assert!(args.content.is_none());
    assert!(args.description.is_none());
    assert!(args.priority.is_none());
    assert!(args.labels.is_none());
    assert!(args.due_string.is_none());
    assert!(args.due_date.is_none());
    assert!(args.due_datetime.is_none());
    assert!(args.due_lang.is_none());
    assert!(args.deadline_date.is_none());
    assert!(args.deadline_lang.is_none());
    assert!(args.duration.is_none());
    assert!(args.duration_unit.is_none());
}

#[test]
fn test_create_project_args_default() {
    let args = CreateProjectArgs::default();

    assert_eq!(args.name, "");
    assert!(args.color.is_none());
    assert!(args.parent_id.is_none());
    assert!(args.is_favorite.is_none());
    assert!(args.view_style.is_none());
}

#[test]
fn test_update_project_args_default() {
    let args = UpdateProjectArgs::default();

    assert!(args.name.is_none());
    assert!(args.color.is_none());
    assert!(args.is_favorite.is_none());
    assert!(args.view_style.is_none());
}

#[test]
fn test_create_label_args_default() {
    let args = CreateLabelArgs::default();

    assert_eq!(args.name, "");
    assert!(args.color.is_none());
    assert!(args.order.is_none());
    assert!(args.is_favorite.is_none());
}

#[test]
fn test_update_label_args_default() {
    let args = UpdateLabelArgs::default();

    assert!(args.name.is_none());
    assert!(args.color.is_none());
    assert!(args.order.is_none());
    assert!(args.is_favorite.is_none());
}

#[test]
fn test_create_section_args_default() {
    let args = CreateSectionArgs::default();

    assert_eq!(args.name, "");
    assert_eq!(args.project_id, "");
    assert!(args.order.is_none());
}

#[test]
fn test_update_section_args_default() {
    let args = UpdateSectionArgs::default();

    assert_eq!(args.name, "");
}

#[test]
fn test_create_comment_args_default() {
    let args = CreateCommentArgs::default();

    assert_eq!(args.content, "");
    assert!(args.task_id.is_none());
    assert!(args.project_id.is_none());
    assert!(args.attachment.is_none());
}

#[test]
fn test_update_comment_args_default() {
    let args = UpdateCommentArgs::default();

    assert_eq!(args.content, "");
}

#[test]
fn test_filter_args_creation() {
    let task_filter = TaskFilterArgs {
        query: "today".to_string(),
        lang: Some("en".to_string()),
        limit: Some(20),
        cursor: None,
    };

    assert_eq!(task_filter.query, "today");
    assert_eq!(task_filter.lang, Some("en".to_string()));
    assert_eq!(task_filter.limit, Some(20));
    assert!(task_filter.cursor.is_none());

    let project_filter = ProjectFilterArgs {
        limit: Some(10),
        cursor: Some("cursor_123".to_string()),
    };

    assert_eq!(project_filter.limit, Some(10));
    assert_eq!(project_filter.cursor, Some("cursor_123".to_string()));
}

#[test]
fn test_task_deserialization_from_api_format() {
    // Test deserialization from actual API format (uses user_id, not creator_id)
    let json = r#"{
        "user_id": "12345678",
        "id": "6X6WMMqgq2PWxjCX",
        "project_id": "6XGgm6PHrGgMpCFX",
        "section_id": null,
        "parent_id": null,
        "added_by_uid": null,
        "assigned_by_uid": null,
        "responsible_uid": null,
        "labels": ["work", "priority"],
        "deadline": null,
        "duration": {"amount": 30, "unit": "minute"},
        "checked": false,
        "is_deleted": false,
        "added_at": "2024-01-15T10:00:00Z",
        "completed_at": null,
        "completed_by_uid": null,
        "updated_at": "2024-01-15T12:00:00Z",
        "due": {
            "date": "2024-01-20",
            "string": "Jan 20",
            "lang": "en",
            "is_recurring": false,
            "datetime": null,
            "timezone": null
        },
        "priority": 4,
        "child_order": 1,
        "content": "Review project documentation",
        "description": "Check all sections for completeness",
        "note_count": 0,
        "day_order": -1,
        "is_collapsed": false
    }"#;

    let task: Task = serde_json::from_str(json).unwrap();
    assert_eq!(task.user_id, "12345678");
    assert_eq!(task.content, "Review project documentation");
    assert!(!task.checked);
    assert!(!task.is_deleted);
}

#[test]
fn test_project_deserialization_from_api_format() {
    // Test deserialization from actual API format
    let json = r#"{
        "id": "6XGgm6PHrGgMpCFX",
        "can_assign_tasks": true,
        "child_order": 1,
        "color": "charcoal",
        "creator_uid": "12345678",
        "created_at": "2024-01-01T00:00:00Z",
        "is_archived": false,
        "is_deleted": false,
        "is_favorite": true,
        "is_frozen": false,
        "name": "My Project",
        "updated_at": "2024-01-15T10:00:00Z",
        "view_style": "list",
        "default_order": 0,
        "description": "Project description",
        "public_key": "abc123publickey",
        "role": "owner",
        "parent_id": null,
        "inbox_project": false,
        "is_collapsed": false,
        "is_shared": false
    }"#;

    let project: Project = serde_json::from_str(json).unwrap();
    assert_eq!(project.id, "6XGgm6PHrGgMpCFX");
    assert_eq!(project.name, "My Project");
    assert!(project.is_favorite);
    assert!(!project.inbox_project);
    assert!(!project.is_shared);
}

#[test]
fn test_section_deserialization_from_api_format() {
    // Test deserialization from actual API format (uses user_id, not creator_id)
    let json = r#"{
        "id": "6fFPHV272WWh3gpW",
        "user_id": "12345678",
        "project_id": "6XGgm6PHrGgMpCFX",
        "added_at": "2024-01-05T09:00:00Z",
        "updated_at": "2024-01-10T14:30:00Z",
        "archived_at": null,
        "name": "In Progress",
        "section_order": 1,
        "is_archived": false,
        "is_deleted": false,
        "is_collapsed": false
    }"#;

    let section: Section = serde_json::from_str(json).unwrap();
    assert_eq!(section.id, "6fFPHV272WWh3gpW");
    assert_eq!(section.user_id, "12345678");
    assert_eq!(section.name, "In Progress");
    assert!(!section.is_deleted);
}

#[test]
fn test_label_deserialization_with_null_order() {
    // Test deserialization when order is null
    let json = r#"{
        "id": "7YilS6YUrHwNqKOh",
        "name": "work",
        "color": "blue",
        "order": null,
        "is_favorite": false
    }"#;

    let label: Label = serde_json::from_str(json).unwrap();
    assert_eq!(label.id, "7YilS6YUrHwNqKOh");
    assert_eq!(label.name, "work");
    assert_eq!(label.order, None);
}

#[test]
fn test_comment_deserialization_from_api_format() {
    // Test deserialization from actual API format (uses file_attachment, not attachment)
    let json = r#"{
        "id": "8ZjmT7ZVsIxOrLPi",
        "posted_uid": "12345678",
        "content": "This is a comment on the task",
        "file_attachment": null,
        "uids_to_notify": null,
        "is_deleted": false,
        "posted_at": "2024-01-15T11:30:00Z",
        "reactions": null
    }"#;

    let comment: Comment = serde_json::from_str(json).unwrap();
    assert_eq!(comment.id, "8ZjmT7ZVsIxOrLPi");
    assert_eq!(comment.content, "This is a comment on the task");
    assert!(!comment.is_deleted);
}

#[test]
fn test_paginated_response_deserialization() {
    // Test deserialization of paginated response format
    let json = r#"{
        "results": [
            {
                "user_id": "12345678",
                "id": "task1",
                "project_id": "proj1",
                "section_id": null,
                "parent_id": null,
                "added_by_uid": null,
                "assigned_by_uid": null,
                "responsible_uid": null,
                "labels": [],
                "deadline": null,
                "duration": null,
                "checked": false,
                "is_deleted": false,
                "added_at": "2024-01-15T10:00:00Z",
                "completed_at": null,
                "completed_by_uid": null,
                "updated_at": null,
                "due": null,
                "priority": 1,
                "child_order": 0,
                "content": "First task",
                "description": "",
                "note_count": 0,
                "day_order": -1,
                "is_collapsed": false
            }
        ],
        "next_cursor": "14540000435w8hj8pXXwPQJJch.X9DBH8ya2Xenok55"
    }"#;

    let response: PaginatedResponse<Task> = serde_json::from_str(json).unwrap();
    assert_eq!(response.results.len(), 1);
    assert_eq!(response.results[0].id, "task1");
    assert_eq!(
        response.next_cursor,
        Some("14540000435w8hj8pXXwPQJJch.X9DBH8ya2Xenok55".to_string())
    );
}
