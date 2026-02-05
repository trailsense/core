use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer};

pub fn setup_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "info,opentelemetry=debug".parse().unwrap());

    let fmt_layer = tracing_subscriber::fmt::layer()
        .event_format(
            tracing_subscriber::fmt::format()
                .with_target(false)
                .with_level(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_timer(tracing_subscriber::fmt::time::ChronoLocal::rfc_3339()),
        )
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_filter(filter);

    tracing_subscriber::registry().with(fmt_layer).init();
}
