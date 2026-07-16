use std::{io::{Read, Write}, net::TcpStream};

pub fn request(url: &str) -> () {

    let url = url.trim().split("://").last().expect("failed to get url");

    let host = url.splitn(2, "/").nth(0).expect("failed to get host");
    let path = url.splitn(2, "/").nth(1);
    
    let mut stream = TcpStream::connect((host, 80)).expect("failed to open connection");

    let req = format!(
        "GET {} HTTP/1.1\r\n
        HOST: {host}\r\n
        Connection: close\r\n
        User-Agent: http/0.1\r\n",
        path.unwrap_or("/"),
    );

    stream.write_all(req.as_bytes()).expect("faile to send");

    let mut res = String::new();

    stream.read_to_string(&mut res).expect("failed to read");

    println!("{}", res);
}