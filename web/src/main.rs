mod handlers;
mod renderer;

use crate::renderer::MiniJinjaRenderer;
use actix_files as fs;
use actix_web::{
    http::StatusCode,
    middleware::{ErrorHandlers, Logger},
    web, App, HttpServer,
};
use dotenvy::dotenv;
use minijinja_autoreload::AutoReloader;
use std::path::PathBuf;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    pretty_env_logger::init();

    // If TEMPLATE_AUTORELOAD is set, then the path tracking is enabled.
    let enable_template_autoreload = std::env::var("TEMPLATE_AUTORELOAD").as_deref() == Ok("true");

    if enable_template_autoreload {
        log::info!("template auto-reloading is enabled");
    } else {
        log::info!(
            "template auto-reloading is disabled; run with TEMPLATE_AUTORELOAD=true to enable"
        );
    }

    // The closure is invoked every time the environment is outdated to recreate it.
    let tmpl_reloader = AutoReloader::new(move |notifier| {
        let mut env: minijinja::Environment<'static> = minijinja::Environment::new();

        let tmpl_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("templates");

        // if watch_path is never called, no fs watcher is created
        if enable_template_autoreload {
            notifier.watch_path(&tmpl_path, true);
        }

        env.set_source(minijinja::Source::from_path(tmpl_path));

        Ok(env)
    });

    let tmpl_reloader = web::Data::new(tmpl_reloader);

    log::info!("starting HTTP server");

    HttpServer::new(move || {
        App::new()
            .app_data(tmpl_reloader.clone())
            .service(fs::Files::new("/static", "static").show_files_listing())
            .service(web::resource("/40dex").route(web::get().to(handlers::index)))
            .service(fs::Files::new("/", "static/dist").index_file("index.html"))
            .wrap(ErrorHandlers::new().handler(StatusCode::NOT_FOUND, handlers::not_found))
            .wrap(Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
