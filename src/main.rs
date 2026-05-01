use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct OrkaCli {
    #[arg(short, long)]
    pub exec: PathBuf,
}

fn main() -> std::io::Result<()> {
    let cli = OrkaCli::parse();
    let child_result = std::process::Command::new(cli.exec).output()?;

    println!("{:?}", child_result.status);

    Ok(())
}
