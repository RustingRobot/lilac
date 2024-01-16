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
    Clean,
    ///Compile all files
    Build,
    ///Run a localhost and update live
    Serve
}