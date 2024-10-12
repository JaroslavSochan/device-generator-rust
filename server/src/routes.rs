use crate::SharedConsumer;

use rdkafka::consumer::{Consumer, MessageStream, StreamConsumer};
use rdkafka::error::KafkaError;
use rdkafka::message::{BorrowedMessage, Message};
use rdkafka::ClientConfig;
use rocket::async_stream::stream;
use rocket::futures::future::ok;
use rocket::futures::{SinkExt, StreamExt};
use rocket::response::stream::{Event, EventStream};
use rocket::State;
use serde::de::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::id;
use std::ptr::null;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use rocket::sentinel::resolution::DefaultSentinel;
use rocket::time::Duration;
use tokio::sync::broadcast::Sender;
use tokio::sync::mpsc;

#[derive(Debug, Clone, Deserialize, Serialize)] //Create separated file for each struct
pub struct Temperature {
    pub temp: f64,
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Heartbeat {
    pub cpu_usage: f64,
    pub mem_usage: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum DeviceEvent {
    Startup,
    Heartbeat(Heartbeat),
    Temperature(Temperature),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Device {
    pub id: u8,
    pub event: DeviceEvent,
}

#[post("/devices/<id>/messages")]
pub async fn subscribe(id: u8, consumer: &State<SharedConsumer>) -> EventStream![Event + '_] {
    EventStream! {

    let mut consumer_cloned = consumer.clone().read().await;
    let mut stream = consumer_cloned.stream();
    while let Some(message_result) = stream.next().await {
        match message_result {
            Ok(message) => {
                if let Some(payload) = message.payload() {
                    if let Ok(msg_str) = std::str::from_utf8(payload) {
                        if let Ok(kafka_message) = serde_json::from_str::<Device>(msg_str) {
                            if kafka_message.id == id {
                                yield Event::json(&kafka_message).event("temperature");
                            }
                        }
                    }
                }
            }
            Err(e) => eprintln!("Kafka error: {}", e)
        }
    }
    }
}
