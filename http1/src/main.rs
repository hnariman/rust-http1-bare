mod http1;
use http1::HttpServer;
use syscalls::HttpError;

fn main() -> Result<(), HttpError> {
    println!("Hello, server!");

    let mut server = HttpServer::new(4000)?;
    server.listen()?;
    Ok(())
}
