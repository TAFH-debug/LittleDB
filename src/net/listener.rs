use std::io::{Read, stdout, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

pub fn listen(address: String) {
    println!("Listening at {}", address);
    stdout().flush();
    let stream = TcpListener::bind(address.clone()).unwrap();
    for i in stream.incoming() {
        let mut i = i.unwrap();
        handle_con(&mut i);
    }
}

pub fn handle_con(stream: &mut TcpStream) {
    let mut received = Vec::new();
    let mut request_type = [0; 1];
    stream.read(&mut request_type);
    match request_type.first().unwrap() {
        0 => {
            let mut buf = [0; 1];
            stream.read(&mut buf);
            let len = buf.first().unwrap();
            let mut vec_str = Vec::new();
            for _ in 0..*len {
                let mut buf2 = [0; 1];
                stream.read(&mut buf2);
                vec_str.push(*buf2.first().unwrap());
            }
            let table_name = String::from_utf8(vec_str).unwrap();
            let values = crate::core::get_values(table_name).unwrap();
            //TODO sending values
        },
        1 => {},
        2 => {},
        _ => {
            stream.write("Invalid request!".as_bytes());
            return;
        }
    }
    println!("{}", String::from_utf8(received).unwrap());
}