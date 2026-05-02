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
    pub arg: Vec<String>,
    #[arg(long, allow_hyphen_values = true)]
    pub env: Vec<String>,
}

impl OrkaCli {
    pub fn name(&self) -> CString {
        CString::new(&*self.exec).unwrap()
    }
    pub fn arguments(&self) -> Vec<CString> {
        let mut out = vec![self.name()];
        out.append(
            &mut self
                .arg
                .iter()
                .map(|v| CString::new(&**v).unwrap())
                .collect(),
        );
        out
    }
    pub fn env_vars(&self) -> Vec<CString> {
        self.env
            .iter()
            .map(|v| CString::new(&**v).unwrap())
            .collect()
    }
}

const STACK_SIZE: usize = 1024 * 1024; //1mb

fn main() -> color_eyre::Result<()> {
    let cli = OrkaCli::parse();
    let orka = Orka::<4096>::new();

    let process = orka.create_process(ProcessArgs {
        name: cli.name(),
        args: cli.arguments(),
        env: cli.env_vars(),
        stack: vec![0; STACK_SIZE],
    })?;

    let v = waitpid(process, None)?;
    println!("{v:?}");

    Ok(())
}
