use super::super::redis::{Cmd, Res};

pub trait Storage {
    fn echo(&self, Cmd) -> Res;
    fn ping(&self, Cmd) -> Res;
}
