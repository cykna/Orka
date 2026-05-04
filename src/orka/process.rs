use std::{ffi::CString, marker::PhantomData, ptr::NonNull};

use nix::{
    sys::wait::{WaitStatus, waitpid},
    unistd::Pid,
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

pub struct Process<'a> {
    pid: Pid,
    stack_bottom: *mut u8,
    stack_top: *mut u8,
    phantom: PhantomData<&'a ()>,
}

impl<'a> Process<'a> {
    pub fn new(pid: Pid, (top, bottom): (*mut u8, *mut u8)) -> Self {
        Self {
            pid,
            stack_bottom: bottom,
            stack_top: top,
            phantom: PhantomData,
        }
    }

    ///Waits for this process to change its status
    pub fn wait(&self) -> color_eyre::eyre::Result<WaitStatus> {
        waitpid(self.pid, None).map_err(|e| e.into())
    }
}

impl<'a> Drop for Process<'a> {
    fn drop(&mut self) {
        unsafe {
            nix::sys::mman::munmap(
                NonNull::new(self.stack_top as *mut _).unwrap(), //stack top since it grows downwards.
                self.stack_bottom.addr() - self.stack_top.addr(), //top <= bottom, then bottom - top >= 0
            )
            .unwrap()
        }
    }
}
