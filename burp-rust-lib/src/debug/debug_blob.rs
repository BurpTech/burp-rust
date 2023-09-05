use std::fmt::{Display, Formatter, Result};

pub struct DebugBlob<'a> {
    blob: &'a [u8],
}

impl DebugBlob<'_> {
    pub fn new(blob: &[u8]) -> DebugBlob {
        return DebugBlob { blob };
    }
}

const VALID_UTF8_LABEL: &str = "VALID_UTF8";
const INVALID_UTF8_LABEL: &str = "INVALID_UTF8";

impl Display for DebugBlob<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match std::str::from_utf8(self.blob) {
            Ok(utf8) => write!(f, "{}: {}", VALID_UTF8_LABEL, utf8),
            Err(utf8_error) => unsafe {
                write!(f, "{}, {}",
                       INVALID_UTF8_LABEL,
                       std::str::from_utf8_unchecked(&self.blob[..utf8_error.valid_up_to()]),
                )
            },
        }
    }
}
