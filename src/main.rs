#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::config::get_configuration;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use riscvottawa::app::{shell, App};
    use riscvottawa::content::ContentStore;

    let store = match ContentStore::load_from_dir("content") {
        Ok(s) => s,
        Err(e) => {
            eprintln!("fatal: {e}");
            std::process::exit(1);
        }
    };
    println!(
        "loaded {} events, {} trainings",
        store.events.len(),
        store.trainings.len()
    );

    let conf = get_configuration(None).expect("failed to load leptos configuration");
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

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
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    println!("listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("failed to bind site address");
    axum::serve(listener, app.into_make_service())
        .await
        .expect("axum server error");
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // This binary only runs under the `ssr` feature. The `hydrate` feature
    // produces a wasm cdylib whose entrypoint is `hydrate()` in lib.rs.
}
