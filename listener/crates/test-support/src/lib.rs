use tokio::sync::OnceCell;

static REDIS_URL: OnceCell<String> = OnceCell::const_new();

/// Lazily start a single Redis container shared across all tests.
///
/// `ReuseDirective::CurrentSession` prevents the `Drop` impl from stopping
/// the container when the `ContainerAsync` handle goes out of scope at the
/// end of the `OnceCell` init closure. The container therefore stays alive
/// for all tests in the suite. Cleanup is handled by `make test-e2e-*`
/// targets via `docker rm -f e2e-redis`.
pub async fn shared_redis_url() -> &'static str {
    REDIS_URL
        .get_or_init(|| async {
            use testcontainers::core::{ImageExt, ReuseDirective};
            use testcontainers::runners::AsyncRunner;
            use testcontainers_modules::redis::Redis;

            let container = Redis::default()
                .with_container_name("e2e-redis")
                .with_tag("6.2.0")
                .with_reuse(ReuseDirective::CurrentSession)
                .start()
                .await
                .expect("failed to start Redis container — is Docker running?");
            let port = container
                .get_host_port_ipv4(6379)
                .await
                .expect("failed to get Redis container port");
            format!("redis://127.0.0.1:{port}")
        })
        .await
        .as_str()
}
