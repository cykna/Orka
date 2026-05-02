use std::{ffi::CString, num::NonZero};

use color_eyre::eyre::Result;
use nix::{
    libc::SIGCHLD,
    sched::CloneFlags,
    sys::mman::{MapFlags, ProtFlags, mprotect},
    unistd::{Pid, execve},
};

pub struct ProcessArgs {
    ///The path of the process to be runned on a different namespace
    pub name: CString,
    ///The arguments the process will receive. If this is ["hello world"], and `name` is `echo`, this is the same as `echo "hello world"`
    pub args: Vec<CString>,
    ///The environment values of this process
    pub env: Vec<CString>,
    ///The number of pages the stack will have.
    pub stack_size: usize,
}

pub struct Orka<const PAGE_SIZE: usize = 4096>;

impl<const PAGE_SIZE: usize> Orka<PAGE_SIZE> {
    pub fn new() -> Self {
        Self
    }

    ///Allocates a new stack with the given amount of `page_amount`. Returns the base and the top addresses of it.
    pub fn allocate_stack(page_amount: usize) -> (*mut u8, *mut u8) {
        let total = PAGE_SIZE * (page_amount + 1);
        let ptr = unsafe {
            nix::sys::mman::mmap_anonymous(
                None,
                NonZero::new(total).unwrap(),
                ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
                MapFlags::MAP_PRIVATE | MapFlags::MAP_STACK | MapFlags::MAP_GROWSDOWN,
            )
        }
        .unwrap();
        unsafe { mprotect(ptr, PAGE_SIZE, ProtFlags::PROT_NONE).unwrap() };

        // retorna o topo (end of buffer)
        let ptr = ptr.as_ptr() as *mut _;
        (ptr, unsafe { ptr.add(total) })
    }

    pub fn create_process(&self, args: ProcessArgs) -> Result<Pid> {
        let (base, top) = Self::allocate_stack(args.stack_size);
        let child = unsafe {
            nix::sched::clone(
                Box::new(move || {
                    let v = execve(&args.name, &args.args, &args.env);
                    //This shit shouldnt execut if execve executes properly
                    println!("{v:?} {:?}", std::io::Error::last_os_error());
                    0
                }),
                std::slice::from_raw_parts_mut(top, 0),
                CloneFlags::CLONE_NEWPID,
                Some(SIGCHLD),
            )
        }?;
        Ok(child)
    }
}
