use std::{fs, path::Path, process};
use toml::Table;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub directory_index: String,
    pub webserver_port: u16,
    pub start_delimiter: String,
    pub end_delimiter: String
}

pub fn request_settings() -> Settings{
    if !Path::new("_lilac/settings.toml").exists(){
        print!("Could not find a settings.toml file :( Is this directory using lilac?\n\
                use lilac init to create a lilac directory or restore the default settings.toml");
        process::exit(1);
    }

    let config_string = fs::read_to_string("_lilac/settings.toml").unwrap();

    match toml::from_str(&config_string) {
        Err(e) => {
            print!("lilac settings.toml could not be parsed correctly. Perhaps a field is missing or an invalid type was entered?\n\
                    use lilac init to restore the default settings.toml.\n\n{:?}", e);
            process::exit(1);
        },
        Ok(r) => r
    }
}