use std::error::Error;

mod cli;
mod server;

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::parse_args();
    server::start(&args)?;
    Ok(())
}
