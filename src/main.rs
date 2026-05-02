mod orka;
use std::ffi::CString;

use clap::Parser;
use nix::sys::wait::waitpid;

use crate::orka::{Orka, ProcessArgs};
#[derive(Parser, Debug)]
struct OrkaCli {
    #[arg(short, long)]
    pub exec: String,
    #[arg(long, allow_hyphen_values = true)]
    pub arguments: Vec<String>,
}

impl OrkaCli {
    pub fn name(&self) -> CString {
        CString::new(&*self.exec).unwrap()
    }
    pub fn arguments(&self) -> Vec<CString> {
        let mut out = vec![self.name()];
        out.append(
            &mut self
                .arguments
                .iter()
                .map(|v| CString::new(&**v).unwrap())
                .collect(),
        );
        out
    }
}

const STACK_SIZE: usize = 1024 * 1024; //1mb

fn main() -> color_eyre::Result<()> {
    let cli = OrkaCli::parse();
    let orka = Orka::new();

    let process = orka.create_process(ProcessArgs {
        name: cli.name(),
        args: cli.arguments(),
        env: Vec::new(),
        stack: vec![0; STACK_SIZE],
    })?;

    let v = waitpid(process, None)?;
    println!("{v:?}");

    Ok(())
}
