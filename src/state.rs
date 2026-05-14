use std::{sync::Arc, time::Instant};

use crate::{
    clients::http_client::HttpClient,
    config::AppConfig,
    events::publisher::EventPublisher,
    modules::items::{repository::InMemoryItemRepository, service::ItemService},
};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub started_at: Instant,
    pub items: ItemService,
    pub http_client: HttpClient,
    pub events: EventPublisher,
}

impl AppState {
    pub fn new(config: AppConfig) -> Self {
        let events = EventPublisher;
        let items = ItemService::new(InMemoryItemRepository::default(), events.clone());
        Self {
            config: Arc::new(config),
            started_at: Instant::now(),
            items,
            http_client: HttpClient::default(),
            events,
        }
    }

    pub fn uptime_seconds(&self) -> u64 {
        self.started_at.elapsed().as_secs()
    }
}
