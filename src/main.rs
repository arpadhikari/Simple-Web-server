use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use web_server::ThreadPool;

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let req_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, file) = if req_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(file).unwrap();
    let length = contents.len();

    let res = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(res.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        // thread::spawn(|| handle_connection(stream));

        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Shutting down!");
}
