mod device;
use device::Device;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use tokio::time::{self, Duration};
use serde_json::to_string;

#[tokio::main]
async fn main() {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092") // TODO: load it from config file
        .create()
        .expect("Producer creation error");

    let mut devices = Vec::<Device>::new();
    for id in 0..3 { // Number of devices
        devices.push(Device::new(id));
    }

    // Startup
    for device in &mut devices {

        device.event(Device::startup_message());
        let startup_event = to_string(&device).unwrap();
        producer.send(
            FutureRecord::to("device-events").payload(&startup_event).key(&device.id.to_string()),
            Duration::from_secs(0),
        ).await.unwrap();
        println!("Sent startup message for device {:?}", device);
    }

    let mut heartbeat_interval = time::interval(Duration::from_secs(15));
    let mut temperature_interval = time::interval(Duration::from_secs(3));

    loop {
        tokio::select! {
            // Heartbeat
            _ = heartbeat_interval.tick() => {
                for device in &mut devices {
                    let event = Device::generate_heartbeat();
                    device.event(event);
                    let json_event = to_string(&device).unwrap();
                    producer.send(
                        FutureRecord::to("device-events").payload(&json_event).key(&device.id.to_string()),
                        Duration::from_secs(0),
                    ).await.unwrap();
                    println!("Sent heartbeat for device {:?}", device);
                }
            }
            // Temperature
            _ = temperature_interval.tick() => {
                for device in  &mut devices {
                    let event = Device::generate_temperature();
                    device.event(event);
                    let json_event = to_string(&device).unwrap();
                    producer.send(
                        FutureRecord::to("device-events").payload(&json_event).key(&device.id.to_string()),
                        Duration::from_secs(0),
                    ).await.unwrap();
                    println!("Sent temperature for device {:?}", device);
                }
            }
        }
    }
}
