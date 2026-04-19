use crate::HttpServer;
use crate::http1::error::Http1Error;
use crate::http1::types::*;
use std::arch::asm;

impl HttpServer {
    pub(super) fn create_socket() -> Result<i32, Http1Error> {
        let result: i32;
        unsafe {
            asm!(
                "syscall",
                in("rax") LinuxSyscalls::SYS_SOCKET as usize,
                in("rdi") Inet::AF_INET as usize,
                in("rsi") SockStream::TCP as usize,
                in("rdx") 0,  // Protocol - use the default value for TCP + IPV4 etc
                lateout("rax") result, // out but late, wait for kernel response
                options(nostack) // do not modify stack frame, faster and safer
            );
        }
        if (result < 0) {
            return Err(Http1Error::CreateSocketError(result));
        }
        Ok(result)
    }

    pub(super) fn bind_socket(&self) -> Result<i32, Http1Error> {
        let addr = SockAddrIn {
            sin_family: Inet::AF_INET as u16,
            sin_port: self.port.to_be(),
            sin_addr: 0,      // any 0.0.0.0
            sin_zero: [0; 8], // Padding
        };
        let result: i32;

        unsafe {
            asm!(
                "syscall",
                in("rax") LinuxSyscalls::SYS_BIND as usize,
                in("rdi") self.socket_fd,
                in("rsi") &addr as *const SockAddrIn as usize,
                in ("rdx") size_of::<SockAddrIn>(),
                lateout("rax") result,
                options(nostack)
            )
        }
        if result < 0 {
            return Err(Http1Error::BindSocketError(result));
        }
        Ok(result)
    }

    pub(super) fn set_socket_to_listen(&self) -> Result<i32, Http1Error> {
        let result: i32;
        unsafe {
            asm!(
                "syscall",
                in("rax") LinuxSyscalls::SYS_LISTEN as usize,
                in("rdi") self.socket_fd,
                in("rsi") 4096, // backlog, usually limits used 128, or 4096 etc
                lateout("rax") result,
                options(nostack)
            )
        }
        if result < 0 {
            return Err(Http1Error::ListenError(result));
        }
        Ok(result)
    }

    /// this is blocking call, will listen until any request to port received
    pub(super) fn accept_incoming(&mut self) -> Result<(), Http1Error> {
        let result: i32;
        unsafe {
            asm!(
                "syscall",
                in("rax") LinuxSyscalls::SYS_ACCEPT as usize,
                in("rdi") self.socket_fd,
                in("rsi") 0, // Incoming client's IP
                in("rdx") 0, // buffer size
                lateout("rax") result,
                options(nostack)
            )
        }
        if result < 0 {
            return Err(Http1Error::AcceptError(result));
        }
        self.listener_socket = Some(result);
        Ok(())
    }

    pub(super) fn read_socket(&self) -> Result<([u8; 1024], isize), Http1Error> {
        let mut buffer: Buffer = [0; 1024];
        let count: isize;
        let listener = self.listener_socket.unwrap();
        dbg!(listener);
        unsafe {
            asm!(
                "syscall",
                in("rax") LinuxSyscalls::SYS_READ as usize,
                in("rdi") listener,
                in("rsi") buffer.as_mut_ptr(),
                in("rdx") buffer.len(),
                lateout("rax") count,
                options(nostack)
            )
        }
        if count < 0 {
            return Err(Http1Error::BindSocketError(count as i32));
        }
        Ok((buffer, count))
    }

    pub(super) fn close_socket(&mut self) -> Result<(), Http1Error> {
        let result: i32;
        unsafe {
            asm!(
                "syscall",
                in("rax") LinuxSyscalls::SYS_CLOSE as usize,
                in("rdi") self.socket_fd,
                lateout("rax") result,
                options(nostack)
            )
        }
        if result < 0 {
            return Err(Http1Error::BindSocketError(result));
        }
        self.listener_socket = None;
        Ok(())
    }

    /// Kernel will have socket timeout on restart of the server app,
    /// so we need to reuse the socket instead of waiting for assigning
    /// (anyways assign was failing during TIMOUT)
    pub(super) fn set_reuse_address(&self) -> Result<i32, Http1Error> {
        let sock: i32 = 1;
        let result: i32;
        unsafe {
            asm!(
                "syscall",
                in("rax") LinuxSyscalls::SYS_SETSOCKOPT as usize,
                in("rdi") &self.socket_fd,
                in("rsi") 1, // SOL_SOCKET
                in("rdx") 2, // SOL_REUSEADDR
                in("r10") &sock as *const i32 as usize,
                in("r8") 4,
                lateout("rax") result,
                options(nostack)
            )
        }

        if result < 0 {
            return Err(Http1Error::BindSocketError(result));
        }
        Ok(result)
    }

    pub(super) fn write_socket(&self, text: &str) -> Result<i32, Http1Error> {
        let result: i32;
        unsafe {
            asm!(
                "syscall",
                in("rax") LinuxSyscalls::SYS_WRITE as usize,
                in("rdi") self.socket_fd,
                in("rsi") text.as_ptr(),
                in("rdx") text.len(),
                lateout("rax") result,
                options(nostack)
            )
        }
        if result < 0 {
            return Err(Http1Error::ListenError(result));
        }
        Ok(result)
    }

    pub(super) fn create_response(&self, text: &str) -> String {
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
}
