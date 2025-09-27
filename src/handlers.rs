use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde_json::{json, Value};
use std::collections::HashMap;
use uuid::Uuid;

use crate::models::{CreateItemRequest, Item, ItemResponse};

// In-memory storage for demo purposes
// In a real application, you'd use a database
type AppState = HashMap<Uuid, Item>;

pub async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "message": "Rust Practice API is running!",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

pub async fn create_item(
    Json(payload): Json<CreateItemRequest>,
) -> Result<(StatusCode, Json<ItemResponse>), StatusCode> {
    let item = Item {
        id: Uuid::new_v4(),
        name: payload.name,
        description: payload.description,
        price: payload.price,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let response = ItemResponse {
        id: item.id,
        name: item.name.clone(),
        description: item.description.clone(),
        price: item.price,
        created_at: item.created_at,
        updated_at: item.updated_at,
    };

    // In a real app, you'd save to database here
    tracing::info!("Created item: {:?}", item);

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn get_items() -> Json<Value> {
    // In a real app, you'd fetch from database
    Json(json!({
        "items": [],
        "count": 0,
        "message": "No items found. Use POST /api/items to create some!"
    }))
}

pub async fn get_item_by_id(
    Path(id): Path<Uuid>,
) -> Result<Json<ItemResponse>, StatusCode> {
    // In a real app, you'd fetch from database
    Err(StatusCode::NOT_FOUND)
}