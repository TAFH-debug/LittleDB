use crate::read_string;
use std::io::{stdout, Read, Write};
use std::net::{TcpListener, TcpStream};


pub fn listen(address: String) {
    println!("Listening at {}", address);
    stdout().flush().expect("Stout error");
    let stream = TcpListener::bind(address.clone()).unwrap();
    for i in stream.incoming() {
        let mut i = i.unwrap();
        handle_con(&mut i);
    }
}

pub fn handle_con(stream: &mut TcpStream) {
    let received = Vec::new();
    let mut request_type = [0; 1];
    stream.read(&mut request_type);
    match request_type.first().unwrap() {
        0 => {
            //SELECT * FROM
            let table_name = read_string!(stream);
            let values = crate::core::data_read::get_values_binary(table_name).unwrap();
            let header = values.0;
            let mut data = values.1;
            stream.write(&header.len().to_be_bytes());
            stream.write(header.as_bytes());
            stream.write(&*data);
        }
        1 => {
            //CREATE TABLE
            let tbl_name = read_string!(stream);
            let types = read_string!(stream);
            crate::core::data_wrt::_create_table(tbl_name, types);
        },
        2 => {
            todo!()
        }
        _ => {
            stream.write("Invalid request!".as_bytes());
            return;
        }
    }
    println!("{}", String::from_utf8(received).unwrap());
}
