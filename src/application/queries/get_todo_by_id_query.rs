use axum::{ extract::Path, http::StatusCode, response::IntoResponse, Json };

use crate::infrastructure::data::repositories::todo_repository::TodoRepository;

pub async fn get_todo_by_id_query(
    Path(todo_id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let repository = TodoRepository::new();
    let id = todo_id.to_string();

    let todo = repository.get_by_id(id).await;
    
    Ok((StatusCode::OK, Json(todo)))
}