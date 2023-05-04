use papr_server_rs::oracle;
use std::env;

#[tokio::main]
async fn main() -> Result<(), eyre::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <address>", args[0]);
        std::process::exit(1);
    }

    let controller = &args[1];
    oracle::cache_oracle_values(controller).await?;

    Ok(())
}
