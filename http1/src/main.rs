mod http1;
use http1::{Http1Error, HttpServer};

fn main() -> Result<(), Http1Error> {
    println!("Hello, server!");

    let mut server = HttpServer::new(4000)?;
    server.listen()?;
    Ok(())
}
