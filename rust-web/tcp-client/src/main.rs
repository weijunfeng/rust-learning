use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    stream.write_all("hello".as_bytes()).unwrap();
    let mut buf = [0; 5];
    stream.read(&mut buf).unwrap();
    println!("response server:{:?}", str::from_utf8(&buf));
}
