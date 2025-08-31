use todoist_api::*;

#[test]
fn test_task_creation() {
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
        labels: vec!["test".to_string(), "important".to_string()],
        created_at: "2024-01-01T00:00:00Z".to_string(),
        due: None,
        deadline: None,
        duration: None,
        assignee_id: None,
        url: "https://todoist.com".to_string(),
        comment_count: 0,
    };

    assert_eq!(task.id, "123");
    assert_eq!(task.content, "Test task");
    assert_eq!(task.description, "Test description");
    assert_eq!(task.project_id, "proj_123");
    assert_eq!(task.priority, 3);
    assert!(!task.is_completed);
    assert_eq!(task.labels.len(), 2);
    assert!(task.labels.contains(&"test".to_string()));
    assert!(task.labels.contains(&"important".to_string()));
}

#[test]
fn test_project_creation() {
    let project = Project {
        id: "proj_123".to_string(),
        name: "Test Project".to_string(),
        comment_count: 5,
        order: 1,
        color: "blue".to_string(),
        is_shared: false,
        is_favorite: true,
        is_inbox_project: false,
        is_team_inbox: false,
        view_style: "list".to_string(),
        url: "https://todoist.com".to_string(),
        parent_id: None,
    };

    assert_eq!(project.id, "proj_123");
    assert_eq!(project.name, "Test Project");
    assert_eq!(project.comment_count, 5);
    assert_eq!(project.color, "blue");
    assert!(!project.is_shared);
    assert!(project.is_favorite);
    assert_eq!(project.view_style, "list");
}

#[test]
fn test_label_creation() {
    let label = Label {
        id: "label_123".to_string(),
        name: "Important".to_string(),
        color: "red".to_string(),
        order: 1,
        is_favorite: true,
    };

    assert_eq!(label.id, "label_123");
    assert_eq!(label.name, "Important");
    assert_eq!(label.color, "red");
    assert_eq!(label.order, 1);
    assert!(label.is_favorite);
}

#[test]
fn test_section_creation() {
    let section = Section {
        id: "section_123".to_string(),
        name: "Development".to_string(),
        project_id: "proj_123".to_string(),
        order: 1,
    };

    assert_eq!(section.id, "section_123");
    assert_eq!(section.name, "Development");
    assert_eq!(section.project_id, "proj_123");
    assert_eq!(section.order, 1);
}

#[test]
fn test_comment_creation() {
    let comment = Comment {
        id: "comment_123".to_string(),
        content: "This is a comment".to_string(),
        posted_at: "2024-01-01T00:00:00Z".to_string(),
        attachment: None,
        project_id: None,
        task_id: Some("task_123".to_string()),
    };

    assert_eq!(comment.id, "comment_123");
    assert_eq!(comment.content, "This is a comment");
    assert_eq!(comment.posted_at, "2024-01-01T00:00:00Z");
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
    };

    assert_eq!(due.string, "tomorrow at 12:00");
    assert_eq!(due.date, "2024-01-02");
    assert!(!due.is_recurring);
    assert!(due.datetime.is_some());
    assert!(due.timezone.is_some());
}

#[test]
fn test_deadline_creation() {
    let deadline = Deadline {
        date: "2024-01-15".to_string(),
    };

    assert_eq!(deadline.date, "2024-01-15");
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
    assert!(args.assignee_id.is_none());
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
    assert!(args.assignee_id.is_none());
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
