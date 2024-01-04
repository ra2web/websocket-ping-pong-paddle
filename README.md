# WebSocket Ping-Pong Paddle

[中文版文档](README-CN.md)

## Introduction
WebSocket Ping-Pong Paddle is a simple yet powerful tool, written in Rust, that provides real-time, bi-directional communication between a server and client. It's designed to handle WebSocket Ping and Pong frames for maintaining the WebSocket connections and ensuring their liveliness.

This is an excellent project for Rust beginners to learn about network communication based on WebSocket. Also a ra2web study program.

## Features
- Listens for incoming WebSocket connections.
- Responds to Ping frames with Pong frames.
- Returns non-standard Ping messages as is.
- Uses the asynchronous programming model, making efficient use of system resources.

## Use Cases
This tool can be used for:
- Debugging WebSocket connections locally or remotely.
- Testing the liveliness of WebSocket connections by regularly sending Ping frames and expecting Pong frames.
- Reflecting non-standard Ping messages, which can be used for custom communication protocols over WebSocket.

## Future Prospects
As the project evolves, we aim to add more features, such as:
- Customizable Ping and Pong messages.
- Logging and metrics for connection health and performance.
- Support for secure WebSocket connections (wss://).

## Getting Started
To use the WebSocket Ping-Pong Paddle, follow these steps:

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/websocket-ping-pong-paddle.git
   ```

2. Navigate to the project directory:
   ```bash
   cd websocket-ping-pong-paddle
   ```

3. Run the program with default settings (listening on 0.0.0.0:11451):
   ```bash
   cargo run
   ```

   Or specify a custom address and port:
   ```bash
   cargo run 127.0.0.1 12345
   ```

## Contributing
Contributions are welcome! If you encounter any issues or have suggestions for improvements, please open an issue or submit a pull request on the GitHub repository.

## License
This project is licensed under the [Apache License 2.0](LICENSE).
