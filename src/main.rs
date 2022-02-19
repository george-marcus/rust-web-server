use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // listening to connection attemps which
    // might not be successful for a number of reasons, many of them are operating system specific
    for stream in listener.incoming() {
        // to panic immediately in case of errors
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

/*
    The TcpStream instance keeps track of what data it returns to us internally.
    It might read more data than we asked for and save that data for the next time we ask for data.
    It therefore needs to be mut
*/
fn handle_connection(mut stream: TcpStream) {
    //Buffer is big enough to hold the data of a basic request
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    //Convert the bytes in the buffer to a string and print that string
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    /*
     Because we’re reading raw bytes into the buffer, we transform get into a byte string
     by adding the b"" byte string syntax at the start of the content data.
    */
    
    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        
        ("HTTP/1.1 200 OK", "views/hello.html")

    } else {

        ("HTTP/1.1 404 NOT FOUND", "views/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

     /*
         To ensure a valid HTTP response,
         we add the Content-Length header which is set to the size of our response body
     */
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    //Because the write operation could fail, we use unwrap on any error result as before.
    stream.write(response.as_bytes()).unwrap();

    //Flush will wait and prevent the program from continuing until all the bytes are written to the connection
    stream.flush().unwrap();
}
