use multithread_web_server::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use serde::Deserialize;


#[derive(Deserialize)]
struct Config {
    port: u16,
    thread_pool_size: usize,
}

fn main() {
    let config = read_config("config.json").expect("Failed to read config file");

    let address = format!("127.0.0.1:{}", config.port);
    
    let listener = TcpListener::bind(address).unwrap();
    let pool = ThreadPool::new(config.thread_pool_size);

    println!("Server running on port {}", config.port);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();


        pool.execute(move || {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn read_config(filename: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let config_content = fs::read_to_string(filename)?;
    let config: Config = serde_json::from_str(&config_content)?;
    Ok(config)
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}