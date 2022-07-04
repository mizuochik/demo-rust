use tokio::{
    fs::File,
    io::{self, AsyncReadExt, AsyncWriteExt},
};

#[tokio::main]
async fn main() -> io::Result<()> {
    read().await?;
    read_to_end().await?;
    write_all().await?;
    Ok(())
}

async fn read() -> io::Result<()> {
    let mut f = File::open("Cargo.toml").await?;
    let mut buf = [0; 10];
    let n = f.read(&mut buf).await?;
    println!("0..{} {:?}", n, &buf[..n]);
    read_to_end().await?;
    Ok(())
}

async fn read_to_end() -> io::Result<()> {
    let mut f = File::open("Cargo.toml").await?;
    let mut buf = Vec::new();
    let n = f.read_to_end(&mut buf).await?;
    println!("0..{} {:?}", n, &buf[..n]);
    Ok(())
}

async fn write_all() -> io::Result<()> {
    let mut f = File::create("hello.txt").await?;
    f.write_all(b"Hello World").await?;
    Ok(())
}
