use crate::providers::png::{
    chunks::{Chunk, ChunkContext, PngChunkHeader},
    error::{PngConstructionError, PngWriteError},
};

/// The iCCP chunk provides an advanced color profile.
#[derive(Clone, Debug, PartialEq, PartialOrd, Hash)]
pub struct Iccp {
    header: PngChunkHeader,
    profile_name: String, // converting from the NUL-terminated grossness
    compression_method: CompressionMethod,
    compressed_profile: Vec<u8>, // not decompressing that. have fun nerds
}

impl Chunk for Iccp {
    const TYPE: [u8; 4] = *b"iCCP";

    fn read(
        blob: &mut &[u8],
        header: PngChunkHeader,
        context: ChunkContext,
    ) -> Result<Self, PngConstructionError> {
        // check that there's no context
        debug_assert_eq!(context, ChunkContext::Other);

        // grab profile name
        let mut profile_name: String = String::new();
        for _ in 0..79 {
            // grab a byte
            let byte: u8 = Self::read_u8(blob, "profile name (one byte)")?;
            if byte == 0x00 {
                break;
            }

            // push it to the string
            profile_name.push(byte as char);
        }

        // grab compression method
        let compression_method_raw: u8 = Self::read_u8(blob, "compression method")?;
        let compression_method: CompressionMethod = if compression_method_raw != 0 {
            return Err(PngConstructionError::IccpUnknownCompressionValue {
                value: compression_method_raw,
            });
        } else {
            CompressionMethod::ZlibDeflate
        };

        // read the rest of the compressed profile
        let profile_len: usize = profile_name.len().saturating_sub(2);
        let mut compressed_profile: Vec<u8> = Vec::with_capacity(profile_len);
        for _ in 0..profile_len {
            let byte: u8 = Self::read_u8(blob, "compressed profile (one byte)")?;
            compressed_profile.push(byte);
        }

        Ok(Self {
            header,
            profile_name,
            compression_method,
            compressed_profile,
        })
    }

    fn write<W: std::io::Write>(&self, buf: &mut W) -> Result<(), PngWriteError> {
        // write the header
        self.header.write(buf)?;

        // profile name
        for c in self.profile_name.chars().map(|c: char| c as u8) {
            if profile_name_char_is_allowed(c) {
                Self::write_u8(c, buf, "profile name (one byte)")?;
            } else {
                log::error!("iCCP chunk: A character in the profile name was not Latin-1: `{c}`");
                return Err(PngWriteError::IccpProfileNameNotLatin1 { c });
            }
        }

        // NUL terminator on profile name
        Self::write_u8(0_u8, buf, "null separator")?;

        // compression method (will always be 0)
        Self::write_u8(0_u8, buf, "compression method")?;

        // compressed profile
        for byte in &self.compressed_profile {
            Self::write_u8(*byte, buf, "compressed profile (one byte)")?;
        }

        Ok(())
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Hash)]
pub enum CompressionMethod {
    ZlibDeflate = 0,
}

/// Checks if a character in the profile name is allowed.
const fn profile_name_char_is_allowed(c: u8) -> bool {
    (c >= 0x20 && c <= 0x7E) || (c >= 0xA1)
}
