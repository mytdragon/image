//! Encoding of AVIF images.
///
/// The [AVIF] specification defines an image derivative of the AV1 bitstream, an open video codec.
///
/// [AVIF]: https://aomediacodec.github.io/av1-avif/
#[cfg(feature = "avif-decode")]
pub use self::decoder::AvifDecoder;
#[cfg(feature = "avif-encode")]
pub use self::encoder::{AvifEncoder, ColorSpace};

#[cfg(feature = "avif-decode")]
mod decoder;
#[cfg(feature = "avif-encode")]
mod encoder;
#[cfg(feature = "avif-decode")]
mod ycgco;
#[cfg(feature = "avif-decode")]
mod yuv;
