use crate::{RtcError, SdpParseError};
use std::fmt;

/// shiguredo_webrtc 用の Error 型。
#[derive(Debug)]
pub enum Error {
    Message(String),
    NullPointer(&'static str),
    NulError(std::ffi::NulError),
    Utf8Error(std::str::Utf8Error),
    RtcError(RtcError),
    SdpParseError(SdpParseError),
    InvalidSdp,
    InvalidIceCandidate,
    OutOfIndex(usize),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Message(message) => f.write_str(message),
            Error::NullPointer(name) => write!(f, "{name} が null を返しました"),
            Error::NulError(err) => write!(f, "{err}"),
            Error::Utf8Error(err) => write!(f, "{err}"),
            Error::RtcError(err) => {
                if let Ok(message) = err.message() {
                    return f.write_str(&message);
                }
                f.write_str("RTCError が発生しました")
            }
            Error::SdpParseError(err) => {
                let line = err.line().ok();
                let description = err.description().ok();
                match (line, description) {
                    (Some(line), Some(description)) => {
                        write!(f, "SDP parse error: {} ({})", description, line)
                    }
                    (Some(line), None) => write!(f, "SDP parse error: {}", line),
                    (None, Some(description)) => write!(f, "SDP parse error: {}", description),
                    (None, None) => f.write_str("SDP parse error が発生しました"),
                }
            }
            Error::InvalidSdp => f.write_str("不正な SDP です"),
            Error::InvalidIceCandidate => f.write_str("不正な ICE candidate です"),
            Error::OutOfIndex(index) => write!(f, "インデックス {} が範囲外です", index),
        }
    }
}

impl std::error::Error for Error {}

impl From<String> for Error {
    fn from(message: String) -> Self {
        Error::Message(message)
    }
}

impl From<&str> for Error {
    fn from(message: &str) -> Self {
        Error::Message(message.to_string())
    }
}

impl From<std::ffi::NulError> for Error {
    fn from(err: std::ffi::NulError) -> Self {
        Error::NulError(err)
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(err: std::str::Utf8Error) -> Self {
        Error::Utf8Error(err)
    }
}

impl From<RtcError> for Error {
    fn from(err: RtcError) -> Self {
        Error::RtcError(err)
    }
}

impl From<SdpParseError> for Error {
    fn from(err: SdpParseError) -> Self {
        Error::SdpParseError(err)
    }
}

/// shiguredo_webrtc 用の Result 型。
pub type Result<T> = std::result::Result<T, Error>;
