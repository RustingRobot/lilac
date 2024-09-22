use std::{
    fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, path::Path, sync::{mpsc, Arc, Mutex}, thread, time::Duration
};

use notify::RecursiveMode;
use notify_debouncer_mini::{new_debouncer, DebouncedEventKind};

use tungstenite::accept;

use crate::{broadcaster::UnboundedBroadcast, exit::err_exit, settings::{self, Settings}};

const INJECT: &str = r#"
<script>
    const socket = new WebSocket("ws://localhost:8081");
    socket.addEventListener("message", (event) => {
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

    //let (tx, rx) = unbounded();
    let write = Arc::new(Mutex::new(UnboundedBroadcast::new()));
    let read = write.clone();
    thread::spawn(|| start_watcher(write));
    thread::spawn(|| start_ws_server(read));

    println!("\nlocalhost server is online!");
    println!("\u{1b}[34;1mhttp://localhost:{}/\u{1b}[0m", settings.webserver_port);
    println!("\nPress Ctrl + C to quit...\n");

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
        Ok(mut r) => {status_line = "HTTP/1.1 200 OK"; r.append(&mut INJECT.as_bytes().to_owned()); r}
    };

    let length = contents.len();
    
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n");
    stream.write_all(&[response.as_bytes(), &contents].concat()).unwrap();
}

fn start_watcher(broadcaster: Arc<Mutex<UnboundedBroadcast<bool>>>){
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
                        let shared = broadcaster.lock().unwrap();
                        shared.send(true).unwrap();
                    }
                }),
            Err(error) => println!("Error {error:?}"),
        }
    }
}

fn start_ws_server(broadcaster: Arc<Mutex<UnboundedBroadcast<bool>>>){
    let listener = TcpListener::bind("127.0.0.1:8081").unwrap();
    for stream in listener.incoming() {
        let mut shared = broadcaster.lock().unwrap();
        let new_receiver = shared.subscribe();
        thread::spawn(move || {
        let mut websocket = accept(stream.unwrap()).unwrap();
        for _ in new_receiver{
            println!("\u{1b}[34;1m----- building website -----\u{1b}[0m");
            let build = thread::spawn(|| super::build::build());
            if let Ok(_) = build.join(){
                if let Err(_) = websocket.send("test".into()){
                    return;
                }
            }else{
                println!("\u{1b}[31;1m\noops, something went wrong :(\n\u{1b}[0m")
            }
        }
    });
    }
}