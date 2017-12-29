use super::types::Storage;
use super::super::redis::*;

pub fn NewDefault() -> Default {
    Default {}
}

pub struct Default;

impl Storage for Default {
    fn echo(&self, cmd: EchoCmd) -> Res {
        Res::Echo { msg: cmd.msg }
    }

    fn ping(&self, cmd: PingCmd) -> Res {
        Res::Pong { msg: cmd.msg }
    }
}
