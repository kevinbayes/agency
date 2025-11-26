mod config;
mod handler;
mod error;
pub mod a2a;
mod cli;
mod common;

use std::collections::HashSet;
use std::{env, thread};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::time::Duration;
use axum::{Json, middleware, Router};
use axum::http::{Method, Uri};
use axum::routing::get;
use axum::response::Response;
use axum::response::IntoResponse;
use clap::{Arg, ArgMatches, Command};
use serde_json::json;
use tower_http::services::ServeDir;
use tracing::info;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use sqlx::{MySql, Pool};
use uuid::Uuid;
use crate::config::config::{read_json_config,};
use crate::common::jwks_supplier::JwksReadThroughCache;
use crate::error::{LocalError, LocalResult};
use crate::handler::actuator::actuator_routes;
use crate::handler::auth_middleware::{auth_middleware, AuthState};
use env_logger::{Logger, Env};
use log::debug;
use crate::cli::agent::interact::{interact_agent, start_agent, stop_agent};
use crate::cli::agent::sandbox::{create_sandbox, sync_sandbox};
use crate::cli::agent_command::{create_agents_command};
use crate::cli::agent_command_handler::sandbox_command_handler;
use crate::cli::worker_command::create_worker_command;

#[tokio::main]
async fn main() {

    env_logger::init();
    cli().await;
}

async fn cli() {

    let matches = Command::new("Agency Cli")
        .version("1.0")
        .about("Agency helps manage your agents")
        .next_line_help(true)
        .subcommand_required(false)
        .subcommand(create_agents_command())
        .subcommand(create_worker_command())
        .get_matches();

    match matches.clone().subcommand() {
        Some(("agents", cli_command)) => {
            debug!("enter: agent");
            handle_agents_command(matches, cli_command).await;
        },
        Some(("worker", cli_command)) => {
            debug!("enter: worker");
            handle_worker_command(matches, Some(cli_command)).await;
        },
        _ => {
            println!("fallback: unknown command");
        }
    }
}
async fn handle_agents_command(matches: ArgMatches, cli_command: &ArgMatches) {

    match cli_command.clone().subcommand() {
        Some(("init", cli_command)) => {
            debug!("enter: agent");
            create_sandbox().unwrap()
        },
        Some(("start", sub_command)) => {
            debug!("enter: start agent");
            start_agent(sub_command).unwrap();
        },
        Some(("interact", sub_command)) => {
            debug!("enter: agent");
            interact_agent().unwrap();
        },
        Some(("stop", sub_command)) => {
            debug!("enter: agent");
            stop_agent().unwrap();
        },
        Some(("sandbox", sub_command)) => {
            debug!("enter: agent");
            sandbox_command_handler(matches, cli_command, sub_command);
        },
        _ => {
            println!("fallback: unknown command");
        }
    }
}

async fn handle_worker_command(p0: ArgMatches, p1: Option<&ArgMatches>) {

    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();

    let value = env::var("CONFIG_LOCATION").unwrap_or_else(|_| "./config/default.json".to_string());
    let all_configuration = read_json_config(&value).unwrap();
    let host_config = all_configuration.clone().host.unwrap();

    let app = Router::new()
        .merge(actuator_routes())
        .layer(middleware::map_response(main_response_mapper))
        .fallback_service(static_routes())
        ;

    let addr = format!("{}:{}", host_config.host, host_config.port);
    println!("Starting service on {}.", addr);
    let listener = tokio::net::TcpListener::bind(addr.as_str())
        .await
        .unwrap();
    println!("listening on {}", addr.as_str());
    tracing::warn!("listening on {}", addr.as_str());
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
        .await
        .unwrap();
}

async fn main_response_mapper(
    res: Response,
) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    let uuid = Uuid::new_v4();

    // -- Get the eventual response error.
    let service_error = res.extensions().get::<LocalError>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    let error_response =
        client_status_error
            .as_ref()
            .map(|(status_code, client_error)| {
                let client_error_body = json!({
    				"error": {
    					"type": client_error.as_ref(),
    					"req_uuid": uuid.to_string(),
    				}
    			});

                println!("    ->> client_error_body: {client_error_body}");

                // Build the new response from the client_error_body
                (*status_code, Json(client_error_body)).into_response()
            });

    error_response.unwrap_or(res)
}

fn static_routes ()-> Router {
    let web_location = env::var("WEB_LOCATION").unwrap_or_else(|_| "./web/dist".to_string());
    Router::new().fallback_service(ServeDir::new(web_location))
}

