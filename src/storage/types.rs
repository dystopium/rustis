use super::super::redis::*;

pub trait Storage {
    fn echo(&self, EchoCmd) -> Res;
    fn ping(&self, PingCmd) -> Res;
}
