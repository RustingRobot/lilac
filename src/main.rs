mod args;

use args::LilacArgs;
use args::CmdType::*;
use clap::Parser;

fn main() {
    let args = LilacArgs::parse();
    
    match args.cmd {
        Init =>{
            print!("YES")
        },
        Clean =>{
            print!("NO")
        },
        Build =>{
            print!("NO")
        },
        Serve =>{
            print!("NO")
        }
    }
}
