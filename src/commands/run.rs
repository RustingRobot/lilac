use std::{
    fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, path::Path, sync::mpsc, time::Duration, thread
};
use notify::RecursiveMode;
use notify_debouncer_mini::new_debouncer;

use crate::{exit::err_exit, settings::{self, Settings}};

pub fn run(){
    let settings = settings::request_settings();

    let listener = match TcpListener::bind(format!("127.0.0.1:{}",settings.webserver_port)){
        Err(_) => {
            err_exit(&format!("Port {} is already in use. Consider changing it in _lilac/settings.toml", settings.webserver_port));
        }
        Ok(r) => r
    };

    thread::spawn(|| start_watcher());

    println!("\nlocalhost server is online!");
    println!("http://localhost:{}/", settings.webserver_port);
    println!("\nPress Ctrl + C to quit...");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, &settings);
    }
}

fn handle_connection(mut stream: TcpStream, settings: &Settings) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let mut status_line = "HTTP/1.1 404 NOT FOUND";

    //determine, what file the requester actually wants
    let path = {
    if request_line == "GET / HTTP/1.1" { //root file
        &settings.directory_index
    }else{
        let options = request_line.split(" ").collect::<Vec<_>>();
        options[1] //might break on invalid HTTP requests :v
    }};

    //serve file if it exists. If not send 404
    let contents = match fs::read(format!("_lilac/build{path}")){
        Err(_) => "[404] file does not exist :(".as_bytes().to_vec(),
        Ok(r) => {status_line = "HTTP/1.1 200 OK"; r}
    };

    let length = contents.len();
    
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n");
    stream.write_all(&[response.as_bytes(), &contents].concat()).unwrap();
}

fn start_watcher(){
    let (tx, rx) = mpsc::channel();
    let mut debouncer = new_debouncer(Duration::from_secs(1), tx).unwrap();

    debouncer
        .watcher()
        .watch(Path::new("."), RecursiveMode::Recursive)
        .unwrap();

    // print all events, non returning
    for result in rx {
        match result {
            Ok(events) => events
                .iter()
                .for_each(|event| println!("Event {event:?}")),
            Err(error) => println!("Error {error:?}"),
        }
    }
}