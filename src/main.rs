use clap::Parser;
use nix::{
    libc::{SIGCHLD, execve},
    sched::CloneFlags,
    sys::wait::waitpid,
};
use std::ptr::null;

#[derive(Parser, Debug)]
struct OrkaCli {
    #[arg(short, long)]
    pub exec: String,
}
const STACK_SIZE: usize = 1024 * 1024; //1mb
fn main() -> color_eyre::Result<()> {
    let cli = OrkaCli::parse();

    let child = unsafe {
        nix::sched::clone(
            Box::new(move || {
                let ptr = cli.exec.as_bytes().as_ptr() as *const i8;
                let v = execve(ptr, null(), null());
                println!("{v} Laburai");
                0
            }),
            &mut [0; STACK_SIZE],
            CloneFlags::CLONE_NEWPID | CloneFlags::CLONE_NEWNET,
            Some(SIGCHLD),
        )?
    };

    let v = waitpid(child, None)?;
    println!("{v:?}");

    println!("{child}");

    Ok(())
}
