use async_std::io;
use async_std::net::{TcpStream, ToSocketAddrs};
use async_std::prelude::*;

#[async_std::main]
async fn main() -> io::Result<()> {
    let mut client = Client::new("localhost:6379").await?;
    client.set("ryan".into(), "Tampa".into()).await.unwrap();
    println!("{}", client.get("ryan".into()).await.unwrap());
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

struct Client {
    stream: TcpStream,
}

impl Client {
    async fn new<A: ToSocketAddrs>(addr: A) -> Result<Client, io::Error> {
        let stream = TcpStream::connect(addr).await?;
        Ok(Client { stream })
    }
}

impl Client {
    async fn get(&mut self, key: String) -> Result<String, Error> {
        let command = RespValue::Array(vec![
            RespValue::BulkString(b"GET".to_vec()),
            RespValue::BulkString(key.into_bytes()),
        ]);
        let mut buffer = vec![];
        command.serialize(&mut buffer);
        self.stream.write_all(&buffer).await?;

        let bytes_read = self.stream.read(&mut buffer).await?;
        let resp = parse_response(&buffer[..bytes_read])?;
        Ok(resp.to_owned())
    }
    async fn set(&mut self, key: String, value: String) -> Result<(), Error> {
        let command = RespValue::Array(vec![
            RespValue::BulkString(b"SET".to_vec()),
            RespValue::BulkString(key.into_bytes()),
            RespValue::BulkString(value.into_bytes()),
        ]);

        let mut buffer = vec![];
        command.serialize(&mut buffer);
        self.stream.write_all(&buffer).await?;

        let bytes_read = self.stream.read(&mut buffer).await?;
        parse_response(&buffer[..bytes_read])?;
        Ok(())
    }
}
#[derive(Debug)]
struct Error {}
impl std::convert::From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error {}
    }
}

enum RespValue {
    SimpleString(String),
    Error(Vec<u8>),
    Integer(i64),
    BulkString(Vec<u8>),
    Array(Vec<RespValue>),
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
                buf.append(&mut format!("{}", data.len()).into_bytes());
                buf.push('\r' as u8);
                buf.push('\n' as u8);
                buf.append(&mut data);
                buf.push('\r' as u8);
                buf.push('\n' as u8);
            }
            RespValue::SimpleString(_) => {
                unimplemented!()
            }
            RespValue::Error(_) => {
                unimplemented!()
            }
            RespValue::Integer(_) => {
                unimplemented!()
            }
        }
    }
}
