use std::fs;
use std::io::Read;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;
use rust_webserver::ThreadPool;

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7800").unwrap();

    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        pool.execute( || {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer =[0; 1024];
    stream.read(&mut buffer).unwrap();
    
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status, filename) = 
        if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK", "index.html")
        } else if buffer.starts_with(sleep){
            thread::sleep(Duration::from_secs(6));
            ("HTTP/1,1 200 OK", "index.html")
        }
        else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };
      
    let contents = fs::read_to_string(filename).unwrap();

        let response  = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status,
            contents.len(),
            contents,
           
        );
    
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

}