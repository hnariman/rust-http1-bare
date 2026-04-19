#![allow(unused, non_camel_case_types, clippy::upper_case_acronyms)]
use core::result;
use syscalls::*;

pub struct HttpServer {
    socket_fd: i32,
    listener_socket: Option<i32>,
    port: u16,
}

impl HttpServer {
    pub fn new(port: u16) -> Result<Self, HttpError> {
        let fd = syscalls::create_socket()?;
        Ok(Self {
            socket_fd: fd,
            port,
            listener_socket: None,
        })
    }

    pub fn listen(&mut self) -> Result<(), HttpError> {
        syscalls::set_reuse_address(self.socket_fd)?;
        self.listener_socket = Some(syscalls::bind_socket(4000, self.socket_fd)?);
        syscalls::set_socket_to_listen(self.socket_fd)?;

        let mut buffer = [0; 1024];

        loop {
            let listener = syscalls::accept_incoming(self.socket_fd)?;
            let count = syscalls::read_socket(listener, &mut buffer)?;
            if count > 0 {
                if let Ok(txt) = std::str::from_utf8(&buffer[..count as usize]) {
                    println!("{txt}")
                }
                syscalls::write_socket(listener, &syscalls::create_response("hello"))?;
                println!("-----------------------------------");
                syscalls::close_socket(listener)?;
            }
        }
        Ok(())
    }
}
