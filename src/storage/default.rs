use super::types::Storage;

use super::super::redis::Cmd;
use super::super::redis::Res;

pub struct Default;

impl Storage for Default {
    fn echo(&self, cmd: Cmd) -> Res {
        //
    }

    fn ping(&self, cmd: Cmd) -> Res {
        //
    }
}
