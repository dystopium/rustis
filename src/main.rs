extern crate bytes;
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;

use std::io;
use std::str;
use bytes::BytesMut;
use futures::{future, Future};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::{Decoder, Encoder, Framed};
use tokio_proto::TcpServer;
use tokio_proto::pipeline::ServerProto;
use tokio_service::Service;

fn main() {
    println!("Rustis start!");

    let addr = "127.0.0.1:9999".parse().unwrap();
    let server = TcpServer::new(RedisProto, addr);

    // this lambda instantiates a new service for each incoming connection
    server.serve(|| Ok(RedisService));
}

pub struct RedisCodec;

impl Decoder for RedisCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<String>> {
        if let Some(i) = buf.iter().position(|&b| b == b'\n') {
            let line = buf.split_to(i);
            // remove the \n
            buf.split_to(1);

            match str::from_utf8(&line) {
                Ok(s) => Ok(Some(s.to_string())),
                Err(_) => Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid UTF-8")),
            }
        } else {
            Ok(None)
        }
    }
}

impl Encoder for RedisCodec {
    type Item = String;
    type Error = io::Error;

    fn encode(&mut self, msg: String, buf: &mut BytesMut) -> io::Result<()> {
        buf.extend(msg.as_bytes());
        buf.extend(b"\n");
        Ok(())
    }
}

pub struct RedisProto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for RedisProto {
    type Request = String;
    type Response = String;

    type Transport = Framed<T, RedisCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(RedisCodec))
    }
}

pub struct RedisService;

impl Service for RedisService {
    type Request = String;
    type Response = String;

    type Error = io::Error;

    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        Box::new(future::ok(req))
    }
}
