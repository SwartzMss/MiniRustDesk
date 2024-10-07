
use flexi_logger::*;
use anyhow::{Error, Ok};

pub type ResultType<F, E = anyhow::Error> = anyhow::Result<F, E>;



fn main() -> ResultType<()> {
    let _logger = Logger::try_with_env_or_str("info")?
        .log_to_stdout()
        .format(opt_format)
        .write_mode(WriteMode::Async)
        .start()?;
    Ok(())
}
