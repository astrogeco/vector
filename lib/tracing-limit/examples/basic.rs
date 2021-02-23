#[macro_use]
extern crate tracing;

use tracing::Dispatch;
use tracing_limit::RateLimitedLayer;
use tracing_subscriber::layer::SubscriberExt;

fn main() {
    let subscriber = tracing_subscriber::registry::Registry::default()
        .with(tracing_subscriber::filter::EnvFilter::from("trace"))
        .with(RateLimitedLayer::new(
            tracing_subscriber::fmt::Layer::default().without_time(),
        ));

    let dispatch = Dispatch::new(subscriber);

    tracing::dispatcher::with_default(&dispatch, || {
        // This should print every 2 events
        for i in 0..40 {
            info!(
                message = "Hello, world!",
                count = &i,
                internal_log_rate_secs = 5
            );
            trace!("This field is not rate limited!");
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    })
}
