use anyhow::Context;
use domo_arigato::state::connect;
use std::env;

fn main() -> anyhow::Result<()> {
    let mut args = env::args();
    let host = args.nth(1).context("missing arguments: <host> <port>")?;
    let port = args
        .next()
        .context("missing arguments: <port>")?
        .parse()
        .context("invalid port number")?;

    let (data, ping) = connect(host, port, -1)?.status()?.query()?;
    println!("Ping: {}ms", ping.as_millis());
    println!("Info: {:#?}", data);
    Ok(())
}
