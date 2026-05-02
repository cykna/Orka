use std::ffi::CString;

use color_eyre::eyre::Result;
use nix::{
    libc::SIGCHLD,
    sched::CloneFlags,
    unistd::{Pid, execve},
};

pub struct ProcessArgs {
    ///The path of the process to be runned on a different namespace
    pub name: CString,
    ///The arguments the process will receive. If this is ["hello world"], and `name` is `echo`, this is the same as `echo "hello world"`
    pub args: Vec<CString>,
    ///The environment values of this process
    pub env: Vec<CString>,
    ///The stack of the process
    pub stack: Vec<u8>,
}

pub struct Orka;

impl Orka {
    pub fn new() -> Self {
        Self
    }
    pub fn create_process(&self, mut args: ProcessArgs) -> Result<Pid> {
        let child = unsafe {
            nix::sched::clone(
                Box::new(move || {
                    let v = execve(&args.name, &args.args, &args.env);
                    //This shit shouldnt execut if execve executes properly
                    println!("{v:?} {:?}", std::io::Error::last_os_error());
                    0
                }),
                &mut args.stack,
                CloneFlags::CLONE_NEWPID,
                Some(SIGCHLD),
            )
        }?;
        Ok(child)
    }
}
