use std::{fs, thread};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

use simple_web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let thread_pool = ThreadPool::new(4);
    for stream in listener.incoming().take(2) {
        match stream {
            Ok(stream) => {
                thread_pool.execute(|| {
                    handle_connect(stream);
                });
            }
            Err(e) => {
                println!("stream error {e}")
            }
        }
    }
    println!("shutting down");
}

/// 处理一个链接
fn handle_connect(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();
    // println!("request:{http_request:#?}");
    // 获取请求的第一个行，不处理全部请求内容
    let first_line = buf_reader.lines().next();
    match first_line {
        None => {
            println!("no request")
        }
        Some(lines) => {
            let (status_line, path) = match &lines.unwrap()[..] {
                "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
                "GET /sleep HTTP/1.1" => {
                    thread::sleep(Duration::from_secs(5));
                    ("HTTP/1.1 200 OK", "hello_sleep.html")
                }
                _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
            };
            let response = build_response(status_line, path);
            stream.write_all(response.as_bytes()).unwrap();
        }
    }
}

/// 创建一个响应内容
fn build_response(status_line: &str, path: &str) -> String {
    let contents = fs::read_to_string(path).unwrap();
    let length = contents.len();
    return format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
}
