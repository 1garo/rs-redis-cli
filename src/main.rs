use std::process;

use async_std::io;
use async_std::net::{TcpStream, ToSocketAddrs};
use async_std::prelude::*;

#[async_std::main]
async fn main() -> io::Result<()> {
    // TODO: undestand return output
    let mut client = Client::new("localhost:6379").await?;
    client.set("ryan".into(), "13".into()).await.unwrap();
    println!("before incr: {}", client.get("ryan".into()).await.unwrap());
    println!("incr: {}", client.op_on_int("ryan".into(), "incr".to_string()).await.unwrap());
    println!("after incr: {}", client.get("ryan".into()).await.unwrap());
    Ok(())
}

fn parse_response(buffer: &[u8]) -> Result<&str, io::Error> {
    if buffer.is_empty() {
        println!("{}", "buffer is empty");
        process::exit(1);
    }

    if buffer[0] == ('-' as u8) {
        println!("{}", "some error happened");
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
    async fn op_on_int(&mut self, key: String, op: String) -> Result<String, io::Error> {
        let command = RespValue::Array(vec![
            RespValue::BulkString(format!("{}", op).into_bytes()),
            RespValue::BulkString(key.into_bytes()),
        ]);
        let mut buffer = vec![];
        command.serialize(&mut buffer);
        self.stream.write_all(&buffer).await?;

        let bytes_read = self.stream.read(&mut buffer).await?;
        let resp = parse_response(&buffer[..bytes_read])?;
        Ok(resp.to_owned())
    }

    async fn get(&mut self, key: String) -> Result<String, io::Error> {
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
    async fn set(&mut self, key: String, value: String) -> Result<(), io::Error> {
        let command = RespValue::Array(vec![
            RespValue::BulkString(b"SET".to_vec()),
            RespValue::BulkString(key.into_bytes()),
            RespValue::BulkString(value.into_bytes()),
            // RespValue::Integer(&mut value.parse::<i64>().unwrap()),
        ]);
        // println!("{:?}", command);

        let mut buffer = vec![];
        command.serialize(&mut buffer);
        // println!("\n{:?}", buffer);
        self.stream.write_all(&buffer).await?;

        let bytes_read = self.stream.read(&mut buffer).await?;
        parse_response(&buffer[..bytes_read])?;
        Ok(())
    }
}
// #[derive(Debug)]
// struct Error {
//     e: io::Error
// }

// impl std::convert::From<io::Error> for Error {
//     fn from(e: io::Error) -> Self {
//         Error {e}
//     }
// }

#[derive(Debug)]
enum RespValue {
    SimpleString(String),
    // Error(Vec<u8>),
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
            RespValue::SimpleString(data) => {
                buf.push(b'+');
                buf.append(&mut format!("{}", data.len()).into_bytes());
                buf.push('\r' as u8);
                buf.push('\n' as u8);
                buf.append(&mut format!("{}", data).into_bytes());
                buf.push('\r' as u8);
                buf.push('\n' as u8);

            }
            // RespValue::Error(_) => {
            //     unimplemented!()
            // }
            RespValue::Integer(data) => {
                buf.push(b':');
                buf.append(&mut format!("{}", data).into_bytes());
                buf.push('\r' as u8);
                buf.push('\n' as u8);
            }
        }
    }
}
