use serde::Serialize;

#[derive(Clone, Default)]
pub struct EventPublisher;

impl EventPublisher {
    pub async fn publish<T: Serialize + ?Sized>(&self, topic: &str, payload: &T) {
        let payload =
            serde_json::to_string(payload).unwrap_or_else(|_| "<unserializable>".to_string());
        tracing::info!(topic, %payload, "event published");
    }
}
