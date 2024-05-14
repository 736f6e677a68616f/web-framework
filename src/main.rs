use anyhow::Result;
use redis::AsyncCommands;
use random;
use random::Source;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = init_rds().await?;
    Ok(())
}

async fn init_rds() -> Result<()> {
    let cli = redis::Client::open("redis://192.168.16.109:6379")?;
    let mut con = cli.get_multiplexed_async_connection().await?;
    let mut source = random::default(42);
    let num = source.read::<f64>();
    con.set("key", num).await?;
    let r = con.get("key").await?;
    dbg!(r);
    Ok(())
}