use clap::Parser;
use nix::spawn::{PosixSpawnAttr, PosixSpawnFileActions};
use std::ffi::CStr;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct OrkaCli {
    #[arg(short, long)]
    pub exec: PathBuf,
}

fn main() -> color_eyre::Result<()> {
    let cli = OrkaCli::parse();
    let pid = nix::spawn::posix_spawn(
        &cli.exec,
        &PosixSpawnFileActions::init()?,
        &PosixSpawnAttr::init()?,
        &[] as &[&CStr],
        &[] as &[&CStr],
    )?;
    println!("{:?}", pid);

    Ok(())
}
