#![allow(non_camel_case_types)]
#[repr(C)]
pub struct SockAddrIn {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: u32,
    pub sin_zero: [u8; 8],
}

pub type Buffer = [u8; 1024];

// Linux syscalls to be used
pub enum LinuxSyscalls {
    SYS_READ = 0,
    SYS_WRITE = 1,
    SYS_SOCKET = 41,
    SYS_BIND = 49,
    SYS_LISTEN = 50,
    SYS_ACCEPT = 43,
    SYS_CLOSE = 3,
    SYS_SETSOCKOPT = 54,
}

// for more info use ```man 2 socket```
pub enum Inet {
    AF_INET = 2,   // IPV4
    AF_INET6 = 10, // IPV6
}

pub enum SockStream {
    TCP = 1,
    UDP = 2,
}
