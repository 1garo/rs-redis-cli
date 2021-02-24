use async_std::net::{TcpStream, ToSocketAddrs};
use async_std::io;
use async_std::prelude::*;

// TODO: implement basic redis cliente [https://www.youtube.com/watch?v=8TfjFZ478Rs&list=WL&index=2&t=290s&ab_channel=RyanLevick] 33:26
// TODO: implement api that receive csv files and 
#[async_std::main]
async fn main() -> io::Result<()> {
    let _stream = TcpStream::connect("localhost:6379").await?;
    println!("Hello, world!");
    Ok(())
}

fn parse_response(buffer: &[u8]) -> Result<&str, Error> {
    if buffer.is_empty() {
        return Err(Error {});
    }

    if buffer[0] == ('-' as u8) {
        return Err(Error {});
    }

    Ok(std::str::from_utf8(&buffer[1..buffer.len() - 2]).unwrap())
}

#[derive(Debug)]
struct Error {}

impl std::convert:From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error {}
    }
}

enum RespValue {
    SimpleString(String),
    Error(Vec<u8>),
    Integer(i64),
    BulkString(Vec<u8>),
    Array(Vec<RespValue>)
}

impl RespValue {
    fn serialize(self, buf: &mut Vec<u8>) {
        match self {
            RespValue::Array(values) => {
                buf.push(b'*');
                buf.append(&mut format!("{}", values.len()).into_bytes());
                buf.push('\r' as u8);
                buf.push('\n' as u8);
                for value in values {
                    value.serialize(buf);
                }
            }
            RespValue::BulkString(mut data) => {
                buf.push(b'$');
                buf.append(&mut format!("{}", values.len()).into_bytes());
                buf.push('\r' as u8);
                buf.push('\n' as u8);
                buf.append(&mut data);
                buf.push('\r' as u8);
                buf.push('\n' as u8);

            }
            RespValue::SimpleString(_) => {unimplemented!()}
            RespValue::Error(_) => {unimplemented!()}
            RespValue::Integer(_) => {unimplemented!()}
        }
    }
}