pub mod audit_parse;

pub(crate) struct Operation {
    user: String,
    group: String,
    executable: String,
    syscall: String,
    key: OperationKey,
}

pub(crate) enum OperationKey {
    READ,
    WRITE,
}
