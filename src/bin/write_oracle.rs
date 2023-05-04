use papr_server_rs::oracle;

#[tokio::main]
async fn main() -> Result<(), eyre::Error> {
    oracle::cache_oracle_values("0x3b29c19ff2fcea0ff98d0ef5b184354d74ea74b0").await?;

    Ok(())
}
