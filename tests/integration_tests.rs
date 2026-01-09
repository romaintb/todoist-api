use serde_json::json;
use todoist_api::*;
use wiremock::{
    matchers::{method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

#[tokio::test]
async fn test_wrapper_creation() {
    let _todoist = TodoistWrapper::new("test-token".to_string());
}

// ===== PROJECT OPERATIONS =====

#[tokio::test]
async fn test_get_projects() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/projects"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "results": [
                {
                    "id": "proj_123",
                    "name": "Test Project",
                    "color": "blue",
                    "shared": false,
                    "is_favorite": false,
                    "is_inbox_project": false,
                    "view_style": "list"
                }
            ],
            "next_cursor": null
        })))
        .mount(&mock_server)
        .await;

    let todoist = TodoistWrapper::with_base_url("test-token".to_string(), mock_server.uri());

    let result = todoist.get_projects(None, None).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.results.len(), 1);
    assert_eq!(response.results[0].id, "proj_123");
    assert!(response.next_cursor.is_none());
}

#[tokio::test]
async fn test_get_projects_with_limit() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/projects"))
        .and(query_param("limit", "10"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "results": [],
            "next_cursor": null
        })))
        .mount(&mock_server)
        .await;

    let todoist = TodoistWrapper::with_base_url("test-token".to_string(), mock_server.uri());

    let result = todoist.get_projects(Some(10), None).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_project() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/projects/proj_123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "proj_123",
            "name": "Test Project",
            "color": "blue",
            "shared": false,
            "is_favorite": false,
            "is_inbox_project": false,
            "view_style": "list"
        })))
        .mount(&mock_server)
        .await;

    let todoist = TodoistWrapper::with_base_url("test-token".to_string(), mock_server.uri());

    let result = todoist.get_project("proj_123").await;
    assert!(result.is_ok());
    let project = result.unwrap();
    assert_eq!(project.id, "proj_123");
    assert_eq!(project.name, "Test Project");
}

#[tokio::test]
async fn test_get_project_not_found() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/projects/notexist"))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({
            "error": "Project not found"
        })))
        .mount(&mock_server)
        .await;

    let todoist = TodoistWrapper::with_base_url("test-token".to_string(), mock_server.uri());

    let result = todoist.get_project("notexist").await;
    assert!(result.is_err());
    match result {
        Err(TodoistError::NotFound { .. }) => (),
        _ => panic!("Expected NotFound error"),
    }
}

#[tokio::test]
async fn test_create_project() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/projects"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "new_proj",
            "name": "New Project",
            "color": "red",
            "shared": false,
            "is_favorite": false,
            "is_inbox_project": false,
            "view_style": "list"
        })))
        .mount(&mock_server)
        .await;

    let todoist = TodoistWrapper::with_base_url("test-token".to_string(), mock_server.uri());

    let args = CreateProjectArgs {
        name: "New Project".to_string(),
        color: Some("red".to_string()),
        ..Default::default()
    };

    let result = todoist.create_project(&args).await;
    assert!(result.is_ok());
    let project = result.unwrap();
    assert_eq!(project.id, "new_proj");
    assert_eq!(project.name, "New Project");
}

#[tokio::test]
async fn test_update_project() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/projects/proj_123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "proj_123",
            "name": "Updated Name",
            "color": "green",
            "shared": false,
            "is_favorite": true,
            "is_inbox_project": false,
            "view_style": "list"
        })))
        .mount(&mock_server)
        .await;

    let todoist = TodoistWrapper::with_base_url("test-token".to_string(), mock_server.uri());

    let args = UpdateProjectArgs {
        name: Some("Updated Name".to_string()),
        color: Some("green".to_string()),
        ..Default::default()
    };

    let result = todoist.update_project("proj_123", &args).await;
    assert!(result.is_ok());
    let project = result.unwrap();
    assert_eq!(project.name, "Updated Name");
    assert_eq!(project.color, "green");
}

