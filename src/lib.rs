#![warn(clippy::pedantic)]
mod common;
mod huffman;
mod lz77;
mod rle;
mod tests;

pub use crate::common::CompressionScheme;
pub use crate::huffman::Huffman;
pub use crate::lz77::LZ77;
pub use crate::rle::RLE;
