mod custom_error;
mod file_handler;
mod httpserver;
mod jobs;
mod messenger;
mod settings;
mod telemetry;

use std::sync::mpsc;

// pretty_env_logger related
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

fn main() {
    pretty_env_logger::init();
    info!("Starting fota service");

    // Create channel for passing notification from messenger to jobs thread
    let (tx_notification, rx_notification) = mpsc::channel();
    // Create channel for passing new job from http server to jobs thread
    let (tx_new_job, rx_new_job) = mpsc::channel();

    // Initialize messenger, it already handle mqtt connection on other thread
    let messenger = messenger::Messenger::new(tx_notification);

    // Initialize jobs and run
    let jobs = jobs::JobScheduler::new(messenger, rx_notification, rx_new_job);
    jobs.run();

    let mut http = httpserver::HTTPServer::new(tx_new_job);
    http.run();
}
