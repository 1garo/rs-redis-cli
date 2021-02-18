use async_std::net::TcpStream;
use async_std::io;

// TODO: imeplement basic redis cliente [https://www.youtube.com/watch?v=8TfjFZ478Rs&list=WL&index=2&t=290s&ab_channel=RyanLevick] 33:26
#[async_std::main]
async fn main() -> io::Result<()> {
    let _stream = TcpStream::connect("localhost:6379").await?;
    println!("Hello, world!");
    Ok(())
}
