use prometheus::{Encoder, TextEncoder, IntCounter, Registry};
use std::sync::Arc;
use warp::Filter;

pub struct Metrics {
    pub block_count: IntCounter,
    pub transaction_count: IntCounter,
}

impl Metrics {
    pub fn new() -> Self {
        let block_count = IntCounter::new("block_count", "Number of blocks mined").unwrap();
        let transaction_count = IntCounter::new("transaction_count", "Number of transactions processed").unwrap();
        Metrics {
            block_count,
            transaction_count,
        }
    }

    pub fn register(&self, registry: &Registry) {
        registry.register(Box::new(self.block_count.clone())).unwrap();
        registry.register(Box::new(self.transaction_count.clone())).unwrap();
    }
}

pub async fn serve_metrics(registry: Arc<Registry>) {
    let metrics_route = warp::path!("metrics").map(move || {
        let encoder = TextEncoder::new();
        let mut buffer = Vec::new();
        let metric_families = registry.gather();
        encoder.encode(&metric_families, &mut buffer).unwrap();
        warp::reply::with_header(buffer, "Content-Type", encoder.format_type())
    });

    warp::serve(metrics_route).run(([127, 0, 0, 1], 9090)).await;
} 