#[tokio::test]
async fn test_update_project_no_fields() {
    let todoist = TodoistWrapper::new("test-token".to_string());
    let args = UpdateProjectArgs::default();

    let result = todoist.update_project("proj_123", &args).await;
    assert!(result.is_err());
    match result {
        Err(TodoistError::ValidationError { .. }) => (),
        _ => panic!("Expected ValidationError"),
    }
}

#[tokio::test]
async fn test_delete_project() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/projects/proj_123"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let todoist = TodoistWrapper::with_base_url("test-token".to_string(), mock_server.uri());

    let result = todoist.delete_project("proj_123").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_projects_filtered() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/projects"))
        .and(query_param("limit", "5"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {
                "id": "proj_1",
                "name": "Project 1",
                "color": "blue",
                "shared": false,
                "is_favorite": false,
                "is_inbox_project": false,
                "view_style": "list"
            }
        ])))
        .mount(&mock_server)
        .await;

    let todoist = TodoistWrapper::with_base_url("test-token".to_string(), mock_server.uri());

    let args = ProjectFilterArgs {
        limit: Some(5),
        cursor: None,
    };

    let result = todoist.get_projects_filtered(&args).await;
    assert!(result.is_ok());
    let projects = result.unwrap();
    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].id, "proj_1");
}

// ===== TASK OPERATIONS =====

#[tokio::test]
async fn test_get_tasks() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/tasks"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "results": [
                {
                    "id": "task_123",
                    "creator_id": "user_1",
                    "content": "Test Task",
                    "description": "Test description",
                    "project_id": "proj_1",
                    "section_id": null,
                    "parent_id": null,
                    "added_by_uid": null,
                    "assigned_by_uid": null,
                    "responsible_uid": null,
                    "labels": [],
                    "deadline": null,
                    "duration": null,
                    "added_at": "2024-01-01T00:00:00Z",
                    "updated_at": null,
                    "due": null,
                    "priority":1,
                    "child_order": 0,
                    "note_count": 0,
                    "day_order": 0,
                    "is_collapsed": false
                }
            ],
            "next_cursor": "cursor123"
        })))
        .mount(&mock_server)
        .await;

    let todoist = TodoistWrapper::with_base_url("test-token".to_string(), mock_server.uri());

    let result = todoist.get_tasks(None, None).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.results.len(), 1);
    assert_eq!(response.results[0].id, "task_123");
    assert_eq!(response.next_cursor, Some("cursor123".to_string()));
}

#[tokio::test]
async fn test_get_task() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/tasks/task_123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "task_123",
            "creator_id": "user_1",
            "content": "Single Task",
            "description": "Description",
            "project_id": "proj_1",
            "section_id": null,
            "parent_id": null,
            "added_by_uid": null,
            "assigned_by_uid": null,
            "responsible_uid": null,
            "labels": [],
            "deadline": null,
            "duration": null,
            "added_at": "2024-01-01T00:00:00Z",
            "updated_at": null,
            "due": null,
            "priority": 2,
            "child_order": 0,
            "note_count": 0,
            "day_order": 0,
            "is_collapsed": false
        })))
        .mount(&mock_server)
        .await;

    let todoist = TodoistWrapper::with_base_url("test-token".to_string(), mock_server.uri());

    let result = todoist.get_task("task_123").await;
    assert!(result.is_ok());
    let task = result.unwrap();
    assert_eq!(task.id, "task_123");
    assert_eq!(task.content, "Single Task");
}

