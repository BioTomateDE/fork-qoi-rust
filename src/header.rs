use core::convert::TryInto;

use crate::consts::{QOI_HEADER_SIZE, QOI_MAGIC, QOI_PIXELS_MAX};
use crate::encode_max_len;
use crate::error::{Error, Result};
use crate::utils::unlikely;

/// Image header: dimensions, channels, color space.
///
/// ### Notes
/// A valid image header must satisfy the following conditions:
/// * Both width and height must be non-zero.
/// * Maximum number of pixels is 400Mp (=4e8 pixels).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Header {
    /// Image width in pixels
    pub width: u16,
    /// Image height in pixels
    pub height: u16,
    /// Image data length in bytes
    pub length: Option<u32>,
}

// impl Default for Header {
//     #[inline]
//     fn default() -> Self {
//         Self {
//             width: 1,
//             height: 1,
//             channels: Channels::default(),
//             colorspace: ColorSpace::default(),
//         }
//     }
// }

impl Header {
    /// Creates a new header and validates image dimensions.
    #[inline]
    pub const fn try_new(width: u16, height: u16, length: Option<u32>) -> Result<Self> {
        let n_pixels = (width as usize).saturating_mul(height as usize);
        if unlikely(n_pixels == 0 || n_pixels > QOI_PIXELS_MAX) {
            return Err(Error::InvalidImageDimensions { width, height });
        }
        Ok(Self { width, height, length })
    }
    
    /// Serializes the header into a bytes array.
    #[inline]
    pub fn encode(&self) -> Result<[u8; QOI_HEADER_SIZE]> {
        let data_length = self.length.ok_or_else(|| Error::DataLengthNotSet)?;
        
        let mut out = [0; QOI_HEADER_SIZE];
        out[..4].copy_from_slice(&QOI_MAGIC.to_le_bytes());
        out[4..6].copy_from_slice(&self.width.to_le_bytes());
        out[6..8].copy_from_slice(&self.height.to_le_bytes());
        out[8..12].copy_from_slice(&data_length.to_le_bytes());
        Ok(out)
    }

    /// Deserializes the header from a byte array.
    #[inline]
    pub fn decode(data: impl AsRef<[u8]>) -> Result<Self> {
        let data = data.as_ref();
        if unlikely(data.len() < QOI_HEADER_SIZE) {
            return Err(Error::UnexpectedBufferEnd);
        }
        let magic = u32::from_le_bytes(data[0..4].try_into().unwrap());
        let width = u16::from_le_bytes(data[4..6].try_into().unwrap());
        let height = u16::from_le_bytes(data[6..8].try_into().unwrap());
        let length = u32::from_le_bytes(data[8..12].try_into().unwrap());
        if unlikely(magic != QOI_MAGIC) {
            return Err(Error::InvalidMagic { magic });
        }
        Self::try_new(width, height, Some(length))
    }

    /// Returns a number of pixels in the image.
    #[inline]
    pub const fn n_pixels(&self) -> usize {
        (self.width as usize).saturating_mul(self.height as usize)
    }

    /// Returns the total number of bytes in the raw pixel array.
    ///
    /// This may come useful when pre-allocating a buffer to decode the image into.
    #[inline]
    pub const fn n_bytes(&self) -> usize {
        self.n_pixels() * 4
    }

    /// The maximum number of bytes the encoded image will take.
    ///
    /// Can be used to pre-allocate the buffer to encode the image into.
    #[inline]
    pub fn encode_max_len(&self) -> usize {
        encode_max_len(self.width, self.height)
    }
}
