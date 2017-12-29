extern crate bytes;

use bytes::BytesMut;

pub enum Res {
    Echo { msg: String },

    Pong { msg: Option<String> },
}

impl Res {
    pub fn serialize(&self, buf: &mut BytesMut) {
        match self {
            &Res::Echo { msg } => {
                buf.extend("+".as_bytes());
                buf.extend(msg.as_bytes());
                buf.extend("\r\n".as_bytes());
            }

            &Res::Pong { msg } => {
                if let Some(m) = msg {
                    buf.extend("+".as_bytes());
                    buf.extend(m.as_bytes());
                    buf.extend("\r\n".as_bytes());
                } else {
                    buf.extend("+PONG\r\n".as_bytes())
                }
            }
        }
    }
}
