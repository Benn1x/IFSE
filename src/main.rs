#![deny(warnings)]

use log::info;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::Appender;
use log4rs::config::Root;
use log4rs::Config;
use std::path::Path;

pub mod api;
pub mod config;
pub mod database;
pub mod file_system;
pub mod search_engine;

// TODO Config System, still need to decide what kinda config
fn main() {
    let stdout = match FileAppender::builder().build(Path::new("log.log")) {
        Ok(res) => res,
        Err(_) => panic!("Err initializing the Logging file"),
    };
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        .unwrap();
    let _handle = log4rs::init_config(config).unwrap();

    // load config
    info!("Successfully loaded the config");

    let engine = search_engine::search_engine::Engine::new(String::from(""));
    info!("Successfully started Search-Engine");

    let mut api = api::api::API::new(engine);
    info!("Successfully started API");
    api.search();
}
