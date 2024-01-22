use std::fs::{self, File};
use std::io::{prelude::*, ErrorKind};
use std::process;
use toml::to_string;
use serde::Serialize;

#[derive(Serialize)]
struct Config {
    resources_path: String,
    directory_index: String
}

pub fn init(){
    match try_create_files(){
        Err(e) => {
            match e.kind(){
                ErrorKind::AlreadyExists => print!("This directory is already using lilac"),
                ErrorKind::PermissionDenied => print!("Cannot create files here: permission denied"),
                _ => print!("Something unexpected has happened! {:?}", e)
            };
            process::exit(1);
        }
        _ => {}
    }
}

fn try_create_files() -> std::io::Result<()>{
    let default_config = Config {
        resources_path: "/res".to_owned(),
        directory_index: "/index.html".to_owned()
    };

    fs::create_dir_all("_lilac/build")?;
    let toml_string = to_string(&default_config).unwrap();
    let mut file = File::create("_lilac/settings.toml")?;
    file.write_all(toml_string.as_bytes())?;
    Ok(())
}