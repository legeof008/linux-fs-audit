use serde::Serialize;
use std::fmt;

pub mod audit_parse;

#[derive(Debug, Serialize)]
pub(crate) struct Operation {
    pub(crate) user: String,
    pub(crate) group: String,
    pub(crate) executable: String,
    pub(crate) syscall: String,
    pub(crate) timestamp: String,
    pub(crate) key: OperationKey,
}
#[derive(Debug, Serialize)]
pub(crate) struct FileOperatedOn {
    pub(crate) name: String,
    pub(crate) timestamp: String,
}

#[derive(Debug, Serialize, PartialEq)]
pub(crate) enum OperationKey {
    READ,
    WRITE,
}

impl fmt::Display for OperationKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
