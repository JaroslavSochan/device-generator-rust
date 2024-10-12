# Device Data Streaming Solution

This project simulates IoT devices sending telemetry data to a server using Kafka as a message broker. The solution consists of two main components:
1. **Simulator**: Simulates devices that generate data events.
2. **Server**: Provides an API for clients to subscribe to device data streams.

## Overview

The solution streams data from simulated devices to customers in real time using the following components:

- **Simulator**: Mimics devices with IDs 1, 2, and 3, sending various events:
    - **Startup**: Sent once when the device starts.
    - **Heartbeat**: Sent every 15 seconds with random `cpu_usage` and `mem_usage`.
    - **Temperature**: Sent every 3 seconds with a random temperature value.

- **Kafka**: Acts as a message broker. The simulator publishes messages to Kafka, and the server consumes these messages for further processing.

- **Server**: Listens to Kafka for incoming device events and provides main API endpoint for customers to subscribe to data from specific devices.

## Architecture

1. **Simulator** ➜ **Kafka** ➜ **Server** ➜ **Clients**

    - The **Simulator** generates events and sends them to Kafka.
    - The **Server** listens to Kafka and forwards relevant events to subscribed clients through an HTTP API.
    - **Clients** can subscribe to a particular device's events using the provided API.

## Components

### 1. Simulator

The **simulator** is responsible for simulating device events. It generates:
- **Startup** events once at startup for each device.
- **Heartbeat** events every 15 seconds, with random values for `cpu_usage` and `mem_usage`.
- **Temperature** events every 3 seconds, with a random temperature value.

### 2. Server

The **server** provides an HTTP API for clients to subscribe to device data streams. It listens for messages published by the simulator to Kafka and streams data to clients based on their subscriptions.

### 3. Kafka

**Kafka** serves as a message broker, decoupling the simulator from the server. Messages from the simulator are sent to a Kafka topic (`device-events`), which the server subscribes to.

## API Endpoints

The server provides the following API endpoints:

### POST `/api/devices/{id}/messages`

- **Description**: Allows clients to subscribe to events from a specific device (based on its ID). Clients receive a real-time stream of JSON data.
- **Request Example**:
    ```http
    POST /api/devices/1/messages
    ```
- **Response Example**:
    ```json
    { 
    "id": 0, 
    "event":
      { 
        "cpu_usage": "79.12426541566022", 
        "mem_usage": "10.751320968737698" 
      }
    }
    ```

## How It Works

1. **Simulator**:
    - Initializes three devices (IDs 1, 2, and 3).
    - Sends **Startup** events once per device when the simulator starts.
    - Periodically sends **Heartbeat** and **Temperature** events with random values.
    - Publishes events to the `device-events` topic on Kafka.

2. **Server**:
    - Uses a **Kafka consumer** to subscribe to the `device-events` topic.
    - Streams relevant data to clients based on their subscriptions to specific device IDs.
    - Utilizes **broadcast channels** to efficiently send events to multiple subscribed clients.

3. **Clients**:
    - Subscribe to the server's endpoint (`/api/devices/{id}/messages`).
    - Receive real-time updates on device events as JSON streams.

## Dependencies

### Rust Dependencies

- `rdkafka`: For Kafka communication.
- `tokio`: For asynchronous programming.
- `rocket`: For building the server API.
- `serde`, `serde_json`: For serialization and deserialization.
- `rand`: For generating random values.