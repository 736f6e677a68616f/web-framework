use anyhow::Result;
use tokio::fs;
#[tokio::main]
async fn main() -> Result<()>{
    let f = fs::read("G:/a/src/aaa.txt").await?;
    
    println!("{:?}", String::from_utf8(f));
    Ok(())
}
