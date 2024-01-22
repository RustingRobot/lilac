use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}, fs, process,
};
use toml::Table;

pub fn run(){
    print!("localhost server is online! Press Ctrl + C to quit...\n");
    let _ = open::that("http://127.0.0.1:8001/");
    let listener = TcpListener::bind("127.0.0.1:8001").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let mut status_line = "HTTP/1.1 404 NOT FOUND";
    let value;

    //determine, what file the requester actually wants
    let path = {
    if request_line == "GET / HTTP/1.1" { //root file
        value = match fs::read_to_string("_lilac/settings.toml"){
            Err(_) => {
                print!("Could not read from settings.toml :( Is lilac properly initiated?");
                process::exit(1);
            }
            Ok(r) => r.parse::<Table>().unwrap()
        };
        value["directory_index"].as_str().unwrap()
    }else{
        let options = request_line.split(" ").collect::<Vec<_>>();
        options[1] //might break on invalid HTTP requests :v
    }};

    //serve file if it exists. If not send 404
    let contents = match fs::read(format!("_lilac/build{path}")){
        Err(_) => "<!DOCTYPE html><html><body><h1>File Not Found</h1></body></html>".as_bytes().to_vec(),
        Ok(r) => {status_line = "HTTP/1.1 200 OK"; r}
    };

    let length = contents.len();
    
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n");
    stream.write_all(&[response.as_bytes(), &contents].concat()).unwrap();
}