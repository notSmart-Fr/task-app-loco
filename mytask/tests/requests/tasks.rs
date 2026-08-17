use loco_rs::testing::prelude::*;
use mytask::app::App; // Replace `mytask` with your crate name from Cargo.toml
use serde_json::json;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn can_create_and_list_tasks() {
    request::<App, _, _>(|request, _ctx| async move {
        // 1. Create a task via POST /tasks
        let res = request
            .post("/tasks")
            .json(&json!({
                "title": "Write unit tests",
                "description": "Cover all CRUD operations"
            }))
            .await;

        assert_eq!(res.status_code(), 200);
        assert!(res.text().contains("Write unit tests"));

        // 2. Fetch all tasks via GET /tasks
        let res = request.get("/tasks").await;
        assert_eq!(res.status_code(), 200);
        assert!(res.text().contains("Write unit tests"));
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_filter_tasks_by_completion() {
    request::<App, _, _>(|request, _ctx| async move {
        // Create one completed task and one pending task
        request
            .post("/tasks")
            .json(&json!({"title": "Pending Task"}))
            .await;

        let res = request
            .post("/tasks")
            .json(&json!({"title": "Completed Task"}))
            .await;
        
        let created: serde_json::Value = res.json();
        let task_id = created["id"].as_i64().unwrap();

        // Mark second task as completed
        request
            .put(&format!("/tasks/{task_id}"))
            .json(&json!({"is_completed": true}))
            .await;

        // Query only completed tasks
        let res = request.get("/tasks?is_completed=true").await;
        assert!(res.text().contains("Completed Task"));
        assert!(!res.text().contains("Pending Task"));
    })
    .await;
}