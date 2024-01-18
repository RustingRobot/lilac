mod args;
mod init;

use args::LilacArgs;
use args::CmdType::*;
use clap::Parser;
use std::fs;
use std::io::ErrorKind;
use std::process;

fn main() {
    let args = LilacArgs::parse();
    
    match args.cmd {
        Init =>{
            init::init()
        },
        Remove =>{
            let result = fs::remove_dir_all(".lilac");
            match result{
                Err(e) => {
                    match e.kind(){
                        ErrorKind::NotFound => print!("This directory does not use lilac"),
                        ErrorKind::PermissionDenied => print!("Cannot delete files here: permission denied"),
                        _ => print!("Something unexpected has happened! {:?}", e)
                    };
                    process::exit(1);
                }
                _ => {}
            }
        },
        Clean =>{
            print!("NO")
        },
        Build =>{
            print!("NO")
        },
        Run =>{
            print!("NO")
        }
    }
}
