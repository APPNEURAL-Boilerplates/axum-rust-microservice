use super::model::Item;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Clone, Default)]
pub struct InMemoryItemRepository {
    store: Arc<RwLock<HashMap<Uuid, Item>>>,
}

impl InMemoryItemRepository {
    pub async fn list(&self) -> Vec<Item> {
        self.store.read().await.values().cloned().collect()
    }
    pub async fn get(&self, id: Uuid) -> Option<Item> {
        self.store.read().await.get(&id).cloned()
    }
    pub async fn insert(&self, item: Item) -> Item {
        self.store.write().await.insert(item.id, item.clone());
        item
    }
}
