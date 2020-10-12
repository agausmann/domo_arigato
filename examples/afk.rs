use anyhow::Context;
use domo_arigato::auth::authenticate;
use domo_arigato::state::connect;
use std::env;
use std::io::{stdin, stdout, BufRead, Write};

fn main() -> anyhow::Result<()> {
    let mut args = env::args();
    let host = args.nth(1).context("missing arguments: <host> <port>")?;
    let port = args
        .next()
        .context("missing arguments: <port>")?
        .parse()
        .context("invalid port number")?;

    let stdin = stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    print!("Mojang account ID: ");
    stdout().flush()?;
    let account_id = lines.next().context("EOF")??;
    print!("Password: ");
    stdout().flush()?;
    let password = lines.next().context("EOF")??;

    let authentication = authenticate(&account_id, &password)?;
    connect(host, port, 751)?
        .login()?
        .login(&authentication)?
        .run()?;

    Ok(())
}
