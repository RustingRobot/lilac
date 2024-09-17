use core::time;
use std::{
    cmp::Reverse, fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, path::Path, sync::mpsc, thread, time::Duration
};
use notify::RecursiveMode;
use notify_debouncer_mini::{new_debouncer, DebouncedEventKind};

use tungstenite::{accept, handshake::server::{Request, Response}};

use crossbeam_channel::{unbounded, Receiver, Sender};

use crate::{exit::err_exit, settings::{self, Settings}};

const INJECT: &str = r#"
<script>
    const socket = new WebSocket("ws://localhost:8081");
    socket.addEventListener("message", (event) => {
        alert("test");
        location.reload();
    });      
</script>
"#;

pub fn run(){
    let settings = settings::request_settings();

    let listener = match TcpListener::bind(format!("127.0.0.1:{}",settings.webserver_port)){
        Err(_) => {
            err_exit(&format!("Port {} is already in use. Consider changing it in _lilac/settings.toml", settings.webserver_port));
        }
        Ok(r) => r
    };

    let (tx, rx) = unbounded();
    thread::spawn(move || start_watcher(tx));
    thread::spawn(move || start_ws_server(rx));

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
    let mut contents = match fs::read(format!("_lilac/build{path}")){
        Err(_) => "[404] file does not exist :(".as_bytes().to_vec(),
        Ok(r) => {status_line = "HTTP/1.1 200 OK"; r}
    };

    contents.append(&mut INJECT.as_bytes().to_owned());

    let length = contents.len();
    
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n");
    stream.write_all(&[response.as_bytes(), &contents].concat()).unwrap();
}

fn start_watcher(sender: Sender<bool>){
    let (tx, rx) = mpsc::channel();
    let mut debouncer = new_debouncer(Duration::from_secs(1), tx).unwrap();

    debouncer
        .watcher()
        .watch(Path::new("."), RecursiveMode::Recursive)
        .unwrap();

    debouncer.watcher().unwatch(Path::new("./_lilac")).unwrap();

    for result in rx {
        match result {
            Ok(events) => events
                .iter()
                .for_each(|event| {
                    if event.kind == DebouncedEventKind::Any {
                        println!("sending update");
                        sender.send(true).unwrap();
                    }
                }),
            Err(error) => println!("Error {error:?}"),
        }
    }
}

fn start_ws_server(receiver: Receiver<bool>){
    let listener = TcpListener::bind("127.0.0.1:8081").unwrap();
    for stream in listener.incoming() {
        let new_receiver = receiver.clone();
        thread::spawn(move || {
        let mut websocket = accept(stream.unwrap()).unwrap();
        for _ in new_receiver{
            super::build::build();
            println!("sending websoket msg");
            if let Err(_) = websocket.send("test".into()){
                println!("return");
                break;
            }
        }
    });
    }
}