#[tokio::test]
async fn test_create_task() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/tasks"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "new_task",
            "creator_id": "user_1",
            "content": "New Task",
            "description": "New Description",
            "project_id": "proj_1",
            "section_id": null,
            "parent_id": null,
            "added_by_uid": null,
            "assigned_by_uid": null,
            "responsible_uid": null,
            "labels": ["important"],
            "deadline": null,
            "duration": null,
            "added_at": "2024-01-01T00:00:00Z",
            "updated_at": null,
            "due": null,
            "priority": 3,
            "child_order": 0,
            "note_count": 0,
            "day_order": 0,
            "is_collapsed": false
        })))
        .mount(&mock_server)
        .await;

    let todoist = TodoistWrapper::with_base_url("test-token".to_string(), mock_server.uri());

    let args = CreateTaskArgs {
        content: "New Task".to_string(),
        description: Some("New Description".to_string()),
        project_id: Some("proj_1".to_string()),
        priority: Some(3),
        labels: Some(vec!["important".to_string()]),
        ..Default::default()
    };

    let result = todoist.create_task(&args).await;
    assert!(result.is_ok());
    let task = result.unwrap();
    assert_eq!(task.id, "new_task");
    assert_eq!(task.content, "New Task");
    assert_eq!(task.priority, 3);
}

#[tokio::test]
async fn test_update_task() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/tasks/task_123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "task_123",
            "creator_id": "user_1",
            "content": "Updated Task",
            "description": "Updated Description",
            "project_id": "proj_1",
            "section_id": null,
            "parent_id": null,
            "added_by_uid": null,
            "assigned_by_uid": null,
            "responsible_uid": null,
            "labels": [],
            "deadline": null,
            "duration": null,
            "added_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-02T00:00:00Z",
            "due": null,
            "priority": 4,
            "child_order": 0,
            "note_count": 0,
            "day_order": 0,
            "is_collapsed": false
        })))
        .mount(&mock_server)
        .await;

    let todoist = TodoistWrapper::with_base_url("test-token".to_string(), mock_server.uri());

    let args = UpdateTaskArgs {
        content: Some("Updated Task".to_string()),
        description: Some("Updated Description".to_string()),
        priority: Some(4),
        ..Default::default()
    };

    let result = todoist.update_task("task_123", &args).await;
    assert!(result.is_ok());
    let task = result.unwrap();
    assert_eq!(task.content, "Updated Task");
    assert_eq!(task.priority, 4);
}

#[tokio::test]
async fn test_complete_task() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/tasks/task_123/close"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let todoist = TodoistWrapper::with_base_url("test-token".to_string(), mock_server.uri());

    let result = todoist.complete_task("task_123").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_reopen_task() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/tasks/task_123/reopen"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let todoist = TodoistWrapper::with_base_url("test-token".to_string(), mock_server.uri());

    let result = todoist.reopen_task("task_123").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_task() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/tasks/task_123"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let todoist = TodoistWrapper::with_base_url("test-token".to_string(), mock_server.uri());

    let result = todoist.delete_task("task_123").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_tasks_for_project() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/tasks"))
        .and(query_param("project_id", "proj_123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "results": [
                {
                    "id": "task_1",
                    "creator_id": "user_1",
                    "content": "Project Task",
                    "description": "",
                    "project_id": "proj_123",
                    "section_id": null,
                    "parent_id": null,
                    "added_by_uid": null,
                    "assigned_by_uid": null,
                    "responsible_uid": null,
                    "labels": [],
                    "deadline": null,
                    "duration": null,
                    "added_at": "2024-01-01T00:00:00Z",
                    "updated_at": null,
                    "due": null,
                    "priority": 1,
                    "child_order": 0,
                    "note_count": 0,
                    "day_order": 0,
                    "is_collapsed": false
                }
            ],
            "next_cursor": null
        })))
        .mount(&mock_server)
        .await;

    let todoist = TodoistWrapper::with_base_url("test-token".to_string(), mock_server.uri());

    let result = todoist.get_tasks_for_project("proj_123", None, None).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.results.len(), 1);
    assert_eq!(response.results[0].project_id, "proj_123");
}

