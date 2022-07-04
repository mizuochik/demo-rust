use tokio::{io::{self, AsyncReadExt}, fs::File};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut f = File::open("Cargo.toml").await?;
    let mut buf = Vec::new();
    let n = f.read_to_end(&mut buf).await?;
    println!("0..{} {:?}", n, &buf[..n]);
    Ok(())
}
