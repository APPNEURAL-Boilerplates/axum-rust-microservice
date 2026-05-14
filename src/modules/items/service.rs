use super::{dto::CreateItemRequest, model::Item, repository::InMemoryItemRepository};
use crate::{common::error::AppError, events::publisher::EventPublisher};
use serde_json::json;
use uuid::Uuid;

#[derive(Clone)]
pub struct ItemService {
    repository: InMemoryItemRepository,
    events: EventPublisher,
}

impl ItemService {
    pub fn new(repository: InMemoryItemRepository, events: EventPublisher) -> Self {
        Self { repository, events }
    }
    pub async fn list(&self) -> Vec<Item> {
        self.repository.list().await
    }
    pub async fn get(&self, id: Uuid) -> Result<Item, AppError> {
        self.repository
            .get(id)
            .await
            .ok_or_else(|| AppError::not_found("Item"))
    }
    pub async fn create(&self, request: CreateItemRequest) -> Result<Item, AppError> {
        request.validate()?;
        let item = Item {
            id: Uuid::new_v4(),
            name: request.name.trim().to_string(),
            description: request.description,
            price: request.price,
        };
        let item = self.repository.insert(item).await;
        self.events
            .publish("item.created", &json!({ "id": item.id }))
            .await;
        Ok(item)
    }
}