#[tokio::test]
async fn test_get_tasks_by_filter() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/tasks"))
        .and(query_param("query", "today"))
        .and(query_param("lang", "en"))
        .and(query_param("limit", "20"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "results": [],
            "next_cursor": null
        })))
        .mount(&mock_server)
        .await;

    let todoist = TodoistWrapper::with_base_url("test-token".to_string(), mock_server.uri());

    let args = TaskFilterArgs {
        query: "today".to_string(),
        lang: Some("en".to_string()),
        limit: Some(20),
        cursor: None,
    };

    let result = todoist.get_tasks_by_filter(&args).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.results.len(), 0);
}

#[tokio::test]
async fn test_get_completed_tasks_by_completion_date() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/tasks/completed/by_completion_date"))
        .and(query_param("since", "2024-01-01T00:00:00Z"))
        .and(query_param("until", "2024-01-31T23:59:59Z"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "results": [],
            "next_cursor": null
        })))
        .mount(&mock_server)
        .await;

    let todoist = TodoistWrapper::with_base_url("test-token".to_string(), mock_server.uri());

    let args = CompletedTasksFilterArgs {
        since: Some("2024-01-01T00:00:00Z".to_string()),
        until: Some("2024-01-31T23:59:59Z".to_string()),
        ..Default::default()
    };

    let result = todoist.get_completed_tasks_by_completion_date(&args).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.results.len(), 0);
}

#[tokio::test]
async fn test_get_completed_tasks_by_due_date() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/tasks/completed/by_due_date"))
        .and(query_param("project_id", "proj_123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "results": [],
            "next_cursor": null
        })))
        .mount(&mock_server)
        .await;

    let todoist = TodoistWrapper::with_base_url("test-token".to_string(), mock_server.uri());

    let args = CompletedTasksFilterArgs {
        project_id: Some("proj_123".to_string()),
        ..Default::default()
    };

    let result = todoist.get_completed_tasks_by_due_date(&args).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.results.len(), 0);
}

// ===== LABEL OPERATIONS =====

#[tokio::test]
async fn test_get_labels() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/labels"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "results": [
                {
                    "id": "label_1",
                    "name": "Important",
                    "color": "red",
                    "order": 1,
                    "is_favorite": false
                }
            ],
            "next_cursor": null
        })))
        .mount(&mock_server)
        .await;

    let todoist = TodoistWrapper::with_base_url("test-token".to_string(), mock_server.uri());

    let result = todoist.get_labels(None, None).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.results.len(), 1);
    assert_eq!(response.results[0].id, "label_1");
}

#[tokio::test]
async fn test_get_label() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/labels/label_123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "label_123",
            "name": "Work",
            "color": "blue",
            "order": 2,
            "is_favorite": true
        })))
        .mount(&mock_server)
        .await;

    let todoist = TodoistWrapper::with_base_url("test-token".to_string(), mock_server.uri());

    let result = todoist.get_label("label_123").await;
    assert!(result.is_ok());
    let label = result.unwrap();
    assert_eq!(label.id, "label_123");
    assert_eq!(label.name, "Work");
}

#[tokio::test]
async fn test_create_label() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/labels"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "new_label",
            "name": "New Label",
            "color": "green",
            "order": 3,
            "is_favorite": false
        })))
        .mount(&mock_server)
        .await;

    let todoist = TodoistWrapper::with_base_url("test-token".to_string(), mock_server.uri());

    let args = CreateLabelArgs {
        name: "New Label".to_string(),
        color: Some("green".to_string()),
        order: Some(3),
        ..Default::default()
    };

    let result = todoist.create_label(&args).await;
    assert!(result.is_ok());
    let label = result.unwrap();
    assert_eq!(label.id, "new_label");
    assert_eq!(label.name, "New Label");
}

#[tokio::test]
async fn test_update_label() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/labels/label_123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "label_123",
            "name": "Updated Label",
            "color": "purple",
            "order": 5,
            "is_favorite": true
        })))
        .mount(&mock_server)
        .await;

    let todoist = TodoistWrapper::with_base_url("test-token".to_string(), mock_server.uri());

    let args = UpdateLabelArgs {
        name: Some("Updated Label".to_string()),
        color: Some("purple".to_string()),
        order: Some(5),
        is_favorite: Some(true),
    };

    let result = todoist.update_label("label_123", &args).await;
    assert!(result.is_ok());
    let label = result.unwrap();
    assert_eq!(label.name, "Updated Label");
    assert_eq!(label.color, "purple");
}

