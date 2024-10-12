#[macro_use]
extern crate rocket;

mod routes;

use std::collections::HashMap;
use rocket::tokio;
use std::sync::{Arc, Mutex};
use rdkafka::ClientConfig;
use rdkafka::consumer::{Consumer, MessageStream, StreamConsumer};
use tokio::sync::RwLock;

type SharedConsumer = Arc<RwLock<StreamConsumer>>;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092") // TODO: load configs from file
        .set("group.id", "server_group")
        .set("enable.auto.commit", "true")
        .set("auto.offset.reset", "latest")
        .create()
        .expect("Failed to create consumer");

    consumer.subscribe(&["device-events"]).expect("Failed to subscribe");

    let shared_consumer = Arc::new(RwLock::new(consumer));

    let rocket = rocket::build()
        .mount("/api", routes![routes::subscribe])
        .manage(shared_consumer)
        .launch()
        .await?;
    Ok(())
}
