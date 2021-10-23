use std::ffi::OsStr;

use tokio::process::Command;

pub trait CommandExt {
    fn arg_pair(&mut self, arg1: impl AsRef<OsStr>, arg2: impl AsRef<OsStr>) -> &mut Self;
}

impl CommandExt for Command {
    fn arg_pair(&mut self, arg1: impl AsRef<OsStr>, arg2: impl AsRef<OsStr>) -> &mut Self {
        self.arg(arg1);
        self.arg(arg2);
        self
    }
}
