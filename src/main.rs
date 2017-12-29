extern crate bytes;
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;

use std::io;
use bytes::BytesMut;
use futures::{future, Future};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::{Decoder, Encoder, Framed};
use tokio_proto::TcpServer;
use tokio_proto::pipeline::ServerProto;
use tokio_service::Service;

mod redis;
use redis::{Cmd, Res};

mod storage;
use storage::{Default, Storage};

fn main() {
    println!("Rustis start!");

    let addr = "0.0.0.0:6379".parse().unwrap();
    let server = TcpServer::new(RedisProto, addr);

    // this lambda instantiates a new service for each incoming connection
    server.serve(|| {
        Ok(RedisService {
            storage: Box::new(Default),
        })
    });
}

pub struct RedisCodec;

impl Decoder for RedisCodec {
    type Item = Cmd;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<Cmd>> {
        // TODO: for pipelining, make this a while(parse) and then batch them up
        if let Ok(Some(cmd)) = redis::parse_cmd(buf) {
            Ok(Some(cmd))
        } else {
            Ok(None)
        }
    }
}

impl Encoder for RedisCodec {
    type Item = Res;
    type Error = io::Error;

    fn encode(&mut self, msg: Res, buf: &mut BytesMut) -> io::Result<()> {
        msg.serialize(buf);
        Ok(())
    }
}

pub struct RedisProto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for RedisProto {
    type Request = Cmd;
    type Response = Res;

    type Transport = Framed<T, RedisCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(RedisCodec))
    }
}

pub struct RedisService {
    storage: Box<Storage>,
}

impl Service for RedisService {
    type Request = Cmd;
    type Response = Res;

    type Error = io::Error;

    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let res = match req {
            Cmd::Echo(cmd) => self.storage.echo(cmd),
            Cmd::Ping(cmd) => self.storage.ping(cmd),
        };
        Box::new(future::ok(res))
    }
}
