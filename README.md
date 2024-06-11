# Multithreaded Web Server in Rust

## Table of Contents
- [Multithreaded Web Server in Rust](#multithreaded-web-server-in-rust)
  - [Table of Contents](#table-of-contents)
  - [Introduction](#introduction)
  - [Features](#features)
  - [Requirements](#requirements)
  - [Installation](#installation)
  - [Usage](#usage)
  - [Configuration](#configuration)
  - [Testing](#testing)
  - [Performance](#performance)
  - [License](#license)

## Introduction

This project is a multithreaded web server written in Rust, based on the concepts and examples from the Rust Programming Language book. The server is designed to handle multiple client requests concurrently, ensuring efficient and responsive web services.

## Features

- **Multithreading**: Efficient handling of concurrent requests using a thread pool.
- **HTTP/1.1 Support**: Basic implementation of the HTTP/1.1 protocol.
- **Static File Serving**: Serves static files from a configurable directory.
- **Graceful Shutdown**: Properly handles shutdown signals to clean up resources.
- **Configurable**: Easy configuration of server parameters like port number and thread pool size.

## Requirements

- **Rust**: Ensure you have Rust installed. You can download it from [rust-lang.org](https://www.rust-lang.org/).

## Installation

1. **Clone the Repository**
    ```sh
    git clone https://github.com/yourusername/multithreaded-web-server.git
    cd multithreaded-web-server
    ```

2. **Build the Project**
    ```sh
    cargo build --release
    ```

## Usage

1. **Starting the Server**
    ```sh
    cargo run --release
    ```

2. **Accessing the Server**
    Open a web browser and navigate to `http://localhost:7878` (or the configured port).

## Configuration

Configuration options can be set in the `config.json` file located in the root directory. The available settings include:

- **port**: Port number on which the server will listen (default: `7878`)
- **thread_pool_size**: Number of threads in the thread pool (default: `4`)
- **document_root**: Root directory for serving static files (default: `./www`)

Example `config.json`:
```json
{
    "port": 7878,
    "thread_pool_size": 4,
    "document_root": "./www"
}
```

## Testing

To test the server's functionality and performance, you can use tools like `curl` or `ApacheBench`.

Example using `curl`:
```sh
curl http://localhost:7878/index.html
```

Example using `ApacheBench`:
```sh
ab -n 1000 -c 10 http://localhost:7878/index.html
```

## Performance

The performance of the server can be monitored by observing the request handling times and server response times. Rust's efficient memory management and concurrency model ensure that the server performs well under load.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

Feel free to contribute to the project by opening issues or submitting pull requests. Your feedback and contributions are welcome!