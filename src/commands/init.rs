use std::fs::{self, File};
use std::io::{prelude::*, ErrorKind};
use toml::to_string;
use crate::exit::err_exit;
use crate::settings::Settings;

pub fn init(){
    match try_create_files(){
        Err(e) => {
            match e.kind(){
                ErrorKind::AlreadyExists => err_exit("This directory is already using lilac"),
                ErrorKind::PermissionDenied => err_exit("Cannot create files here: permission denied"),
                _ => err_exit(&format!("Something unexpected has happened! {:?}", e))
            };
        }
        _ => {}
    }
}

fn try_create_files() -> std::io::Result<()>{
    let default_config = Settings {
        directory_index: "/index.html".to_owned(),
        webserver_port: 8080,
        start_delimiter: "[[".to_owned(),
        end_delimiter: "]]".to_owned(),
        subsection_marker: '='
    };

    fs::create_dir_all("_lilac/build")?;
    let toml_string = to_string(&default_config).unwrap();
    let mut file = File::create("_lilac/settings.toml")?;
    file.write_all(toml_string.as_bytes())?;
    Ok(())
}