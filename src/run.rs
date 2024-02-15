use std::{
    fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, path::Path, process, sync::mpsc, time::Duration, thread
};
use toml::Table;
use notify::RecursiveMode;
use notify_debouncer_mini::new_debouncer;

pub fn run(){

    let config = match fs::read_to_string("_lilac/settings.toml"){
        Err(_) => {
            print!("Could not read from settings.toml :( Is lilac properly initiated?");
            process::exit(1);
        }
        Ok(r) => r.parse::<Table>().unwrap()
    };

    let listener = match TcpListener::bind(format!("127.0.0.1:{}",config["port"])){
        Err(_) => {
            print!("Port {} is already in use. Consider changing it in _lilac/settings.toml", config["port"]);
            process::exit(1);
        }
        Ok(r) => r
    };

    thread::spawn(|| start_watcher());

    println!("\nlocalhost server is online!");
    println!("http://127.0.0.1:{}/", config["port"]);
    println!("\nPress Ctrl + C to quit...");

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