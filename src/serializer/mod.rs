use serde::Serialize;
use std::fmt;

pub mod audit_parse;

#[derive(Debug, Serialize)]
pub(crate) struct Operation {
    user: String,
    group: String,
    executable: String,
    syscall: String,
    key: OperationKey,
}

#[derive(Debug, Serialize)]
pub(crate) enum OperationKey {
    READ,
    WRITE,
}

impl fmt::Display for OperationKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
