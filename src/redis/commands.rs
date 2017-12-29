extern crate bytes;

use std::io;
use bytes::BytesMut;

pub enum Cmd {
    Echo {
        msg: String,
    },

    Get {
        key: String,
    },

    Ping {
        msg: Option<String>,
    },

    Set {
        key: String,
        value: String,
        ex: Option<u64>,
        px: Option<u64>,
        nx: Option<bool>,
        xx: Option<bool>,
    },
}

// returns an optional command. If the return valuse is Some(Cmd) then the buf
// has been modified to skip past the bytes consumed.
pub fn parse_cmd(buf: &mut BytesMut) -> io::Result<Option<Cmd>> {
    let rn = &[b'\r', b'\n'];

    let foo = buf.as_ref();

    if foo[0] != b'*' {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid RESP request",
        ));
    }

    let start: u32 = 1;
    let end: u32 = 1;

    expect(buf, rn, 1);
}

fn expect(buf: &mut BytesMut, expected: &[u8], idx: usize) -> bool {
    let mut bp = idx;
    let mut ep = 0;

    while bp < buf.len() && ep < expected.len() {
        if *buf.get(bp).unwrap() != expected[ep] {
            return false;
        }

        bp += 1;
        ep += 1;
    }

    true
}
