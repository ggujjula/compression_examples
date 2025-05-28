/*
 * Compress data in a file:
 * let path = Path::new("asdlkfjasd")
 * let file = File::open(path);
 *  * match on file result*
 * let path = Path::new("asjdfldasldjfalkdsjf")
 * let file2 = File::create(path);
 *  * match on file result*
 * RLE::compress(file1, file2)
 */

use std::error::Error;
use std::fmt::{Display, Error as FmtError, Formatter};
use std::io::{Read, Result as IoResult, Write};
/*
 * Compression scheme is a thing that can take an input stream or file,
 * and produce either a compressed output or decompressed output.
 */
pub trait CompressionScheme {
    fn compress(input: impl Read, output: impl Write) -> IoResult<()>;
    fn decompress(input: impl Read, output: impl Write) -> IoResult<()>;
}

#[derive(Debug)]
pub enum CompressionError {
    IncompleteInput,
}

impl Display for CompressionError {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result<(), FmtError> {
        Ok(())
    }
}

impl Error for CompressionError {}
