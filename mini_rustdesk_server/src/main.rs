
use flexi_logger::*;
mod config;
use config::RENDEZVOUS_PORT;
mod common;
mod rendezvous_server;
use rendezvous_server::RendezvousServer;
use crate::common::{init_args,get_arg_or};
use anyhow::{Error, Ok};

pub type ResultType<F, E = anyhow::Error> = anyhow::Result<F, E>;

fn main() -> ResultType<()> {
    let _logger = Logger::try_with_env_or_str("info")?
        .log_to_stdout()
        .format(opt_format)
        .write_mode(WriteMode::Async)
        .start()?;
    let args = format!(
        "-p, --port=[NUMBER(default={RENDEZVOUS_PORT})] 'Sets the listening port'
        -R, --rendezvous-servers=[HOSTS] 'Sets rendezvous servers, separated by comma'
        -r, --relay-servers=[HOST] 'Sets the default relay servers, separated by comma'
        -k, --key=[KEY] 'Only allow the client with the same key'",
    );
    init_args(&args, "hbbs", "RustDesk ID/Rendezvous Server");
    let port = get_arg_or("port", RENDEZVOUS_PORT.to_string()).parse::<i32>()?;
    if port < 3 {
        return Err(Error::msg("Invalid port number"))
    }
    let key  = get_arg_or("key", "-".to_owned());

    log::info!("port = {},key = {}", port, key);
    RendezvousServer::start(port, &key)?;
    Ok(())
}
