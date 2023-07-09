use std::fs;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use rust_webserver::ThreadPool;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming().take(2){ //we only process the first two request and then shut down
        let stream = stream.unwrap();
        pool.execute(|| {
            handel_connection(stream);
        });
        

    }
}

fn handel_connection(mut stream: TcpStream){
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n"; //here b means byte array of the string
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get){
        ("HTTP/1.1 200 OK","index.html")
    }
    else if buffer.starts_with(sleep){
        thread::sleep(Duration::from_secs(5)); //this will make hte server sleep for 5 seconds and next request to the serever will only be handles after 5 secs
        ("HTTP/1.1 200 OK","index.html")
    }
    else{
        ("HTTP/1.1 400 NOT FOUND","404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
    contents.len(),
contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    
}
