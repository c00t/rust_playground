use std::{io::{Read, Write}, net::{TcpListener,TcpStream}};
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7876").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        println!("--Connection establish!--");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream:TcpStream){
    let mut buffer = [0;1024];

    stream.read(&mut buffer).unwrap();

    let get_root = b"GET / HTTP/1.1\r\n";

    let (status_line,filename) = if buffer.starts_with(get_root) {
        ("HTTP/1.1 200 OK","hello.html")
    }else{
        ("HTTP/1.1 404 NOT FOUND","404.html")
    };

    // if buffer.starts_with(get_root) {
    //     let contents = fs::read_to_string("hello.html").unwrap();

    //     let response = format!(
    //         "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
    //         contents.len(),
    //         contents,
    //     );
    
    //     stream.write(response.as_bytes()).unwrap();
    //     stream.flush().unwrap();
    // }else{
    //     // may be return a 404 response
    //     // Some test
    //     let not_found_response = "HTTP/1.1 404 NOT FOUND";

    //     let contents = fs::read_to_string("404.html").unwrap();

    //     let response = format!(
    //         "{}\r\nContent-Length: {}\r\n\r\n{}",
    //         not_found_response,
    //         contents.len(),
    //         contents,
    //     );
    
    //     stream.write(response.as_bytes()).unwrap();
    //     stream.flush().unwrap();
    // }

    //println!("Request: {}",String::from_utf8_lossy(&buffer));

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents,
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}