#[tokio::test]
async fn test_delete_label() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/labels/label_123"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let todoist = TodoistWrapper::with_base_url("test-token".to_string(), mock_server.uri());

    let result = todoist.delete_label("label_123").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_labels_filtered() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/labels"))
        .and(query_param("limit", "10"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {
                "id": "label_1",
                "name": "Label 1",
                "color": "red",
                "order": 1,
                "is_favorite": false
            }
        ])))
        .mount(&mock_server)
        .await;

    let todoist = TodoistWrapper::with_base_url("test-token".to_string(), mock_server.uri());

    let args = LabelFilterArgs {
        limit: Some(10),
        cursor: None,
    };

    let result = todoist.get_labels_filtered(&args).await;
    assert!(result.is_ok());
    let labels = result.unwrap();
    assert_eq!(labels.len(), 1);
    assert_eq!(labels[0].id, "label_1");
}

// ===== SECTION OPERATIONS =====

#[tokio::test]
async fn test_get_sections() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/sections"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "results": [
                {
                    "id": "sec_123",
                    "creator_id": "user_1",
                    "project_id": "proj_1",
                    "added_at": "2024-01-01T00:00:00Z",
                    "updated_at": null,
                    "archived_at": null,
                    "name": "Development",
                    "section_order": 1,
                    "is_archived": false,
                    "is_collapsed": false
                }
            ],
            "next_cursor": null
        })))
        .mount(&mock_server)
        .await;

    let todoist = TodoistWrapper::with_base_url("test-token".to_string(), mock_server.uri());

    let result = todoist.get_sections(None, None).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.results.len(), 1);
    assert_eq!(response.results[0].id, "sec_123");
}

#[tokio::test]
async fn test_get_section() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/sections/sec_123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "sec_123",
            "creator_id": "user_1",
            "project_id": "proj_1",
            "added_at": "2024-01-01T00:00:00Z",
            "updated_at": null,
            "archived_at": null,
            "name": "Testing",
            "section_order": 2,
            "is_archived": false,
            "is_collapsed": false
        })))
        .mount(&mock_server)
        .await;

    let todoist = TodoistWrapper::with_base_url("test-token".to_string(), mock_server.uri());

    let result = todoist.get_section("sec_123").await;
    assert!(result.is_ok());
    let section = result.unwrap();
    assert_eq!(section.id, "sec_123");
    assert_eq!(section.name, "Testing");
}

#[tokio::test]
async fn test_create_section() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/sections"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "new_sec",
            "creator_id": "user_1",
            "project_id": "proj_1",
            "added_at": "2024-01-01T00:00:00Z",
            "updated_at": null,
            "archived_at": null,
            "name": "New Section",
            "section_order": 3,
            "is_archived": false,
            "is_collapsed": false
        })))
        .mount(&mock_server)
        .await;

    let todoist = TodoistWrapper::with_base_url("test-token".to_string(), mock_server.uri());

    let args = CreateSectionArgs {
        name: "New Section".to_string(),
        project_id: "proj_1".to_string(),
        order: Some(3),
    };

    let result = todoist.create_section(&args).await;
    assert!(result.is_ok());
    let section = result.unwrap();
    assert_eq!(section.id, "new_sec");
    assert_eq!(section.name, "New Section");
}

#[tokio::test]
async fn test_update_section() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/sections/sec_123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "sec_123",
            "creator_id": "user_1",
            "project_id": "proj_1",
            "added_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-02T00:00:00Z",
            "archived_at": null,
            "name": "Updated Section",
            "section_order": 1,
            "is_archived": false,
            "is_collapsed": false
        })))
        .mount(&mock_server)
        .await;

    let todoist = TodoistWrapper::with_base_url("test-token".to_string(), mock_server.uri());

    let args = UpdateSectionArgs {
        name: "Updated Section".to_string(),
    };

    let result = todoist.update_section("sec_123", &args).await;
    assert!(result.is_ok());
    let section = result.unwrap();
    assert_eq!(section.name, "Updated Section");
}

