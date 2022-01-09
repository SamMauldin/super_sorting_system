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
use actix_web::{middleware, web, App, HttpServer};
use thiserror::Error;

use crate::{
    pathfinding::{verify_pathfinding_config, PathfindingError},
    services::{
        agent_expiration::AgentExpirationService, defragger::DefraggerService,
        hold_expiration::HoldExpirationService, scanner::ScannerService, service::Service,
    },
    state::StateData,
};

#[derive(Error, Debug)]
enum StartupError {
    #[error("Failed to read configuration")]
    FigmentError(figment::Error),
    #[error(transparent)]
    CreateServerError(std::io::Error),
    #[error(transparent)]
    PathfindingConfigError(PathfindingError),
}

#[actix_web::main]
async fn main() -> Result<(), StartupError> {
    env_logger::init();
    info!("Hello from Super Sorting System's Operator");

    let config = web::Data::new(config::read_config().map_err(StartupError::FigmentError)?);

    verify_pathfinding_config(&config).map_err(|err| StartupError::PathfindingConfigError(err))?;

    let state: StateData = web::Data::new(Mutex::new(Default::default()));

    let bg_state = state.clone();
    let bg_config = config.clone();

    thread::spawn(move || {
        let state = bg_state.clone();
        let config = bg_config.clone();

        let mut scanner_service = ScannerService::new(&config);
        let mut agent_expiration_service = AgentExpirationService::new(&config);
        let mut defragger_service = DefraggerService::new(&config);
        let mut hold_expiration_service = HoldExpirationService::new(&config);

        loop {
            thread::sleep(Duration::from_millis(1000));
            let mut state = state.lock().unwrap();

            scanner_service.tick(&mut state);
            agent_expiration_service.tick(&mut state);
            defragger_service.tick(&mut state);
            hold_expiration_service.tick(&mut state);
        }
    });

    let app_config = config.clone();
    let app_state = state.clone();

    HttpServer::new(move || {
        App::new()
            .app_data(app_config.clone())
            .app_data(app_state.clone())
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::trim())
            .wrap(Cors::permissive())
            .configure(|serv_config| api::agent_service::configure(serv_config, &app_config))
            .configure(|serv_config| api::admin_service::configure(serv_config, &app_config))
            .configure(|serv_config| api::automation_service::configure(serv_config, &app_config))
            .configure(|serv_config| api::data_service::configure(serv_config, &app_config))
    })
    .bind((config.host.as_str(), config.port))
    .map_err(StartupError::CreateServerError)?
    .run()
    .await
    .map_err(StartupError::CreateServerError)
}
