use std::mem;
use std::os::unix::io::RawFd;
use std::ptr::null_mut;
use libc::{self, c_int};
use {Errno, Result};
use sys::time::TimeVal;

pub use libc::FD_SETSIZE;

// FIXME: Change to repr(transparent) once it's stable
#[repr(C)]
pub struct FdSet(libc::fd_set);

impl FdSet {
    pub fn new() -> FdSet {
        let mut fdset = unsafe { mem::uninitialized() };
        unsafe { libc::FD_ZERO(&mut fdset) };
        FdSet(fdset)
    }

    pub fn insert(&mut self, fd: RawFd) {
        unsafe { libc::FD_SET(fd, &mut self.0) };
    }

    pub fn remove(&mut self, fd: RawFd) {
        unsafe { libc::FD_CLR(fd, &mut self.0) };
    }

    // FIXME: Change to take `&self` once https://github.com/rust-lang/libc/pull/718 lands
    pub fn contains(&mut self, fd: RawFd) -> bool {
        unsafe { libc::FD_ISSET(fd, &mut self.0) }
    }

    pub fn clear(&mut self) {
        unsafe { libc::FD_ZERO(&mut self.0) };
    }
}

pub fn select(nfds: c_int,
              readfds: Option<&mut FdSet>,
              writefds: Option<&mut FdSet>,
              errorfds: Option<&mut FdSet>,
              timeout: Option<&mut TimeVal>) -> Result<c_int> {
    let readfds = readfds.map(|set| set as *mut _ as *mut libc::fd_set).unwrap_or(null_mut());
    let writefds = writefds.map(|set| set as *mut _ as *mut libc::fd_set).unwrap_or(null_mut());
    let errorfds = errorfds.map(|set| set as *mut _ as *mut libc::fd_set).unwrap_or(null_mut());
    let timeout = timeout.map(|tv| tv as *mut _ as *mut libc::timeval)
                         .unwrap_or(null_mut());

    let res = unsafe {
        libc::select(nfds, readfds, writefds, errorfds, timeout)
    };

    Errno::result(res)
}
