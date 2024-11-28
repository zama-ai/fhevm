use anyhow::Result;

mod cli;
mod server;

fn main() -> Result<()> {
    let args = cli::parse_args();
    server::start(&args)?;
    Ok(())
}
