use std::fmt;

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    CtrlCError(ctrlc::Error),
    ClapError(clap::Error),
    UnknownInstruction(u8),
    UnknownCbInstruction(u8),
    InvalidReadFromMemoryLocation(u16),
    InvalidWriteToMemoryLocation(u16),
    TODOHalt,
    Abort(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::UnknownInstruction(inst) => write!(f, "Unknown instruction `{:02x}`", inst),
            Error::UnknownCbInstruction(inst) => write!(f, "Unknow instruction `cb {:02x}`", inst),
            Error::InvalidReadFromMemoryLocation(addr) => {
                write!(f, "Invalid read from memory location `{:04x}`", addr)
            }
            Error::InvalidWriteToMemoryLocation(addr) => {
                write!(f, "Invalid write to memory location `{:04x}`", addr)
            }
            Error::TODOHalt => write!(f, "TODO HALT"),
            Error::IoError(original) => write!(f, "IO Error: {}", original),
            Error::ClapError(original) => write!(f, "ClapError: {}", original),
            Error::CtrlCError(original) => write!(f, "CtrlCError: {}", original),
            Error::Abort(msg) => write!(f, "Aborting, {}", msg),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IoError(error)
    }
}

impl From<ctrlc::Error> for Error {
    fn from(error: ctrlc::Error) -> Self {
        Error::CtrlCError(error)
    }
}

impl From<clap::Error> for Error {
    fn from(error: clap::Error) -> Self {
        Error::ClapError(error)
    }
}
