#[derive(Clone, Default)]
pub struct ExampleWorker;

impl ExampleWorker {
    pub async fn tick(&self) {
        tracing::debug!("example worker tick");
    }
}
