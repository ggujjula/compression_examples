use crate::common::CompressionScheme;
use std::io::{Read, Result as IoResult, Write};

pub struct LZ77;

impl CompressionScheme for LZ77 {
    fn compress(input: impl Read, output: impl Write) -> IoResult<()> {
        todo!();
    }

    fn decompress(input: impl Read, output: impl Write) -> IoResult<()> {
        todo!();
    }
}
