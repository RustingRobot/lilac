use std::{fs, path::Path};
use serde::{Deserialize, Serialize};

use crate::exit::err_exit;

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub directory_index: String,
    pub webserver_port: u16,
    pub start_delimiter: String,
    pub end_delimiter: String,
    pub subsection_marker: char
}

pub fn request_settings() -> Settings{
    if !Path::new("_lilac/settings.toml").exists(){
        err_exit("Could not find a settings.toml file :( Is this directory using lilac?\n\
                use \u{1b}[34;1mlilac init\u{1b}[0m to create a lilac directory or restore the default settings.toml");
    }

    let config_string = fs::read_to_string("_lilac/settings.toml").unwrap();

    match toml::from_str(&config_string) {
        Err(e) => {
            err_exit(&format!("lilac settings.toml could not be parsed correctly. Perhaps a field is missing or an invalid type was entered?\n\
                    use lilac init to restore the default settings.toml.\n\n{:?}", e))
        },
        Ok(r) => r
    }
}