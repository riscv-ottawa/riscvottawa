#![recursion_limit = "256"]

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::http::{header, HeaderValue};
    use axum::Router;
    use leptos::config::get_configuration;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use riscvottawa::app::{shell, App};
    use riscvottawa::content::ContentStore;
    use tower::ServiceBuilder;
    use tower_http::compression::CompressionLayer;
    use tower_http::services::ServeDir;
    use tower_http::set_header::SetResponseHeaderLayer;

    let store = match ContentStore::load_from_dir("content") {
        Ok(s) => s,
        Err(e) => {
            eprintln!("fatal: {e}");
            std::process::exit(1);
        }
    };
    println!(
        "loaded {} events, {} projects, {} resources",
        store.events.len(),
        store.projects.len(),
        store.resources.len()
    );

    let conf = get_configuration(None).expect("failed to load leptos configuration");
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    // The bundles under /pkg (wasm/js/css) keep stable filenames: file hashing
    // is not enabled, so the names don't change between builds. That makes them
    // unsafe to cache immutably. A returning visitor would keep an old bundle
    // forever, hydrate it against freshly rendered HTML, hit a mismatch, and end
    // up with a dead page (frozen countdown, unresponsive menu) until a hard
    // refresh. `no-cache` lets the browser store the bundle but revalidate on
    // every load; ServeDir replies with a cheap 304 while the file is unchanged
    // and serves the new bundle the moment a deploy changes it.
    let pkg_dir = format!(
        "{}/{}",
        leptos_options.site_root, leptos_options.site_pkg_dir
    );
    let pkg_service = ServiceBuilder::new()
        .layer(SetResponseHeaderLayer::overriding(
            header::CACHE_CONTROL,
            HeaderValue::from_static("no-cache"),
        ))
        .service(ServeDir::new(pkg_dir));

    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            {
                let store = store.clone();
                move || provide_context(store.clone())
            },
            {
                let options = leptos_options.clone();
                move || shell(options.clone())
            },
        )
        .nest_service("/pkg", pkg_service)
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options)
        // Compress SSR HTML and assets on the fly (Brotli/gzip via content
        // negotiation). Self-contained so it works behind any proxy, or none;
        // a proxy that already compresses will pass an encoded response through.
        .layer(CompressionLayer::new());

    println!("listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("failed to bind site address");
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("axum server error");
}

#[cfg(feature = "ssr")]
async fn shutdown_signal() {
    use tokio::signal;

    let ctrl_c = async {
        signal::ctrl_c().await.expect("install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // This binary only runs under the `ssr` feature. The `hydrate` feature
    // produces a wasm cdylib whose entrypoint is `hydrate()` in lib.rs.
}