#[tokio::test]
async fn test_delete_section() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/sections/sec_123"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let todoist = TodoistWrapper::with_base_url("test-token".to_string(), mock_server.uri());

    let result = todoist.delete_section("sec_123").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_sections_filtered() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/sections"))
        .and(query_param("project_id", "proj_123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "results": [
                {
                    "id": "sec_1",
                    "creator_id": "user_1",
                    "project_id": "proj_123",
                    "added_at": "2024-01-01T00:00:00Z",
                    "updated_at": null,
                    "archived_at": null,
                    "name": "Section 1",
                    "section_order": 1,
                    "is_archived": false,
                    "is_collapsed": false
                }
            ],
            "next_cursor": null
        })))
        .mount(&mock_server)
        .await;

    let todoist = TodoistWrapper::with_base_url("test-token".to_string(), mock_server.uri());

    let args = SectionFilterArgs {
        project_id: Some("proj_123".to_string()),
        limit: None,
        cursor: None,
    };

    let result = todoist.get_sections_filtered(&args).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.results.len(), 1);
    assert_eq!(response.results[0].project_id, "proj_123");
}

// ===== COMMENT OPERATIONS =====

#[tokio::test]
async fn test_get_comments() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/comments"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {
                "id": "comment_1",
                "content": "Test comment",
                "posted_at": "2024-01-01T00:00:00Z",
                "attachment": null,
                "project_id": null,
                "task_id": "task_1"
            }
        ])))
        .mount(&mock_server)
        .await;

    let todoist = TodoistWrapper::with_base_url("test-token".to_string(), mock_server.uri());

    let result = todoist.get_comments().await;
    assert!(result.is_ok());
    let comments = result.unwrap();
    assert_eq!(comments.len(), 1);
    assert_eq!(comments[0].id, "comment_1");
}

#[tokio::test]
async fn test_get_comment() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/comments/comment_123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "comment_123",
            "content": "Single comment",
            "posted_at": "2024-01-01T00:00:00Z",
            "attachment": null,
            "project_id": null,
            "task_id": "task_1"
        })))
        .mount(&mock_server)
        .await;

    let todoist = TodoistWrapper::with_base_url("test-token".to_string(), mock_server.uri());

    let result = todoist.get_comment("comment_123").await;
    assert!(result.is_ok());
    let comment = result.unwrap();
    assert_eq!(comment.id, "comment_123");
    assert_eq!(comment.content, "Single comment");
}

#[tokio::test]
async fn test_create_comment() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/comments"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "new_comment",
            "content": "New comment",
            "posted_at": "2024-01-01T00:00:00Z",
            "attachment": null,
            "project_id": null,
            "task_id": "task_1"
        })))
        .mount(&mock_server)
        .await;

    let todoist = TodoistWrapper::with_base_url("test-token".to_string(), mock_server.uri());

    let args = CreateCommentArgs {
        content: "New comment".to_string(),
        task_id: Some("task_1".to_string()),
        project_id: None,
        attachment: None,
    };

    let result = todoist.create_comment(&args).await;
    assert!(result.is_ok());
    let comment = result.unwrap();
    assert_eq!(comment.id, "new_comment");
    assert_eq!(comment.content, "New comment");
}

#[tokio::test]
async fn test_update_comment() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/comments/comment_123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "comment_123",
            "content": "Updated comment",
            "posted_at": "2024-01-01T00:00:00Z",
            "attachment": null,
            "project_id": null,
            "task_id": "task_1"
        })))
        .mount(&mock_server)
        .await;

    let todoist = TodoistWrapper::with_base_url("test-token".to_string(), mock_server.uri());

    let args = UpdateCommentArgs {
        content: "Updated comment".to_string(),
    };

    let result = todoist.update_comment("comment_123", &args).await;
    assert!(result.is_ok());
    let comment = result.unwrap();
    assert_eq!(comment.content, "Updated comment");
}

