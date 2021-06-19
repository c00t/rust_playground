use std::{io::{Read, Write}, net::{TcpListener,TcpStream}, thread, time::Duration};
use std::fs;
use thread_pool::ThreadPool;
// fn main() {
//     let listener = TcpListener::bind("127.0.0.1:7876").unwrap();

//     for stream in listener.incoming() {
//         let stream = stream.unwrap();
        
//         println!("--Connection establish!--");
//         thread::spawn(|| {
//             handle_connection(stream); //Spawn a Thread for every request.
//         });
//     }
// }

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7876").unwrap();
    let pool = ThreadPool::new(4); // 

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        
        println!("--Connection establish!--");
        pool.execute(|| {
            handle_connection(stream); //Spawn a Thread for every request.
        });
    }

    println!("Shutting Down.");
}



fn handle_connection(mut stream:TcpStream){
    let mut buffer = [0;1024];

    stream.read(&mut buffer).unwrap();

    let get_root = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line,filename) = if buffer.starts_with(get_root) {
        ("HTTP/1.1 200 OK","hello.html")
    }else if buffer.starts_with(sleep){
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 404 NOT FOUND","404.html")
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