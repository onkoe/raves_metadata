/// An error that occurs when constructing a [`Png`] for its metadata.
#[derive(Clone, Debug, PartialEq, PartialOrd, Hash)]
pub enum PngConstructionError {
    /// The file ran out of bytes before we could check for a signature.
    ///
    /// It might be empty.
    NoSignature,

    /// No PNG signature was detected.
    NotAPng {
        /// The signature that was found instead.
        found: [u8; 8],
    },
}

impl core::fmt::Display for PngConstructionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const NOT_A_PNG_MSG: &str = "The given file's signature indicated it was not a PNG";

        match self {
            PngConstructionError::NoSignature => {
                f.write_str("File didn't have enough bytes for a signature.")
            }

            PngConstructionError::NotAPng { found } => match core::str::from_utf8(found) {
                Ok(utf8_found) => write!(
                    f,
                    "{NOT_A_PNG_MSG}. Signature was: `{found:?}`. (UTF-8: `{utf8_found}`)"
                ),
                Err(_) => write!(
                    f,
                    "{NOT_A_PNG_MSG}. Signature was: `{found:?}`. (Not valid UTF-8.)`"
                ),
            },
        }
    }
}

impl core::error::Error for PngConstructionError {}
