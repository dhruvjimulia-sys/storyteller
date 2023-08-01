use super::*;

pub const IO_ERROR: Error = Error {
    error_type: "IO Error",
    error_message: "IO Error Description"
};

pub const VARIABLE_NOT_FOUND: Error = Error {
    error_type: "Variable Not Found Error",
    error_message: "Variable Not Found Error Description"
};

pub const LABEL_NOT_FOUND: Error = Error {
    error_type: "Label Not Found Error",
    error_message: "Label Not Found Error Description"
};