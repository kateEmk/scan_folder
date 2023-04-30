#[derive(Debug, Clone)]
pub struct OperationFailure {
    pub path: String,
    pub line: usize,
    pub msg: String,
}

impl std::error::Error for OperationFailure {}

impl std::fmt::Display for OperationFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "File {}:{}.\nMessage:\n{}",
            self.path, self.line, self.msg
        )
    }
}

impl OperationFailure {
    pub fn pretty(self) -> String {
        format!("{}:{}.\n{}", self.path, self.line, self.msg)
    }
}

#[macro_export]
macro_rules! ok_or_err {
    ($result:expr $(,)?) => {
        match $result {
            Some(value) => value,
            _ => {
                let msg = format!(
                    "Operation failed. \n"
                );
                fail_inner!(msg);
            }
        }
    };
}
pub use ok_or_err;

#[macro_export]
macro_rules! fail_inner {
    ($msg: expr $(,)?) => {
        if cfg!(feature = "fire-panics-on-asserts") {
            panic!("{}", $msg)
        } else {
            return Err(OperationFailure {
                path: file!().to_string(),
                line: line!() as usize,
                msg: $msg,
            }
            .into());
        }
    };
}
pub use fail_inner;