#[tokio::test]
async fn test_delete_comment() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/comments/comment_123"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let todoist = TodoistWrapper::with_base_url("test-token".to_string(), mock_server.uri());

    let result = todoist.delete_comment("comment_123").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_comments_filtered() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/comments"))
        .and(query_param("task_id", "task_123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {
                "id": "comment_1",
                "content": "Task comment",
                "posted_at": "2024-01-01T00:00:00Z",
                "attachment": null,
                "project_id": null,
                "task_id": "task_123"
            }
        ])))
        .mount(&mock_server)
        .await;

    let todoist = TodoistWrapper::with_base_url("test-token".to_string(), mock_server.uri());

    let args = CommentFilterArgs {
        task_id: Some("task_123".to_string()),
        project_id: None,
        limit: None,
        cursor: None,
    };

    let result = todoist.get_comments_filtered(&args).await;
    assert!(result.is_ok());
    let comments = result.unwrap();
    assert_eq!(comments.len(), 1);
    assert_eq!(comments[0].task_id, Some("task_123".to_string()));
}

// ===== ERROR HANDLING TESTS =====

#[tokio::test]
async fn test_rate_limiting() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/tasks"))
        .respond_with(
            ResponseTemplate::new(429)
                .insert_header("Retry-After", "60")
                .set_body_json(json!({
                    "error": "Rate limit exceeded"
                })),
        )
        .mount(&mock_server)
        .await;

    let todoist = TodoistWrapper::with_base_url("test-token".to_string(), mock_server.uri());

    let result = todoist.get_tasks(None, None).await;
    assert!(result.is_err());
    match result {
        Err(TodoistError::RateLimited { retry_after, message }) => {
            assert_eq!(retry_after, Some(60));
            assert!(message.contains("Rate limit exceeded"));
        }
        _ => panic!("Expected RateLimited error"),
    }
}

#[tokio::test]
async fn test_authentication_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/projects"))
        .respond_with(ResponseTemplate::new(401).set_body_json(json!({
            "error": "Unauthorized"
        })))
        .mount(&mock_server)
        .await;

    let todoist = TodoistWrapper::with_base_url("test-token".to_string(), mock_server.uri());

    let result = todoist.get_projects(None, None).await;
    assert!(result.is_err());
    match result {
        Err(TodoistError::AuthenticationError { .. }) => (),
        _ => panic!("Expected AuthenticationError"),
    }
}

#[tokio::test]
async fn test_server_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/labels"))
        .respond_with(ResponseTemplate::new(500).set_body_json(json!({
            "error": "Internal server error"
        })))
        .mount(&mock_server)
        .await;

    let todoist = TodoistWrapper::with_base_url("test-token".to_string(), mock_server.uri());

    let result = todoist.get_labels(None, None).await;
    assert!(result.is_err());
    match result {
        Err(TodoistError::ServerError { status_code, .. }) => {
            assert_eq!(status_code, 500);
        }
        _ => panic!("Expected ServerError"),
    }
}

#[tokio::test]
async fn test_validation_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/tasks"))
        .respond_with(ResponseTemplate::new(400).set_body_json(json!({
            "error": "Invalid content"
        })))
        .mount(&mock_server)
        .await;

    let todoist = TodoistWrapper::with_base_url("test-token".to_string(), mock_server.uri());

    let args = CreateTaskArgs {
        content: "".to_string(),
        ..Default::default()
    };

    let result = todoist.create_task(&args).await;
    assert!(result.is_err());
    match result {
        Err(TodoistError::ValidationError { .. }) => (),
        _ => panic!("Expected ValidationError"),
    }
}
