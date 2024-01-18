use std::fs;
use std::io::{prelude::*, ErrorKind};
use std::process;

pub fn init(){
    let dir_result = fs::create_dir(".lilac");
    match dir_result{
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