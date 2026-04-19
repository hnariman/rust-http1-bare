use core::result;

use crate::http2::error::HttpError;

pub mod error;
pub mod internal;
pub mod types;

pub struct HttpServer {
    socket_fd: i32,
    listener_socket: Option<i32>,
    port: u16,
}

impl HttpServer {
    pub fn new(port: u16) -> Result<Self, HttpError> {
        let fd = HttpServer::create_socket()?;
        Ok(Self {
            socket_fd: fd,
            port,
            listener_socket: None,
        })
    }

    pub fn listen(&mut self) -> Result<(), HttpError> {
        self.set_reuse_address();
        self.listener_socket = Some(self.bind_socket()?);
        self.set_socket_to_listen()?;

        loop {
            self.accept_incoming();
            let (buffer, count) = self.read_socket()?;
            if count > 0 {
                if let Ok(txt) = std::str::from_utf8(&buffer[..count as usize]) {
                    println!("{txt}")
                }
                self.write_socket(&self.create_response("text"));
                self.close_socket();
            }
        }
        Ok(())
    }
}
