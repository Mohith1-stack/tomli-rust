use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

// Embed the static files at compile time!
const INDEX_HTML: &str = include_str!("../static/index.html");
const STYLE_CSS: &str = include_str!("../static/style.css");
const MAIN_JS: &str = include_str!("../static/main.js");
const WASM_JS: &str = include_str!("../static/pkg/tomli_wasm.js");
const WASM_BG: &[u8] = include_bytes!("../static/pkg/tomli_wasm_bg.wasm");

fn main() {
    let port = 3000;
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    println!("🚀 Rust Zero-Dependency Server running at http://localhost:{}", port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => eprintln!("Failed to establish connection: {}", e),
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    if let Ok(_) = stream.read(&mut buffer) {
        let request = String::from_utf8_lossy(&buffer[..]);
        let request_line = request.lines().next().unwrap_or("");
        
        let path = if request_line.starts_with("GET ") {
            let end_idx = request_line[4..].find(' ').unwrap_or(0);
            &request_line[4..4 + end_idx]
        } else {
            ""
        };

        let response = match path {
            "/" | "/index.html" => {
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                    INDEX_HTML.len(),
                    INDEX_HTML
                ).into_bytes()
            }
            "/style.css" => {
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/css\r\nContent-Length: {}\r\n\r\n{}",
                    STYLE_CSS.len(),
                    STYLE_CSS
                ).into_bytes()
            }
            "/main.js" => {
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/javascript\r\nContent-Length: {}\r\n\r\n{}",
                    MAIN_JS.len(),
                    MAIN_JS
                ).into_bytes()
            }
            "/pkg/tomli_wasm.js" => {
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/javascript\r\nContent-Length: {}\r\n\r\n{}",
                    WASM_JS.len(),
                    WASM_JS
                ).into_bytes()
            }
            "/pkg/tomli_wasm_bg.wasm" => {
                let header = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/wasm\r\nContent-Length: {}\r\n\r\n",
                    WASM_BG.len()
                );
                let mut resp = header.into_bytes();
                resp.extend_from_slice(WASM_BG);
                resp
            }
            _ => {
                "HTTP/1.1 404 NOT FOUND\r\n\r\n404 Not Found".to_string().into_bytes()
            }
        };

        if let Err(e) = stream.write_all(&response) {
            eprintln!("Failed to write to stream: {}", e);
        }
        let _ = stream.flush();
    }
}
