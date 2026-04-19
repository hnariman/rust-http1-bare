pub mod error;
mod types;

use std::arch::asm;

use types::{Inet, LinuxSyscalls, Result, SockAddrIn, SockStream};

pub use crate::error::{AsmResult, HttpError};

#[inline]
pub fn create_socket() -> Result<i32> {
    let result: i32;
    unsafe {
        asm!(
            "syscall",
            in("rax") LinuxSyscalls::SYS_SOCKET as usize,
            in("rdi") Inet::AF_INET as usize,
            in("rsi") SockStream::TCP as usize,
            in("rdx") 0,  // Protocol - use the default value for TCP + IPV4 etc
            out("rcx") _,
            out("r11") _,
            lateout("rax") result,
            options(nostack)
        );
    }
    result.or_err(HttpError::CreateSocketError)
}

#[inline]
pub fn bind_socket(port: u16, fd: i32) -> Result<i32> {
    let addr = SockAddrIn {
        sin_family: Inet::AF_INET as u16,
        sin_port: port.to_be(),
        sin_addr: 0,      // any 0.0.0.0
        sin_zero: [0; 8], // Padding
    };
    let result: i32;

    unsafe {
        asm!(
            "syscall",
            in("rax") LinuxSyscalls::SYS_BIND as usize,
            in("rdi") fd,
            in("rsi") &addr as *const SockAddrIn as usize,
            in ("rdx") size_of::<SockAddrIn>(),
            out("rcx") _,
            out("r11") _,
            lateout("rax") result,
            options(nostack)
        )
    }
    result.or_err(HttpError::BindSocketError)
}

#[inline]
pub fn set_socket_to_listen(fd: i32) -> Result<i32> {
    let result: i32;
    unsafe {
        asm!(
            "syscall",
            in("rax") LinuxSyscalls::SYS_LISTEN as usize,
            in("rdi") fd,
            in("rsi") 4096, // backlog, usually limits used 128, or 4096 etc
            out("rcx") _,
            out("r11") _,
            lateout("rax") result,
            options(nostack)
        )
    }
    result.or_err(HttpError::ListenError)
}

/// this is blocking call, will listen until any request to port received
#[inline]
pub fn accept_incoming(fd: i32) -> Result<i32> {
    let result: i32;
    unsafe {
        asm!(
            "syscall",
            in("rax") LinuxSyscalls::SYS_ACCEPT as usize,
            in("rdi") fd,
            in("rsi") 0, // Incoming client's IP
            in("rdx") 0, // buffer size
            out("rcx") _,
            out("r11") _,
            lateout("rax") result,
            options(nostack)
        )
    }
    result.or_err(HttpError::AcceptError)
}

#[inline]
pub fn read_socket(listener: i32, buffer: &mut [u8; 1024]) -> Result<i32> {
    let count: isize;
    unsafe {
        asm!(
            "syscall",
            in("rax") LinuxSyscalls::SYS_READ as usize,
            in("rdi") listener,
            in("rsi") buffer.as_mut_ptr(),
            in("rdx") buffer.len(),
            out("rcx") _,
            out("r11") _,
            lateout("rax") count,
            options(nostack)
        )
    }

    (count as i32).or_err(HttpError::ReadSocketError)
}

#[inline]
pub fn close_socket(fd: i32) -> Result<i32> {
    let result: i32;
    unsafe {
        asm!(
            "syscall",
            in("rax") LinuxSyscalls::SYS_CLOSE as usize,
            in("rdi") fd,
            out("rcx") _,
            out("r11") _,
            lateout("rax") result,
            options(nostack)
        )
    }
    result.or_err(HttpError::CloseSocketError)
}

/// Kernel will have socket timeout on restart of the server app,
/// so we need to reuse the socket instead of waiting for assigning
/// (anyways assign was failing during TIMOUT)
#[inline]
pub fn set_reuse_address(fd: i32) -> Result<i32> {
    let sock: i32 = 1;
    let result: i32;
    unsafe {
        asm!(
            "syscall",
            in("rax") LinuxSyscalls::SYS_SETSOCKOPT as usize,
            in("rdi") fd,
            in("rsi") 1, // SOL_SOCKET
            in("rdx") 2, // SOL_REUSEADDR
            in("r10") &sock as *const i32 as usize,
            in("r8") 4,
            out("rcx") _,
            out("r11") _,
            lateout("rax") result,
            options(nostack)
        )
    }
    result.or_err(HttpError::ReuseSocketError)
}

#[inline]
pub fn write_socket(fd: i32, text: &str) -> Result<i32> {
    let result: i32;
    unsafe {
        asm!(
            "syscall",
            in("rax") LinuxSyscalls::SYS_WRITE as usize,
            in("rdi") fd,
            in("rsi") text.as_ptr(),
            in("rdx") text.len(),
            // clobbers for clean-up
            out("rcx") _,
            out("r11") _,
            lateout("rax") result,
            options(nostack)
        )
    }
    result.or_err(HttpError::WriteSocketError)
}

#[inline]
pub fn create_response(text: &str) -> String {
    format!(
        "HTTP/1.1 200 OK\r\n\
        Content-Type: text/plain\r\n\
        Content-Length: {}\r\n\
        Connection: close\r\n\
        \r\n\
        {}
        ",
        text.len(),
        text
    )
}
