#![feature(drain_filter)]
#[macro_use]
extern crate log;

mod api;
mod config;
mod pathfinding;
mod services;
mod state;
mod stats;
mod types;

use std::{sync::Mutex, thread, time::Duration};

use actix_cors::Cors;
use actix_web::{guard, middleware, web, App, HttpServer};
use thiserror::Error;
use uuid::Uuid;

use crate::{
    services::{
        agent_expiration::AgentExpirationService, defragger::DefraggerService,
        hold_expiration::HoldExpirationService, inventory_scanner::InventoryScannerService,
        node_scanner::NodeScannerService, service::Service,
    },
    state::StateData,
};

#[derive(Error, Debug)]
enum StartupError {
    #[error("Failed to read configuration")]
    FigmentError(figment::Error),
    #[error(transparent)]
    CreateServerError(std::io::Error),
}

#[actix_web::main]
async fn main() -> Result<(), StartupError> {
    env_logger::init();
    info!("Hello from Super Sorting System's Operator");

    let config = web::Data::new(config::read_config().map_err(StartupError::FigmentError)?);

    let state: StateData = web::Data::new(Mutex::new(Default::default()));

    let bg_state = state.clone();
    let bg_config = config.clone();

    thread::spawn(move || {
        let state = bg_state.clone();
        let config = bg_config.clone();

        let mut inventory_scanner_service = InventoryScannerService::new(&config);
        let mut agent_expiration_service = AgentExpirationService::new(&config);
        let mut defragger_service = DefraggerService::new(&config);
        let mut hold_expiration_service = HoldExpirationService::new(&config);
        let mut node_scanner_service = NodeScannerService::new(&config);

        loop {
            thread::sleep(Duration::from_millis(1000));
            let mut state = state.lock().unwrap();

            inventory_scanner_service.tick(&mut state);
            agent_expiration_service.tick(&mut state);
            defragger_service.tick(&mut state);
            hold_expiration_service.tick(&mut state);
            node_scanner_service.tick(&mut state);
        }
    });

    let app_config = config.clone();
    let app_state = state.clone();

    let server = HttpServer::new(move || {
        let app_config_guard = app_config.clone();

        App::new()
            .app_data(app_config.clone())
            .app_data(app_state.clone())
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::trim())
            .wrap(Cors::permissive())
            .service(
                web::scope("")
                    .guard(guard::fn_guard(move |req| {
                        req.head()
                            .headers()
                            .get("X-Api-Key")
                            .and_then(|header| header.to_str().ok())
                            .and_then(|header| Uuid::parse_str(header).ok())
                            .map(|header| app_config_guard.api_keys.contains(&header))
                            .unwrap_or(false)
                    }))
                    .configure(api::agent_service::configure)
                    .configure(api::admin_service::configure)
                    .configure(api::automation_service::configure)
                    .configure(api::data_service::configure),
            )
    })
    .bind((config.host.as_str(), config.port))
    .map_err(StartupError::CreateServerError)?;
    info!("Bound to {:?}", (config.host.as_str(), config.port));
    server.run().await.map_err(StartupError::CreateServerError)
}
