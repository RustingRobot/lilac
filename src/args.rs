use clap:: {
    Args,
    Parser,
    Subcommand
};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct LilacArgs{
    #[clap(subcommand)]
    pub cmd: CmdType,
}

#[derive(Debug, Subcommand)]
pub enum CmdType{
    ///Create a new Lilac project
    Init,
    ///Remove Lilac from this directory
    Remove,
    ///Remove all compiled files
    Clean,
    ///Compile all changed files
    Build,
    ///Run a localhost with live-update
    Run
}