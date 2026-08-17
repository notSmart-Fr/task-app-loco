use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use crate::models::_entities::tasks::{self, Entity as Task};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateTaskParams {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateTaskParams {
    pub title: Option<String>,
    pub description: Option<String>,
    pub is_completed: Option<bool>,
}

/// GET /tasks - List all tasks
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    let tasks = Task::find().all(&ctx.db).await?;
    format::json(tasks)
}

/// POST /tasks - Create a new task
pub async fn create(
    State(ctx): State<AppContext>,
    Json(params): Json<CreateTaskParams>,
) -> Result<Response> {
    let item = tasks::ActiveModel {
        title: ActiveValue::Set(params.title),
        description: ActiveValue::Set(params.description),
        is_completed: ActiveValue::Set(false),
        ..Default::default()
    };
    let task = item.insert(&ctx.db).await?;
    format::json(task)
}

/// GET /tasks/{id} - Get single task by ID
pub async fn get_one(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let task = Task::find_by_id(id).one(&ctx.db).await?;
    match task {
        Some(task) => format::json(task),
        None => format::empty(), // Returns 404/Empty if not found
    }
}

/// PUT /tasks/{id} - Update a task
pub async fn update(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<UpdateTaskParams>,
) -> Result<Response> {
    let task = Task::find_by_id(id).one(&ctx.db).await?;
    let mut item: tasks::ActiveModel = match task {
        Some(t) => t.into(),
        None => return format::empty(),
    };

    if let Some(title) = params.title {
        item.title = ActiveValue::Set(title);
    }
    if let Some(description) = params.description {
        item.description = ActiveValue::Set(Some(description));
    }
    if let Some(is_completed) = params.is_completed {
        item.is_completed = ActiveValue::Set(is_completed);
    }

    let updated = item.update(&ctx.db).await?;
    format::json(updated)
}

/// DELETE /tasks/{id} - Remove a task
pub async fn remove(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let task = Task::find_by_id(id).one(&ctx.db).await?;
    if let Some(t) = task {
        t.delete(&ctx.db).await?;
    }
    format::empty()
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("tasks")
        .add("/", get(list))
        .add("/", post(create))
        .add("/{id}", get(get_one))
        .add("/{id}", put(update))
        .add("/{id}", delete(remove))
}